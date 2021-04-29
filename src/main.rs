use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_prototype_lyon::prelude::*;

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
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(plugins::rng::RngPlugin)
        .add_plugin(plugins::universe::UniversePlugin)
        .add_system(exit_on_esc.system())
        .run();
}

fn exit_on_esc(input: Res<Input<KeyCode>>, _query: Query<()>) {
    if input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
