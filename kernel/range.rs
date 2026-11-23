// SPDX-License-Identifier: GPL-2.0
/*
 * Range add and subtract
 */

// Dependencies: linux/init.h, linux/minmax.h, linux/printk.h, linux/sort.h,
// linux/string.h, linux/range.h.

use core::cmp::{max, min};
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::copy;
use kernel::bindings::{_printk, range};
use kernel::ffi::c_int;

type CmpFn = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;
type SwapFn = unsafe extern "C" fn(*mut c_void, *mut c_void, c_int);

extern "C" {
    fn sort(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpFn>, swap_func: Option<SwapFn>);
}

/// `range[index]`.
///
/// # Safety
/// `range` covers `index`.
unsafe fn at(range: *mut range, index: c_int) -> *mut range {
    // SAFETY: forwarded caller contract.
    unsafe { range.offset(index as isize) }
}

/// Appends `start..end` to the array; returns the new range count.
///
/// # Safety
/// `range` holds `az` entries.
#[no_mangle]
pub unsafe extern "C" fn add_range(range: *mut range, az: c_int, mut nr_range: c_int, start: u64, end: u64) -> c_int {
    if start >= end {
        return nr_range;
    }

    /* Out of slots: */
    if nr_range >= az {
        return nr_range;
    }

    // SAFETY: nr_range < az.
    unsafe {
        (*at(range, nr_range)).start = start;
        (*at(range, nr_range)).end = end;
    }

    nr_range = nr_range.wrapping_add(1);

    nr_range
}

/// Merges `start..end` with overlapping ranges, then appends it.
///
/// # Safety
/// `range` holds `az` entries, the first `nr_range` in use.
#[no_mangle]
pub unsafe extern "C" fn add_range_with_merge(
    range: *mut range,
    az: c_int,
    mut nr_range: c_int,
    mut start: u64,
    mut end: u64,
) -> c_int {
    if start >= end {
        return nr_range;
    }

    // SAFETY: indices stay below nr_range.
    unsafe {
        /* get new start/end: */
        let mut i: c_int = 0;
        while i < nr_range {
            let r = at(range, i);

            if (*r).end != 0 {
                let common_start = max((*r).start, start);
                let common_end = min((*r).end, end);
                if common_start <= common_end {
                    /* new start/end, will add it back at last */
                    start = min((*r).start, start);
                    end = max((*r).end, end);

                    copy(at(range, i + 1), r, nr_range.wrapping_sub(i + 1) as usize);
                    (*at(range, nr_range - 1)).start = 0;
                    (*at(range, nr_range - 1)).end = 0;
                    nr_range -= 1;
                    i -= 1;
                }
            }
            i += 1;
        }

        /* Need to add it: */
        add_range(range, az, nr_range, start, end)
    }
}

/// Removes `start..end` from every range of the array.
///
/// # Safety
/// `range` holds `az` entries.
#[no_mangle]
pub unsafe extern "C" fn subtract_range(range: *mut range, az: c_int, start: u64, end: u64) {
    if start >= end {
        return;
    }

    // SAFETY: indices stay below az.
    unsafe {
        for j in 0..az {
            let r = at(range, j);

            if (*r).end == 0 {
                continue;
            }

            if start <= (*r).start && end >= (*r).end {
                (*r).start = 0;
                (*r).end = 0;
                continue;
            }

            if start <= (*r).start && end < (*r).end && (*r).start < end {
                (*r).start = end;
                continue;
            }

            if start > (*r).start && end >= (*r).end && (*r).end > start {
                (*r).end = start;
                continue;
            }

            if start > (*r).start && end < (*r).end {
                /* Find the new spare: */
                let mut i: c_int = 0;
                while i < az {
                    if (*at(range, i)).end == 0 {
                        break;
                    }
                    i += 1;
                }
                if i < az {
                    (*at(range, i)).end = (*r).end;
                    (*at(range, i)).start = end;
                } else {
                    _printk(c"\x013%s: run out of slot in ranges\n".as_ptr().cast(), c"subtract_range".as_ptr());
                }
                (*r).end = start;
                continue;
            }
        }
    }
}

/// # Safety
/// `x1` and `x2` point to ranges.
unsafe extern "C" fn cmp_range(x1: *const c_void, x2: *const c_void) -> c_int {
    // SAFETY: forwarded sort() contract.
    unsafe {
        let r1 = x1.cast::<range>();
        let r2 = x2.cast::<range>();

        if (*r1).start < (*r2).start {
            return -1;
        }
        if (*r1).start > (*r2).start {
            return 1;
        }
    }
    0
}

/// Compacts empty slots to the end, then sorts the used ranges.
///
/// # Safety
/// `range` holds `az` entries.
#[no_mangle]
pub unsafe extern "C" fn clean_sort_range(range: *mut range, az: c_int) -> c_int {
    let mut k: c_int = az.wrapping_sub(1);
    let mut nr_range: c_int = az;

    // SAFETY: indices stay below az.
    unsafe {
        let mut i: c_int = 0;
        while i < k {
            if (*at(range, i)).end != 0 {
                i += 1;
                continue;
            }
            let mut j: c_int = k;
            while j > i {
                if (*at(range, j)).end != 0 {
                    k = j;
                    break;
                }
                j = j.wrapping_sub(1);
            }
            if j == i {
                break;
            }
            (*at(range, i)).start = (*at(range, k)).start;
            (*at(range, i)).end = (*at(range, k)).end;
            (*at(range, k)).start = 0;
            (*at(range, k)).end = 0;
            k = k.wrapping_sub(1);
            i += 1;
        }
        /* count it */
        for i in 0..az {
            if (*at(range, i)).end == 0 {
                nr_range = i;
                break;
            }
        }

        /* sort them */
        sort(range.cast::<c_void>(), nr_range as usize, size_of::<range>(), Some(cmp_range), None);
    }

    nr_range
}

/// Sorts the first `nr_range` ranges by start.
///
/// # Safety
/// `range` holds `nr_range` entries.
#[no_mangle]
pub unsafe extern "C" fn sort_range(range: *mut range, nr_range: c_int) {
    /* sort them */
    // SAFETY: forwarded caller contract.
    unsafe { sort(range.cast::<c_void>(), nr_range as usize, size_of::<range>(), Some(cmp_range), None) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
