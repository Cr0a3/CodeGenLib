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

    pub fn set(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    pub fn add(&mut self, value: Vec<u8>) {
        for b in value {
            self.value.push(b);
        }
    }
}