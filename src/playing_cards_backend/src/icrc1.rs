use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call;
use ic_cdk::api::call::CallResult;

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: Principal,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct BalanceArgs {
    pub owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    GenericError { error_code: u64, message: String },
}

pub async fn icrc1_transfer(args: TransferArgs) -> CallResult<(Result<Nat, TransferError>,)> {
    let principal = Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai").unwrap();
    let method = "icrc1_transfer";
    call::call(principal, method, (args,)).await
}

pub async fn icrc1_balance_of(args: BalanceArgs) -> CallResult<(Nat,)> {
    let principal = Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai").unwrap();
    let method = "icrc1_balance_of";
    call::call(principal, method, (args,)).await
}

pub async fn icrc1_fee() -> CallResult<(Nat,)> {
    let principal = Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai").unwrap();
    let method = "icrc1_fee";
    call::call(principal, method, ()).await
}
