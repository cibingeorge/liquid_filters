
pub struct CurrencyConfig<'a> {
    #[allow(dead_code)]
    pub value: &'a str,
    #[allow(dead_code)]
    pub name: &'a str,
    pub symbol: &'a str,
    pub is_symbol_prefix: bool,
    pub separator: &'a str,
    pub delimiter: &'a str,
    pub precision: Option<i64>,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn currency_config(value: &str) -> Option<&CurrencyConfig> {
    CURRENCY_CONFIG.get(value)
}
