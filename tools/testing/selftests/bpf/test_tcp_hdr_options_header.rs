/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2020 Facebook */

#[repr(C, packed)]
pub struct bpf_test_option {
    pub flags: u8,
    pub max_delack_ms: u8,
    pub rand: u8,
}

pub const OPTION_RESEND: u32 = 0;
pub const OPTION_MAX_DELACK_MS: u32 = 1;
pub const OPTION_RAND: u32 = 2;
pub const __NR_OPTION_FLAGS: u32 = 3;

pub const OPTION_F_RESEND: u32 = 1 << OPTION_RESEND;
pub const OPTION_F_MAX_DELACK_MS: u32 = 1 << OPTION_MAX_DELACK_MS;
pub const OPTION_F_RAND: u32 = 1 << OPTION_RAND;
pub const OPTION_MASK: u32 = (1 << __NR_OPTION_FLAGS) - 1;

#[inline]
pub const fn TEST_OPTION_FLAGS(flags: u32, option: u32) -> u32 {
    1 & (flags >> option)
}

#[inline]
pub fn SET_OPTION_FLAGS(flags: &mut u32, option: u32) {
    *flags |= 1 << option;
}

/* Store in bpf_sk_storage */
#[repr(C)]
pub struct hdr_stg {
    pub active: bool,
    pub resend_syn: bool, /* active side only */
    pub syncookie: bool,  /* passive side only */
    pub fastopen: bool,   /* passive side only */
}

#[repr(C)]
pub struct linum_err {
    pub linum: u32,
    pub err: i32,
}

pub const TCPHDR_FIN: u32 = 0x01;
pub const TCPHDR_SYN: u32 = 0x02;
pub const TCPHDR_RST: u32 = 0x04;
pub const TCPHDR_PSH: u32 = 0x08;
pub const TCPHDR_ACK: u32 = 0x10;
pub const TCPHDR_URG: u32 = 0x20;
pub const TCPHDR_ECE: u32 = 0x40;
pub const TCPHDR_CWR: u32 = 0x80;
pub const TCPHDR_SYNACK: u32 = TCPHDR_SYN | TCPHDR_ACK;

pub const TCPOPT_EOL: u32 = 0;
pub const TCPOPT_NOP: u32 = 1;
pub const TCPOPT_MSS: u32 = 2;
pub const TCPOPT_WINDOW: u32 = 3;
pub const TCPOPT_EXP: u32 = 254;

pub const TCP_BPF_EXPOPT_BASE_LEN: u32 = 4;
pub const MAX_TCP_HDR_LEN: u32 = 60;
pub const MAX_TCP_OPTION_SPACE: u32 = 40;

/*
 * The following items are conditionally present in C when
 * BPF_PROG_TEST_TCP_HDR_OPTIONS is defined.
 */

pub const CG_OK: i32 = 1;
pub const CG_ERR: i32 = 0;

/* Defined by the C header only when missing. */
pub const SOL_TCP: u32 = 6;

#[repr(C)]
pub union tcp_exprm_opt_anon {
    pub data: [u8; 4],
    pub data32: u32,
}

#[repr(C, packed)]
pub struct tcp_exprm_opt {
    pub kind: u8,
    pub len: u8,
    pub magic: u16,
    pub u: tcp_exprm_opt_anon,
}

#[repr(C)]
pub union tcp_opt_anon {
    pub data: [u8; 4],
    pub data32: u32,
}

#[repr(C, packed)]
pub struct tcp_opt {
    pub kind: u8,
    pub len: u8,
    pub u: tcp_opt_anon,
}

/*
 * C BPF map declaration:
 *
 * struct {
 *     __uint(type, BPF_MAP_TYPE_HASH);
 *     __uint(max_entries, 2);
 *     __type(key, int);
 *     __type(value, struct linum_err);
 * } lport_linum_map SEC(".maps");
 *
 * The __uint/__type/SEC macros and the concrete map representation are supplied
 * by BPF headers/build tooling outside this isolated header.
 */
unsafe extern "C" {
    pub static mut lport_linum_map: core::ffi::c_void;
}

#[inline]
pub unsafe fn tcp_hdrlen(th: *const tcphdr) -> u32 {
    ((*th).doff as u32) << 2
}

#[inline]
pub unsafe fn skops_tcp_flags(skops: *const bpf_sock_ops) -> u8 {
    (*skops).skb_tcp_flags
}

#[inline]
pub unsafe fn clear_hdr_cb_flags(skops: *mut bpf_sock_ops) {
    bpf_sock_ops_cb_flags_set(
        skops,
        (*skops).bpf_sock_ops_cb_flags
            & !(BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG
                | BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG),
    );
}

#[inline]
pub unsafe fn set_hdr_cb_flags(skops: *mut bpf_sock_ops, extra: u32) {
    bpf_sock_ops_cb_flags_set(
        skops,
        (*skops).bpf_sock_ops_cb_flags
            | BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG
            | BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG
            | extra,
    );
}

#[inline]
pub unsafe fn clear_parse_all_hdr_cb_flags(skops: *mut bpf_sock_ops) {
    bpf_sock_ops_cb_flags_set(
        skops,
        (*skops).bpf_sock_ops_cb_flags & !BPF_SOCK_OPS_PARSE_ALL_HDR_OPT_CB_FLAG,
    );
}

#[inline]
pub unsafe fn set_parse_all_hdr_cb_flags(skops: *mut bpf_sock_ops) {
    bpf_sock_ops_cb_flags_set(
        skops,
        (*skops).bpf_sock_ops_cb_flags | BPF_SOCK_OPS_PARSE_ALL_HDR_OPT_CB_FLAG,
    );
}

/*
 * C macro RET_CG_ERR(__err) expands to a statement expression that records the
 * source line, stores the error by local port in lport_linum_map, clears header
 * callback flags, and returns CG_ERR from the caller. Rust has no direct
 * statement-expression macro equivalent with __LINE__ assignment and caller
 * return semantics, so this macro preserves the source-level behavior for Rust
 * call sites.
 */
#[macro_export]
macro_rules! RET_CG_ERR {
    ($skops:expr, $__err:expr) => {{
        let mut __linum_err = $crate::linum_err {
            linum: line!(),
            err: $__err,
        };
        let mut __lport = unsafe { (*$skops).local_port };
        unsafe {
            bpf_map_update_elem(
                core::ptr::addr_of_mut!($crate::lport_linum_map).cast(),
                core::ptr::addr_of_mut!(__lport).cast(),
                core::ptr::addr_of_mut!(__linum_err).cast(),
                BPF_NOEXIST,
            );
            $crate::clear_hdr_cb_flags($skops);
            $crate::clear_parse_all_hdr_cb_flags($skops);
        }
        return $crate::CG_ERR;
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
