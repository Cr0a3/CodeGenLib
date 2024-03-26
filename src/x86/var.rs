use super::mem::AdressManager;

/// The Variabel struct is the handler for variables
#[derive(Clone)]
pub struct Variabel<'a> {
    pub size: u64,
    pub name: String,
    adrmng: &'a AdressManager,
}

impl<'a> Variabel<'a> {
    pub fn new(typ: VarDataType, name: &String, adrmng: &'a mut AdressManager) -> Self {
        let size: u64 = match typ {
            VarDataType::Custom(x) => x,
            VarDataType::I16 => 2,
            VarDataType::I32 => 4,
            VarDataType::I64 => 8,
            VarDataType::U16 => 2,
            VarDataType::U32 => 4,
            VarDataType::U64 => 8,
        };

        adrmng.reg_var(name, size);

        Self {
            size: size,
            name: name.to_string(),
            adrmng: adrmng,
        }
    }

    // Returns the adress of variable
    pub fn adr(&self) -> u64 {
        self.adrmng.get_adr(&self.name) as u64
    }
}

/// Size/type of the variabel
pub enum VarDataType {
    U16, I16,
    U32, I32,
    U64, I64,

    Custom(u64),
}