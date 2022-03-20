use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;
#[test]
fn should_working_create_kitty() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
	});
}
#[test]
fn should_working_set_price() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		SubstrateKitties::create_kitty(Origin::signed(1));
		let id = KittiesOwned::<Test>::get(1)[0];
		assert_ok!(SubstrateKitties::set_price(Origin::signed(1), id, Some(100)));
	});
}

#[test]
fn can_not_set_price() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		SubstrateKitties::create_kitty(Origin::signed(1));
		let id = KittiesOwned::<Test>::get(1)[0];
		assert_noop!(
			SubstrateKitties::set_price(Origin::signed(2), id, Some(100)),
			Error::<Test>::NotKittyOwner
		);
	});
}

#[test]
fn should_working_buy() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		SubstrateKitties::create_kitty(Origin::signed(1));
		let id = KittiesOwned::<Test>::get(1)[0];
		SubstrateKitties::set_price(Origin::signed(1), id, Some(0));
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(2), id, 0));
	});
}	
#[test]
fn should_working_buy_to_low() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		SubstrateKitties::create_kitty(Origin::signed(1));
		let id = KittiesOwned::<Test>::get(1)[0];
		SubstrateKitties::set_price(Origin::signed(1), id, Some(100));
		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(2), id, 2),
			Error::<Test>::KittyBidPriceTooLow
		);
	});
}	