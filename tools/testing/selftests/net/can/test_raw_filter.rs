// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * Copyright (c) 2011 Volkswagen Group Electronic Research
 * All rights reserved.
 */

// Translated from C includes:
// stdio.h, stdlib.h, unistd.h, string.h
// sys/types.h, sys/socket.h, sys/ioctl.h, sys/time.h, net/if.h, linux/if.h
// linux/can.h, linux/can/raw.h
// "kselftest_harness.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const ID: canid_t = 0x123;

const IFNAMSIZ: usize = 16;
const PF_CAN: c_int = 29;
const AF_CAN: c_int = PF_CAN;
const SOCK_RAW: c_int = 3;
const SOL_CAN_RAW: c_int = 101;
const CAN_RAW: c_int = 1;
const CAN_RAW_FILTER: c_int = 1;
const CAN_RAW_RECV_OWN_MSGS: c_int = 4;
const SIOCGIFINDEX: c_ulong = 0x8933;
const CAN_EFF_FLAG: canid_t = 0x80000000;
const CAN_RTR_FLAG: canid_t = 0x40000000;
const CAN_ERR_MASK: canid_t = 0x1fffffff;
const CAN_SFF_MASK: canid_t = 0x000007ff;
const CAN_EFF_MASK: canid_t = 0x1fffffff;
const KSFT_FAIL: c_int = 1;

type canid_t = u32;
type size_t = usize;
type socklen_t = u32;
type ssize_t = isize;
type time_t = c_long;
type suseconds_t = c_long;

static mut CANIF: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];

#[repr(C)]
struct can_frame {
    can_id: canid_t,
    len: u8,
    __pad: u8,
    __res0: u8,
    len8_dlc: u8,
    data: [u8; 8],
}

