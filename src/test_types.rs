#[cfg(test)]
mod tests {
    use super::super::defaults::*;
    use super::super::types::*;

    fn get_env() -> Environment {
        environment![
            "km" to "m" is 1000.0,
            "m" to "cm" is 1000.0,
            "h" to "s" is 3600.0
        ]
    }

    #[test]
    fn environment_creation() {
        assert_eq!(
            get_env(),
            Environment::new(&vec![
                Conversion::new("km", "m", 1000.0),
                Conversion::new("m", "cm", 1000.0),
                Conversion::new("h", "s", 3600.0),
            ],),
        );
    }

    #[test]
    fn environment_size() {
        // all self, mirror and transitive
        // dependencies should be there
        assert_eq!(get_env().conversions.len(), 14);

        // combinatorial explosion!
        assert_eq!(ENVIRONMENT.conversions.len(), 188);
    }

    #[test]
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
}
