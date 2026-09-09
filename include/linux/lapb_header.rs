/* SPDX-License-Identifier: GPL-2.0 */
/*
 * These are the public elements of the Linux LAPB module.
 *
 * C dependencies retained as opaque declarations: linux/skbuff.h,
 * linux/timer.h.
 */

use core::ffi::c_int;

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

pub const LAPB_OK: c_int = 0;
pub const LAPB_BADTOKEN: c_int = 1;
pub const LAPB_INVALUE: c_int = 2;
pub const LAPB_CONNECTED: c_int = 3;
pub const LAPB_NOTCONNECTED: c_int = 4;
pub const LAPB_REFUSED: c_int = 5;
pub const LAPB_TIMEDOUT: c_int = 6;
pub const LAPB_NOMEM: c_int = 7;

pub const LAPB_STANDARD: c_int = 0x00;
pub const LAPB_EXTENDED: c_int = 0x01;

pub const LAPB_SLP: c_int = 0x00;
pub const LAPB_MLP: c_int = 0x02;

pub const LAPB_DTE: c_int = 0x00;
pub const LAPB_DCE: c_int = 0x04;

#[repr(C)]
pub struct lapb_register_struct {
    pub connect_confirmation:
        Option<unsafe extern "C" fn(dev: *mut net_device, reason: c_int)>,
    pub connect_indication:
        Option<unsafe extern "C" fn(dev: *mut net_device, reason: c_int)>,
    pub disconnect_confirmation:
        Option<unsafe extern "C" fn(dev: *mut net_device, reason: c_int)>,
    pub disconnect_indication:
        Option<unsafe extern "C" fn(dev: *mut net_device, reason: c_int)>,
    pub data_indication: Option<unsafe extern "C" fn(
        dev: *mut net_device,
        skb: *mut sk_buff,
    ) -> c_int>,
    pub data_transmit:
        Option<unsafe extern "C" fn(dev: *mut net_device, skb: *mut sk_buff)>,
}

#[repr(C)]
pub struct lapb_parms_struct {
    pub t1: u32,
    pub t1timer: u32,
    pub t2: u32,
    pub t2timer: u32,
    pub n2: u32,
    pub n2count: u32,
    pub window: u32,
    pub state: u32,
    pub mode: u32,
}

unsafe extern "C" {
    pub fn lapb_register(
        dev: *mut net_device,
        callbacks: *const lapb_register_struct,
    ) -> c_int;
    pub fn lapb_unregister(dev: *mut net_device) -> c_int;
    pub fn lapb_getparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> c_int;
    pub fn lapb_setparms(dev: *mut net_device, parms: *mut lapb_parms_struct) -> c_int;
    pub fn lapb_connect_request(dev: *mut net_device) -> c_int;
    pub fn lapb_disconnect_request(dev: *mut net_device) -> c_int;
    pub fn lapb_data_request(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
    pub fn lapb_data_received(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
