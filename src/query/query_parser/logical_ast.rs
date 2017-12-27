use query::Occur;
use schema::Term;
use std::fmt;

#[derive(Clone)]
pub enum LogicalLiteral {
    Term(Term),
    Phrase(Vec<Term>),
}

#[derive(Clone)]
pub enum LogicalAST {
    Clause(Vec<(Occur, LogicalAST)>),
    Leaf(Box<LogicalLiteral>),
}

fn occur_letter(occur: Occur) -> &'static str {
    match occur {
        Occur::Must => "+",
        Occur::MustNot => "-",
        Occur::Should => "",
    }
}

impl fmt::Debug for LogicalAST {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            LogicalAST::Clause(ref clause) => {
                if clause.is_empty() {
                    write!(formatter, "<emptyclause>")?;
                } else {
                    let (ref occur, ref subquery) = clause[0];
                    write!(formatter, "({}{:?}", occur_letter(*occur), subquery)?;
                    for &(ref occur, ref subquery) in &clause[1..] {
                        write!(formatter, " {}{:?}", occur_letter(*occur), subquery)?;
                    }
                    formatter.write_str(")")?;
                }
                Ok(())
            }
            LogicalAST::Leaf(ref literal) => write!(formatter, "{:?}", literal),
        }
    }
}

impl From<LogicalLiteral> for LogicalAST {
    fn from(literal: LogicalLiteral) -> LogicalAST {
        LogicalAST::Leaf(box literal)
    }
}

impl fmt::Debug for LogicalLiteral {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            LogicalLiteral::Term(ref term) => write!(formatter, "{:?}", term),
            LogicalLiteral::Phrase(ref terms) => write!(formatter, "\"{:?}\"", terms),
        }
    }
}
