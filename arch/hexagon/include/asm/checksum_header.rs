/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// C header guard: _ASM_CHECKSUM_H

// C macro: do_csum do_csum
unsafe extern "C" {
    pub fn do_csum(voidptr: *const core::ffi::c_void, len: i32) -> u32;
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented
 */
// C macro: csum_tcpudp_nofold csum_tcpudp_nofold
unsafe extern "C" {
    pub fn csum_tcpudp_nofold(
        saddr: __be32,
        daddr: __be32,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __wsum;
}

// C macro: csum_tcpudp_magic csum_tcpudp_magic
unsafe extern "C" {
    pub fn csum_tcpudp_magic(
        saddr: __be32,
        daddr: __be32,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __sum16;
}

// Dependency intent preserved from: #include <asm-generic/checksum.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
