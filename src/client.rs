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
        .insert_resource(Score::default())
        .insert_resource(Paused::new(true))
        .add_event::<Scored>()
        .run();
}
