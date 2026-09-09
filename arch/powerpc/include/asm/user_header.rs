/* SPDX-License-Identifier: GPL-2.0 */

// Adapted from <asm-alpha/user.h>
//
// Core file format: The core file is written in such a way that gdb
// can understand it and provide useful information to the user (under
// linux we use the `trad-core' bfd, NOT the osf-core).  The file contents
// are as follows:
//
//  upage: 1 page consisting of a user struct that tells gdb
//	what is present in the file.  Directly after this is a
//	copy of the task_struct, which is currently not used by gdb,
//	but it may come in handy at some point.  All of the registers
//	are stored as part of the upage.  The upage should always be
//	only one page long.
//  data: The data segment follows next.  We use current->end_text to
//  current->brk to pick up all of the user variables, plus any memory
//  that may have been sbrk'ed.  No attempt is made to determine if a
//  page is demand-zero or if a page is totally unused, we just cover
//  the entire range.  All of the addresses are rounded in such a way
//  that an integral number of pages is written.
//  stack: We need the stack information in order to get a meaningful
//  backtrace.  We need to write the data from usp to
//  current->start_stack, so we round each of these in order to be able
//  to write an integer number of pages.

// Supplied by the corresponding PowerPC ptrace definitions.
use crate::user_pt_regs;

#[repr(C)]
pub struct user {
    pub regs: user_pt_regs,       /* entire machine state */
    pub u_tsize: usize,           /* text size (pages) */
    pub u_dsize: usize,           /* data size (pages) */
    pub u_ssize: usize,           /* stack size (pages) */
    pub start_code: usize,        /* text starting address */
    pub start_data: usize,        /* data starting address */
    pub start_stack: usize,       /* stack starting address */
    pub signal: isize,            /* signal causing core dump */
    pub u_ar0: usize,              /* help gdb find registers */
    pub magic: usize,              /* identifies a core file */
    pub u_comm: [i8; 32],         /* user command name */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
