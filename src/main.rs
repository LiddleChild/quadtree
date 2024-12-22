use macroquad::prelude::*;
use quadtree::{utils::point_to_vec2, world::World};

const NUM_PARTICLE: usize = 4000;
const PARTICLE_SIZE: f32 = 3.0;

const CREATE_PARTICLE_LIMITER: f64 = 0.01;

#[macroquad::main("Quad tree")]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());

    for _ in 0..NUM_PARTICLE {
        world.create_particle(PARTICLE_SIZE);
    }

    let mut last_fps_counted = 0.0;
    let mut raw_fps = 0.0;
    let mut fps = 0.0;

    let mut last_particle_created = 0.0;

    loop {
        // update
        if get_time() - last_fps_counted >= 1.0 {
            last_fps_counted = get_time();
            fps = raw_fps;
            raw_fps = 0.0;
        } else {
            raw_fps += 1.0;
        }

        if is_mouse_button_down(MouseButton::Left)
            && get_time() - last_particle_created >= CREATE_PARTICLE_LIMITER
        {
            world.create_particle_at(point_to_vec2(mouse_position()), PARTICLE_SIZE);
            last_particle_created = get_time();
        }

        world.update(get_frame_time());

        // render
        clear_background(BLACK);

        for bound in &world.quadtree_bound() {
            draw_rectangle_lines(bound.x, bound.y, bound.w, bound.h, 1.0, DARKGRAY);
        }

        for particle in world.particles() {
            let mut color = DARKGRAY;

            for other_particle in world.query_particle(particle, particle.size) {
                if particle.id != other_particle.id && particle.collides(&other_particle) {
                    color = WHITE;
                    break;
                }
            }

            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.size,
                color,
            );
        }

        draw_text(format!("{} FPS", fps).as_str(), 5.0, 20.0, 20.0, GREEN);

        let particle_count_text = format!("{} particles", world.count_particle());
        let text_dim = measure_text(particle_count_text.as_str(), None, 20, 1.0);
        draw_text(
            particle_count_text.as_str(),
            screen_width() - text_dim.width - 5.0,
            20.0,
            20.0,
            GREEN,
        );

        next_frame().await;
    }
}
