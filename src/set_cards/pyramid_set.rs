
pub type PyramidSet = u64;
pub mod tools;
pub mod views;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct  PyrSet{
    pub pyr_bin: PyramidSet,
}
