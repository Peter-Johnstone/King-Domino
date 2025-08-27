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
}