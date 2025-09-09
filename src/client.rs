use bevy::{DefaultPlugins, app::App};

use crate::game::{
    events::Scored,
    resources::{Paused, Score},
    systems::{FixedUpdatePlugin, StartupPlugin, UpdatePlugin},
};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StartupPlugin)
        .add_plugins(FixedUpdatePlugin)
        .add_plugins(UpdatePlugin)
        .insert_resource(Score(0, 0))
        .insert_resource(Paused(true))
        .add_event::<Scored>()
        .run();
}
