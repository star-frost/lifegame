use crate::{CellStates, GRID_SIZE, set_cell_state};
use bevy::prelude::*;

/// 定义图案按钮的种类
#[derive(Component)]
pub enum PatternButton {
    Glider,
    Block,
    Pulsar,
    KaiYing,
    Shuttle,
    CShuttle,
}

/// 返回脉冲星图案坐标
fn pulsar_pattern() -> Vec<(usize, usize)> {
    vec![
        (18, 14),
        (18, 13),
        (18, 12),
        (19, 15),
        (20, 15),
        (21, 15),
        (23, 14),
        (23, 13),
        (23, 12),
        (21, 10),
        (20, 10),
        (19, 10),
        (19, 17),
        (20, 17),
        (21, 17),
        (18, 18),
        (18, 19),
        (18, 20),
        (23, 18),
        (23, 19),
        (23, 20),
        (19, 22),
        (20, 22),
        (21, 22),
        (16, 18),
        (16, 19),
        (16, 20),
        (15, 17),
        (14, 17),
        (13, 17),
        (13, 15),
        (14, 15),
        (15, 15),
        (16, 14),
        (16, 13),
        (16, 12),
        (15, 10),
        (14, 10),
        (13, 10),
        (11, 12),
        (11, 13),
        (11, 14),
        (11, 18),
        (11, 19),
        (11, 20),
        (13, 22),
        (14, 22),
        (15, 22),
    ]
}

/// 滑翔机
fn glider_pattern() -> Vec<(usize, usize)> {
    vec![(2, 2), (3, 3), (1, 4), (2, 4), (3, 4)]
}

/// 方块
fn block_pattern() -> Vec<(usize, usize)> {
    vec![(10, 10), (11, 10), (10, 11), (11, 11)]
}

/// 慨影
fn kaiying_pattern() -> Vec<(usize, usize)> {
    vec![
        (18, 21),
        (19, 20),
        (20, 19),
        (20, 18),
        (20, 16),
        (20, 17),
        (18, 12),
        (19, 13),
        (20, 14),
        (20, 15),
        (17, 13),
        (16, 14),
        (16, 15),
        (16, 16),
        (16, 17),
        (16, 18),
        (16, 19),
        (17, 20),
    ]
}

fn cshuffle_pattern() -> Vec<(usize, usize)> {
    vec![
        (18, 11),
        (18, 10),
        (19, 9),
        (19, 8),
        (19, 7),
        (18, 6),
        (18, 5),
        (20, 6),
        (21, 7),
        (22, 8),
        (21, 9),
        (20, 10),
        (23, 17),
        (24, 17),
        (25, 18),
        (26, 18),
        (27, 18),
        (28, 17),
        (29, 17),
        (28, 19),
        (27, 20),
        (26, 21),
        (25, 20),
        (24, 19),
        (17, 22),
        (17, 23),
        (16, 24),
        (16, 25),
        (16, 26),
        (17, 27),
        (17, 28),
        (15, 23),
        (14, 24),
        (13, 25),
        (14, 26),
        (15, 27),
        (12, 16),
        (11, 16),
        (10, 15),
        (9, 15),
        (8, 15),
        (7, 16),
        (6, 16),
        (7, 14),
        (8, 13),
        (9, 12),
        (10, 13),
        (11, 14),
    ]
}

fn shuttle_pattern() -> Vec<(usize, usize)> {
    vec![
        (6, 15),
        (5, 15),
        (5, 16),
        (6, 16),
        (25, 15),
        (26, 15),
        (26, 16),
        (25, 16),
        (10, 15),
        (11, 15),
        (11, 14),
        (11, 16),
        (12, 13),
        (13, 12),
        (14, 13),
        (14, 14),
        (14, 15),
        (14, 16),
        (14, 17),
        (13, 14),
        (13, 15),
        (13, 16),
        (12, 17),
        (13, 18),
    ]
}

/// 右侧图案按钮 UI 生成
pub fn patterns_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf");

    // “示例图案”标题
    parent.spawn(TextBundle::from_section(
        "示例图案：",
        TextStyle {
            font: font.clone(),
            font_size: 18.0,
            color: Color::BLACK,
        },
    ));

    // 按钮列表
    let patterns = vec![
        ("方块", PatternButton::Block),
        ("滑翔机", PatternButton::Glider),
        ("脉冲星", PatternButton::Pulsar),
        ("慨影", PatternButton::KaiYing),
        ("穿梭机", PatternButton::Shuttle),
        ("环状穿梭机", PatternButton::CShuttle),
    ];

    for (label, pattern_type) in patterns {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(160.0),
                        height: Val::Px(40.0),
                        margin: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.4, 0.4, 0.8).into(),
                    ..default()
                },
                pattern_type,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            });
    }
}

/// 点击按钮后加载对应图案
pub fn pattern_button_system(
    mut interaction_query: Query<(&Interaction, &PatternButton), Changed<Interaction>>,
    mut cell_states: ResMut<CellStates>,
    mut commands: Commands,
) {
    for (interaction, pattern) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let coords = match pattern {
                PatternButton::Pulsar => pulsar_pattern(),
                PatternButton::Glider => glider_pattern(),
                PatternButton::Block => block_pattern(),
                PatternButton::KaiYing => kaiying_pattern(),
                PatternButton::Shuttle => shuttle_pattern(),
                PatternButton::CShuttle => cshuffle_pattern(),
            };

            // 清空旧图案
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    set_cell_state(&mut commands, &mut cell_states, x, y, false);
                }
            }

            // 应用新图案
            for (x, y) in coords {
                if x < GRID_SIZE && y < GRID_SIZE {
                    set_cell_state(&mut commands, &mut cell_states, x, y, true);
                }
            }

            info!("已加载图案: {:?}", std::mem::discriminant(pattern));
        }
    }
}
