// SPDX-License-Identifier: GPL-2.0
/*
 * Range add and subtract
 */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct range {
    pub start: u64,
    pub end: u64,
}

unsafe extern "C" {
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn sort(
        base: *mut c_void,
        num: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
        swap: *mut c_void,
    );
    fn pr_err(fmt: *const i8, ...);
}

pub unsafe fn add_range(
    range: *mut range,
    az: c_int,
    mut nr_range: c_int,
    start: u64,
    end: u64,
) -> c_int {
    if start >= end {
        return nr_range;
    }

    /* Out of slots: */
    if nr_range >= az {
        return nr_range;
    }

    (*range.add(nr_range as usize)).start = start;
    (*range.add(nr_range as usize)).end = end;

    nr_range += 1;

    nr_range
}

pub unsafe fn add_range_with_merge(
    range: *mut range,
    az: c_int,
    mut nr_range: c_int,
    mut start: u64,
    mut end: u64,
) -> c_int {
    let mut i: c_int;

    if start >= end {
        return nr_range;
    }

    /* get new start/end: */
    i = 0;
    while i < nr_range {
        let (common_start, common_end);

        if (*range.add(i as usize)).end == 0 {
            i += 1;
            continue;
        }

        common_start = (*range.add(i as usize)).start.max(start);
        common_end = (*range.add(i as usize)).end.min(end);
        if common_start > common_end {
            i += 1;
            continue;
        }

        /* new start/end, will add it back at last */
        start = (*range.add(i as usize)).start.min(start);
        end = (*range.add(i as usize)).end.max(end);

        memmove(
            range.add(i as usize) as *mut c_void,
            range.add((i + 1) as usize) as *const c_void,
            ((nr_range - (i + 1)) as usize) * core::mem::size_of::<range>(),
        );
        (*range.add((nr_range - 1) as usize)).start = 0;
        (*range.add((nr_range - 1) as usize)).end = 0;
        nr_range -= 1;
        i -= 1;
        i += 1;
    }

    /* Need to add it: */
    add_range(range, az, nr_range, start, end)
}

pub unsafe fn subtract_range(range: *mut range, az: c_int, start: u64, end: u64) {
    let mut i: c_int;
    let mut j: c_int;

    if start >= end {
        return;
    }

    j = 0;
    while j < az {
        if (*range.add(j as usize)).end == 0 {
            j += 1;
            continue;
        }

        if start <= (*range.add(j as usize)).start && end >= (*range.add(j as usize)).end {
            (*range.add(j as usize)).start = 0;
            (*range.add(j as usize)).end = 0;
            j += 1;
            continue;
        }

        if start <= (*range.add(j as usize)).start
            && end < (*range.add(j as usize)).end
            && (*range.add(j as usize)).start < end
        {
            (*range.add(j as usize)).start = end;
            j += 1;
            continue;
        }

        if start > (*range.add(j as usize)).start
            && end >= (*range.add(j as usize)).end
            && (*range.add(j as usize)).end > start
        {
            (*range.add(j as usize)).end = start;
            j += 1;
            continue;
        }

        if start > (*range.add(j as usize)).start && end < (*range.add(j as usize)).end {
            /* Find the new spare: */
            i = 0;
            while i < az {
                if (*range.add(i as usize)).end == 0 {
                    break;
                }
                i += 1;
            }
            if i < az {
                (*range.add(i as usize)).end = (*range.add(j as usize)).end;
                (*range.add(i as usize)).start = end;
            } else {
                pr_err(b"%s: run out of slot in ranges\n\0".as_ptr() as *const i8);
            }
            (*range.add(j as usize)).end = start;
        }
        j += 1;
    }
}

unsafe extern "C" fn cmp_range(x1: *const c_void, x2: *const c_void) -> c_int {
    let r1 = &*(x1 as *const range);
    let r2 = &*(x2 as *const range);

    if r1.start < r2.start {
        return -1;
    }
    if r1.start > r2.start {
        return 1;
    }
    0
}

pub unsafe fn clean_sort_range(range: *mut range, az: c_int) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut k = az - 1;
    let mut nr_range = az;

    i = 0;
    while i < k {
        if (*range.add(i as usize)).end != 0 {
            i += 1;
            continue;
        }
        j = k;
        while j > i {
            if (*range.add(j as usize)).end != 0 {
                k = j;
                break;
            }
            j -= 1;
        }
        if j == i {
            break;
        }
        (*range.add(i as usize)).start = (*range.add(k as usize)).start;
        (*range.add(i as usize)).end = (*range.add(k as usize)).end;
        (*range.add(k as usize)).start = 0;
        (*range.add(k as usize)).end = 0;
        k -= 1;
        i += 1;
    }
    /* count it */
    i = 0;
    while i < az {
        if (*range.add(i as usize)).end == 0 {
            nr_range = i;
            break;
        }
        i += 1;
    }

    /* sort them */
    sort(
        range as *mut c_void,
        nr_range as usize,
        core::mem::size_of::<range>(),
        cmp_range,
        core::ptr::null_mut(),
    );

    nr_range
}

pub unsafe fn sort_range(range: *mut range, nr_range: c_int) {
    /* sort them */
    sort(
        range as *mut c_void,
        nr_range as usize,
        core::mem::size_of::<range>(),
        cmp_range,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
