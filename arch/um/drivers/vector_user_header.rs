/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C header guard: __UM_VECTOR_USER_H

pub const MAXVARGS: usize = 20;

pub const TOKEN_IFNAME: &str = "ifname";

pub const TRANS_RAW: &str = "raw";
pub const TRANS_RAW_LEN: usize = TRANS_RAW.len();

pub const TRANS_TAP: &str = "tap";
pub const TRANS_TAP_LEN: usize = TRANS_TAP.len();

pub const TRANS_GRE: &str = "gre";
pub const TRANS_GRE_LEN: usize = TRANS_GRE.len();

pub const TRANS_L2TPV3: &str = "l2tpv3";
pub const TRANS_L2TPV3_LEN: usize = TRANS_L2TPV3.len();

pub const TRANS_HYBRID: &str = "hybrid";
pub const TRANS_HYBRID_LEN: usize = TRANS_HYBRID.len();

pub const TRANS_BESS: &str = "bess";
pub const TRANS_BESS_LEN: usize = TRANS_BESS.len();

pub const DEFAULT_BPF_LEN: usize = 6;

// C conditional definition: preserve the externally supplied value when present.
#[cfg(not(any(IPPROTO_GRE)))]
pub const IPPROTO_GRE: u16 = 0x2F;

pub const GRE_MODE_CHECKSUM: u16 = cpu_to_be16(8 << 12); // checksum
pub const GRE_MODE_RESERVED: u16 = cpu_to_be16(4 << 12); // unused
pub const GRE_MODE_KEY: u16 = cpu_to_be16(2 << 12); // KEY present
pub const GRE_MODE_SEQUENCE: u16 = cpu_to_be16(1 << 12); // sequence

pub const GRE_IRB: u16 = cpu_to_be16(0x6558);

pub const L2TPV3_DATA_PACKET: u32 = 0x30000;

// IANA-assigned IP protocol ID for L2TPv3
// C conditional definition: preserve the externally supplied value when present.
#[cfg(not(any(IPPROTO_L2TP)))]
pub const IPPROTO_L2TP: u16 = 0x73;

#[repr(C)]
pub struct arglist {
    pub numargs: i32,
    pub tokens: [*mut core::ffi::c_char; MAXVARGS],
    pub values: [*mut core::ffi::c_char; MAXVARGS],
}

/* Separating read and write FDs allows us to have different
 * rx and tx method. Example - read tap via raw socket using
 * recvmmsg, write using legacy tap write calls
 */
#[repr(C)]
pub struct vector_fds {
    pub rx_fd: i32,
    pub tx_fd: i32,
    pub remote_addr: *mut core::ffi::c_void,
    pub remote_addr_size: i32,
}

pub const VECTOR_READ: i32 = 1;

extern "C" {
    pub fn uml_parse_vector_ifspec(arg: *mut core::ffi::c_char) -> *mut arglist;
    pub fn uml_vector_user_open(unit: i32, parsed: *mut arglist) -> *mut vector_fds;
    pub fn uml_vector_fetch_arg(
        ifspec: *mut arglist,
        token: *mut core::ffi::c_char,
    ) -> *mut core::ffi::c_char;
    pub fn uml_vector_recvmsg(fd: i32, hdr: *mut core::ffi::c_void, flags: i32) -> i32;
    pub fn uml_vector_sendmsg(fd: i32, hdr: *mut core::ffi::c_void, flags: i32) -> i32;
    pub fn uml_vector_writev(fd: i32, hdr: *mut core::ffi::c_void, iovcount: i32) -> i32;
    pub fn uml_vector_sendmmsg(
        fd: i32,
        msgvec: *mut core::ffi::c_void,
        vlen: u32,
        flags: u32,
    ) -> i32;
    pub fn uml_vector_recvmmsg(
        fd: i32,
        msgvec: *mut core::ffi::c_void,
        vlen: u32,
        flags: u32,
    ) -> i32;
    pub fn uml_vector_default_bpf(mac: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn uml_vector_user_bpf(filename: *mut core::ffi::c_char) -> *mut core::ffi::c_void;
    pub fn uml_vector_attach_bpf(fd: i32, bpf: *mut core::ffi::c_void) -> i32;
    pub fn uml_vector_detach_bpf(fd: i32, bpf: *mut core::ffi::c_void) -> i32;
    pub fn uml_raw_enable_qdisc_bypass(fd: i32) -> bool;
    pub fn uml_raw_enable_vnet_headers(fd: i32) -> bool;
    pub fn uml_tap_enable_vnet_headers(fd: i32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
