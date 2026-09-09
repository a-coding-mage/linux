/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding firmware-loader translation unit.
use core::ffi::c_char;

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_priv {
    _private: [u8; 0],
}

// CONFIG_FW_LOADER_USER_HELPER
#[cfg(feature = "CONFIG_FW_LOADER_USER_HELPER")]
extern "C" {
    pub fn firmware_fallback_sysfs(
        fw: *mut firmware,
        name: *const c_char,
        device: *mut device,
        opt_flags: u32,
        ret: i32,
    ) -> i32;
    pub fn kill_pending_fw_fallback_reqs(kill_all: bool);

    pub fn fw_fallback_set_cache_timeout();
    pub fn fw_fallback_set_default_timeout();
}

// !CONFIG_FW_LOADER_USER_HELPER
#[cfg(not(feature = "CONFIG_FW_LOADER_USER_HELPER"))]
pub unsafe fn firmware_fallback_sysfs(
    _fw: *mut firmware,
    _name: *const c_char,
    _device: *mut device,
    _opt_flags: u32,
    ret: i32,
) -> i32 {
    // Keep carrying over the same error.
    ret
}

#[cfg(not(feature = "CONFIG_FW_LOADER_USER_HELPER"))]
pub unsafe fn kill_pending_fw_fallback_reqs(_kill_all: bool) {}

#[cfg(not(feature = "CONFIG_FW_LOADER_USER_HELPER"))]
pub unsafe fn fw_fallback_set_cache_timeout() {}

#[cfg(not(feature = "CONFIG_FW_LOADER_USER_HELPER"))]
pub unsafe fn fw_fallback_set_default_timeout() {}

// CONFIG_EFI_EMBEDDED_FIRMWARE
#[cfg(feature = "CONFIG_EFI_EMBEDDED_FIRMWARE")]
extern "C" {
    pub fn firmware_fallback_platform(fw_priv: *mut fw_priv) -> i32;
}

// !CONFIG_EFI_EMBEDDED_FIRMWARE
#[cfg(not(feature = "CONFIG_EFI_EMBEDDED_FIRMWARE"))]
pub unsafe fn firmware_fallback_platform(_fw_priv: *mut fw_priv) -> i32 {
    // -ENOENT; errno constant supplied by the surrounding kernel translation.
    -ENOENT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
