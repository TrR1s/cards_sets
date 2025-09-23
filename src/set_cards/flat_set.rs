pub type FlatSet64 = u64;
pub mod tools;
pub mod views;
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct  FlatSet{
    pub flat_bin: FlatSet64,
}