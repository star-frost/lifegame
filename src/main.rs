use bevy::prelude::*;

const GRID_SIZE: usize = 35;
const CELL_SIZE: f32 = 20.0;
const BORDER_SIZE: f32 = 1.0; // 黑色网格线宽度

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".to_string(),
                resolution: (1200.0, 800.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, setup)
        .add_systems(Update, (keep_alive, spawn_grid_cells, cell_click_system))
        .insert_resource(CellStates::default())
        .run();
}

fn keep_alive() {}

#[derive(Component)]
struct GridArea;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Resource)]
struct CellStates {
    states: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl Default for CellStates {
    fn default() -> Self {
        Self {
            states: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf");

    // UI 根节点
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        })
        .with_children(|parent| {
            // 左侧规则说明
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.95, 0.95, 0.95).into(),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "游戏规则：\n1. 存活细胞周围有2或3个存活邻居则继续存活。\n2. 死亡细胞周围有3个存活邻居则复活。",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            // 中间网格区
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(GRID_SIZE as f32 * (CELL_SIZE + 2.0 * BORDER_SIZE)),
                        height: Val::Px(GRID_SIZE as f32 * (CELL_SIZE + 2.0 * BORDER_SIZE)),
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(GridArea);

            // 右侧示例图案
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.95, 0.95, 0.95).into(),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "示例图案：\n- 滑翔机\n- 轻型飞船\n- 振荡器",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}

// 每帧生成网格格子（只生成一次）
fn spawn_grid_cells(
    mut commands: Commands,
    query: Query<Entity, With<GridArea>>,
    existing: Query<&Cell>,
    mut cell_states: ResMut<CellStates>,
) {
    if !existing.is_empty() {
        return;
    }

    let grid_entity = query.single();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            commands.entity(grid_entity).with_children(|parent| {
                // 外层黑色边框
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(CELL_SIZE + 2.0 * BORDER_SIZE),
                            height: Val::Px(CELL_SIZE + 2.0 * BORDER_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // 内层白色格子，可点击
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(CELL_SIZE),
                                    height: Val::Px(CELL_SIZE),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            })
                            .insert(Cell { x, y });
                    });
            });

            cell_states.states[y][x] = false;
        }
    }
}

// 点击切换状态
fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Cell), Changed<Interaction>>,
    mut cell_states: ResMut<CellStates>,
) {
    for (interaction, mut color, cell) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let state = &mut cell_states.states[cell.y][cell.x];
            *state = !*state;
            *color = if *state {
                Color::BLACK.into()
            } else {
                Color::WHITE.into()
            };
        }
    }
}
