use std::fmt;

use serde::{Deserialize, Serialize};

pub const DOMINO_COUNT: usize = 16;
pub const DOMINOS: [Domino; DOMINO_COUNT] = [
    Domino {
        label: 21,
        value: 1,
    },
    Domino {
        label: 22,
        value: 1,
    },
    Domino {
        label: 23,
        value: 1,
    },
    Domino {
        label: 24,
        value: 1,
    },
    Domino {
        label: 25,
        value: 2,
    },
    Domino {
        label: 26,
        value: 2,
    },
    Domino {
        label: 27,
        value: 2,
    },
    Domino {
        label: 28,
        value: 2,
    },
    Domino {
        label: 29,
        value: 3,
    },
    Domino {
        label: 30,
        value: 3,
    },
    Domino {
        label: 31,
        value: 3,
    },
    Domino {
        label: 32,
        value: 3,
    },
    Domino {
        label: 33,
        value: 4,
    },
    Domino {
        label: 34,
        value: 4,
    },
    Domino {
        label: 35,
        value: 4,
    },
    Domino {
        label: 36,
        value: 4,
    },
];

#[derive(Serialize, Deserialize, Clone)]
pub struct Domino {
    pub label: u8,
    pub value: u8,
}

impl fmt::Debug for Domino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}
