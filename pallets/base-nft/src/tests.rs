//! Unit tests for the non-fungible-token module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;

#[test]
fn create_class_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
	});
}

#[test]
fn create_class_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		NextClassId::<Runtime>::mutate(|id| *id = <Runtime as Config>::ClassId::max_value());
		assert_noop!(
			NonFungibleTokenModule::create_class(&ALICE, vec![1], ()),
			Error::<Runtime>::NoAvailableClassId
		);
	});
}

#[test]
fn mint_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_eq!(next_class_id, CLASS_ID);
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 0);
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 1);
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);

		let next_class_id = NonFungibleTokenModule::next_class_id();
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 0);
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			next_class_id,
			vec![1],
			()
		));
		assert_eq!(NonFungibleTokenModule::next_token_id(next_class_id), 1);

		assert_eq!(NonFungibleTokenModule::next_token_id(CLASS_ID), 2);
	});
}

#[test]
fn mint_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		Classes::<Runtime>::mutate(CLASS_ID, |class_info| {
			class_info.as_mut().unwrap().total_issuance = <Runtime as Config>::TokenId::max_value();
		});
		assert_noop!(
			NonFungibleTokenModule::mint([&BOB].to_vec(), CLASS_ID, vec![1], ()),
			Error::<Runtime>::NumOverflow
		);

		NextTokenId::<Runtime>::mutate(CLASS_ID, |id| {
			*id = <Runtime as Config>::TokenId::max_value()
		});
		assert_noop!(
			NonFungibleTokenModule::mint([&BOB].to_vec(), CLASS_ID, vec![1], ()),
			Error::<Runtime>::NoAvailableTokenId
		);
	});
}

#[test]
fn transfer_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_ok!(NonFungibleTokenModule::transfer(
			[&BOB].to_vec(),
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID),
			75
		));
		assert_ok!(NonFungibleTokenModule::transfer(
			[&BOB].to_vec(),
			[&ALICE].to_vec(),
			(CLASS_ID, TOKEN_ID),
			25
		));
		assert_ok!(NonFungibleTokenModule::transfer(
			[&ALICE].to_vec(),
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID),
			50
		));
		assert!(NonFungibleTokenModule::is_owner(
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID)
		));
	});
}

#[test]
fn transfer_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_noop!(
			NonFungibleTokenModule::transfer(
				[&BOB].to_vec(),
				[&ALICE].to_vec(),
				(CLASS_ID, TOKEN_ID_NOT_EXIST),
				30
			),
			Error::<Runtime>::TokenNotFound
		);
		assert_noop!(
			NonFungibleTokenModule::transfer(
				[&ALICE].to_vec(),
				[&BOB].to_vec(),
				(CLASS_ID, TOKEN_ID),
				40
			),
			Error::<Runtime>::NoPermission
		);
		assert_noop!(
			NonFungibleTokenModule::mint([&BOB].to_vec(), CLASS_ID_NOT_EXIST, vec![1], ()),
			Error::<Runtime>::ClassNotFound
		);
		assert_noop!(
			NonFungibleTokenModule::transfer(
				[&ALICE].to_vec(),
				[&ALICE].to_vec(),
				(CLASS_ID, TOKEN_ID),
				25
			),
			Error::<Runtime>::NoPermission
		);
	});
}

#[test]
fn burn_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_ok!(NonFungibleTokenModule::burn(
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID)
		));
	});
}

#[test]
fn burn_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_noop!(
			NonFungibleTokenModule::burn([&BOB].to_vec(), (CLASS_ID, TOKEN_ID_NOT_EXIST)),
			Error::<Runtime>::TokenNotFound
		);

		assert_noop!(
			NonFungibleTokenModule::burn([&ALICE].to_vec(), (CLASS_ID, TOKEN_ID)),
			Error::<Runtime>::NoPermission
		);
	});

	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));

		Classes::<Runtime>::mutate(CLASS_ID, |class_info| {
			class_info.as_mut().unwrap().total_issuance = 0;
		});
		assert_noop!(
			NonFungibleTokenModule::burn([&BOB].to_vec(), (CLASS_ID, TOKEN_ID)),
			Error::<Runtime>::NumOverflow
		);
	});
}

#[test]
fn destroy_class_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_ok!(NonFungibleTokenModule::burn(
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID)
		));
		assert_ok!(NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID));
		assert_eq!(Classes::<Runtime>::contains_key(CLASS_ID), false);
		assert_eq!(NextTokenId::<Runtime>::contains_key(CLASS_ID), false);
	});
}

#[test]
fn destroy_class_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NonFungibleTokenModule::create_class(&ALICE, vec![1], ()));
		assert_ok!(NonFungibleTokenModule::mint(
			[&BOB].to_vec(),
			CLASS_ID,
			vec![1],
			()
		));
		assert_noop!(
			NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID_NOT_EXIST),
			Error::<Runtime>::ClassNotFound
		);

		assert_noop!(
			NonFungibleTokenModule::destroy_class(&BOB, CLASS_ID),
			Error::<Runtime>::NoPermission
		);

		assert_noop!(
			NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID),
			Error::<Runtime>::CannotDestroyClass
		);

		assert_ok!(NonFungibleTokenModule::burn(
			[&BOB].to_vec(),
			(CLASS_ID, TOKEN_ID)
		));
		assert_ok!(NonFungibleTokenModule::destroy_class(&ALICE, CLASS_ID));
		assert_eq!(Classes::<Runtime>::contains_key(CLASS_ID), false);
	});
}
