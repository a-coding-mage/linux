/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/jump_label.h, asm/cpufeature.h, asm-generic/qspinlock_types.h,
// asm/paravirt.h, asm/rmwcc.h, and (when CONFIG_PARAVIRT is enabled)
// asm/paravirt-spinlock.h.

pub const _Q_PENDING_LOOPS: i32 = 1 << 9;

// The original header aliases this symbol to itself so the generic qspinlock
// implementation can refer to the x86 implementation.
#[inline(always)]
pub unsafe fn queued_fetch_set_pending_acquire(lock: *mut qspinlock) -> u32 {
    let mut val: u32;

    /*
     * We can't use GEN_BINARY_RMWcc() inside an if() stmt because asm goto
     * and CONFIG_PROFILE_ALL_BRANCHES=y results in a label inside a
     * statement expression, which GCC doesn't like.
     */
    val = GEN_BINARY_RMWcc!(
        concat!(LOCK_PREFIX, "btsl"),
        (*lock).val.counter,
        c,
        "I",
        _Q_PENDING_OFFSET
    ) * _Q_PENDING_VAL;
    val |= atomic_read(&(*lock).val) & !_Q_PENDING_MASK;

    val
}

#[cfg(not(CONFIG_PARAVIRT))]
#[inline]
pub unsafe fn native_pv_lock_init() {}

// The generic qspinlock implementation is included here by the C header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
