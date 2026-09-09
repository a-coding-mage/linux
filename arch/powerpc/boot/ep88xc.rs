// SPDX-License-Identifier: GPL-2.0-only
/*
 * Embedded Planet EP88xC with PlanetCore firmware
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// The declarations below are supplied by the surrounding PowerPC boot code.
extern "C" {
    fn dt_fixup_memory(base: u64, size: u64);
    fn planetcore_set_mac_addrs(table: *mut core::ffi::c_char);
    fn planetcore_get_decimal(
        table: *mut core::ffi::c_char,
        key: *const core::ffi::c_char,
        value: *mut u64,
    ) -> i32;
    fn printf(format: *const core::ffi::c_char, ...);
    fn mpc885_fixup_clocks(value: u64);
    fn planetcore_prepare_table(table: *mut core::ffi::c_char);
    fn simple_alloc_init(
        start: *mut core::ffi::c_void,
        size: usize,
        align: usize,
        min_alloc: usize,
    );
    fn fdt_init(dtb: *mut core::ffi::c_void);
    fn planetcore_set_stdout_path(table: *mut core::ffi::c_char);
    fn serial_console_init();

    static mut _end: core::ffi::c_void;
    static mut _dtb_start: core::ffi::c_void;
    static mut platform_ops: PlatformOps;
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static PLANETCORE_KEY_CRYSTAL_HZ: core::ffi::c_char;
    static PLANETCORE_KEY_MB_RAM: core::ffi::c_char;
}

static mut table: *mut core::ffi::c_char = core::ptr::null_mut();
static mut mem_size: u64 = 0;

unsafe extern "C" fn platform_fixups() {
    let mut val: u64 = 0;

    dt_fixup_memory(0, mem_size);
    planetcore_set_mac_addrs(table);

    if planetcore_get_decimal(
        table,
        &PLANETCORE_KEY_CRYSTAL_HZ,
        &mut val,
    ) == 0 {
        printf(b"No PlanetCore crystal frequency key.\r\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    mpc885_fixup_clocks(val);
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: usize,
    _r4: usize,
    _r5: usize,
    _r6: usize,
    _r7: usize,
) {
    table = r3 as *mut core::ffi::c_char;
    planetcore_prepare_table(table);

    if planetcore_get_decimal(table, &PLANETCORE_KEY_MB_RAM, &mut mem_size) == 0 {
        return;
    }

    mem_size = mem_size.wrapping_mul(1024 * 1024);
    simple_alloc_init(
        &mut _end as *mut core::ffi::c_void,
        mem_size.wrapping_sub((&_end as *const core::ffi::c_void) as usize as u64) as usize,
        32,
        64,
    );

    fdt_init(&mut _dtb_start as *mut core::ffi::c_void);

    planetcore_set_stdout_path(table);

    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
