/// Pegasus provides an improved network that guarantees
/// reliable in-order delivery of messages.  It manages
/// liveness at the link level and provides a form of
/// common knowledge across the link.
///
/// Example of a doc test
/// ```
/// let foo = 5;
/// assert_eq!(foo, 5);
/// ```
use thiserror::Error;
pub fn foo() -> Result<(), FooError> {
    bar().map_err(|e| FooError::BarError(e))?;
    Err(FooError::SimpleError.into())
}
fn bar() -> Result<(), BarError> {
    Err(BarError::SimpleError)
}
#[derive(Debug, Error)]
pub enum FooError {
    #[error("Foo error")]
    SimpleError,
    #[error(transparent)]
    BarError(BarError),
}
#[derive(Debug, Error)]
pub enum BarError {
    #[error("Bar error")]
    SimpleError,
}
