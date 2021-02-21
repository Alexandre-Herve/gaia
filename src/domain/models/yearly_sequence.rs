use super::{
    in_days::InDays,
    sequence::Sequence,
};

pub type Sequences<'a> = Vec<InDays<'a, Sequence<'a>>>;

#[derive(Debug)]
pub struct YearlySequence<'a> {
    sequences: Sequences<'a>,
}

impl<'a> YearlySequence<'a> {
    pub fn new(
        sequences: Sequences<'a>
    ) -> Option<YearlySequence<'a>> {
        let y_seq = YearlySequence { sequences };
        if !y_seq.is_valid() {
            return None;
        }
        Some(y_seq)
    }

    fn is_valid(&self) -> bool {
        let invalid = self.sequences
            .iter()
            .enumerate()
            .any(|(i, in_days_seq)| {
                if i == 0 { return false }
                let previous_shift = self.sequences[i - 1].in_days();
                let previous_duration = self.sequences[i - 1].value().duration();
                let current_shift = in_days_seq.in_days();
                // No overlap allowed
                previous_shift + previous_duration > current_shift
            });
        !invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::variety::Variety;

    #[test]
    fn is_valid() {
        let chou = Variety::new(
            String::from("Chou"),
            10
        );
        let sequence = Sequence::new(vec![
            InDays(0, &chou)
        ]);
        let yearly_seq = YearlySequence::new(vec![
            InDays(0, &sequence),
            InDays(11, &sequence)
        ]);
        assert_eq!(yearly_seq.unwrap().is_valid(), true);
    }

    #[test]
    fn is_invalid() {
        let chou = Variety::new(
            String::from("Chou"),
            10
        );
        let sequence = Sequence::new(vec![
            InDays(0, &chou)
        ]);
        let sequences = vec![
            InDays(0, &sequence),
            InDays(1, &sequence)
        ];
        let yearly_seq = YearlySequence::new(sequences);
        if let Some(_) = yearly_seq {
            panic!(
                "YearlySequence::new() with invalid sequences should have returned None"
            );
        }
    }
}
