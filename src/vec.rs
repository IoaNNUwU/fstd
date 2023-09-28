pub use std::collections::TryReserveError;

pub struct Vec<T>(std::vec::Vec<T>);

impl<T> Vec<T> {
    
    pub const fn new() -> Self {
        Self(std::vec::Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(std::vec::Vec::with_capacity(capacity))
    }

    pub fn capacity(self) -> (usize, Self) {
        (self.0.capacity(), self)
    }

    pub fn reserve(mut self, additional: usize) -> Self {
        self.0.reserve(additional);
        self
    }

    pub fn reserve_exact(mut self, additional: usize) -> Self {
        self.0.reserve_exact(additional);
        self
    }

    pub fn try_reserve(mut self, additional: usize) -> Result<Self, (TryReserveError, Self)> {
        match self.0.try_reserve(additional) {
            Ok(_) => Ok(self),
            Err(err) => Err((err, self)),
        }
    }

    pub fn shrink_to_fit(mut self) -> Self {
        self.0.shrink_to_fit();
        self
    }

    pub fn shrink_to(mut self, min_capacity: usize) -> Self {
        self.0.shrink_to(min_capacity);
        self
    }
}

