use win_toast_sys::*;

#[test]
fn is_compatible() {
  unsafe {
    assert!(WinToast_isCompatible());
  }
}

/// https://github.com/mohabouje/WinToast#example-of-usage
#[test]
fn it_works() {
  use lazy_static::lazy_static;
  use std::{
    sync::{Condvar, Mutex},
    time::Duration,
  };

  lazy_static! {
    static ref PAIR: (Mutex<()>, Condvar) = (Mutex::new(()), Condvar::new());
  }

  fn wake() {
    let (lock, cvar) = &*PAIR;
    let _guard = lock.lock().unwrap();
    cvar.notify_all();
  }

  fn wait() {
    let (lock, cvar) = &*PAIR;
    let guard = lock.lock().unwrap();
    cvar.notify_all();

    // 30 seconds probably long enough for only buggy toasts
    let _guard = cvar.wait_timeout(guard, Duration::from_secs(30)).unwrap();
  }

  unsafe {
    if !WinToast_isCompatible() {
      panic!("Error, your system in not supported!");
    }

    let win_toast = WinToast_instance();

    let company_name = wide("Test");
    let product_name = wide("Hello World");
    let blank = wide("");

    WinToast_setAppName(win_toast, product_name.as_ptr());
    let aumi = WinToast_configureAUMI(
      win_toast,
      company_name.as_ptr(),
      product_name.as_ptr(),
      blank.as_ptr(),
      blank.as_ptr(),
    );
    WinToast_setAppUserModelId(win_toast, aumi);
    WinToast_string_delete(aumi);

    if WinToast_initialize(win_toast) != WinToast_WinToastError::NoError {
      panic!("Error, could not initialize the lib!");
    }

    extern "C" fn activated(action_index: i32) {
      println!("activated {:?}", action_index);

      wake();
    }

    extern "C" fn dismissed(state: IWinToastHandler_WinToastDismissalReason) {
      println!("dismissed {:?}", state);

      wake();
    }

    extern "C" fn failed() {
      println!("failed");

      wake();
    }

    let handler = WinToastHandler_new(Some(activated), Some(dismissed), Some(failed));
    let template = WinToastTemplate_new(WinToastTemplate_WinToastTemplateType::Text01);

    let title = wide("TITLE");
    WinToastTemplate_setTextField(
      template,
      title.as_ptr(),
      WinToastTemplate_TextField::FirstLine,
    );

    WinToastTemplate_setActivationType(template, WinToastTemplate_ActivationType::Protocol);

    let launch = wide("ms-settings:windowsupdate");
    WinToastTemplate_setLaunch(template, launch.as_ptr());

    let ret = WinToast_showToast(win_toast, template, handler);

    if ret.error != WinToast_WinToastError::NoError {
      WinToastTemplate_delete(template);
      WinToastHandler_delete(handler);

      panic!("Error: Could not launch your toast notification!");
    } else {
      println!("Showing notification with id {}", ret.id);
    }

    wait();

    WinToastTemplate_delete(template);
    WinToastHandler_delete(handler);
  }
}

fn wide<T: Into<String>>(s: T) -> Vec<u16> {
  use std::{ffi::OsString, os::windows::ffi::OsStrExt};

  let s = s.into();
  let os_string = OsString::from(s);
  let mut wide: Vec<_> = os_string.encode_wide().collect();
  wide.push(0);

  wide
}
