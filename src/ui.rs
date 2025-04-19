//! UI - this a module for GUI systems
use bevy::{color::palettes::css::GOLD, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
use crate::player::structures::{PlayerComponent, PlayerControllerData};

#[derive(Component)] /// label of fps text
pub struct FpsText;

#[derive(Component)] /// label of player's text
pub struct PlayerDataText;

/// setup gui interface
pub fn setup_gui(
    mut commands: Commands,
) {
    // add fps text
    commands.spawn((
        TextFont { font_size: 42.0, ..default() },
        TextColor(GOLD.into()),
        Text::new("FPS: "),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        },
    )).with_child((TextSpan::default(), FpsText));

    // add text of player data
    commands.spawn((
        TextFont { font_size: 30.0, ..default() },
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(5.0),
            ..Default::default()
        }
    )).with_child((TextSpan::default(), PlayerDataText));

    // add center point
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent.spawn(Node {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..Default::default()
                }
            ).insert(BackgroundColor(Color::srgb(235.0, 35.0, 12.0)));
        }
    );
}

/// update text for debug
pub fn update_gui_text(
    diagnostics: Res<DiagnosticsStore>,
    player_data_query: Query<(&PlayerComponent, &PlayerControllerData)>,
    mut fps_text_query: Query<&mut TextSpan, (With<FpsText>, Without<PlayerDataText>)>,
    mut player_text_query: Query<&mut TextSpan, (With<PlayerDataText>, Without<FpsText>)>
) {
    let player_data = player_data_query.single();

    for mut span in &mut fps_text_query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }

    for mut span in &mut player_text_query {
        **span = format!("{:?}\n{:?}", player_data.1, player_data.0);
    }
}

