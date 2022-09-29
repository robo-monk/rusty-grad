use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
enum Operation {
    None,
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone)]
pub struct Value<T> {
    label: String,
    data: T,
    grad: f64,
    children: (Option<Box<Value<T>>>, Option<Box<Value<T>>>),
    operation: Operation,
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

// def __add__(self, other):
//     other = other if isinstance(other, Value) else Value(other)
//     out = Value(self.data + other.data, (self, other), '+')

//     def _backward():
//         self.grad += out.grad
//         other.grad += out.grad
//     out._backward = _backward

//     return out
impl Add for Value<f64> {
    type Output = Value<f64>;
    fn add(self, other: Self) -> Self::Output {
        Value {
            data: self.data + other.data,
            label: format!("{}+{}", self.label, other.label),
            grad: 0.0,
            children: (self.into(), other.into()),
            operation: Operation::Add
        }
    }
}

impl Sub for Value<f64> {
    type Output = Value<f64>;
    fn sub(self, other: Self) -> Self::Output {
        Value {
            data: self.data - other.data,
            label: format!("{}-{}", self.label, other.label),
            grad: 0.0,
            children: (self.into(), other.into()),
            operation: Operation::Sub
        }
    }
}

impl Mul for Value<f64> {
    type Output = Value<f64>;
    fn mul(self, other: Self) -> Self::Output {
         Value {
            data: self.data * other.data,
            label: format!("{}*{}", self.label, other.label),
            grad: 0.0,
            children: (self.into(), other.into()),
            operation: Operation::Mul
        }
    }
}

impl Value<f64> { 
    fn backward(&mut self) -> () {
        match self.operation {
            Operation::None => {}
            Operation::Add => {
            }
            Operation::Sub => {}
            Operation::Mul => {}

        };

    }

    pub fn label(&mut self, label: &str) -> () {
        self.label = label.to_string();
    }

    pub fn new(data: f64) -> Self {
        Value {
            data,
            grad: 0.0,
            children: (None, None),
            label: "".to_string(),
            operation: Operation::None
        }
    }
}
