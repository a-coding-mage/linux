/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by <linux/types.h> in the original header.

macro_rules! tcpoptstrip_set_bit {
    ($bmap:expr, $idx:expr) => {
        $bmap[($idx) >> 5] |= 1u32 << (($idx) & 31)
    };
}

macro_rules! tcpoptstrip_test_bit {
    ($bmap:expr, $idx:expr) => {
        (((1u32 << (($idx) & 31)) & $bmap[($idx) >> 5]) != 0)
    };
}

#[repr(C)]
struct xt_tcpoptstrip_target_info {
    strip_bmap: [__u32; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
