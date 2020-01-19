use core::fmt::Debug;

pub trait Float: num_traits::Float + Debug {}

impl Float for f32 {}
impl Float for f64 {}