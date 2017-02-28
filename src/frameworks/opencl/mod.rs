//! OpenCL backend support for Parenchyma.
//!
//! ## Terminology
//!
//! Work-group: A collection of work items + has a unique work-group ID. work-groups are assigned
//! to execute on compute-units
//! Work-item: An instance of a kernel at run time + has a unique ID within the work-group
//!
//! ## Flow
//!
//! - Initialize the framework
//! - Select the desired platform
//! - Select the desired devices from the platform
//! - Create a context 
//! - create a command queue per device
//! - Compile programs
//!     - A program is essentially a collection of kernels.
//! - Create a kernel from the successfully compiled program
//!     - A kernel is the smallest unit of execution. Kernels are expensive to start so they're
//!     typically used to do a large amount of work.
//! - Specify arguments to the kernel
//! - Allocate memory on devices
//! - Transfer data to devices
//! - Execute
//! - Transfer results back
//! - Free memory on devices

pub mod cl;
pub mod error;
pub mod hl;

mod utility;