/* SPDX-License-Identifier: GPL-2.0-or-later */

/* BookE/4xx */
pub const INTERRUPT_CRITICAL_INPUT: u32 = 0x100;
/* BookE */
pub const INTERRUPT_DEBUG: u32 = 0xd00;
#[cfg(CONFIG_BOOKE)]
pub const INTERRUPT_PERFMON: u32 = 0x260;
#[cfg(CONFIG_BOOKE)]
pub const INTERRUPT_DOORBELL: u32 = 0x280;
/* BookS/4xx/8xx */
pub const INTERRUPT_MACHINE_CHECK: u32 = 0x200;
/* BookS/8xx */
pub const INTERRUPT_SYSTEM_RESET: u32 = 0x100;
/* BookS */
pub const INTERRUPT_DATA_SEGMENT: u32 = 0x380;
pub const INTERRUPT_INST_SEGMENT: u32 = 0x480;
pub const INTERRUPT_TRACE: u32 = 0xd00;
pub const INTERRUPT_H_DATA_STORAGE: u32 = 0xe00;
pub const INTERRUPT_HMI: u32 = 0xe60;
pub const INTERRUPT_H_FAC_UNAVAIL: u32 = 0xf80;
#[cfg(CONFIG_PPC_BOOK3S)]
pub const INTERRUPT_DOORBELL: u32 = 0xa00;
#[cfg(CONFIG_PPC_BOOK3S)]
pub const INTERRUPT_PERFMON: u32 = 0xf00;
#[cfg(CONFIG_PPC_BOOK3S)]
pub const INTERRUPT_ALTIVEC_UNAVAIL: u32 = 0xf20;
/* BookE/BookS/4xx/8xx */
pub const INTERRUPT_DATA_STORAGE: u32 = 0x300;
pub const INTERRUPT_INST_STORAGE: u32 = 0x400;
pub const INTERRUPT_EXTERNAL: u32 = 0x500;
pub const INTERRUPT_ALIGNMENT: u32 = 0x600;
pub const INTERRUPT_PROGRAM: u32 = 0x700;
pub const INTERRUPT_SYSCALL: u32 = 0xc00;
pub const INTERRUPT_TRACE_COMMON: u32 = 0xd00;
/* BookE/BookS/44x */
pub const INTERRUPT_FP_UNAVAIL: u32 = 0x800;
/* BookE/BookS/44x/8xx */
pub const INTERRUPT_DECREMENTER: u32 = 0x900;
#[cfg(not(any(CONFIG_BOOKE, CONFIG_PPC_BOOK3S)))]
pub const INTERRUPT_PERFMON: u32 = 0x0;
/* 8xx */
pub const INTERRUPT_SOFT_EMU_8xx: u32 = 0x1000;
pub const INTERRUPT_INST_TLB_MISS_8xx: u32 = 0x1100;
pub const INTERRUPT_DATA_TLB_MISS_8xx: u32 = 0x1200;
pub const INTERRUPT_INST_TLB_ERROR_8xx: u32 = 0x1300;
pub const INTERRUPT_DATA_TLB_ERROR_8xx: u32 = 0x1400;
pub const INTERRUPT_DATA_BREAKPOINT_8xx: u32 = 0x1c00;
pub const INTERRUPT_INST_BREAKPOINT_8xx: u32 = 0x1d00;
/* 603 */
pub const INTERRUPT_INST_TLB_MISS_603: u32 = 0x1000;
pub const INTERRUPT_DATA_LOAD_TLB_MISS_603: u32 = 0x1100;
pub const INTERRUPT_DATA_STORE_TLB_MISS_603: u32 = 0x1200;

/* Includes and kernel attributes are supplied by the surrounding translation unit. */
#[macro_export]
macro_rules! INT_SOFT_MASK_BUG_ON {
    ($regs:expr, $cond:expr) => {{
        if (unsafe { user_mode($regs) } || unsafe { TRAP($regs) } != INTERRUPT_PROGRAM) {
            unsafe { BUG_ON($cond); }
        }
    }};
}

/* interrupt_handler: __visible noinline notrace __no_kcsan __no_sanitize_address */

