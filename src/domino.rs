use std::fmt;

pub const DOMINO_COUNT: usize = 16;
pub const DOMINOS: [Domino; DOMINO_COUNT] = [
    Domino {
        label: 21,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 22,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 23,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 24,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 25,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 26,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 27,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 28,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 29,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 30,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 31,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 32,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 33,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 34,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 35,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 36,
        value: 4,
        pickable: true,
    },
];
pub struct Domino {
    pub label: u8,
    pub value: u8,
    pub pickable: bool,
}

impl fmt::Debug for Domino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [pickable={}]", self.label, self.pickable)
    }
}

impl Domino {
    pub fn from(label: u8) -> Domino {
        match label {
            // find shorter way to do this
            21 => Domino {
                label: 21,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            22 => Domino {
                label: 22,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            23 => Domino {
                label: 23,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            24 => Domino {
                label: 24,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            25 => Domino {
                label: 25,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            26 => Domino {
                label: 26,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            27 => Domino {
                label: 27,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            28 => Domino {
                label: 28,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            29 => Domino {
                label: 29,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            30 => Domino {
                label: 30,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            31 => Domino {
                label: 31,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            32 => Domino {
                label: 32,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            33 => Domino {
                label: 33,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            34 => Domino {
                label: 34,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            35 => Domino {
                label: 35,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            36 => Domino {
                label: 36,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refactor Domino handling
            },
            _ => panic!("Wrong label"),
        }
    }
}
