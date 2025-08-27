use num_enum::TryFromPrimitive;

static NUM_PLAYERS: u8 = 2;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub(crate) enum Turn {
    #[default]Player1,
    Player2,
    Player3,
    Player4,
}

impl Turn {

    /// Advances the turn by one.
    pub(crate) fn advance(&mut self) {
        let next = ((*self as u8) + 1) % NUM_PLAYERS;
        *self = Turn::try_from(next).unwrap();
    }

}