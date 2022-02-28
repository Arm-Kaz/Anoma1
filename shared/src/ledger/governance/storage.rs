use crate::types::storage::{Key, DbKeySeg, KeySeg};

use super::ADDRESS;


const PROPOSAL_PREFIX: &str = "proposal";
const PROPOSAL_VOTE: &str = "vote";
const PROPOSAL_AUTHOR: &str = "author";
const PROPOSAL_CONTENT: &str = "content";
const PROPOSAL_START_EPOCH: &str = "start_epoch";
const PROPOSAL_END_EPOCH: &str = "end_epoch";
const PROPOSAL_GRACE_EPOCH: &str = "grace_epoch";
const PROPOSAL_FUNDS: &str = "balance";
const PROPOSAL_CODE: &str = "proposal_code";

const MIN_PROPOSAL_FUND_KEY: &str = "min_fund";
const MAX_PROPOSAL_CODE_SIZE_KEY: &str = "max_code_size";
const MIN_PROPOSAL_PERIOD_KEY: &str = "min_period";
const MAX_PROPOSAL_CONTENT_KEY: &str = "max_content";
const COUNTER_KEY: &str = "counter";

/// Check if a key is a vote key
pub fn is_vote_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(vote),
            DbKeySeg::AddressSeg(_address),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && vote == PROPOSAL_VOTE => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is author key
pub fn is_author_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(author),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && author == PROPOSAL_AUTHOR => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is proposal key
pub fn is_proposal_code_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(proposal_code),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && proposal_code == PROPOSAL_CODE => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is grace epoch key
pub fn is_grace_epoch_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(grace_epoch),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && grace_epoch == PROPOSAL_GRACE_EPOCH => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is content key
pub fn is_content_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(content),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && content == PROPOSAL_CONTENT => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is balance key
pub fn is_balance_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(funds),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && funds == PROPOSAL_FUNDS => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is start epoch key
pub fn is_start_epoch_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(start_epoch),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && start_epoch == PROPOSAL_START_EPOCH => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is epoch key
pub fn is_end_epoch_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(prefix),
            DbKeySeg::StringSeg(id),
            DbKeySeg::StringSeg(end_epoch),
        ] if addr == &ADDRESS && prefix == PROPOSAL_PREFIX && end_epoch == PROPOSAL_END_EPOCH => {
            id.parse::<u64>().is_ok()
        }
        _ => false,
    }
}

/// Check if key is counter key
pub fn is_counter_key(key: &Key) -> bool {
    match &key.segments[..] {
        [
            DbKeySeg::AddressSeg(addr),
            DbKeySeg::StringSeg(counter)
        ] if addr == &ADDRESS && counter == COUNTER_KEY => true,
        _ => false,
    }
}

/// Check if key is parameter key
pub fn is_parameter_key(key: &Key) -> bool {
    // TODO: implement
    true
}

/// Check if key is start epoch or end epoch key
pub fn is_start_or_end_epoch_key(key: &Key) -> bool {
    is_end_epoch_key(key) || is_start_epoch_key(key)
}

pub fn get_min_proposal_fund_key() -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&MIN_PROPOSAL_FUND_KEY.to_owned())
        .expect("Cannot obtain a storage key")
} 

pub fn get_max_proposal_code_size_key() -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&MAX_PROPOSAL_CODE_SIZE_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_min_proposal_period_key() -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&MIN_PROPOSAL_PERIOD_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_max_proposal_content_key() -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&MAX_PROPOSAL_CONTENT_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_counter_key() -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&COUNTER_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_content_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_CONTENT.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_author_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_AUTHOR.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_voting_start_epoch_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_START_EPOCH.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_voting_end_epoch_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_END_EPOCH.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_funds_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_FUNDS.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_voting_grace_epoch_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_GRACE_EPOCH.to_owned())
        .expect("Cannot obtain a storage key")
}

pub fn get_proposal_code_key(id: u64) -> Key {
    Key::from(ADDRESS.to_db_key())
        .push(&PROPOSAL_PREFIX.to_owned())
        .expect("Cannot obtain a storage key")
        .push(&id.to_string())
        .expect("Cannot obtain a storage key")
        .push(&PROPOSAL_CODE.to_owned())
        .expect("Cannot obtain a storage key")
}