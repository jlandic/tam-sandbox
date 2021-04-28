use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod pcg;
mod plugins;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "TAM".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::rng::RngPlugin)
        .add_plugin(plugins::universe::UniversePlugin)
        .run();
}
