#[repr(i32)]
pub enum PriceConditionTriggerMode {
    Default = 0,
    Double_Ask_Bid = 1,
    Last = 2,
    Double_Last = 3,
    Bid_Ask = 4,
    Last_Of_Bid_Ask = 7,
    Mid_Point = 8,
}

pub enum OrderCondition {
    Execution {
        // inherit orderCondition
        is_conjunction_connection: bool,
        sec_type: String,
        exchange: String,
        symbol: String,
    },

    Margin {
        // inherit OperatorCondition
        is_conjunction_connection: bool,
        is_more: bool,

        percent: i32,
    },

    PercentChange {
        // inherit ContractCondition
        is_conjunction_connection: bool,
        is_more: bool,

        con_id: i32,
        exchange: String,

        change_percent: f64,
    },

    Price {
        // inherit ContractCondition
        is_conjunction_connection: bool,
        is_more: bool,

        con_id: i32,
        exchange: String,

        price: f64,
        trigger_mode: i32,
    },

    Time {
        // inherit OperatorCondition
        is_conjunction_connection: bool,
        is_more: bool,

        time: String,
    },

    Volume {
        // inherit ContractCondition
        is_conjunction_connection: bool,
        is_more: bool,

        con_id: i32,
        exchange: String,

        volume: i32,
    },
}
