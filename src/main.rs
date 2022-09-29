mod value;
use value::Value;

fn main() {
    println!("Hello, world!");
    let mut a = Value::new(1.0); a.label("a");
    let mut b = Value::new(0.5); b.label("b");
    let mut c = Value::new(2.5); c.label("c");
    let mut d = Value::new(25.95); d.label("d");

    // let res = a + b + c * d ;
    let res = a + b + c;
    dbg!(res);
    // println!("result is {:?}", res);
}
