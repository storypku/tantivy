use super::PhraseWeight;
use Result;
use core::searcher::Searcher;
use query::Query;
use query::Weight;
use schema::Term;
use std::any::Any;

/// `PhraseQuery` matches a specific sequence of words.
///
/// For instance the phrase query for `"part time"` will match
/// the sentence
///
/// **Alan just got a part time job.**
///
/// On the other hand it will not match the sentence.
///
/// **This is my favorite part of the job.**
///
/// Using a `PhraseQuery` on a field requires positions
/// to be indexed for this field.
/// 
#[derive(Debug)]
pub struct PhraseQuery {
    phrase_terms: Vec<Term>,
}

impl Query for PhraseQuery {
    /// Used to make it possible to cast Box<Query>
    /// into a specific type. This is mostly useful for unit tests.
    fn as_any(&self) -> &Any {
        self
    }

    /// Create the weight associated to a query.
    ///
    /// See [`Weight`](./trait.Weight.html).
    fn weight(&self, _searcher: &Searcher) -> Result<Box<Weight>> {
        Ok(box PhraseWeight::from(self.phrase_terms.clone()))
    }
}

impl From<Vec<Term>> for PhraseQuery {
    fn from(phrase_terms: Vec<Term>) -> PhraseQuery {
        assert!(phrase_terms.len() > 1);
        PhraseQuery { phrase_terms }
    }
}
