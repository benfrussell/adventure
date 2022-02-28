use std::time::Duration;
use actions::{ActionList, do_actions};
use bevy::prelude::*;
use bevy::core::FixedTimestep;
use benimator::*;

mod actions;

#[derive(Default)]
struct Handles {
    adv_walk_anim: Handle<SpriteSheetAnimation>,
    adv_stand_anim: Handle<SpriteSheetAnimation>,
    adv_texture_atlas: Handle<TextureAtlas>
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default())
        .insert_resource(Handles { ..Default::default() })
        .add_startup_system(window_setup)
        .add_startup_system(camera_setup)
        .add_startup_system(load_handles)
        .add_startup_system(spawn_adventurer)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                .with_system(do_actions)
        )
        .run()
}

fn window_setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(640.0, 480.0);
}

fn camera_setup(mut commands: Commands) {
    // Don't forget the camera ;-)
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 0.2;
    commands.spawn_bundle(camera_bundle);
}

fn load_handles(
    mut handles: ResMut<Handles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>, 
    mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
        handles.adv_stand_anim = animations.add(SpriteSheetAnimation::from_range(
            0..=1,
            Duration::from_millis(600),
        ));

        handles.adv_walk_anim = animations.add(SpriteSheetAnimation::from_range(
            2..=3,
            Duration::from_millis(200),
        ));
    
        handles.adv_texture_atlas = textures.add(TextureAtlas::from_grid(
            asset_server.load("sprite\\adventurer.png"),
            Vec2::new(16.0, 16.0), 
            4, 
            1));
}

fn spawn_adventurer(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: handles.adv_texture_atlas.clone_weak(),
            transform: Transform::identity(),
            ..Default::default()
            })
            .insert(handles.adv_stand_anim.clone_weak())
            .insert(Play)
            .insert(ActionList::new());
}