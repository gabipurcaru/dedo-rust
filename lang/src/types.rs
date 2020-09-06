use std::collections::HashMap;
use std::string::String;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub conversions: Conversions,
    values: Vec<Result<Value, ()>>,
    vars: HashMap<String, Value>,
}

impl Environment {
    pub fn new(conversions: &Conversions) -> Environment {
        Environment {
            conversions: Environment::expand_conversions(conversions),
            values: Vec::new(),
            vars: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, val: Result<Value, ()>) {
        self.values.push(val);
    }

    fn conversion_ratio(&self, from: &Unit, to: &Unit) -> Result<f64, String> {
        self.conversions
            .0
            .get(&(from.clone(), to.clone()))
            .map(|ratio| ratio.clone())
            .ok_or(format!("Cannot convert from {:?} to {:?}", from, to))
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

    fn expand_conversions(basic_conversions: &Conversions) -> Conversions {
        let mut conversions = HashMap::new();

        // Add a -> b, b -> a, a -> a and b -> b conversions
        for conversion in basic_conversions.clone().0 {
            match conversion {
                ((from, to), ratio) => {
                    conversions.insert((from.clone(), to.clone()), ratio.clone());
                    conversions.insert((to.clone(), from.clone()), 1. / ratio);
                    conversions.insert((from.clone(), from.clone()), 1.);
                    conversions.insert((to.clone(), to.clone()), 1.);
                }
            }
        }

        // for all a -> b and b -> c, add a -> c, aka transitive conversions

        loop {
            // loop until no changes are made anymore
            let mut is_saturated = true;

            let mut new_conversions = HashMap::new();
            for ((left_from, left_to), left_ratio) in &conversions {
                for ((right_from, right_to), right_ratio) in &conversions {
                    let key = (left_from.clone(), right_to.clone());
                    if left_to == right_from
                        && !conversions.contains_key(&key)
                    {
                        new_conversions.insert(
                            key,
                            left_ratio * right_ratio,
                        );
                        is_saturated = false;
                    }
                }
            }

            if is_saturated {
                break;
            }

            conversions.extend(new_conversions);
        }

        Conversions(conversions)
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
            num: 1.0 / right.num,
            units: UnitSet(inverted_units),
        };

        self.mul(left, inverted_right)
    }

    /// Unit powers
    ///
    /// Example:
    /// ```
    /// # use std::collections::HashMap;
    /// # use dedo_rust::*;
    /// # use dedo_rust::types::*;
    /// # use dedo_rust::defaults::*;
    /// # let env = ENVIRONMENT.clone();
    /// let res = env.pow(Value::simple(12.0, "usd"), Value::unitless(4.0));
    /// assert_eq!(res, Value::new(20736.0, units!("usd" to 4)));
    /// ```
    pub fn pow(&self, left: Value, right: Value) -> Value {
        let pow = right.num.round();

        let units = left
            .units
            .0
            .into_iter()
            .map(|(k, v)| (k, v * (pow as i32)))
            .collect();
        Value {
            num: left.num.powf(pow),
            units: UnitSet(units),
        }
    }

    pub fn ident(&self, ident: String) -> Result<Value, ()> {
        match (ident.as_ref(), self.vars.get(&ident)) {
            ("sum", _) => self.sum(),
            ("prod", _) => self.prod(),
            ("prev", _) => self.prev(),
            (_, Some(v)) => Ok(v.clone()),
            (units, _) => Ok(Value::simple(1.0, units)),
        }
    }

    pub fn sum(&self) -> Result<Value, ()> {
        let mut res: Result<Value, ()> = Err(());
        for row in self.values.iter().rev() {
            match (row.clone(), res.clone()) {
                (Ok(lhs), Ok(rhs)) => {
                    res = Ok(self.add(lhs, rhs));
                }
                (Ok(lhs), Err(())) => {
                    res = Ok(lhs.clone());
                }
                _ => break,
            }
        }
        res
    }

    pub fn prod(&self) -> Result<Value, ()> {
        let mut res: Result<Value, ()> = Err(());
        for row in self.values.iter().rev() {
            match (row.clone(), res.clone()) {
                (Ok(lhs), Ok(rhs)) => {
                    res = Ok(self.mul(lhs, rhs));
                }
                (Ok(lhs), Err(())) => {
                    res = Ok(lhs.clone());
                }
                _ => break,
            }
        }
        res
    }

    pub fn prev(&self) -> Result<Value, ()> {
        for row in self.values.iter().rev() {
            return row.clone();
        }
        return Err(());
    }

    pub fn assign<U: Into<String>>(&mut self, ident: U, value: Value) -> Result<Value, ()> {
        self.vars.insert(ident.into(), value.clone());
        Ok(value.clone())
    }

    pub fn convert(&self, value: Value, target: Value) -> Value {
        self.convert_units(&value, &target.units)
    }
}

/// Utility to help create a static environment. Basic syntax is
/// environment!["some_unit" to "some_other_unit" is 123.456, ...]
#[macro_export]
macro_rules! environment {
    ($($from:literal to $to:literal is $num:literal),*) => {
        {
            let mut tmp_map: std::collections::HashMap<(Unit, Unit), f64>
                = std::collections::HashMap::new();
            $(
                tmp_map.insert(($from.into(), $to.into()), $num as f64);
            )*
            Environment::new(&Conversions(tmp_map))
        }
    } ;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Conversions(pub HashMap<(Unit, Unit), f64>);

#[derive(Clone, Debug, Hash)]
pub struct Unit(pub String);

impl From<&str> for Unit {
    fn from(name: &str) -> Self {
        Unit(name.into())
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for Unit {}

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

    /// A value with no units and the given number
    ///
    /// Example:
    /// ```
    /// # use dedo_rust::types::*;
    /// let value = Value::unitless(14.1);
    /// ```
    pub fn unitless(num: f64) -> Value {
        Value {
            num,
            units: UnitSet(HashMap::new()),
        }
    }

    /// The zero value
    ///
    /// Example:
    /// ```
    /// # use dedo_rust::types::*;
    /// # use std::collections::HashMap;
    /// let value = Value::zero();
    /// assert_eq!(value, Value::unitless(0.0));
    /// ```
    pub fn zero() -> Value {
        Self::unitless(0.0)
    }

    /// Negates a value
    ///
    /// Example:
    /// ```
    /// # use dedo_rust::types::*;
    /// let value = Value::simple(123.0, "$").negate();
    /// let other_value = Value::negate(Value::simple(123.0, "$"));
    /// # assert_eq!(value, Value::simple(-123.0, "$"));
    /// # assert_eq!(other_value, Value::simple(-123.0, "$"));
    /// ```
    pub fn negate(self) -> Value {
        Value {
            num: -self.num,
            units: self.units,
        }
    }
}
