/* SPDX-License-Identifier: GPL-2.0 */

// `u64` and `bool` correspond to the types supplied by <linux/types.h>.

#[repr(C)]
pub struct range {
    pub start: u64,
    pub end: u64,
}

#[inline]
pub unsafe fn range_len(range: *const range) -> u64 {
    (*range).end - (*range).start + 1
}

/* True if r1 completely contains r2 */
#[inline]
pub unsafe fn range_contains(r1: *const range, r2: *const range) -> bool {
    (*r1).start <= (*r2).start && (*r1).end >= (*r2).end
}

/* True if any part of r1 overlaps r2 */
#[inline]
pub unsafe fn range_overlaps(r1: *const range, r2: *const range) -> bool {
    (*r1).start <= (*r2).end && (*r1).end >= (*r2).start
}

extern "C" {
    pub fn add_range(
        range: *mut range,
        az: i32,
        nr_range: i32,
        start: u64,
        end: u64,
    ) -> i32;

    pub fn add_range_with_merge(
        range: *mut range,
        az: i32,
        nr_range: i32,
        start: u64,
        end: u64,
    ) -> i32;

    pub fn subtract_range(range: *mut range, az: i32, start: u64, end: u64);

    pub fn clean_sort_range(range: *mut range, az: i32) -> i32;

    pub fn sort_range(range: *mut range, nr_range: i32);
}

#[macro_export]
macro_rules! DEFINE_RANGE {
    ($start:expr, $end:expr) => {
        $crate::range {
            start: $start,
            end: $end,
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
