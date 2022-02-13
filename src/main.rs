use std::time::Duration;
use bevy::prelude::*;
use benimator::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default())
        .add_startup_system(window_setup)
        .add_startup_system(spawn)
        .run()
}

fn window_setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(640.0, 480.0);
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>, mut animations: ResMut<Assets<SpriteSheetAnimation>>) {
    // Don't forget the camera ;-)
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 0.2;
    commands.spawn_bundle(camera_bundle);
  
    let stand_anim_handle = animations.add(SpriteSheetAnimation::from_range(
      0..=1,
      Duration::from_millis(500),
    ));
  
    let adventurer_atlas = TextureAtlas::from_grid(
        asset_server.load("sprite\\adventurer.png"), 
        Vec2::new(16.0, 16.0), 
        4, 
        1);

    commands.spawn_bundle(SpriteSheetBundle {
              texture_atlas: textures.add(adventurer_atlas),
              transform: Transform::identity(),
              ..Default::default()
            })
            .insert(stand_anim_handle)
            .insert(Play);
  }