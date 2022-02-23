// mod print;
// mod types;
// mod vars;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
mod cli;

fn main() {
    // print::run();

    println!(
        "Binary: {:b} Hex:{:x} Octal: {:o}",
        0b010101, 0x010101, 0o010101
    );

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
    // enums::run();
    cli::run();
}
