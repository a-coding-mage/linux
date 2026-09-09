/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; the C preprocessor header guard is not represented in Rust.

pub const ARCH_EXIT_TO_USER_MODE_WORK: ::core::ffi::c_ulong =
    _TIF_GUARDED_STORAGE | _TIF_PER_TRAP;

extern "C" {
    pub fn do_per_trap(regs: *mut pt_regs);
}

#[inline(always)]
pub unsafe fn arch_enter_from_user_mode(regs: *mut pt_regs) {
    // CONFIG_DEBUG_ENTRY is a build-time condition from the original source.
    if IS_ENABLED_CONFIG_DEBUG_ENTRY {
        debug_user_asce(0);
    }

    pai_kernel_enter(regs);
}

#[inline(always)]
pub unsafe fn arch_exit_to_user_mode_work(
    regs: *mut pt_regs,
    ti_work: ::core::ffi::c_ulong,
) {
    if ti_work & _TIF_PER_TRAP != 0 {
        clear_thread_flag(TIF_PER_TRAP);
        do_per_trap(regs);
    }

    if ti_work & _TIF_GUARDED_STORAGE != 0 {
        gs_load_bc_cb(regs);
    }
}

#[inline(always)]
pub unsafe fn arch_exit_to_user_mode() {
    load_user_fpu_regs();

    // CONFIG_DEBUG_ENTRY is a build-time condition from the original source.
    if IS_ENABLED_CONFIG_DEBUG_ENTRY {
        debug_user_asce(1);
    }

    pai_kernel_exit(current_pt_regs());
}

#[inline(always)]
pub unsafe fn arch_in_rcu_eqs() -> bool {
    // CONFIG_KVM is a build-time condition from the original source.
    if IS_ENABLED_CONFIG_KVM {
        return (*current).flags & PF_VCPU != 0;
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
