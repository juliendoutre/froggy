use crate::{despawn_screen, GameState};
use bevy::prelude::*;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu.
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("bevy-icon.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    bottom: Val::Px(50.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "Made with Bevy",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ));
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
