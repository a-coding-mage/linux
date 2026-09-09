/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright Samuel Mendoza-Jonas, IBM Corporation 2018.
 */

// Dependency corresponding to <linux/netdevice.h> and "internal.h".

#[repr(C)]
pub struct ncsi_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ncsi_package {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ncsi_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nlmsghdr {
    _private: [u8; 0],
}

pub type u32 = ::core::ffi::c_uint;

unsafe extern "C" {
    pub fn ncsi_send_netlink_rsp(
        nr: *mut ncsi_request,
        np: *mut ncsi_package,
        nc: *mut ncsi_channel,
    ) -> ::core::ffi::c_int;

    pub fn ncsi_send_netlink_timeout(
        nr: *mut ncsi_request,
        np: *mut ncsi_package,
        nc: *mut ncsi_channel,
    ) -> ::core::ffi::c_int;

    pub fn ncsi_send_netlink_err(
        dev: *mut net_device,
        snd_seq: u32,
        snd_portid: u32,
        nlhdr: *const nlmsghdr,
        err: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
