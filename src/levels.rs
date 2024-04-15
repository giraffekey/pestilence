use crate::{Obstacle, Position, UnitType};

#[derive(Debug, Clone)]
pub struct Level {
    pub id: usize,
    pub tilemap: Vec<Vec<usize>>,
    pub units: Vec<(UnitType, Position)>,
    pub obstacles: Vec<(Obstacle, Position)>,
    pub initial_dna: u16,
}

impl Level {
    pub fn dimensions(&self) -> (usize, usize) {
        (self.tilemap[0].len(), self.tilemap.len())
    }

    pub fn offset(&self) -> (f32, f32) {
        let (width, height) = self.dimensions();
        let offset_x = width as f32 / 2.0 * 64.0 - 32.0;
        let offset_y = height as f32 / 2.0 * 64.0 - 32.0;
        (offset_x, offset_y)
    }
}

pub fn levels() -> Vec<Level> {
    vec![
        Level {
            id: 0,
            tilemap: vec![
                vec![
                    0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                ],
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
                ],
            ],
            units: vec![
                (UnitType::Assault, Position(5, 3)),
                (UnitType::Assault, Position(5, 5)),
                (UnitType::Assault, Position(13, 13)),
                (UnitType::Scout, Position(10, 4)),
                (UnitType::Scout, Position(17, 18)),
                (UnitType::Scout, Position(15, 18)),
                (UnitType::Sniper, Position(15, 3)),
                (UnitType::Sniper, Position(15, 5)),
                (UnitType::Ballistic, Position(4, 8)),
                (UnitType::Ballistic, Position(12, 2)),
                (UnitType::Juggernaut, Position(13, 7)),
                (UnitType::Juggernaut, Position(17, 11)),
                (UnitType::Heavy, Position(16, 10)),
                (UnitType::Commander, Position(3, 4)),
                (UnitType::Commander, Position(16, 16)),
            ],
            obstacles: vec![
                (Obstacle::Wall, Position(4, 3)),
                (Obstacle::Wall, Position(4, 4)),
                (Obstacle::Wall, Position(4, 5)),
                (Obstacle::Wall, Position(8, 3)),
                (Obstacle::Wall, Position(8, 5)),
                (Obstacle::Wall, Position(12, 3)),
                (Obstacle::Wall, Position(13, 3)),
                (Obstacle::Wall, Position(11, 5)),
                (Obstacle::Wall, Position(14, 6)),
                (Obstacle::Wall, Position(15, 6)),
                (Obstacle::Wall, Position(16, 8)),
                (Obstacle::Wall, Position(15, 10)),
                (Obstacle::Wall, Position(15, 16)),
                (Obstacle::Wall, Position(15, 15)),
                (Obstacle::Wall, Position(16, 15)),
            ],
            initial_dna: 4,
        },
        Level {
            id: 1,
            tilemap: vec![
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                ],
                vec![
                    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
                ],
                vec![
                    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                ],
                vec![
                    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            ],
            units: vec![
                (UnitType::Assault, Position(8, 8)),
                (UnitType::Assault, Position(8, 12)),
                (UnitType::Assault, Position(12, 8)),
                (UnitType::Assault, Position(12, 12)),
                (UnitType::Scout, Position(5, 5)),
                (UnitType::Scout, Position(5, 15)),
                (UnitType::Scout, Position(15, 5)),
                (UnitType::Scout, Position(15, 15)),
                (UnitType::Sniper, Position(6, 6)),
                (UnitType::Sniper, Position(6, 14)),
                (UnitType::Sniper, Position(14, 6)),
                (UnitType::Sniper, Position(14, 14)),
                (UnitType::Ballistic, Position(10, 6)),
                (UnitType::Ballistic, Position(10, 14)),
                (UnitType::Juggernaut, Position(6, 10)),
                (UnitType::Juggernaut, Position(14, 10)),
                (UnitType::Heavy, Position(10, 10)),
                (UnitType::Commander, Position(10, 2)),
                (UnitType::Commander, Position(2, 10)),
                (UnitType::Commander, Position(10, 18)),
                (UnitType::Commander, Position(18, 10)),
            ],
            obstacles: vec![
                (Obstacle::Wall, Position(8, 10)),
                (Obstacle::Wall, Position(10, 8)),
                (Obstacle::Wall, Position(12, 10)),
                (Obstacle::Wall, Position(10, 12)),
                (Obstacle::Wall, Position(7, 7)),
                (Obstacle::Wall, Position(7, 13)),
                (Obstacle::Wall, Position(13, 7)),
                (Obstacle::Wall, Position(13, 13)),
                (Obstacle::Wall, Position(4, 8)),
                (Obstacle::Wall, Position(16, 8)),
                (Obstacle::Wall, Position(4, 8)),
                (Obstacle::Wall, Position(4, 12)),
                (Obstacle::Wall, Position(8, 4)),
                (Obstacle::Wall, Position(12, 4)),
                (Obstacle::Wall, Position(8, 16)),
                (Obstacle::Wall, Position(12, 16)),
                (Obstacle::Wall, Position(16, 8)),
                (Obstacle::Wall, Position(16, 12)),
                (Obstacle::Wall, Position(3, 10)),
                (Obstacle::Wall, Position(10, 3)),
                (Obstacle::Wall, Position(10, 17)),
                (Obstacle::Wall, Position(17, 10)),
            ],
            initial_dna: 6,
        },
    ]
}
