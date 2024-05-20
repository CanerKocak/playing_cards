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

    let (result,): (Result<BlockIndex, TransferError>,) =
        api::call::call(ledger_canister_id, "icrc1_transfer", (transfer_args,))
            .await
            .map_err(|_| TransferError::GenericError {
                error_code: Nat::from(0u64),
                message: "Call failed".to_string(),
            })?;

    result
}

fn dapp_to_user_wallet_args(amount: Nat) -> TransferArg {
    let caller = api::caller();
    let subaccount = Subaccount::try_from(caller);
    let subaccount = subaccount.map(|sa| sa.0).ok();

    TransferArg {
        from_subaccount: subaccount,
        to: Account {
            owner: caller,
            subaccount: None,
        },
        fee: None,
        memo: None,
        created_at_time: None,
        amount,
    }
}

fn dapp_to_wallet_args(to: Principal, amount: Nat) -> TransferArg {
    let dapp_principal = api::id();

    TransferArg {
        from_subaccount: Subaccount::try_from(dapp_principal).map(|sa| sa.0).ok(),
        to: Account {
            owner: to,
            subaccount: None,
        },
        fee: None,
        memo: None,
        created_at_time: None,
        amount,
    }
}

async fn get_subaccount_balance(subaccount: Subaccount) -> Result<Nat, TransferError> {
    let ledger_canister_id = Principal::from_text(WINDOGE98).unwrap();
    let account = Account {
        owner: Principal::from_slice(&api::id().as_slice()),
        subaccount: Some(subaccount.0),
    };
    let (balance,): (Nat,) = api::call::call(ledger_canister_id, "icrc1_balance_of", (account,))
        .await
        .map_err(|_| TransferError::GenericError {
            error_code: Nat::from(0u64),
            message: "Failed to fetch balance".to_string(),
        })?;
    Ok(balance)
}

#[update]
async fn send_exe_to_wallet() -> Result<BlockIndex, Error> {
    let caller: Principal = api::caller();
    let subaccount = Subaccount::try_from(caller).unwrap();

    let balance = get_subaccount_balance(subaccount)
        .await
        .map_err(|e| match e {
            TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
        })?;

    let fee = get_transfer_fee().await.map_err(|e| match e {
        TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
    })?;

    let amount = balance - fee;

    let transfer_args = dapp_to_user_wallet_args(amount);

    let block_index = icrc1_transfer(transfer_args).await.map_err(|e| match e {
        TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
    })?;

    Ok(block_index)
}

#[update]
async fn transfer_exe(to: Principal, amount: u64) -> Result<BlockIndex, Error> {
    let amount_nat = Nat::from(amount);
    let transfer_args = dapp_to_wallet_args(to, amount_nat);

    let block_index = icrc1_transfer(transfer_args).await.map_err(|e| match e {
        TransferError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailed(format!("Transfer failed: {:?}", e)),
    })?;

    Ok(block_index)
}
