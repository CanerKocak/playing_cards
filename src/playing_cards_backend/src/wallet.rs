// wallet.rs
use super::*;
use candid::{Nat, Principal};
use ic_cdk::api;
use ic_ledger_types::Subaccount;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferArg, TransferError};

const WINDOGE98: &str = "rh2pm-ryaaa-aaaan-qeniq-cai";

async fn get_transfer_fee() -> Result<Nat, TransferError> {
    let ledger_canister_id = Principal::from_text(WINDOGE98).unwrap();
    let (fee,): (Nat,) = api::call::call(ledger_canister_id, "icrc1_fee", ())
        .await
        .map_err(|_| TransferError::GenericError {
            error_code: Nat::from(0u64),
            message: "Failed to fetch transfer fee".to_string(),
        })?;
    Ok(fee)
}

#[query]
fn whoami() -> Principal {
    api::caller()
}

#[query]
fn whoamisub() -> Subaccount {
    Subaccount::try_from(api::caller()).unwrap()
}

async fn icrc1_transfer(transfer_args: TransferArg) -> Result<BlockIndex, TransferError> {
    let ledger_canister_id = Principal::from_text(WINDOGE98).unwrap();

    let error_code = Nat::from(0u64); // Use u64 to construct Nat

    let (result,): (Result<BlockIndex, TransferError>,) =
        api::call::call(ledger_canister_id, "transfer", (transfer_args,))
            .await
            .map_err(|_| TransferError::GenericError {
                error_code: error_code.clone(),
                message: "Call failed".to_string(),
            })?;

    result
}

fn dapp_to_user_wallet_args(amount: u64) -> TransferArg {
    let caller = api::caller();
    let subaccount = Subaccount::try_from(caller).unwrap();

    TransferArg {
        from_subaccount: Some(subaccount.0),
        to: Account::try_from(caller).unwrap(),
        fee: None,
        memo: None,
        created_at_time: None,
        amount: Nat::from(amount),
    }
}

fn dapp_to_wallet_args(to: Principal, amount: u64) -> TransferArg {
    let caller = api::caller();
    let subaccount = Subaccount::try_from(caller).unwrap();

    TransferArg {
        from_subaccount: Some(subaccount.0),
        to: Account::try_from(to).unwrap(),
        fee: None,
        memo: None,
        created_at_time: None,
        amount: Nat::from(amount),
    }
}

#[update]
async fn send_exe_to_wallet(amount: u64) -> Result<BlockIndex, Error> {
    let transfer_args = dapp_to_user_wallet_args(amount);

    let block_index = icrc1_transfer(transfer_args).await.map_err(|e| match e {
        TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
    })?;

    Ok(block_index)
}

#[update]
async fn transfer_exe(to: Principal, amount: u64) -> Result<BlockIndex, Error> {
    let transfer_args = dapp_to_wallet_args(to, amount);

    let block_index = icrc1_transfer(transfer_args).await.map_err(|e| match e {
        TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
    })?;

    Ok(block_index)
}
