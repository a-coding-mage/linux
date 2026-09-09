// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2010 Ben. Herrenschmidt, IBM Corporation.
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
 */

// Dependencies supplied by the surrounding PowerPC boot environment.
// The C source includes: types.h, elf.h, string.h, stdio.h, page.h, ops.h,
// reg.h, io.h, dcr.h, 4xx.h, 44x.h, and libfdt.h.

// BSS_STACK(4096);

static mut ibm4xx_memstart: u32 = 0;

unsafe fn iss_4xx_fixups() {
    let mut memory: *mut core::ffi::c_void;
    let mut reg: [u32; 3] = [0; 3];

    memory = finddevice(b"/memory\0".as_ptr() as *const core::ffi::c_char);
    if memory.is_null() {
        fatal(b"Can't find memory node\n\0".as_ptr() as *const core::ffi::c_char);
    }
    /* This assumes #address-cells = 2, #size-cells =1 and that */
    getprop(
        memory,
        b"reg\0".as_ptr() as *const core::ffi::c_char,
        reg.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&reg),
    );
    if reg[2] != 0 {
        /* If the device tree specifies the memory range, use it */
        ibm4xx_memstart = reg[1];
    } else {
        /* othersize, read it from the SDRAM controller */
        ibm4xx_sdram_fixup_memsize();
    }
}

unsafe fn iss_4xx_vmlinux_alloc(_size: usize) -> *mut core::ffi::c_void {
    ibm4xx_memstart as usize as *mut core::ffi::c_void
}

// SPRN_PIR 0x11E: Processor Identification Register
const SPRN_PIR: u32 = 0x11E;

pub unsafe fn platform_init() {
    let end_of_ram: usize = 0x08000000;
    let avail_ram: usize = end_of_ram - (_end as usize);
    let pir_reg: u32;

    simple_alloc_init(_end, avail_ram, 128, 64);
    platform_ops.fixups = Some(iss_4xx_fixups);
    platform_ops.vmlinux_alloc = Some(iss_4xx_vmlinux_alloc);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    pir_reg = mfspr(SPRN_PIR);
    fdt_set_boot_cpuid_phys(_dtb_start, pir_reg);
    fdt_init(_dtb_start);
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
