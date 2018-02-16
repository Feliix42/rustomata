mod automaton;
use dyck::multiple::automaton::MultipleDyckAutomaton;
use util::partition::Partition;
pub use dyck::Bracket;
use recognisable::automaton::recognise;
use Configuration;
use tree_stack_automaton::TreeStack;
use self::automaton::MDTreeElem;

/// An object that represents the mutliple Dyck language of an alphabet Σ with respect to
/// a partition of Σ.
pub struct MultipleDyckLanguage<T: Ord + Clone>(MultipleDyckAutomaton<T>);

impl<'a, T: Clone + Eq + Ord> MultipleDyckLanguage<T> {
    /// Represents a multiple Dyck language with respect to
    /// a partition Π = {π₁, …, πₙ} of an implicit alphabet Σ = π₁ ∪ … ∪ πₙ.
    pub fn new(p: &Partition<T>) -> Self {
        MultipleDyckLanguage(MultipleDyckAutomaton::new(p))
    }

    /// Unweightedly recognizes an element w ∈ Σ* of a multiple Dyck language with respect to
    /// Σ and a partition of Σ.
    pub fn recognize(&self, word: &[Bracket<T>]) -> Option<Configuration<TreeStack<MDTreeElem<T>>, Bracket<T>, u8>> {
        let &MultipleDyckLanguage(ref mda) = self;
        let word_ = word.to_owned();
        let mut b = recognise(mda, word_);
        b.next().map(|item| item.0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn mutliple_dyck_language() {
        use super::Bracket::*;
        use super::MultipleDyckLanguage;
        use util::partition::Partition;

        let words = vec![
            vec![Open(1), Close(1), Open(2), Close(2)],
            vec![
                Open(1),
                Open(2),
                Close(2),
                Open(1),
                Close(1),
                Close(1),
                Open(2),
                Close(2),
            ],
            vec![
                Open(1),
                Open(2),
                Close(2),
                Close(1),
                Open(2),
                Open(1),
                Close(1),
                Close(2),
            ],
            vec![
                Open(1),
                Open(3),
                Close(3),
                Close(1),
                Open(2),
                Open(4),
                Close(4),
                Close(2),
            ],
        ];

        let partition = Partition::new(vec![
            vec![1, 2].into_iter().collect(),
            vec![3, 4].into_iter().collect(),
        ]).unwrap();

        let mdl = MultipleDyckLanguage::new(&partition);

        for dyckword in words {
            assert!(mdl.recognize(&dyckword).is_some());
        }

        let not_words = vec![
            vec![Open(1), Close(2), Open(2), Close(1)],
            vec![
                Open(1),
                Open(4),
                Close(4),
                Close(1),
                Open(3),
                Open(2),
                Close(2),
                Close(3),
            ],
        ];

        eprintln!("{:?}", mdl.recognize(&not_words[0]));
        eprintln!("{:?}", mdl.recognize(&not_words[1]));
        // assert!(!mdl.recognize(&not_words[0]));
        // assert!(!mdl.recognize(&not_words[1]));
        // for not_dyckword in not_words {
        //     assert!(!mdl.recognize(&not_dyckword));
        // }
    }
}
