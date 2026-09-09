/* SPDX-License-Identifier: GPL-2.0 */

// Kernel dependencies supplied by other translated units:
// atomic_long_t, LONG_MAX, PAGE_SIZE, BITS_PER_LONG, and configuration flags.

#[repr(C)]
pub struct page_counter {
    // Keep usage separate from other fields in the v2 cacheline.
    pub usage: atomic_long_t,
    pub failcnt: libc::c_ulong, // v1-only field

    // CACHELINE_PADDING(_pad1_);

    // effective memory.min and memory.min usage tracking
    pub emin: libc::c_ulong,
    pub min_usage: atomic_long_t,
    pub children_min_usage: atomic_long_t,

    // effective memory.low and memory.low usage tracking
    pub elow: libc::c_ulong,
    pub low_usage: atomic_long_t,
    pub children_low_usage: atomic_long_t,

    pub watermark: libc::c_ulong,
    // Latest cg2 reset watermark
    pub local_watermark: libc::c_ulong,

    // Keep all the read-most fields in a separate cacheline.
    // CACHELINE_PADDING(_pad2_);

    pub protection_support: bool,
    pub track_failcnt: bool,
    pub min: libc::c_ulong,
    pub low: libc::c_ulong,
    pub high: libc::c_ulong,
    pub max: libc::c_ulong,
    pub parent: *mut page_counter,
}

#[cfg(target_pointer_width = "32")]
pub const PAGE_COUNTER_MAX: libc::c_ulong = libc::LONG_MAX as libc::c_ulong;
#[cfg(not(target_pointer_width = "32"))]
pub const PAGE_COUNTER_MAX: libc::c_ulong =
    (libc::LONG_MAX as libc::c_ulong) / (PAGE_SIZE as libc::c_ulong);

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
pub unsafe fn page_counter_read(counter: *mut page_counter) -> libc::c_ulong {
    atomic_long_read(&(*counter).usage)
}

extern "C" {
    pub fn page_counter_cancel(counter: *mut page_counter, nr_pages: libc::c_ulong);
    pub fn page_counter_charge(counter: *mut page_counter, nr_pages: libc::c_ulong);
    pub fn page_counter_try_charge(
        counter: *mut page_counter,
        nr_pages: libc::c_ulong,
        fail: *mut *mut page_counter,
    ) -> bool;
    pub fn page_counter_uncharge(counter: *mut page_counter, nr_pages: libc::c_ulong);
    pub fn page_counter_set_min(counter: *mut page_counter, nr_pages: libc::c_ulong);
    pub fn page_counter_set_low(counter: *mut page_counter, nr_pages: libc::c_ulong);
}

#[inline]
pub unsafe fn page_counter_set_high(counter: *mut page_counter, nr_pages: libc::c_ulong) {
    // WRITE_ONCE(counter->high, nr_pages)
    core::ptr::write_volatile(&mut (*counter).high, nr_pages);
}

extern "C" {
    pub fn page_counter_set_max(counter: *mut page_counter, nr_pages: libc::c_ulong) -> libc::c_int;
    pub fn page_counter_memparse(
        buf: *const libc::c_char,
        max: *const libc::c_char,
        nr_pages: *mut libc::c_ulong,
    ) -> libc::c_int;
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
