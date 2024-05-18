#![allow(clippy::collapsible_else_if)]

#[macro_use]
extern crate ic_cdk_macros;
extern crate serde;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::mem;
use std::num::TryFromIntError;
use std::result::Result as StdResult;

use candid::{CandidType, Deserialize, Encode, Nat, Principal};
use ic_cdk::api::call::ArgDecoderConfig;
use ic_cdk::{
    api::{self, call},
    storage,
};
use ic_certified_map::Hash;
use include_base64::include_base64;

use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;

mod http;
mod mint_default_cards;

// use mint_default_cards::mint_all_default_cards;

const MGMT: Principal = Principal::from_slice(&[]);

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

type Tokens = u64;

// type TransferResult = Result<u64, TransferError>;

#[derive(CandidType, Deserialize)]
struct StableState {
    state: State,
    hashes: Vec<(String, Hash)>,
}

#[pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|state| mem::take(&mut *state.borrow_mut()));
    let hashes = http::HASHES.with(|hashes| mem::take(&mut *hashes.borrow_mut()));
    let hashes = hashes.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let stable_state = StableState { state, hashes };
    storage::stable_save((stable_state,)).unwrap();
}
#[post_upgrade]
fn post_upgrade() {
    let (StableState { state, hashes },) = storage::stable_restore().unwrap();
    STATE.with(|state0| *state0.borrow_mut() = state);
    let hashes = hashes.into_iter().collect();
    http::HASHES.with(|hashes0| *hashes0.borrow_mut() = hashes);
}

#[derive(CandidType, Deserialize)]
struct InitArgs {
    custodians: Option<HashSet<Principal>>,
    logo: Option<LogoResult>,
    name: String,
    symbol: String,
}

fn create_metadata(
    purpose: MetadataPurpose,
    content_type: &str,
    location_type: u8,
    data: Vec<u8>,
) -> MetadataDesc {
    vec![MetadataPart {
        purpose,
        data,
        key_val_data: vec![
            (
                "contentType".to_string(),
                MetadataVal::TextContent(content_type.to_string()),
            ),
            (
                "locationType".to_string(),
                MetadataVal::Nat8Content(location_type),
            ),
        ]
        .into_iter()
        .collect(),
    }]
}

#[init]
fn init(args: InitArgs) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.custodians = args
            .custodians
            .unwrap_or_else(|| HashSet::from_iter([api::caller()]));
        state.name = args.name;
        state.symbol = args.symbol;
        state.logo = args.logo;
    });
}

#[derive(CandidType, Deserialize)]
enum Error {
    Unauthorized,
    Unauthorized2(String),
    InvalidTokenId,
    ZeroAddress,
    InsufficientBalance,
    NFTNotForSale,
    BidderAlreadyPlacedBid,
    BidderHasNotPlacedBid,
    TransferFailed(String),
    BalanceRetrievalFailed,
    InsufficientPrepaidBalance,
    PrepaidBalanceRetrievalFailed,
    Other,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::Unauthorized2(msg) => write!(f, "Unauthorized: {}", msg),
            Error::InvalidTokenId => write!(f, "Invalid token ID"),
            Error::ZeroAddress => write!(f, "Zero address"),
            Error::InsufficientBalance => write!(f, "Insufficient balance"),
            Error::NFTNotForSale => write!(f, "NFT not for sale"),
            Error::BidderAlreadyPlacedBid => write!(f, "Bidder has already placed a bid"),
            Error::BidderHasNotPlacedBid => write!(f, "Bidder has not placed a bid"),
            Error::TransferFailed(msg) => write!(f, "Transfer failed: {}", msg),
            Error::BalanceRetrievalFailed => write!(f, "Balance retrieval failed"),
            Error::InsufficientPrepaidBalance => write!(f, "Insufficient prepaid balance"),
            Error::PrepaidBalanceRetrievalFailed => write!(f, "Prepaid balance retrieval failed"),
            Error::Other => write!(f, "Other error"),
        }
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidTokenId
    }
}

type Result<T = u128, E = Error> = StdResult<T, E>;

// --------------
// base interface
// --------------

