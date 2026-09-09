// SPDX-License-Identifier: GPL-2.0

// Dependency: linux/types.h

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
extern "C" {
    pub fn __hrtimer_rearm_deferred();
}

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
#[inline(always)]
pub unsafe fn hrtimer_test_and_clear_rearm_deferred_tif(tif_work: libc::c_ulong) -> bool {
    // lockdep_assert_irqs_disabled();

    if (tif_work & _TIF_HRTIMER_REARM) != 0 {
        clear_thread_flag(TIF_HRTIMER_REARM);
        return true;
    }
    false
}

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
pub const TIF_REARM_MASK: libc::c_ulong =
    _TIF_NEED_RESCHED | _TIF_NEED_RESCHED_LAZY | _TIF_HRTIMER_REARM;

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred_user_irq(
    tif_work: *mut libc::c_ulong,
    tif_mask: libc::c_ulong,
) -> bool {
    if (tif_mask & _TIF_HRTIMER_REARM) == 0 {
        return false;
    }
    if ((*tif_work & TIF_REARM_MASK) == _TIF_HRTIMER_REARM) {
        clear_thread_flag(TIF_HRTIMER_REARM);
        __hrtimer_rearm_deferred();
        *tif_work &= !_TIF_HRTIMER_REARM;
        return *tif_work == 0;
    }
    false
}

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred_tif(tif_work: libc::c_ulong) {
    if hrtimer_test_and_clear_rearm_deferred_tif(tif_work) {
        __hrtimer_rearm_deferred();
    }
}

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred() {
    hrtimer_rearm_deferred_tif(read_thread_flags());
}

#[cfg(CONFIG_HRTIMER_REARM_DEFERRED)]
#[inline(always)]
pub unsafe fn hrtimer_test_and_clear_rearm_deferred() -> bool {
    hrtimer_test_and_clear_rearm_deferred_tif(read_thread_flags())
}

#[cfg(not(CONFIG_HRTIMER_REARM_DEFERRED))]
#[inline(always)]
pub unsafe fn __hrtimer_rearm_deferred() {}

#[cfg(not(CONFIG_HRTIMER_REARM_DEFERRED))]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred() {}

#[cfg(not(CONFIG_HRTIMER_REARM_DEFERRED))]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred_tif(_tif_work: libc::c_ulong) {}

#[cfg(not(CONFIG_HRTIMER_REARM_DEFERRED))]
#[inline(always)]
pub unsafe fn hrtimer_rearm_deferred_user_irq(
    _tif_work: *mut libc::c_ulong,
    _tif_mask: libc::c_ulong,
) -> bool {
    false
}

#[cfg(not(CONFIG_HRTIMER_REARM_DEFERRED))]
#[inline(always)]
pub unsafe fn hrtimer_test_and_clear_rearm_deferred() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
