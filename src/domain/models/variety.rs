#[derive(Debug)]
pub struct Variety {
    name: String,
}

impl Variety {
    pub fn new(name: String) -> Variety {
        Variety {
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variety() {
        let v = Variety::new(String::from("Test"));
        assert_eq!(true, true)
    }
}
