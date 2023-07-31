// pub mod hashpool;
pub mod traits;

#[cfg(test)]
mod test;

pub mod prelude {
    // pub use crate::hashpool;
    pub use crate::traits::*;
}
