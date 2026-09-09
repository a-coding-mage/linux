// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/mm/extable.c
 *  Taken from:
 *   linux/arch/i386/mm/extable.c
 */

// Declarations supplied by the corresponding Linux headers.
use crate::{exception_table_entry, pt_regs};

extern "C" {
    fn search_exception_tables(addr: usize) -> *const exception_table_entry;
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables((*regs).pc as usize);
    if !fixup.is_null() {
        (*regs).pc = (*fixup).fixup;
        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
