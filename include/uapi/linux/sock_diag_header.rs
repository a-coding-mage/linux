/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The original header includes <linux/types.h> for the __u8 type.

pub const SOCK_DIAG_BY_FAMILY: i32 = 20;
pub const SOCK_DESTROY: i32 = 21;

#[repr(C)]
pub struct sock_diag_req {
    pub sdiag_family: u8,
    pub sdiag_protocol: u8,
}

pub const SK_MEMINFO_RMEM_ALLOC: i32 = 0;
pub const SK_MEMINFO_RCVBUF: i32 = 1;
pub const SK_MEMINFO_WMEM_ALLOC: i32 = 2;
pub const SK_MEMINFO_SNDBUF: i32 = 3;
pub const SK_MEMINFO_FWD_ALLOC: i32 = 4;
pub const SK_MEMINFO_WMEM_QUEUED: i32 = 5;
pub const SK_MEMINFO_OPTMEM: i32 = 6;
pub const SK_MEMINFO_BACKLOG: i32 = 7;
pub const SK_MEMINFO_DROPS: i32 = 8;
pub const SK_MEMINFO_VARS: i32 = 9;

pub const SKNLGRP_NONE: i32 = 0;
pub const SKNLGRP_INET_TCP_DESTROY: i32 = 1;
pub const SKNLGRP_INET_UDP_DESTROY: i32 = 2;
pub const SKNLGRP_INET6_TCP_DESTROY: i32 = 3;
pub const SKNLGRP_INET6_UDP_DESTROY: i32 = 4;
pub const __SKNLGRP_MAX: i32 = 5;
pub const SKNLGRP_MAX: i32 = __SKNLGRP_MAX - 1;

pub const SK_DIAG_BPF_STORAGE_REQ_NONE: i32 = 0;
pub const SK_DIAG_BPF_STORAGE_REQ_MAP_FD: i32 = 1;
pub const __SK_DIAG_BPF_STORAGE_REQ_MAX: i32 = 2;
pub const SK_DIAG_BPF_STORAGE_REQ_MAX: i32 = __SK_DIAG_BPF_STORAGE_REQ_MAX - 1;

pub const SK_DIAG_BPF_STORAGE_REP_NONE: i32 = 0;
pub const SK_DIAG_BPF_STORAGE: i32 = 1;
pub const __SK_DIAG_BPF_STORAGE_REP_MAX: i32 = 2;
pub const SK_DIAB_BPF_STORAGE_REP_MAX: i32 = __SK_DIAG_BPF_STORAGE_REP_MAX - 1;

pub const SK_DIAG_BPF_STORAGE_NONE: i32 = 0;
pub const SK_DIAG_BPF_STORAGE_PAD: i32 = 1;
pub const SK_DIAG_BPF_STORAGE_MAP_ID: i32 = 2;
pub const SK_DIAG_BPF_STORAGE_MAP_VALUE: i32 = 3;
pub const __SK_DIAG_BPF_STORAGE_MAX: i32 = 4;
pub const SK_DIAG_BPF_STORAGE_MAX: i32 = __SK_DIAG_BPF_STORAGE_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
