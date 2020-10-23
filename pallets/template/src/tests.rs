use crate::{Error, mock::*, UserDetails};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn storing_user_details_should_work() {
	new_test_ext().execute_with(|| {
		let details = UserDetails{ name : Vec::from("jacobmakarsky") , age: 18, birthYear: None };
		assert_ok!(TemplateModule::update_user_details(Origin::signed(1), details.clone()));
		assert_eq!(TemplateModule::get_user_details(), details);
	});
}

#[test]
fn above_age_21_is_adult() {
	new_test_ext().execute_with(|| {
		let details = HackathonDetails{ username : Vec::from("jacobmakarsky") , age: 21, birthYear: None };
		assert_ok!(TemplateModule::update_user_details(Origin::signed(1), details.clone()));
		assert_eq!(TemplateModule::get_user_details().age, Some(1999));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
