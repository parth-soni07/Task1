use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;
use ic_cdk::export::Principal;

#[derive(CandidType, Deserialize, Clone)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub total_supply: u64,
    pub owner: Principal,
    pub decimals: u8,
}

pub struct TokenSimple {
    balances: HashMap<Principal, u64>,
    total_supply: u64,
    decimals: u8,
    name: String,
    symbol: String,
}

impl TokenSimple {
    pub fn new(owner: Principal, total_supply: u64, decimals: u8, name: String, symbol: String) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner, total_supply); // Assign the total supply to the owner.
        Self {
            balances,
            total_supply,
            decimals,
            name,
            symbol,
        }
    }

    pub fn balance_of(&self, user: Principal) -> u64 {
        *self.balances.get(&user).unwrap_or(&0)
    }

    pub fn total_supply(&self) -> u64 {
        self.total_supply
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
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
    static TOKEN_SIMPLE: std::cell::RefCell<Option<TokenSimple>> = std::cell::RefCell::new(None);
}

#[ic_cdk_macros::update]
fn init_token(symbol: String, name: String, total_supply: u64, decimals: u8) {
    let owner = ic_cdk::caller();
    TOKEN_SIMPLE.with(|token| {
        *token.borrow_mut() = Some(TokenSimple::new(owner, total_supply, decimals, name, symbol));
    });
}

#[ic_cdk_macros::query]
fn balance_of(user: Principal) -> u64 {
    TOKEN_SIMPLE.with(|token| {
        if let Some(t) = token.borrow().as_ref() {
            t.balance_of(user)
        } else {
            0
        }
    })
}

#[ic_cdk_macros::query]
fn total_supply() -> u64 {
    TOKEN_SIMPLE.with(|token| {
        if let Some(t) = token.borrow().as_ref() {
            t.total_supply()
        } else {
            0
        }
    })
}

#[ic_cdk_macros::query]
fn symbol() -> String {
    TOKEN_SIMPLE.with(|token| {
        if let Some(t) = token.borrow().as_ref() {
            t.symbol()
        } else {
            "".to_string()
        }
    })
}

#[ic_cdk_macros::query]
fn name() -> String {
    TOKEN_SIMPLE.with(|token| {
        if let Some(t) = token.borrow().as_ref() {
            t.name()
        } else {
            "".to_string()
        }
    })
}

#[ic_cdk_macros::query]
fn decimals() -> u8 {
    TOKEN_SIMPLE.with(|token| {
        if let Some(t) = token.borrow().as_ref() {
            t.decimals()
        } else {
            0
        }
    })
}

#[ic_cdk_macros::update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let from = ic_cdk::caller();
    TOKEN_SIMPLE.with(|token| {
        if let Some(ref mut t) = token.borrow_mut().as_mut() {
            t.transfer(from, to, amount)
        } else {
            Err("Token not initialized".to_string())
        }
    })
}

#[ic_cdk_macros::query]
fn whoami() -> Principal {
    ic_cdk::caller()
}