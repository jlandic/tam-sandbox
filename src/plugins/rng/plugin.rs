use super::resources::Seed;
use bevy::prelude::*;

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Seed(298374826504098726));
    }
}
