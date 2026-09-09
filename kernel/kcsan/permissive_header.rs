/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Special rules for ignoring entire classes of data-racy memory accesses. None
 * of the rules here imply that such data races are generally safe!
 *
 * All rules in this file can be configured via CONFIG_KCSAN_PERMISSIVE. Keep
 * them separate from core code to make it easier to audit.
 *
 * Copyright (C) 2019, Google LLC.
 */

// The C header dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct task_struct {
    pub flags: usize,
}

extern "C" {
    pub static mut current: *mut task_struct;
    pub fn hweight64(value: u64) -> u32;
}

/* Access ignore rules based on address. */
#[inline(always)]
pub unsafe fn kcsan_ignore_address(ptr: *const core::ffi::c_void) -> bool {
    // Build-time CONFIG_KCSAN_PERMISSIVE is represented by this condition.
    if !cfg!(feature = "CONFIG_KCSAN_PERMISSIVE") {
        return false;
    }

    /*
     * Data-racy bitops on current->flags are too common, ignore completely
     * for now.
     */
    ptr == (&raw const (*current).flags).cast::<core::ffi::c_void>()
}

/* Data race ignore rules based on access type and value change patterns. */
pub unsafe fn kcsan_ignore_data_race(
    size: usize,
    type_: i32,
    old: u64,
    new: u64,
    diff: u64,
) -> bool {
    // Build-time CONFIG_KCSAN_PERMISSIVE is represented by this condition.
    if !cfg!(feature = "CONFIG_KCSAN_PERMISSIVE") {
        return false;
    }

    /*
     * Rules here are only for plain read accesses, so that we still report
     * data races between plain read-write accesses.
     */
    if type_ != 0 || size > core::mem::size_of::<usize>() {
        return false;
    }

    /*
     * A common pattern is checking/setting just 1 bit in a variable; for
     * example:
     *
     *	if (flags & SOME_FLAG) { ... }
     *
     * and elsewhere flags is updated concurrently:
     *
     *	flags |= SOME_OTHER_FLAG; // just 1 bit
     *
     * While it is still recommended that such accesses be marked
     * appropriately, in many cases these types of data races are so common
     * that marking them all is often unrealistic and left to maintainer
     * preference.
     *
     * The assumption in all cases is that with all known compiler
     * optimizations (including those that tear accesses), because no more
     * than 1 bit changed, the plain accesses are safe despite the presence
     * of data races.
     *
     * The rules here will ignore the data races if we observe no more than
     * 1 bit changed.
     *
     * Of course many operations can effecively change just 1 bit, but the
     * general assuption that data races involving 1-bit changes can be
     * tolerated still applies.
     *
     * And in case a true bug is missed, the bug likely manifests as a
     * reportable data race elsewhere.
     */
    if hweight64(diff) == 1 {
        /*
         * Exception: Report data races where the values look like
         * ordinary booleans (one of them was 0 and the 0th bit was
         * changed) More often than not, they come with interesting
         * memory ordering requirements, so let's report them.
         */
        if !((old == 0 || new == 0) && diff == 1) {
            return true;
        }
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
