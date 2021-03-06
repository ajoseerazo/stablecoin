use super::*;

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn owner_can_add_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		Owner::<Test>::put(owner);
		let minter = 2;
		assert_ok!(Copcoin::add_minter(Origin::signed(owner), minter));
		// Read pallet storage and assert an expected result.
		assert!(Copcoin::is_minter(minter));
	});
}

#[test]
fn can_remove_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		Owner::<Test>::put(owner);
		let minter = 2;
		Minters::<Test>::insert(minter, true);

		assert_ok!(Copcoin::remove_minter(Origin::signed(owner), minter));
		assert!(!Copcoin::is_minter(minter));
	});
}

#[test]
fn only_owner_can_add_minter() {
	new_test_ext().execute_with(|| {
		let non_owner = 1;
		let minter = 2;
		assert_noop!(
			Copcoin::add_minter(Origin::signed(non_owner), minter),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn minter_can_mint() {
	new_test_ext().execute_with(|| {
		let minter = 2;
		Minters::<Test>::insert(minter, true);

		let recipient = 3;
		let amount = 42;
		assert_ok!(Copcoin::mint(Origin::signed(minter), recipient, amount));

		assert_eq!(Copcoin::balance_of(recipient), amount);
	})
}