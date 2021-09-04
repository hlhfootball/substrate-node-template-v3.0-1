use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{OnFinalize, OnInitialize};

pub const KITTY_RESERVE: u128 = 1_000;
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const NOBODY: u64 = 99;

fn run_to_block( n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

#[test]
fn can_create_work() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        // assert_eq!(Balances::total_balance(ALICE), 0);
        //创建Kittiy
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(
            ALICE, 0,
        )));
        //检查总数量
        assert_eq!(KittiesCount::<Test>::get(), 1);
        //检查拥有者
        assert_eq!(Owner::<Test>::get(0), Some(ALICE));
        //检查质押数量
        assert_eq!(Balances::reserved_balance(ALICE), KITTY_RESERVE);
    });
}

#[test]
fn can_create_faile_not_enough_money() {
    new_test_ext().execute_with(|| {
        //检查质押不足时创建Kitty，是否返回正确错误
        assert_noop!(
            KittiesModule::create(Origin::signed(NOBODY)),
            Error::<Test>::MoneyNotEnough
        );
    });
}

#[test]
fn can_transfer_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_eq!(KittiesCount::<Test>::get(), 1);
        //检查质押数量
        assert_eq!(Balances::reserved_balance(ALICE), KITTY_RESERVE);

        //检查转移Kitty
        assert_ok!(KittiesModule::transfer(Origin::signed(ALICE), BOB, 0));
        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyTransfer(
            ALICE, BOB, 0,
        )));
        //检查质押数量
        assert_eq!(Balances::reserved_balance(ALICE), 0);
        //检查质押数量
        assert_eq!(Balances::reserved_balance(BOB), KITTY_RESERVE);
    });
}

#[test]
fn can_transfer_faile_not_enough_money() {
    new_test_ext().execute_with(|| {
        //检查质押不足时创建Kitty，是否返回正确错误
        assert_noop!(
            KittiesModule::create(Origin::signed(NOBODY)),
            Error::<Test>::MoneyNotEnough
        );
    });
}

#[test]
fn can_transfer_failed_not_owner() {
    new_test_ext().execute_with(|| {
        //检查非拥有才转移Kitty，是否返回正确错误
        assert_noop! {
            KittiesModule::transfer(Origin::signed(ALICE),BOB,99),
            Error::<Test>::NotOwner
        }
    });
}

#[test]
fn can_transfer_failed_already_owned() {
    new_test_ext().execute_with(|| {
        //检查转移Kitty给本人，是否返回正确错误
        assert_noop! {
            KittiesModule::transfer(Origin::signed(ALICE),ALICE,0),
            Error::<Test>::AlreadyOwned
        }
    });
}

#[test]
fn can_bread_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        assert_eq!(KittiesCount::<Test>::get(), 2);
        assert_eq!(Owner::<Test>::get(0), Some(ALICE));
        assert_eq!(Owner::<Test>::get(1), Some(ALICE));
        //检查质押数量
        assert_eq!(Balances::reserved_balance(ALICE), 2 * KITTY_RESERVE);

        //检查繁殖Kitty
        assert_ok!(KittiesModule::bread(Origin::signed(ALICE), 0, 1));
        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(
            ALICE, 2,
        )));
        //检查总数量
        assert_eq!(KittiesCount::<Test>::get(), 3);
        //检查拥有者
        assert_eq!(Owner::<Test>::get(2), Some(ALICE));
        //检查质押数量
        assert_eq!(Balances::reserved_balance(ALICE), 3 * KITTY_RESERVE);
    });
}

#[test]
fn can_bread_failed_invalid_same_parent_index() {
    new_test_ext().execute_with(|| {
        //检查父母为同一Kitty，是否返回正确错误
        assert_noop! {
            KittiesModule::bread(Origin::signed(ALICE),1,1),
            Error::<Test>::SameParentIndex
        }
    });
}

#[test]
fn can_bread_failed_invalid_kittyindex() {
    new_test_ext().execute_with(|| {
        //检查父母不存在时，是否返回正确错误
        assert_noop! {
            KittiesModule::bread(Origin::signed(ALICE),0,1),
            Error::<Test>::InvalidKittyIndex
        }
    });
}

#[test]
fn can_sale_work() {
    new_test_ext().execute_with(|| {
        //创建Kittiy
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));

        //挂售Kittiy
        assert_ok!(KittiesModule::sale(Origin::signed(ALICE), 0, Some(5_000)));
        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyForSale(
            ALICE,
            0,
            Some(5_000),
        )));
    });
}

#[test]
fn can_sale_failed_not_owner() {
    new_test_ext().execute_with(|| {
        //检查挂售Kitty非本人时，是否返回正确错误
        assert_noop! {
            KittiesModule::sale(Origin::signed(ALICE),0,Some(5_000)),
            Error::<Test>::NotOwner
        }
    });
}

#[test]
fn can_buy_failed_not_owner() {
    new_test_ext().execute_with(|| {
        //检查购买本人挂售Kitty时，是否返回正确错误
        assert_noop! {
            KittiesModule::buy(Origin::signed(ALICE),99),
            Error::<Test>::NotOwner
        }
    });
}

#[test]
fn can_buy_failed_not_for_sale() {
    new_test_ext().execute_with(|| {
        //创建Kittiy
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));

        //检查购买本人挂售Kitty时，是否返回正确错误
        assert_noop! {
            KittiesModule::buy(Origin::signed(BOB),0),
            Error::<Test>::NotForSale
        }
    });
}

#[test]
fn can_buy_failed_already_owned() {
    new_test_ext().execute_with(|| {
        //创建Kittiy
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        //挂售Kittiy
        assert_ok!(KittiesModule::sale(Origin::signed(ALICE), 0, Some(5_000)));

        //检查购买本人挂售Kitty时，是否返回正确错误
        assert_noop! {
            KittiesModule::buy(Origin::signed(ALICE),0),
            Error::<Test>::AlreadyOwned
        }
    });
}

#[test]
fn can_buy_work() {
    new_test_ext().execute_with(|| {
        //创建Kittiy
        assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
        //挂售Kittiy
        assert_ok!(KittiesModule::sale(Origin::signed(ALICE), 0, Some(8_000)));
        //检查拥有者
        assert_eq!(Owner::<Test>::get(0), Some(ALICE));
        //检查挂单
        assert_eq!(KittyPrices::<Test>::get(0), Some(8_000));

        //购买Kittiy
        assert_ok!(KittiesModule::buy(Origin::signed(BOB), 0));
        //检查事件
        System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittySaleOut(
            BOB,
            0,
            Some(8_000),
        )));

        //检查是否已经收到转账
        assert_eq!(Balances::free_balance(ALICE), 10_000 + 8_000);
        //检查是否已经转出
        assert_eq!(
            Balances::free_balance(BOB),
            20_000 - 8_000 - KITTY_RESERVE
        );

        //检查原拥有者质押数量
        assert_eq!(Balances::reserved_balance(ALICE), 0);
        //检查新拥有者质押数量
        assert_eq!(Balances::reserved_balance(BOB), KITTY_RESERVE);

        //检查拥有者
        assert_eq!(Owner::<Test>::get(0), Some(BOB));
        //检查挂单
        assert_eq!(KittyPrices::<Test>::get(0), None);
    });
}