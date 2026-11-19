use crate::world;
use crate::gravity_world;
use macroquad::math;



pub struct GravityParticleSystem {}

impl GravityParticleSystem{

    pub fn new() -> Self{
        GravityParticleSystem {}
    }


    pub fn process(&self, world: &mut world::World, gravity_world: &mut gravity_world::GravityWorld, dt: f32){
        let size = world.size;
        for i in 0..size {
            if world.particle_class[i] == world::ParticleClass::GravityParticle {
                let gsize = gravity_world.size;
                for j in 0..gsize{
                    if gravity_world.points_x[j] == world.x[i] && gravity_world.points_y[j] == world.y[i]{
                        continue;
                    }

                    let position = math::vec2(world.x[i], world.y[i]);
                    let subj_pos = math::vec2(gravity_world.points_x[j], gravity_world.points_y[j]);
                    let to_target = subj_pos - position;

                    let distance = to_target.length();

                    let radius = gravity_world.radius_attract[j];
                    if distance < radius{
                        let force = (1.0 - distance / radius) * 200.0;
                        let a = to_target / distance;
                        world.x[i] += a.x * force * dt;
                        world.y[i] += a.y * force * dt;
                    }

                    
                }
            }
            gravity_world.points_x[i] = world.x[i];
            gravity_world.points_y[i] = world.y[i];

        }

        

    }
}