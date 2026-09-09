// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for AmigaOne
 *
 * Author: Gerhard Pircher (gerhard_pircher@gmx.net)
 *
 *   Based on cuboot-83xx.c
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by ops.h, stdio.h, cuboot.h, and ppcboot.h.

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn platform_fixups() {
    dt_fixup_memory((*core::ptr::addr_of!(bd)).bi_memstart,
                    (*core::ptr::addr_of!(bd)).bi_memsize);
    dt_fixup_cpu_clocks((*core::ptr::addr_of!(bd)).bi_intfreq,
                        (*core::ptr::addr_of!(bd)).bi_busfreq / 4,
                        (*core::ptr::addr_of!(bd)).bi_busfreq);
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    CUBOOT_INIT!();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
