/* SPDX-License-Identifier: GPL-2.0 */

// Kernel dependencies supplied by other translated units:
// atomic_long_t, LONG_MAX, PAGE_SIZE, BITS_PER_LONG, and configuration flags.

#[repr(C)]
pub struct page_counter {
    // Keep usage separate from other fields in the v2 cacheline.
    pub usage: atomic_long_t,
    pub failcnt: kernel::ffi::c_ulong, // v1-only field

    // CACHELINE_PADDING(_pad1_);

    // effective memory.min and memory.min usage tracking
    pub emin: kernel::ffi::c_ulong,
    pub min_usage: atomic_long_t,
    pub children_min_usage: atomic_long_t,

    // effective memory.low and memory.low usage tracking
    pub elow: kernel::ffi::c_ulong,
    pub low_usage: atomic_long_t,
    pub children_low_usage: atomic_long_t,

    pub watermark: kernel::ffi::c_ulong,
    // Latest cg2 reset watermark
    pub local_watermark: kernel::ffi::c_ulong,

    // Keep all the read-most fields in a separate cacheline.
    // CACHELINE_PADDING(_pad2_);

    pub protection_support: bool,
    pub track_failcnt: bool,
    pub min: kernel::ffi::c_ulong,
    pub low: kernel::ffi::c_ulong,
    pub high: kernel::ffi::c_ulong,
    pub max: kernel::ffi::c_ulong,
    pub parent: *mut page_counter,
}

#[cfg(target_pointer_width = "32")]
pub const PAGE_COUNTER_MAX: kernel::ffi::c_ulong = kernel::ffi::c_long::MAX as kernel::ffi::c_ulong;
#[cfg(not(target_pointer_width = "32"))]
pub const PAGE_COUNTER_MAX: kernel::ffi::c_ulong =
    (kernel::ffi::c_long::MAX as kernel::ffi::c_ulong) / (PAGE_SIZE as kernel::ffi::c_ulong);

// Protection is supported only for the first counter (with id 0).
#[inline]
pub unsafe fn page_counter_init(
    counter: *mut page_counter,
    parent: *mut page_counter,
    protection_support: bool,
) {
    (*counter).usage = ATOMIC_LONG_INIT(0);
    (*counter).max = PAGE_COUNTER_MAX;
    (*counter).parent = parent;
    (*counter).protection_support = protection_support;
    (*counter).track_failcnt = false;
}

#[inline]
pub unsafe fn page_counter_read(counter: *mut page_counter) -> kernel::ffi::c_ulong {
    atomic_long_read(&(*counter).usage)
}

extern "C" {
    pub fn page_counter_cancel(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong);
    pub fn page_counter_charge(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong);
    pub fn page_counter_try_charge(
        counter: *mut page_counter,
        nr_pages: kernel::ffi::c_ulong,
        fail: *mut *mut page_counter,
    ) -> bool;
    pub fn page_counter_uncharge(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong);
    pub fn page_counter_set_min(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong);
    pub fn page_counter_set_low(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong);
}

#[inline]
pub unsafe fn page_counter_set_high(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong) {
    // WRITE_ONCE(counter->high, nr_pages)
    core::ptr::write_volatile(&mut (*counter).high, nr_pages);
}

extern "C" {
    pub fn page_counter_set_max(counter: *mut page_counter, nr_pages: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
    pub fn page_counter_memparse(
        buf: *const kernel::ffi::c_char,
        max: *const kernel::ffi::c_char,
        nr_pages: *mut kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;
}

#[inline]
pub unsafe fn page_counter_reset_watermark(counter: *mut page_counter) {
    let usage = page_counter_read(counter);

    // Update local_watermark first, so it is always <= watermark
    // (modulo CPU/compiler re-ordering).
    (*counter).local_watermark = usage;
    (*counter).watermark = usage;
}

// CONFIG_MEMCG || CONFIG_CGROUP_DMEM
extern "C" {
    pub fn page_counter_calculate_protection(
        root: *mut page_counter,
        counter: *mut page_counter,
        recursive_protection: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
