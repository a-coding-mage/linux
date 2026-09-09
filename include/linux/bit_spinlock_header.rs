/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/preempt.h, linux/atomic.h, linux/bug.h,
// and asm/processor.h (for cpu_relax()).

/*
 * For static context analysis, we need a unique token for each possible bit
 * that can be used as a bit_spinlock. The easiest way to do that is to create a
 * fake context that we can cast to with the __bitlock(bitnum, addr) macro
 * below, which will give us unique instances for each (bit, addr) pair that the
 * static analysis can use.
 */
#[repr(C)]
pub struct __context_bitlock {}

#[inline(always)]
unsafe fn __bitlock(bitnum: i32, addr: *mut usize) -> *mut __context_bitlock {
    (bitnum as usize).wrapping_add(addr as usize) as *mut __context_bitlock
}

/*
 *  bit-based spin_lock()
 *
 * Don't use this unless you really need to: spin_lock() and spin_unlock()
 * are significantly faster.
 */
#[inline(always)]
pub unsafe fn bit_spin_lock(bitnum: i32, addr: *mut usize) {
    /*
     * Assuming the lock is uncontended, this never enters
     * the body of the outer loop. If it is contended, then
     * within the inner loop a non-atomic test is used to
     * busywait with less bus contention for a good time to
     * attempt to acquire the lock bit.
     */
    preempt_disable();
    // Equivalent of: #if defined(CONFIG_SMP) || defined(CONFIG_DEBUG_SPINLOCK)
    #[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
    while (unlikely(test_and_set_bit_lock(bitnum, addr))) {
        preempt_enable();
        loop {
            cpu_relax();
            if !test_bit(bitnum, addr) {
                break;
            }
        }
        preempt_disable();
    }
    // #endif
    // __acquire(__bitlock(bitnum, addr));
}

/*
 * Return true if it was acquired
 */
#[inline(always)]
pub unsafe fn bit_spin_trylock(bitnum: i32, addr: *mut usize) -> i32 {
    preempt_disable();
    // Equivalent of: #if defined(CONFIG_SMP) || defined(CONFIG_DEBUG_SPINLOCK)
    #[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
    if unlikely(test_and_set_bit_lock(bitnum, addr)) {
        preempt_enable();
        return 0;
    }
    // #endif
    // __acquire(__bitlock(bitnum, addr));
    1
}

/*
 *  bit-based spin_unlock()
 */
#[inline(always)]
pub unsafe fn bit_spin_unlock(bitnum: i32, addr: *mut usize) {
    // Equivalent of: #ifdef CONFIG_DEBUG_SPINLOCK
    #[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
    bug_on(!test_bit(bitnum, addr));
    // #endif
    // Equivalent of: #if defined(CONFIG_SMP) || defined(CONFIG_DEBUG_SPINLOCK)
    #[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
    clear_bit_unlock(bitnum, addr);
    // #endif
    preempt_enable();
    // __release(__bitlock(bitnum, addr));
}

/*
 *  bit-based spin_unlock()
 *  non-atomic version, which can be used eg. if the bit lock itself is
 *  protecting the rest of the flags in the word.
 */
#[inline(always)]
pub unsafe fn __bit_spin_unlock(bitnum: i32, addr: *mut usize) {
    // Equivalent of: #ifdef CONFIG_DEBUG_SPINLOCK
    #[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
    bug_on(!test_bit(bitnum, addr));
    // #endif
    // Equivalent of: #if defined(CONFIG_SMP) || defined(CONFIG_DEBUG_SPINLOCK)
    #[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
    __clear_bit_unlock(bitnum, addr);
    // #endif
    preempt_enable();
    // __release(__bitlock(bitnum, addr));
}

/*
 * Return true if the lock is held.
 */
#[inline]
pub unsafe fn bit_spin_is_locked(bitnum: i32, addr: *mut usize) -> i32 {
    // Equivalent of: #if defined(CONFIG_SMP) || defined(CONFIG_DEBUG_SPINLOCK)
    #[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
    {
        return test_bit(bitnum, addr) as i32;
    }
    // #elif defined(CONFIG_PREEMPT_COUNT)
    #[cfg(all(
        not(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK")),
        feature = "CONFIG_PREEMPT_COUNT"
    ))]
    {
        return preempt_count() as i32;
    }
    // #else
    #[cfg(all(
        not(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK")),
        not(feature = "CONFIG_PREEMPT_COUNT")
    ))]
    {
        return 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
