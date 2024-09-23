use std::error;

use eyre::{Ok, Result};

#[test]
fn converting_options_to_results() -> Result<()> {
  let something = maybe_something().ok_or(CustomError::SomethingWasNothing)?;
  Ok(())
}

fn maybe_something() -> Option<u32> {
  None
}

#[derive(Debug, thiserror::Error)]
enum CustomError {
  #[error("Something wasn't really something, it turned out to be nothing")]
  SomethingWasNothing
}