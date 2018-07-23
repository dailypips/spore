pub struct OrderState {
    pub status: String,
    pub init_margin: String,
    pub maint_margin: String,
    pub equity_with_loan: String,
    pub commission: f64,
    pub min_commission: f64,
    pub max_commission: f64,
    pub commission_currency: String,
    pub warning_text: String,
}
