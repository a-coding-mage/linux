/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/ptrace.h, linux/signal.h, and
// asm/processor-flags.h.

pub const FIX_EFLAGS: u64 = X86_EFLAGS_AC
    | X86_EFLAGS_OF
    | X86_EFLAGS_DF
    | X86_EFLAGS_TF
    | X86_EFLAGS_SF
    | X86_EFLAGS_ZF
    | X86_EFLAGS_AF
    | X86_EFLAGS_PF
    | X86_EFLAGS_CF
    | X86_EFLAGS_RF;

extern "C" {
    pub fn signal_fault(regs: *mut crate::pt_regs, frame: *mut core::ffi::c_void, where_: *mut core::ffi::c_char);

    pub fn get_sigframe(
        ksig: *mut crate::ksignal,
        regs: *mut crate::pt_regs,
        frame_size: usize,
        fpstate: *mut *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;

    pub fn ia32_setup_frame(ksig: *mut crate::ksignal, regs: *mut crate::pt_regs) -> i32;
    pub fn ia32_setup_rt_frame(ksig: *mut crate::ksignal, regs: *mut crate::pt_regs) -> i32;
    pub fn x64_setup_rt_frame(ksig: *mut crate::ksignal, regs: *mut crate::pt_regs) -> i32;
    pub fn x32_setup_rt_frame(ksig: *mut crate::ksignal, regs: *mut crate::pt_regs) -> i32;
}

/*
 * To prevent immediate repeat of single step trap on return from SIGTRAP
 * handler if the trap flag (TF) is set without an external debugger attached,
 * clear the software event flag in the augmented SS, ensuring no single-step
 * trap is pending upon ERETU completion.
 *
 * Note, this function should be called in sigreturn() before the original
 * state is restored to make sure the TF is read from the entry frame.
 */
#[inline(always)]
pub unsafe fn prevent_single_step_upon_eretu(regs: *mut crate::pt_regs) {
    /*
     * If the trap flag (TF) is set, i.e., the sigreturn() SYSCALL instruction
     * is being single-stepped, do not clear the software event flag in the
     * augmented SS, thus a debugger won't skip over the following instruction.
     */
    // CONFIG_X86_FRED is a build-time condition supplied by the surrounding build.
    #[cfg(CONFIG_X86_FRED)]
    if !((*regs).flags & X86_EFLAGS_TF != 0) {
        (*regs).fred_ss.swevent = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
