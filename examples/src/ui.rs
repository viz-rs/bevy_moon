use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::{
    camera_controller::pan_camera::{PanCamera, PanCameraPlugin},
    color::palettes::css::{ANTIQUE_WHITE, BLACK, BLUE, DEEP_SKY_BLUE, GRAY, GREEN, RED, WHITE},
    prelude::*,
};

use lucide_icons::Icon;

use bevy_moon::prelude::{Corners, MoonPlugin, ObjectPosition, div, img, text};
use taffy::{LengthPercentage, Rect};

const LOOP_LENGTH: f32 = 4.0;

#[derive(Resource, Default)]
struct AnimationState {
    playing: bool,
    paused_at: f32,
    paused_total: f32,
    t: f32,
}

trait UpdateTransform {
    fn update(&self, t: f32, transform: &mut Transform);
}

/// Moves a component around the origin
#[derive(Component)]
struct Move((f32, f32));

impl UpdateTransform for Move {
    fn update(&self, t: f32, transform: &mut Transform) {
        transform.translation.x = self.0.0 + ops::sin(t * TAU - FRAC_PI_2) * 50.0;
        transform.translation.y = self.0.1 + ops::cos(t * TAU - FRAC_PI_2) * 50.0;
    }
}

#[derive(Component)]
struct Scale;

impl UpdateTransform for Scale {
    fn update(&self, t: f32, transform: &mut Transform) {
        transform.scale.x = 1.0 + 0.5 * ops::cos(t * TAU).max(0.0);
        transform.scale.y = 1.0 + 1.5 * ops::cos(t * TAU + PI).max(0.0);
    }
}

#[derive(Component)]
struct Rotate;

impl UpdateTransform for Rotate {
    fn update(&self, t: f32, transform: &mut Transform) {
        let q = Quat::from_rotation_z(ops::cos(t * TAU) * 45.0);
        transform.rotation = q;
    }
}

fn update_animation(
    mut animation: ResMut<AnimationState>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let delta = time.elapsed_secs();

    if keys.just_pressed(KeyCode::Space) {
        animation.playing = !animation.playing;

        if !animation.playing {
            animation.paused_at = delta;
        } else {
            animation.paused_total += delta - animation.paused_at;
        }
    }

    if animation.playing {
        animation.t = (delta - animation.paused_total) % LOOP_LENGTH / LOOP_LENGTH;
    }
}

