/*
 * Copyright (C) 2010, Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009, Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Declarations corresponding to the types and function supplied by
// <linux/extable.h> and <linux/uaccess.h>.
#[repr(C)]
pub struct pt_regs {
    pub ea: usize,
}

#[repr(C)]
pub struct exception_table_entry {
    pub insn: usize,
    pub fixup: usize,
}

unsafe extern "C" {
    fn search_exception_tables(addr: usize) -> *const exception_table_entry;
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables((*regs).ea);
    if !fixup.is_null() {
        (*regs).ea = (*fixup).fixup;
        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
