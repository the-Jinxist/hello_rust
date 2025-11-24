mod primitives;
mod custom_types;
mod variable_bindings;
mod types;
mod conversion;

fn main() {
    println!("Hello, Rust!");
    println!("I'm a rustacean!");

    // primitives::execute_primitives();
    // primitives::execute_lieral_operators();
    // primitives::execute_tuple();
    // primitives::execute_array();

    // custom_types::execute_custom_types();
    // custom_types::execute_enums();

    // variable_bindings::execure_variable_bindings();

    types::excute_types();
    variable_bindings::execure_variable_bindings();

    conversion::execute_conversion();
}
