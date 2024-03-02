use misc::planets::ListPlanets;

pub struct SolarSystemApp {
    planets: Box<ListPlanets>,
}

impl SolarSystemApp {
    pub fn new() -> Self {
        Self {
            planets: Box::new(ListPlanets::init()),
        }
    }
}

fn main() -> Result<(), eframe::Error> {}
