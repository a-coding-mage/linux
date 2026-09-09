/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/ratelimit_types.h, linux/sched.h, and linux/spinlock.h.

#[inline]
pub unsafe fn ratelimit_state_init(
    rs: *mut ratelimit_state,
    interval: i32,
    burst: i32,
) {
    core::ptr::write_bytes(rs as *mut u8, 0, core::mem::size_of::<ratelimit_state>());

    raw_spin_lock_init(core::ptr::addr_of_mut!((*rs).lock));
    (*rs).interval = interval;
    (*rs).burst = burst;
}

#[inline]
pub unsafe fn ratelimit_default_init(rs: *mut ratelimit_state) {
    ratelimit_state_init(rs, DEFAULT_RATELIMIT_INTERVAL, DEFAULT_RATELIMIT_BURST);
}

#[inline]
pub unsafe fn ratelimit_state_inc_miss(rs: *mut ratelimit_state) {
    atomic_inc(core::ptr::addr_of_mut!((*rs).missed));
}

#[inline]
pub unsafe fn ratelimit_state_get_miss(rs: *mut ratelimit_state) -> i32 {
    atomic_read(core::ptr::addr_of_mut!((*rs).missed))
}

#[inline]
pub unsafe fn ratelimit_state_reset_miss(rs: *mut ratelimit_state) -> i32 {
    atomic_xchg_relaxed(core::ptr::addr_of_mut!((*rs).missed), 0)
}

#[inline]
pub unsafe fn ratelimit_state_reset_interval(rs: *mut ratelimit_state, interval_init: i32) {
    let mut flags: c_ulong = 0;

    raw_spin_lock_irqsave(core::ptr::addr_of_mut!((*rs).lock), &mut flags);
    (*rs).interval = interval_init;
    (*rs).flags &= !RATELIMIT_INITIALIZED;
    atomic_set(core::ptr::addr_of_mut!((*rs).rs_n_left), (*rs).burst);
    ratelimit_state_reset_miss(rs);
    raw_spin_unlock_irqrestore(core::ptr::addr_of_mut!((*rs).lock), flags);
}

#[inline]
pub unsafe fn ratelimit_state_exit(rs: *mut ratelimit_state) {
    let mut m: i32;

    if (*rs).flags & RATELIMIT_MSG_ON_RELEASE == 0 {
        return;
    }

    m = ratelimit_state_reset_miss(rs);
    if m != 0 {
        pr_warn(
            "%s: %d output lines suppressed due to ratelimiting\n",
            current.comm,
            m,
        );
    }
}

#[inline]
pub unsafe fn ratelimit_set_flags(rs: *mut ratelimit_state, flags: c_ulong) {
    (*rs).flags = flags;
}

extern "C" {
    pub static mut printk_ratelimit_state: ratelimit_state;
}

// The following CONFIG_PRINTK conditional is preserved from the C header.
// Define CONFIG_PRINTK in the surrounding build when selecting the printk form.
#[cfg(CONFIG_PRINTK)]
#[macro_export]
macro_rules! WARN_ON_RATELIMIT {
    ($condition:expr, $state:expr) => {{
        let __rtn_cond = $condition;
        WARN_ON(__rtn_cond && __ratelimit($state));
        __rtn_cond
    }};
}

#[cfg(CONFIG_PRINTK)]
#[macro_export]
macro_rules! WARN_RATELIMIT {
    ($condition:expr, $format:expr $(, $args:expr)*) => {{
        static mut _RS: ratelimit_state = ratelimit_state {
            lock: unsafe { core::mem::zeroed() },
            interval: DEFAULT_RATELIMIT_INTERVAL,
            burst: DEFAULT_RATELIMIT_BURST,
            flags: 0,
            missed: unsafe { core::mem::zeroed() },
            rs_n_left: unsafe { core::mem::zeroed() },
        };
        let rtn = $condition;
        if unlikely(rtn && unsafe { __ratelimit(core::ptr::addr_of_mut!(_RS)) }) {
            WARN!(rtn, $format $(, $args)*);
        }
        rtn
    }};
}

#[cfg(not(CONFIG_PRINTK))]
#[macro_export]
macro_rules! WARN_ON_RATELIMIT {
    ($condition:expr, $state:expr) => { WARN_ON($condition) };
}

#[cfg(not(CONFIG_PRINTK))]
#[macro_export]
macro_rules! WARN_RATELIMIT {
    ($condition:expr, $format:expr $(, $args:expr)*) => {{
        let rtn = WARN!($condition, $format $(, $args)*);
        rtn
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
