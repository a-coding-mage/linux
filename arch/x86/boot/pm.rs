// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Prepare the machine for transition to protected mode.
 */

// Dependencies supplied by boot.h, asm/desc_defs.h, and asm/segment.h.

/*
 * Invoke the realmode switch hook if present; otherwise
 * disable all interrupts.
 */
unsafe fn realmode_switch_hook() {
    if boot_params.hdr.realmode_swtch != 0 {
        core::arch::asm!(
            "lcallw *[{hook}]",
            hook = in(reg) &boot_params.hdr.realmode_swtch,
            out("eax") _,
            out("ebx") _,
            out("ecx") _,
            out("edx") _,
        );
    } else {
        core::arch::asm!("cli");
        outb(0x80, 0x70); /* Disable NMI */
        io_delay();
    }
}

/*
 * Disable all interrupts at the legacy PIC.
 */
unsafe fn mask_all_interrupts() {
    outb(0xff, 0xa1); /* Mask all interrupts on the secondary PIC */
    io_delay();
    outb(0xfb, 0x21); /* Mask all but cascade on the primary PIC */
    io_delay();
}

/*
 * Reset IGNNE# if asserted in the FPU.
 */
unsafe fn reset_coprocessor() {
    outb(0, 0xf0);
    io_delay();
    outb(0, 0xf1);
    io_delay();
}

/*
 * Set up the GDT
 */

#[repr(C, packed)]
struct GdtPtr {
    len: u16,
    ptr: u32,
}

unsafe fn setup_gdt() {
    /* There are machines which are known to not boot with the GDT
       being 8-byte unaligned.  Intel recommends 16 byte alignment. */
    #[repr(align(16))]
    struct BootGdt([u64; GDT_ENTRY_BOOT_TSS + 1]);

    static BOOT_GDT: BootGdt = BootGdt({
        let mut entries = [0u64; GDT_ENTRY_BOOT_TSS + 1];
        /* CS: code, read/execute, 4 GB, base 0 */
        entries[GDT_ENTRY_BOOT_CS] = GDT_ENTRY(DESC_CODE32, 0, 0xfffff);
        /* DS: data, read/write, 4 GB, base 0 */
        entries[GDT_ENTRY_BOOT_DS] = GDT_ENTRY(DESC_DATA32, 0, 0xfffff);
        /* TSS: 32-bit tss, 104 bytes, base 4096 */
        /* We only have a TSS here to keep Intel VT happy;
           we don't actually use it for anything. */
        entries[GDT_ENTRY_BOOT_TSS] = GDT_ENTRY(DESC_TSS32, 4096, 103);
        entries
    });

    /* Xen HVM incorrectly stores a pointer to the gdt_ptr, instead
       of the gdt_ptr contents.  Thus, make it static so it will
       stay in memory, at least long enough that we switch to the
       proper kernel GDT. */
    static mut GDT: GdtPtr = GdtPtr { len: 0, ptr: 0 };

    GDT.len = core::mem::size_of::<BootGdt>() as u16 - 1;
    GDT.ptr = (&BOOT_GDT as *const BootGdt as u32).wrapping_add(ds() << 4);

    core::arch::asm!("lgdt [{0}]", in(reg) &GDT);
}

/*
 * Set up the IDT
 */
unsafe fn setup_idt() {
    static NULL_IDT: GdtPtr = GdtPtr { len: 0, ptr: 0 };
    core::arch::asm!("lidt [{0}]", in(reg) &NULL_IDT);
}

/*
 * Actual invocation sequence
 */
pub unsafe fn go_to_protected_mode() {
    /* Hook before leaving real mode, also disables interrupts */
    realmode_switch_hook();

    /* Enable the A20 gate */
    if enable_a20() {
        puts("A20 gate not responding, unable to boot...\n");
        die();
    }

    /* Reset coprocessor (IGNNE#) */
    reset_coprocessor();

    /* Mask all interrupts in the PIC */
    mask_all_interrupts();

    /* Actual transition to protected mode... */
    setup_idt();
    setup_gdt();
    protected_mode_jump(
        boot_params.hdr.code32_start,
        (&boot_params as *const _ as u32).wrapping_add(ds() << 4),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
