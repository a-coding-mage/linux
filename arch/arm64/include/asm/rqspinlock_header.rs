/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the original header includes <asm/barrier.h> and
// <asm-generic/rqspinlock.h>. Their symbols are supplied by other translated
// files.

/*
 * Hardcode res_smp_cond_load_acquire implementations for arm64 to a custom
 * version based on [0]. In rqspinlock code, our conditional expression involves
 * checking the value _and_ additionally a timeout. However, on arm64, the WFE-
 * based implementation may never spin again if no stores occur to the locked
 * byte in the lock word. As such, we may be stuck forever if event-stream based
 * unblocking is not available on the platform for WFE spin loops
 * (arch_timer_evtstrm_available).
 *
 * Once support for smp_cond_load_acquire_timewait [0] lands, we can drop this
 * copy-paste.
 *
 * While we rely on the implementation to amortize the cost of sampling
 * cond_expr for us, it will not happen when event stream support is unavailable,
 * time_expr check is amortized. This is not the common case, and it would be
 * difficult to fit our logic in the time_expr_ns >= time_limit_ns comparison,
 * hence just let it be. In case of event-stream, the loop is woken up at
 * microsecond granularity.
 *
 * [0]: https://lore.kernel.org/lkml/20250203214911.898276-1-ankur.a.arora@oracle.com
 */

// Build-time condition from the C header: only define these fallbacks when
// smp_cond_load_acquire_timewait is not already provided.

pub const SMP_COND_TIME_CHECK_COUNT: u32 = 200;

// C macro translation. `ptr`, `cond_expr`, `time_expr_ns`, and `time_limit_ns`
// are intentionally macro expressions so their evaluation and scope follow the
// source-level interface. External helpers are supplied by barrier code.
#[macro_export]
macro_rules! __smp_cond_load_relaxed_spinwait {
    ($ptr:expr, $cond_expr:expr, $time_expr_ns:expr, $time_limit_ns:expr) => {{
        let __ptr = $ptr;
        let mut __val;
        let mut __count: u32 = 0;
        loop {
            __val = unsafe { core::ptr::read_volatile(__ptr) };
            if $cond_expr {
                break;
            }
            cpu_relax();
            if {
                let old = __count;
                __count = __count.wrapping_add(1);
                old < SMP_COND_TIME_CHECK_COUNT
            } {
                continue;
            }
            if ($time_expr_ns) >= ($time_limit_ns) {
                break;
            }
            __count = 0;
        }
        __val
    }};
}

#[macro_export]
macro_rules! __smp_cond_load_acquire_timewait {
    ($ptr:expr, $cond_expr:expr, $time_expr_ns:expr, $time_limit_ns:expr) => {{
        let __ptr = $ptr;
        let mut __val;
        loop {
            __val = smp_load_acquire(__ptr);
            if $cond_expr {
                break;
            }
            __cmpwait_relaxed(__ptr, __val);
            if ($time_expr_ns) >= ($time_limit_ns) {
                break;
            }
        }
        __val
    }};
}

#[macro_export]
macro_rules! smp_cond_load_acquire_timewait {
    ($ptr:expr, $cond_expr:expr, $time_expr_ns:expr, $time_limit_ns:expr) => {{
        let mut _val;
        let __wfe: i32 = arch_timer_evtstrm_available();
        if likely(__wfe != 0) {
            _val = $crate::__smp_cond_load_acquire_timewait!(
                $ptr, $cond_expr, $time_expr_ns, $time_limit_ns
            );
        } else {
            _val = $crate::__smp_cond_load_relaxed_spinwait!(
                $ptr, $cond_expr, $time_expr_ns, $time_limit_ns
            );
            smp_acquire__after_ctrl_dep();
        }
        _val
    }};
}

#[macro_export]
macro_rules! res_smp_cond_load_acquire {
    ($v:expr, $c:expr) => {
        $crate::smp_cond_load_acquire_timewait!($v, $c, 0, 1)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
