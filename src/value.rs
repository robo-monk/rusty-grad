use std::ops::Add;

#[derive(Debug)]
pub struct Value<T> {
    data: T
}

// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output {
//     }
// }

impl Add for Value<f64> {
    type Output = Value<f64>;
    fn add(self, rhs: Self) -> Self::Output {
        let data = self.data + rhs.data;
        Value { data }
    }
}

impl Value<f64> { 
    pub fn new(data: f64) -> Self {
        Value {
            data
        }
    }
}
