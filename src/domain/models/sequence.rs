use super::{
    in_days::InDays,
    variety::Variety,
};

#[derive(Debug)]
pub struct Sequence<'a> {
    varieties: Vec<&'a InDays<'a, Variety>>,
}

impl<'a> Sequence<'a> {
    pub fn new(varieties: Vec<&'a InDays<'a, Variety>>) -> Sequence {
        Sequence { varieties }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        let v = Sequence::new(vec![]);
        assert_eq!(v, v)
    }
}
