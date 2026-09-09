// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Realtek RTD1195
 *
 * Copyright (c) 2017-2019 Andreas Färber
 */

use core::ffi::{c_char, c_void};

// Supplied by Linux kernel dependencies.
type PhysAddr = u64;

extern "C" {
    fn memblock_remove(base: PhysAddr, size: PhysAddr) -> i32;
    fn pr_err(fmt: *const c_char, ...) -> c_void;
}

unsafe fn rtd1195_memblock_remove(base: PhysAddr, size: PhysAddr) {
    let ret: i32 = memblock_remove(base, size);
    if ret != 0 {
        // The kernel's %pa conversion expects a pointer to the physical address.
        static FORMAT: &[u8] = b"Failed to remove memblock %pa (%d)\n\0";
        pr_err(FORMAT.as_ptr() as *const c_char, &base as *const PhysAddr, ret);
    }
}

unsafe fn rtd1195_reserve() {
    /* Exclude boot ROM from RAM */
    rtd1195_memblock_remove(0x0000_0000, 0x0000_a800);

    /* Exclude peripheral register spaces from RAM */
    rtd1195_memblock_remove(0x1800_0000, 0x0007_0000);
    rtd1195_memblock_remove(0x1810_0000, 0x0100_0000);
}

static RTD1195_DT_COMPAT: [*const c_char; 2] = [
    b"realtek,rtd1195\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Translation of DT_MACHINE_START(rtd1195, "Realtek RTD1195") ... MACHINE_END.
// The machine descriptor type and registration mechanism are supplied by the
// architecture dependencies.
#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub dt_compat: *const *const c_char,
    pub reserve: unsafe fn(),
    pub l2c_aux_val: u32,
    pub l2c_aux_mask: u32,
}

#[no_mangle]
pub static RTD1195: MachineDesc = MachineDesc {
    name: b"Realtek RTD1195\0".as_ptr() as *const c_char,
    dt_compat: RTD1195_DT_COMPAT.as_ptr(),
    reserve: rtd1195_reserve,
    l2c_aux_val: 0x0,
    l2c_aux_mask: !0x0,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
