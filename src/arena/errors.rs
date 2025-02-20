use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameStateError {
    #[error("Invalid round")]
    InvalidRoundIndex,
    #[error("The ammount bet doesn't call the previous bet")]
    BetSizeDoesntCall,
    #[error("The ammount bet doesn't call our own previous bet")]
    BetSizeDoesntCallSelf,
    #[error("The raise is below the minimum raise size")]
    RaiseSizeTooSmall,
    #[error("Can't advance after showdown")]
    CantAdvanceRound,
}
