#include "interface.hpp"

#include <string>

#include "../WinToast/src/wintoastlib.h"

using namespace WinToastLib;

class CustomHandler : public IWinToastHandler {
 public:
  void toastActivated() const {
    std::wcout << L"The user clicked in this toast" << std::endl;
    exit(0);
  }

  void toastActivated(int actionIndex) const {
    std::wcout << L"The user clicked on action #" << actionIndex << std::endl;
    exit(16 + actionIndex);
  }

  void toastDismissed(WinToastDismissalReason state) const {
    switch (state) {
      case UserCanceled:
        std::wcout << L"The user dismissed this toast" << std::endl;
        exit(1);
        break;
      case TimedOut:
        std::wcout << L"The toast has timed out" << std::endl;
        exit(2);
        break;
      case ApplicationHidden:
        std::wcout
            << L"The application hid the toast using ToastNotifier.hide()"
            << std::endl;
        exit(3);
        break;
      default:
        std::wcout << L"Toast not activated" << std::endl;
        exit(4);
        break;
    }
  }

  void toastFailed() const {
    std::wcout << L"Error showing current toast" << std::endl;
    exit(5);
  }
};

extern "C" bool initialize(const wchar_t* appName,
                           const wchar_t* appUserModelID) {
  if (!WinToast::isCompatible()) {
    std::wcerr << L"Error, your system in not supported!" << std::endl;
    return false;
  }

  WinToast::instance()->setAppName(std::wstring(appName));
  WinToast::instance()->setAppUserModelId(std::wstring(appUserModelID));

  if (!WinToast::instance()->initialize()) {
    std::wcerr << L"Error, your system in not compatible!" << std::endl;
    return false;
  }

  return true;
}

extern "C" bool notify(const wchar_t* text, const wchar_t* attribute) {
  WinToastTemplate templ(WinToastTemplate::Text02);
  templ.setTextField(text, WinToastTemplate::FirstLine);
  templ.setAttributionText(attribute);

  if (WinToast::instance()->showToast(templ, new CustomHandler()) < 0) {
    std::wcerr << L"Could not launch your toast notification!";
    return false;
  }

  return true;
}
