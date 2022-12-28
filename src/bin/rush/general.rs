
pub mod general {

  extern crate libc;
  use std::fs::File;
  use std::os::unix::fs::OpenOptionsExt;

  pub fn check_dev_tty() {
    let mut _tty = File::options().read(true).write(true).custom_flags(libc::O_NONBLOCK).open("/dev/tty");
  }
}
