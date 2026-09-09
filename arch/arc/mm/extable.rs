// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Borrowed heavily from MIPS
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pt_regs {
    pub ret: usize,
}

#[repr(C)]
pub struct exception_table_entry {
    pub fixup: usize,
}

extern "C" {
    fn search_exception_tables(addr: usize) -> *const exception_table_entry;
    fn instruction_pointer(regs: *const pt_regs) -> usize;
}

pub unsafe extern "C" fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables(instruction_pointer(regs));
    if !fixup.is_null() {
        (*regs).ret = (*fixup).fixup;

        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
