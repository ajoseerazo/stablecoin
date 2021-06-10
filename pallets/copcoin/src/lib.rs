#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
	traits::Get,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Config> as Copcoin {
		Owner get(fn owner): T::AccountId;
		TotalSupply get(fn total_supply): u64;
		Balances get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => u64; // u256
		Minters get(fn is_minter): map hasher(twox_64_concat) T::AccountId => bool;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		SupplyChanged(u64, AccountId),
		Mint(AccountId, AccountId, u64),
		Burn(AccountId, u64),
		MinterAdded(AccountId),
		MinterRemoved(AccountId),
		OwnerSet(AccountId),
		NewOwner(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		// Tried to call a function that is limited to the owner
		NotOwner,
		// A non minter account tries to mint
		NotMinter,
		SupplyOverflow
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn set_owner(origin, new_owner: T::AccountId) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			<Owner<T>>::put(&new_owner);

			Self::deposit_event(RawEvent::NewOwner(new_owner));
			Ok(())
		}

		// Add an account as a minter
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn add_minter(origin, new_minter: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _owner = Self::ensure_owner(who)?;

			<Minters<T>>::insert(&new_minter, true);

			// Emit an event.
			Self::deposit_event(RawEvent::MinterAdded(new_minter));
			// Return a successful DispatchResult
			Ok(())
		}

		// remove an account from the set of minters
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn remove_minter(origin, minter: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _owner = Self::ensure_owner(who)?;

			<Minters<T>>::remove(&minter);

			Self::deposit_event(RawEvent::MinterRemoved(minter));
			Ok(())
		}

		// Create `amount` of coins out of thn air and deposit then into `to_account`
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn mint(origin, to_account: T::AccountId, amount: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let minter = Self::ensure_minter(who)?;

			let supply = Self::total_supply();

			let new_supply = supply.checked_add(amount).ok_or(Error::<T>::SupplyOverflow)?;
			// ^ verify
			// v update
			<TotalSupply>::put(new_supply);
			<Balances<T>>::mutate(&to_account, |balance| {
				*balance = balance.saturating_add(amount);
			});
			Self::deposit_event(RawEvent::Mint(minter, to_account, amount));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	fn ensure_owner(acc: T::AccountId) -> Result<T::AccountId, DispatchError> {
		if acc != Self::owner() {
			return Err(DispatchError::from(Error::<T>::NotOwner));
		}
		Ok(acc)
	}

	fn ensure_minter(acc: T::AccountId) -> Result<T::AccountId, DispatchError> {
		if !Self::is_minter(&acc) {
			return Err(DispatchError::from(Error::<T>::NotMinter));
		}
		Ok(acc)
	}
}
