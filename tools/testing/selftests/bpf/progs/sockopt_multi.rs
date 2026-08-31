// SPDX-License-Identifier: GPL-2.0
// Translated from C includes:
// <netinet/in.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

use core::ffi::c_void;

const SOL_IP: i32 = 0;
const IP_TOS: i32 = 1;

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut c_void,
    pub optval: *mut u8,
    pub optval_end: *mut u8,
    pub level: i32,
    pub optname: i32,
    pub optlen: i32,
    pub retval: i32,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut page_size: i32 = 0;

#[no_mangle]
#[link_section = "cgroup/getsockopt"]
pub unsafe extern "C" fn _getsockopt_child(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut u8 = (*ctx).optval_end;
    let optval: *mut u8 = (*ctx).optval;

    if (*ctx).level != SOL_IP || (*ctx).optname != IP_TOS {
        /* optval larger than PAGE_SIZE use kernel's buffer. */
        if (*ctx).optlen > page_size {
            (*ctx).optlen = 0;
        }
        return 1;
    }

    if optval.add(1) > optval_end {
        return 0; /* EPERM, bounds check */
    }

    if *optval.add(0) != 0x80 {
        return 0; /* EPERM, unexpected optval from the kernel */
    }

    (*ctx).retval = 0; /* Reset system call return value to zero */

    *optval.add(0) = 0x90;
    (*ctx).optlen = 1;

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/getsockopt"]
pub unsafe extern "C" fn _getsockopt_parent(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut u8 = (*ctx).optval_end;
    let optval: *mut u8 = (*ctx).optval;

    if (*ctx).level != SOL_IP || (*ctx).optname != IP_TOS {
        /* optval larger than PAGE_SIZE use kernel's buffer. */
        if (*ctx).optlen > page_size {
            (*ctx).optlen = 0;
        }
        return 1;
    }

    if optval.add(1) > optval_end {
        return 0; /* EPERM, bounds check */
    }

    if *optval.add(0) != 0x90 {
        return 0; /* EPERM, unexpected optval from the kernel */
    }

    (*ctx).retval = 0; /* Reset system call return value to zero */

    *optval.add(0) = 0xA0;
    (*ctx).optlen = 1;

    return 1;
}

#[no_mangle]
#[link_section = "cgroup/setsockopt"]
pub unsafe extern "C" fn _setsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let optval_end: *mut u8 = (*ctx).optval_end;
    let optval: *mut u8 = (*ctx).optval;

    if (*ctx).level != SOL_IP || (*ctx).optname != IP_TOS {
        /* optval larger than PAGE_SIZE use kernel's buffer. */
        if (*ctx).optlen > page_size {
            (*ctx).optlen = 0;
        }
        return 1;
    }

    if optval.add(1) > optval_end {
        return 0; /* EPERM, bounds check */
    }

    *optval.add(0) = (*optval.add(0)).wrapping_add(0x10);
    (*ctx).optlen = 1;

    return 1;
}
