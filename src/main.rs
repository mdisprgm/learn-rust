mod print;

fn main() {
    print::run();

    println!(
        "Binary: {:b} Hex:{:x} Octal: {:o}",
        0b010101, 0x010101, 0o010101
    );

    println!("{:?}", (12, true, "Hello"));
}
