/* SPDX-License-Identifier: GPL-2.0 */

// Core file format: The core file is written in such a way that gdb
// can understand it and provide useful information to the user (under
// linux we use the 'trad-core' bfd).  There are quite a number of
// obstacles to being able to view the contents of the floating point
// registers, and until these are solved you will not be able to view the
// contents of them.  Actually, you can read in the core file and look at
// the contents of the user struct to find out what the floating point
// registers contain.
// The actual file contents are as follows:
// UPAGE: 1 page consisting of a user struct that tells gdb what is present
// in the file.  Directly after this is a copy of the task_struct, which
// is currently not used by gdb, but it may come in useful at some point.
// All of the registers are stored as part of the upage.  The upage should
// always be only one page.
// DATA: The data area is stored.  We use current->end_text to
// current->brk to pick up all of the user variables, plus any memory
// that may have been malloced.  No attempt is made to determine if a page
// is demand-zero or if a page is totally unused, we just cover the entire
// range.  All of the addresses are rounded in such a way that an integral
// number of pages is written.
// STACK: We need the stack information in order to get a meaningful
// backtrace.  We need to write the data from (esp) to
// current->start_stack, so we round each of these off in order to be able
// to write an integer number of pages.
// The minimum core file size is 3 pages, or 12288 bytes.

#[repr(C)]
pub struct user_m68kfp_struct {
    pub fpregs: [core::ffi::c_ulong; 8 * 3], // fp0-fp7 registers
    pub fpcntl: [core::ffi::c_ulong; 3],     // fp control regs
}

// This is the old layout of "struct pt_regs" as of Linux 1.x, and
// is still the layout used by user (the new pt_regs doesn't have
// all registers).
#[repr(C)]
pub struct user_regs_struct {
    pub d1: core::ffi::c_long,
    pub d2: core::ffi::c_long,
    pub d3: core::ffi::c_long,
    pub d4: core::ffi::c_long,
    pub d5: core::ffi::c_long,
    pub d6: core::ffi::c_long,
    pub d7: core::ffi::c_long,
    pub a0: core::ffi::c_long,
    pub a1: core::ffi::c_long,
    pub a2: core::ffi::c_long,
    pub a3: core::ffi::c_long,
    pub a4: core::ffi::c_long,
    pub a5: core::ffi::c_long,
    pub a6: core::ffi::c_long,
    pub d0: core::ffi::c_long,
    pub usp: core::ffi::c_long,
    pub orig_d0: core::ffi::c_long,
    pub stkadj: core::ffi::c_short,
    pub sr: core::ffi::c_short,
    pub pc: core::ffi::c_long,
    pub fmtvec: core::ffi::c_short,
    pub __fill: core::ffi::c_short,
}

// When the kernel dumps core, it starts by dumping the user struct -
// this will be used by gdb to figure out where the data and stack segments
// are within the file, and what virtual addresses to use.
#[repr(C)]
pub struct user {
    // We start with the registers, to mimic the way that "memory" is returned
    // from the ptrace(3,...) function.
    pub regs: user_regs_struct, // Where the registers are actually stored
    // ptrace does not yet supply these.  Someday....
    pub u_fpvalid: core::ffi::c_int, // True if math co-processor being used.
    // for this mess. Not yet used.
    pub m68kfp: user_m68kfp_struct, // Math Co-processor registers.
    // The rest of this junk is to help gdb figure out what goes where
    pub u_tsize: core::ffi::c_ulong, // Text segment size (pages).
    pub u_dsize: core::ffi::c_ulong, // Data segment size (pages).
    pub u_ssize: core::ffi::c_ulong, // Stack segment size (pages).
    pub start_code: core::ffi::c_ulong, // Starting virtual address of text.
    // Starting virtual address of stack area. This is actually the bottom
    // of the stack, the top of the stack is always found in the esp register.
    pub start_stack: core::ffi::c_ulong,
    pub signal: core::ffi::c_long, // Signal that caused the core dump.
    pub reserved: core::ffi::c_int, // No longer used
    // Used by gdb to help find the values for the registers.
    pub u_ar0: core::ffi::c_ulong,
    // Math Co-processor pointer.
    pub u_fpstate: *mut user_m68kfp_struct,
    pub magic: core::ffi::c_ulong, // To uniquely identify a core file
    pub u_comm: [core::ffi::c_char; 32], // User command that was responsible
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