#[repr(C)]
struct can_filter {
    can_id: canid_t,
    can_mask: canid_t,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_can {
    can_family: u16,
    can_ifindex: c_int,
    can_addr: [u8; 8],
}

#[repr(C)]
union ifr_ifrn {
    ifrn_name: [c_char; IFNAMSIZ],
}

#[repr(C)]
union ifr_ifru {
    ifru_ifindex: c_int,
}

#[repr(C)]
struct ifreq {
    ifr_ifrn: ifr_ifrn,
    ifr_ifru: ifr_ifru,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct fd_set {
    fds_bits: [c_long; 16],
}

#[repr(C)]
struct can_filters {
    sock: c_int,
}

#[repr(C)]
struct can_filters_variant {
    testcase: c_int,
    id: canid_t,
    mask: canid_t,
    exp_num_rx: c_int,
    exp_flags: &'static [canid_t],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn perror(s: *const c_char);
    fn getenv(name: *const c_char) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;

    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn TH_LOG(format: *const c_char, ...);
    fn ASSERT_GE_i32(actual: c_int, expected: c_int);
    fn ASSERT_EQ_i32(actual: c_int, expected: c_int);
    fn ASSERT_NE_i32(actual: c_int, expected: c_int);
    fn ASSERT_EQ_u32(actual: canid_t, expected: canid_t);
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    for i in 0..(*set).fds_bits.len() {
        (*set).fds_bits[i] = 0;
    }
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let bits = 8 * size_of::<c_long>() as c_int;
    (*set).fds_bits[(fd / bits) as usize] |= 1 << (fd % bits);
}

unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> c_int {
    let bits = 8 * size_of::<c_long>() as c_int;
    (((*set).fds_bits[(fd / bits) as usize] & (1 << (fd % bits))) != 0) as c_int
}

unsafe fn send_can_frames(sock: c_int, testcase: c_int) -> c_int {
    let mut frame: can_frame = core::mem::zeroed();

    frame.len = 1;
    frame.data[0] = testcase as u8;

    frame.can_id = ID;
    if write(sock, &frame as *const _ as *const c_void, size_of::<can_frame>()) < 0 {
        goto_write_err();
        return 1;
    }

    frame.can_id = ID | CAN_RTR_FLAG;
    if write(sock, &frame as *const _ as *const c_void, size_of::<can_frame>()) < 0 {
        goto_write_err();
        return 1;
    }

    frame.can_id = ID | CAN_EFF_FLAG;
    if write(sock, &frame as *const _ as *const c_void, size_of::<can_frame>()) < 0 {
        goto_write_err();
        return 1;
    }

    frame.can_id = ID | CAN_EFF_FLAG | CAN_RTR_FLAG;
    if write(sock, &frame as *const _ as *const c_void, size_of::<can_frame>()) < 0 {
        goto_write_err();
        return 1;
    }

    return 0;

    unsafe fn goto_write_err() {
        perror(c"write".as_ptr());
    }
}

unsafe fn can_filters_setup(self_: *mut can_filters) {
    let mut addr: sockaddr_can = core::mem::zeroed();
    let mut ifr: ifreq = core::mem::zeroed();
    let recv_own_msgs: c_int = 1;
    let mut ret: c_int;

    let s = socket(PF_CAN, SOCK_RAW, CAN_RAW);
    ASSERT_GE_i32(s, 0);
    TH_LOG(c"failed to create CAN_RAW socket: %d".as_ptr(), errno);

    strncpy(
        ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
        CANIF.as_ptr(),
        size_of::<[c_char; IFNAMSIZ]>(),
    );
    ret = ioctl(s, SIOCGIFINDEX, &mut ifr);
    ASSERT_GE_i32(ret, 0);
    TH_LOG(c"failed SIOCGIFINDEX: %d".as_ptr(), errno);

    addr.can_family = AF_CAN as u16;
    addr.can_ifindex = ifr.ifr_ifru.ifru_ifindex;

    setsockopt(
        s,
        SOL_CAN_RAW,
        CAN_RAW_RECV_OWN_MSGS,
        &recv_own_msgs as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    );

    ret = bind(
        s,
        &addr as *const _ as *const sockaddr,
        size_of::<sockaddr_can>() as socklen_t,
    );
    ASSERT_EQ_i32(ret, 0);
    TH_LOG(c"failed bind socket: %d".as_ptr(), errno);

    (*self_).sock = s;
}

unsafe fn can_filters_teardown(self_: *mut can_filters) {
    close((*self_).sock);
}

/* Receive all frames when filtering for the ID in standard frame format */
static BASE_EXP_FLAGS: [canid_t; 4] = [0, CAN_RTR_FLAG, CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static BASE: can_filters_variant = can_filters_variant {
    testcase: 1,
    id: ID,
    mask: CAN_SFF_MASK,
    exp_num_rx: 4,
    exp_flags: &BASE_EXP_FLAGS,
};

/* Ignore EFF flag in filter ID if not covered by filter mask */
static BASE_EFF_EXP_FLAGS: [canid_t; 4] = [0, CAN_RTR_FLAG, CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static BASE_EFF: can_filters_variant = can_filters_variant {
    testcase: 2,
    id: ID | CAN_EFF_FLAG,
    mask: CAN_SFF_MASK,
    exp_num_rx: 4,
    exp_flags: &BASE_EFF_EXP_FLAGS,
};

/* Ignore RTR flag in filter ID if not covered by filter mask */
static BASE_RTR_EXP_FLAGS: [canid_t; 4] = [0, CAN_RTR_FLAG, CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static BASE_RTR: can_filters_variant = can_filters_variant {
    testcase: 3,
    id: ID | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK,
    exp_num_rx: 4,
    exp_flags: &BASE_RTR_EXP_FLAGS,
};

/* Ignore EFF and RTR flags in filter ID if not covered by filter mask */
static BASE_EFFRTR_EXP_FLAGS: [canid_t; 4] = [0, CAN_RTR_FLAG, CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static BASE_EFFRTR: can_filters_variant = can_filters_variant {
    testcase: 4,
    id: ID | CAN_EFF_FLAG | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK,
    exp_num_rx: 4,
    exp_flags: &BASE_EFFRTR_EXP_FLAGS,
};

/* Receive only SFF frames when expecting no EFF flag */
static FILTER_EFF_EXP_FLAGS: [canid_t; 2] = [0, CAN_RTR_FLAG];
static FILTER_EFF: can_filters_variant = can_filters_variant {
    testcase: 5,
    id: ID,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_EFF_EXP_FLAGS,
};

/* Receive only EFF frames when filter id and filter mask include EFF flag */
static FILTER_EFF_EFF_EXP_FLAGS: [canid_t; 2] = [CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static FILTER_EFF_EFF: can_filters_variant = can_filters_variant {
    testcase: 6,
    id: ID | CAN_EFF_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_EFF_EFF_EXP_FLAGS,
};

/* Receive only SFF frames when expecting no EFF flag, ignoring RTR flag */
static FILTER_EFF_RTR_EXP_FLAGS: [canid_t; 2] = [0, CAN_RTR_FLAG];
static FILTER_EFF_RTR: can_filters_variant = can_filters_variant {
    testcase: 7,
    id: ID | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_EFF_RTR_EXP_FLAGS,
};

/* Receive only EFF frames when filter id and filter mask include EFF flag,
 * ignoring RTR flag
 */
static FILTER_EFF_EFFRTR_EXP_FLAGS: [canid_t; 2] = [CAN_EFF_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static FILTER_EFF_EFFRTR: can_filters_variant = can_filters_variant {
    testcase: 8,
    id: ID | CAN_EFF_FLAG | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_EFF_EFFRTR_EXP_FLAGS,
};

/* Receive no remote frames when filtering for no RTR flag */
static FILTER_RTR_EXP_FLAGS: [canid_t; 2] = [0, CAN_EFF_FLAG];
static FILTER_RTR: can_filters_variant = can_filters_variant {
    testcase: 9,
    id: ID,
    mask: CAN_SFF_MASK | CAN_RTR_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_RTR_EXP_FLAGS,
};

/* Receive no remote frames when filtering for no RTR flag, ignoring EFF flag */
static FILTER_RTR_EFF_EXP_FLAGS: [canid_t; 2] = [0, CAN_EFF_FLAG];
static FILTER_RTR_EFF: can_filters_variant = can_filters_variant {
    testcase: 10,
    id: ID | CAN_EFF_FLAG,
    mask: CAN_SFF_MASK | CAN_RTR_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_RTR_EFF_EXP_FLAGS,
};

/* Receive only remote frames when filter includes RTR flag */
static FILTER_RTR_RTR_EXP_FLAGS: [canid_t; 2] = [CAN_RTR_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static FILTER_RTR_RTR: can_filters_variant = can_filters_variant {
    testcase: 11,
    id: ID | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_RTR_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_RTR_RTR_EXP_FLAGS,
};

/* Receive only remote frames when filter includes RTR flag, ignoring EFF
 * flag
 */
static FILTER_RTR_EFFRTR_EXP_FLAGS: [canid_t; 2] = [CAN_RTR_FLAG, CAN_EFF_FLAG | CAN_RTR_FLAG];
static FILTER_RTR_EFFRTR: can_filters_variant = can_filters_variant {
    testcase: 12,
    id: ID | CAN_EFF_FLAG | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_RTR_FLAG,
    exp_num_rx: 2,
    exp_flags: &FILTER_RTR_EFFRTR_EXP_FLAGS,
};

/* Receive only SFF data frame when filtering for no flags */
static FILTER_EFFRTR_EXP_FLAGS: [canid_t; 1] = [0];
static FILTER_EFFRTR: can_filters_variant = can_filters_variant {
    testcase: 13,
    id: ID,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &FILTER_EFFRTR_EXP_FLAGS,
};

/* Receive only EFF data frame when filtering for EFF but no RTR flag */
static FILTER_EFFRTR_EFF_EXP_FLAGS: [canid_t; 1] = [CAN_EFF_FLAG];
static FILTER_EFFRTR_EFF: can_filters_variant = can_filters_variant {
    testcase: 14,
    id: ID | CAN_EFF_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &FILTER_EFFRTR_EFF_EXP_FLAGS,
};

/* Receive only SFF remote frame when filtering for RTR but no EFF flag */
static FILTER_EFFRTR_RTR_EXP_FLAGS: [canid_t; 1] = [CAN_RTR_FLAG];
static FILTER_EFFRTR_RTR: can_filters_variant = can_filters_variant {
    testcase: 15,
    id: ID | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &FILTER_EFFRTR_RTR_EXP_FLAGS,
};

/* Receive only EFF remote frame when filtering for EFF and RTR flag */
static FILTER_EFFRTR_EFFRTR_EXP_FLAGS: [canid_t; 1] = [CAN_EFF_FLAG | CAN_RTR_FLAG];
static FILTER_EFFRTR_EFFRTR: can_filters_variant = can_filters_variant {
    testcase: 16,
    id: ID | CAN_EFF_FLAG | CAN_RTR_FLAG,
    mask: CAN_SFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &FILTER_EFFRTR_EFFRTR_EXP_FLAGS,
};

/* Receive only SFF data frame when filtering for no EFF flag and no RTR flag
 * but based on EFF mask
 */
static EFF_EXP_FLAGS: [canid_t; 1] = [0];
static EFF: can_filters_variant = can_filters_variant {
    testcase: 17,
    id: ID,
    mask: CAN_EFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &EFF_EXP_FLAGS,
};

/* Receive only EFF data frame when filtering for EFF flag and no RTR flag but
 * based on EFF mask
 */
static EFF_EFF_EXP_FLAGS: [canid_t; 1] = [CAN_EFF_FLAG];
static EFF_EFF: can_filters_variant = can_filters_variant {
    testcase: 18,
    id: ID | CAN_EFF_FLAG,
    mask: CAN_EFF_MASK | CAN_EFF_FLAG | CAN_RTR_FLAG,
    exp_num_rx: 1,
    exp_flags: &EFF_EFF_EXP_FLAGS,
};

static CAN_FILTERS_VARIANTS: [&can_filters_variant; 18] = [
    &BASE,
    &BASE_EFF,
    &BASE_RTR,
    &BASE_EFFRTR,
    &FILTER_EFF,
    &FILTER_EFF_EFF,
    &FILTER_EFF_RTR,
    &FILTER_EFF_EFFRTR,
    &FILTER_RTR,
    &FILTER_RTR_EFF,
    &FILTER_RTR_RTR,
    &FILTER_RTR_EFFRTR,
    &FILTER_EFFRTR,
    &FILTER_EFFRTR_EFF,
    &FILTER_EFFRTR_RTR,
    &FILTER_EFFRTR_EFFRTR,
    &EFF,
    &EFF_EFF,
];

/* This test verifies that the raw CAN filters work, by checking if only frames
 * with the expected set of flags are received. For each test case, the given
 * filter (id and mask) is added and four CAN frames are sent with every
 * combination of set/unset EFF/RTR flags.
 */
unsafe fn test_filter(self_: *mut can_filters, variant: *const can_filters_variant) {
    let mut rfilter: can_filter = core::mem::zeroed();
    let mut ret: c_int;

    rfilter.can_id = (*variant).id;
    rfilter.can_mask = (*variant).mask;
    setsockopt(
        (*self_).sock,
        SOL_CAN_RAW,
        CAN_RAW_FILTER,
        &rfilter as *const _ as *const c_void,
        size_of::<can_filter>() as socklen_t,
    );

    TH_LOG(
        c"filters: can_id = 0x%08X can_mask = 0x%08X".as_ptr(),
        rfilter.can_id,
        rfilter.can_mask,
    );

    ret = send_can_frames((*self_).sock, (*variant).testcase);
    ASSERT_EQ_i32(ret, 0);
    TH_LOG(c"failed to send CAN frames".as_ptr());

    let mut i: c_int = 0;
    while i <= (*variant).exp_num_rx {
        let mut frame: can_frame = core::mem::zeroed();
        let mut tv = timeval {
            tv_sec: 0,
            tv_usec: 50000, /* 50ms timeout */
        };
        let mut rdfs: fd_set = core::mem::zeroed();

        FD_ZERO(&mut rdfs);
        FD_SET((*self_).sock, &mut rdfs);

        ret = select(
            (*self_).sock + 1,
            &mut rdfs,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut tv,
        );
        ASSERT_GE_i32(ret, 0);
        TH_LOG(c"failed select for frame %d, err: %d)".as_ptr(), i, errno);

        ret = FD_ISSET((*self_).sock, &mut rdfs);
        if i == (*variant).exp_num_rx {
            ASSERT_EQ_i32(ret, 0);
            TH_LOG(c"too many frames received".as_ptr());
        } else {
            ASSERT_NE_i32(ret, 0);
            TH_LOG(c"too few frames received".as_ptr());

            ret = read(
                (*self_).sock,
                &mut frame as *mut _ as *mut c_void,
                size_of::<can_frame>(),
            ) as c_int;
            ASSERT_GE_i32(ret, 0);
            TH_LOG(c"failed to read frame %d, err: %d".as_ptr(), i, errno);

            TH_LOG(c"rx: can_id = 0x%08X rx = %d".as_ptr(), frame.can_id, i);

            ASSERT_EQ_u32(ID, frame.can_id & CAN_SFF_MASK);
            TH_LOG(c"received wrong can_id".as_ptr());
            ASSERT_EQ_i32((*variant).testcase, frame.data[0] as c_int);
            TH_LOG(c"received wrong test case".as_ptr());

            ASSERT_EQ_u32(
                frame.can_id & !CAN_ERR_MASK,
                (*variant).exp_flags[i as usize],
            );
            TH_LOG(c"received unexpected flags".as_ptr());
        }

        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let ifname = getenv(c"CANIF".as_ptr());

    if ifname.is_null() {
        printf(c"CANIF environment variable must contain the test interface\n".as_ptr());
        return KSFT_FAIL;
    }

    strncpy(CANIF.as_mut_ptr(), ifname, size_of::<[c_char; IFNAMSIZ]>() - 1);

    test_harness_run(argc, argv)
}
