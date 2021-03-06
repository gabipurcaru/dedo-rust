extern crate test;

#[cfg(test)]
mod tests {
    use super::super::defaults::*;
    use super::super::types::*;
    use super::test::Bencher;
    use std::collections::HashMap;
    use wasm_bindgen_test::*;

    fn get_env() -> Environment {
        environment![
            "km" to "m" is 1000.0,
            "m" to "cm" is 100.0,
            "h" to "min" is 60.0,
            "min" to "s" is 60.0
        ]
    }

    #[test]
    #[wasm_bindgen_test]
    fn environment_creation() {
        let mut map = HashMap::new();
        map.insert(("km".into(), "m".into()), 1000.0);
        map.insert(("m".into(), "cm".into()), 100.0);
        map.insert(("h".into(), "min".into()), 60.0);
        map.insert(("min".into(), "s".into()), 60.0);

        assert_eq!(get_env(), Environment::new(&Conversions(map)));
    }

    #[test]
    #[wasm_bindgen_test]
    fn environment_size() {
        // all self, mirror and transitive
        // dependencies should be there
        assert_eq!(get_env().conversions.0.len(), 18);

        // combinatorial explosion!
        assert_eq!(ENVIRONMENT.conversions.0.len(), 555);
    }

    #[bench]
    fn environment_bench(b: &mut Bencher) {
        b.iter(|| {
            environment![
               "gbp" to "£" is 1,
               "usd" to "$" is 1,
               "eur" to "€" is 1,
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
               "s" to "second" is 1,
               "sec" to "second" is 1,
               "min" to "minute" is 1,
               "h" to "hour" is 1,
               "d" to "day" is 1,
               "w" to "week" is 1,
               "y" to "year" is 1
            ]
        })
    }

    #[test]
    #[wasm_bindgen_test]
    fn conversion() {
        let env = get_env();

        // 2km = 2000m
        assert_eq!(
            env.convert_units(&Value::simple(2., "km"), &units!("m" to 1)),
            Value::simple(2000., "m")
        );

        // 36km/h = 10m/s
        assert_eq!(
            env.convert_units(
                &Value::new(
                    36.,
                    units!(
                        "km" to 1,
                        "h" to -1
                    )
                ),
                &units!("m" to 1, "s" to -1)
            ),
            Value::new(
                10.,
                units!(
                    "m" to 1,
                    "s" to -1
                )
            )
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn addition() {
        let env = get_env();
        let left = Value::new(
            12.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        let right = Value::new(
            10.,
            units!(
                "m" to 1,
                "s" to -1
            ),
        );
        let expected = Value::new(
            48.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        assert_eq!(env.add(left, right), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn subtraction() {
        let env = get_env();
        let left = Value::new(
            50.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        let right = Value::new(
            10.,
            units!(
                "m" to 1,
                "s" to -1
            ),
        );
        let expected = Value::new(
            14.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        assert_eq!(env.sub(left, right), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn multiplication() {
        let env = get_env();
        let left = Value::new(
            36.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        let right = Value::new(
            10.,
            units!(
                "m" to 1,
                "s" to 1
            ),
        );
        let expected = Value::new(
            0.0001,
            units!(
                "km" to 2
            ),
        );
        assert_eq!(env.mul(left, right), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn division() {
        let env = get_env();
        let left = Value::new(
            36.,
            units!(
                "km" to 1,
                "h" to -1
            ),
        );
        let right = Value::new(
            10.,
            units!(
                "m" to 1,
                "s" to 1
            ),
        );
        let expected = Value::new(
            12960000.0,
            units!(
                "h" to -2
            ),
        );
        assert_eq!(env.div(left, right), expected);

        assert_eq!(
            env.div(Value::unitless(2.0), Value::unitless(2.0)),
            Value::unitless(1.0)
        );
    }
}
