#[macro_use]
extern crate alloc;

// Exposed on all platforms

mod assertions;
mod checksum;
mod coin;
mod coins;
mod conversion;
mod deps;
mod ibc;
mod import_helpers;
#[cfg(feature = "iterator")]
mod iterator;
mod metadata;
mod never;
mod pagination;
mod panic;
mod query;
mod results;
mod sections;
mod serde;
mod stdack;
mod storage;
mod traits;
mod types;
mod uuid;

/// This module is to simplify no_std imports
pub(crate) mod prelude;

/// This modules is very advanced and will not be used directly by the vast majority of users.
/// We want to offer it to ensure a stable storage key composition system but don't encourage
/// contract devs to use it directly.
pub mod storage_keys;

pub use crate::checksum::{Checksum, ChecksumError};
pub use crate::coin::{coin, coins, has_coins, Coin};
pub use crate::coins::Coins;
pub use crate::deps::{Deps, DepsMut, OwnedDeps};
pub use crate::ibc::IbcChannelOpenResponse;
pub use crate::ibc::{
    Ibc3ChannelOpenResponse, IbcAcknowledgement, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcEndpoint, IbcMsg, IbcOrder, IbcPacket,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout,
    IbcTimeoutBlock,
};
#[cfg(feature = "iterator")]
pub use crate::iterator::{Order, Record};
pub use crate::metadata::{DenomMetadata, DenomUnit};
pub use crate::never::Never;
pub use crate::pagination::PageRequest;
pub use crate::query::{
    AllBalanceResponse, AllDelegationsResponse, AllDenomMetadataResponse, AllValidatorsResponse,
    BalanceResponse, BankQuery, BondedDenomResponse, ChannelResponse, CodeInfoResponse,
    ContractInfoResponse, CustomQuery, DecCoin, Delegation, DelegationResponse,
    DelegationRewardsResponse, DelegationTotalRewardsResponse, DelegatorReward,
    DelegatorValidatorsResponse, DelegatorWithdrawAddressResponse, DenomMetadataResponse,
    DistributionQuery, FullDelegation, GrpcQuery, IbcQuery, ListChannelsResponse, PortIdResponse,
    QueryRequest, StakingQuery, SupplyResponse, Validator, ValidatorResponse, WasmQuery,
};
#[cfg(all(feature = "stargate", feature = "cosmwasm_1_2"))]
pub use crate::results::WeightedVoteOption;
pub use crate::results::{
    attr, wasm_execute, wasm_instantiate, AnyMsg, Attribute, BankMsg, ContractResult, CosmosMsg,
    CustomMsg, Empty, Event, MsgResponse, QueryResponse, Reply, ReplyOn, Response, SubMsg,
    SubMsgResponse, SubMsgResult, SystemResult, WasmMsg,
};
#[cfg(feature = "staking")]
pub use crate::results::{DistributionMsg, StakingMsg};
#[cfg(feature = "stargate")]
pub use crate::results::{GovMsg, VoteOption};
#[allow(deprecated)]
pub use crate::serde::{
    from_binary, from_json, from_slice, to_binary, to_json_binary, to_json_string, to_json_vec,
    to_vec,
};
pub use crate::stdack::StdAck;
pub use crate::storage::MemoryStorage;
pub use crate::traits::{Api, Querier, QuerierResult, QuerierWrapper, Storage};
pub use crate::types::{BlockInfo, ContractInfo, Env, MessageInfo, TransactionInfo};
pub use crate::uuid::{new_uuid, Uuid};

// Exposed in wasm build only

#[cfg(target_arch = "wasm32")]
mod exports;
#[cfg(target_arch = "wasm32")]
mod imports;
#[cfg(target_arch = "wasm32")]
mod memory; // Used by exports and imports only. This assumes pointers are 32 bit long, which makes it untestable on dev machines.

#[cfg(target_arch = "wasm32")]
pub use crate::exports::{do_execute, do_instantiate, do_migrate, do_query, do_reply, do_sudo};
#[cfg(all(feature = "stargate", target_arch = "wasm32"))]
pub use crate::exports::{
    do_ibc_channel_close, do_ibc_channel_connect, do_ibc_channel_open, do_ibc_packet_ack,
    do_ibc_packet_receive, do_ibc_packet_timeout,
};
#[cfg(target_arch = "wasm32")]
pub use crate::imports::{ExternalApi, ExternalQuerier, ExternalStorage};

/// Exposed for testing only
/// Both unit tests and integration tests are compiled to native code, so everything in here does not need to compile to Wasm.
#[cfg(not(target_arch = "wasm32"))]
pub mod testing;

// Re-exports

pub use cosmwasm_core::CoreError as StdError;
pub use cosmwasm_core::CoreResult as StdResult;
pub use cosmwasm_core::{
    from_base64, from_hex, instantiate2_address, to_base64, to_hex, Addr, Binary, CanonicalAddr,
    CheckedFromRatioError, CheckedMultiplyFractionError, CheckedMultiplyRatioError,
    CoinFromStrError, CoinsError, ConversionOverflowError, Decimal, Decimal256,
    Decimal256RangeExceeded, DecimalRangeExceeded, DivideByZeroError, DivisionError, Fraction,
    HexBinary, Instantiate2AddressError, Int128, Int256, Int512, Int64, Isqrt, OverflowError,
    OverflowOperation, RecoverPubkeyError, SignedDecimal, SignedDecimal256,
    SignedDecimal256RangeExceeded, SignedDecimalRangeExceeded, SystemError, Timestamp, Uint128,
    Uint256, Uint512, Uint64, VerificationError,
};

#[cfg(not(target_arch = "wasm32"))]
pub use cosmwasm_core::assert_approx_eq;

pub use cosmwasm_derive::entry_point;
