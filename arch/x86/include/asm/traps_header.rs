/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/context_tracking_state.h, linux/kprobes.h, asm/debugreg.h,
// asm/idtentry.h, asm/siginfo.h (TRAP_TRACE, TRAP_HWBKPT, TRAP_BRKPT),
// and asm/trap_pf.h.

#[cfg(target_arch = "x86_64")]
extern "C" {
    pub fn sync_regs(eregs: *mut pt_regs) -> *mut pt_regs;
    pub fn fixup_bad_iret(bad_regs: *mut pt_regs) -> *mut pt_regs;
    pub fn vc_switch_off_ist(eregs: *mut pt_regs) -> *mut pt_regs;
}

extern "C" {
    pub fn ibt_selftest() -> ::core::ffi::c_int;
    pub fn ibt_selftest_noendbr() -> ::core::ffi::c_int;
}

#[cfg(CONFIG_X86_F00F_BUG)]
extern "C" {
    /* For handling the FOOF bug */
    pub fn handle_invalid_op(regs: *mut pt_regs);
}

pub fn handle_bug(regs: *mut pt_regs) -> bool;

#[inline]
pub unsafe fn get_si_code(condition: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    if condition & DR_STEP != 0 {
        TRAP_TRACE
    } else if condition & (DR_TRAP0 | DR_TRAP1 | DR_TRAP2 | DR_TRAP3) != 0 {
        TRAP_HWBKPT
    } else {
        TRAP_BRKPT
    }
}

extern "C" {
    pub fn math_emulate(info: *mut math_emu_info);
}

pub fn fault_in_kernel_space(address: ::core::ffi::c_ulong) -> bool;

#[cfg(CONFIG_VMAP_STACK)]
extern "C" {
    pub fn handle_stack_overflow(
        regs: *mut pt_regs,
        fault_address: ::core::ffi::c_ulong,
        info: *mut stack_info,
    ) -> !;
}

#[inline]
pub unsafe fn cond_local_irq_enable(regs: *mut pt_regs) {
    if (*regs).flags & X86_EFLAGS_IF != 0 {
        local_irq_enable();
    }
}

#[inline]
pub unsafe fn cond_local_irq_disable(regs: *mut pt_regs) {
    if (*regs).flags & X86_EFLAGS_IF != 0 {
        local_irq_disable();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
