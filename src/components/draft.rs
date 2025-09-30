use crate::components::domino::Domino;
use crate::components::player::Player;

pub(crate) const DRAFT_SIZE: usize = 4;


#[derive(Clone)]
pub(crate) struct Draft {


    been_selected_by: [Option<u8>; DRAFT_SIZE],
    draft: [Domino; DRAFT_SIZE],
    picked: usize,
}

impl Draft {

    /// Creates a new draft from sorted list of dominoes
    pub(crate) fn new(unsorted_draft: [Domino; DRAFT_SIZE]) -> Self {
        let mut draft = unsorted_draft;
        draft.sort();

        Self {
            been_selected_by: [None; DRAFT_SIZE],
            draft,
            picked: 0,
        }
    }



    /// Creates a new empty draft with len 0
    pub(crate) fn null() -> Self {

        Self {
            been_selected_by: [None; DRAFT_SIZE],
            draft: [Domino::null(); DRAFT_SIZE],
            picked: DRAFT_SIZE,
        }

    }

    /// Checks if the draft is null
    pub(crate) fn is_null(&self) -> bool {
        // draft is null iff the first domino inside it is null
        self.draft[0].is_null()
    }


    /// Returns the new player order. Called on the pick draft as it becomes the place draft.
    pub(crate) fn apply_new_order(&self, players: &mut [Player; DRAFT_SIZE]) {
        debug_assert!(self.is_empty());
        players.sort_by_key(|p| {
            self.been_selected_by
                .iter()
                .position(|&slot| slot == Some(p.id()))
                .expect("player id not found in been_selected_by")
        });
    }

    /// Returns the id of the player on the domino index of the draft (or none, if no player sits on the domino)
    pub(crate) fn player_on(&self, idx: usize) -> Option<u8> {
        debug_assert!(self.picked <= DRAFT_SIZE);

        self.been_selected_by[idx]
    }


    /// Checks if every domino has been picked
    pub(crate) fn is_empty(&self) -> bool {

        // Make sure we haven't somehow picked more dominoes than exist in the draft
        debug_assert!(self.picked <= DRAFT_SIZE);

        // Every domino in the draft has been picked.
        self.picked == DRAFT_SIZE

    }

    pub(crate) fn pickable(&self, idx: usize) -> bool {
        debug_assert!(idx < DRAFT_SIZE);

        self.been_selected_by[idx].is_none()
    }


    /// Picks the domino at the index from the draft
    pub(crate) fn pick(&mut self, idx: usize, player_id: u8) -> Domino {
        // make sure we pick a valid index, and there are still dominoes left to be picked
        debug_assert!(idx < DRAFT_SIZE && !self.is_empty());

        // make sure we haven't already picked the domino
        debug_assert!(self.pickable(idx));

        self.picked += 1;

        self.been_selected_by[idx] = Some(player_id);

        self.draft[idx]
    }

    /// Iterates through the list of dominoes in the draft
    pub fn iter(&self) -> impl Iterator<Item = &Domino> {

        self.draft[..DRAFT_SIZE].iter()
    }
}