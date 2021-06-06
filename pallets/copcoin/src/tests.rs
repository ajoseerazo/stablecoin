use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn can_set_supply() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Copcoin::set_supply(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Copcoin::total_supply(), 42);
	});
}

/*#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}*/
