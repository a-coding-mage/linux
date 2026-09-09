/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding ARM headers:
// use asm::page::*;
// use asm::ptrace::pt_regs;

/* Core file format: The core file is written in such a way that gdb
   can understand it and provide useful information to the user (under
   linux we use the 'trad-core' bfd).  There are quite a number of
   obstacles to being able to view the contents of the floating point
   registers, and until these are solved you will not be able to view the
   contents of them.  Actually, you can read in the core file and look at
   the contents of the user struct to find out what the floating point
   registers contain.
   The actual file contents are as follows:
   UPAGE: 1 page consisting of a user struct that tells gdb what is present
   in the file.  Directly after this is a copy of the task_struct, which
   is currently not used by gdb, but it may come in useful at some point.
   All of the registers are stored as part of the upage.  The upage should
   always be only one page.
   DATA: The data area is stored.  We use current->end_text to
   current->brk to pick up all of the user variables, plus any memory
   that may have been malloced.  No attempt is made to determine if a page
   is demand-zero or if a page is totally unused, we just cover the entire
   range.  All of the addresses are rounded in such a way that an integral
   number of pages is written.
   STACK: We need the stack information in order to get a meaningful
   backtrace.  We need to write the data from (esp) to
   current->start_stack, so we round each of these off in order to be able
   to write an integer number of pages.
   The minimum core file size is 3 pages, or 12288 bytes.
*/

#[repr(C)]
pub struct user_fp_fp_reg {
    // C bit-fields are represented by their containing 32-bit words.
    pub sign1: u32,
    pub unused: u32,
    pub sign2: u32,
    pub exponent: u32,
    pub j: u32,
    pub mantissa1: u32,
    pub mantissa0: u32,
}

#[repr(C)]
pub struct user_fp {
    pub fpregs: [user_fp_fp_reg; 8],
    pub fpsr: u32,
    pub fpcr: u32,
    pub ftype: [u8; 8],
    pub init_flag: u32,
}

/* When the kernel dumps core, it starts by dumping the user struct -
   this will be used by gdb to figure out where the data and stack segments
   are within the file, and what virtual addresses to use. */
#[repr(C)]
pub struct user {
    /* We start with the registers, to mimic the way that "memory" is returned
       from the ptrace(3,...) function.  */
    pub regs: pt_regs,
    /* ptrace does not yet supply these.  Someday.... */
    pub u_fpvalid: i32,
    /* for this mess. Not yet used. */
    /* The rest of this junk is to help gdb figure out what goes where */
    pub u_tsize: u32,
    pub u_dsize: u32,
    pub u_ssize: u32,
    pub start_code: u32,
    /* Starting virtual address of stack area.
       This is actually the bottom of the stack,
       the top of the stack is always found in the
       esp register.  */
    pub start_stack: u32,
    pub signal: i32,
    pub reserved: i32,
    pub u_ar0: u32,
    pub magic: u32,
    pub u_comm: [i8; 32],
    pub u_debugreg: [i32; 8],
    pub u_fp: user_fp,
    pub u_fp0: *mut user_fp_struct,
}

/* User specific VFP registers. If only VFPv2 is present, registers 16 to 31
   are ignored by the ptrace system call and the signal handler.
*/
#[repr(C)]
pub struct user_vfp {
    pub fpregs: [u64; 32],
    pub fpscr: u32,
}

/* VFP exception registers exposed to user space during signal delivery.
   Fields not relavant to the current VFP architecture are ignored.
*/
#[repr(C)]
pub struct user_vfp_exc {
    pub fpexc: u32,
    pub fpinst: u32,
    pub fpinst2: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
