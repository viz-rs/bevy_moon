use bevy::{
    camera_controller::pan_camera::{PanCamera, PanCameraPlugin},
    color::palettes::css::{BLACK, BLUE, GRAY, GREEN, PINK, RED, WHITE},
    prelude::*,
};

use bevy_moon::prelude::{BoxShadow, Corners, MoonPlugin, div, image};
use taffy::{LengthPercentage, Rect};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, PanCamera::default()));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(100.0, 100.0)))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::oklcha(0.81, 0.1, 251., 0.99)))),
        Transform::from_xyz(0.0, -50.0, 0.0),
    ));

    // commands.spawn((
    //     div().w(200.0).h_auto().flex_col(),
    //     children![
    //         div().w_full().h(50.0),
    //         (
    //             div().w_full().h(50.0).flex_col(),
    //             children![
    //                 div().flex_auto(),
    //                 div().flex_auto(),
    //                 (div().flex_auto(), children![div().flex_auto()])
    //             ]
    //         ),
    //         div().w_full().h(100.0),
    //         (
    //             div().flex_auto().h(50.0),
    //             children![
    //                 div().flex_auto(),
    //                 div().flex_auto(),
    //                 (div().flex_auto(), children![div().flex_auto()])
    //             ]
    //         ),
    //         div().w_full().h(50.0),
    //     ],
    // ));

    // commands.spawn((
    //     div().w(100.0).h(100.0),
    //     children![(
    //         div().w_p(0.5),
    //         children![
    //             div().w_full(),
    //             (div().w_full(), children![div().flex_auto()])
    //         ]
    //     ),],
    // ));

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
            .w(128.0)
            .h(128.0)
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
        image(asset_server.load("images/bevy.png")),
        Transform::from_xyz(-150.0, -150.0, 0.0),
    ));

    commands.spawn((
        div()
            .w(128.0)
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
            .corner_radii(Corners::all(12.0))
            .shadow_md(),
        children![(
            div().flex().flex_auto(),
            image(asset_server.load("images/bevy.png")),
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
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PanCameraPlugin)
        .add_plugins(MoonPlugin)
        .add_systems(Startup, setup)
        // .insert_resource(ClearColor(GRAY.into()));
        .insert_resource(ClearColor(Color::oklch(0.98, 0.0, 0.0)));

    app.run();
}
