use crate::deck::Deck;
use crate::domino::Domino;

pub(crate) struct Draft {

    draft: [Domino; 4],
    len: usize,
}

impl Draft {


    pub(crate) fn new(unsorted_draft: [Domino; 4], len: usize) -> Draft {
        let mut draft = unsorted_draft;
        draft.sort();

        Draft {
            draft,
            len
        }
    }
    pub(crate) fn is_empty(&self) -> bool {

        self.len == 0

    }


    /// Picks the domino at the index from the draft
    pub(crate) fn pick(&mut self, idx: usize) -> Domino {
        // make sure we pick a valid index
        debug_assert!(idx < self.len);

        self.len -= 1;
        self.draft.swap(idx, self.len);

        let picked = self.draft[idx];
        self.draft[idx] = Domino::null();

        picked
    }
}