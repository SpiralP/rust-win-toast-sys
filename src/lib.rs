use libc::wchar_t;
use std::{ffi::OsString, os::windows::ffi::OsStrExt};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    unsafe {
      let app_name = wide("hello world");
      let app_user_model_id = wide("hello");

      assert!(initialize(app_name.as_ptr(), app_user_model_id.as_ptr()));

      let text = wide("text!");
      let sub = wide("bottom text!");

      assert!(notify(text.as_ptr(), sub.as_ptr()));

      std::thread::sleep(std::time::Duration::from_millis(1000));
    }
  }
}

fn wide<T: Into<String>>(s: T) -> Vec<u16> {
  let s = s.into();
  let os_string = OsString::from(s);
  let mut wide: Vec<_> = os_string.encode_wide().collect();
  wide.push(0);

  wide
}
