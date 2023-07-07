use cosmwasm_schema::cw_serde_prost;

use crate::addresses::Addr;
use crate::coin::Coin;
use crate::timestamp::Timestamp;

#[cw_serde_prost]
#[derive(Eq)]
pub struct Env {
    #[prost(message, required, tag = "1")]
    pub block: BlockInfo,
    /// Information on the transaction this message was executed in.
    /// The field is unset when the `MsgExecuteContract`/`MsgInstantiateContract`/`MsgMigrateContract`
    /// is not executed as part of a transaction.
    #[prost(message, tag = "2")]
    pub transaction: Option<TransactionInfo>,
    #[prost(message, required, tag = "3")]
    pub contract: ContractInfo,
}

#[cw_serde_prost]
#[derive(Eq)]
pub struct TransactionInfo {
    /// The position of this transaction in the block. The first
    /// transaction has index 0.
    ///
    /// This allows you to get a unique transaction indentifier in this chain
    /// using the pair (`env.block.height`, `env.transaction.index`).
    ///
    #[prost(uint32, tag = "1")]
    pub index: u32,
}

#[cw_serde_prost]
#[derive(Eq)]
pub struct BlockInfo {
    /// The height of a block is the number of blocks preceding it in the blockchain.
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// Absolute time of the block creation in seconds since the UNIX epoch (00:00:00 on 1970-01-01 UTC).
    ///
    /// The source of this is the [BFT Time in Tendermint](https://github.com/tendermint/tendermint/blob/58dc1726/spec/consensus/bft-time.md),
    /// which has the same nanosecond precision as the `Timestamp` type.
    ///
    /// # Examples
    ///
    /// Using chrono:
    ///
    /// ```
    /// # use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Env, MessageInfo, Timestamp, TransactionInfo};
    /// # let env = Env {
    /// #     block: BlockInfo {
    /// #         height: 12_345,
    /// #         time: Timestamp::from_nanos(1_571_797_419_879_305_533),
    /// #         chain_id: "cosmos-testnet-14002".to_string(),
    /// #     },
    /// #     transaction: Some(TransactionInfo { index: 3 }),
    /// #     contract: ContractInfo {
    /// #         address: Addr::unchecked("contract"),
    /// #     },
    /// # };
    /// # extern crate chrono;
    /// use chrono::NaiveDateTime;
    /// let seconds = env.block.time.seconds();
    /// let nsecs = env.block.time.subsec_nanos();
    /// let dt = NaiveDateTime::from_timestamp(seconds as i64, nsecs as u32);
    /// ```
    ///
    /// Creating a simple millisecond-precision timestamp (as used in JavaScript):
    ///
    /// ```
    /// # use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Env, MessageInfo, Timestamp, TransactionInfo};
    /// # let env = Env {
    /// #     block: BlockInfo {
    /// #         height: 12_345,
    /// #         time: Timestamp::from_nanos(1_571_797_419_879_305_533),
    /// #         chain_id: "cosmos-testnet-14002".to_string(),
    /// #     },
    /// #     transaction: Some(TransactionInfo { index: 3 }),
    /// #     contract: ContractInfo {
    /// #         address: Addr::unchecked("contract"),
    /// #     },
    /// # };
    /// let millis = env.block.time.nanos() / 1_000_000;
    /// ```
    #[prost(message, required, tag = "2")]
    pub time: Timestamp,
    #[prost(string, tag = "3")]
    pub chain_id: String,
}

/// Additional information from [MsgInstantiateContract] and [MsgExecuteContract], which is passed
/// along with the contract execution message into the `instantiate` and `execute` entry points.
///
/// It contains the essential info for authorization - identity of the call, and payment.
///
/// [MsgInstantiateContract]: https://github.com/CosmWasm/wasmd/blob/v0.15.0/x/wasm/internal/types/tx.proto#L47-L61
/// [MsgExecuteContract]: https://github.com/CosmWasm/wasmd/blob/v0.15.0/x/wasm/internal/types/tx.proto#L68-L78
#[cw_serde_prost]
#[derive(Eq)]
pub struct MessageInfo {
    /// The `sender` field from `MsgInstantiateContract` and `MsgExecuteContract`.
    /// You can think of this as the address that initiated the action (i.e. the message). What that
    /// means exactly heavily depends on the application.
    ///
    /// The x/wasm module ensures that the sender address signed the transaction or
    /// is otherwise authorized to send the message.
    ///
    /// Additional signers of the transaction that are either needed for other messages or contain unnecessary
    /// signatures are not propagated into the contract.
    #[prost(message, required, tag = "1")]
    pub sender: Addr,
    /// The funds that are sent to the contract as part of `MsgInstantiateContract`
    /// or `MsgExecuteContract`. The transfer is processed in bank before the contract
    /// is executed such that the new balance is visible during contract execution.
    #[prost(message, repeated, tag = "2")]
    pub funds: Vec<Coin>,
}

#[cw_serde_prost]
#[derive(Eq)]
pub struct ContractInfo {
    #[prost(message, required, tag = "1")]
    pub address: Addr,
}
