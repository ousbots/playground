use crate::animation;

use bevy::prelude::*;

pub fn run_app() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    animation::add_systems(&mut app);

    app.run();
}
