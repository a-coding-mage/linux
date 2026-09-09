/* SPDX-License-Identifier: GPL-2.0 */

// REGSET_FP_LEGACY is present only for 32-bit x86 builds.
#[repr(C)]
pub enum {
    REGSET_GENERAL,
    #[cfg(feature = "CONFIG_X86_32")]
    REGSET_FP_LEGACY,
    REGSET_FP,
    REGSET_XSTATE,
}

// The C header includes compiler and generic ptrace definitions.  Their
// declarations are supplied by the surrounding translation unit.
// __FRAME_OFFSETS is needed to get the R* macros on non-32-bit x86 builds.

macro_rules! user_mode {
    ($r:expr) => { UPT_IS_USER(unsafe { &(*$r).regs }) };
}

macro_rules! PT_REGS_AX { ($r:expr) => { UPT_AX(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_BX { ($r:expr) => { UPT_BX(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_CX { ($r:expr) => { UPT_CX(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_DX { ($r:expr) => { UPT_DX(unsafe { &(*$r).regs }) }; }

macro_rules! PT_REGS_SI { ($r:expr) => { UPT_SI(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_DI { ($r:expr) => { UPT_DI(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_BP { ($r:expr) => { UPT_BP(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_EFLAGS { ($r:expr) => { UPT_EFLAGS(unsafe { &(*$r).regs }) }; }

macro_rules! PT_REGS_CS { ($r:expr) => { UPT_CS(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_SS { ($r:expr) => { UPT_SS(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_DS { ($r:expr) => { UPT_DS(unsafe { &(*$r).regs }) }; }
macro_rules! PT_REGS_ES { ($r:expr) => { UPT_ES(unsafe { &(*$r).regs }) }; }

macro_rules! PT_REGS_ORIG_SYSCALL { ($r:expr) => { PT_REGS_AX!($r) }; }
macro_rules! PT_REGS_SYSCALL_RET { ($r:expr) => { PT_REGS_AX!($r) }; }

macro_rules! PT_FIX_EXEC_STACK { ($sp:expr) => {}; }
macro_rules! profile_pc { ($regs:expr) => { PT_REGS_IP!($regs) }; }
macro_rules! UPT_RESTART_SYSCALL { ($r:expr) => {{ $r.ip -= 2; }}; }
macro_rules! PT_REGS_SET_SYSCALL_RETURN { ($r:expr, $res:expr) => {{ PT_REGS_AX!($r) = $res; }}; }

#[inline]
pub unsafe fn regs_return_value(regs: *mut pt_regs) -> c_long {
    PT_REGS_AX!(regs)
}

/*
 * Forward declaration to avoid including sysdep/tls.h, which causes a
 * circular include, and compilation failures.
 */
pub struct user_desc;

#[cfg(feature = "CONFIG_X86_32")]
extern "C" {
    pub fn ptrace_get_thread_area(
        child: *mut task_struct,
        idx: c_int,
        user_desc: *mut user_desc,
    ) -> c_int;
    pub fn ptrace_set_thread_area(
        child: *mut task_struct,
        idx: c_int,
        user_desc: *mut user_desc,
    ) -> c_int;
    pub fn arch_switch_tls(to: *mut task_struct) -> c_int;
}

#[cfg(not(feature = "CONFIG_X86_32"))]
mod non_32 {
    macro_rules! PT_REGS_R8 { ($r:expr) => { UPT_R8(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R9 { ($r:expr) => { UPT_R9(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R10 { ($r:expr) => { UPT_R10(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R11 { ($r:expr) => { UPT_R11(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R12 { ($r:expr) => { UPT_R12(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R13 { ($r:expr) => { UPT_R13(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R14 { ($r:expr) => { UPT_R14(unsafe { &(*$r).regs }) }; }
    macro_rules! PT_REGS_R15 { ($r:expr) => { UPT_R15(unsafe { &(*$r).regs }) }; }

    #[inline]
    pub unsafe fn ptrace_get_thread_area(
        _child: *mut task_struct,
        _idx: c_int,
        _user_desc: *mut user_desc,
    ) -> c_int { -ENOSYS }

    #[inline]
    pub unsafe fn ptrace_set_thread_area(
        _child: *mut task_struct,
        _idx: c_int,
        _user_desc: *mut user_desc,
    ) -> c_int { -ENOSYS }

    extern "C" {
        pub fn arch_prctl(task: *mut task_struct, option: c_int, addr: *mut c_ulong) -> c_long;
    }
}

macro_rules! user_stack_pointer { ($regs:expr) => { PT_REGS_SP!($regs) }; }

extern "C" {
    pub fn arch_switch_to(to: *mut task_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