fn update_transform<T: UpdateTransform + Component>(
    animation: Res<AnimationState>,
    mut containers: Query<(&mut Transform, &T)>,
) {
    if !animation.playing {
        return;
    }
    for (mut transform, update_transform) in &mut containers {
        update_transform.update(animation.t, &mut transform);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let lucide_font = Font::try_from_bytes(lucide_icons::LUCIDE_FONT_BYTES.to_vec(), "Lucide");
    let icon_font = asset_server.add(lucide_font);

    let font = asset_server.load::<Font>("fonts/FiraMono-Medium.ttf");

    commands.spawn((Camera2d, PanCamera::default()));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(100.0, 100.0)))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::oklcha(0.81, 0.1, 251., 0.99)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        div()
            .w(100.0)
            .h(100.0)
            .background(GREEN)
            .corner_radii(Corners {
                top_left: 5.0,
                top_right: 15.0,
                bottom_right: 25.0,
                bottom_left: 35.0,
            })
            .border(Rect {
                top: LengthPercentage::length(5.0),
                right: LengthPercentage::length(10.0),
                bottom: LengthPercentage::length(15.0),
                left: LengthPercentage::length(20.0),
            })
            .border_color(RED)
            .shadow_sm(),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    commands.spawn((
        div()
            .w(75.0)
            .h(75.0)
            .background(GREEN)
            .border(Rect {
                top: LengthPercentage::length(1.0),
                right: LengthPercentage::length(1.0),
                bottom: LengthPercentage::length(1.0),
                left: LengthPercentage::length(1.0),
            })
            .border_color(BLACK)
            .corner_radii(Corners::all(10.0))
            .shadow_sm(),
        Transform::from_xyz(187.5, 0.0, 0.0),
        children![(
            div().w(50.0).h(50.0).background(BLUE),
            Transform::from_xyz(10.0, -10.0, 0.0),
        )],
    ));

    commands.spawn((
        div()
            .w(64.0)
            .h(64.0)
            .border(Rect {
                top: LengthPercentage::length(2.0),
                right: LengthPercentage::length(2.0),
                bottom: LengthPercentage::length(2.0),
                left: LengthPercentage::length(2.0),
            })
            .border_color(RED)
            .background(WHITE)
            .corner_radii(Corners::all(12.0))
            .shadow_md(),
        img(asset_server.load("images/bevy.png")),
        Transform::from_xyz(-200.0, -64.0, 0.0),
    ));
    commands.spawn((
        div()
            .w(64.0)
            .h(64.0)
            .border(Rect {
                top: LengthPercentage::length(2.0),
                right: LengthPercentage::length(2.0),
                bottom: LengthPercentage::length(2.0),
                left: LengthPercentage::length(2.0),
            })
            .border_color(RED)
            .background(WHITE)
            .corner_radii(Corners::all(12.0))
            .shadow_md(),
        img(asset_server.load("images/bevy.png")).flip_x(),
        Transform::from_xyz(-200.0, -134.0, 0.0),
    ));
    commands.spawn((
        div()
            .w(64.0)
            .h(64.0)
            .border(Rect {
                top: LengthPercentage::length(2.0),
                right: LengthPercentage::length(2.0),
                bottom: LengthPercentage::length(2.0),
                left: LengthPercentage::length(2.0),
            })
            .border_color(RED)
            .background(WHITE)
            .corner_radii(Corners::all(12.0))
            .shadow_md(),
        img(asset_server.load("images/bevy.png")).flip_y(),
        Transform::from_xyz(-200.0, -204.0, 0.0),
    ));
    commands.spawn((
        div()
            .w(64.0)
            .h(64.0)
            .border(Rect {
                top: LengthPercentage::length(2.0),
                right: LengthPercentage::length(2.0),
                bottom: LengthPercentage::length(2.0),
                left: LengthPercentage::length(2.0),
            })
            .border_color(RED)
            .background(WHITE)
            .corner_radii(Corners::all(12.0))
            .shadow_md(),
        img(asset_server.load("images/bevy.png")).flip_x().flip_y(),
        Transform::from_xyz(-200.0, -274.0, 0.0),
        Scale,
        Move((-200.0, -274.0)),
        Rotate,
    ));

    commands.spawn((
        div()
            .w(264.0)
            .h(128.0)
            .flex()
            .border(Rect {
                top: LengthPercentage::length(2.0),
                right: LengthPercentage::length(2.0),
                bottom: LengthPercentage::length(2.0),
                left: LengthPercentage::length(2.0),
            })
            .border_color(RED)
            .background(WHITE)
            .corner_radii(Corners::all(48.0))
            .shadow_md(),
        children![(
            div().flex().flex_auto().corner_radii(Corners::all(48.0)),
            img(asset_server.load("images/bevy_logo_light.png"))
                .object_fit_scale_down()
                .object_position(ObjectPosition::BOTTOM_RIGHT)
                .flip_x(),
        )],
        Transform::from_xyz(150.0, 150.0, 0.0),
    ));

    commands.spawn((
        div()
            .w(96.0)
            .h(96.0)
            .background(WHITE)
            .corner_radii(Corners::all(25.0).top_left(0.0).bottom_right(0.0))
            .shadow_sm(),
        Transform::from_xyz(100.0, -100.0, 0.0),
    ));

    commands.spawn((
        div().w(216.0).h(29.0).background(GRAY),
        text("Hello Bevy!"),
        TextColor::WHITE,
        TextFont::default()
            .with_font(font.clone())
            .with_font_size(24.0),
        Transform::from_xyz(-100.0, 150.0, 0.0),
    ));

    commands.spawn((
        div().w(216.0).h(29.0).background(ANTIQUE_WHITE),
        text("Hello Moon!"),
        TextColor::BLACK,
        TextFont::default()
            .with_font(font.clone())
            .with_font_size(24.0),
        Transform::from_xyz(-100.0, 150.0 - 29.0, 0.0),
        Scale,
        Move((-100.0, 150.0 - 29.0)),
        Rotate,
    ));

    commands.spawn((
        div().w(250.0).flex().p_px().background(WHITE).shadow_lg(),
        children![
          (
            div().w_full(),
            text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
            TextColor::BLACK,
            TextFont::default().with_font(font).with_font_size(24.0),
          ),
        ],
        Transform::from_xyz(-450.0, 200.0, 0.0),
    ));

    commands.spawn((
        Text2d::new(Icon::Bird.to_string()),
        TextColor::BLACK,
        TextFont::default()
            .with_font(icon_font.clone())
            .with_font_size(24.0),
        Transform::from_xyz(0.0, -50.0, 0.0).with_scale(Vec3::splat(2.0)),
    ));

    commands.spawn((
        div()
            .w(50.0)
            .h(50.0)
            .flex_auto()
            .items_center()
            .justify_center()
            .p_px()
            .background(WHITE)
            .shadow_lg(),
        children![(
            text(Icon::Bird.to_string()),
            TextColor::BLACK,
            TextFont::default()
                .with_font(icon_font.clone())
                .with_font_size(24.0),
        )],
        Transform::from_xyz(0.0, 0.0, 0.0),
        Scale,
        Move((0.0, 0.0)),
        Rotate,
    ));

    commands.spawn((
        div()
            .flex_auto()
            .items_center()
            .justify_center()
            .background(WHITE)
            .shadow_sm(),
        children![(
            text("Tip: "),
            TextFont::default(),
            TextColor::BLACK,
            children![
                (TextSpan::new("Enter "), TextColor::BLACK),
                (
                    TextSpan::new(Icon::Space.to_string()),
                    TextFont::default().with_font(icon_font),
                    TextColor(DEEP_SKY_BLUE.into())
                ),
                (TextSpan::new(" to enable animation"), TextColor::BLACK),
            ],
        )],
        Transform::from_xyz(300.0, 300.0, 0.0),
        Scale,
    ));
}

fn main() {
    let mut app = App::new();

    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins.build();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        default_plugins = default_plugins
            .disable::<bevy::winit::WinitPlugin>()
            .set(WindowPlugin::default());
    }

    app.add_plugins(default_plugins)
        .add_plugins(PanCameraPlugin)
        .add_plugins(MoonPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_transform::<Move>,
                update_transform::<Scale>,
                update_transform::<Rotate>,
                update_animation,
            ),
        )
        .init_resource::<AnimationState>()
        // .insert_resource(ClearColor(GRAY.into()));
        .insert_resource(ClearColor(Color::oklch(0.98, 0.0, 0.0)));

    #[cfg(not(any(target_family = "wasm", target_os = "android", target_os = "ios")))]
    app.insert_resource(bevy::winit::WinitSettings {
        focused_mode: bevy::winit::UpdateMode::Continuous,
        unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(
            std::time::Duration::from_secs(60),
        ),
    });

    app.run();
}
