pub struct Validator<'a, T> {
    object: &'a T,
    error_messages: Vec<&'a str>,
}

impl<'a, T> Validator<'a, T>
{
    pub fn of(object: &'a T) -> Self {
        Self {
            object,
            error_messages: vec![],
        }
    }

    pub fn validate<R>(mut self,
                       projection: impl Fn(&T) -> &R,
                       predicate: impl Fn(&R) -> bool,
                       error_message: &'a str) -> Self {
        let result_function = projection(&self.object);

        if !predicate(result_function) {
            self.error_messages.push(error_message)
        }

        self
    }

    pub fn get_error_messages(self) -> Vec<&'a str> {
        self.error_messages
    }
}