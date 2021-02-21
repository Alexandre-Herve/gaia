#[derive(
    Debug,
    Eq,
    Hash,
    PartialEq
)]
pub struct Plot {
    id: String,
    name: String,
}

impl Plot {
    pub fn new(id: String, name: String) -> Plot {
        Plot { id, name }
    }
}
