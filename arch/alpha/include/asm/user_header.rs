/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/sched.h, linux/ptrace.h, asm/page.h, and asm/reg.h.

/*
 * Core file format: The core file is written in such a way that gdb
 * can understand it and provide useful information to the user (under
 * linux we use the `trad-core' bfd, NOT the osf-core).  The file contents
 * are as follows:
 *
 *  upage: 1 page consisting of a user struct that tells gdb
 *\twhat is present in the file.  Directly after this is a
 *\tcopy of the task_struct, which is currently not used by gdb,
 *\tbut it may come in handy at some point.  All of the registers
 *\tare stored as part of the upage.  The upage should always be
 *\tonly one page long.
 *  data: The data segment follows next.  We use current->end_text to
 *\tcurrent->brk to pick up all of the user variables, plus any memory
 *\tthat may have been sbrk'ed.  No attempt is made to determine if a
 *\tpage is demand-zero or if a page is totally unused, we just cover
 *\tthe entire range.  All of the addresses are rounded in such a way
 *\tthat an integral number of pages is written.
 *  stack: We need the stack information in order to get a meaningful
 *\tbacktrace.  We need to write the data from usp to
 *\tcurrent->start_stack, so we round each of these in order to be able
 *\tto write an integer number of pages.
 */
#[repr(C)]
pub struct User {
    pub regs: [u64; EF_SIZE / 8 + 32], // integer and fp regs
    pub u_tsize: usize,                // text size (pages)
    pub u_dsize: usize,                // data size (pages)
    pub u_ssize: usize,                // stack size (pages)
    pub start_code: u64,               // text starting address
    pub start_data: u64,               // data starting address
    pub start_stack: u64,              // stack starting address
    pub signal: i64,                   // signal causing core dump
    pub u_ar0: u64,                    // help gdb find registers
    pub magic: u64,                    // identifies a core file
    pub u_comm: [i8; 32],              // user command name
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
