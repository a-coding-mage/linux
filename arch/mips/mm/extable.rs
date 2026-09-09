/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 99, 2001 - 2004 Ralf Baechle <ralf@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/extable.h, linux/spinlock.h, asm/branch.h, linux/uaccess.h

extern "C" {
    fn search_exception_tables(addr: usize) -> *const exception_table_entry;
    fn exception_epc(regs: *mut pt_regs) -> usize;
}

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables(exception_epc(regs));
    if !fixup.is_null() {
        (*regs).cp0_epc = (*fixup).nextinsn;

        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
