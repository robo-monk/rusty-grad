mod value;
use value::Value;

fn main() {
    println!("Hello, world!");
    let a = Value::new(1.0); 
    let b = Value::new(0.5);

    let res = a + b;
    dbg!(res);
    // println!("result is {:?}", res);
}
