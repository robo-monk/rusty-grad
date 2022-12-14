use std::ops::{Add, Sub, Mul};
use std::collections::HashSet;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Operation {
    None,
    Add,
    Sub,
    Mul,
}

static VALUE_COUNTER: AtomicUsize = AtomicUsize::new(0);
#[derive(Debug, Clone, PartialEq)]
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

    // def backward(self):

    //     # topological order all of the children in the graph
    //     topo = []
    //     visited = set()
    //     def build_topo(v):
    //         if v not in visited:
    //             visited.add(v)
    //             for child in v._prev:
    //                 build_topo(child)
    //             topo.append(v)
    //     build_topo(self)

    //     # go one variable at a time and apply the chain rule to get its gradient
    //     self.grad = 1
    //     for v in reversed(topo):
    //         v._backward()

    fn backward(self) -> () {
        let mut _topological_map: Vec<Value<f64>> = Vec::new();
        let mut _visited: HashSet<Value<f64>> = HashSet::new();

        fn build_topological_map(v: Value<f64>, topo: Vec<Value<f64>>, visited: HashSet<Value<f64>>){
            visited.contains(v);
            // if visited
            // if visited
        }

    }

    fn _backward(self) -> () {
        match self.operation {
            Operation::None => {}
            Operation::Add => {
                if self.children.0.is_some() {
                    self.children.0.unwrap().grad +=  self.grad;
                }
                if self.children.1.is_some() {
                    self.children.1.unwrap().grad +=  self.grad;
                }
            }
            Operation::Sub => {}
            Operation::Mul => {}

        };

    }

    pub fn label(&mut self, label: &str) -> () {
        self.label = label.to_string();
    }

    pub fn new(data: f64) -> Self {
       Value(VALUE_COUNTER.fetch_add(1, Ordering::SeqCst));

        Value {
            data,
            grad: 0.0,
            children: (None, None),
            label: "".to_string(),
            operation: Operation::None
        }
    }
}
