use std::ops::Deref;

pub mod string;
pub mod vec;
pub mod fmt;

pub trait Itself<T> {
    fn immutable(self) -> Immutable<T>;
}

impl<T> Itself<T> for T {
    fn immutable(self) -> Immutable<T> {
        Immutable(self)
    }
}

pub struct Immutable<T>(T);

impl<T> Deref for Immutable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}