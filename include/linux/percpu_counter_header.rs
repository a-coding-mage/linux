/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/percpu_counter.h. */

/* percpu_counter batch for local add or sub */
pub const PERCPU_COUNTER_LOCAL_BATCH: i32 = i32::MAX;

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct percpu_counter {
    pub lock: raw_spinlock_t,
    pub count: i64,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub list: list_head, /* All percpu_counters are on a list */
    pub counters: *mut i32,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut percpu_counter_batch: i32;
    pub fn __percpu_counter_init_many(
        fbc: *mut percpu_counter,
        amount: i64,
        gfp: gfp_t,
        nr_counters: u32,
        key: *mut lock_class_key,
    ) -> i32;
    pub fn percpu_counter_destroy_many(fbc: *mut percpu_counter, nr_counters: u32);
    pub fn percpu_counter_set(fbc: *mut percpu_counter, amount: i64);
    pub fn percpu_counter_add_batch(fbc: *mut percpu_counter, amount: i64, batch: i32);
    pub fn __percpu_counter_sum(fbc: *mut percpu_counter) -> i64;
    pub fn __percpu_counter_compare(fbc: *mut percpu_counter, rhs: i64, batch: i32) -> i32;
    pub fn __percpu_counter_limited_add(
        fbc: *mut percpu_counter,
        limit: i64,
        amount: i64,
        batch: i32,
    ) -> bool;
    pub fn percpu_counter_sync(fbc: *mut percpu_counter);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_init_many(
    fbc: *mut percpu_counter, value: i64, gfp: gfp_t, nr_counters: u32,
) -> i32 {
    static mut KEY: lock_class_key = lock_class_key { _private: [] };
    __percpu_counter_init_many(fbc, value, gfp, nr_counters, &mut KEY)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_init(fbc: *mut percpu_counter, value: i64, gfp: gfp_t) -> i32 {
    percpu_counter_init_many(fbc, value, gfp, 1)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_destroy(fbc: *mut percpu_counter) {
    percpu_counter_destroy_many(fbc, 1)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_compare(fbc: *mut percpu_counter, rhs: i64) -> i32 {
    __percpu_counter_compare(fbc, rhs, percpu_counter_batch)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_add(fbc: *mut percpu_counter, amount: i64) {
    percpu_counter_add_batch(fbc, amount, percpu_counter_batch)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_limited_add(fbc: *mut percpu_counter, limit: i64, amount: i64) -> bool {
    __percpu_counter_limited_add(fbc, limit, amount, percpu_counter_batch)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_add_local(fbc: *mut percpu_counter, amount: i64) {
    percpu_counter_add_batch(fbc, amount, PERCPU_COUNTER_LOCAL_BATCH)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_sum_positive(fbc: *mut percpu_counter) -> i64 {
    let ret = __percpu_counter_sum(fbc);
    if ret < 0 { 0 } else { ret }
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_sum(fbc: *mut percpu_counter) -> i64 { __percpu_counter_sum(fbc) }

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_read(fbc: *mut percpu_counter) -> i64 { (*fbc).count }

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_read_positive(fbc: *mut percpu_counter) -> i64 {
    let ret = core::ptr::read_volatile(&(*fbc).count);
    if ret >= 0 { ret } else { 0 }
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn percpu_counter_initialized(fbc: *mut percpu_counter) -> bool {
    !(*fbc).counters.is_null()
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[repr(C)]
pub struct percpu_counter { pub count: i64 }

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_init_many(fbc: *mut percpu_counter, amount: i64, _gfp: gfp_t, nr_counters: u32) -> i32 {
    for i in 0..nr_counters { (*fbc.add(i as usize)).count = amount; }
    0
}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_init(fbc: *mut percpu_counter, amount: i64, gfp: gfp_t) -> i32 { percpu_counter_init_many(fbc, amount, gfp, 1) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_destroy_many(_fbc: *mut percpu_counter, _nr_counters: u32) {}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_destroy(_fbc: *mut percpu_counter) {}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_set(fbc: *mut percpu_counter, amount: i64) { (*fbc).count = amount; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_compare(fbc: *mut percpu_counter, rhs: i64) -> i32 { ((*fbc).count > rhs) as i32 - ((*fbc).count < rhs) as i32 }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn __percpu_counter_compare(fbc: *mut percpu_counter, rhs: i64, _batch: i32) -> i32 { percpu_counter_compare(fbc, rhs) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_add(fbc: *mut percpu_counter, amount: i64) { (*fbc).count = (*fbc).count.wrapping_add(amount); }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_limited_add(fbc: *mut percpu_counter, limit: i64, amount: i64) -> bool {
    if amount == 0 { return true; }
    let count = (*fbc).count.wrapping_add(amount);
    if (amount > 0 && count <= limit) || (amount < 0 && count >= limit) { (*fbc).count = count; true } else { false }
}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_add_local(fbc: *mut percpu_counter, amount: i64) { percpu_counter_add(fbc, amount) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_add_batch(fbc: *mut percpu_counter, amount: i64, _batch: i32) { percpu_counter_add(fbc, amount) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_read(fbc: *mut percpu_counter) -> i64 { (*fbc).count }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_read_positive(fbc: *mut percpu_counter) -> i64 { (*fbc).count }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_sum_positive(fbc: *mut percpu_counter) -> i64 { percpu_counter_read_positive(fbc) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_sum(fbc: *mut percpu_counter) -> i64 { percpu_counter_read(fbc) }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_initialized(_fbc: *mut percpu_counter) -> bool { true }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn percpu_counter_sync(_fbc: *mut percpu_counter) {}

#[inline]
pub unsafe fn percpu_counter_inc(fbc: *mut percpu_counter) { percpu_counter_add(fbc, 1); }
#[inline]
pub unsafe fn percpu_counter_dec(fbc: *mut percpu_counter) { percpu_counter_add(fbc, -1); }
#[inline]
pub unsafe fn percpu_counter_sub(fbc: *mut percpu_counter, amount: i64) { percpu_counter_add(fbc, amount.wrapping_neg()); }
#[inline]
pub unsafe fn percpu_counter_sub_local(fbc: *mut percpu_counter, amount: i64) { percpu_counter_add_local(fbc, amount.wrapping_neg()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
