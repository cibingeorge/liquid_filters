#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "json",
    description = "Json renders the input",
    parsed(ToJsonFilter)
)]
pub struct ToJson;

#[derive(Debug, Default, Display_filter)]
#[name = "json"]
struct ToJsonFilter;

impl Filter for ToJsonFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let value = input.to_value();
        let serialized = serde_json::to_string(&value).unwrap();
        Ok(Value::scalar(serialized))
    }
}



#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn unit_ruby_serialize() {
        let rendered = crate::render_template!(
            "{{a}}".to_owned(),
            json!({"a": 1})
        );
        assert_eq!(
            rendered,
            "1"
        );

        let rendered = crate::render_template!(
            "{{b}}".to_owned(),
            json!({"b": null})
        );
        assert_eq!(
            rendered,
            ""
        );


        let rendered = crate::render_template!(
            "{{b}}".to_owned(),
            json!({"b": 1.1})
        );
        assert_eq!(
            rendered,
            "1.1"
        );

        let rendered = crate::render_template!(
            "{{b}}".to_owned(),
            json!({"b": "str"})
        );
        assert_eq!(
            rendered,
            "str"
        );

        let rendered = crate::render_template!(
            "{{b.d[2]}}".to_owned(),
            json!({"b": {"c": "str", "e": {}, "d": [1, 2, {"e": "f"}]}})
        );
        assert_eq!(
            rendered,
            r#"{"e"=>"f"}"#
        );


        let rendered = crate::render_template!(
            "{{b.e}}".to_owned(),
            json!({"b": {"c": "str", "e": {}, "d": [1, 2, {"e": "f"}]}})
        );
        assert_eq!(
            rendered,
            "{}"
        );


        let rendered = crate::render_template!(
            "{{a}}".to_owned(),
            json!({"a": [1,2,3], "b": {"c": "str", "e": {}, "d": [1, 2, {"e": "f"}]}})
        );
        assert_eq!(
            rendered,
            "[1, 2, 3]"
        );


    }

}

