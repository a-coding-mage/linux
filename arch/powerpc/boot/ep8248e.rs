// SPDX-License-Identifier: GPL-2.0-only
/*
 * Embedded Planet EP8248E with PlanetCore firmware
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by ops.h, stdio.h, planetcore.h, pq2.h, and io.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type U64 = u64;

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut _end: u8;
    static _dtb_start: u8;
    static mut platform_ops: PlatformOps;

    fn dt_fixup_memory(address: U64, size: U64);
    fn planetcore_set_mac_addrs(table: *mut c_char);
    fn planetcore_get_decimal(table: *mut c_char, key: c_int, value: *mut U64) -> c_int;
    fn pq2_fixup_clocks(value: U64);
    fn printf(format: *const c_char, ...) -> c_int;
    fn planetcore_prepare_table(table: *mut c_char);
    fn simple_alloc_init(start: *mut c_void, size: c_ulong, align: c_ulong, max_alloc: c_ulong);
    fn fdt_init(dtb_start: *const c_void);
    fn planetcore_set_stdout_path(table: *mut c_char);
    fn serial_console_init();
}

// PLANETCORE_KEY_CRYSTAL_HZ and PLANETCORE_KEY_MB_RAM are supplied by planetcore.h.
extern "C" {
    static PLANETCORE_KEY_CRYSTAL_HZ: c_int;
    static PLANETCORE_KEY_MB_RAM: c_int;
}

static mut table: *mut c_char = core::ptr::null_mut();
static mut mem_size: U64 = 0;

unsafe extern "C" fn platform_fixups() {
    let mut val: U64 = 0;

    dt_fixup_memory(0, mem_size);
    planetcore_set_mac_addrs(table);

    if planetcore_get_decimal(table, PLANETCORE_KEY_CRYSTAL_HZ, &mut val) == 0 {
        printf(b"No PlanetCore crystal frequency key.\r\n\0".as_ptr() as *const c_char);
        return;
    }

    pq2_fixup_clocks(val);
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: c_ulong,
    _r4: c_ulong,
    _r5: c_ulong,
    _r6: c_ulong,
    _r7: c_ulong,
) {
    table = r3 as *mut c_char;
    planetcore_prepare_table(table);

    if planetcore_get_decimal(table, PLANETCORE_KEY_MB_RAM, &mut mem_size) == 0 {
        return;
    }

    mem_size = mem_size.wrapping_mul(1024 * 1024);
    simple_alloc_init(
        core::ptr::addr_of_mut!(_end) as *mut c_void,
        mem_size.wrapping_sub(core::ptr::addr_of!(_end) as c_ulong as U64) as c_ulong,
        32,
        64,
    );

    fdt_init(core::ptr::addr_of!(_dtb_start) as *const c_void);

    planetcore_set_stdout_path(table);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
