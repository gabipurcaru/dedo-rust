use super::types::*;

lazy_static! {
    pub static ref ENVIRONMENT: Environment = environment![
       "gbp" to "£" is 1,
       "usd" to "$" is 1,
       "eur" to "€" is 1,
       "gbp" to "eur" is 1.06,
       "gbp" to "ron" is 5.07,
       "gbp" to "usd" is 1.20,
       "km" to "m" is 1000,
       "m" to "dm" is 10,
       "dm" to "cm" is 10,
       "cm" to "mm" is 10,
       "kg" to "g" is 1000,
       "g" to "mg" is 1000,
       "minute" to "second" is 60,
       "hour" to "minute" is 60,
       "day" to "hour" is 24,
       "week" to "day" is 7,
       "month" to "day" is 30.5,
       "year" to "month" is 12,
       "minute" to "minutes" is 1,
       "hour" to "hours" is 1,
       "day" to "days" is 1,
       "week" to "weeks" is 1,
       "month" to "months" is 1,
       "year" to "years" is 1,
       "sec" to "second" is 1,
       "min" to "minute" is 1,
       "s" to "second" is 1,
       // m is for meters, not minutes!
       "h" to "hour" is 1,
       "d" to "day" is 1,
       "w" to "week" is 1,
       "y" to "year" is 1
    ];
}
