/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Types from <linux/types.h> are supplied by the surrounding translation.

/* Just some random number */
pub const TCPDIAG_GETSOCK: i32 = 18;
pub const DCCPDIAG_GETSOCK: i32 = 19;
pub const INET_DIAG_GETSOCK_MAX: i32 = 24;

/* Socket identity */
#[repr(C)]
pub struct inet_diag_sockid {
    pub idiag_sport: __be16,
    pub idiag_dport: __be16,
    pub idiag_src: [__be32; 4],
    pub idiag_dst: [__be32; 4],
    pub idiag_if: __u32,
    pub idiag_cookie: [__u32; 2],
}

pub const INET_DIAG_NOCOOKIE: __u32 = !0u32;

/* Request structure */
#[repr(C)]
pub struct inet_diag_req {
    pub idiag_family: __u8,
    pub idiag_src_len: __u8,
    pub idiag_dst_len: __u8,
    pub idiag_ext: __u8, // Family of addresses / query extended information
    pub id: inet_diag_sockid,
    pub idiag_states: __u32, // States to dump
    pub idiag_dbs: __u32, // Tables to dump (NI)
}

#[repr(C)]
pub struct inet_diag_req_v2 {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub idiag_ext: __u8,
    pub pad: __u8,
    pub idiag_states: __u32,
    pub id: inet_diag_sockid,
}

/* Alias layout for struct inet_diag_req_v2, with the raw protocol specified. */
#[repr(C)]
pub struct inet_diag_req_raw {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub idiag_ext: __u8,
    pub sdiag_raw_protocol: __u8,
    pub idiag_states: __u32,
    pub id: inet_diag_sockid,
}

pub const INET_DIAG_REQ_NONE: i32 = 0;
pub const INET_DIAG_REQ_BYTECODE: i32 = 1;
pub const INET_DIAG_REQ_SK_BPF_STORAGES: i32 = 2;
pub const INET_DIAG_REQ_PROTOCOL: i32 = 3;
pub const __INET_DIAG_REQ_MAX: i32 = 4;
pub const INET_DIAG_REQ_MAX: i32 = __INET_DIAG_REQ_MAX - 1;

/* Bytecode is sequence of 4 byte commands followed by variable arguments. */
#[repr(C)]
pub struct inet_diag_bc_op {
    pub code: core::ffi::c_uchar,
    pub yes: core::ffi::c_uchar,
    pub no: core::ffi::c_ushort,
}

pub const INET_DIAG_BC_NOP: i32 = 0;
pub const INET_DIAG_BC_JMP: i32 = 1;
pub const INET_DIAG_BC_S_GE: i32 = 2;
pub const INET_DIAG_BC_S_LE: i32 = 3;
pub const INET_DIAG_BC_D_GE: i32 = 4;
pub const INET_DIAG_BC_D_LE: i32 = 5;
pub const INET_DIAG_BC_AUTO: i32 = 6;
pub const INET_DIAG_BC_S_COND: i32 = 7;
pub const INET_DIAG_BC_D_COND: i32 = 8;
pub const INET_DIAG_BC_DEV_COND: i32 = 9;
pub const INET_DIAG_BC_MARK_COND: i32 = 10;
pub const INET_DIAG_BC_S_EQ: i32 = 11;
pub const INET_DIAG_BC_D_EQ: i32 = 12;
pub const INET_DIAG_BC_CGROUP_COND: i32 = 13;

#[repr(C)]
pub struct inet_diag_hostcond {
    pub family: __u8,
    pub prefix_len: __u8,
    pub port: core::ffi::c_int,
    pub addr: [__be32; 0],
}

#[repr(C)]
pub struct inet_diag_markcond {
    pub mark: __u32,
    pub mask: __u32,
}

/* Base info structure. It contains socket identity and netstat information. */
#[repr(C)]
pub struct inet_diag_msg {
    pub idiag_family: __u8,
    pub idiag_state: __u8,
    pub idiag_timer: __u8,
    pub idiag_retrans: __u8,
    pub id: inet_diag_sockid,
    pub idiag_expires: __u32,
    pub idiag_rqueue: __u32,
    pub idiag_wqueue: __u32,
    pub idiag_uid: __u32,
    pub idiag_inode: __u32,
}

