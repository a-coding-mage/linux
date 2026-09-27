/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2001-2002 Pavel Machek <pavel@suse.cz>
 * Based on code
 * Copyright 2001 Patrick Mochel <mochel@osdl.org>
 */

// Dependencies supplied by the corresponding architecture headers:
// asm/desc.h, asm/fpu/api.h, and asm/msr.h.

/* image of the saved processor state */
#[repr(C, packed)]
pub struct saved_context {
    pub cr0: ::kernel::ffi::c_ulong,
    pub cr2: ::kernel::ffi::c_ulong,
    pub cr3: ::kernel::ffi::c_ulong,
    pub cr4: ::kernel::ffi::c_ulong,
    pub misc_enable: u64,
    pub saved_msrs: saved_msrs,
    pub gdt_desc: desc_ptr,
    pub idt: desc_ptr,
    pub ldt: u16,
    pub tss: u16,
    pub tr: ::kernel::ffi::c_ulong,
    pub safety: ::kernel::ffi::c_ulong,
    pub return_address: ::kernel::ffi::c_ulong,
    /*
     * On x86_32, all segment registers except gs are saved at kernel
     * entry in pt_regs.
     */
    pub gs: u16,
    pub misc_enable_saved: bool,
}

/* routines for saving/restoring kernel state */
extern "C" {
    pub static mut core_restore_code: [::kernel::ffi::c_char; 0];
    pub static mut restore_registers: [::kernel::ffi::c_char; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
