#![allow(clippy::invisible_characters)]

use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "shuffle",
    description = "Shuffles an array.",
    parsed(ShuffleFilter)
)]
pub struct Shuffle;

#[derive(Debug, Default, Display_filter)]
#[name = "shuffle"]
struct ShuffleFilter;

impl Filter for ShuffleFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        if let Some(array) = input.as_array() {
            let mut values: Vec<Value>  = array.values().map(|x| x.to_value()).collect();
            values.shuffle(&mut thread_rng());
            return Ok(Value::array(values));
        }
        Ok(input.to_value())
    }
}