pub const IDIAG_TIMER_OFF: i32 = 0;
pub const IDIAG_TIMER_ON: i32 = 1;
pub const IDIAG_TIMER_KEEPALIVE: i32 = 2;
pub const IDIAG_TIMER_TIMEWAIT: i32 = 3;
pub const IDIAG_TIMER_PROBE0: i32 = 4;
pub const IDIAG_TIMER_DELACK: i32 = 5;

pub const INET_DIAG_NONE: i32 = 0;
pub const INET_DIAG_MEMINFO: i32 = 1;
pub const INET_DIAG_INFO: i32 = 2;
pub const INET_DIAG_VEGASINFO: i32 = 3;
pub const INET_DIAG_CONG: i32 = 4;
pub const INET_DIAG_TOS: i32 = 5;
pub const INET_DIAG_TCLASS: i32 = 6;
pub const INET_DIAG_SKMEMINFO: i32 = 7;
pub const INET_DIAG_SHUTDOWN: i32 = 8;
pub const INET_DIAG_DCTCPINFO: i32 = 9;
pub const INET_DIAG_PROTOCOL: i32 = 10;
pub const INET_DIAG_SKV6ONLY: i32 = 11;
pub const INET_DIAG_LOCALS: i32 = 12;
pub const INET_DIAG_PEERS: i32 = 13;
pub const INET_DIAG_PAD: i32 = 14;
pub const INET_DIAG_MARK: i32 = 15;
pub const INET_DIAG_BBRINFO: i32 = 16;
pub const INET_DIAG_CLASS_ID: i32 = 17;
pub const INET_DIAG_MD5SIG: i32 = 18;
pub const INET_DIAG_ULP_INFO: i32 = 19;
pub const INET_DIAG_SK_BPF_STORAGES: i32 = 20;
pub const INET_DIAG_CGROUP_ID: i32 = 21;
pub const INET_DIAG_SOCKOPT: i32 = 22;
pub const __INET_DIAG_MAX: i32 = 23;
pub const INET_DIAG_MAX: i32 = __INET_DIAG_MAX - 1;

pub const INET_ULP_INFO_UNSPEC: i32 = 0;
pub const INET_ULP_INFO_NAME: i32 = 1;
pub const INET_ULP_INFO_TLS: i32 = 2;
pub const INET_ULP_INFO_MPTCP: i32 = 3;
pub const __INET_ULP_INFO_MAX: i32 = 4;
pub const INET_ULP_INFO_MAX: i32 = __INET_ULP_INFO_MAX - 1;

#[repr(C)]
pub struct inet_diag_meminfo {
    pub idiag_rmem: __u32,
    pub idiag_wmem: __u32,
    pub idiag_fmem: __u32,
    pub idiag_tmem: __u32,
}

/* C bitfields occupy the two bytes below; use masks to access individual bits. */
#[repr(C)]
pub struct inet_diag_sockopt {
    pub first: __u8,
    pub second: __u8,
}

#[repr(C)]
pub struct tcpvegas_info {
    pub tcpv_enabled: __u32,
    pub tcpv_rttcnt: __u32,
    pub tcpv_rtt: __u32,
    pub tcpv_minrtt: __u32,
}

#[repr(C)]
pub struct tcp_dctcp_info {
    pub dctcp_enabled: __u16,
    pub dctcp_ce_state: __u16,
    pub dctcp_alpha: __u32,
    pub dctcp_ab_ecn: __u32,
    pub dctcp_ab_tot: __u32,
}

#[repr(C)]
pub struct tcp_bbr_info {
    pub bbr_bw_lo: __u32,
    pub bbr_bw_hi: __u32,
    pub bbr_min_rtt: __u32,
    pub bbr_pacing_gain: __u32,
    pub bbr_cwnd_gain: __u32,
}

#[repr(C)]
pub union tcp_cc_info {
    pub vegas: tcpvegas_info,
    pub dctcp: tcp_dctcp_info,
    pub bbr: tcp_bbr_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
