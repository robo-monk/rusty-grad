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
    operation: Operation,
    label: String,
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
    fn add(mut self, other: Self) -> Self::Output {
        let data = self.data + other.data;
        // Value { data  }
        self.data = data;
        self.other = other.into();
        self.operation = Operation::Add;
        self
    }
}

impl Sub for Value<f64> {
    type Output = Value<f64>;
    fn sub(mut self, other: Self) -> Self::Output {
        let data = self.data - other.data;
        self.data = data;
        self.other = other.into();
        self.operation = Operation::Sub;
        self
    }
}

impl Mul for Value<f64> {
    type Output = Value<f64>;
    fn mul(mut self, other: Self) -> Self::Output {
        let data = self.data * other.data;
        self.data = data;
        self.other = other.into();
        self.operation = Operation::Mul;
        self
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

impl Value<f64> { 
    fn backward(&mut self) -> () {
        match self.operation {
            Operation::None => {}
            Operation::Add => {
                self.grad += self.other.as_ref().unwrap().grad
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
            other: None,
            label: "".to_string(),
            operation: Operation::None
        }
    }

    // pub fn new(data: f64, label: str) -> Self {
    //     Value {
    //         data,
    //         grad: 0.0,
    //         other: None,
    //         operation: Operation::None
    //     }
    // }
}
