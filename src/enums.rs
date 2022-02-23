enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending
    match m {
        Movement::Up => {
            println!("Avatar movin Up")
        }
        Movement::Down => {
            println!("Avatar movin Down")
        }
        Movement::Left => {
            println!("Avatar movin Left")
        }
        Movement::Right => {
            println!("Avatar movin Right")
        }
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
