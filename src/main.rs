mod value;
use value::Value;

fn main() {
    println!("Hello, world!");
    let a = Value::new(1.0); 
    let b = Value::new(0.5);
    let c = Value::new(2.5);
    let d = Value::new(25.95);

    let res = a + b + c * d ;
    dbg!(res);
    // println!("result is {:?}", res);
}
