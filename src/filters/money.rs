#![allow(clippy::invisible_characters)]

use liquid_core::Expression;
use liquid_core::FilterParameters;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, FromFilterParameters, ParseFilter};
use liquid_core::{Value, ValueView};

use crate::utils::format_currency;

#[derive(Debug, FilterParameters)]
struct MoneyArgs {
    #[parameter(description = "Use the currency symbol.", arg_type = "bool")]
    use_symbol: Option<Expression>,

    #[parameter(description = "Use space as separator.", arg_type = "bool")]
    use_space: Option<Expression>,

    #[parameter(description = "Currency type. USD,EUR,GBP,INR etc", arg_type = "str")]
    currency_type: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "money",
    description = "Converts a number to money.",
    parameters(MoneyArgs),
    parsed(MoneyFilter)
)]
pub struct Money;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "money"]
struct MoneyFilter {
    #[parameters]
    args: MoneyArgs,
}

impl Filter for MoneyFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let render_context = runtime
            .registers()
            .get::<crate::RenderContext>();

        let formatted_as_money = format_currency(&render_context, input, args.use_symbol, args.use_space, args.currency_type)?;
        Ok(Value::scalar(formatted_as_money))
    }
}

#[derive(Debug, FilterParameters)]
struct MoneyWithoutTrailingZerosArgs {
    #[parameter(description = "Use the currency symbol.", arg_type = "bool")]
    use_symbol: Option<Expression>,

    #[parameter(description = "Use space as separator.", arg_type = "bool")]
    use_space: Option<Expression>,

    #[parameter(description = "Currency type. USD,EUR,GBP,INR etc", arg_type = "str")]
    currency_type: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "money_without_trailing_zeros",
    description = "Converts a number to money.",
    parameters(MoneyWithoutTrailingZerosArgs),
    parsed(MoneyWithoutTrailingZerosFilter)
)]
pub struct MoneyWithoutTrailingZeros;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "money_without_trailing_zeros"]
struct MoneyWithoutTrailingZerosFilter {
    #[parameters]
    args: MoneyWithoutTrailingZerosArgs,
}

impl Filter for MoneyWithoutTrailingZerosFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;
        let render_context = runtime
            .registers()
            .get::<crate::RenderContext>();

        let formatted_as_money = format_currency(&render_context, input, args.use_symbol, args.use_space, args.currency_type)?;
        Ok(Value::scalar(formatted_as_money.replace(".00", "")))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn unit_money() {
        let runtime = liquid_core::runtime::RuntimeBuilder::new().build();
        {
            let mut cxt = runtime.registers().get_mut::<crate::RenderContext>();
            cxt.currency_type = Arc::new("USD".to_owned());
        }

        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, "10", true, true, "USD").unwrap(),
            liquid_core::value!("10.00")
        );
        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, "10.00").unwrap(),
            liquid_core::value!("10.00")
        );
        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, "10.99").unwrap(),
            liquid_core::value!("10.99")
        );

        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, 10).unwrap(),
            liquid_core::value!("10.00")
        );

        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, 10.00).unwrap(),
            liquid_core::value!("10.00")
        );
        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, 10.99).unwrap(),
            liquid_core::value!("10.99")
        );

        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, 10.9999).unwrap(),
            liquid_core::value!("11.00")
        );

        assert_eq!(
            crate::call_filter_with_runtime!(runtime, Money, 100000.9).unwrap(),
            liquid_core::value!("100,000.90")
        );

    }

}
