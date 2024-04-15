use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    text::BreakLineOn,
    window::{PresentMode, PrimaryWindow},
};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

mod levels;

pub use levels::*;

pub const GAME_WIDTH: f32 = 640.0;
pub const GAME_HEIGHT: f32 = 480.0;
pub const BORDER: i16 = 5;

pub const UNITS: [Unit; 7] = [
    Unit {
        id: 0,
        kind: UnitType::Assault,
        parasite: false,
        max_health: 3,
        health: 3,
        damage: 2,
        speed: 3,
        range: 4,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Cardinal,
            charge: false,
            aoe: false,
            all_directions: false,
        },
        dna: 2,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Scout,
        parasite: false,
        max_health: 2,
        health: 2,
        damage: 1,
        speed: 5,
        range: 3,
        move_direction: Direction::Diagonal,
        attack_pattern: AttackPattern {
            direction: Direction::Cardinal,
            charge: false,
            aoe: false,
            all_directions: false,
        },
        dna: 1,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Sniper,
        parasite: false,
        max_health: 3,
        health: 3,
        damage: 4,
        speed: 4,
        range: 5,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Diagonal,
            charge: false,
            aoe: false,
            all_directions: false,
        },
        dna: 2,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Ballistic,
        parasite: false,
        max_health: 3,
        health: 3,
        damage: 2,
        speed: 3,
        range: 2,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Diagonal,
            charge: false,
            aoe: true,
            all_directions: true,
        },
        dna: 3,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Juggernaut,
        parasite: false,
        max_health: 4,
        health: 4,
        damage: 3,
        speed: 3,
        range: 4,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Cardinal,
            charge: true,
            aoe: false,
            all_directions: false,
        },
        dna: 3,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Heavy,
        parasite: false,
        max_health: 5,
        health: 5,
        damage: 2,
        speed: 2,
        range: 3,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Cardinal,
            charge: false,
            aoe: true,
            all_directions: false,
        },
        dna: 4,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
    Unit {
        id: 0,
        kind: UnitType::Commander,
        parasite: false,
        max_health: 4,
        health: 4,
        damage: 3,
        speed: 3,
        range: 4,
        move_direction: Direction::Cardinal,
        attack_pattern: AttackPattern {
            direction: Direction::Any,
            charge: false,
            aoe: false,
            all_directions: false,
        },
        dna: 5,
        has_moved: false,
        has_attacked: false,
        attack_directions: None,
    },
];

#[derive(Event)]
pub struct ChangeLevel {
    pub level_id: usize,
}

#[derive(Debug, Clone, Resource)]
pub struct Sprites {
    attack_directions: (Handle<Image>, Handle<TextureAtlasLayout>),
    obstacles: (Handle<Image>, Handle<TextureAtlasLayout>),
    selections: (Handle<Image>, Handle<TextureAtlasLayout>),
    tiles: (Handle<Image>, Handle<TextureAtlasLayout>),
    ui_background: (Handle<Image>, Handle<TextureAtlasLayout>),
    units: (
        Handle<Image>,
        Handle<TextureAtlasLayout>,
        Vec<AnimationTimer>,
    ),
}

#[derive(Debug, Clone)]
pub enum Animation {
    UnitMove {
        id: usize,
        start: Position,
        goal: Position,
        progress: f32,
    },
    UnitAttack {
        id: usize,
    },
    UnitDeath {
        id: usize,
    },
}

#[derive(Debug, Clone, Resource)]
pub struct AnimationQueue {
    queue: Vec<Animation>,
    started: bool,
    finished: bool,
}

#[derive(Debug, Clone, Resource)]
pub struct CurrentLevel(Level);

#[derive(Debug, Clone, Resource)]
pub struct Selected(Option<usize>);

#[derive(Debug, Clone, PartialEq, Resource)]
pub enum Turn {
    HumansMove,
    Parasites,
    HumansAttack,
}

