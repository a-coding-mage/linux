/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers:
// linux/bits.h, linux/param.h, linux/spinlock_types_raw.h

pub const DEFAULT_RATELIMIT_INTERVAL: _ = 5 * HZ;
pub const DEFAULT_RATELIMIT_BURST: i32 = 10;

/* issue num suppressed message on exit */
pub const RATELIMIT_MSG_ON_RELEASE: _ = BIT(0);
pub const RATELIMIT_INITIALIZED: _ = BIT(1);

#[repr(C)]
pub struct ratelimit_state {
    pub lock: raw_spinlock_t, /* protect the state */

    pub interval: i32,
    pub burst: i32,
    pub rs_n_left: atomic_t,
    pub missed: atomic_t,
    pub flags: u32,
    pub begin: ::core::ffi::c_ulong,
}

#[macro_export]
macro_rules! RATELIMIT_STATE_INIT_FLAGS {
    ($name:ident, $interval_init:expr, $burst_init:expr, $flags_init:expr) => {
        ratelimit_state {
            lock: __RAW_SPIN_LOCK_UNLOCKED!($name.lock),
            interval: $interval_init,
            burst: $burst_init,
            flags: $flags_init,
            ..unsafe { ::core::mem::zeroed() }
        }
    };
}

#[macro_export]
macro_rules! RATELIMIT_STATE_INIT {
    ($name:ident, $interval_init:expr, $burst_init:expr) => {
        RATELIMIT_STATE_INIT_FLAGS!($name, $interval_init, $burst_init, 0)
    };
}

#[macro_export]
macro_rules! RATELIMIT_STATE_INIT_DISABLED {
    () => {
        RATELIMIT_STATE_INIT!(ratelimit_state, 0, DEFAULT_RATELIMIT_BURST)
    };
}

#[macro_export]
macro_rules! DEFINE_RATELIMIT_STATE {
    ($name:ident, $interval_init:expr, $burst_init:expr) => {
        let mut $name: ratelimit_state =
            RATELIMIT_STATE_INIT!($name, $interval_init, $burst_init);
    };
}

unsafe extern "C" {
    pub fn ___ratelimit(rs: *mut ratelimit_state, func: *const ::core::ffi::c_char) -> i32;
}

#[macro_export]
macro_rules! __ratelimit {
    ($state:expr) => {
        ___ratelimit($state, concat!(module_path!(), "\0").as_ptr() as *const ::core::ffi::c_char)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
