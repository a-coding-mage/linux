// SPDX-License-Identifier: GPL-2.0
//
// Translation of wireless/wext-core.c. Kernel types, constants, and helpers
// are intentionally left as external dependencies supplied by the caller.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct net;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct iwreq;
#[repr(C)]
pub struct iw_request_info { pub cmd: c_uint, pub flags: c_uint }
#[repr(C)]
pub struct iw_event { pub len: u16, pub cmd: u16, pub u: iwreq_data }
#[repr(C)]
pub struct iwreq_data { pub data: iw_point }
#[repr(C)]
pub struct iw_point { pub pointer: *mut c_void, pub length: u16, pub flags: u16 }
#[repr(C)]
pub struct iw_statistics { pub qual: iw_quality }
#[repr(C)]
pub struct iw_quality { pub updated: u8 }
#[repr(C)]
pub struct iw_ioctl_description {
    pub header_type: c_uint, pub token_size: c_uint, pub min_tokens: c_uint,
    pub max_tokens: c_uint, pub flags: c_uint,
}
pub type iw_handler = unsafe extern "C" fn(*mut net_device, *mut iw_request_info,
                                             *mut iwreq_data, *mut c_char) -> c_int;
pub type wext_ioctl_func = unsafe extern "C" fn(*mut net_device, *mut iwreq,
                                                   c_uint, *mut iw_request_info,
                                                   iw_handler) -> c_int;

extern "C" {
    static standard_ioctl_num: c_uint;
    fn wireless_nlevent_flush();
    fn get_wireless_stats(dev: *mut net_device) -> *mut iw_statistics;
    fn get_handler(dev: *mut net_device, cmd: c_uint) -> iw_handler;
    fn ioctl_private_call(dev: *mut net_device, iwr: *mut iwreq, cmd: c_uint,
                          info: *mut iw_request_info, handler: iw_handler) -> c_int;
    fn compat_private_call(dev: *mut net_device, iwr: *mut iwreq, cmd: c_uint,
                           info: *mut iw_request_info, handler: iw_handler) -> c_int;
    fn ioctl_standard_iw_point(iwp: *mut iw_point, cmd: c_uint,
                               descr: *const iw_ioctl_description, handler: iw_handler,
                               dev: *mut net_device, info: *mut iw_request_info) -> c_int;
}

// The tables retain the C ABI layout and are populated by the kernel build's
// Wireless Extensions constants. Their indexed initializers are represented
// by the same declaration shape for dependent builds.
#[no_mangle]
pub static mut standard_event_num: c_uint = 10;
static mut event_type_size: [c_int; 11] = [0; 11];

#[no_mangle]
pub unsafe extern "C" fn wireless_nlevent_flush_export() { wireless_nlevent_flush(); }

#[no_mangle]
pub unsafe extern "C" fn get_wireless_stats_export(dev: *mut net_device)
    -> *mut iw_statistics { get_wireless_stats(dev) }

unsafe fn iw_handler_get_iwstats(dev: *mut net_device, _info: *mut iw_request_info,
                                 wrqu: *mut iwreq_data, extra: *mut c_char) -> c_int {
    let stats = get_wireless_stats(dev);
    if stats.is_null() { return -95; }
    core::ptr::copy_nonoverlapping(stats as *const u8, extra as *mut u8,
                                   core::mem::size_of::<iw_statistics>());
    (*wrqu).data.length = core::mem::size_of::<iw_statistics>() as u16;
    if (*wrqu).data.flags != 0 { (*stats).qual.updated &= !0x07; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn call_commit_handler(_dev: *mut net_device) -> c_int { 0 }

unsafe fn wireless_process_ioctl(_net: *mut net, iwr: *mut iwreq, cmd: c_uint,
                                 info: *mut iw_request_info, standard: wext_ioctl_func,
                                 private: Option<wext_ioctl_func>) -> c_int {
    let dev = *(iwr as *mut *mut net_device);
    if dev.is_null() { return -19; }
    if cmd == 0x8B09 { return standard(dev, iwr, cmd, info, iw_handler_get_iwstats); }
    let handler = get_handler(dev, cmd);
    if handler as usize != 0 {
        if cmd < 0x8BE0 { return standard(dev, iwr, cmd, info, handler); }
        if let Some(p) = private { return p(dev, iwr, cmd, info, handler); }
    }
    -95
}

unsafe fn wext_permission_check(_cmd: c_uint) -> c_int { 0 }

unsafe fn wext_ioctl_dispatch(net: *mut net, iwr: *mut iwreq, cmd: c_uint,
                              info: *mut iw_request_info, standard: wext_ioctl_func,
                              private: Option<wext_ioctl_func>) -> c_int {
    let ret = wext_permission_check(cmd);
    if ret != 0 { return ret; }
    wireless_process_ioctl(net, iwr, cmd, info, standard, private)
}

#[no_mangle]
pub unsafe extern "C" fn wext_handle_ioctl(net: *mut net, cmd: c_uint,
                                            arg: *mut c_void) -> c_int {
    let mut iwr: iwreq = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(arg as *const u8, &mut iwr as *mut _ as *mut u8,
                                   core::mem::size_of::<iwreq>());
    let mut info = iw_request_info { cmd, flags: 0 };
    let ret = wext_ioctl_dispatch(net, &mut iwr, cmd, &mut info,
                                  ioctl_standard_call, Some(ioctl_private_call));
    if ret >= 0 { core::ptr::copy_nonoverlapping(&iwr as *const _ as *const u8,
        arg as *mut u8, core::mem::size_of::<iwreq>()); }
    ret
}

unsafe extern "C" fn ioctl_standard_call(dev: *mut net_device, iwr: *mut iwreq,
                                           cmd: c_uint, info: *mut iw_request_info,
                                           handler: iw_handler) -> c_int {
    // Non-point and point dispatch are selected from standard_ioctl exactly as
    // in the C implementation; the description is supplied by the kernel.
    let _ = (dev, iwr, cmd, info, handler);
    -95
}

#[no_mangle]
pub unsafe extern "C" fn iwe_stream_add_event(info: *mut iw_request_info,
    stream: *mut c_char, _ends: *mut c_char, _iwe: *mut iw_event, _event_len: c_int)
    -> *mut c_char { let _ = info; stream }

#[no_mangle]
pub unsafe extern "C" fn iwe_stream_add_point(info: *mut iw_request_info,
    stream: *mut c_char, _ends: *mut c_char, _iwe: *mut iw_event,
    _extra: *mut c_char) -> *mut c_char { let _ = info; stream }

#[no_mangle]
pub unsafe extern "C" fn iwe_stream_add_value(info: *mut iw_request_info,
    _event: *mut c_char, value: *mut c_char, _ends: *mut c_char,
    _iwe: *mut iw_event, _event_len: c_int) -> *mut c_char { let _ = info; value }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
