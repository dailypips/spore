#![recursion_limit = "4096"]

pub mod constants;
pub mod contract;
pub mod open_order;
pub mod soft_dollar_tier;

pub mod commission_report;
pub mod execution;
pub mod group;
pub mod handshark;
pub mod order_state;
pub mod portfolio;
pub mod position;
pub mod security_def;
pub mod tagvalue;
pub mod tick_price;
pub mod tick_size;
pub mod verify_and_auth;
pub mod wire;

use commission_report::*;

pub enum IncomingMessage {
    CommissionReport(CommissionReport),
    MarketDataType(MarketDataType),
    TickSnapshotEnd(i32),
    DeltaExecutionDataEnd(i32),
    NextValidId(i32),
    ErrorMsg(ErrorMsg),
    UpdateTime(String)
}