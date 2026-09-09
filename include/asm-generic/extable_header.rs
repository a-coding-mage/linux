/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The exception table consists of pairs of addresses: the first is the
 * address of an instruction that is allowed to fault, and the second is the
 * address at which the program should continue.  No registers are modified,
 * so it is entirely up to the continuation code to figure out what to do.
 *
 * All the routines below use bits of fixup code that are out of line
 * with the main instruction path.  This means when everything is well,
 * we don't even have to jump over them.  Further, they do not intrude
 * on our cache or tlb entries.
 */

#[repr(C)]
pub struct exception_table_entry {
    pub insn: usize,
    pub fixup: usize,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn fixup_exception(regs: *mut pt_regs) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
