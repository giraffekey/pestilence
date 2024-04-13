use crate::{Obstacle, Position, UnitType};

#[derive(Debug, Clone)]
pub struct Level {
    pub id: usize,
    pub tilemap: Vec<Vec<usize>>,
    pub units: Vec<(UnitType, Position)>,
    pub obstacles: Vec<(Obstacle, Position)>,
    pub initial_dna: u16,
}

pub fn levels() -> Vec<Level> {
    vec![
        Level {
            id: 0,
            tilemap: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 2, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 2, 1, 1, 1, 1],
                vec![1, 2, 1, 1, 1, 1, 1, 1],
            ],
            units: vec![
                (UnitType::Soldier, Position(0, 1)),
                (UnitType::Scout, Position(2, 2)),
                (UnitType::Sniper, Position(4, 3)),
                (UnitType::Ballistic, Position(6, 0)),
                (UnitType::Juggernaut, Position(3, 5)),
                (UnitType::Heavy, Position(5, 3)),
                (UnitType::Commander, Position(7, 5)),
            ],
            obstacles: vec![
                (Obstacle::Wall, Position(1, 1)),
                (Obstacle::Wall, Position(3, 3)),
                (Obstacle::Wall, Position(4, 2)),
                (Obstacle::Wall, Position(6, 4)),
                (Obstacle::Boulder, Position(4, 1)),
            ],
            initial_dna: 10,
        },
        Level {
            id: 1,
            tilemap: vec![
                vec![0, 1, 1, 1, 1, 1, 0, 0],
                vec![1, 1, 1, 1, 1, 2, 1, 1],
                vec![1, 2, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 2, 1, 1, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 0, 0],
            ],
            units: vec![
                (UnitType::Scout, Position(0, 1)),
                (UnitType::Scout, Position(2, 2)),
                (UnitType::Sniper, Position(4, 3)),
                (UnitType::Ballistic, Position(6, 0)),
                (UnitType::Ballistic, Position(3, 5)),
                (UnitType::Heavy, Position(5, 3)),
                (UnitType::Commander, Position(7, 4)),
            ],
            obstacles: vec![
                (Obstacle::Wall, Position(1, 1)),
                (Obstacle::Wall, Position(3, 3)),
                (Obstacle::Wall, Position(4, 2)),
                (Obstacle::Wall, Position(6, 4)),
                (Obstacle::Boulder, Position(4, 1)),
            ],
            initial_dna: 10,
        },
    ]
}
