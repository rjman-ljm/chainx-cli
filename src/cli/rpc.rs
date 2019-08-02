use structopt::StructOpt;
use web3::futures::Future;

use crate::error::Result;
use crate::rpc::{AuthorRpc, ChainRpc, ChainXRpc, StateRpc, SystemRpc};
use crate::types::{Chain, Hash, HashOrHeight};

#[derive(Debug, StructOpt)]
pub enum RpcCommand {
    // Chain RPC
    // ========================================================================
    /// Get header of a relay chain block.
    #[structopt(name = "header")]
    Header {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    /// Get hash of the last finalized block in the canon chain.
    #[structopt(name = "header_finalized")]
    FinalizedHeader,
    /// Get hash of the n-th block in the canon chain.
    #[structopt(name = "block_hash")]
    BlockHash {
        /// Block height [default: latest block height]
        #[structopt(value_name = "NUM")]
        height: Option<u64>,
    },
    /// Get header and body of a relay chain block.
    #[structopt(name = "block")]
    Block {
        /// 0x-prefix hex block hash string or block height [default: hash or height of the latest block]
        #[structopt(value_name = "HEIGHT/HASH")]
        hash_or_height: Option<HashOrHeight>,
    },

    // System Rpc
    // ========================================================================
    /// Get the node's implementation name. Plain old string.
    #[structopt(name = "system_name")]
    SystemName,
    /// Get the node implementation's version. Should be a semver string.
    #[structopt(name = "system_version")]
    SystemVersion,
    /// Get the chain's type. Given as a string identifier.
    #[structopt(name = "system_chain")]
    SystemChain,
    /// Get a custom set of properties as a JSON object, defined in the chain spec.
    #[structopt(name = "system_properties")]
    SystemProperties,
    /// Return health status of the node.
    #[structopt(name = "system_health")]
    SystemHealth,
    /// Returns currently connected peers.
    #[structopt(name = "system_peers")]
    SystemPeers,
    /// Returns current state of the network.
    #[structopt(name = "system_network_state")]
    SystemNetworkState,

    // ChainX Rpc
    // ========================================================================
    #[structopt(name = "next_renominate")]
    NextRenominate {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "asset")]
    Asset {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "assets")]
    Assets {
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "verify_addr")]
    VerifyAddr {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// Address
        #[structopt(value_name = "ADDR")]
        addr: String,
        /// Memo
        #[structopt(value_name = "MEMO")]
        memo: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "withdraw_limit")]
    WithdrawLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "deposit_limit")]
    DepositLimit {
        /// Token name
        #[structopt(value_name = "TOKEN", default_value = "PCX")]
        token: String,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "withdraw_list")]
    WithdrawList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "deposit_list")]
    DepositList {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "nomination_records")]
    NominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "psedu_nomination_records")]
    PseduNominationRecords {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "intention")]
    Intention {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "intentions")]
    Intentions {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "psedu_intentions")]
    PseduIntentions {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "trading_pairs")]
    TradingPairs {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "quotations")]
    Quotations {
        /// Trading pair index
        #[structopt(value_name = "INDEX")]
        id: u32,
        #[structopt(value_name = "HASH")]
        piece: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "orders")]
    Orders {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Page index
        #[structopt(value_name = "INDEX", default_value = "0")]
        index: u32,
        /// Page size
        #[structopt(value_name = "SIZE", default_value = "5")]
        size: u32,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "addr_by_account")]
    AddrByAccount {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "trustee_session")]
    TrusteeSession {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// Trustee session era, [0, latest) [default: latest trustee session era]
        #[structopt(value_name = "ERA")]
        era: Option<u32>,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "trustee_info")]
    TrusteeInfo {
        /// 0x-prefix hex hash string for account
        #[structopt(value_name = "ACCOUNT")]
        who: Hash,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "call_fee")]
    CallFee {
        /// The parameters of Call
        #[structopt(value_name = "PARAMS")]
        call: String,
        /// The length of transaction
        tx_len: u64,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "withdraw_tx")]
    WithdrawTx {
        /// Chain name
        #[structopt(value_name = "CHAIN", default_value = "Bitcoin")]
        chain: Chain,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "mock_btc_new_trustees")]
    MockBtcNewTrustees {
        /// 0x-prefix hex hash string for new trustee accounts
        #[structopt(value_name = "ACCOUNTS")]
        candidates: Vec<Hash>,
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
    #[structopt(name = "particular_accounts")]
    ParticularAccounts {
        /// 0x-prefix hex block hash string [default: latest block hash]
        #[structopt(value_name = "HASH")]
        hash: Option<Hash>,
    },
}

