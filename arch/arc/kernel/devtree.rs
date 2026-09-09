// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Based on reduced version of METAG
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/reboot.h, linux/memblock.h, linux/of.h,
// linux/of_fdt.h, asm/mach_desc.h, and asm/serial.h.

#[cfg(feature = "CONFIG_SERIAL_EARLYCON")]
static mut arc_base_baud: ::core::ffi::c_uint = 0;

#[cfg(feature = "CONFIG_SERIAL_EARLYCON")]
pub unsafe fn arc_early_base_baud() -> ::core::ffi::c_uint {
    arc_base_baud / 16
}

#[cfg(feature = "CONFIG_SERIAL_EARLYCON")]
unsafe fn arc_set_early_base_baud(dt_root: ::core::ffi::c_ulong) {
    if of_flat_dt_is_compatible(dt_root, b"abilis,arc-tb10x\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
        arc_base_baud = 166666666; /* Fixed 166.6MHz clk (TB10x) */
    } else if of_flat_dt_is_compatible(dt_root, b"snps,arc-sdp\0".as_ptr() as *const ::core::ffi::c_char) != 0
        || of_flat_dt_is_compatible(dt_root, b"snps,hsdk\0".as_ptr() as *const ::core::ffi::c_char) != 0
    {
        arc_base_baud = 33333333; /* Fixed 33MHz clk (AXS10x & HSDK) */
    } else {
        arc_base_baud = 50000000; /* Fixed default 50MHz */
    }
}

#[cfg(not(feature = "CONFIG_SERIAL_EARLYCON"))]
unsafe fn arc_set_early_base_baud(_dt_root: ::core::ffi::c_ulong) {}

unsafe extern "C" {
    static __arch_info_begin: *const machine_desc;
    static __arch_info_end: *const machine_desc;

    fn of_flat_dt_is_compatible(
        root: ::core::ffi::c_ulong,
        compatible: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn early_init_dt_scan(dt: *mut ::core::ffi::c_void, pa: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn __pa(addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    fn of_flat_dt_match_machine(
        default_match: *const *const ::core::ffi::c_char,
        get_next_compat: unsafe fn(*mut *const *const ::core::ffi::c_char) -> *const ::core::ffi::c_void,
    ) -> *const machine_desc;
    fn of_get_flat_dt_root() -> ::core::ffi::c_ulong;
    fn machine_halt() -> !;
}

#[repr(C)]
pub struct machine_desc {
    pub dt_compat: *const *const ::core::ffi::c_char,
}

unsafe fn arch_get_next_mach(match_: *mut *const *const ::core::ffi::c_char) -> *const ::core::ffi::c_void {
    static mut mdesc: *const machine_desc = unsafe { __arch_info_begin };
    let m = mdesc;

    if m >= unsafe { __arch_info_end } {
        return core::ptr::null();
    }

    mdesc = unsafe { mdesc.add(1) };
    unsafe { *match_ = (*m).dt_compat };
    m as *const ::core::ffi::c_void
}

/**
 * setup_machine_fdt - Machine setup when an dtb was passed to the kernel
 * @dt:        virtual address pointer to dt blob
 *
 * If a dtb was passed to the kernel, then use it to choose the correct
 * machine_desc and to setup the system.
 */
pub unsafe fn setup_machine_fdt(dt: *mut ::core::ffi::c_void) -> *const machine_desc {
    let mdesc: *const machine_desc;
    let dt_root: ::core::ffi::c_ulong;

    if early_init_dt_scan(dt, __pa(dt)) == 0 {
        return core::ptr::null();
    }

    mdesc = of_flat_dt_match_machine(core::ptr::null(), arch_get_next_mach);
    if mdesc.is_null() {
        machine_halt();
    }

    dt_root = of_get_flat_dt_root();
    arc_set_early_base_baud(dt_root);

    mdesc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