impl Turn {
    pub fn next(&mut self) {
        *self = match self {
            Self::HumansMove => Self::Parasites,
            Self::Parasites => Self::HumansAttack,
            Self::HumansAttack => Self::HumansMove,
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct TurnOrder(Vec<usize>);

#[derive(Debug, Clone, Resource)]
pub struct Dna(u16);

#[derive(Debug, Clone, Resource)]
pub struct Cost(Option<u16>);

#[derive(Debug, Clone, Component)]
pub struct AnimationTimer {
    timer: Timer,
    first: usize,
    last: usize,
}

#[derive(Debug, Clone, Component)]
pub struct Tile;

#[derive(Debug, Clone)]
pub enum UnitType {
    Assault,
    Scout,
    Sniper,
    Ballistic,
    Juggernaut,
    Heavy,
    Commander,
}

impl UnitType {
    pub fn index(&self) -> usize {
        match self {
            Self::Assault => 0,
            Self::Scout => 1,
            Self::Sniper => 2,
            Self::Ballistic => 3,
            Self::Juggernaut => 4,
            Self::Heavy => 5,
            Self::Commander => 6,
        }
    }

    pub fn order(&self) -> u16 {
        match self {
            Self::Commander => 0,
            Self::Scout => 1,
            Self::Sniper => 2,
            Self::Assault => 3,
            Self::Ballistic => 4,
            Self::Juggernaut => 5,
            Self::Heavy => 6,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Assault => "Assault",
            Self::Scout => "Scout",
            Self::Sniper => "Sniper",
            Self::Ballistic => "Ballistic",
            Self::Juggernaut => "Juggernaut",
            Self::Heavy => "Heavy",
            Self::Commander => "Commander",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Cardinal,
    Diagonal,
    Any,
}

impl Direction {
    pub fn vectors(&self) -> Vec<(isize, isize)> {
        match self {
            Self::Cardinal => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Self::Diagonal => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
            Self::Any => vec![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ],
        }
    }
    pub fn text(&self) -> &str {
        match self {
            Self::Cardinal => "cardinally",
            Self::Diagonal => "diagonally",
            Self::Any => "any direction",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttackPattern {
    direction: Direction,
    charge: bool,
    aoe: bool,
    all_directions: bool,
}

#[derive(Debug, Clone, Component)]
pub struct Unit {
    id: usize,
    kind: UnitType,
    parasite: bool,
    max_health: u16,
    health: u16,
    damage: u16,
    speed: u16,
    range: u16,
    move_direction: Direction,
    attack_pattern: AttackPattern,
    dna: u16,
    has_moved: bool,
    has_attacked: bool,
    attack_directions: Option<Vec<(isize, isize)>>,
}

impl Unit {
    pub fn animation_index(&self) -> usize {
        let offset = if self.parasite { 4 } else { 0 };
        match self.kind {
            UnitType::Assault => offset,
            UnitType::Scout => 8 + offset,
            UnitType::Sniper => 16 + offset,
            UnitType::Ballistic => 24 + offset,
            UnitType::Juggernaut => 32 + offset,
            UnitType::Heavy => 40 + offset,
            UnitType::Commander => 48 + offset,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub enum Obstacle {
    Wall,
    Boulder,
}

impl Obstacle {
    pub fn index(&self) -> usize {
        match self {
            Obstacle::Wall => 0,
            Obstacle::Boulder => 1,
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct Selection;

#[derive(Debug, Clone, PartialEq, Component)]
pub enum StatText {
    Dna,
    Name,
    Health,
    Damage,
    Speed,
    Range,
    Reward,
    MoveDirection,
    AttackDirection,
    Cost,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component)]
pub struct Position(usize, usize);

#[derive(Debug, Clone, Component)]
pub struct PossibleMovement;

#[derive(Debug, Clone, Component)]
pub struct PossibleAttack(usize, usize);

#[derive(Debug, Clone, Component)]
pub struct AttackDirection;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pestilence".into(),
                name: Some("pestilence".into()),
                resolution: (GAME_WIDTH, GAME_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_event::<ChangeLevel>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                select_unit,
                infect_unit,
                movement,
                attack,
                turn,
                move_camera,
                (win, listen_change_level).chain(),
                animate,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(AnimationQueue {
        queue: Vec::new(),
        started: false,
        finished: false,
    });
    commands.insert_resource(Selected(None));
    commands.insert_resource(Turn::HumansMove);

    commands.spawn(Camera2dBundle::default());

    let sprites = setup_sprites(&mut commands, &asset_server, &mut texture_atlas_layouts);

    commands.spawn((
        Selection,
        SpriteSheetBundle {
            texture: sprites.selections.0.clone(),
            atlas: TextureAtlas {
                layout: sprites.selections.1.clone(),
                index: 0,
            },
            transform: Transform::from_xyz(-GAME_WIDTH, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
    ));

    let (level, order) = setup_level(&mut commands, &sprites, 0);
    commands.insert_resource(CurrentLevel(level.clone()));
    commands.insert_resource(TurnOrder(order));
    commands.insert_resource(Dna(level.initial_dna));

    commands.spawn(AtlasImageBundle {
        style: Style {
            width: Val::Px(128.0),
            height: Val::Px(416.0),
            position_type: PositionType::Absolute,
            right: Val::Px(32.0),
            top: Val::Px(32.0),
            ..default()
        },
        texture_atlas: sprites.ui_background.1.into(),
        image: UiImage::new(sprites.ui_background.0),
        ..default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(128.0),
                height: Val::Px(416.0),
                position_type: PositionType::Absolute,
                right: Val::Px(32.0),
                top: Val::Px(32.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                StatText::Dna,
                TextBundle::from_section(
                    format!("DNA: {}", level.initial_dna),
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(24.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Name,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(52.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Health,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(88.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Damage,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(104.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Speed,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(120.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Range,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(136.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Reward,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(152.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::MoveDirection,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(168.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::AttackDirection,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(184.0),
                    ..default()
                }),
            ));

            parent.spawn((
                StatText::Cost,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::WHITE,
                        ..TextStyle::default()
                    },
                )
                .with_style(Style {
                    top: Val::Px(224.0),
                    ..default()
                }),
            ));
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                left: Val::Px(GAME_WIDTH / 2.0 - 96.0),
                top: Val::Px(GAME_HEIGHT / 2.0 - 80.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(64.0),
                        height: Val::Px(32.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Infect".into(),
                                style: TextStyle {
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                    ..TextStyle::default()
                                },
                            }],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::NoWrap,
                        },
                        ..default()
                    });
                });
        });
}

fn setup_sprites(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Sprites {
    let attack_directions_texture = asset_server.load("sprites/attack_directions.png");
    let attack_directions_layout =
        TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 4, 1, None, None);
    let attack_directions_texture_atlas_layout =
        texture_atlas_layouts.add(attack_directions_layout);

    let obstacles_texture = asset_server.load("sprites/obstacles.png");
    let obstacles_layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 2, 1, None, None);
    let obstacles_texture_atlas_layout = texture_atlas_layouts.add(obstacles_layout);

    let selections_texture = asset_server.load("sprites/selections.png");
    let selections_layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 3, 1, None, None);
    let selections_texture_atlas_layout = texture_atlas_layouts.add(selections_layout);

    let tiles_texture = asset_server.load("sprites/tiles.png");
    let tiles_layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 3, 1, None, None);
    let tiles_texture_atlas_layout = texture_atlas_layouts.add(tiles_layout);

    let ui_background_texture = asset_server.load("sprites/ui_background.png");
    let ui_background_layout =
        TextureAtlasLayout::from_grid(Vec2::new(64.0, 208.0), 1, 1, None, None);
    let ui_background_texture_atlas_layout = texture_atlas_layouts.add(ui_background_layout);

    let units_texture = asset_server.load("sprites/units.png");
    let units_layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 4, 56, None, None);
    let units_texture_atlas_layout = texture_atlas_layouts.add(units_layout);

    let units_animations = (0..14)
        .flat_map(|i| {
            [
                AnimationTimer {
                    timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                    first: i * 16,
                    last: i * 16 + 3,
                },
                AnimationTimer {
                    timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                    first: i * 16 + 4,
                    last: i * 16 + 7,
                },
                AnimationTimer {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                    first: i * 16 + 8,
                    last: i * 16 + 9,
                },
                AnimationTimer {
                    timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                    first: i * 16 + 12,
                    last: i * 16 + 14,
                },
            ]
        })
        .collect();

    let sprites = Sprites {
        attack_directions: (
            attack_directions_texture,
            attack_directions_texture_atlas_layout,
        ),
        obstacles: (obstacles_texture, obstacles_texture_atlas_layout),
        selections: (selections_texture, selections_texture_atlas_layout),
        tiles: (tiles_texture, tiles_texture_atlas_layout),
        ui_background: (ui_background_texture, ui_background_texture_atlas_layout),
        units: (units_texture, units_texture_atlas_layout, units_animations),
    };

    commands.insert_resource(sprites.clone());
    sprites
}

fn setup_level(commands: &mut Commands, sprites: &Sprites, level_id: usize) -> (Level, Vec<usize>) {
    let level = &levels()[level_id];

    let width = level.tilemap[0].len();
    let height = level.tilemap.len();
    let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
    let offset_y = height as f32 / 2.0 * 64.0 - 32.0;

    for (j, row) in level.tilemap.iter().enumerate() {
        for (i, sprite_index) in row.iter().enumerate() {
            commands.spawn((
                Tile,
                SpriteSheetBundle {
                    texture: sprites.tiles.0.clone(),
                    atlas: TextureAtlas {
                        layout: sprites.tiles.1.clone(),
                        index: *sprite_index,
                    },
                    transform: Transform::from_xyz(
                        i as f32 * 64.0 - offset_x,
                        j as f32 * 64.0 - offset_y,
                        -2.0,
                    )
                    .with_scale(Vec3::splat(2.0)),
                    ..default()
                },
            ));
        }
    }

    for i in -BORDER..width as i16 + BORDER {
        for j in -BORDER..height as i16 + BORDER {
            if !(i >= 0 && i < width as i16 && j >= 0 && j < height as i16) {
                commands.spawn((
                    Tile,
                    SpriteSheetBundle {
                        texture: sprites.tiles.0.clone(),
                        atlas: TextureAtlas {
                            layout: sprites.tiles.1.clone(),
                            index: 0,
                        },
                        transform: Transform::from_xyz(
                            i as f32 * 64.0 - offset_x,
                            j as f32 * 64.0 - offset_y,
                            -2.0,
                        )
                        .with_scale(Vec3::splat(2.0)),
                        ..default()
                    },
                ));
            }
        }
    }

    for (id, (unit_type, position)) in level.units.iter().enumerate() {
        let Position(col, row) = position;
        let unit = Unit {
            id,
            ..UNITS[unit_type.index()].clone()
        };
        let timer = sprites.units.2[unit.animation_index()].clone();
        commands.spawn((
            unit.clone(),
            position.clone(),
            SpriteSheetBundle {
                texture: sprites.units.0.clone(),
                atlas: TextureAtlas {
                    layout: sprites.units.1.clone(),
                    index: timer.first,
                },
                transform: Transform::from_xyz(
                    *col as f32 * 64.0 - offset_x,
                    *row as f32 * 64.0 - offset_y,
                    -1.0,
                )
                .with_scale(Vec3::splat(2.0)),
                ..default()
            },
            timer,
        ));
    }

    for (obstacle, position) in &level.obstacles {
        let Position(col, row) = position;
        commands.spawn((
            obstacle.clone(),
            position.clone(),
            SpriteSheetBundle {
                texture: sprites.obstacles.0.clone(),
                atlas: TextureAtlas {
                    layout: sprites.obstacles.1.clone(),
                    index: obstacle.index(),
                },
                transform: Transform::from_xyz(
                    *col as f32 * 64.0 - offset_x,
                    *row as f32 * 64.0 - offset_y,
                    -1.0,
                )
                .with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ));
    }

    let mut turn_order: Vec<_> = level.units.iter().enumerate().collect();
    turn_order.sort_by(|(_, (unit_a, position_a)), (_, (unit_b, position_b))| {
        unit_a
            .order()
            .cmp(&unit_b.order())
            .then(position_a.0.cmp(&position_b.0))
            .then(position_a.1.cmp(&position_b.1))
    });
    let turn_order: Vec<_> = turn_order.iter().map(|(id, _)| *id).collect();

    (level.clone(), turn_order)
}

fn listen_change_level(
    mut commands: Commands,
    mut events: EventReader<ChangeLevel>,
    sprites: Res<Sprites>,
    mut current_level: ResMut<CurrentLevel>,
    mut turn_order: ResMut<TurnOrder>,
    mut dna: ResMut<Dna>,
    tiles: Query<Entity, With<Tile>>,
    obstacles: Query<Entity, With<Unit>>,
    units: Query<Entity, With<Unit>>,
    movements: Query<Entity, With<PossibleMovement>>,
    attacks: Query<Entity, With<PossibleAttack>>,
    attack_directions: Query<Entity, With<AttackDirection>>,
) {
    for event in events.read() {
        for entity in tiles.iter() {
            commands
                .entity(entity)
                .remove::<(Tile, SpriteSheetBundle)>();
        }

        for entity in obstacles.iter() {
            commands
                .entity(entity)
                .remove::<(Obstacle, Position, SpriteSheetBundle)>();
        }

        for entity in units.iter() {
            commands
                .entity(entity)
                .remove::<(Unit, Position, SpriteSheetBundle)>();
        }

        for entity in movements.iter() {
            commands
                .entity(entity)
                .remove::<(PossibleMovement, Position, SpriteSheetBundle)>();
        }

        for entity in attacks.iter() {
            commands
                .entity(entity)
                .remove::<(PossibleAttack, Position, SpriteSheetBundle)>();
        }

        for entity in attack_directions.iter() {
            commands
                .entity(entity)
                .remove::<(AttackDirection, Position, SpriteSheetBundle)>();
        }

        let (level, order) = setup_level(&mut commands, &*sprites, event.level_id);
        current_level.0 = level.clone();
        turn_order.0 = order;
        dna.0 = level.initial_dna;
    }
}

fn select_unit(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    sprites: Res<Sprites>,
    animation_queue: Res<AnimationQueue>,
    level: Res<CurrentLevel>,
    mut selected: ResMut<Selected>,
    turn: Res<Turn>,
    mut camera_selection: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<&mut Transform, With<Selection>>,
    )>,
    mut units: Query<(&mut Unit, &Position)>,
    obstacles: Query<(&Obstacle, &Position)>,
    mut stat_texts: Query<(&StatText, &mut Text)>,
    movements: Query<Entity, With<PossibleMovement>>,
    attacks: Query<Entity, With<PossibleAttack>>,
    attack_directions: Query<Entity, With<AttackDirection>>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        let camera = camera_selection.p0();
        let camera_transform = camera.iter().next().unwrap();
        let mouse_x = position.x + camera_transform.translation.x;
        let mouse_y = position.y - camera_transform.translation.y;

        let CurrentLevel(level) = &*level;
        let width = level.tilemap[0].len();
        let height = level.tilemap.len();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;
        let col = ((mouse_x - GAME_WIDTH / 2.0 + offset_x + 32.0) / 64.0).floor();
        let row = ((GAME_HEIGHT / 2.0 - mouse_y + offset_y + 32.0) / 64.0).floor();

        let mut selection = camera_selection.p1();
        let mut selection_transform = selection.iter_mut().next().unwrap();
        if col >= 0.0 && col < width as f32 && row >= 0.0 && row < height as f32 {
            selection_transform.translation.x = col * 64.0 - offset_x;
            selection_transform.translation.y = row * 64.0 - offset_y;

            if mouse_button_input.just_released(MouseButton::Left) && *turn == Turn::Parasites {
                for entity in movements.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleMovement, Position, SpriteSheetBundle)>();
                }

                for entity in attacks.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleAttack, Position, SpriteSheetBundle)>();
                }

                for entity in attack_directions.iter() {
                    commands
                        .entity(entity)
                        .remove::<(AttackDirection, Position, SpriteSheetBundle)>();
                }

                if let Some((unit, position)) = units
                    .iter()
                    .find(|(_, position)| **position == Position(col as usize, row as usize))
                {
                    *selected = Selected(Some(unit.id));

                    let units_list: Vec<_> = units
                        .iter()
                        .map(|(unit, position)| (unit.clone(), position.clone()))
                        .collect();
                    let obstacles_list: Vec<_> = obstacles
                        .iter()
                        .map(|(obstacle, position)| (obstacle.clone(), position.clone()))
                        .collect();

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Name)
                        .unwrap();
                    text.sections[0].value = unit.kind.name().into();

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Health)
                        .unwrap();
                    text.sections[0].value = format!("Health: {}/{}", unit.health, unit.max_health);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Damage)
                        .unwrap();
                    text.sections[0].value = format!("Damage: {}", unit.damage);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Speed)
                        .unwrap();
                    text.sections[0].value = format!("Speed: {}", unit.speed);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Range)
                        .unwrap();
                    text.sections[0].value = format!("Range: {}", unit.range);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Reward)
                        .unwrap();
                    text.sections[0].value = format!("Reward: {} DNA", unit.dna);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::MoveDirection)
                        .unwrap();
                    text.sections[0].value = format!("Moves {}", unit.move_direction.text());

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::AttackDirection)
                        .unwrap();
                    text.sections[0].value =
                        format!("Attacks {}", unit.attack_pattern.direction.text());

                    if unit.parasite {
                        if !unit.has_attacked {
                            let (mut unit, position) = units
                                .iter_mut()
                                .find(|(_, position)| {
                                    **position == Position(col as usize, row as usize)
                                })
                                .unwrap();
                            if unit.has_moved {
                                let attacks = possible_attacks(
                                    &unit,
                                    &position,
                                    &level,
                                    &units_list,
                                    &obstacles_list,
                                );

                                if attacks.is_empty() {
                                    unit.has_attacked = true;
                                }

                                for (i, attack_direction) in attacks.iter().enumerate() {
                                    for (j, attack) in attack_direction.iter().enumerate() {
                                        let Position(col, row) = attack;
                                        commands.spawn((
                                            PossibleAttack(i, j),
                                            attack.clone(),
                                            SpriteSheetBundle {
                                                texture: sprites.selections.0.clone(),
                                                atlas: TextureAtlas {
                                                    layout: sprites.selections.1.clone(),
                                                    index: 2,
                                                },
                                                transform: Transform::from_xyz(
                                                    *col as f32 * 64.0 - offset_x,
                                                    *row as f32 * 64.0 - offset_y,
                                                    -0.5,
                                                )
                                                .with_scale(Vec3::splat(2.0)),
                                                ..default()
                                            },
                                        ));
                                    }
                                }
                            } else {
                                let movements = possible_movements(
                                    &unit,
                                    position,
                                    &level,
                                    &units_list,
                                    &obstacles_list,
                                );
                                for movement in movements {
                                    let Position(col, row) = movement;
                                    commands.spawn((
                                        PossibleMovement,
                                        movement,
                                        SpriteSheetBundle {
                                            texture: sprites.selections.0.clone(),
                                            atlas: TextureAtlas {
                                                layout: sprites.selections.1.clone(),
                                                index: 1,
                                            },
                                            transform: Transform::from_xyz(
                                                col as f32 * 64.0 - offset_x,
                                                row as f32 * 64.0 - offset_y,
                                                -0.5,
                                            )
                                            .with_scale(Vec3::splat(2.0)),
                                            ..default()
                                        },
                                    ));
                                }
                            }
                        }
                    } else {
                        let (_, mut text) = stat_texts
                            .iter_mut()
                            .find(|(stat_text, _)| **stat_text == StatText::Cost)
                            .unwrap();
                        text.sections[0].value = format!("Cost: {}", unit.dna * 2);

                        if let Some(attack_directions) = &unit.attack_directions {
                            for direction in attack_directions {
                                let index = match direction {
                                    (1, 0) | (-1, 0) => 0,
                                    (0, 1) | (0, -1) => 1,
                                    (1, -1) | (-1, 1) => 2,
                                    (1, 1) | (-1, -1) => 3,
                                    _ => unreachable!(),
                                };

                                let positions = attack_positions(
                                    &unit,
                                    *direction,
                                    position,
                                    &level,
                                    &units_list,
                                    &obstacles_list,
                                );
                                for position in positions {
                                    let Position(col, row) = position;
                                    commands.spawn((
                                        AttackDirection,
                                        position,
                                        SpriteSheetBundle {
                                            texture: sprites.attack_directions.0.clone(),
                                            atlas: TextureAtlas {
                                                layout: sprites.attack_directions.1.clone(),
                                                index,
                                            },
                                            transform: Transform::from_xyz(
                                                col as f32 * 64.0 - offset_x,
                                                row as f32 * 64.0 - offset_y,
                                                -0.5,
                                            )
                                            .with_scale(Vec3::splat(2.0)),
                                            ..default()
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        } else {
            selection_transform.translation.x = -GAME_WIDTH;
        }
    } else {
        let mut selection = camera_selection.p1();
        let mut selection_transform = selection.iter_mut().next().unwrap();
        selection_transform.translation.x = -GAME_WIDTH;
    }
}

fn infect_unit(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    sprites: Res<Sprites>,
    animation_queue: Res<AnimationQueue>,
    mut selected: ResMut<Selected>,
    mut dna: ResMut<Dna>,
    mut units: Query<(&mut Unit, &mut AnimationTimer, &mut TextureAtlas)>,
    mut stat_texts: Query<(&StatText, &mut Text)>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        // If clicked infect button
        if mouse_button_input.just_released(MouseButton::Left)
            && position.x >= GAME_WIDTH - 96.0 - 32.0
            && position.x <= GAME_WIDTH - 96.0 + 32.0
            && position.y >= GAME_HEIGHT - 80.0 - 16.0
            && position.y <= GAME_HEIGHT - 80.0 + 16.0
        {
            if let Selected(Some(id)) = *selected {
                let (mut unit, mut timer, mut unit_texture) =
                    units.iter_mut().find(|(unit, _, _)| unit.id == id).unwrap();

                let cost = unit.dna * 2;
                if dna.0 >= cost {
                    unit.parasite = true;

                    let new_timer = sprites.units.2[unit.animation_index()].clone();
                    unit_texture.index = new_timer.first;
                    *timer = new_timer;

                    dna.0 -= cost;

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Dna)
                        .unwrap();
                    text.sections[0].value = format!("DNA: {}", dna.0);

                    let (_, mut text) = stat_texts
                        .iter_mut()
                        .find(|(stat_text, _)| **stat_text == StatText::Cost)
                        .unwrap();
                    text.sections[0].value = "".into();

                    selected.0 = None;
                }
            }
        }
    }
}

fn movement(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    sprites: Res<Sprites>,
    mut animation_queue: ResMut<AnimationQueue>,
    level: Res<CurrentLevel>,
    selected: Res<Selected>,
    camera: Query<&Transform, With<Camera>>,
    mut units_obstacles_spaces: ParamSet<(
        Query<(&mut Unit, &mut Position)>,
        Query<(&Obstacle, &Position)>,
        Query<(&PossibleMovement, &Position, Entity)>,
        Query<(&PossibleAttack, &Position, Entity)>,
    )>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        let camera_transform = camera.iter().next().unwrap();
        let mouse_x = position.x + camera_transform.translation.x;
        let mouse_y = position.y - camera_transform.translation.y;

        let CurrentLevel(level) = &*level;
        let width = level.tilemap[0].len();
        let height = level.tilemap.len();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;
        let col = ((mouse_x - GAME_WIDTH / 2.0 + offset_x + 32.0) / 64.0).floor();
        let row = ((GAME_HEIGHT / 2.0 - mouse_y + offset_y + 32.0) / 64.0).floor();
        let movement = Position(col as usize, row as usize);

        let movements = units_obstacles_spaces.p2();
        if mouse_button_input.just_released(MouseButton::Left)
            && movements
                .iter()
                .any(|(_, position, _)| *position == movement)
        {
            if let Selected(Some(id)) = *selected {
                for (_, _, entity) in movements.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleMovement, Position, SpriteSheetBundle)>();
                }

                let attacks = units_obstacles_spaces.p3();
                for (_, _, entity) in attacks.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleAttack, Position, SpriteSheetBundle)>();
                }

                let mut units = units_obstacles_spaces.p0();
                let (mut unit, mut position) =
                    units.iter_mut().find(|(unit, _)| unit.id == id).unwrap();

                animation_queue.queue.push(Animation::UnitMove {
                    id,
                    start: *position,
                    goal: movement,
                    progress: 0.0,
                });
                animation_queue.started = true;

                *position = movement;
                unit.has_moved = true;

                let units_list: Vec<_> = units_obstacles_spaces
                    .p0()
                    .iter()
                    .map(|(unit, position)| (unit.clone(), position.clone()))
                    .collect();
                let obstacles_list: Vec<_> = units_obstacles_spaces
                    .p1()
                    .iter()
                    .map(|(obstacle, position)| (obstacle.clone(), position.clone()))
                    .collect();
                let (unit, position) = units_list.iter().find(|(unit, _)| unit.id == id).unwrap();
                let attacks =
                    possible_attacks(unit, &position, &level, &units_list, &obstacles_list);

                if attacks.is_empty() {
                    let mut units = units_obstacles_spaces.p0();
                    let (mut unit, _) = units.iter_mut().find(|(unit, _)| unit.id == id).unwrap();
                    unit.has_attacked = true;
                }

                for (i, attack_direction) in attacks.iter().enumerate() {
                    for (j, attack) in attack_direction.iter().enumerate() {
                        let Position(col, row) = attack;
                        commands.spawn((
                            PossibleAttack(i, j),
                            attack.clone(),
                            SpriteSheetBundle {
                                texture: sprites.selections.0.clone(),
                                atlas: TextureAtlas {
                                    layout: sprites.selections.1.clone(),
                                    index: 2,
                                },
                                transform: Transform::from_xyz(
                                    *col as f32 * 64.0 - offset_x,
                                    *row as f32 * 64.0 - offset_y,
                                    -0.5,
                                )
                                .with_scale(Vec3::splat(2.0)),
                                ..default()
                            },
                        ));
                    }
                }
            }
        }
    }
}

