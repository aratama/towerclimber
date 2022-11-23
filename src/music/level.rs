use crate::sound::{Music, Track};

pub static LEVEL_BGM_SCORE: &Music = &Music {
    unit: 7,
    tracks: [
        Track {
            volume: 90,
            notes: &[],
        },
        Track {
            volume: 90,
            notes: &[],
        },
        Track {
            volume: 100,
            notes: &[
                // 0
                (45, 1, 3),
                (45, 1, 3),
                (45, 2, 1),
                (45, 1, 2),
                (45, 2, 0),
                // 1
                (45, 1, 3),
                (45, 1, 3),
                (45, 2, 1),
                (45, 1, 2),
                (45, 2, 0),
                // 2
                (41, 1, 3),
                (41, 1, 3),
                (41, 2, 1),
                (41, 1, 2),
                (41, 2, 0),
                // 2
                (00, 0, 2),
                (43, 1, 1),
                (43, 1, 3),
                (43, 2, 1),
                (43, 1, 2),
                (43, 2, 0),
            ],
        },
        Track {
            volume: 20,
            notes: &[
                // 1
                (90, 1, 3),
                (75, 1, 3),
                (90, 1, 3),
                (75, 1, 3),
                // 2
                (90, 1, 3),
                (75, 1, 3),
                (90, 1, 3),
                (75, 1, 3),
                // 3
                (90, 1, 3),
                (75, 1, 3),
                (90, 1, 3),
                (75, 1, 3),
                // 3
                (90, 1, 3),
                (75, 1, 3),
                (90, 1, 3),
                (75, 1, 3),
            ],
        },
    ],
};