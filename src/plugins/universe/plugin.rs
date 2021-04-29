use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::pcg::universe::Galaxy;
use crate::plugins::rng::Seed;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_galaxy.system());
    }
}

fn spawn_galaxy(mut commands: Commands, seed: Res<Seed>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let galaxy = Galaxy::generate(&seed, 10000, 2.0, 800.0, 8, 0.1, 8.0);

    for (position, star) in galaxy.stars {
        let shape = shapes::Circle {
            radius: star.radius,
            ..shapes::Circle::default()
        };

        commands
            .spawn()
            .insert_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(star.color, star.color),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(0.0),
                },
                Transform::from_xyz(position.x as f32, position.y as f32, 0.0),
            ))
            .insert(star);
    }
}
