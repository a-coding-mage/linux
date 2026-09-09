/* SPDX-License-Identifier: GPL-2.0 */

// #include <asm/page.h>
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

/*
 * Pentium III FXSR, SSE support
 * Gareth Hughes <gareth@valinux.com>, May 2000
 *
 * Provide support for the GDB 5.0+ PTRACE_{GET|SET}FPXREGS requests for
 * interacting with the FXSR-format floating point environment.  Floating
 * point data can be accessed in the regular format in the usual manner,
 * and both the standard and SIMD floating point data can be accessed via
 * the new ptrace requests.  In either case, changes to the FPU environment
 * will be reflected in the task's state as expected.
 */

#[repr(C)]
pub struct user_i387_struct {
    pub cwd: i32,
    pub swd: i32,
    pub twd: i32,
    pub fip: i32,
    pub fcs: i32,
    pub foo: i32,
    pub fos: i32,
    pub st_space: [i32; 20], /* 8*10 bytes for each FP-reg = 80 bytes */
}

#[repr(C)]
pub struct user_fxsr_struct {
    pub cwd: u16,
    pub swd: u16,
    pub twd: u16,
    pub fop: u16,
    pub fip: i32,
    pub fcs: i32,
    pub foo: i32,
    pub fos: i32,
    pub mxcsr: i32,
    pub reserved: i32,
    pub st_space: [i32; 32], /* 8*16 bytes for each FP-reg = 128 bytes */
    pub xmm_space: [i32; 32], /* 8*16 bytes for each XMM-reg = 128 bytes */
    pub padding: [i32; 56],
}

/*
 * This is the old layout of "struct pt_regs", and
 * is still the layout used by user mode (the new
 * pt_regs doesn't have all registers as the kernel
 * doesn't use the extra segment registers)
 */
#[repr(C)]
pub struct user_regs_struct {
    pub bx: u32,
    pub cx: u32,
    pub dx: u32,
    pub si: u32,
    pub di: u32,
    pub bp: u32,
    pub ax: u32,
    pub ds: u32,
    pub es: u32,
    pub fs: u32,
    pub gs: u32,
    pub orig_ax: u32,
    pub ip: u32,
    pub cs: u32,
    pub flags: u32,
    pub sp: u32,
    pub ss: u32,
}

/* When the kernel dumps core, it starts by dumping the user struct -
   this will be used by gdb to figure out where the data and stack segments
   are within the file, and what virtual addresses to use. */
#[repr(C)]
pub struct user {
    /* We start with the registers, to mimic the way that "memory" is returned
       from the ptrace(3,...) function.  */
    pub regs: user_regs_struct, /* Where the registers are actually stored */
    /* ptrace does not yet supply these.  Someday.... */
    pub u_fpvalid: i32, /* True if math co-processor being used. */
    /* for this mess. Not yet used. */
    pub i387: user_i387_struct, /* Math Co-processor registers. */
    /* The rest of this junk is to help gdb figure out what goes where */
    pub u_tsize: u32, /* Text segment size (pages). */
    pub u_dsize: u32, /* Data segment size (pages). */
    pub u_ssize: u32, /* Stack segment size (pages). */
    pub start_code: u32, /* Starting virtual address of text. */
    pub start_stack: u32, /* Starting virtual address of stack area.
                              This is actually the bottom of the stack,
                              the top of the stack is always found in the
                              esp register.  */
    pub signal: i32, /* Signal that caused the core dump. */
    pub reserved: i32, /* No longer used */
    pub u_ar0: u32, /* Used by gdb to help find the values for */
    /* the registers. */
    pub u_fpstate: *mut user_i387_struct, /* Math Co-processor pointer. */
    pub magic: u32, /* To uniquely identify a core file */
    pub u_comm: [i8; 32], /* User command that was responsible */
    pub u_debugreg: [i32; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
