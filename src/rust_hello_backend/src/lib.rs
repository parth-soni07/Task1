use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;
use ic_cdk::export::Principal;

#[derive(CandidType, Deserialize, Clone)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub total_supply: u64,
    pub owner: Principal,
}

pub struct TokenSwap {
    balances: HashMap<Principal, u64>,
    total_supply: u64,
}

impl TokenSwap {
    pub fn new(owner: Principal, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner, total_supply);
        Self {
            balances,
            total_supply,
        }
    }

    pub fn balance_of(&self, user: Principal) -> u64 {
        *self.balances.get(&user).unwrap_or(&0)
    }
    pub fn transfer(&mut self, from: Principal, to: Principal, amount: u64) -> Result<(), String> {
        let from_balance = self.balances.get(&from).unwrap_or(&0);
        if *from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        *self.balances.entry(from).or_insert(0) -= amount;
        *self.balances.entry(to).or_insert(0) += amount;
        Ok(())
    }
}

thread_local! {
    static TOKEN_SWAP: std::cell::RefCell<Option<TokenSwap>> = std::cell::RefCell::new(None);
}

#[ic_cdk_macros::update]
fn init_token(symbol: String, name: String, total_supply: u64) {
    let owner = ic_cdk::caller();
    TOKEN_SWAP.with(|swap| {
        *swap.borrow_mut() = Some(TokenSwap::new(owner, total_supply));
    });
}
#[ic_cdk_macros::update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let from = ic_cdk::caller();
    TOKEN_SWAP.with(|swap| {
        if let Some(ref mut s) = swap.borrow_mut().as_mut() {
            s.transfer(from, to, amount)
        } else {
            Err("Token not initialized".to_string())
        }
    })
}

#[ic_cdk_macros::query]
fn get_balance(user: Principal) -> u64 {
    TOKEN_SWAP.with(|swap| {
        if let Some(s) = swap.borrow().as_ref() {
            s.balance_of(user)
        } else {
            0
        }
    })
}


