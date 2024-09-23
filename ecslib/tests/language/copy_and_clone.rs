use std::ops::Add;

#[test]
fn copy_and_clone(){
  let mut player_location = Vector2 { x: 10.0, y: 15.0 };
  dbg!(player_location);
  player_location.x += 1.0;
  dbg!(player_location);

  let player_velocity = Vector2 { x: 1.0, y: 2.0 };
  let new_location = player_location + player_velocity;
  dbg!(new_location);
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
  pub x: f32,
  pub y: f32
}

impl Add for Vector2 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y
    }
  }
}