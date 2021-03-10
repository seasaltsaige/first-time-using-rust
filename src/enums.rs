enum Movement {
  Up,
  Down,
  Left,
  Right
}

fn move_player(m: Movement) {
  match m {
    Movement::Up => println!("Going Up"),
    Movement::Left => println!("Going Left"),
    Movement::Right => println!("Going Right"),
    Movement::Down => println!("Going Down")
  }
}

pub fn run() {
  let avatar1 = Movement::Left;
  let avatar2 = Movement::Up;
  let avatar3 = Movement::Right;
  let avatar4 = Movement::Down;

  move_player(avatar1);
  move_player(avatar2);
  move_player(avatar3);
  move_player(avatar4);
}