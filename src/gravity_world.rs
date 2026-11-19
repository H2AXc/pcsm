
pub struct GravityWorld{
    pub size: usize,
    pub points_x: Vec<f32>,
    pub points_y: Vec<f32>,
    pub radius_attract: Vec<f32>,
    pub radius_repel: Vec<f32>
}

impl GravityWorld{
    pub fn new() -> Self{
        Self { size: 0, points_x: Vec::new(), points_y: Vec::new(), radius_attract: Vec::new(), radius_repel: Vec::new() }
    }
}