#[query(name = "balanceOfDip721")]
fn balance_of(user: Principal) -> u64 {
    STATE.with(|state| {
        state
            .borrow()
            .nfts
            .iter()
            .filter(|n| n.owner == user)
            .count() as u64
    })
}

#[query(name = "ownerOfDip721")]
fn owner_of(token_id: u64) -> Result<Principal> {
    STATE.with(|state| {
        let owner = state
            .borrow()
            .nfts
            .get(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?
            .owner;
        Ok(owner)
    })
}

#[update(name = "transferFromDip721")]
fn transfer_from(from: Principal, to: Principal, token_id: u64) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = &mut *state;
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;
        let caller = api::caller();
        if nft.owner != caller
            && nft.approved != Some(caller)
            && !state
                .operators
                .get(&from)
                .map(|s| s.contains(&caller))
                .unwrap_or(false)
            && !state.custodians.contains(&caller)
        {
            Err(Error::Unauthorized)
        } else if nft.owner != from {
            Err(Error::Other)
        } else {
            nft.approved = None;
            nft.owner = to;
            Ok(state.next_txid())
        }
    })
}

#[update(name = "safeTransferFromDip721")]
fn safe_transfer_from(from: Principal, to: Principal, token_id: u64) -> Result {
    if to == MGMT {
        Err(Error::ZeroAddress)
    } else {
        transfer_from(from, to, token_id)
    }
}

#[query(name = "supportedInterfacesDip721")]
fn supported_interfaces() -> &'static [InterfaceId] {
    &[
        InterfaceId::TransferNotification,
        // InterfaceId::Approval, // Psychedelic/DIP721#5
        InterfaceId::Burn,
        InterfaceId::Mint,
    ]
}

#[derive(CandidType, Deserialize, Clone)]
struct LogoResult {
    logo_type: Cow<'static, str>,
    data: Cow<'static, str>,
}

#[export_name = "canister_query logoDip721"]
fn logo() /* -> &'static LogoResult */
{
    ic_cdk::setup();
    STATE.with(|state| call::reply((state.borrow().logo.as_ref().unwrap_or(&DEFAULT_LOGO),)))
}

#[query(name = "nameDip721")]
fn name() -> String {
    STATE.with(|state| state.borrow().name.clone())
}

#[query(name = "symbolDip721")]
fn symbol() -> String {
    STATE.with(|state| state.borrow().symbol.clone())
}

const DEFAULT_LOGO: LogoResult = LogoResult {
    data: Cow::Borrowed(include_base64!("logo.png")),
    logo_type: Cow::Borrowed("image/png"),
};

#[query(name = "totalSupplyDip721")]
fn total_supply() -> u64 {
    STATE.with(|state| state.borrow().nfts.len() as u64)
}

#[export_name = "canister_query getMetadataDip721"]
fn get_metadata(/* token_id: u64 */) /* -> Result<&'static MetadataDesc> */
{
    ic_cdk::setup();
    let token_id = call::arg_data::<(u64,)>(ArgDecoderConfig::default()).0;
    let res: Result<()> = STATE.with(|state| {
        let state = state.borrow();
        let metadata = &state
            .nfts
            .get(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?
            .metadata;
        call::reply((Ok::<_, Error>(metadata),));
        Ok(())
    });
    if let Err(e) = res {
        call::reply((Err::<MetadataDesc, _>(e),));
    }
}

#[derive(CandidType)]
struct ExtendedMetadataResult<'a> {
    metadata_desc: MetadataDescRef<'a>,
    token_id: u64,
}

#[export_name = "canister_update getMetadataForUserDip721"]
fn get_metadata_for_user(/* user: Principal */) /* -> Vec<ExtendedMetadataResult> */
{
    ic_cdk::setup();
    let user = call::arg_data::<(Principal,)>(ArgDecoderConfig::default()).0;
    STATE.with(|state| {
        let state = state.borrow();
        let metadata: Vec<_> = state
            .nfts
            .iter()
            .filter(|n| n.owner == user)
            .map(|n| ExtendedMetadataResult {
                metadata_desc: &n.metadata,
                token_id: n.id,
            })
            .collect();
        call::reply((metadata,));
    });
}

