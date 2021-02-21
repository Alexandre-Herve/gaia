use super::{
    in_days::InDays,
    variety::Variety,
};

#[derive(Debug)]
pub struct Sequence<'a> {
    varieties: Vec<InDays<'a, Variety>>,
}

impl<'a> Sequence<'a> {
    pub fn new(
        varieties: Vec<InDays<'a, Variety>>
    ) -> Sequence {
        Sequence { varieties }
    }

    pub fn duration(&self) -> u32 {
        self.varieties
            .iter()
            .map(|i| i.0 + i.1.days_in_ground)
            .max()
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        Sequence::new(vec![]);
    }
}
