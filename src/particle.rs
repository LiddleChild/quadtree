use macroquad::math::Vec2;

use crate::quadtree::Positionable;

#[derive(Debug, Clone)]
pub struct Particle {
    pub id: usize,
    pub position: Vec2,
    pub size: f32,
    pub velocity: Vec2,
}

impl Particle {
    pub fn new(id: usize, position: Vec2, size: f32, velocity: Vec2) -> Self {
        Self {
            id,
            position,
            size,
            velocity,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    pub fn collides(&self, other: &Self) -> bool {
        (self.position - other.position).length() <= self.size + other.size
    }

    pub fn elastic_collision(&mut self, other: &Self) {
        let correction = (self.size + other.size - (self.position - other.position).length()) / 2.0;
        self.position += (self.position - other.position).normalize() * correction;

        let v1 = self.velocity;
        let v2 = other.velocity;
        let t1 = self.position - other.position;

        self.velocity = v1 - ((v1 - v2).dot(t1) / t1.length().powf(2.0)) * t1;
    }
}

impl Positionable for Particle {
    fn position(&self) -> Vec2 {
        self.position
    }
}
