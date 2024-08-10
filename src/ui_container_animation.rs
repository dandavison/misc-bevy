use bevy::{color::palettes::css::CRIMSON, prelude::*};

pub struct MyPlugin;

#[derive(Component)]
pub struct LineItem;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

fn startup(mut commands: Commands) {
    let items: Vec<_> = ["Hello", "World"]
        .into_iter()
        .map(|text| {
            commands
                .spawn(TextBundle::from_section(text, TextStyle { ..default() }))
                .insert(LineItem)
                .id()
        })
        .collect();
    commands
        .spawn(NodeBundle::default())
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .push_children(&items);
        });
}

fn update(
    mut _commands: Commands,
    mut text_bundle_query: Query<(&mut Style, &mut Transform), With<LineItem>>,
) {
    for (mut style, mut transform) in &mut text_bundle_query {
        //        style.position_type = PositionType::Absolute;
        transform.translation.y += 10.0;
    }
}
