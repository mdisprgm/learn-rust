enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
}
impl IpAddr {
    fn foo(&self) {
        println!("{:?}", self);
    }
}

fn move_avatar(m: Movement) {
    // Perform action depending
    match m {
        Movement::Up => { println!("Avatar moving Up") }
        Movement::Down => { println!("Avatar moving Down") }
        Movement::Left => { println!("Avatar moving Left") }
        Movement::Right => { println!("Avatar moving Right") }
    }
}

#[allow(dead_code)]
enum Test {
    A,
    B,
    C,
    D,
}

pub fn run() {
    let a = IpAddr::V4(String::from("10.10.10.10"));
    println!("{:?}", a);
    a.foo();
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    println!("{} {:?}", x, y);
    println!("{}, {}", y.is_none(), y.is_some());

    if y.is_some() {
        println!("{}", y.unwrap());
    }
    let some_value = Test::A;
    if let Test::C = some_value {
        println!("It's C");
    }
}
