use uuid::Uuid;

/// Stores information about a single medication.
#[derive(Debug)]
pub struct Med {
    name: String,
    id: Uuid,
}

impl Med {
    /// Constructs a new medication with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
        }
    }
}
