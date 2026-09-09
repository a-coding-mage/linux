/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2001-2003 Pavel Machek <pavel@suse.cz>
 * Based on code
 * Copyright 2001 Patrick Mochel <mochel@osdl.org>
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/desc.h, asm/fpu/api.h, and asm/msr.h.

/*
 * Image of the saved processor state, used by the low level ACPI suspend to
 * RAM code and by the low level hibernation code.
 *
 * If you modify it, check how it is used in arch/x86/kernel/acpi/wakeup_64.S
 * and make sure that __save/__restore_processor_state(), defined in
 * arch/x86/power/cpu.c, still work as required.
 *
 * Because the structure is packed, make sure to avoid unaligned members. For
 * optimisation purposes but also because tools like kmemleak only search for
 * pointers that are aligned.
 */
#[repr(C, packed)]
pub struct saved_context {
    pub regs: pt_regs,

    /*
     * User CS and SS are saved in current_pt_regs().  The rest of the
     * segment selectors need to be saved and restored here.
     */
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,

    /*
     * Usermode FSBASE and GSBASE may not match the fs and gs selectors,
     * so we save them separately.  We save the kernelmode GSBASE to
     * restore percpu access after resume.
     */
    pub kernelmode_gs_base: usize,
    pub usermode_gs_base: usize,
    pub fs_base: usize,

    pub cr0: usize,
    pub cr2: usize,
    pub cr3: usize,
    pub cr4: usize,
    pub misc_enable: u64,
    pub saved_msrs: saved_msrs,
    pub efer: usize,
    pub gdt_pad: u16, // Unused
    pub gdt_desc: desc_ptr,
    pub idt_pad: u16,
    pub idt: desc_ptr,
    pub ldt: u16,
    pub tss: u16,
    pub tr: usize,
    pub safety: usize,
    pub return_address: usize,
    pub misc_enable_saved: bool,
}

// Equivalent of the C token-pasting macro: set_debugreg((thread)->debugreg##register, register)
#[macro_export]
macro_rules! loaddebug {
    ($thread:expr, $register:ident) => {
        set_debugreg!($thread.debugreg$register, $register)
    };
}

/* routines for saving/restoring kernel state */
extern "C" {
    pub static mut core_restore_code: [i8; 0];
    pub static mut restore_registers: [i8; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
