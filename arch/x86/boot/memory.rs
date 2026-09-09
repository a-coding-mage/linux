// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * Memory detection code
 */

// Dependency declarations and build-time definitions are supplied by boot.h.

const SMAP: u32 = 0x534d4150; /* ASCII "SMAP" */

static unsafe fn detect_memory_e820() {
    let mut count = 0;
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    let mut desc: *mut boot_e820_entry = boot_params.e820_table.as_mut_ptr();
    static mut buf: boot_e820_entry = core::mem::zeroed(); /* static so it is zeroed */

    initregs(&mut ireg);
    ireg.ax = 0xe820;
    ireg.cx = core::mem::size_of::<boot_e820_entry>();
    ireg.edx = SMAP;
    ireg.di = (&raw mut buf) as *mut boot_e820_entry as usize;

    /*
     * Note: at least one BIOS is known which assumes that the
     * buffer pointed to by one e820 call is the same one as the
     * previous call, and only changes modified fields.  Therefore,
     * we use a temporary buffer and copy the results entry by entry.
     *
     * This routine deliberately does not try to account for
     * ACPI 3+ extended attributes.  This is because there are
     * BIOSes in the field which report zero for the valid bit for
     * all ranges, and we don't currently make any use of the
     * other attribute bits.  Revisit this if we see the extended
     * attribute bits deployed in a meaningful way in the future.
     */

    loop {
        intcall(0x15, &mut ireg, &mut oreg);
        ireg.ebx = oreg.ebx; /* for next iteration... */

        /* BIOSes which terminate the chain with CF = 1 as opposed
           to %ebx = 0 don't always report the SMAP signature on
           the final, failing, probe. */
        if oreg.eflags & X86_EFLAGS_CF != 0 {
            break;
        }

        /* Some BIOSes stop returning SMAP in the middle of
           the search loop.  We don't know exactly how the BIOS
           screwed up the map at that point, we might have a
           partial map, the full map, or complete garbage, so
           just return failure. */
        if oreg.eax != SMAP {
            count = 0;
            break;
        }

        *desc = buf;
        desc = desc.add(1);
        count += 1;

        if ireg.ebx == 0 || count >= boot_params.e820_table.len() {
            break;
        }
    }

    boot_params.e820_entries = count;
}

static unsafe fn detect_memory_e801() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ax = 0xe801;
    intcall(0x15, &mut ireg, &mut oreg);

    if oreg.eflags & X86_EFLAGS_CF != 0 {
        return;
    }

    /* Do we really need to do this? */
    if oreg.cx != 0 || oreg.dx != 0 {
        oreg.ax = oreg.cx;
        oreg.bx = oreg.dx;
    }

    if oreg.ax > 15 * 1024 {
        return; /* Bogus! */
    } else if oreg.ax == 15 * 1024 {
        boot_params.alt_mem_k = (oreg.bx << 6) + oreg.ax;
    } else {
        /*
         * This ignores memory above 16MB if we have a memory
         * hole there.  If someone actually finds a machine
         * with a memory hole at 16MB and no support for
         * 0E820h they should probably generate a fake e820
         * map.
         */
        boot_params.alt_mem_k = oreg.ax;
    }
}

static unsafe fn detect_memory_88() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ah = 0x88;
    intcall(0x15, &mut ireg, &mut oreg);

    boot_params.screen_info.ext_mem_k = oreg.ax;
}

unsafe fn detect_memory() {
    detect_memory_e820();
    detect_memory_e801();
    detect_memory_88();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
