// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// <stdio.h>, <string.h>, <ynl.h>, <net/if.h>, <kselftest_harness.h>,
// and "ethtool-user.h".

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_error,
}

#[repr(C)]
pub struct ethtool {
    pub ys: *mut ynl_sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_header_present {
    pub header: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_header_len {
    pub dev_name: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_header {
    pub _len: ethtool_header_len,
    pub dev_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_channels_present {
    pub rx_count: bool,
    pub tx_count: bool,
    pub combined_count: bool,
}

#[repr(C)]
pub struct ethtool_channels_get_req_dump {
    pub _present: ethtool_header_present,
}

#[repr(C)]
pub struct ethtool_channels_get_rsp {
    pub header: ethtool_header,
    pub _present: ethtool_channels_present,
    pub rx_count: c_int,
    pub tx_count: c_int,
    pub combined_count: c_int,
}

#[repr(C)]
pub struct ethtool_channels_get_list {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_rings_present {
    pub rx: bool,
    pub tx: bool,
}

#[repr(C)]
pub struct ethtool_rings_get_req_dump {
    pub _present: ethtool_header_present,
}

#[repr(C)]
pub struct ethtool_rings_get_rsp {
    pub header: ethtool_header,
    pub _present: ethtool_rings_present,
    pub rx: c_int,
    pub tx: c_int,
}

#[repr(C)]
pub struct ethtool_rings_get_list {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static ynl_ethtool_family: ynl_family;

    pub fn ynl_sock_create(family: *const ynl_family, yarg: *mut c_void) -> *mut ynl_sock;
    pub fn ynl_sock_destroy(ys: *mut ynl_sock);

    pub fn ethtool_channels_get_dump(
        ys: *mut ynl_sock,
        req: *mut ethtool_channels_get_req_dump,
    ) -> *mut ethtool_channels_get_list;
    pub fn ethtool_channels_get_list_free(list: *mut ethtool_channels_get_list);

    pub fn ethtool_rings_get_dump(
        ys: *mut ynl_sock,
        req: *mut ethtool_rings_get_req_dump,
    ) -> *mut ethtool_rings_get_list;
    pub fn ethtool_rings_get_list_free(list: *mut ethtool_rings_get_list);

    pub fn ynl_dump_empty(dump: *const c_void) -> bool;

    // Rust translation expects bindings for the C ynl_dump_foreach() expansion.
    pub fn ynl_dump_foreach_channels_get(
        list: *mut ethtool_channels_get_list,
        cb: unsafe extern "C" fn(*mut ethtool_channels_get_rsp, *mut c_void),
        data: *mut c_void,
    );
    pub fn ynl_dump_foreach_rings_get(
        list: *mut ethtool_rings_get_list,
        cb: unsafe extern "C" fn(*mut ethtool_rings_get_rsp, *mut c_void),
        data: *mut c_void,
    );

    pub fn printf(fmt: *const c_char, ...) -> c_int;
    pub fn ksft_print_msg(fmt: *const c_char, ...) -> c_int;
}

macro_rules! assert_ne {
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left == $right {
            panic!($($arg)*);
        }
    };
}

macro_rules! expect_true {
    ($cond:expr) => {
        if !$cond {
            eprintln!("EXPECT_TRUE failed: {}", stringify!($cond));
        }
    };
}

macro_rules! skip {
    (return, $($arg:tt)*) => {{
        eprintln!($($arg)*);
        return;
    }};
}

pub unsafe fn ethtool_setup(self_: *mut ethtool) {
    unsafe {
        (*self_).ys = ynl_sock_create(&ynl_ethtool_family, ptr::null_mut());
        assert_ne!(
            ptr::null_mut(),
            (*self_).ys,
            "failed to create ethtool socket"
        );
    }
}

pub unsafe fn ethtool_teardown(self_: *mut ethtool) {
    unsafe {
        ynl_sock_destroy((*self_).ys);
    }
}

unsafe extern "C" fn ethtool_channels_dev(dev: *mut ethtool_channels_get_rsp, _data: *mut c_void) {
    unsafe {
        expect_true!((*dev).header._len.dev_name as bool);
        ksft_print_msg(c"%8s: ".as_ptr(), (*dev).header.dev_name);
        expect_true!(
            (*dev)._present.rx_count
                || (*dev)._present.tx_count
                || (*dev)._present.combined_count
        );
        if (*dev)._present.rx_count {
            printf(c"rx %d ".as_ptr(), (*dev).rx_count);
        }
        if (*dev)._present.tx_count {
            printf(c"tx %d ".as_ptr(), (*dev).tx_count);
        }
        if (*dev)._present.combined_count {
            printf(c"combined %d ".as_ptr(), (*dev).combined_count);
        }
        printf(c"\n".as_ptr());
    }
}

pub unsafe fn ethtool_channels(self_: *mut ethtool) {
    unsafe {
        let mut creq: ethtool_channels_get_req_dump = core::mem::zeroed();
        let channels: *mut ethtool_channels_get_list;

        creq._present.header = 1; // ethtool needs an empty nest
        channels = ethtool_channels_get_dump((*self_).ys, &mut creq);
        assert_ne!(
            ptr::null_mut(),
            channels,
            "channels dump failed: {:?}",
            (*(*self_).ys).err.msg
        );

        if ynl_dump_empty(channels.cast::<c_void>()) {
            ethtool_channels_get_list_free(channels);
            skip!(return, "no entries in channels dump");
        }

        ynl_dump_foreach_channels_get(channels, ethtool_channels_dev, ptr::null_mut());
        ethtool_channels_get_list_free(channels);
    }
}

unsafe extern "C" fn ethtool_rings_dev(dev: *mut ethtool_rings_get_rsp, _data: *mut c_void) {
    unsafe {
        expect_true!((*dev).header._len.dev_name as bool);
        ksft_print_msg(c"%8s: ".as_ptr(), (*dev).header.dev_name);
        expect_true!((*dev)._present.rx || (*dev)._present.tx);
        if (*dev)._present.rx {
            printf(c"rx %d ".as_ptr(), (*dev).rx);
        }
        if (*dev)._present.tx {
            printf(c"tx %d ".as_ptr(), (*dev).tx);
        }
        printf(c"\n".as_ptr());
    }
}

pub unsafe fn ethtool_rings(self_: *mut ethtool) {
    unsafe {
        let mut rreq: ethtool_rings_get_req_dump = core::mem::zeroed();
        let rings: *mut ethtool_rings_get_list;

        rreq._present.header = 1; // ethtool needs an empty nest
        rings = ethtool_rings_get_dump((*self_).ys, &mut rreq);
        assert_ne!(
            ptr::null_mut(),
            rings,
            "rings dump failed: {:?}",
            (*(*self_).ys).err.msg
        );

        if ynl_dump_empty(rings.cast::<c_void>()) {
            ethtool_rings_get_list_free(rings);
            skip!(return, "no entries in rings dump");
        }

        ynl_dump_foreach_rings_get(rings, ethtool_rings_dev, ptr::null_mut());
        ethtool_rings_get_list_free(rings);
    }
}

pub fn main() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