impl RpcCommand {
    /// Dispatch rpc subcommand
    #[rustfmt::skip]
    pub fn dispatch<Rpc>(self, rpc: Rpc) -> Result<()>
    where
        Rpc: AuthorRpc + ChainRpc + ChainXRpc + StateRpc + SystemRpc,
    {
        use RpcCommand::*;
        let fut = match self {
            // Chain Rpc
            Header { hash } => rpc.header(hash),
            FinalizedHeader => rpc.finalized_head(),
            BlockHash { height } => rpc.block_hash(height),
            Block { hash_or_height } => match hash_or_height {
                Some(HashOrHeight::Height(number)) => rpc.block_by_number(Some(number)),
                Some(HashOrHeight::Hash(hash)) => rpc.block_by_hash(Some(hash)),
                None => rpc.block_by_hash(None),
            }

            // System Rpc
            SystemName => rpc.system_name(),
            SystemVersion => rpc.system_version(),
            SystemChain => rpc.system_chain(),
            SystemProperties => rpc.system_properties(),
            SystemHealth => rpc.system_health(),
            SystemPeers => rpc.system_peers(),
            SystemNetworkState => rpc.system_network_state(),

            // ChainX Rpc
            NextRenominate { who, hash } => rpc.next_renominate(who, hash),
            Asset { who, index, size, hash } => rpc.asset(who, index, size, hash),
            Assets { index, size, hash } => rpc.assets(index, size, hash),
            VerifyAddr { token, addr, memo, hash} => rpc.verify_addr(token, addr, memo, hash),
            WithdrawLimit { token, hash } => rpc.withdraw_limit(token, hash),
            DepositLimit { token, hash } => rpc.deposit_limit(token, hash),
            WithdrawList { chain, index, size, hash} => rpc.withdraw_list(chain, index, size, hash),
            DepositList { chain, index, size, hash } => rpc.deposit_list(chain, index, size, hash),
            NominationRecords { who, hash } => rpc.nomination_records(who, hash),
            PseduNominationRecords { who, hash } => rpc.psedu_nomination_records(who, hash),
            Intention { who, hash } => rpc.intention(who, hash),
            Intentions { hash } => rpc.intentions(hash),
            PseduIntentions { hash } => rpc.psedu_intentions(hash),
            TradingPairs { hash } => rpc.trading_pairs(hash),
            Quotations { id, piece, hash } => rpc.quotations(id, piece, hash),
            Orders { who, index, size, hash} => rpc.orders(who, index, size, hash),
            AddrByAccount { who, chain, hash } => rpc.addr_by_account(who, chain, hash),
            TrusteeSession { chain, era, hash } => rpc.trustee_session_info(chain, era, hash),
            TrusteeInfo { who, hash } => rpc.trustee_by_account(who, hash),
            CallFee { call, tx_len, hash } => rpc.call_fee(call, tx_len, hash),
            WithdrawTx { chain, hash } => rpc.withdraw_tx(chain, hash),
            MockBtcNewTrustees { candidates, hash } => rpc.mock_btc_new_trustees(candidates, hash),
            ParticularAccounts { hash } => rpc.particular_accounts(hash),
        };
        let response = fut.wait()?;
        let response = serde_json::to_string_pretty(&response)?;
        println!("{}", response);
        Ok(())
    }
}