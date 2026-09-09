// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2015 Broadcom Corporation

// External dependency: Linux kernel device-tree node type.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// External dependency supplied by clk-iproc.
extern "C" {
    fn iproc_armpll_setup(node: *mut device_node);
}

unsafe fn bcm63138_armpll_init(node: *mut device_node) {
    iproc_armpll_setup(node);
}

// CLK_OF_DECLARE(bcm63138_armpll, "brcm,bcm63138-armpll", bcm63138_armpll_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
