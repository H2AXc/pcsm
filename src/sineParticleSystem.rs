use rand::Rng;
use crate::world;
use macroquad::time::get_time;
pub struct SimpleParticleSystem{}

impl SineParticleSystem{
    pub fn new() -> Self{
        SineParticleSystem{}
    }

    pub fn process(&self, world: &mut world::World){
        let size = world.size;
        for i in 0..size {
            if world.particle_class[i] == world::ParticleClass::SineParticle {

                let time = get_time() as f32;

                let speed = 1.0;
                let freq = 0.2;
                let amplitude = 1.0;

                let phase = world.x[i] * freq + time * speed;
                let y_pos = world.y[i] + phase.sin() * amplitude;

                //world.x[i] += random_x;
                world.y[i] = y_pos;
            }
        }
    }
}


