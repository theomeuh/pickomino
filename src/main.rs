use rand::Rng;
use std::fmt;

fn main() {
    #[derive(Debug)]
    enum DieValue {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Maggot = 6,
    }


    struct Die {
        label: DieValue,
        value: u8,
    }

    impl fmt::Display for Die {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.label)
        }
    }

    impl Die {
        fn roll() -> Die {
            match rand::thread_rng().gen_range(1..=6) {
                1 => Die {
                    label: DieValue::One,
                    value: 1,
                },
                2 => Die {
                    label: DieValue::Two,
                    value: 2,
                },
                3 => Die {
                    label: DieValue::Three,
                    value: 3,
                },
                4 => Die {
                    label: DieValue::Four,
                    value: 4,
                },
                5 => Die {
                    label: DieValue::Five,
                    value: 5,
                },
                6 => Die {
                    label: DieValue::Maggot,
                    value: 5,
                },
                _ => unreachable!(),
            }
        }
    }

    struct DiceRoll {
        dice: [Die; 8],
    }

    impl fmt::Display for DiceRoll {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                self.dice[0].label,
                self.dice[1].label,
                self.dice[2].label,
                self.dice[3].label,
                self.dice[4].label,
                self.dice[5].label,
                self.dice[6].label,
                self.dice[7].label
            )
        }
    }

    impl DiceRoll {
        fn roll() -> DiceRoll {
            DiceRoll {
                dice: [
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                ],
            }
        }
    }

    println!("Dice roll: {}", DiceRoll::roll())
}
