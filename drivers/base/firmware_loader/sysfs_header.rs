/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/device.h> and "firmware.h".
// MODULE_IMPORT_NS("FIRMWARE_LOADER_PRIVATE");

#[cfg(feature = "CONFIG_FW_LOADER_SYSFS")]
extern "C" {
    pub static mut fw_fallback_config: firmware_fallback_config;
    pub static mut dev_attr_loading: device_attribute;
}

#[cfg(not(feature = "CONFIG_FW_LOADER_SYSFS"))]
#[inline]
pub fn register_sysfs_loader() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_FW_LOADER_SYSFS"))]
#[inline]
pub fn unregister_sysfs_loader() {}

// CONFIG_FW_LOADER_USER_HELPER
/// Firmware fallback configuration settings.
#[repr(C)]
pub struct firmware_fallback_config {
    pub force_sysfs_fallback: ::core::ffi::c_uint,
    pub ignore_sysfs_fallback: ::core::ffi::c_uint,
    pub old_timeout: ::core::ffi::c_int,
    pub loading_timeout: ::core::ffi::c_int,
}

// These getters are vetted to use int properly.
#[inline]
pub unsafe fn __firmware_loading_timeout() -> ::core::ffi::c_int {
    fw_fallback_config.loading_timeout
}

// These setters are vetted to use int properly.
#[inline]
pub unsafe fn __fw_fallback_set_timeout(timeout: ::core::ffi::c_int) {
    fw_fallback_config.loading_timeout = timeout;
}

// CONFIG_FW_LOADER_SYSFS
#[cfg(all(feature = "CONFIG_FW_LOADER_USER_HELPER", feature = "CONFIG_SYSCTL"))]
extern "C" {
    pub fn register_sysfs_loader() -> ::core::ffi::c_int;
    pub fn unregister_sysfs_loader();
}

#[cfg(not(all(feature = "CONFIG_FW_LOADER_USER_HELPER", feature = "CONFIG_SYSCTL")))]
#[inline]
pub fn register_firmware_config_sysctl() -> ::core::ffi::c_int { 0 }

#[cfg(not(all(feature = "CONFIG_FW_LOADER_USER_HELPER", feature = "CONFIG_SYSCTL")))]
#[inline]
pub fn unregister_firmware_config_sysctl() {}

// CONFIG_FW_LOADER_USER_HELPER && CONFIG_SYSCTL
#[cfg(feature = "CONFIG_FW_UPLOAD")]
extern "C" {
    pub fn register_firmware_config_sysctl() -> ::core::ffi::c_int;
    pub fn unregister_firmware_config_sysctl();
}

#[cfg(not(feature = "CONFIG_FW_UPLOAD"))]
#[inline]
pub fn fw_upload_start(_fw_sysfs: *mut fw_sysfs) -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_FW_UPLOAD"))]
#[inline]
pub fn fw_upload_free(_fw_sysfs: *mut fw_sysfs) {}

#[repr(C)]
pub struct fw_sysfs {
    pub nowait: bool,
    pub dev: device,
    pub fw_priv: *mut fw_priv,
    pub fw: *mut firmware,
    pub fw_upload_priv: *mut ::core::ffi::c_void,
}

// #define to_fw_sysfs(__dev) container_of_const(__dev, struct fw_sysfs, dev)

extern "C" {
    pub fn __fw_load_abort(fw_priv: *mut fw_priv);
}

#[inline]
pub unsafe fn fw_load_abort(fw_sysfs: *mut fw_sysfs) {
    let fw_priv = (*fw_sysfs).fw_priv;
    __fw_load_abort(fw_priv);
}

extern "C" {
    pub fn fw_create_instance(
        firmware: *mut firmware,
        fw_name: *const ::core::ffi::c_char,
        device: *mut device,
        opt_flags: u32,
    ) -> *mut fw_sysfs;
}

// CONFIG_FW_UPLOAD
extern "C" {
    pub static mut dev_attr_status: device_attribute;
    pub static mut dev_attr_error: device_attribute;
    pub static mut dev_attr_cancel: device_attribute;
    pub static mut dev_attr_remaining_size: device_attribute;

    pub fn fw_upload_start(fw_sysfs: *mut fw_sysfs) -> ::core::ffi::c_int;
    pub fn fw_upload_free(fw_sysfs: *mut fw_sysfs);
    pub fn fw_upload_is_visible(
        kobj: *mut kobject,
        attr: *mut attribute,
        n: ::core::ffi::c_int,
    ) -> umode_t;
}

// Forward declarations supplied by the included kernel headers.
pub enum device {}
pub enum device_attribute {}
pub enum fw_priv {}
pub enum firmware {}
pub enum kobject {}
pub enum attribute {}
pub type umode_t = u16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
