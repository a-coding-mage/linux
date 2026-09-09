/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Intel Corporation
 * Author: johannes@sipsolutions.net
 */

// This file is included into asm/io.h.
// The declarations below are active when CONFIG_INDIRECT_IOMEM is enabled.

#[cfg(CONFIG_INDIRECT_IOMEM)]
extern "C" {
    pub fn ioremap(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void;

    pub fn iounmap(addr: *mut core::ffi::c_void);

    pub fn __raw_readb(addr: *const core::ffi::c_void) -> u8;

    pub fn __raw_readw(addr: *const core::ffi::c_void) -> u16;

    pub fn __raw_readl(addr: *const core::ffi::c_void) -> u32;

    // CONFIG_64BIT
    #[cfg(CONFIG_64BIT)]
    pub fn __raw_readq(addr: *const core::ffi::c_void) -> u64;

    pub fn __raw_writeb(value: u8, addr: *mut core::ffi::c_void);

    pub fn __raw_writew(value: u16, addr: *mut core::ffi::c_void);

    pub fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);

    // CONFIG_64BIT
    #[cfg(CONFIG_64BIT)]
    pub fn __raw_writeq(value: u64, addr: *mut core::ffi::c_void);

    pub fn memset_io(addr: *mut core::ffi::c_void, value: core::ffi::c_int, size: usize);

    pub fn memcpy_fromio(
        buffer: *mut core::ffi::c_void,
        addr: *const core::ffi::c_void,
        size: usize,
    );

    pub fn memcpy_toio(
        addr: *mut core::ffi::c_void,
        buffer: *const core::ffi::c_void,
        size: usize,
    );
}

// The CONFIG_INDIRECT_IOMEM_FALLBACK checks in the C header require the
// corresponding real_* fallback symbols to be supplied by the including code.
// The C self-referential macros (for example, #define ioremap ioremap) are
// preprocessor declarations and have no separate Rust item equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
