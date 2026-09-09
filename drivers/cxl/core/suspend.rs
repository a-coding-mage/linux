// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */

// Dependencies supplied by the corresponding Linux headers and cxlmem.h.

static mut mem_active: atomic_t = atomic_t { counter: 0 };

pub unsafe fn cxl_mem_active() -> bool {
    atomic_read(&mem_active) != 0
}

pub unsafe fn cxl_mem_active_inc() {
    atomic_inc(&mut mem_active);
}

// EXPORT_SYMBOL_NS_GPL(cxl_mem_active_inc, "CXL");

pub unsafe fn cxl_mem_active_dec() {
    atomic_dec(&mut mem_active);
}

// EXPORT_SYMBOL_NS_GPL(cxl_mem_active_dec, "CXL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
