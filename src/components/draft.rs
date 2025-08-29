use crate::components::domino::Domino;


const DRAFT_SIZE: usize = 4;

pub(crate) struct Draft {

    draft: [Domino; DRAFT_SIZE],
    picked: usize,
}

impl Draft {

    /// Creates a new draft from sorted list of dominoes
    pub(crate) fn new(unsorted_draft: [Domino; DRAFT_SIZE]) -> Self {
        let mut draft = unsorted_draft;
        draft.sort();

        Self {
            draft,
            picked: 0,
        }
    }


    /// Creates a new empty draft with len 0
    pub(crate) fn empty() -> Self {

        Self {
            draft: [Domino::null(), Domino::null(), Domino::null(), Domino::null()],
            picked: DRAFT_SIZE,
        }

    }


    /// Checks if every domino has been picked
    pub(crate) fn is_empty(&self) -> bool {

        // Make sure we haven't somehow picked more dominoes than exist in the draft
        debug_assert!(self.picked <= DRAFT_SIZE);

        // Every domino in the draft has been picked.
        self.picked == DRAFT_SIZE

    }


    /// Picks the domino at the index from the draft
    pub(crate) fn pick(&mut self, idx: usize) -> Domino {
        // make sure we pick a valid index, and there are still dominos left to be picked
        debug_assert!(idx < DRAFT_SIZE && !self.is_empty());

        // make sure we haven't already picked the domino
        debug_assert!(self.draft[idx].is_selectable());

        self.picked += 1;

        self.draft[idx].select();

        self.draft[idx]
    }

    /// Iterates through the list of dominoes in the draft
    pub fn iter(&self) -> impl Iterator<Item = &Domino> {

        self.draft[..DRAFT_SIZE].iter()
    }



}