// ----------------------
// notification interface
// ----------------------

#[update(name = "transferFromNotifyDip721")]
fn transfer_from_notify(from: Principal, to: Principal, token_id: u64, data: Vec<u8>) -> Result {
    let res = transfer_from(from, to, token_id)?;
    if let Ok(arg) = Encode!(&api::caller(), &from, &token_id, &data) {
        // Using call_raw ensures we don't need to await the future for the call to be executed.
        // Calling an arbitrary function like this means that a malicious recipient could call
        // transferFromNotifyDip721 in their onDIP721Received function, resulting in an infinite loop.
        // This will trap eventually, but the transfer will have already been completed and the state-change persisted.
        // That means the original transfer must reply before that happens, or the caller will be
        // convinced that the transfer failed when it actually succeeded. So we don't await the call,
        // so that we'll reply immediately regardless of how long the notification call takes.
        let _ = api::call::call_raw(to, "onDIP721Received", arg, 0);
    }
    Ok(res)
}

#[update(name = "safeTransferFromNotifyDip721")]
fn safe_transfer_from_notify(
    from: Principal,
    to: Principal,
    token_id: u64,
    data: Vec<u8>,
) -> Result {
    if to == MGMT {
        Err(Error::ZeroAddress)
    } else {
        transfer_from_notify(from, to, token_id, data)
    }
}

// ------------------
// approval interface
// ------------------

#[update(name = "approveDip721")]
fn approve(user: Principal, token_id: u64) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = &mut *state;
        let caller = api::caller();
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;
        if nft.owner != caller
            && nft.approved != Some(caller)
            && !state
                .operators
                .get(&user)
                .map(|s| s.contains(&caller))
                .unwrap_or(false)
            && !state.custodians.contains(&caller)
        {
            Err(Error::Unauthorized)
        } else {
            nft.approved = Some(user);
            Ok(state.next_txid())
        }
    })
}

#[update(name = "setApprovalForAllDip721")]
fn set_approval_for_all(operator: Principal, is_approved: bool) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let caller = api::caller();
        if operator != caller {
            let operators = state.operators.entry(caller).or_default();
            if operator == MGMT {
                if !is_approved {
                    operators.clear();
                } else {
                    // cannot enable everyone as an operator
                }
            } else {
                if is_approved {
                    operators.insert(operator);
                } else {
                    operators.remove(&operator);
                }
            }
        }
        Ok(state.next_txid())
    })
}

// #[query(name = "getApprovedDip721")] // Psychedelic/DIP721#5
fn _get_approved(token_id: u64) -> Result<Principal> {
    STATE.with(|state| {
        let approved = state
            .borrow()
            .nfts
            .get(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?
            .approved
            .unwrap_or_else(api::caller);
        Ok(approved)
    })
}

#[query(name = "isApprovedForAllDip721")]
fn is_approved_for_all(operator: Principal) -> bool {
    STATE.with(|state| {
        state
            .borrow()
            .operators
            .get(&api::caller())
            .map(|s| s.contains(&operator))
            .unwrap_or(false)
    })
}

// --------------
// mint interface
// --------------

#[update(name = "mintDip721")]
fn mint(
    to: Principal,
    metadata: MetadataDesc,
    blob_content: Vec<u8>,
) -> Result<MintResult, ConstrainedError> {
    let (txid, tkid) = STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.custodians.contains(&api::caller()) {
            return Err(ConstrainedError::Unauthorized);
        }
        let new_id = state.nfts.len() as u64;
        let nft = Nft {
            owner: to,
            approved: None,
            id: new_id,
            metadata,
            content: blob_content,
        };
        state.nfts.push(nft);
        Ok((state.next_txid(), new_id))
    })?;
    http::add_hash(tkid);
    Ok(MintResult {
        id: txid,
        token_id: tkid,
    })
}

// --------------
// burn interface
// --------------

#[update(name = "burnDip721")]
fn burn(token_id: u64) -> Result {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;
        if nft.owner != api::caller() {
            Err(Error::Unauthorized)
        } else {
            nft.owner = MGMT;
            Ok(state.next_txid())
        }
    })
}

