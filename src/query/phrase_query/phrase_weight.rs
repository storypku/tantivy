use super::PhraseScorer;
use Result;
use core::SegmentReader;
use postings::IntersectionDocSet;
use query::EmptyScorer;
use query::Scorer;
use query::Weight;
use schema::IndexRecordOption;
use schema::Term;

pub struct PhraseWeight {
    phrase_terms: Vec<Term>,
}

impl From<Vec<Term>> for PhraseWeight {
    fn from(phrase_terms: Vec<Term>) -> PhraseWeight {
        PhraseWeight { phrase_terms }
    }
}

impl Weight for PhraseWeight {
    fn scorer<'a>(&'a self, reader: &'a SegmentReader) -> Result<Box<Scorer + 'a>> {
        let mut term_postings_list = Vec::new();
        for term in &self.phrase_terms {
            let inverted_index = reader.inverted_index(term.field());
            let term_postings_option =
                inverted_index.read_postings(term, IndexRecordOption::WithFreqsAndPositions);
            if let Some(term_postings) = term_postings_option {
                term_postings_list.push(term_postings);
            } else {
                return Ok(box EmptyScorer);
            }
        }
        Ok(box PhraseScorer {
            intersection_docset: IntersectionDocSet::from(term_postings_list),
        })
    }
}
