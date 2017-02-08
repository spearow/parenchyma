#![feature(pub_restricted, type_ascription)]

extern crate cuda_sys;

pub mod api;
pub mod error;
pub use self::api::{CudaAttribute, CudaDriver, CudaDeviceHandle};