pub fn run() {
    let mut s = String::from("Hello");

    takes_ownership(s.clone());

    let x = 5;

    makes_copy(x);

    let len = calculate_length(&mut s);
    println!("The length of \"{}\" is {}", s, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}

/*
해제될 메모리의 참조자를 반환할 수 없음

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

아래 코드는 함수 외부로 이동하게 되므로 가능
fn dangle() -> String {
    let s = String::from("Hello");
    s
}
*/