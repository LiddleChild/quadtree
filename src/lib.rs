pub mod particle;
pub mod quadtree;
pub mod world;

pub mod utils {
    use macroquad::prelude::{vec2, Vec2};

    pub fn point_to_vec2(point: (f32, f32)) -> Vec2 {
        vec2(point.0, point.1)
    }
}
