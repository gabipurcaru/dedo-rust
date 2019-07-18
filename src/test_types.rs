#[cfg(test)]
mod tests {
    use super::super::defaults::*;
    use super::super::types::*;

    fn get_env() -> Environment {
        environment![
            "km" to "m" is 1000.0,
            "m" to "cm" is 1000.0
        ]
    }

    #[test]
    fn environment_creation() {
        assert_eq!(
            get_env(),
            Environment::new(&vec![
                Conversion::new("km", "m", 1000.0),
                Conversion::new("m", "cm", 1000.0),
            ],),
        );
    }

    #[test]
    fn environment_size() {
        // all self, mirror and transitive
        // dependencies should be there
        assert_eq!(get_env().conversions.len(), 10);

        // combinatorial explosion!
        assert_eq!(ENVIRONMENT.conversions.len(), 188);
    }

    #[test]
    fn conversion() {
        let env = get_env();
        assert_eq!(
            env.convert_units(&Value::new(2, Unit("km".to_string())), &UnitSet::single("m")),
            Value::new(2000, Unit("m".to_string()))
        );
    }
}
