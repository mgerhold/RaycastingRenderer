use crate::vector2::Vector2;

pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
}

impl Default for Player {
    fn default() -> Self {
        Self { position: Default::default(), direction: Vector2{ x: 0.0, y: 1.0} }
    }
}