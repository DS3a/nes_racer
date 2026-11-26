use nalgebra::{self, Vector2};

// At the top level of main.rs
pub(crate) const BASE_SPEED: f64 = 1.0;
pub(crate) const BOOST_MUL: f64 = 1.2;

pub struct Racer {
    id: u32,
    heading: f64, // the angle with respect to the x axis. the x axis points right, and the y axis, up.
    base_speed: f64,
    closest_point_spline: f64,
    position: nalgebra::Vector2<f64>,
    is_started: bool,
    is_boosted: bool,
    boost_mul: f64,
}

impl Racer {
    pub fn new(id: u32) -> Racer {
        return Racer { id, 
            heading: 0f64, 
            base_speed: BASE_SPEED, 
            closest_point_spline: 0f64,
            position: nalgebra::Vector2::<f64>::new(0f64, 0f64), 
            is_started: false, 
            is_boosted: false, 
            boost_mul: BOOST_MUL }
    }

    pub(crate) fn id(&self) -> u32 {
        self.id
    }

    pub(crate) fn position(&self) -> &Vector2<f64> {
        &self.position
    }

    pub(crate) fn heading(&self) -> f64 {
        self.heading
    }

    pub fn change_heading(&mut self, new_heading: f64) -> &f64 {
        self.heading = new_heading;

        return &self.heading;
    }

    pub fn heading_step(&mut self, step: f64) -> &f64 {
        self.heading += step;

        return &self.heading;
    }

    fn get_speed(&self) -> f64 {
        let mut speed: f64 = self.base_speed;

        if (self.is_boosted) {
            speed *= self.boost_mul;
        }

        return speed;
    }

    pub fn propagate(&mut self, timestep: f64) -> &nalgebra::Vector2<f64> {
        let speed: f64 = self.get_speed();

        let mut vel: nalgebra::Vector2<f64> = Default::default();
        vel.x = speed * self.heading.cos();
        vel.y = speed * self.heading.sin(); 


        self.position += vel * timestep;

        return &self.position;
    }
}