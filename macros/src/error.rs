use syn::Error;

pub trait CollectError {
    fn push_error(&mut self, error: Error);
}

impl CollectError for Option<Error> {
    fn push_error(&mut self, error: Error) {
        match self {
            Some(e) => e.combine(error),
            None => *self = Some(error),
        }
    }
}
