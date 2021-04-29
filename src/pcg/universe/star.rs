use bevy::prelude::Color;
use rand::{rngs::StdRng, Rng};

#[derive(Debug)]
pub struct Star {
    // 1.0 = Mass of our sun
    pub mass: f32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    WhiteDwarf,
}

#[allow(dead_code)]
impl Star {
    const HABITABLE_ZONE_INNER_BOUND_RATIO: f32 = 0.53;
    const HABITABLE_ZONE_OUTER_BOUND_RATIO: f32 = 1.1;
    const MASS_TO_SURFACE_TEMPERATURE_EXP: f32 = 0.505;
    const MASS_TO_RADIUS_EXP_SMALLER_SUN: f32 = 0.8;
    const MASS_TO_RADIUS_EXP_BIGGER_SUN: f32 = 0.5;
    const ORBIT_INNER_BOUNDARY: f32 = 0.1;
    const ORBIT_OUTER_BOUNDARY: f32 = 40.0;
    const FROSTLINE_MULTIPLIER: f32 = 4.85;

    const TINY_MASS_LUMINOSITY_EXP: f32 = 2.7;
    const NORMAL_MASS_LUMINOSITY_EXP: f32 = 4.7;
    const BIGGER_MASS_LUMINOSITY_EXP: f32 = 3.1;
    const GIANT_MASS_LUMINOSITY_EXP: f32 = 1.6;

    pub fn from_mass(mass: f32) -> Self {
        Self { mass }
    }

    pub fn from_class(rng: &mut StdRng, class: StarClass) -> Self {
        match class {
            StarClass::O => Self::from_mass(rng.gen_range(16.0..32.0)),
            StarClass::B => Self::from_mass(rng.gen_range(2.1..16.0)),
            StarClass::A => Self::from_mass(rng.gen_range(1.4..2.1)),
            StarClass::F => Self::from_mass(rng.gen_range(1.04..1.4)),
            StarClass::G => Self::from_mass(rng.gen_range(0.8..1.04)),
            StarClass::K => Self::from_mass(rng.gen_range(0.45..0.8)),
            StarClass::M => Self::from_mass(rng.gen_range(0.08..0.45)),
            StarClass::WhiteDwarf => Self::from_mass(rng.gen_range(0.01..0.8)),
        }
    }

    fn class_for_mass(mass: f32) -> StarClass {
        if mass < 0.08 {
            StarClass::WhiteDwarf
        } else if mass < 0.45 {
            StarClass::M
        } else if mass < 0.8 {
            StarClass::K
        } else if mass < 1.04 {
            StarClass::G
        } else if mass < 1.4 {
            StarClass::F
        } else if mass < 2.1 {
            StarClass::A
        } else if mass < 16.0 {
            StarClass::B
        } else {
            StarClass::O
        }
    }

    pub fn class(&self) -> StarClass {
        Self::class_for_mass(self.mass)
    }

    pub fn luminosity(&self) -> f32 {
        if self.mass <= 0.1 {
            self.mass.powf(Self::TINY_MASS_LUMINOSITY_EXP)
        } else if self.mass <= 1.0 {
            self.mass.powf(Self::NORMAL_MASS_LUMINOSITY_EXP)
        } else if self.mass <= 10.0 {
            self.mass.powf(Self::BIGGER_MASS_LUMINOSITY_EXP)
        } else {
            self.mass.powf(Self::GIANT_MASS_LUMINOSITY_EXP)
        }
    }

    pub fn color(&self) -> Color {
        match self.class() {
            StarClass::O => Color::rgba_u8(155, 176, 255, 1),
            StarClass::B => Color::rgba_u8(170, 191, 255, 40),
            StarClass::A => Color::rgba_u8(202, 215, 255, 100),
            StarClass::F => Color::rgba_u8(248, 247, 255, 100),
            StarClass::G => Color::rgba_u8(255, 244, 234, 100),
            StarClass::K => Color::rgba_u8(255, 210, 161, 150),
            StarClass::M => Color::rgba_u8(255, 204, 111, 150),
            StarClass::WhiteDwarf => Color::rgba_u8(155, 176, 255, 50),
        }
    }

    pub fn radius(&self) -> f32 {
        if self.mass > 1.0 {
            self.mass.powf(Self::MASS_TO_RADIUS_EXP_BIGGER_SUN)
        } else {
            self.mass.powf(Self::MASS_TO_RADIUS_EXP_SMALLER_SUN)
        }
    }

    pub fn surface_temperature(&self) -> f32 {
        self.mass.powf(Self::MASS_TO_SURFACE_TEMPERATURE_EXP)
    }

    pub fn lifetime(&self) -> u32 {
        (self.mass / self.luminosity()).round().abs() as u32
    }

    pub fn habitable_zone(&self) -> (f32, f32) {
        (
            (self.luminosity() / Self::HABITABLE_ZONE_OUTER_BOUND_RATIO).sqrt(),
            (self.luminosity() / Self::HABITABLE_ZONE_INNER_BOUND_RATIO).sqrt(),
        )
    }

    pub fn orbit_boundaries(&self) -> (f32, f32) {
        (
            self.mass * Self::ORBIT_INNER_BOUNDARY,
            self.mass * Self::ORBIT_OUTER_BOUNDARY,
        )
    }

    pub fn frostline(&self) -> f32 {
        Self::FROSTLINE_MULTIPLIER * self.luminosity().sqrt()
    }

    pub fn class_allows_life(&self) -> bool {
        [StarClass::K, StarClass::G, StarClass::F].contains(&self.class())
    }
}
