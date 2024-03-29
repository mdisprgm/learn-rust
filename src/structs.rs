// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// Tuple Struct
struct ColorTup(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    // I hate explicit self parameter
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn associated_func() {
        println!("maybe just a static function");
    }
}

pub fn run() {
    let c = Color {
        r: 0xfa,
        g: 0xde,
        b: 0x85,
    };

    println!("Color: {} {} {}", c.r, c.g, c.b);

    let ct = ColorTup(250, 222, 133);
    println!("ColorTuple: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("J", "HS");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Full Name: {}", p.full_name());

    p.set_last_name(".HS");
    println!("Full Name: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());

    let sq = Rectangle {
        height: 100,
        width: 30,
    };
    println!("{} {}", sq.height, sq.width);
    println!("{}", sq.area());
    println!("{:?}", sq);
    println!("{:#?}", sq);
    Rectangle::associated_func();
}
