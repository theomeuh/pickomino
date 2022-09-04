use std::fmt;

use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    // PickDomino,
    // StealDomino,
    // DrawDice,
    // GetGameState,
    // StartGame,
    // CreateUser,
    ListActions,
}
pub struct Actions(pub Vec<Action>);

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, action| {
            result.and_then(|_| writeln!(f, "{:#?}", action))
        })
    }
}
pub struct ActionParseError;

impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid action")
    }
}

impl std::str::FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Action, Self::Err> {
        match s {
            "list_actions" | "list" => Ok(Action::ListActions),
            // "create user" => Ok(Action::CreateUser),
            _ => Err(ActionParseError),
        }
    }
}
