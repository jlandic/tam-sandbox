use bevy::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let rng: Pcg64 = Seeder::from("stripy zebra").make_rng();

        app.insert_resource(rng);
    }
}
