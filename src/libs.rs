use bevy::prelude::*;
use crate::ball_game::BallGamePlugin;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,setup)
            .add_plugins(BallGamePlugin)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}