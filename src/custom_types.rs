#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

struct Point {
    x: f32, 
    y: f32
}

struct Pair(i32, f32);

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

pub fn execute_custom_types(){

    let name = String::from("Favour");
    let age: u8 = 20;

    let person = Person{name, age};
    println!("person: {:?}", person);

    let x: f32 = 36.0;
    let y: f32 = 36.0;

    let point = Point{x, y};
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    let bottom_right = Point {x: 25.0, ..another_point};

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point{x: left_edge, y: top_edge} = another_point;
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageLoadFailed,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn inspect(event: WebEvent, stage: Stage, role: Role) {

    //
    use crate::custom_types::Role::*;
    use crate::custom_types::Stage::{Beginner};

    match stage {
        Beginner => println!("Beginner stage"),
        Stage::Advanced => println!("Advanced stage"),
    }

    match role {
        Student => {
            println!("Student role");
        },
        _ => {
            println!("Any role");
        }
    }

    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        // Destructure `Click` into `x` and `y`.
        WebEvent::PageLoadFailed => println!("page load failed"),
        WebEvent::Paste(value) => println!("Pasted value: {}", value),
        WebEvent::Click{x, y} => println!("Clicked at x: {} and at y: {}", x, y),
    
    }
}

// enum with implicit discriminator (starts at 0)
enum Nice {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

//A possibly mutable variable with 'static lifetime. 
//The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
static LANGUAGE :&str = "Rustacean";

//An unchangeable value (the common case).
const THRESHOLD: i32 = 10;

pub fn execute_enums() {


    inspect(WebEvent::PageLoadFailed, Stage::Advanced, Role::Teacher);
    inspect(WebEvent::PageLoad, Stage::Advanced, Role::Teacher);
    inspect(WebEvent::PageUnload, Stage::Advanced, Role::Teacher);
    inspect(WebEvent::KeyPress('f'), Stage::Advanced, Role::Teacher);
    inspect(WebEvent::Paste(String::from("pasted")), Stage::Advanced, Role::Teacher);

    println!("zero is {}", Nice::Zero as i32);
    println!("one is {}", Nice::One as i32);

    println!("roses are #{:06x}", Color::Red as u32);
    println!("violets are #{:06x}", Color::Blue as u32);
}