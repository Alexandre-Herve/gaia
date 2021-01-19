use super::{
    in_days::InDays,
    sequence::Sequence,
};

#[derive(Debug)]
pub struct YearlySequence<'a> {
    sequences: Vec<&'a InDays<'a, Sequence<'a>>>,
}

impl<'a> YearlySequence<'a> {
    pub fn new(sequences: Vec<&'a InDays<Sequence<'a>>>) -> YearlySequence<'a> {
        YearlySequence { sequences }
    }
}

