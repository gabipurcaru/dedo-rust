use std::collections::HashMap;
use std::string::String;

#[derive(Debug, PartialEq)]
pub struct Environment {
    pub conversions: Vec<Conversion>,
}

impl Environment {
    pub fn new(conversions: &Vec<Conversion>) -> Environment {
        Environment {
            conversions: Environment::expand_conversions(conversions),
        }
    }

    fn conversion_ratio(&self, from: &Unit, to: &Unit) -> Result<f64, String> {
        for conversion in &self.conversions {
            if conversion.from == *from && conversion.to == *to {
                return Ok(conversion.ratio);
            }
        }

        Err(format!("Cannot convert from {:?} to {:?}", from, to))
    }

    /// unit conversions e.g. 1km/h to 0.28m/s etc.
    pub fn convert_units(&self, value: &Value, new_units: &UnitSet) -> Value {
        let mut converted = value.clone();

        match value {
            Value {
                num: _,
                units: UnitSet(units),
            } => {
                for (unit, val) in units.iter() {
                    for (to_unit, _) in new_units.0.iter() {
                        match self.conversion_ratio(unit, to_unit) {
                            Err(_) => {
                                // pass
                            }
                            Ok(ratio) => {
                                converted.num *= ratio.powf(*val as f64);
                                converted.units.0.remove(&unit);
                                converted.units.0.insert(to_unit.clone(), val.clone());
                            }
                        }
                    }
                }
            }
        }

        converted
    }

    fn expand_conversions(basic_conversions: &Vec<Conversion>) -> Vec<Conversion> {
        let mut conversions = Vec::new();

        // Add a -> b, b -> a, a -> a and b -> b conversions
        for conversion in basic_conversions {
            match conversion {
                Conversion { from, to, ratio } => {
                    conversions.push(Conversion {
                        from: from.clone(),
                        to: to.clone(),
                        ratio: ratio.clone(),
                    });
                    conversions.push(Conversion {
                        from: to.clone(),
                        to: from.clone(),
                        ratio: 1. / ratio,
                    });
                    conversions.push(Conversion {
                        from: from.clone(),
                        to: from.clone(),
                        ratio: 1.,
                    });
                    conversions.push(Conversion {
                        from: to.clone(),
                        to: to.clone(),
                        ratio: 1.,
                    });
                }
            }
        }

        // for all a -> b and b -> c, add a -> c, aka transitive conversions

        let initial_length = conversions.len();
        loop {
            // loop until no changes are made anymore
            let mut is_saturated = true;

            for left in 0..initial_length {
                for right in 0..initial_length {
                    if conversions[left].to == conversions[right].from
                        && Environment::should_add(
                            &conversions,
                            &conversions[left],
                            &conversions[right],
                        )
                    {
                        conversions.push(Conversion {
                            from: conversions[left].from.clone(),
                            to: conversions[right].to.clone(),
                            ratio: conversions[left].ratio * conversions[right].ratio,
                        });
                        is_saturated = false;
                    }
                }
            }

            if is_saturated {
                break;
            }
        }

        conversions
    }

    fn should_add(conversions: &Vec<Conversion>, left: &Conversion, right: &Conversion) -> bool {
        for conversion in conversions {
            if conversion.from == left.from && conversion.to == right.to {
                return false;
            }
        }

        true
    }

    pub fn add(&self, left: Value, right: Value) -> Value {
        let converted_right = self.convert_units(&right, &left.units);

        Value {
            num: left.num + converted_right.num,
            units: left.units,
        }
    }

    pub fn sub(&self, left: Value, right: Value) -> Value {
        let converted_right = self.convert_units(&right, &left.units);

        Value {
            num: left.num - converted_right.num,
            units: left.units,
        }
    }

    pub fn mul(&self, left: Value, right: Value) -> Value {
        let converted_right = self.convert_units(&right, &left.units);
        let mut result_units = left.units.0.clone();

        for (unit, num) in converted_right.units.0.iter() {
            let result_num = result_units.entry(unit.clone()).or_insert(0);
            *result_num += num;

            if *result_num == 0 {
                result_units.remove(unit);
            }
        }

        Value {
            num: left.num * converted_right.num,
            units: UnitSet(result_units),
        }
    }

    pub fn div(&self, left: Value, right: Value) -> Value {
        let inverted_units = right
            .units
            .0
            .clone()
            .into_iter()
            .map(|(k, v)| (k, -v))
            .collect();

        let inverted_right = Value {
            num: right.num,
            units: UnitSet(inverted_units),
        };

        self.mul(left, inverted_right)
    }
}

/// Utility to help create a static environment. Basic syntax is
/// environment!["some_unit" to "some_other_unit" is 123.456, ...]
#[macro_export]
macro_rules! environment {
    ($($from:literal to $to:literal is $num:literal),*) => {
        {
            let tmp_vec: Vec<Conversion> = vec![
                $(
                    Conversion::new($from, $to, $num as f64),
                )*
            ];
            Environment::new(&tmp_vec)
        }
    } ;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Conversion {
    from: Unit,
    to: Unit,
    ratio: f64,
}

impl Conversion {
    pub fn new(from: &'static str, to: &'static str, ratio: f64) -> Conversion {
        Conversion {
            from: Unit(from.to_string()),
            to: Unit(to.to_string()),
            ratio,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Unit(pub String);

impl From<&str> for Unit {
    fn from(name: &str) -> Self {
        Unit(name.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitSet(pub HashMap<Unit, i32>);

impl From<Unit> for UnitSet {
    fn from(unit: Unit) -> UnitSet {
        let mut unit_map: HashMap<Unit, i32> = HashMap::new();
        unit_map.insert(unit.into(), 1);
        UnitSet(unit_map)
    }
}

#[macro_export]
macro_rules! units {
    ($($from:literal to $to:literal),*) => {
        {
            use std::collections::HashMap;
            let mut tmp_map: HashMap<Unit, i32> = HashMap::new();
            $(
                tmp_map.insert($from.into(), $to);
            )*
            UnitSet(tmp_map)
        }
    } ;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    num: f64,
    units: UnitSet,
}

impl Value {
    pub fn new<U: Into<UnitSet>>(num: f64, units: U) -> Value {
        Value {
            num,
            units: units.into(),
        }
    }

    pub fn simple<U: Into<String>>(num: f64, units: U) -> Value {
        let unit_str: String = units.into();
        Value {
            num,
            units: UnitSet::from(Unit(unit_str)),
        }
    }

    pub fn unitless(num: f64) -> Value {
        Value {
            num,
            units: UnitSet(HashMap::new()),
        }
    }

    pub fn zero() -> Value {
        Self::unitless(0.0)
    }
}
