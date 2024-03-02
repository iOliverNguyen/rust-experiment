use std::ops::Deref;

#[derive(Clone)]
pub struct Planet {
    pub name: String,
    pub desc: String,
}

impl Planet {
    fn new(name: &str, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}

pub struct ListPlanets {
    pub items: Vec<Planet>,
    pub default_index: usize,
}

impl Deref for ListPlanets {
    type Target = [Planet];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl ListPlanets {
    pub fn init() -> Self {
        let planets = vec![
Planet::new("Mercury", "Mercury is the smallest and closest planet to the Sun. It has a rocky surface marked with craters similar to the Moon. Because of its proximity to the Sun, it experiences extreme temperature variations"),
Planet::new("Venus","Venus is similar in size and structure to Earth but is shrouded in a thick, toxic atmosphere that traps heat, leading to surface temperatures hot enough to melt lead. Its surface is volcanic and covered in sulfuric acid clouds."),
Planet::new("Earth", "Earth is the only planet known to support life, with a diverse ecosystem. It has a unique atmosphere composed of nitrogen, oxygen, and other gases, liquid water on its surface, and a varied terrain including mountains, valleys, and oceans."),
Planet::new("Mars", "Mars, known as the Red Planet due to its reddish appearance, has the tallest volcano and the deepest, longest canyon in the Solar System. It has water ice at its poles and evidence suggests it once had liquid water on its surface."),
Planet::new("Jupiter", "Jupiter is the largest planet in the Solar System. A gas giant, it has a Great Red Spot, a giant storm that has raged for hundreds of years. Jupiter has a strong magnetic field and at least 79 moons, including the four large Galilean moons: Io, Europa, Ganymede, and Callisto."),
Planet::new("Saturn", "Saturn is best known for its stunning ring system, made up of ice and rock particles. It is a gas giant like Jupiter, with a composition mainly of hydrogen and helium. Saturn has numerous moons, with Titan being the largest and having a thick atmosphere."),
Planet::new("Uranus", "Uranus is unique for its tilted axis, which causes it to rotate on its side. It is an ice giant with a colder atmosphere containing water, ammonia, and methane. Uranus has a faint ring system and 27 known moons."),
Planet::new("Neptune", "Neptune, an ice giant, is known for its deep blue color, caused by methane in its atmosphere. It has the strongest winds of any planet in the Solar System. Neptune has 14 known moons, with Triton being the largest and geologically active."),
        ];
        let default_index = planets
            .iter()
            .enumerate()
            .find_map(|(idx, planet)| {
                if &planet.name == "Earth" {
                    Some(idx)
                } else {
                    None
                }
            })
            .unwrap();
        ListPlanets {
            items: planets,
            default_index,
        }
    }
}
