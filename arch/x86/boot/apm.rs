// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 *   Original APM BIOS checking by Stephen Rothwell, May 1994
 *   (sfr@canb.auug.org.au)
 *
 * ----------------------------------------------------------------------- */

/*
 * Get APM BIOS information
 */

// Dependency declarations supplied by boot.h are intentionally left external.

pub unsafe fn query_apm_bios() -> i32 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    /* APM BIOS installation check */
    initregs(&mut ireg);
    intcall(0x15, &mut ireg, &mut oreg);

    if (oreg.flags & X86_EFLAGS_CF) != 0 {
        return -1; // No APM BIOS
    }

    if oreg.bx != 0x504d {
        // "PM" signature
        return -1;
    }

    if (oreg.cx & 0x02) == 0 {
        // 32 bits supported?
        return -1;
    }

    /* Disconnect first, just in case */
    ireg.al = 0x04;
    intcall(0x15, &mut ireg, core::ptr::null_mut());

    /* 32-bit connect */
    ireg.al = 0x03;
    intcall(0x15, &mut ireg, &mut oreg);

    boot_params.apm_bios_info.cseg = oreg.ax;
    boot_params.apm_bios_info.offset = oreg.ebx;
    boot_params.apm_bios_info.cseg_16 = oreg.cx;
    boot_params.apm_bios_info.dseg = oreg.dx;
    boot_params.apm_bios_info.cseg_len = oreg.si;
    boot_params.apm_bios_info.cseg_16_len = oreg.hsi;
    boot_params.apm_bios_info.dseg_len = oreg.di;

    if (oreg.flags & X86_EFLAGS_CF) != 0 {
        return -1;
    }

    /* Redo the installation check as the 32-bit connect;
       some BIOSes return different flags this way... */

    ireg.al = 0x00;
    intcall(0x15, &mut ireg, &mut oreg);

    if (oreg.eflags & X86_EFLAGS_CF) != 0 || oreg.bx != 0x504d {
        /* Failure with 32-bit connect, try to disconnect and ignore */
        ireg.al = 0x04;
        intcall(0x15, &mut ireg, core::ptr::null_mut());
        return -1;
    }

    boot_params.apm_bios_info.version = oreg.ax;
    boot_params.apm_bios_info.flags = oreg.cx;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
