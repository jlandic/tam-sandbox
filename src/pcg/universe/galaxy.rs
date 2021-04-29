use bevy::prelude::*;
use lazy_static::lazy_static;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::BTreeMap;
use std::f32::consts::PI;

use crate::pcg::universe::{Star, StarClass};
use crate::plugins::rng::Seed;
use crate::plugins::std::Position;

const MAX_RANDOM_POSITION_ITERATIONS: u8 = 5;

const CLASS_O_AMOUNT_RATIO: f32 = 0.001;
const CLASS_B_AMOUNT_RATIO: f32 = 0.001;
const CLASS_A_AMOUNT_RATIO: f32 = 0.005;
const CLASS_F_AMOUNT_RATIO: f32 = 0.03;
const CLASS_G_AMOUNT_RATIO: f32 = 0.07;
const CLASS_K_AMOUNT_RATIO: f32 = 0.1;
const CLASS_M_AMOUNT_RATIO: f32 = 0.7;

lazy_static! {
    static ref STAR_CLASS_DISTRIBUTION: BTreeMap<StarClass, f32> = {
        let mut m = BTreeMap::new();
        m.insert(StarClass::O, CLASS_O_AMOUNT_RATIO);
        m.insert(StarClass::B, CLASS_B_AMOUNT_RATIO);
        m.insert(StarClass::A, CLASS_A_AMOUNT_RATIO);
        m.insert(StarClass::F, CLASS_F_AMOUNT_RATIO);
        m.insert(StarClass::G, CLASS_G_AMOUNT_RATIO);
        m.insert(StarClass::K, CLASS_K_AMOUNT_RATIO);
        m.insert(StarClass::M, CLASS_M_AMOUNT_RATIO);
        m
    };
}

pub struct Galaxy {
    pub stars: BTreeMap<Position, Star>,
}

impl Galaxy {
    pub fn generate(
        seed: &Seed,
        stars_count: i32,
        gravity: f32,
        radius: f32,
        arms_count: u8,
        arm_spread: f32,
        rotation_strength: f32,
    ) -> Self {
        let mut stars: BTreeMap<Position, Star> = BTreeMap::new();
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed.0);

        for (class, ratio) in STAR_CLASS_DISTRIBUTION.iter() {
            for _ in 0..(stars_count as f32 * *ratio).round() as i32 {
                let position = Galaxy::random_star_position(
                    &mut rng,
                    gravity,
                    radius,
                    arms_count,
                    arm_spread,
                    rotation_strength,
                );

                let star = Star::from_class(&mut rng, *class);

                stars.insert(position, star);
            }
        }

        Self { stars }
    }

    fn random_star_position(
        rng: &mut StdRng,
        gravity: f32,
        radius: f32,
        arms_count: u8,
        arm_spread: f32,
        rotation_strength: f32,
    ) -> Position {
        let mut v;
        let arm_divisor = PI / arms_count as f32;
        let mut iterations = 1;

        loop {
            let mut valid = false;
            // Random distance from galaxy center
            let distance: f32 = rng.gen_range(0.06..1.0);

            // Get a random direction, then take the position at `distance` from the galaxy center, in this direction,
            // taking the gravity setting into account.
            v = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                * distance.powf(gravity);

            let d = v.x.atan2(v.y);

            for i in 1..=arms_count {
                // Map arm "i" to a circle radius from -PI to PI
                let j = (i - 1) as f32 * (PI + PI) / (arms_count - 1) as f32 - PI;

                // Validate the position if it's somewhere close to the arm direction,
                // more or less depending on the arm spread setting.
                if d > j && d < j + arm_divisor * arm_spread {
                    valid = true;
                    break;
                }
            }

            if valid || iterations >= MAX_RANDOM_POSITION_ITERATIONS {
                break;
            }
            iterations += 1;
        }

        v = Mat3::from_rotation_z(-v.length() * rotation_strength).transform_vector2(v);
        Position {
            x: (v.x * radius).round() as i32,
            y: (v.y * radius).round() as i32,
        }
    }
}
