/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the original header:
// #include <linux/types.h>
// #include <linux/pkt_cls.h>

#[repr(C)]
pub struct tcf_em_nbyte {
    pub off: u16,
    // Original declaration: __u16 len:12;
    pub len: u16,
    // Original declaration: __u8 layer:4;
    pub layer: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