#[derive(CandidType, Deserialize, Default, Clone)]
struct State {
    nfts: Vec<Nft>,
    custodians: HashSet<Principal>,
    operators: HashMap<Principal, HashSet<Principal>>, // owner to operators
    logo: Option<LogoResult>,
    name: String,
    symbol: String,
    txid: u128,
    sale_listings: HashMap<u64, SaleListing>, // token_id to SaleListing
    icrc1_canister_id: Option<String>,        // Canister ID of the ICRC1 canister
}

#[derive(CandidType, Deserialize, Clone)]
struct Nft {
    owner: Principal,
    approved: Option<Principal>,
    id: u64,
    metadata: MetadataDesc,
    content: Vec<u8>,
}

type MetadataDesc = Vec<MetadataPart>;
type MetadataDescRef<'a> = &'a [MetadataPart];

#[derive(CandidType, Deserialize, Clone)]
struct MetadataPart {
    purpose: MetadataPurpose,
    key_val_data: HashMap<String, MetadataVal>,
    data: Vec<u8>,
}

#[derive(CandidType, Deserialize, PartialEq, Clone)]
enum MetadataPurpose {
    Preview,
    Rendered,
}

#[derive(CandidType, Deserialize)]
struct MintResult {
    token_id: u64,
    id: u128,
}

#[allow(clippy::enum_variant_names)]
#[derive(CandidType, Deserialize, Clone)]
enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
}

impl State {
    fn next_txid(&mut self) -> u128 {
        let txid = self.txid;
        self.txid += 1;
        txid
    }
}

#[derive(CandidType, Deserialize)]
enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}

#[derive(CandidType, Deserialize, Debug)]
enum ConstrainedError {
    Unauthorized,
}

#[update]
fn set_name(name: String) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            state.name = name;
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    })
}

#[update]
fn set_symbol(sym: String) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            state.symbol = sym;
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    })
}

#[update]
fn set_logo(logo: Option<LogoResult>) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            state.logo = logo;
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    })
}

#[update]
fn set_custodian(user: Principal, custodian: bool) -> Result<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.custodians.contains(&api::caller()) {
            if custodian {
                state.custodians.insert(user);
            } else {
                state.custodians.remove(&user);
            }
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    })
}

#[query]
fn is_custodian(principal: Principal) -> bool {
    STATE.with(|state| state.borrow().custodians.contains(&principal))
}

/// This makes this Candid service self-describing, so that for example Candid UI, but also other
/// tools, can seamlessly integrate with it. The concrete interface (method name etc.) is
/// provisional, but works.
///
/// without this I couldn't open the web interface of the canister to test it quickly
#[query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../playing_cards_backend.did").to_string()
}

// ----------------------
// fetch all NFTs
// ----------------------
#[query]
fn fetch_nfts() -> Vec<Nft> {
    STATE.with(|state| state.borrow().nfts.clone())
}

// ----------------------
// NFT Sales
// ----------------------

#[derive(CandidType, Deserialize, Clone)]
struct SaleListing {
    token_id: u64,
    seller: Principal,
    price: Tokens,
}

#[update]
fn list_nft_for_sale(token_id: u64, price: Tokens) -> Result<(), Error> {
    let caller = api::caller();
    if owner_of(token_id).map_err(|_| Error::InvalidTokenId)? != caller {
        return Err(Error::Unauthorized);
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();

        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;

        // transfer the NFT to the canister
        nft.owner = api::id();

        state.sale_listings.insert(
            token_id,
            SaleListing {
                token_id,
                seller: caller,
                price,
            },
        );
        Ok(())
    })
}

#[update]
fn remove_sale_listing(token_id: u64) -> Result<(), Error> {
    // from state get sale_listing and the nft
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let sale_listing = state
            .sale_listings
            .remove(&token_id)
            .ok_or(Error::NFTNotForSale)?;

        let nft = state
            .nfts
            .get_mut(usize::try_from(token_id)?)
            .ok_or(Error::InvalidTokenId)?;

        // transfer the NFT back to the seller
        if sale_listing.seller != api::caller() {
            return Err(Error::Unauthorized);
        }

        nft.owner = sale_listing.seller;

        state.sale_listings.remove(&token_id);

        Ok(())
    })
}

