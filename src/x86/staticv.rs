/// The StaticValue struct is the handler for static values (consts)
#[derive(Clone)]
pub struct StaticValue {
    pub name: String,
    pub value: Vec<u8>,
    pub global: bool,
}

impl StaticValue {
    pub fn new(name: &str, global: bool) -> Self {
        Self {
            name: name.into(),
            value: vec![],
            global: global,
        }
    }

    /// Sets the constant
    pub fn set(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    /// Adds to the constant
    pub fn add(&mut self, value: Vec<u8>) {
        for b in value {
            self.value.push(b);
        }
    }

    /// Clears the constant
    pub fn clear(&mut self) {
        self.value = vec![];
    }
}