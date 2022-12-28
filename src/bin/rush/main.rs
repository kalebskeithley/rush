
mod general;
use crate::general::general::*;

use std::env;

fn main(){

  let _argc=env::Args::len(&env::args());

  loop {
    let top_level = || -> Result<(), TopErrors> {
      do_top_init()?;
      Ok(())
    };
    if let Err(_err) = top_level() {
      println!("Caught top error");
    }
  }
}

enum TopErrors {
  TopError,
}

fn do_top_init() -> Result<(), TopErrors>{
  let _err=TopErrors::TopError;
  println!("top_init");
  check_dev_tty();
  Ok(())
}
