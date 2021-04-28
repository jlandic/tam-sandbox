use bevy::prelude::*;
use rand_pcg::Pcg64;

use crate::pcg::universe::Galaxy;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_galaxy.system());
    }
}

fn spawn_galaxy(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<Pcg64>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let galaxy = Galaxy::generate(&mut rng, 10000, 4.0, 800.0, 8, 0.1, 8.0);

    for (position, star) in galaxy.stars {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(star.color.into()),
                sprite: Sprite {
                    size: Vec2::new((star.radius / 2.0).max(1.0), (star.radius / 2.0).max(1.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Transform::from_xyz(
                position.x as f32,
                position.y as f32,
                0.0,
            ))
            .insert(star);
    }
}
