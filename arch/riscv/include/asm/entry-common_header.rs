/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/stacktrace.h, asm/thread_info.h, and asm/vector.h.

/// Prepare to exit to user mode for RISC-V.
#[inline]
pub unsafe fn arch_exit_to_user_mode_prepare(
    regs: *mut pt_regs,
    ti_work: ::core::ffi::c_ulong,
) {
    if ti_work & _TIF_RISCV_V_DEFER_RESTORE != 0 {
        clear_thread_flag(TIF_RISCV_V_DEFER_RESTORE);
        /*
         * We are already called with irq disabled, so go without
         * keeping track of riscv_v_flags.
         */
        riscv_v_vstate_restore(&mut (*current).thread.vstate, regs);
    }
}

// #define arch_exit_to_user_mode_prepare arch_exit_to_user_mode_prepare

extern "C" {
    pub fn handle_page_fault(regs: *mut pt_regs);
    pub fn handle_break(regs: *mut pt_regs);
}

#[cfg(CONFIG_RISCV_MISALIGNED)]
extern "C" {
    pub fn handle_misaligned_load(regs: *mut pt_regs) -> ::core::ffi::c_int;
    pub fn handle_misaligned_store(regs: *mut pt_regs) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_RISCV_MISALIGNED))]
#[inline]
pub unsafe fn handle_misaligned_load(_regs: *mut pt_regs) -> ::core::ffi::c_int {
    -1
}

#[cfg(not(CONFIG_RISCV_MISALIGNED))]
#[inline]
pub unsafe fn handle_misaligned_store(_regs: *mut pt_regs) -> ::core::ffi::c_int {
    -1
}

extern "C" {
    pub fn handle_user_cfi_violation(regs: *mut pt_regs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
