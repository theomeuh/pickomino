#[derive(Debug)]
pub enum PickominoError {
    NoDiceToRoll,
    UnknownDiceLabel,
    CancelAction,
}
