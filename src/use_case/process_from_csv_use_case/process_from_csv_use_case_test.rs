use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::domain::account_module::account::Account;
use crate::domain::payments_engine::PaymentsEngine;
use crate::use_case::process_from_csv_use_case::process_from_csv;

#[tokio::test]
async fn test_csv_processing_use_case_against_comprehensive_fixture() {
    let payments_engine= PaymentsEngine::new();

    let result = process_from_csv("fixtures/comprehensive_test_with_errors.csv".to_string(), payments_engine).await.await.unwrap().unwrap();
    let mut resulting_accounts: Vec<&Account> = result.accounts().collect();
    resulting_accounts.sort_by_key(|account| account.id());

    let expected = [
        (1, -0.5,    0.0,       -0.5,    true),
        (2, 2.0,     0.0,       2.0,     false),
        (3, 5.0,     0.0,       5.0,     false),
        (4, 1001.0,  12.0,      1013.0,  false),
        (5, 123456789.1239, 0.0, 123456789.1239, false),
    ];

    for (account, &(expected_account_id, expected_available_amount, expected_held_amount, expected_total_amount, expected_account_frozen)) in resulting_accounts.iter().zip(&expected) {
        assert_eq!(account.id(), expected_account_id);
        assert_eq!(account.available_amount(), Decimal::from_f64(expected_available_amount).unwrap());
        assert_eq!(account.held_amount(), Decimal::from_f64(expected_held_amount).unwrap());
        assert_eq!(account.total_amount(), Decimal::from_f64(expected_total_amount).unwrap());
        assert_eq!(matches!(account, Account::Frozen(_)), expected_account_frozen);
    }

    assert_eq!(resulting_accounts.len(), 5);
}