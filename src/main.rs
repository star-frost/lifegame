pub mod evolution;
use bevy::prelude::*;
use evolution::next_generation;

mod patterns;
use patterns::*;

pub const GRID_SIZE: usize = 35;
const CELL_SIZE: f32 = 20.0;
const BORDER_SIZE: f32 = 1.0; // 黑色网格线宽度
const STEP_INTERVAL: f32 = 0.2; // 每代间隔（秒）

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
        .insert_resource(CellStates::default())
        .insert_resource(Simulation { running: false })
        .insert_resource(EvolutionTimer(Timer::from_seconds(
            STEP_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_grid_cells,
                cell_click_system,
                button_system,
                evolution_runner_debug,
                pattern_button_system,
            ),
        )
        .run();
}

#[derive(Component)]
struct GridArea;

#[derive(Component)]
struct Cell {
    // 保持含义：x 列，y 行 （x = 列 index, y = 行 index）
    x: usize,
    y: usize,
}

#[derive(Component)]
enum ControlButton {
    Start,
    Clear,
}

#[derive(Component)]
struct StartButtonText;

#[derive(Resource)]
struct CellStates {
    // 逻辑状态：states[row][col] => states[y][x]
    states: [[bool; GRID_SIZE]; GRID_SIZE],
    // 映射到实际Button实体，便于直接更新 UI
    entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
}

impl Default for CellStates {
    fn default() -> Self {
        Self {
            states: [[false; GRID_SIZE]; GRID_SIZE],
            entities: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }
}

#[derive(Resource)]
struct Simulation {
    running: bool,
}

#[derive(Resource)]
struct EvolutionTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/ZCOOLKuaiLe-Regular.ttf");

    // 根节点（水平布局）
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
            // 左侧控制栏（竖直布局）
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(15.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.95, 0.95, 0.95).into(),
                    ..default()
                })
                .with_children(|p| {
                    // 上 1/3：规则说明
                    p.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(33.0),
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|q| {
                        q.spawn(TextBundle::from_section(
                            "游戏规则：\n1. 存活细胞周围有 2 或 3 个邻居则继续存活。\n2. 死亡细胞周围有 3 个邻居则复活。",
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ));
                    });

                    // 下 2/3：按钮区（垂直排列）
                    p.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(67.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(12.0),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|q| {
                        // Start / Stop 按钮
                        // Start / Stop 按钮
q.spawn(ButtonBundle {
    style: Style {
        width: Val::Px(140.0),
        height: Val::Px(48.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    background_color: Color::srgb(0.2, 0.6, 0.9).into(),
    ..default()
})
.insert(ControlButton::Start)
.with_children(|r| {
    r.spawn(TextBundle::from_section(
        "开始演化",
        TextStyle {
            font: font.clone(),
            font_size: 18.0,
            color: Color::WHITE,
        },
    ))
    .insert(StartButtonText); 
});


                        // Clear 按钮
                        q.spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(140.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.85, 0.2, 0.2).into(),
                            ..default()
                        })
                        .insert(ControlButton::Clear)
                        .with_children(|r| {
                            r.spawn(TextBundle::from_section(
                                "清空网格",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    });
                });

            // 中间网格区
parent
    .spawn(NodeBundle {
        style: Style {
            width: Val::Px(GRID_SIZE as f32 * (CELL_SIZE + 2.0 * BORDER_SIZE)),
            height: Val::Px(GRID_SIZE as f32 * (CELL_SIZE + 2.0 * BORDER_SIZE)),
            // 关键：使子节点 absolute 定位基于此容器
            position_type: PositionType::Relative,
            // 不要使用 flex_wrap 了
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    })
    .insert(GridArea);


parent
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::srgb(0.95, 0.95, 0.95).into(),
        ..default()
    })
    .with_children(|p| {
        patterns_ui(p, &asset_server);
    });
        });
}

