//! Fcitx rskk.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

// FIXME: Use logging features of fcitx, instead of directly writing to stderr.
#[cfg(debug_assertions)]
macro_rules! trace {
    () => {
        eprintln!("[{} @{}:{}:{}]", module_path!(), file!(), line!(), column!())
    };
    ($($rest:tt)*) => {
        eprintln!(
            "[{} @{}:{}:{}] {}",
            module_path!(),
            file!(),
            line!(),
            column!(),
            format_args!($($rest)*)
        )
    };
}

#[cfg(not(debug_assertions))]
macro_rules! trace {
    ($($rest:tt)*) => {};
}

use fcitx_sys;

use self::{fcitx::FcitxInstance, im::Im};

mod fcitx;
mod im;

/// Fcitx ABI version.
#[no_mangle]
pub static ABI_VERSION: u32 = fcitx::ABI_VERSION;

/// IME specification.
#[no_mangle]
pub static ime2: fcitx_sys::FcitxIMClass2 = fcitx_sys::FcitxIMClass2 {
    Create: Some(ime_create),
    Destroy: Some(ime_destroy),
    ReloadConfig: None,
    padding1: None,
    padding2: None,
    padding3: None,
    padding4: None,
    padding5: None,
};

/// Creates a new IM instance from the given raw fcitx instance.
// This is called by fcitx.
unsafe extern "C" fn ime_create(
    fcitx_instance: *mut fcitx_sys::FcitxInstance,
) -> *mut std::ffi::c_void {
    trace!("ime_create(): Creating IM");
    let instance = Box::new(Im::new(FcitxInstance::new(fcitx_instance)));
    trace!("ime_create(): Created IM");
    Box::into_raw(instance) as *mut std::ffi::c_void
}

/// Destroys the given IM instance.
// This is called by fcitx.
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ime_destroy(im_class: *mut std::ffi::c_void) {
    trace!("ime_destroy(): Destroying IM");
    drop(Box::from_raw(im_class as *mut Im));
    trace!("ime_destroy(): Destroyed IM");
}
