/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// <asm/ibt.h>

// For x86-64, PV_CALLEE_SAVE_REGS_THUNK() saves and restores 8 64-bit
// registers. For i386, however, only 1 32-bit register needs to be saved
// and restored. So an optimized version of __pv_queued_spin_unlock() is
// hand-coded for 64-bit, but it isn't worthwhile to do it for 32-bit.

extern "C" {
    pub fn __pv_queued_spin_unlock_slowpath(lock: *mut qspinlock, locked: u8);
}

#[cfg(target_pointer_width = "64")]
pub const __PV_CALLEE_SAVE_REGS_THUNK___PV_QUEUED_SPIN_UNLOCK_SLOWPATH_SECTION: &str =
    ".spinlock.text";

#[cfg(target_pointer_width = "64")]
pub const PV_UNLOCK_ASM: &str = concat!(
    "FRAME_BEGIN\n\t",
    "push  %rdx\n\t",
    "mov   $", "_Q_LOCKED_VAL", ",%eax\n\t",
    "xor   %edx,%edx\n\t",
    "LOCK_PREFIXcmpxchg %dl,(%rdi)\n\t",
    "jne   .slowpath\n\t",
    "pop   %rdx\n\t",
    "FRAME_END\n\t",
    "ASM_RET\n\t",
    ".slowpath:\n\t",
    "push   %rsi\n\t",
    "movzbl %al,%esi\n\t",
    "call __raw_callee_save___pv_queued_spin_unlock_slowpath\n\t",
    "pop    %rsi\n\t",
    "pop    %rdx\n\t",
    "FRAME_END"
);

// Optimized assembly version of __raw_callee_save___pv_queued_spin_unlock.
// The original DEFINE_ASM_FUNC places this function in .spinlock.text.
#[cfg(target_pointer_width = "64")]
pub const __RAW_CALLEE_SAVE___PV_QUEUED_SPIN_UNLOCK: (&str, &str, &str) = (
    "__raw_callee_save___pv_queued_spin_unlock",
    PV_UNLOCK_ASM,
    ".spinlock.text",
);

// On 32-bit x86, __PV_CALLEE_SAVE_REGS_THUNK(__pv_queued_spin_unlock,
// ".spinlock.text") emits the callee-save thunk.
#[cfg(target_pointer_width = "32")]
extern "C" {
    pub fn __pv_queued_spin_unlock(lock: *mut qspinlock);
}

#[cfg(target_pointer_width = "32")]
pub const __PV_CALLEE_SAVE_REGS_THUNK___PV_QUEUED_SPIN_UNLOCK_SECTION: &str =
    ".spinlock.text";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
