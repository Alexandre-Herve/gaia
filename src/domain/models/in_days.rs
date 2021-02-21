#[derive(Debug)]
pub struct InDays<'a, T>(pub u32, pub &'a T);

impl<'a, T> InDays<'a, T> {
    pub fn new(days: u32, data: &T) -> InDays<T> {
        InDays(days, data)
    }

    pub fn in_days(&self) -> u32 {
        self.0
    }

    pub fn value(&self) -> &T {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days() {
        let d = InDays(1, &2);
        assert_eq!(d.in_days(), 1)
    }

    #[test]
    fn value() {
        let d = InDays(1, &2);
        assert_eq!(d.value(), &2)
    }
}
