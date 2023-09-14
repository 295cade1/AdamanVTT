struct TokenID(u32);
struct Position(f32, f32);

pub struct Order {
  pub command: Command,
} 

pub enum Command {
    Move(TokenID, Position),
}

