use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug)]
pub struct Value {
    data: f64,
}

impl Value {
    pub fn new(val: f64) -> Value {
        return Value { data: val };
    }
}

impl Add for &Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        return Value { data: self.data + rhs.data };
    }
}

impl Mul for &Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        return Value { data: self.data * rhs.data };
    }
}

impl Sub for &Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        return Value { data: self.data - rhs.data };
    }
}

impl Div for &Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        return Value { data: self.data / rhs.data };
    }
}