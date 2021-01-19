#[derive(Debug, Hash)]
pub struct Plot {
    id: String,
    name: String,
}

impl Plot {
    pub fn new(id: String, name: String) -> Plot {
        Plot { id, name }
    }
}

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Plot {}
