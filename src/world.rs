use std::fmt;

#[derive(PartialEq, Eq)]
pub enum ParticleClass{
    SimpleParticle,
    SineParticle,
    GravityParticle,
}


impl fmt::Display for ParticleClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParticleClass::SimpleParticle => write!(f, "SimpleParticle"),
            ParticleClass::SineParticle => write!(f, "SineParticle"),
            ParticleClass::GravityParticle => write!(f, "GravityParticle"),
        }
    }
}
pub struct World{
    pub size: usize,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub particle_class: Vec<ParticleClass>,
}


impl World {
    pub fn new() -> Self {   
        Self{
            size: 0,
            x: Vec::new(),
            y: Vec::new(),
            particle_class: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, x: f32, y: f32, class: ParticleClass) -> usize{
        self.x.push(x);
        self.y.push(y);
        self.particle_class.push(class);
        self.size = self.particle_class.len();
        return self.size;
    }

    pub fn printparticle(&self, id: usize){
        println!("x: {}", self.x[id]);
        println!("y: {}", self.y[id]);
        println!("class: {}", self.particle_class[id]);
    }
}