// buy NFT
#[derive(CandidType, Deserialize, Clone)]
struct BuyNftArgs {
    token_id: u64,
    amount: Tokens,
}

#[query]
fn list_sale_listings() -> Vec<SaleListing> {
    STATE.with(|state| state.borrow().sale_listings.values().cloned().collect())
}

#[query]
fn whoami() -> Principal {
    api::caller()
}

#[query]
fn whoami_string() -> String {
    api::caller().to_string()
}

use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferError};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TransferArgs {
    amount: NumTokens,
    to_account: Account,
}

// ----------------------
// windoge98 EXE transfer
// ----------------------

#[update]
async fn transfer_from_canister(args: TransferArgs) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens to account {}",
        &args.amount,
        &args.to_account,
    );

    let transfer_args: TransferArg = TransferArg {
        memo: None,
        amount: args.amount,
        from_subaccount: None,
        fee: None,
        to: args.to_account,
        created_at_time: None,
    };

    ic_cdk::call::<(TransferArg,), (Result<BlockIndex, TransferError>,)>(
        Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai")
            .expect("Could not decode the principal."),
        "icrc1_transfer",
        (transfer_args,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[derive(CandidType, Deserialize)]
struct RecordAccount {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
type TransferFromResult = Result<BlockIndex, TransferFromError>;

#[update]
async fn transfer_from_caller(
    caller: Principal, // Add the caller principal as an argument
    args: TransferArgs,
) -> Result<BlockIndex, String> {
    ic_cdk::println!(
        "Transferring {} tokens from caller {:?} to account {:?}",
        args.amount,
        caller, // Use the provided caller principal
        args.to_account.owner,
    );

    let from_account = Account {
        owner: caller, // Use the provided caller principal
        subaccount: None,
    };

    let transfer_from_arg = TransferFromArgs {
        spender_subaccount: None,
        from: from_account,
        to: args.to_account,
        amount: args.amount,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    call::call::<(TransferFromArgs,), (TransferFromResult,)>(
        Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai")
            .expect("Could not decode the principal."),
        "icrc2_transfer_from",
        (transfer_from_arg,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer error {:?}", e))
}

#[derive(CandidType, Deserialize)]
pub struct BalanceArgs {
    pub owner: Principal,
}

#[update]
async fn user_balance() -> Result<Nat, String> {
    let balance_args = BalanceArgs {
        owner: api::caller(),
    };

    let principal = Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai").unwrap();
    let method = "icrc1_balance_of";

    call::call(principal, method, (balance_args,))
        .await
        .map(|(balance,)| balance)
        .map_err(|(code, msg)| {
            format!(
                "Failed to get balance. Code: {:?}, Message: {:?}",
                code, msg
            )
        })
}

use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
#[update]
async fn approve_allowance(amount: NumTokens) -> Result<BlockIndex, String> {
    let approve_args = ApproveArgs {
        from_subaccount: None,
        spender: Account {
            owner: api::caller(),
            subaccount: None,
        },
        amount,
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    call::call::<(ApproveArgs,), (Result<BlockIndex, ApproveError>,)>(
        Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai")
            .expect("Could not decode the principal."),
        "icrc2_approve",
        (approve_args,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger approve error {:?}", e))
}

#[update]
async fn transfer_from_owner(to: Principal, amount: NumTokens) -> Result<BlockIndex, String> {
    let from = api::caller();
    let transfer_from_args = TransferFromArgs {
        from: Account {
            owner: from,
            subaccount: None,
        },
        to: Account {
            owner: to,
            subaccount: None,
        },
        amount,
        fee: None,
        memo: None,
        created_at_time: None,
        spender_subaccount: None,
    };

    call::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        Principal::from_text("rh2pm-ryaaa-aaaan-qeniq-cai")
            .expect("Could not decode the principal."),
        "icrc2_transfer_from",
        (transfer_from_args,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer_from error {:?}", e))
}
// ----------------------
// candid interface
// ----------------------
ic_cdk::export_candid!();
