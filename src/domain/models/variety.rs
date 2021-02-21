#[derive(Debug)]
pub struct Variety {
    pub name: String,
    pub days_in_ground: u32,
}

impl Variety {
    pub fn new(
        name: String,
        days_in_ground: u32
    ) -> Variety {
        Variety {
            name,
            days_in_ground
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variety() {
        let v = Variety::new(String::from("Test"), 10);
        assert_eq!(v.days_in_ground, 10);
        assert_eq!(v.name, String::from("Test"));
    }
}
