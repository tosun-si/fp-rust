pub struct FluentDecorator<'a, T, R> {
    object: T,
    function: Box<dyn Fn(T) -> R + 'a>,
}

impl<'a, T: 'a, R: 'a> FluentDecorator<'a, T, R> {
    pub fn from(object: T, default_value: impl Fn(T) -> R + 'a) -> Self {
        Self {
            object,
            function: Box::new(default_value),
        }
    }

    pub fn with<N: 'a>(self,
                       function_to_compose: impl Fn(R) -> N + 'a) -> FluentDecorator<'a, T, N> {
        FluentDecorator {
            object: self.object,
            function: Box::new(move |el| function_to_compose((self.function)(el))),
        }
    }

    pub fn calculate(self) -> R {
        (self.function)(self.object)
    }
}