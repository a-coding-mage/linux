/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright © 2016 Intel Corporation
 *
 * Authors:
 *    Rafael Antognolli <rafael.antognolli@intel.com>
 *    Scott  Bauer      <scott.bauer@intel.com>
 */

// Translated from the Linux kernel header sed-opal.h.
// External types and ioctl constants are supplied by the corresponding dependencies.

#[repr(C)]
pub struct opal_dev {
    _private: [u8; 0],
}

pub type sec_send_recv = unsafe extern "C" fn(
    data: *mut core::ffi::c_void,
    spsp: u16,
    secp: u8,
    buffer: *mut core::ffi::c_void,
    len: usize,
    send: bool,
) -> core::ffi::c_int;

#[cfg(CONFIG_BLK_SED_OPAL)]
extern "C" {
    pub fn free_opal_dev(dev: *mut opal_dev);
    pub fn opal_unlock_from_suspend(dev: *mut opal_dev) -> bool;
    pub fn init_opal_dev(
        data: *mut core::ffi::c_void,
        send_recv: Option<sec_send_recv>,
    ) -> *mut opal_dev;
    pub fn sed_ioctl(
        dev: *mut opal_dev,
        cmd: core::ffi::c_uint,
        ioctl_ptr: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

}

#[cfg(CONFIG_BLK_SED_OPAL)]
pub const OPAL_AUTH_KEY: &str = "opal-boot-pin";
#[cfg(CONFIG_BLK_SED_OPAL)]
pub const OPAL_AUTH_KEY_PREV: &str = "opal-boot-pin-prev";

#[cfg(CONFIG_BLK_SED_OPAL)]
#[inline]
pub unsafe fn is_sed_ioctl(cmd: core::ffi::c_uint) -> bool {
    match cmd {
        IOC_OPAL_SAVE
        | IOC_OPAL_LOCK_UNLOCK
        | IOC_OPAL_TAKE_OWNERSHIP
        | IOC_OPAL_ACTIVATE_LSP
        | IOC_OPAL_SET_PW
        | IOC_OPAL_ACTIVATE_USR
        | IOC_OPAL_REVERT_TPR
        | IOC_OPAL_LR_SETUP
        | IOC_OPAL_ADD_USR_TO_LR
        | IOC_OPAL_ENABLE_DISABLE_MBR
        | IOC_OPAL_ERASE_LR
        | IOC_OPAL_SECURE_ERASE_LR
        | IOC_OPAL_PSID_REVERT_TPR
        | IOC_OPAL_MBR_DONE
        | IOC_OPAL_WRITE_SHADOW_MBR
        | IOC_OPAL_GENERIC_TABLE_RW
        | IOC_OPAL_GET_STATUS
        | IOC_OPAL_GET_LR_STATUS
        | IOC_OPAL_GET_GEOMETRY
        | IOC_OPAL_DISCOVERY
        | IOC_OPAL_REVERT_LSP
        | IOC_OPAL_SET_SID_PW
        | IOC_OPAL_REACTIVATE_LSP
        | IOC_OPAL_LR_SET_START_LEN
        | IOC_OPAL_ENABLE_DISABLE_LR
        | IOC_OPAL_GET_SUM_STATUS
        | IOC_OPAL_STACK_RESET => true,
        _ => false,
    }
}

#[cfg(not(CONFIG_BLK_SED_OPAL))]
#[inline]
pub unsafe fn free_opal_dev(_dev: *mut opal_dev) {}

#[cfg(not(CONFIG_BLK_SED_OPAL))]
#[inline]
pub unsafe fn is_sed_ioctl(_cmd: core::ffi::c_uint) -> bool {
    false
}

#[cfg(not(CONFIG_BLK_SED_OPAL))]
#[inline]
pub unsafe fn sed_ioctl(
    _dev: *mut opal_dev,
    _cmd: core::ffi::c_uint,
    _ioctl_ptr: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_BLK_SED_OPAL))]
#[inline]
pub unsafe fn opal_unlock_from_suspend(_dev: *mut opal_dev) -> bool {
    false
}

#[cfg(not(CONFIG_BLK_SED_OPAL))]
#[inline]
pub fn init_opal_dev(
    _data: *mut core::ffi::c_void,
    _send_recv: Option<sec_send_recv>,
) -> *mut opal_dev {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
