extern crate communicator;

// mod arrays;
// mod cli;
// mod conditionals;
mod enums;
// mod functions;
// mod loops;
// mod ownerships;
// mod pointer_ref;
// mod print;
// mod strings;
// mod structs;
// mod tuples;
// mod types;
// mod vars;
// mod vectors;

fn main() {
    // print::run();
    println!("Binary: {:b} Hex:{:x} Octal: {:o}", 0b010101, 0x010101, 0o010101);

    println!("{:?}", (12, true, "Hello"));
    // vars::run();
    // types::run();

    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    enums::run();
    // cli::run();
    // ownerships::run();
}
