// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2024 Linus Walleij <linus.walleij@linaro.org>

// Translated from the C implementation.  The following names and types are
// supplied by the architecture headers and kernel build configuration.

use core::ffi::c_char;

#[repr(C)]
pub struct map_desc {
    pub virtual_address: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: usize,
}

extern "C" {
    fn __phys_to_pfn(phys: usize) -> usize;
    fn iotable_init(io_desc: *mut map_desc, nr: usize);

    static CONFIG_DEBUG_UART_VIRT: usize;
    static CONFIG_DEBUG_UART_PHYS: usize;
    static SZ_4K: usize;
    static MT_DEVICE: usize;
}

/* This is needed for LL-debug/earlyprintk/debug-macro.S */
#[no_mangle]
pub static mut bcmbca_io_desc: [map_desc; 1] = [map_desc {
    virtual_address: 0,
    pfn: 0,
    length: 0,
    type_: 0,
}];

#[inline(never)]
unsafe fn bcmbca_initialize_io_desc() {
    bcmbca_io_desc[0].virtual_address = CONFIG_DEBUG_UART_VIRT;
    bcmbca_io_desc[0].pfn = __phys_to_pfn(CONFIG_DEBUG_UART_PHYS);
    bcmbca_io_desc[0].length = SZ_4K;
    bcmbca_io_desc[0].type_ = MT_DEVICE;
}

unsafe fn bcmbca_map_io() {
    bcmbca_initialize_io_desc();
    iotable_init(bcmbca_io_desc.as_mut_ptr(), bcmbca_io_desc.len());
}

static BCMBCA_DT_COMPAT: [*const c_char; 2] = [
    // TODO: Add other BCMBCA SoCs here to get debug UART support
    b"brcm,bcm6846\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(BCMBCA_DT, "BCMBCA Broadband Access Processors")
// .map_io = bcmbca_map_io,
// .dt_compat = bcmbca_dt_compat,
// MACHINE_END
#[repr(C)]
pub struct machine_desc {
    pub map_io: unsafe fn(),
    pub dt_compat: *const *const c_char,
}

#[no_mangle]
pub static BCMBCA_DT: machine_desc = machine_desc {
    map_io: bcmbca_map_io,
    dt_compat: BCMBCA_DT_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
