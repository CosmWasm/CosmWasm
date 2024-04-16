use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns stats about what callbacks have been received
    #[returns(crate::state::CallbackStats)]
    CallbackStats {},
}

#[cw_serde]
pub struct ExecuteMsg {
    /// Address on the destination chain
    pub to_address: String,
    /// The channel to send the packet through
    pub channel_id: String,
    /// The amount of seconds from now the transfer should timeout at
    pub timeout_seconds: u32,
    /// Who should receive callbacks for the message
    #[serde(default)]
    pub callback_type: CallbackType,
}

#[cw_serde]
#[derive(Default)]
pub enum CallbackType {
    /// Only this contract on the source chain should receive callbacks
    Src,
    /// Only the destination address should receive callbacks
    Dst,
    /// Both the source contract and the destination address should receive callbacks
    #[default]
    Both,
}
