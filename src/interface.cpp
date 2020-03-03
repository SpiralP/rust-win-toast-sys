

#include <wchar.h>

#include <string>

#include "../WinToast/src/wintoastlib.h"

using namespace WinToastLib;
using namespace std;

inline const wchar_t* clone_string(wstring& str) {
  size_t length = str.length();

  wchar_t* c_str = new wchar_t[length + 1];
  wcscpy(c_str, str.c_str());

  return c_str;
}

extern "C" void WinToast_string_delete(const wchar_t* c_str) {
  delete[] c_str;
}

// ---------------------- WinToast

/// Call `WinToast_delete` when done with the object.
extern "C" void* WinToast_new() {
  WinToast* winToast = new WinToast();

  return winToast;
}

extern "C" void WinToast_delete(void* winToastPtr) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);
  delete winToast;
}

extern "C" void* WinToast_instance() {
  return WinToast::instance();
}

extern "C" WinToast::WinToastError WinToast_initialize(void* winToastPtr) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);

  WinToast::WinToastError error;
  winToast->initialize(&error);
  return error;
}

extern "C" void WinToast_setAppName(void* winToastPtr, const wchar_t* appName) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);

  return winToast->setAppName(std::wstring(appName));
}

/// Call `WinToast_string_delete` when done with the string.
extern "C" const wchar_t* WinToast_configureAUMI(
    void* winToastPtr,
    const wchar_t* companyName,
    const wchar_t* productName,
    const wchar_t* subProduct,
    const wchar_t* versionInformation) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);

  wstring str =
      winToast->configureAUMI(wstring(companyName), wstring(productName),
                              wstring(subProduct), wstring(versionInformation));

  return clone_string(str);
}

extern "C" void WinToast_setAppUserModelId(void* winToastPtr,
                                           const wchar_t* appUserModelID) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);

  return winToast->setAppUserModelId(std::wstring(appUserModelID));
}

struct WinToast_showToast_return {
  INT64 id;
  WinToast::WinToastError error;
};

extern "C" WinToast_showToast_return WinToast_showToast(
    void* winToastPtr,
    void* winToastTemplatePtr,
    void* customHandler) {
  WinToast* winToast = static_cast<WinToast*>(winToastPtr);

  WinToastTemplate* winToastTemplate =
      static_cast<WinToastTemplate*>(winToastTemplatePtr);
  IWinToastHandler* handler = static_cast<IWinToastHandler*>(customHandler);

  WinToast::WinToastError error;
  auto id = winToast->showToast(*winToastTemplate, handler, &error);

  return {id, error};
}

// ---------------------- WinToastTemplate

/// Call `WinToastTemplate_delete` when done with the template.
extern "C" void* WinToastTemplate_new(
    WinToastTemplate::WinToastTemplateType templateType) {
  WinToastTemplate* winToastTemplate = new WinToastTemplate(templateType);

  return winToastTemplate;
}

extern "C" void WinToastTemplate_delete(void* winToastTemplatePtr) {
  WinToastTemplate* winToastTemplate =
      static_cast<WinToastTemplate*>(winToastTemplatePtr);
  delete winToastTemplate;
}

extern "C" void WinToastTemplate_setTextField(void* winToastTemplatePtr,
                                              const wchar_t* txt,
                                              WinToastTemplate::TextField pos) {
  WinToastTemplate* winToastTemplate =
      static_cast<WinToastTemplate*>(winToastTemplatePtr);
  return winToastTemplate->setTextField(txt, pos);
}

extern "C" void WinToastTemplate_setAttributionText(
    void* winToastTemplatePtr,
    const wchar_t* attributionText) {
  WinToastTemplate* winToastTemplate =
      static_cast<WinToastTemplate*>(winToastTemplatePtr);
  return winToastTemplate->setAttributionText(attributionText);
}

// ---------------------- WinToastHandler

typedef void (*HandlerToastActivatedCallback)(int actionIndex);
typedef void (*HandlerToastDismissedCallback)(
    IWinToastHandler::WinToastDismissalReason state);
typedef void (*HandlerToastFailedCallback)();

class CustomHandler : public IWinToastHandler {
 private:
  HandlerToastActivatedCallback toastActivatedCallback;
  HandlerToastDismissedCallback toastDismissedCallback;
  HandlerToastFailedCallback toastFailedCallback;

 public:
  CustomHandler(HandlerToastActivatedCallback toastActivatedCallback,
                HandlerToastDismissedCallback toastDismissedCallback,
                HandlerToastFailedCallback toastFailedCallback) {
    this->toastActivatedCallback = toastActivatedCallback;
    this->toastDismissedCallback = toastDismissedCallback;
    this->toastFailedCallback = toastFailedCallback;
  }

  void toastActivated() const { this->toastActivatedCallback(-1); }
  void toastActivated(int actionIndex) const {
    this->toastActivatedCallback(actionIndex);
  }
  void toastDismissed(WinToastDismissalReason state) const {
    this->toastDismissedCallback(state);
  }
  void toastFailed() const { this->toastFailedCallback(); }
};

/// Call `WinToastHandler_delete` when done with the handler.
extern "C" void* WinToastHandler_new(
    HandlerToastActivatedCallback toastActivatedCallback,
    HandlerToastDismissedCallback toastDismissedCallback,
    HandlerToastFailedCallback toastFailedCallback) {
  CustomHandler* customHandler = new CustomHandler(
      toastActivatedCallback, toastDismissedCallback, toastFailedCallback);

  return customHandler;
}

extern "C" void WinToastHandler_delete(void* handler) {
  CustomHandler* customHandler = static_cast<CustomHandler*>(handler);
  delete customHandler;
}
