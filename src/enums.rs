//Enums are types which have a few definite values 


enum Movement {
    //Variants 
    Up,
    Down,
    Left,
    Right
}


fn move_avatar(m:Movement) {
    //Perform action depending on info
    match m {
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving down"),
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right")
    }
}
pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);




}