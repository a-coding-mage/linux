/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const NF_LOG_TCPSEQ: u32 = 0x01; /* Log TCP sequence numbers */
pub const NF_LOG_TCPOPT: u32 = 0x02; /* Log TCP options */
pub const NF_LOG_IPOPT: u32 = 0x04; /* Log IP options */
pub const NF_LOG_UID: u32 = 0x08; /* Log UID owning local socket */
pub const NF_LOG_NFLOG: u32 = 0x10; /* Unsupported, don't reuse */
pub const NF_LOG_MACDECODE: u32 = 0x20; /* Decode MAC header */
pub const NF_LOG_MASK: u32 = 0x2f;

pub const NF_LOG_PREFIXLEN: u32 = 128;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
