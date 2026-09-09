/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding ptrace/page translations:
// `pt_regs` and `size_t`.

/*
 * Core file format: The core file is written in such a way that gdb
 * can understand it and provide useful information to the user (under
 * linux we use the `trad-core' bfd).  The file contents are as follows:
 *
 *  upage: 1 page consisting of a user struct that tells gdb
 *	what is present in the file.  Directly after this is a
 *	copy of the task_struct, which is currently not used by gdb,
 *	but it may come in handy at some point.  All of the registers
 *	are stored as part of the upage.  The upage should always be
 *	only one page long.
 *  data: The data segment follows next.  We use current->end_text to
 *	current->brk to pick up all of the user variables, plus any memory
 *	that may have been sbrk'ed.  No attempt is made to determine if a
 *	page is demand-zero or if a page is totally unused, we just cover
 *	the entire range.  All of the addresses are rounded in such a way
 *	that an integral number of pages is written.
 *  stack: We need the stack information in order to get a meaningful
 *	backtrace.  We need to write the data from usp to
 *	current->start_stack, so we round each of these in order to be able
 *	to write an integer number of pages.
 */

#[repr(C)]
pub struct user_fpu_struct {
    pub fp_regs: [core::ffi::c_ulong; 16],
    pub xfp_regs: [core::ffi::c_ulong; 16],
    pub fpscr: core::ffi::c_ulong,
    pub fpul: core::ffi::c_ulong,
}

#[repr(C)]
pub struct user {
    pub regs: pt_regs, // entire machine state
    pub fpu: user_fpu_struct, // Math Co-processor registers
    pub u_fpvalid: core::ffi::c_int, // True if math co-processor being used
    pub u_tsize: usize, // text size (pages)
    pub u_dsize: usize, // data size (pages)
    pub u_ssize: usize, // stack size (pages)
    pub start_code: core::ffi::c_ulong, // text starting address
    pub start_data: core::ffi::c_ulong, // data starting address
    pub start_stack: core::ffi::c_ulong, // stack starting address
    pub signal: core::ffi::c_long, // signal causing core dump
    pub u_ar0: core::ffi::c_ulong, // help gdb find registers
    pub u_fpstate: *mut user_fpu_struct, // Math Co-processor pointer
    pub magic: core::ffi::c_ulong, // identifies a core file
    pub u_comm: [core::ffi::c_char; 32], // user command name
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
