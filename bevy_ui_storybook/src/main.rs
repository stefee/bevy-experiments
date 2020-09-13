use bevy::prelude::*;

const DEBUG_MATERIALS: bool = false;

fn main() {
    App::build()
        .add_resource(bevy::window::WindowDescriptor {
            width: 1920,
            height: 1080,
            title: "Bevy UI Storybook Prototype".to_string(),
            ..Default::default()
        })
        .add_resource(bevy::render::pass::ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            material: debug_color(&mut materials, Color::rgb(1.0, 0.0, 1.0).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                //
                // sidebar
                //
                .spawn(NodeComponents {
                    style: Style {
                        flex_basis: Val::Percent(20.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: debug_color(&mut materials, Color::rgb(1.0, 0.0, 0.0).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(0.0),
                                padding: Rect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.2, 0.2, 1.0).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "comp 1".to_string(),
                                    font: font_handle,
                                    style: TextStyle {
                                        font_size: 50.0,
                                        color: Color::BLACK,
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .spawn(border_horizontal_node(&mut materials));
                })
                //
                // vertical border
                //
                .spawn(border_vertical_node(&mut materials))
                //
                // main
                //
                .spawn(NodeComponents {
                    style: Style {
                        flex_basis: Val::Percent(80.0),
                        ..Default::default()
                    },
                    material: debug_color(&mut materials, Color::rgb(0.0, 1.0, 0.0).into()),
                    ..Default::default()
                });
        });
}

fn debug_color(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: ColorMaterial,
) -> Handle<ColorMaterial> {
    if DEBUG_MATERIALS {
        materials.add(color)
    } else {
        Default::default()
    }
}

fn border_vertical_node(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Px(4.0), Val::Percent(100.0)),
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        ..Default::default()
    }
}

fn border_horizontal_node(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(4.0)),
            ..Default::default()
        },
        material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        ..Default::default()
    }
}
