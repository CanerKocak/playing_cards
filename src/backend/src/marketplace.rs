// marketplace.rs
use super::*;

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

type Tokens = u64;

#[derive(CandidType, Deserialize, Clone)]
pub struct SaleListing {
    pub token_id: u64,
    pub seller: Principal,
    pub price: Tokens,
}

// function to sell a NFT

// function to buy a NFT

// function to get all on sale NFTs

// function to get marketplace data for a specific NFT

// function to start the marketplace, by minting 54 NFTs and listing them for sale towards the burn address, meaning that the first sale of the nfts will burn the EXE tokens
// make sure to remember in the frontend to match case so that if the seller is burn address, the frontend will show a fire emoji or something similar
