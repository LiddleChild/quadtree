use std::f32::consts::PI;

use macroquad::prelude::*;
use rand::srand;

use crate::{
    particle::Particle,
    quadtree::{QuadTree, QuadTreeIter},
};

pub struct World {
    width: f32,
    height: f32,
    particles: QuadTree<Particle>,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        let particles = QuadTree::new(width, height, 256);

        srand(42069);

        Self {
            width,
            height,
            particles,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            particle.update(delta_time);

            let lower_bound = particle.size / 2.0;
            let upper_bound = (self.width as f32) - particle.size / 2.0;
            if particle.position.x < lower_bound || particle.position.x > upper_bound {
                particle.velocity.x = -particle.velocity.x;
                particle.position.x = clamp(particle.position.x, lower_bound, upper_bound);
            }

            let upper_bound = (self.height as f32) - particle.size / 2.0;
            if particle.position.y < lower_bound || particle.position.y > upper_bound {
                particle.velocity.y = -particle.velocity.y;
                particle.position.y = clamp(particle.position.y, lower_bound, upper_bound);
            }
        }

        self.particles.update();
    }

    pub fn create_particle_at(&mut self, position: Vec2, size: f32) {
        let rotation = rand::gen_range(0.0, PI * 2.0);
        let velocity =
            vec2(rotation.cos(), rotation.sin()).normalize() * rand::gen_range(50.0, 150.0);

        self.particles.push(Particle::new(
            self.particles.count(),
            position,
            size,
            velocity,
        ));
    }

    pub fn create_particle(&mut self, size: f32) {
        let position = vec2(
            rand::gen_range(size / 2.0, self.width - size / 2.0),
            rand::gen_range(size / 2.0, self.height - size / 2.0),
        );

        self.create_particle_at(position, size);
    }

    pub fn count_particle(&self) -> usize {
        self.particles.count()
    }

    pub fn particles(&self) -> QuadTreeIter<&Particle> {
        self.particles.iter()
    }

    pub fn query_particle(&self, particle: &Particle, range: f32) -> Vec<&Particle> {
        self.particles.query(particle.position, range)
    }

    pub fn quadtree_bound(&self) -> Vec<&Rect> {
        self.particles.bounds()
    }
}