fn attack(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    animation_queue: Res<AnimationQueue>,
    level: Res<CurrentLevel>,
    selected: Res<Selected>,
    mut dna: ResMut<Dna>,
    mut camera_units_attacks: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<(&mut Unit, &mut Position, &mut Transform, Entity)>,
        Query<(&PossibleAttack, &Position, Entity)>,
    )>,
    attack_directions: Query<Entity, With<AttackDirection>>,
    mut stat_texts: Query<(&StatText, &mut Text)>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        let camera = camera_units_attacks.p0();
        let camera_transform = camera.iter().next().unwrap();
        let mouse_x = position.x + camera_transform.translation.x;
        let mouse_y = position.y - camera_transform.translation.y;

        let CurrentLevel(level) = &*level;
        let width = level.tilemap[0].len();
        let height = level.tilemap.len();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;
        let col = ((mouse_x - GAME_WIDTH / 2.0 + offset_x + 32.0) / 64.0).floor();
        let row = ((GAME_HEIGHT / 2.0 - mouse_y + offset_y + 32.0) / 64.0).floor();
        let attack = Position(col as usize, row as usize);

        let attacks = camera_units_attacks.p2();
        if mouse_button_input.just_released(MouseButton::Left) {
            if let Some((attack, position, _)) =
                attacks.iter().find(|(_, position, _)| **position == attack)
            {
                let attack = attack.clone();
                let position = position.clone();
                if let Selected(Some(id)) = *selected {
                    let mut units = camera_units_attacks.p1();
                    let (mut unit, mut unit_position, mut transform, _) = units
                        .iter_mut()
                        .find(|(unit, _, _, _)| unit.id == id)
                        .unwrap();
                    let damage = unit.damage;
                    let attack_pattern = unit.attack_pattern.clone();
                    unit.has_attacked = true;

                    if attack_pattern.charge {
                        let Position(col, row) = position;
                        let Position(unit_col, unit_row) = *unit_position;
                        if unit_col < col {
                            unit_position.0 = col - 1;
                        } else if unit_col > col {
                            unit_position.0 = col + 1;
                        }
                        if unit_row < row {
                            unit_position.1 = row - 1;
                        } else if unit_row > row {
                            unit_position.1 = row + 1;
                        }

                        let Position(col, row) = *unit_position;
                        transform.translation.x = col as f32 * 64.0 - offset_x;
                        transform.translation.y = row as f32 * 64.0 - offset_y;
                    }

                    let attacks = camera_units_attacks.p2();
                    let attack_positions = if attack_pattern.aoe {
                        let PossibleAttack(i, j) = attack;
                        attacks
                            .iter()
                            .filter_map(|(o_attack, position, _)| {
                                let PossibleAttack(oi, oj) = o_attack;
                                if *oj <= j && (attack_pattern.all_directions || *oi == i) {
                                    Some(position.clone())
                                } else {
                                    None
                                }
                            })
                            .collect()
                    } else {
                        vec![position]
                    };

                    for (_, _, entity) in attacks.iter() {
                        commands
                            .entity(entity)
                            .remove::<(PossibleAttack, Position, SpriteSheetBundle)>();
                    }

                    for entity in attack_directions.iter() {
                        commands
                            .entity(entity)
                            .remove::<(AttackDirection, Position, SpriteSheetBundle)>();
                    }

                    let mut units = camera_units_attacks.p1();
                    let targets: Vec<_> = units
                        .iter_mut()
                        .filter(|(_, position, _, _)| attack_positions.contains(position))
                        .collect();

                    for (mut target, _, _, entity) in targets {
                        target.health = target.health.checked_sub(damage).unwrap_or(0);

                        if target.health == 0 {
                            dna.0 += target.dna;

                            let (_, mut text) = stat_texts
                                .iter_mut()
                                .find(|(stat_text, _)| **stat_text == StatText::Dna)
                                .unwrap();
                            text.sections[0].value = format!("DNA: {}", dna.0);

                            commands.entity(entity).remove::<Unit>();
                            commands.entity(entity).remove::<Position>();
                            commands.entity(entity).remove::<SpriteSheetBundle>();
                        }
                    }
                }
            }
        }
    }
}

