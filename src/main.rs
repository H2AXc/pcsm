
mod world;
use world::{World, ParticleClass};
mod gravity_world;



mod gravityParticleSystem;
use gravityParticleSystem::{GravityParticleSystem};


use macroquad::prelude::*;

use crate::gravity_world::GravityWorld;
#[macroquad::main("My game")]
async fn main(){
    let mut world: World = World::new();
    let mut grav_world: GravityWorld = GravityWorld::new();
    
    let mut gv_sys: gravityParticleSystem::GravityParticleSystem = gravityParticleSystem::GravityParticleSystem::new();

    for i in 0..2 {

        let x= macroquad::rand::gen_range(0.0, 100.0);
        let y= macroquad::rand::gen_range(0.0, 100.0);

        
        
        world.add_particle(x, y, world::ParticleClass::GravityParticle);
        grav_world.points_x.push(x);
        grav_world.points_y.push(y);
        grav_world.radius_attract.push(150.0);
        grav_world.size += 1;
    }




    loop{

        let dt = get_frame_time().min(0.05);

        clear_background(GRAY);

        gv_sys.process(&mut world, &mut grav_world, dt);

        for  i in 0..world.size {
            draw_rectangle(world.x[i], world.y[i], 10.0, 10.0, RED);
        }


        
        next_frame().await;
    }
}




