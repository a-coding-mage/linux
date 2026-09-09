// SPDX-License-Identifier: GPL-2.0
/*
 * helpers to map values in a linear range to range index
 *
 * Original idea borrowed from regulator framework
 *
 * It might be useful if we could support also inversely proportional ranges?
 * Copyright 2020 ROHM Semiconductors
 */

// The `linear_range` type is supplied by the corresponding external dependency.
use crate::linear_range;

const EINVAL: i32 = 22;

/**
 * linear_range_values_in_range - return the amount of values in a range
 * @r:          pointer to linear range where values are counted
 *
 * Compute the amount of values in range pointed by @r. Note, values can
 * be all equal - range with selectors 0,...,2 with step 0 still contains
 * 3 values even though they are all equal.
 *
 * Return: the amount of values in range pointed by @r
 */
pub unsafe fn linear_range_values_in_range(r: *const linear_range) -> u32 {
    if r.is_null() {
        return 0;
    }
    (*r).max_sel.wrapping_sub((*r).min_sel).wrapping_add(1)
}

/**
 * linear_range_values_in_range_array - return the amount of values in ranges
 * @r:          pointer to array of linear ranges where values are counted
 * @ranges:     amount of ranges we include in computation.
 */
pub unsafe fn linear_range_values_in_range_array(r: *const linear_range, ranges: i32) -> u32 {
    let mut values_in_range: i32 = 0;

    for i in 0..ranges {
        let values = linear_range_values_in_range(r.add(i as usize));
        if values == 0 {
            return values;
        }
        values_in_range = values_in_range.wrapping_add(values as i32);
    }
    values_in_range as u32
}

pub unsafe fn linear_range_get_max_value(r: *const linear_range) -> u32 {
    (*r).min.wrapping_add(
        (*r).max_sel.wrapping_sub((*r).min_sel).wrapping_mul((*r).step),
    )
}

pub unsafe fn linear_range_get_value(
    r: *const linear_range,
    selector: u32,
    val: *mut u32,
) -> i32 {
    if (*r).min_sel > selector || (*r).max_sel < selector {
        return -EINVAL;
    }
    *val = (*r).min.wrapping_add(
        selector.wrapping_sub((*r).min_sel).wrapping_mul((*r).step),
    );
    0
}

pub unsafe fn linear_range_get_value_array(
    r: *const linear_range,
    ranges: i32,
    selector: u32,
    val: *mut u32,
) -> i32 {
    for i in 0..ranges {
        let range = r.add(i as usize);
        if (*range).min_sel <= selector && (*range).max_sel >= selector {
            return linear_range_get_value(range, selector, val);
        }
    }
    -EINVAL
}

pub unsafe fn linear_range_get_selector_low(
    r: *const linear_range,
    val: u32,
    selector: *mut u32,
    found: *mut bool,
) -> i32 {
    *found = false;
    if (*r).min > val {
        return -EINVAL;
    }
    if linear_range_get_max_value(r) < val {
        *selector = (*r).max_sel;
        return 0;
    }
    *found = true;
    if (*r).step == 0 {
        *selector = (*r).min_sel;
    } else {
        *selector = val.wrapping_sub((*r).min) / (*r).step + (*r).min_sel;
    }
    0
}

pub unsafe fn linear_range_get_selector_low_array(
    r: *const linear_range,
    ranges: i32,
    val: u32,
    selector: *mut u32,
    found: *mut bool,
) -> i32 {
    let mut ret = -EINVAL;
    for i in 0..ranges {
        let tmpret = linear_range_get_selector_low(r.add(i as usize), val, selector, found);
        if tmpret == 0 {
            ret = 0;
        }
        if *found {
            break;
        }
    }
    ret
}

pub unsafe fn linear_range_get_selector_high(
    r: *const linear_range,
    val: u32,
    selector: *mut u32,
    found: *mut bool,
) -> i32 {
    *found = false;
    if linear_range_get_max_value(r) < val {
        return -EINVAL;
    }
    if (*r).min > val {
        *selector = (*r).min_sel;
        return 0;
    }
    *found = true;
    if (*r).step == 0 {
        *selector = (*r).max_sel;
    } else {
        *selector = val
            .wrapping_sub((*r).min)
            .wrapping_add((*r).step)
            .wrapping_sub(1)
            / (*r).step
            + (*r).min_sel;
    }
    0
}

pub unsafe fn linear_range_get_selector_high_array(
    r: *const linear_range,
    ranges: i32,
    val: u32,
    selector: *mut u32,
    found: *mut bool,
) -> i32 {
    for i in 0..ranges {
        let ret = linear_range_get_selector_high(r.add(i as usize), val, selector, found);
        if ret == 0 {
            return 0;
        }
    }
    -EINVAL
}

pub unsafe fn linear_range_get_selector_within(
    r: *const linear_range,
    val: u32,
    selector: *mut u32,
) {
    if (*r).min > val {
        *selector = (*r).min_sel;
        return;
    }
    if linear_range_get_max_value(r) < val {
        *selector = (*r).max_sel;
        return;
    }
    if (*r).step == 0 {
        *selector = (*r).min_sel;
    } else {
        *selector = val.wrapping_sub((*r).min) / (*r).step + (*r).min_sel;
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