/* C declaration/definition macros, retaining their call sequence and ABI. */
#[macro_export]
macro_rules! DECLARE_INTERRUPT_HANDLER_RAW { ($func:ident) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long; }; }
#[macro_export]
macro_rules! DECLARE_INTERRUPT_HANDLER { ($func:ident) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs); }; }
#[macro_export]
macro_rules! DECLARE_INTERRUPT_HANDLER_RET { ($func:ident) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long; }; }
#[macro_export]
macro_rules! DECLARE_INTERRUPT_HANDLER_ASYNC { ($func:ident) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs); }; }
#[macro_export]
macro_rules! DECLARE_INTERRUPT_HANDLER_NMI { ($func:ident) => { pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long; }; }

/* The DEFINE_* macros are represented with an explicit implementation name because
 * Rust has no token-pasting operator equivalent to C's ____##func. */
#[macro_export]
macro_rules! DEFINE_INTERRUPT_HANDLER_RAW { ($func:ident, $inner:ident, $body:block) => {
    pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long { __hard_RI_enable(); $inner(regs) }
    unsafe fn $inner(regs: *mut pt_regs) -> libc::c_long $body
}; }
#[macro_export]
macro_rules! DEFINE_INTERRUPT_HANDLER { ($func:ident, $inner:ident, $body:block) => {
    pub unsafe extern "C" fn $func(regs: *mut pt_regs) { let state = irqentry_enter(regs); instrumentation_begin(); $inner(regs); instrumentation_end(); irqentry_exit(regs, state); }
    unsafe fn $inner(regs: *mut pt_regs) $body
}; }
#[macro_export]
macro_rules! DEFINE_INTERRUPT_HANDLER_RET { ($func:ident, $inner:ident, $body:block) => {
    pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long { let state = irqentry_enter(regs); instrumentation_begin(); let ret = $inner(regs); instrumentation_end(); irqentry_exit(regs, state); ret }
    unsafe fn $inner(regs: *mut pt_regs) -> libc::c_long $body
}; }
#[macro_export]
macro_rules! DEFINE_INTERRUPT_HANDLER_ASYNC { ($func:ident, $inner:ident, $body:block) => {
    pub unsafe extern "C" fn $func(regs: *mut pt_regs) { let state = irqentry_enter(regs); instrumentation_begin(); irq_enter_rcu(); $inner(regs); nap_adjust_return(regs); irq_exit_rcu(); instrumentation_end(); irqentry_exit(regs, state); }
    unsafe fn $inner(regs: *mut pt_regs) $body
}; }
#[macro_export]
macro_rules! DEFINE_INTERRUPT_HANDLER_NMI { ($func:ident, $inner:ident, $body:block) => {
    pub unsafe extern "C" fn $func(regs: *mut pt_regs) -> libc::c_long { let mut nmi_state = interrupt_nmi_state {}; arch_interrupt_nmi_enter_prepare(regs, &mut nmi_state); let state = irqentry_nmi_enter(regs); let ret = $inner(regs); arch_interrupt_nmi_exit_prepare(regs, &mut nmi_state); irqentry_nmi_exit(regs, state); ret }
    unsafe fn $inner(regs: *mut pt_regs) -> libc::c_long $body
}; }

extern "C" {
    fn unrecoverable_exception(regs: *mut pt_regs) -> !;
    fn replay_system_reset();
    fn replay_soft_interrupts();
    fn system_call_exception(regs: *mut pt_regs, r0: libc::c_ulong) -> libc::c_long;
    fn syscall_exit_prepare(r3: libc::c_ulong, regs: *mut pt_regs, scv: libc::c_long) -> libc::c_ulong;
    fn interrupt_exit_user_prepare(regs: *mut pt_regs) -> libc::c_ulong;
    fn interrupt_exit_kernel_prepare(regs: *mut pt_regs) -> libc::c_ulong;
}

#[inline]
pub unsafe fn interrupt_cond_local_irq_enable(regs: *mut pt_regs) {
    if !regs_irqs_disabled(regs) { local_irq_enable(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
