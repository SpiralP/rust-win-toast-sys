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
  use crossbeam_channel::{bounded, Receiver, Sender};
  use lazy_static::lazy_static;
  lazy_static! {
    static ref WAIT: (Sender<()>, Receiver<()>) = bounded(1);
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

    std::thread::spawn(move || {
      std::thread::sleep(std::time::Duration::from_secs(30));
      let _ = WAIT.0.send(());
    });

    extern "C" fn activated(action_index: i32) {
      println!("activated {:?}", action_index);
      let _ = WAIT.0.send(());
    }

    extern "C" fn dismissed(state: IWinToastHandler_WinToastDismissalReason) {
      println!("dismissed {:?}", state);
      let _ = WAIT.0.send(());
    }

    extern "C" fn failed() {
      println!("failed");
      let _ = WAIT.0.send(());
    }

    let handler = WinToastHandler_new(Some(activated), Some(dismissed), Some(failed));
    let template = WinToastTemplate_new(WinToastTemplate_WinToastTemplateType::Text01);

    let title = wide("TITLE");
    WinToastTemplate_setTextField(
      template,
      title.as_ptr(),
      WinToastTemplate_TextField::FirstLine,
    );

    let ret = WinToast_showToast(win_toast, template, handler);

    if ret.error != WinToast_WinToastError::NoError {
      WinToastTemplate_delete(template);
      WinToastHandler_delete(handler);

      panic!("Error: Could not launch your toast notification!");
    } else {
      println!("Showing notification with id {}", ret.id);
    }

    WAIT.1.recv().unwrap();

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
