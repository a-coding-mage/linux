// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 - Google LLC
 * Author: Keir Fraser <keirf@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/list.h and linux/bug.h.

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[inline]
fn nvhe_check_data_corruption(v: bool) -> bool {
    v
}

// The CONFIG_BUG_ON_DATA_CORRUPTION condition is a build-time kernel
// configuration condition and is preserved here as an external dependency.
macro_rules! NVHE_CHECK_DATA_CORRUPTION {
    ($condition:expr) => {{
        let corruption = unlikely($condition);
        if corruption {
            if IS_ENABLED_CONFIG_BUG_ON_DATA_CORRUPTION {
                BUG();
            } else {
                WARN_ON(1);
            }
        }
        nvhe_check_data_corruption(corruption)
    }};
}

// The predicates checked here are taken from lib/list_debug.c.

#[no_mangle]
pub unsafe extern "C" fn __list_add_valid_or_report(
    new: *mut list_head,
    prev: *mut list_head,
    next: *mut list_head,
) -> bool {
    if NVHE_CHECK_DATA_CORRUPTION((*next).prev != prev)
        || NVHE_CHECK_DATA_CORRUPTION((*prev).next != next)
        || NVHE_CHECK_DATA_CORRUPTION(new == prev || new == next)
    {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool {
    let prev: *mut list_head;
    let next: *mut list_head;

    prev = (*entry).prev;
    next = (*entry).next;

    if NVHE_CHECK_DATA_CORRUPTION(next == LIST_POISON1)
        || NVHE_CHECK_DATA_CORRUPTION(prev == LIST_POISON2)
        || NVHE_CHECK_DATA_CORRUPTION((*prev).next != entry)
        || NVHE_CHECK_DATA_CORRUPTION((*next).prev != entry)
    {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
