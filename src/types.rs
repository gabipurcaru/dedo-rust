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

    pub fn convert_units(&self, value: &Value, new_units: &UnitSet) -> Value {
        let mut converted = (*value).clone();

        // TODO: conversions e.g. 1km/h to 0.28m/s etc.

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
}

// Utility to help create a static environment. Basic syntax is
// environment!["some_unit" to "some_other_unit" is 123.456, ...]
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

#[derive(Debug, Clone, PartialEq)]
pub struct UnitSet(pub HashMap<Unit, i32>);

impl UnitSet {
    pub fn single(unit: &'static str) -> UnitSet {
        let mut unit_map = HashMap::new();
        unit_map.insert(Unit(unit.to_string()), 1);
        UnitSet(unit_map)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    num: f64,
    units: UnitSet,
}

impl Value {
    pub fn new(num: i32, unit: Unit) -> Value {
        let mut unit_map = HashMap::new();
        unit_map.insert(Unit("km".to_string()), num);

        let units = UnitSet(unit_map);

        Value {
            num: num as f64,
            units,
        }
    }
}
