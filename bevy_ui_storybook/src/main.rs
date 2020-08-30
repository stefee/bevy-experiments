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

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.2, 0.2, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.3, 0.3, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.4, 0.4, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.5, 0.5, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.6, 0.6, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.7, 0.7, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.8, 0.8, 1.0).into()),
                            ..Default::default()
                        })
                        .spawn(border_horizontal_node(&mut materials))
                        .spawn(NodeComponents {
                            style: Style {
                                flex_basis: Val::Px(100.0),
                                ..Default::default()
                            },
                            material: debug_color(&mut materials, Color::rgb(0.9, 0.9, 1.0).into()),
                            ..Default::default()
                        });
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

fn debug_color(materials: &mut ResMut<Assets<ColorMaterial>>, color: ColorMaterial) -> Handle<ColorMaterial> {
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
