// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bootwrapper for ePAPR compliant firmwares
 *
 * Copyright 2010 David Gibson <david@gibson.dropbear.id.au>, IBM Corporation.
 *
 * Based on earlier bootwrappers by:
 * (c) Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp,\
 *   and
 * Scott Wood <scottwood@freescale.com>
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding bootwrapper.
extern "C" {
    static mut _end: u8;
    fn fatal(format: *const u8, ...);
    fn printf(format: *const u8, ...);
    fn simple_alloc_init(base: *mut u8, size: usize, align: usize, granularity: usize);
    fn fdt_init(fdt: *mut core::ffi::c_void);
    fn serial_console_init();
    fn fdt_totalsize(fdt: *const core::ffi::c_void) -> u32;
    static mut platform_ops: PlatformOps;
}

// BSS_STACK(4096);

const EPAPR_SMAGIC: u32 = 0x6550_4150;
const EPAPR_EMAGIC: u32 = 0x4550_4150;

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

static mut epapr_magic: u32 = 0;
static mut ima_size: usize = 0;
static mut fdt_addr: usize = 0;

unsafe extern "C" fn platform_fixups() {
    if epapr_magic != EPAPR_EMAGIC && epapr_magic != EPAPR_SMAGIC {
        fatal(
            b"r6 contained 0x%08x instead of ePAPR magic number\n\0".as_ptr(),
            epapr_magic,
        );
    }

    let end_addr = (&raw mut _end) as *mut u8 as usize;
    if ima_size < end_addr {
        printf(
            b"WARNING: Image loaded outside IMA! (_end=%p, ima_size=0x%lx)\n\0".as_ptr(),
            (&raw mut _end),
            ima_size,
        );
    }
    if ima_size < fdt_addr {
        printf(
            b"WARNING: Device tree address is outside IMA!(fdt_addr=0x%lx, ima_size=0x%lx)\n\0".as_ptr(),
            fdt_addr,
            ima_size,
        );
    }
    let fdt_size = fdt_totalsize(fdt_addr as *const core::ffi::c_void);
    if ima_size < fdt_addr.wrapping_add(fdt_size as usize) {
        printf(
            b"WARNING: Device tree extends outside IMA! (fdt_addr=0x%lx, size=0x%x, ima_size=0x%lx\n\0".as_ptr(),
            fdt_addr,
            fdt_size,
            ima_size,
        );
    }
}

pub unsafe extern "C" fn epapr_platform_init(
    r3: usize,
    _r4: usize,
    _r5: usize,
    r6: usize,
    r7: usize,
) {
    epapr_magic = r6 as u32;
    ima_size = r7;
    fdt_addr = r3;

    /* FIXME: we should process reserve entries */

    let end_addr = (&raw mut _end) as *mut u8;
    simple_alloc_init(end_addr, ima_size.wrapping_sub(end_addr as usize), 32, 64);

    fdt_init(fdt_addr as *mut core::ffi::c_void);

    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
