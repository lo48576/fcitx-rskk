//! Fcitx binding.

use fcitx_sys;

/// Fcitx ABI version.
pub(crate) static ABI_VERSION: u32 = fcitx_sys::FCITX_ABI_VERSION;

/// Fcitx instance.
pub(crate) struct FcitxInstance(*mut fcitx_sys::FcitxInstance);

impl FcitxInstance {
    /// Creates a new fcitx instance wrapper.
    pub(crate) fn new(instance: *mut fcitx_sys::FcitxInstance) -> Self {
        Self(instance)
    }

    /// Returns the raw pointer to the instance.
    pub(crate) fn raw_ptr(&mut self) -> *mut fcitx_sys::FcitxInstance {
        self.0
    }
}
