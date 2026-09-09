/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* make sure not to change this without changing netfilter.h:NF_LOG_* (!) */
pub const IPT_LOG_TCPSEQ: u32 = 0x01; // Log TCP sequence numbers
pub const IPT_LOG_TCPOPT: u32 = 0x02; // Log TCP options
pub const IPT_LOG_IPOPT: u32 = 0x04; // Log IP options
pub const IPT_LOG_UID: u32 = 0x08; // Log UID owning local socket
pub const IPT_LOG_NFLOG: u32 = 0x10; // Unsupported, don't reuse
pub const IPT_LOG_MACDECODE: u32 = 0x20; // Decode MAC header
pub const IPT_LOG_MASK: u32 = 0x2f;

#[repr(C)]
pub struct ipt_log_info {
    pub level: u8,
    pub logflags: u8,
    pub prefix: [::core::ffi::c_char; 30],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
