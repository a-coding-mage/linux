/*
 * Copyright 2006, Red Hat, Inc., Dave Jones
 * Released under the General Public License (GPL).
 *
 * This file contains the linked list validation and error reporting for
 * LIST_HARDENED and DEBUG_LIST.
 */

use core::ffi::c_char;

// Supplied by the surrounding kernel translation/dependencies.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

unsafe extern "C" {
    fn CHECK_DATA_CORRUPTION(
        condition: bool,
        ptr: *mut list_head,
        message: *const c_char,
        ...,
    ) -> bool;
    static LIST_POISON1: *mut list_head;
    static LIST_POISON2: *mut list_head;
}

/*
 * Check that the data structures for the list manipulations are reasonably
 * valid. Failures here indicate memory corruption (and possibly an exploit
 * attempt).
 */

// __list_valid_slowpath
pub unsafe extern "C" fn __list_add_valid_or_report(
    new: *mut list_head,
    prev: *mut list_head,
    next: *mut list_head,
) -> bool {
    if CHECK_DATA_CORRUPTION(
        prev.is_null(),
        core::ptr::null_mut(),
        c"list_add corruption. prev is NULL.\n".as_ptr(),
    ) || CHECK_DATA_CORRUPTION(
        next.is_null(),
        core::ptr::null_mut(),
        c"list_add corruption. next is NULL.\n".as_ptr(),
    ) || CHECK_DATA_CORRUPTION(
        (*next).prev != prev,
        next,
        c"list_add corruption. next->prev should be prev (%px), but was %px. (next=%px).\n".as_ptr(),
        prev,
        (*next).prev,
        next,
    ) || CHECK_DATA_CORRUPTION(
        (*prev).next != next,
        prev,
        c"list_add corruption. prev->next should be next (%px), but was %px. (prev=%px).\n".as_ptr(),
        next,
        (*prev).next,
        prev,
    ) || CHECK_DATA_CORRUPTION(
        new == prev || new == next,
        core::ptr::null_mut(),
        c"list_add double add: new=%px, prev=%px, next=%px.\n".as_ptr(),
        new,
        prev,
        next,
    ) {
        return false;
    }

    true
}

// EXPORT_SYMBOL(__list_add_valid_or_report);

// __list_valid_slowpath
pub unsafe extern "C" fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool {
    let prev: *mut list_head = (*entry).prev;
    let next: *mut list_head = (*entry).next;

    if CHECK_DATA_CORRUPTION(
        next.is_null(),
        core::ptr::null_mut(),
        c"list_del corruption, %px->next is NULL\n".as_ptr(),
        entry,
    ) || CHECK_DATA_CORRUPTION(
        prev.is_null(),
        core::ptr::null_mut(),
        c"list_del corruption, %px->prev is NULL\n".as_ptr(),
        entry,
    ) || CHECK_DATA_CORRUPTION(
        next == LIST_POISON1,
        next,
        c"list_del corruption, %px->next is LIST_POISON1 (%px)\n".as_ptr(),
        entry,
        LIST_POISON1,
    ) || CHECK_DATA_CORRUPTION(
        prev == LIST_POISON2,
        prev,
        c"list_del corruption, %px->prev is LIST_POISON2 (%px)\n".as_ptr(),
        entry,
        LIST_POISON2,
    ) || CHECK_DATA_CORRUPTION(
        (*prev).next != entry,
        prev,
        c"list_del corruption. prev->next should be %px, but was %px. (prev=%px)\n".as_ptr(),
        entry,
        (*prev).next,
        prev,
    ) || CHECK_DATA_CORRUPTION(
        (*next).prev != entry,
        next,
        c"list_del corruption. next->prev should be %px, but was %px. (next=%px)\n".as_ptr(),
        entry,
        (*next).prev,
        next,
    ) {
        return false;
    }

    true
}

// EXPORT_SYMBOL(__list_del_entry_valid_or_report);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
