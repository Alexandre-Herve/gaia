#[derive(Debug)]
pub struct InDays<'a, T>(u32, &'a T);

impl<'a, T> InDays<'a, T> {
    pub fn new(days: u32, data: &T) -> InDays<T> {
        InDays(days, data)
    }

    pub fn days(&self) -> u32 {
        self.0
    }

    pub fn value(&self) -> &T {
        self.1
    }
}
