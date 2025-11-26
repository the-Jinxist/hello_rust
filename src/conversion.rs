use std::convert::From;
use std::convert::TryFrom;


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

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber{num: 8}));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber{num: 8}));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    print_circe_display();
    parse_string_to_number();

    let polygon = Polygon::from_str("34").unwrap();
    println!("Polygon size: {}", polygon);

}

//Similar to From and Into, TryFrom and TryInto are generic traits for converting between types. 
//Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.

#[derive(Debug, PartialEq)]
struct EvenNumber {
    num: i32,
}

impl TryFrom<i32> for EvenNumber {

    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            Err(())
        } else {
            Ok(EvenNumber{
                num: value
            })
        }
    }
    
}

/*
Converting to String
To convert any type to a String is as simple as implementing the ToString trait for the type.
 Rather than doing so directly, you should implement the fmt::Display trait which automatically provides ToString 
 and also allows printing the type as discussed in the section on print!.

*/

use std::fmt;

struct Circle {
    radius: f64,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn print_circe_display() {
    print!("{}", Circle{radius: 3.0});
}

/*

It's useful to convert strings into many types, but one of the more common string operations is to convert them from string to number. 
The idiomatic approach to this is to use the parse function and either to arrange for type inference or to specify the type to parse using the 'turbofish' syntax.
 Both alternatives are shown in the following example.

This will convert the string into the type specified as long as the FromStr trait is implemented for that type. 
This is implemented for numerous types within the standard library

*/

fn parse_string_to_number() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let turbo_parsed_2 = "10".parse::<i32>();
    match turbo_parsed_2 {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Failed to parse number: {}", e),    
    }


    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

//To obtain this functionality on a user defined type simply implement the FromStr trait for that type.
use std::num::ParseIntError;
use std::str::FromStr;

struct Polygon {
    sized: i32,
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polygon with sides {}", self.sized)
    }
}

impl FromStr for Polygon {

    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<i32>();
        return match value {
            Ok(v) => Ok(Polygon { sized: v }),
            Err(e) => Err(e),  
        };
    }
}