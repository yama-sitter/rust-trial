// mod vars;
// mod stack_heap;
// mod ownership;
// mod lifetime;
// mod generics;
// mod structs;
// mod enums;
// mod traits;
extern crate demo;
mod debug;
mod error_handling;
mod unit_test;

fn main() {
    // println!("Hello, world!");
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // lifetime::run();
    // generics::run();
    // structs::run();
    // enums::run();
    // traits::run();
    // error_handling::run();
    debug::run();
    demo::print_random_number();
}
