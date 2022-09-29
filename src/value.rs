use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
enum Operation {
    None,
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub struct Value<T> {
    data: T,
    grad: f64,
    other: Option<Box<Value<T>>>,
    operation: Operation
    // _backward: Fn
}

// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output {
//     }
// }

impl Into<Option<Box<Value<f64>>>> for Value<f64> {
    fn into(self) -> Option<Box<Value<f64>>> {
        Some(Box::new(self))
    }
}

impl Add for Value<f64> {
    type Output = Value<f64>;
    fn add(self, other: Self) -> Self::Output {
        let data = self.data + other.data;
        // Value { data  }
        Value {
            data,
            grad: 0.0,
            other: other.into(),
            operation: Operation::Sub
        }
    }
}

impl Sub for Value<f64> {
    type Output = Value<f64>;
    fn sub(self, other: Self) -> Self::Output {
        let data = self.data - other.data;
        Value {
            data,
            grad: 0.0,
            other: other.into(),
            operation: Operation::Sub
        }
    }
}

impl Mul for Value<f64> {
    type Output = Value<f64>;
    fn mul(self, other: Self) -> Self::Output {
        let data = self.data * other.data;
        Value {
            data,
            grad: 0.0,
            other: other.into(),
            operation: Operation::Mul
        }
    }
}

impl Value<f64> { 
    fn backward(self) -> () {
        match self.operation {
            Operation::None => {

            }
            Operation::Add => {}
            Operation::Sub => {}
            Operation::Mul => {}

        };

    }
    pub fn new(data: f64) -> Self {
        Value {
            data,
            grad: 0.0,
            other: None,
            operation: Operation::None
        }
    }
}
