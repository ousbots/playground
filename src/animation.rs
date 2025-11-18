use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[derive(Component)]
struct WalkingSprite;

#[derive(Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_index: first,
            last_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// Add the animation systems to the app.
pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, help)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                trigger_animation::<WalkingSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<WalkingSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        );
}

// Loop through all the sprites and advance their animation, defined by the config..
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // Track how long the current sprite has been displayed.
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            // On last frame, reset to the first, otherwise advance.
            if atlas.index == config.last_index {
                atlas.index = config.first_index;
            } else {
                atlas.index += 1;
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

// Create the animation frame timer when animations are triggered.
fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

fn help(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Display help UI in the upper left.
    commands.spawn((
        Text::new("Left Arrow: animate left\nRight Arrow: animate right"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Load the sprite sheet.
    let texture = asset_server.load("man_walking_animation.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let layouts = texture_layouts.add(layout);

    let animation_config = AnimationConfig::new(0, 8, 10);

    // Create the sprite.
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layouts,
                index: animation_config.first_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        WalkingSprite,
        animation_config,
    ));
}
