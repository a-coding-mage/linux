/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Dependencies supplied by arch.h and sysdep/ptrace.h are external to this file.

/* These two are from asm-um/uaccess.h and linux/module.h, check them. */
#[repr(C)]
pub struct exception_table_entry {
    pub insn: libc::c_ulong,
    pub fixup: libc::c_ulong,
}

#[repr(C)]
pub struct uml_pt_regs {
    pub ip: libc::c_ulong,
}

unsafe extern "C" {
    pub fn search_exception_tables(add: libc::c_ulong) -> *const exception_table_entry;
}

/* Compare this to arch/i386/mm/extable.c:fixup_exception() */
pub unsafe fn arch_fixup(address: libc::c_ulong, regs: *mut uml_pt_regs) -> libc::c_int {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables(address);
    if !fixup.is_null() {
        (*regs).ip = (*fixup).fixup;
        return 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
