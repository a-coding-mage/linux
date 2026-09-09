/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <asm/ptrace.h> and <uapi/asm/sigcontext.h>
// are supplied by other translated files.

#[cfg(not(target_arch = "asm"))]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const __SUNOS_MAXWIN: usize = 31;

/* This is what SunOS does, so shall I unless we use new 32bit signals or rt signals. */
#[repr(C)]
pub struct sigcontext32 {
    pub sigc_onstack: c_int,      /* state to restore */
    pub sigc_mask: c_int,         /* sigmask to restore */
    pub sigc_sp: c_int,           /* stack pointer */
    pub sigc_pc: c_int,           /* program counter */
    pub sigc_npc: c_int,          /* next program counter */
    pub sigc_psr: c_int,          /* for condition codes etc */
    pub sigc_g1: c_int,           /* User uses these two registers */
    pub sigc_o0: c_int,           /* within the trampoline code. */

    /* Now comes information regarding the users window set
     * at the time of the signal.
     */
    pub sigc_oswins: c_int,       /* outstanding windows */

    /* stack ptrs for each regwin buf */
    pub sigc_spbuf: [c_uint; __SUNOS_MAXWIN],

    /* Windows to restore after signal */
    pub sigc_wbuf: [reg_window32; __SUNOS_MAXWIN],
}

/* This is what we use for 32bit new non-rt signals. */
#[repr(C)]
pub struct __siginfo32_t_si_regs {
    pub psr: c_uint,
    pub pc: c_uint,
    pub npc: c_uint,
    pub y: c_uint,
    pub u_regs: [c_uint; 16], /* globals and ins */
}

#[repr(C)]
pub struct __siginfo32_t {
    pub si_regs: __siginfo32_t_si_regs,
    pub si_mask: c_int,
}

pub const __SIGC_MAXWIN: usize = 7;

#[repr(C)]
pub struct __siginfo_reg_window {
    pub locals: [c_ulong; 8],
    pub ins: [c_ulong; 8],
}

#[repr(C)]
pub struct __siginfo_rwin_t {
    pub wsaved: c_int,
    pub reg_window: [__siginfo_reg_window; __SIGC_MAXWIN],
    pub rwbuf_stkptrs: [c_ulong; __SIGC_MAXWIN],
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct __siginfo_fpu_t {
    pub si_float_regs: [c_uint; 64],
    pub si_fsr: c_ulong,
    pub si_gsr: c_ulong,
    pub si_fprs: c_ulong,
}

/* This is what SunOS doesn't, so we have to write this alone
   and do it properly. */
#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct sigcontext {
    /* The size of this array has to match SI_MAX_SIZE from siginfo.h */
    pub sigc_info: [c_char; 128],
    #[repr(C)]
    pub sigc_regs: sigcontext_sigc_regs,
    pub sigc_fpu_save: *mut __siginfo_fpu_t,
    pub sigc_stack: sigcontext_sigc_stack,
    pub sigc_mask: c_ulong,
    pub sigc_rwin_save: *mut __siginfo_rwin_t,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct sigcontext_sigc_regs {
    pub u_regs: [c_ulong; 16], /* globals and ins */
    pub tstate: c_ulong,
    pub tpc: c_ulong,
    pub tnpc: c_ulong,
    pub y: c_uint,
    pub fprs: c_uint,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct sigcontext_sigc_stack {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: c_ulong,
}

#[cfg(not(target_pointer_width = "64"))]
#[repr(C)]
pub struct __siginfo_fpu_t {
    pub si_float_regs: [c_ulong; 32],
    pub si_fsr: c_ulong,
    pub si_fpqdepth: c_ulong,
    pub si_fpqueue: [__siginfo_fpu_t_si_fpqueue; 16],
}

#[cfg(not(target_pointer_width = "64"))]
#[repr(C)]
pub struct __siginfo_fpu_t_si_fpqueue {
    pub insn_addr: *mut c_ulong,
    pub insn: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