fn turn(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_queue: ResMut<AnimationQueue>,
    level: Res<CurrentLevel>,
    mut turn: ResMut<Turn>,
    turn_order: Res<TurnOrder>,
    dna: Res<Dna>,
    mut units_obstacles: ParamSet<(
        Query<(&mut Unit, &mut Position, &mut Transform, Entity)>,
        Query<(&Obstacle, &Position)>,
    )>,
    movements: Query<Entity, With<PossibleMovement>>,
    attacks: Query<Entity, With<PossibleAttack>>,
    attack_directions: Query<Entity, With<AttackDirection>>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    match *turn {
        Turn::Parasites => {
            let mut units = units_obstacles.p0();
            let all_units_attacked = units
                .iter()
                .all(|(unit, _, _, _)| !unit.parasite || unit.has_attacked);
            let not_enough_dna = units.iter().all(|(unit, _, _, _)| unit.dna * 2 > dna.0);
            if keyboard_input.just_released(KeyCode::Enter)
                || (all_units_attacked && not_enough_dna)
            {
                for entity in movements.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleMovement, Position, SpriteSheetBundle)>();
                }

                for entity in attacks.iter() {
                    commands
                        .entity(entity)
                        .remove::<(PossibleAttack, Position, SpriteSheetBundle)>();
                }

                for entity in attack_directions.iter() {
                    commands
                        .entity(entity)
                        .remove::<(AttackDirection, Position, SpriteSheetBundle)>();
                }

                turn.next();
                units.iter_mut().for_each(|(mut unit, _, _, _)| {
                    unit.has_moved = false;
                    unit.has_attacked = false;
                });
            }
        }
        Turn::HumansMove => {
            if !units_obstacles
                .p0()
                .iter()
                .any(|(unit, _, _, _)| unit.parasite)
            {
                turn.next();
                return;
            }

            let CurrentLevel(level) = &*level;
            let TurnOrder(turn_order) = &*turn_order;

            let mut units_list: Vec<_> = units_obstacles
                .p0()
                .iter()
                .map(|(unit, position, _, _)| (unit.clone(), position.clone()))
                .collect();
            let obstacles_list: Vec<_> = units_obstacles
                .p1()
                .iter()
                .map(|(obstacle, position)| (obstacle.clone(), position.clone()))
                .collect();

            let mut units = units_obstacles.p0();
            for id in turn_order {
                if let Some((unit, position, _, _)) =
                    units.iter().find(|(unit, _, _, _)| unit.id == *id)
                {
                    if unit.parasite {
                        continue;
                    }

                    let attack_positions: Vec<_> = units
                        .iter()
                        .flat_map(|(target, position, _, _)| {
                            if target.parasite {
                                longest_range_attacks(
                                    &unit,
                                    position,
                                    &level,
                                    &units_list,
                                    &obstacles_list,
                                )
                            } else {
                                Vec::new()
                            }
                        })
                        .collect();

                    let mut nearest_attack_position = None;
                    let mut smallest_cost = u16::MAX;
                    for direction in attack_positions {
                        for attack_position in direction {
                            let path = pathfind(
                                unit,
                                position,
                                &attack_position,
                                &level,
                                &units_list,
                                &obstacles_list,
                            );
                            if let Some((path, cost)) = path {
                                if !path.is_empty() && cost < smallest_cost {
                                    nearest_attack_position = Some(path[0].clone());
                                    smallest_cost = cost;
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(attack_position) = nearest_attack_position {
                        let (_, mut position, _, _) = units
                            .iter_mut()
                            .find(|(unit, _, _, _)| unit.id == *id)
                            .unwrap();

                        animation_queue.queue.push(Animation::UnitMove {
                            id: *id,
                            start: *position,
                            goal: attack_position,
                            progress: 0.0,
                        });
                        animation_queue.started = true;
                        *position = attack_position.clone();

                        units_list = units_list
                            .iter()
                            .map(|(unit, position)| {
                                if unit.id == *id {
                                    (unit.clone(), attack_position.clone())
                                } else {
                                    (unit.clone(), position.clone())
                                }
                            })
                            .collect();
                    }
                }
            }

            for (mut unit, position, _, _) in units.iter_mut() {
                if unit.parasite {
                    continue;
                }

                if unit.attack_pattern.all_directions {
                    unit.attack_directions = Some(unit.attack_pattern.direction.vectors());
                    continue;
                }

                let mut directions = Vec::new();
                'a: for direction in unit.attack_pattern.direction.vectors() {
                    let mut is_valid = true;
                    'b: for dist in 1..=unit.range {
                        let dcol = direction.0 * dist as isize;
                        let drow = direction.1 * dist as isize;
                        let Position(col, row) = *position;

                        if -dcol > col as isize || -drow > row as isize {
                            break 'b;
                        }

                        let attack = Position(
                            (col as isize + dcol) as usize,
                            (row as isize + drow) as usize,
                        );

                        if obstacles_list
                            .iter()
                            .any(|(_, position)| *position == attack)
                        {
                            if dist == 1 {
                                is_valid = false;
                            }
                            break 'b;
                        } else if let Some((target, _)) =
                            units_list.iter().find(|(_, position)| *position == attack)
                        {
                            if target.parasite {
                                unit.attack_directions = Some(vec![direction]);
                                break 'a;
                            } else {
                                is_valid = false;
                                break 'b;
                            }
                        }
                    }
                    if is_valid {
                        directions.push(direction);
                    }
                }

                if unit.attack_directions.is_none() && !directions.is_empty() {
                    directions.sort_by(|(dcol_a, drow_a), (dcol_b, drow_b)| {
                        let mut a_score = 0;
                        let mut b_score = 0;
                        for (target, target_position) in &units_list {
                            let Position(col, row) = *position;
                            let Position(target_col, target_row) = target_position;
                            if target.parasite {
                                if (*target_col < col && *dcol_a == -1)
                                    || (*target_col > col && *dcol_a == 1)
                                    || (*target_col == col && *dcol_a == 0)
                                {
                                    a_score += 1;
                                }
                                if (*target_row < row && *drow_a == -1)
                                    || (*target_row > row && *drow_a == 1)
                                    || (*target_row == row && *drow_a == 0)
                                {
                                    a_score += 1;
                                }

                                if (*target_col < col && *dcol_b == -1)
                                    || (*target_col > col && *dcol_b == 1)
                                    || (*target_col == col && *dcol_b == 0)
                                {
                                    b_score += 1;
                                }
                                if (*target_row < row && *drow_b == -1)
                                    || (*target_row > row && *drow_b == 1)
                                    || (*target_row == row && *drow_b == 0)
                                {
                                    b_score += 1;
                                }
                            }
                        }
                        a_score.cmp(&b_score).reverse()
                    });
                    unit.attack_directions = Some(vec![directions[0]]);
                }
            }

            turn.next();
        }
        Turn::HumansAttack => {
            let CurrentLevel(level) = &*level;
            let TurnOrder(turn_order) = &*turn_order;
            let width = level.tilemap[0].len();
            let height = level.tilemap.len();
            let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
            let offset_y = height as f32 / 2.0 * 64.0 - 32.0;

            for id in turn_order {
                if let Some((unit, position, _, _)) = units_obstacles
                    .p0()
                    .iter()
                    .find(|(unit, _, _, _)| unit.id == *id)
                {
                    let unit = unit.clone();
                    let position = position.clone();
                    if unit.parasite {
                        continue;
                    }

                    if let Some(attack_directions) = unit.attack_directions {
                        for direction in attack_directions {
                            for dist in 1..=unit.range {
                                let dcol = direction.0 * dist as isize;
                                let drow = direction.1 * dist as isize;
                                let Position(col, row) = position;

                                if -dcol > col as isize || -drow > row as isize {
                                    break;
                                }

                                let attack = Position(
                                    (col as isize + dcol) as usize,
                                    (row as isize + drow) as usize,
                                );
                                let Position(col, row) = attack;

                                if col >= width
                                    || row >= height
                                    || level.tilemap[row][col] == 0
                                    || units_obstacles
                                        .p1()
                                        .iter()
                                        .any(|(_, position)| *position == attack)
                                {
                                    break;
                                } else if let Some((mut target, target_position, _, entity)) =
                                    units_obstacles
                                        .p0()
                                        .iter_mut()
                                        .find(|(_, position, _, _)| **position == attack)
                                {
                                    target.health =
                                        target.health.checked_sub(unit.damage).unwrap_or(0);

                                    if target.health == 0 {
                                        commands.entity(entity).remove::<Unit>();
                                        commands.entity(entity).remove::<Position>();
                                        commands.entity(entity).remove::<SpriteSheetBundle>();
                                    }

                                    let target_position = target_position.clone();

                                    if unit.attack_pattern.charge {
                                        let mut units = units_obstacles.p0();
                                        let (_, mut position, mut transform, _) = units
                                            .iter_mut()
                                            .find(|(unit, _, _, _)| unit.id == *id)
                                            .unwrap();

                                        let Position(col, row) = *position;
                                        let Position(target_col, target_row) = target_position;
                                        if col < target_col {
                                            position.0 = col - 1;
                                        } else if row > target_col {
                                            position.0 = col + 1;
                                        }
                                        if row < target_row {
                                            position.1 = row - 1;
                                        } else if row > target_row {
                                            position.1 = row + 1;
                                        }

                                        let Position(col, row) = *position;
                                        transform.translation.x = col as f32 * 64.0 - offset_x;
                                        transform.translation.y = row as f32 * 64.0 - offset_y;
                                    }

                                    if !unit.attack_pattern.aoe {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            turn.next();
        }
    }
}

fn win(
    mut events: EventWriter<ChangeLevel>,
    animation_queue: Res<AnimationQueue>,
    level: Res<CurrentLevel>,
    dna: Res<Dna>,
    units: Query<&Unit>,
) {
    if !animation_queue.queue.is_empty() {
        return;
    }

    // Parasites win
    if units.iter().all(|unit| unit.parasite) {
        let CurrentLevel(level) = &*level;
        let level_id = if level.id + 1 >= levels().len() {
            0
        } else {
            level.id + 1
        };
        events.send(ChangeLevel { level_id });
    }

    // Humans win
    if units.iter().all(|unit| !unit.parasite) && units.iter().all(|unit| unit.dna * 2 > dna.0) {
        let CurrentLevel(level) = &*level;
        events.send(ChangeLevel { level_id: level.id });
    }
}

fn move_camera(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    level: Res<CurrentLevel>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        let mut camera_transform = camera.iter_mut().next().unwrap();
        let CurrentLevel(level) = &*level;
        let width = level.tilemap[0].len();
        let height = level.tilemap.len();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;

        for motion in mouse_motion.read() {
            let min_camera_x = -(BORDER as f32) * 64.0 - offset_x + GAME_WIDTH / 2.0;
            let max_camera_x =
                (width as i16 + BORDER - 1) as f32 * 64.0 - offset_x - GAME_WIDTH / 2.0;
            let min_camera_y = -(BORDER as f32) * 64.0 - offset_y + GAME_HEIGHT / 2.0;
            let max_camera_y =
                (height as i16 + BORDER - 1) as f32 * 64.0 - offset_y - GAME_HEIGHT / 2.0;

            camera_transform.translation.x -= motion.delta.x;
            if camera_transform.translation.x < min_camera_x {
                camera_transform.translation.x = min_camera_x;
            } else if camera_transform.translation.x > max_camera_x {
                camera_transform.translation.x = max_camera_x;
            }

            camera_transform.translation.y += motion.delta.y;
            if camera_transform.translation.y < min_camera_y {
                camera_transform.translation.y = min_camera_y;
            } else if camera_transform.translation.y > max_camera_y {
                camera_transform.translation.y = max_camera_y;
            }
        }
    }
}

fn animate(
    time: Res<Time>,
    sprites: Res<Sprites>,
    mut animation_queue: ResMut<AnimationQueue>,
    level: Res<CurrentLevel>,
    mut animations_units: ParamSet<(
        Query<(&mut AnimationTimer, &mut TextureAtlas)>,
        Query<(
            &Unit,
            &mut Transform,
            &mut AnimationTimer,
            &mut TextureAtlas,
        )>,
    )>,
) {
    for (mut timer, mut texture) in &mut animations_units.p0() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            texture.index = if texture.index == timer.last {
                timer.first
            } else {
                texture.index + 1
            };
        }
    }

    let mut units = animations_units.p1();

    if animation_queue.finished {
        match animation_queue.queue[0] {
            Animation::UnitMove { id, .. }
            | Animation::UnitAttack { id }
            | Animation::UnitDeath { id } => {
                let (unit, _, mut timer, mut texture) = units
                    .iter_mut()
                    .find(|(unit, _, _, _)| unit.id == id)
                    .unwrap();
                let new_timer = sprites.units.2[unit.animation_index()].clone();
                texture.index = new_timer.first;
                *timer = new_timer;
            }
        }

        animation_queue.queue = animation_queue.queue[1..].to_vec();
        animation_queue.finished = false;

        if !animation_queue.queue.is_empty() {
            animation_queue.started = true;
        }
    }

    if animation_queue.started {
        let (id, offset) = match animation_queue.queue[0] {
            Animation::UnitMove { id, .. } => (id, 1),
            Animation::UnitAttack { id } => (id, 2),
            Animation::UnitDeath { id } => (id, 3),
        };

        let (unit, _, mut timer, mut texture) = units
            .iter_mut()
            .find(|(unit, _, _, _)| unit.id == id)
            .unwrap();
        let new_timer = sprites.units.2[unit.animation_index() + offset].clone();
        texture.index = new_timer.first;
        *timer = new_timer;

        animation_queue.started = false;
    }

    if !animation_queue.queue.is_empty() {
        let CurrentLevel(level) = &*level;
        let width = level.tilemap[0].len();
        let height = level.tilemap.len();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;
        match &mut animation_queue.queue[0] {
            Animation::UnitMove {
                id,
                start,
                goal,
                progress,
            } => {
                let (_, mut transform, _, _) = units
                    .iter_mut()
                    .find(|(unit, _, _, _)| unit.id == *id)
                    .unwrap();

                let Position(start_col, start_row) = start;
                let Position(goal_col, goal_row) = goal;

                let col = *start_col as f32 + (*goal_col as f32 - *start_col as f32) * *progress;
                let row = *start_row as f32 + (*goal_row as f32 - *start_row as f32) * *progress;

                transform.translation.x = col * 64.0 - offset_x;
                transform.translation.y = row * 64.0 - offset_y;

                if *progress >= 1.0 {
                    animation_queue.finished = true;
                } else {
                    *progress +=
                        1.0 / (distance(start, goal) as f32) * time.delta().as_secs_f32() * 2.0;
                }
            }
            Animation::UnitAttack { .. } | Animation::UnitDeath { .. } => (),
        }
    }
}

fn possible_movements(
    unit: &Unit,
    position: &Position,
    level: &Level,
    units: &[(Unit, Position)],
    obstacles: &[(Obstacle, Position)],
) -> Vec<Position> {
    let mut movements = Vec::new();
    for direction in unit.move_direction.vectors() {
        for dist in 1..=unit.speed {
            let dcol = direction.0 * dist as isize;
            let drow = direction.1 * dist as isize;
            let Position(col, row) = position;

            if -dcol > *col as isize || -drow > *row as isize {
                break;
            }

            let movement = Position(
                (*col as isize + dcol) as usize,
                (*row as isize + drow) as usize,
            );
            let Position(col, row) = movement;

            let width = level.tilemap[0].len();
            let height = level.tilemap.len();

            if col >= width
                || row >= height
                || level.tilemap[row][col] == 0
                || units.iter().any(|(_, position)| *position == movement)
                || obstacles.iter().any(|(_, position)| *position == movement)
            {
                break;
            } else {
                movements.push(movement);
            }
        }
    }
    movements
}

fn possible_attacks(
    unit: &Unit,
    position: &Position,
    level: &Level,
    units: &[(Unit, Position)],
    obstacles: &[(Obstacle, Position)],
) -> Vec<Vec<Position>> {
    let mut attacks = Vec::new();
    let width = level.tilemap[0].len();
    let height = level.tilemap.len();

    for direction in unit.attack_pattern.direction.vectors() {
        let mut attack_direction = Vec::new();
        for dist in 1..=unit.range {
            let dcol = direction.0 * dist as isize;
            let drow = direction.1 * dist as isize;
            let Position(col, row) = position;

            if -dcol > *col as isize || -drow > *row as isize {
                break;
            }

            let attack = Position(
                (*col as isize + dcol) as usize,
                (*row as isize + drow) as usize,
            );
            let Position(col, row) = attack;

            if col >= width
                || row >= height
                || level.tilemap[row][col] == 0
                || obstacles.iter().any(|(_, position)| *position == attack)
            {
                break;
            } else if units.iter().any(|(_, position)| *position == attack) {
                attack_direction.push(attack);
                if !unit.attack_pattern.aoe {
                    break;
                }
            }
        }
        attacks.push(attack_direction);
    }

    attacks
}

fn attack_positions(
    unit: &Unit,
    direction: (isize, isize),
    position: &Position,
    level: &Level,
    units: &[(Unit, Position)],
    obstacles: &[(Obstacle, Position)],
) -> Vec<Position> {
    let mut positions = Vec::new();
    let width = level.tilemap[0].len();
    let height = level.tilemap.len();

    for dist in 1..=unit.range {
        let dcol = direction.0 * dist as isize;
        let drow = direction.1 * dist as isize;
        let Position(col, row) = position;

        if -dcol > *col as isize || -drow > *row as isize {
            break;
        }

        let attack_position = Position(
            (*col as isize + dcol) as usize,
            (*row as isize + drow) as usize,
        );
        let Position(col, row) = attack_position;

        if col >= width
            || row >= height
            || level.tilemap[row][col] == 0
            || obstacles
                .iter()
                .any(|(_, position)| *position == attack_position)
        {
            break;
        } else if units
            .iter()
            .any(|(_, position)| *position == attack_position)
        {
            positions.push(attack_position);
            if !unit.attack_pattern.aoe {
                break;
            }
        } else {
            positions.push(attack_position);
        }
    }

    positions
}

fn longest_range_attacks(
    unit: &Unit,
    target_position: &Position,
    level: &Level,
    units: &[(Unit, Position)],
    obstacles: &[(Obstacle, Position)],
) -> Vec<Vec<Position>> {
    let mut attacks = Vec::new();
    let width = level.tilemap[0].len();
    let height = level.tilemap.len();

    for direction in unit.attack_pattern.direction.vectors() {
        let mut direction_attacks = Vec::new();
        for dist in 1..=unit.range {
            let dcol = direction.0 * dist as isize;
            let drow = direction.1 * dist as isize;
            let Position(col, row) = target_position;

            if -dcol > *col as isize || -drow > *row as isize {
                break;
            }

            let attack = Position(
                (*col as isize + dcol) as usize,
                (*row as isize + drow) as usize,
            );
            let Position(col, row) = attack;

            if col >= width
                || row >= height
                || level.tilemap[row][col] == 0
                || units
                    .iter()
                    .any(|(other_unit, position)| other_unit.id != unit.id && *position == attack)
                || obstacles.iter().any(|(_, position)| *position == attack)
            {
                break;
            } else {
                direction_attacks.push(attack);
            }
        }
        attacks.push(direction_attacks.iter().rev().copied().collect());
    }

    attacks
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Frontier {
    priority: u16,
    position: Position,
}

impl Ord for Frontier {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority).then_with(|| {
            [self.position.0, self.position.1].cmp(&[other.position.0, other.position.1])
        })
    }
}

impl PartialOrd for Frontier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// A* algorithm
fn pathfind(
    unit: &Unit,
    start: &Position,
    goal: &Position,
    level: &Level,
    units: &[(Unit, Position)],
    obstacles: &[(Obstacle, Position)],
) -> Option<(Vec<Position>, u16)> {
    let mut frontier = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut costs = HashMap::new();

    frontier.push(Frontier {
        priority: 0,
        position: start.clone(),
    });
    costs.insert(start.clone(), 0);

    while let Some(Frontier {
        priority: _,
        position,
    }) = frontier.pop()
    {
        if position == *goal {
            break;
        }

        let units: Vec<_> = units
            .iter()
            .map(|(other_unit, other_position)| {
                if other_unit.id == unit.id {
                    (other_unit.clone(), position.clone())
                } else {
                    (other_unit.clone(), other_position.clone())
                }
            })
            .collect();

        for movement in possible_movements(unit, &position, level, &units, obstacles) {
            let new_cost = costs.get(&position).unwrap() + 1;
            if !costs.contains_key(&movement) || new_cost < *costs.get(&movement).unwrap() {
                frontier.push(Frontier {
                    priority: new_cost + distance(goal, &movement),
                    position: movement.clone(),
                });
                came_from.insert(movement.clone(), Some(position.clone()));
                costs.insert(movement, new_cost);
            }
        }
    }

    let mut position = goal;
    let mut path = Vec::new();

    while position != start {
        path.push(position.clone());
        position = match came_from.get(&position) {
            Some(Some(position)) => position,
            _ => return None,
        };
    }

    path.reverse();

    Some((path, *costs.get(&goal).unwrap()))
}

fn distance(a: &Position, b: &Position) -> u16 {
    let Position(acol, arow) = a;
    let Position(bcol, brow) = b;
    let col_diff = (*acol as f32 - *bcol as f32).abs();
    let row_diff = (*arow as f32 - *brow as f32).abs();
    (col_diff * col_diff + row_diff * row_diff).sqrt() as u16
}
