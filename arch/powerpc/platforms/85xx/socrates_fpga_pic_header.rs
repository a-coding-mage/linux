/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2008 Ilya Yanok, Emcraft Systems
 */

// C header guard: SOCRATES_FPGA_PIC_H
// `__init` is a kernel build attribute with no direct Rust equivalent.

extern "C" {
    pub fn socrates_fpga_pic_init(pic: *mut device_node);
}

// Supplied by the corresponding external dependency.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