// 统一设置某个格子状态并更新对应实体颜色
fn set_cell_state(
    commands: &mut Commands,
    cell_states: &mut CellStates,
    x: usize,
    y: usize,
    alive: bool,
) {
    // 更新逻辑数组（states[y][x]）
    cell_states.states[y][x] = alive;
    // 通过实体映射更新 UI（若存在）
    if let Some(ent) = cell_states.entities[y][x] {
        commands.entity(ent).insert(BackgroundColor(if alive {
            Color::BLACK
        } else {
            Color::WHITE
        }));
    }
}

// 生成格子
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

    // 每个格子占用的像素（含黑色边框）
    let cell_total = CELL_SIZE + 2.0 * BORDER_SIZE;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            // 计算左上相对于容器的像素偏移（UI 的 top 从容器上边向下算）
            let left = x as f32 * cell_total;
            let top = y as f32 * cell_total;

            
            commands.entity(grid_entity).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            
                            position_type: PositionType::Absolute,
                            left: Val::Px(left),
                            top: Val::Px(top),
                            width: Val::Px(cell_total),
                            height: Val::Px(cell_total),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.8, 0.8).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // 内层白色 Button
                        let btn = parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(CELL_SIZE),
                                    height: Val::Px(CELL_SIZE),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            })
                            .insert(Cell { x, y })
                            .id();

                        // 记录实体映射
                        cell_states.entities[y][x] = Some(btn);
                    });
            });

            // 初始化逻辑状态
            cell_states.states[y][x] = false;
        }
    }
}


// 点击单个格子
// 点击时统一通过 set_cell_state 更新状态与 UI
fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &Cell), Changed<Interaction>>,
    mut cell_states: ResMut<CellStates>,
    mut commands: Commands,
) {
    for (interaction, cell) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // 打印调试坐标（行 y，列 x），便于核对视觉与数组映射
            info!("Clicked cell at array coords: (x={}, y={})", cell.x, cell.y);

            // 先读取当前状态
            let current = cell_states.states[cell.y][cell.x];
            let new_state = !current;

            // 再以可变借用的方式更新状态和 UI
            set_cell_state(&mut commands, &mut *cell_states, cell.x, cell.y, new_state);
        }
    }
}

// Start 切换运行态，Clear 清空并把所有方块设为白色
fn button_system(
    mut interaction_query: Query<(&Interaction, &ControlButton), Changed<Interaction>>,
    mut start_text_query: Query<&mut Text, With<StartButtonText>>,
    mut cell_states: ResMut<CellStates>,
    mut sim: ResMut<Simulation>,
    mut commands: Commands,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                ControlButton::Start => {
                    sim.running = !sim.running;
                    info!("演化状态: {}", if sim.running { "运行" } else { "停止" });

                    if let Ok(mut text) = start_text_query.get_single_mut() {
                        text.sections[0].value = if sim.running {
                            "停止演化".to_string()
                        } else {
                            "开始演化".to_string()
                        };
                    }
                }
                ControlButton::Clear => {
                    info!("清空网格");
                    for y in 0..GRID_SIZE {
                        for x in 0..GRID_SIZE {
                            set_cell_state(&mut commands, &mut *cell_states, x, y, false);
                        }
                    }
                }
            }
        }
    }
}

fn evolution_runner_debug(
    time: Res<Time>,
    mut timer: ResMut<EvolutionTimer>,
    mut sim: ResMut<Simulation>,
    mut cell_states: ResMut<CellStates>,
    mut commands: Commands,
) {
    if !sim.running {
        return;
    }

    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }


    let current = cell_states.states;

    // 计算下一代（来自 evolution.rs）
    let next = next_generation(&current);

    // 对比 current 与 next 全表的活细胞数量（快速 sanity check）
    let cur_alive = current.iter().flatten().filter(|b| **b).count();
    let next_alive = next.iter().flatten().filter(|b| **b).count();
    println!(
        "DEBUG: alive count current = {}, next = {}",
        cur_alive, next_alive
    );

    // 应用 next 到资源 & UI（使用统一 helper）
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            set_cell_state(&mut commands, &mut *cell_states, x, y, next[y][x]);
        }
    }

    println!("DEBUG: applied next generation to UI");
}
