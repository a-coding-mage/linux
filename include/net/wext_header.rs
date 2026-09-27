/* SPDX-License-Identifier: GPL-2.0 */

/* C dependency: <net/iw_handler.h> */

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[cfg(CONFIG_WEXT_CORE)]
extern "C" {
    pub fn wext_handle_ioctl(net: *mut net, cmd: ::kernel::ffi::c_uint, arg: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int;
    pub fn compat_wext_handle_ioctl(net: *mut net, cmd: ::kernel::ffi::c_uint, arg: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;

    pub fn get_wireless_stats(dev: *mut net_device) -> *mut iw_statistics;
    pub fn call_commit_handler(dev: *mut net_device) -> ::kernel::ffi::c_int;
}

#[cfg(not(CONFIG_WEXT_CORE))]
#[inline]
pub unsafe fn wext_handle_ioctl(
    _net: *mut net,
    _cmd: ::kernel::ffi::c_uint,
    _arg: *mut ::kernel::ffi::c_void,
) -> ::kernel::ffi::c_int {
    -22
}

#[cfg(not(CONFIG_WEXT_CORE))]
#[inline]
pub unsafe fn compat_wext_handle_ioctl(
    _net: *mut net,
    _cmd: ::kernel::ffi::c_uint,
    _arg: ::kernel::ffi::c_ulong,
) -> ::kernel::ffi::c_int {
    -22
}

#[cfg(CONFIG_WEXT_PROC)]
extern "C" {
    pub fn wext_proc_init(net: *mut net) -> ::kernel::ffi::c_int;
    pub fn wext_proc_exit(net: *mut net);
}

#[cfg(not(CONFIG_WEXT_PROC))]
#[inline]
pub unsafe fn wext_proc_init(_net: *mut net) -> ::kernel::ffi::c_int {
    0
}

#[cfg(not(CONFIG_WEXT_PROC))]
#[inline]
pub unsafe fn wext_proc_exit(_net: *mut net) {}

#[cfg(CONFIG_WEXT_PRIV)]
extern "C" {
    pub fn ioctl_private_call(
        dev: *mut net_device,
        iwr: *mut iwreq,
        cmd: ::kernel::ffi::c_uint,
        info: *mut iw_request_info,
        handler: iw_handler,
    ) -> ::kernel::ffi::c_int;
    pub fn compat_private_call(
        dev: *mut net_device,
        iwr: *mut iwreq,
        cmd: ::kernel::ffi::c_uint,
        info: *mut iw_request_info,
        handler: iw_handler,
    ) -> ::kernel::ffi::c_int;
    pub fn iw_handler_get_private(
        dev: *mut net_device,
        info: *mut iw_request_info,
        wrqu: *mut iwreq_data,
        extra: *mut ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
}

#[cfg(not(CONFIG_WEXT_PRIV))]
pub const ioctl_private_call: Option<iw_handler> = None;

#[cfg(not(CONFIG_WEXT_PRIV))]
pub const compat_private_call: Option<iw_handler> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
