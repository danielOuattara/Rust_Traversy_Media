// Enums are types which have a few definite values

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(movement: Movement) {
    // Perform action depending on info
    match movement {
        Movement::Up => println!("avatar move up"),
        Movement::Down => println!("avatar move down"),
        Movement::Left => println!("avatar move left"),
        Movement::Right => println!("avatar move right"),
    }
}

pub fn run() {
    let avatar_1 = Movement::Left;
    let avatar_2 = Movement::Right;
    let avatar_3 = Movement::Up;
    let avatar_4 = Movement::Down;

    move_avatar(avatar_1);
    move_avatar(avatar_2);
    move_avatar(avatar_3);
    move_avatar(avatar_4);
}
