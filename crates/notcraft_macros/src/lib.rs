#[cfg(feature = "derive")]
pub use notcraft_derive::MyDeriveTrait;

pub trait MyDeriveTrait {
    fn name(&self);
}
