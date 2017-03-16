use {BoxContext, Context, Error, ExtensionPackage, Framework, Hardware, HardwareKind, Result};
use std::marker::{PhantomData, Unsize};
use super::NativeContext;
use utility::TryDefault;

const NATIVE: &'static str = "Native";

/// The native framework
///
/// # Example
///
/// ```rust
/// use parenchyma::{Backend, Native, SharedTensor};
///
/// let ref host: Backend = Backend::new::<Native>().unwrap();
///
/// let sh: SharedTensor = SharedTensor::with(host, [2, 2], vec![1., 2., 3., 4.]).unwrap();
///
/// let tensor = sh.read(host).unwrap();
///
/// println!("{:#?}", tensor);
/// ```
#[derive(Debug)]
pub struct Native;

impl Framework for Native {

    const FRAMEWORK_NAME: &'static str = NATIVE;

    fn available_hardware(&self) -> Vec<Hardware> {
        vec![Hardware {
            id: 0,
            framework: NATIVE,
            kind: HardwareKind::Central,
            name: String::from("HOST CPU"),
            compute_units: 1,
        }]
    }
}

impl<X> BoxContext<X> for Native 
    where X: ExtensionPackage, 
          NativeContext<X>: Unsize<X::Extension> 
          {

    fn enclose(&self, _: Vec<Hardware>) -> Result<Box<Context<Package = X>>> {
        Ok(Box::new(NativeContext(PhantomData)))
    }
}

impl TryDefault for Native {
    type Err = Error;

    fn try_default() -> Result<Native> {
        Ok(Native)
    }
}