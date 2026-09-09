// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright © 2011 Tony Breeds IBM Corporation
 *
 * Based on earlier code:
 *   Copyright (C) Paul Mackerras 1997.
 *
 *   Matt Porter <mporter@kernel.crashing.org>
 *   Copyright 2002-2005 MontaVista Software Inc.
 *
 *   Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *   Copyright (c) 2003, 2004 Zultys Technologies
 *
 *    Copyright 2007 David Gibson, IBM Corporation.
 *    Copyright 2010 Ben. Herrenschmidt, IBM Corporation.
 *    Copyright © 2011 David Kleikamp IBM Corporation
 */

// Dependencies supplied by the surrounding platform implementation.
extern "C" {
    fn mfdcrx(reg: u32) -> u32;
    fn finddevice(path: *const u8) -> *mut core::ffi::c_void;
    fn find_node_by_devtype(node: *mut core::ffi::c_void, devtype: *const u8) -> *mut core::ffi::c_void;
    fn getprop(node: *mut core::ffi::c_void, name: *const u8, value: *mut u32, len: usize) -> i32;
    fn setprop(node: *mut core::ffi::c_void, name: *const u8, value: *const u32, len: usize);
    fn dt_fixup_memory(addr: u64, size: u64);
    fn printf(format: *const u8, ...);
    fn simple_alloc_init(start: *mut u8, size: usize, align: usize, min_size: usize);
    fn ibm44x_dbcr_reset();
    fn mfspr(reg: u32) -> u32;
    fn fdt_check_header(blob: *const u8) -> i32;
    fn fatal(message: *const u8) -> !;
    fn fdt_node_offset_by_prop_value(blob: *const u8, start: i32, prop: *const u8, value: *const u8, len: usize) -> i32;
    fn fdt_getprop(blob: *const u8, node: i32, name: *const u8, len: *mut i32) -> *const u32;
    fn fdt_set_boot_cpuid_phys(blob: *mut u8, cpuid: u32) -> i32;
    fn fdt_init(blob: *mut u8);
    fn serial_console_init();
}

// BSS_STACK(4096)

const MAX_RANKS: u32 = 0x4;
const DDR3_MR0CF: u32 = 0x8001_0011;
const SPRN_PIR: u32 = 0x11e; /* Processor Identification Register */

static mut ibm_currituck_memsize: u64 = 0;

extern "C" {
    static mut _end: u8;
    static mut _dtb_start: u8;
    static mut timebase_period_ns: u32;
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
    exit: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut platform_ops: PlatformOps;
}

unsafe fn ibm_currituck_detect_memsize() -> u64 {
    let mut memsize: u64 = 0;

    for i in 0..MAX_RANKS {
        let mut reg = mfdcrx(DDR3_MR0CF.wrapping_add(i));

        if (reg & 1) == 0 {
            continue;
        }

        reg &= 0x0000_f000;
        reg >>= 12;
        memsize = memsize.wrapping_add(0x800000u64 << reg);
    }

    memsize
}

unsafe extern "C" fn ibm_currituck_fixups() {
    let mut devp = finddevice(b"/\0".as_ptr());
    let mut dma_ranges = [0u32; 7];

    dt_fixup_memory(0, ibm_currituck_memsize);

    while {
        devp = find_node_by_devtype(devp, b"pci\0".as_ptr());
        !devp.is_null()
    } {
        if getprop(devp, b"dma-ranges\0".as_ptr(), dma_ranges.as_mut_ptr(), core::mem::size_of_val(&dma_ranges)) < 0 {
            printf(b"%s: Failed to get dma-ranges\r\n\0".as_ptr(), b"ibm_currituck_fixups\0".as_ptr());
            continue;
        }

        dma_ranges[5] = (ibm_currituck_memsize >> 32) as u32;
        dma_ranges[6] = (ibm_currituck_memsize & 0xffff_ffff) as u32;

        setprop(devp, b"dma-ranges\0".as_ptr(), dma_ranges.as_ptr(), core::mem::size_of_val(&dma_ranges));
    }
}

#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    let end_of_ram: usize;
    let avail_ram: usize;
    let pir_reg: u32;
    let node: i32;
    let mut size: i32 = 0;

    ibm_currituck_memsize = ibm_currituck_detect_memsize();
    if (ibm_currituck_memsize >> 32) != 0 {
        end_of_ram = usize::MAX;
    } else {
        end_of_ram = ibm_currituck_memsize as usize;
    }
    avail_ram = end_of_ram.wrapping_sub((&_end as *const u8) as usize);

    simple_alloc_init(&mut _end, avail_ram, 128, 64);
    platform_ops.fixups = Some(ibm_currituck_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    pir_reg = mfspr(SPRN_PIR);

    /* Make sure FDT blob is sane */
    if fdt_check_header(&_dtb_start) != 0 {
        fatal(b"Invalid device tree blob\n\0".as_ptr());
    }

    node = fdt_node_offset_by_prop_value(&_dtb_start, -1, b"device_type\0".as_ptr(), b"cpu\0".as_ptr(), 4);
    if node < 0 {
        fatal(b"Cannot find cpu node\n\0".as_ptr());
    }
    let timebase = fdt_getprop(&_dtb_start, node, b"timebase-frequency\0".as_ptr(), &mut size);
    if !timebase.is_null() && size == 4 {
        timebase_period_ns = 1_000_000_000u32 / *timebase;
    }

    fdt_set_boot_cpuid_phys(&mut _dtb_start, pir_reg);
    fdt_init(&mut _dtb_start);

    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
