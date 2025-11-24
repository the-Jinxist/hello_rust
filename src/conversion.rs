use std::convert::From;
use std::convert::Into;


#[derive(Debug)]
struct Number {
    value: i32
}

impl From<String> for Number {
    fn from(value: String) -> Self {
        return Number { value: 22}
    }
}

// impl Into<Number> for String {
//     fn into(&self) -> Number {
//         return Number { value: 33 }
//     }
// }

pub fn execute_conversion(){

    let num = Number::from(String::from("34"));
    println!("My number is {:?}", num);

}