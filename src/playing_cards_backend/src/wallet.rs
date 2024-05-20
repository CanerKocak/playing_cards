// wallet.rs
use super::*;
use ic_cdk::api;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub to_account: Account,
    pub amount: Nat,
}

// function that sends tokens from subaccount to wallet

// function that transfers tokens from subaccount to subaccount

// function that returns the nfts the user has
