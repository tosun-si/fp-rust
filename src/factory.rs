use std::collections::HashMap;
use std::hash::Hash;

pub struct Factory<'a, T, R> {
    input_type: &'a T,
    suppliers: HashMap<&'a T, Box<dyn Fn() -> R>>,
}

impl<'a, T: Eq + Hash, R> Factory<'a, T, R>
{
    pub fn from_type(input_type: &'a T) -> Self {
        Self {
            input_type,
            suppliers: HashMap::new(),
        }
    }

    pub fn register(mut self,
                    current_type: &'a T,
                    supplier: impl Fn() -> R + 'static) -> Self {
        self.suppliers.insert(current_type, Box::new(supplier));

        self
    }


    pub fn get(self) -> R {
        self.suppliers.get(self.input_type).expect("No case found in the factory pattern")()
    }
}