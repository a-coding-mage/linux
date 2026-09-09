// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2017 Broadcom

// Dependencies supplied by the Linux kernel and clk-iproc.h are intentionally
// left as external Rust symbols/types.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn iproc_armpll_setup(node: *mut device_node);
}

unsafe fn hr2_armpll_init(node: *mut device_node) {
    iproc_armpll_setup(node);
}

// CLK_OF_DECLARE(hr2_armpll, "brcm,hr2-armpll", hr2_armpll_init);
// Device-tree initialization registration for hr2_armpll is provided by the
// surrounding kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
