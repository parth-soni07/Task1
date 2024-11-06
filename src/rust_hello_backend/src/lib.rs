mod icrc2;
// mod icrc2;

use icrc2::*;
// use icrc2::{TokenExtended, Allowance, TransactionLog};

// use ic_cdk::export::Principal;
// use std::cell::RefCell;

// thread_local! {
//     static TOKEN_SIMPLE: RefCell<Option<TokenSimple>> = RefCell::new(None);
//     static TOKEN_EXTENDED: RefCell<Option<TokenExtended>> = RefCell::new(None);
// }

// #[ic_cdk_macros::update]
// fn init_token(symbol: String, name: String, total_supply: u64, decimals: u8) {
//     let owner = ic_cdk::caller();
//     TOKEN_SIMPLE.with(|token| {
//         *token.borrow_mut() = Some(TokenSimple::new(owner, total_supply, decimals, name, symbol));
//     });
// }

// #[ic_cdk_macros::update]
// fn init_token_extended(symbol: String, name: String, total_supply: u64, decimals: u8, fee: u64, url: String) {
//     let owner = ic_cdk::caller();
//     let base_token = TokenSimple::new(owner, total_supply, decimals, name, symbol);
//     TOKEN_EXTENDED.with(|token| {
//         *token.borrow_mut() = Some(TokenExtended::new(base_token));
//     });
// }

// #[ic_cdk_macros::update]
// fn approve(spender: Principal, amount: u64) -> Result<(), String> {
//     let owner = ic_cdk::caller();
//     TOKEN_EXTENDED.with(|token| {
//         token.borrow_mut().as_mut().ok_or("Token not initialized".to_string())?
//             .approve(owner, spender, amount)
//     })
// }

// #[ic_cdk_macros::query]
// fn allowance(owner: Principal, spender: Principal) -> u64 {
//     TOKEN_EXTENDED.with(|token| {
//         token.borrow().as_ref().map(|t| t.allowance(owner, spender)).unwrap_or(0)
//     })
// }

// #[ic_cdk_macros::update]
// fn transfer_from(owner: Principal, to: Principal, amount: u64) -> Result<(), String> {
//     let spender = ic_cdk::caller();
//     TOKEN_EXTENDED.with(|token| {
//         token.borrow_mut().as_mut().ok_or("Token not initialized".to_string())?
//             .transfer_from(owner, spender, to, amount)
//     })
// }

// #[ic_cdk_macros::query]
// fn get_transaction_logs() -> Vec<TransactionLog> {
//     TOKEN_EXTENDED.with(|token| {
//         token.borrow().as_ref().map(|t| t.get_transaction_logs().clone()).unwrap_or_else(Vec::new)
//     })
// }
