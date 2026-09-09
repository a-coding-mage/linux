/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const FW_ACTION_NOUEVENT: i32 = 0;
pub const FW_ACTION_UEVENT: i32 = 1;

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
    /* firmware loader private fields */
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_upload_err {
    FW_UPLOAD_ERR_NONE,
    FW_UPLOAD_ERR_HW_ERROR,
    FW_UPLOAD_ERR_TIMEOUT,
    FW_UPLOAD_ERR_CANCELED,
    FW_UPLOAD_ERR_BUSY,
    FW_UPLOAD_ERR_INVALID_SIZE,
    FW_UPLOAD_ERR_RW_ERROR,
    FW_UPLOAD_ERR_WEAROUT,
    FW_UPLOAD_ERR_FW_INVALID,
    FW_UPLOAD_ERR_MAX,
}

#[repr(C)]
pub struct fw_upload {
    pub dd_handle: *mut core::ffi::c_void,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct fw_upload_ops {
    pub prepare: Option<unsafe extern "C" fn(
        fw_upload: *mut fw_upload,
        data: *const u8,
        size: u32,
    ) -> fw_upload_err>,
    pub write: Option<unsafe extern "C" fn(
        fw_upload: *mut fw_upload,
        data: *const u8,
        offset: u32,
        size: u32,
        written: *mut u32,
    ) -> fw_upload_err>,
    pub poll_complete: Option<unsafe extern "C" fn(fw_upload: *mut fw_upload) -> fw_upload_err>,
    pub cancel: Option<unsafe extern "C" fn(fw_upload: *mut fw_upload)>,
    pub cleanup: Option<unsafe extern "C" fn(fw_upload: *mut fw_upload)>,
}

pub enum module {}
pub enum device {}

/* Built-in firmware functionality is only available if FW_LOADER=y, but not FW_LOADER=m. */
#[cfg(feature = "CONFIG_FW_LOADER")]
unsafe extern "C" {
    pub fn firmware_request_builtin(fw: *mut firmware, name: *const core::ffi::c_char) -> bool;
}

#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn firmware_request_builtin(
    _fw: *mut firmware,
    _name: *const core::ffi::c_char,
) -> bool {
    false
}

/* Equivalent of IS_REACHABLE(CONFIG_FW_LOADER). */
#[cfg(feature = "CONFIG_FW_LOADER")]
unsafe extern "C" {
    pub fn request_firmware(
        fw: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
    ) -> i32;
    pub fn firmware_request_nowait_nowarn(
        module: *mut module,
        name: *const core::ffi::c_char,
        device: *mut device,
        gfp: usize,
        context: *mut core::ffi::c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>,
    ) -> i32;
    pub fn firmware_request_nowarn(
        fw: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
    ) -> i32;
    pub fn firmware_request_platform(
        fw: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
    ) -> i32;
    pub fn request_firmware_nowait(
        module: *mut module,
        uevent: bool,
        name: *const core::ffi::c_char,
        device: *mut device,
        gfp: usize,
        context: *mut core::ffi::c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>,
    ) -> i32;
    pub fn request_firmware_nowait_cancel(
        device: *mut device,
        context: *mut core::ffi::c_void,
        cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>,
    );
    pub fn request_firmware_direct(
        fw: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
    ) -> i32;
    pub fn request_firmware_into_buf(
        firmware_p: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
        buf: *mut core::ffi::c_void,
        size: usize,
    ) -> i32;
    pub fn request_partial_firmware_into_buf(
        firmware_p: *mut *const firmware,
        name: *const core::ffi::c_char,
        device: *mut device,
        buf: *mut core::ffi::c_void,
        size: usize,
        offset: usize,
    ) -> i32;
    pub fn release_firmware(fw: *const firmware);
}

#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_firmware(_fw: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn firmware_request_nowait_nowarn(_module: *mut module, _name: *const core::ffi::c_char, _device: *mut device, _gfp: usize, _context: *mut core::ffi::c_void, _cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn firmware_request_nowarn(_fw: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn firmware_request_platform(_fw: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_firmware_nowait(_module: *mut module, _uevent: bool, _name: *const core::ffi::c_char, _device: *mut device, _gfp: usize, _context: *mut core::ffi::c_void, _cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_firmware_nowait_cancel(_device: *mut device, _context: *mut core::ffi::c_void, _cont: Option<unsafe extern "C" fn(*const firmware, *mut core::ffi::c_void)>) {}
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn release_firmware(_fw: *const firmware) {}
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_firmware_direct(_fw: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_firmware_into_buf(_firmware_p: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device, _buf: *mut core::ffi::c_void, _size: usize) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_FW_LOADER"))]
pub unsafe fn request_partial_firmware_into_buf(_firmware_p: *mut *const firmware, _name: *const core::ffi::c_char, _device: *mut device, _buf: *mut core::ffi::c_void, _size: usize, _offset: usize) -> i32 { -22 }

#[cfg(feature = "CONFIG_FW_UPLOAD")]
unsafe extern "C" {
    pub fn firmware_upload_register(module: *mut module, parent: *mut device, name: *const core::ffi::c_char, ops: *const fw_upload_ops, dd_handle: *mut core::ffi::c_void) -> *mut fw_upload;
    pub fn firmware_upload_unregister(fw_upload: *mut fw_upload);
}

#[cfg(not(feature = "CONFIG_FW_UPLOAD"))]
pub unsafe fn firmware_upload_register(_module: *mut module, _parent: *mut device, _name: *const core::ffi::c_char, _ops: *const fw_upload_ops, _dd_handle: *mut core::ffi::c_void) -> *mut fw_upload { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_FW_UPLOAD"))]
pub unsafe fn firmware_upload_unregister(_fw_upload: *mut fw_upload) {}

pub unsafe extern "C" fn firmware_request_cache(_device: *mut device, _name: *const core::ffi::c_char) -> i32;

// DEFINE_FREE(firmware, struct firmware *, release_firmware(_T))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
