//! Input method.

use std::{
    ffi::{c_void, CStr},
    os::raw::{c_int, c_uint},
};

use fcitx_sys::FcitxKeySym;

use crate::fcitx::FcitxInstance;

/// IM class instance.
pub(crate) struct Im {
    /// Fcitx instance.
    fcitx_instance: FcitxInstance,
}

impl Im {
    /// Creates a new IM instance.
    pub(crate) fn new(fcitx_instance: FcitxInstance) -> Self {
        trace!("Im::new(): Creating IM");
        let mut this = Self { fcitx_instance };
        trace!("Registering IM");
        this.register();
        trace!("Im::new(): Registered IM");
        this
    }

    /// Registers the IM.
    pub(crate) fn register(&mut self) {
        let priority = 1 as c_int;
        let interface = fcitx_sys::FcitxIMIFace {
            ResetIM: None,
            DoInput: Some(do_input),
            GetCandWords: None,
            PhraseTips: None,
            Save: None,
            Init: None,
            ReloadConfig: None,
            KeyBlocker: None,
            UpdateSurroundingText: None,
            DoReleaseInput: None,
            OnClose: None,
            GetSubModeName: None,
            padding: [std::ptr::null_mut(); 61],
        };
        let unique_name = CStr::from_bytes_with_nul(b"rskk\0").expect("Should never fail");
        let name = CStr::from_bytes_with_nul(b"rskk\0").expect("Should never fail");
        let icon_name = CStr::from_bytes_with_nul(b"rskk\0").expect("Should never fail");
        let lang_code = CStr::from_bytes_with_nul(b"ja\0").expect("Should never fail");

        trace!("Im::register(): Calling `fcitx_sys::FcitxInstanceRegisterIMv2`");
        unsafe {
            fcitx_sys::FcitxInstanceRegisterIMv2(
                self.fcitx_instance.raw_ptr(),
                self as *mut _ as *mut c_void,
                unique_name.as_ptr(),
                name.as_ptr(),
                icon_name.as_ptr(),
                interface,
                priority,
                lang_code.as_ptr(),
            )
        }
        trace!("Im::register(): Finished `fcitx_sys::FcitxInstanceRegisterIMv2`");
    }
}

/// Processes the key input.
unsafe extern "C" fn do_input(arg: *mut c_void, arg1: FcitxKeySym, arg2: c_uint) -> u32 {
    trace!(
        "do_input(): arg={:?}, arg1={:?}, arg2={:?}",
        arg,
        arg1,
        arg2
    );
    fcitx_sys::_INPUT_RETURN_VALUE_IRV_TO_PROCESS
}
