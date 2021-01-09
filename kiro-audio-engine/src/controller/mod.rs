use crate::param_value::ParamValue;

pub mod controller;
pub(crate) mod owned_data;

pub type ProcParams = Vec<ParamValue>;

pub use controller::Controller;
