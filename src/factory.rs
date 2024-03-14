use std::collections::HashMap;
use std::hash::Hash;

// #[derive(Copy)]
pub struct Factory<'a, T, R> {
    suppliers: HashMap<&'a T, Box<dyn Fn() -> R>>,
    default_value: Box<dyn Fn() -> R>,
}

impl<'a, T: Eq + Hash, R> Factory<'a, T, R>
{
    pub fn from_type(default_value: impl Fn() -> R + 'static) -> Self {
        Self {
            suppliers: HashMap::new(),
            default_value: Box::new(default_value),
        }
    }

    pub fn register(mut self,
                    current_type: &'a T,
                    supplier: impl Fn() -> R + 'static) -> Self {
        self.suppliers.insert(current_type, Box::new(supplier));

        self
    }


    pub fn get(&self,
               input_type: &'a T) -> R {
        self.suppliers.get(input_type).unwrap_or(&self.default_value)()
    }
}