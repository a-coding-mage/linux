// SPDX-License-Identifier: GPL-2.0
// Dependency declarations supplied by the surrounding Linux list implementation.
use crate::{list_cmp_func_t, list_head};

/*
 * Returns a list organized in an intermediate format suited
 * to chaining of merge() calls: null-terminated, no reserved or
 * sentinel head node, "prev" links not maintained.
 */
unsafe fn merge(
    priv_: *mut core::ffi::c_void,
    cmp: list_cmp_func_t,
    mut a: *mut list_head,
    mut b: *mut list_head,
) -> *mut list_head {
    let mut head: *mut list_head = core::ptr::null_mut();
    let mut tail: *mut *mut list_head = &mut head;

    loop {
        /* if equal, take 'a' -- important for sort stability */
        if cmp(priv_, a, b) <= 0 {
            *tail = a;
            tail = &mut (*a).next;
            a = (*a).next;
            if a.is_null() {
                *tail = b;
                break;
            }
        } else {
            *tail = b;
            tail = &mut (*b).next;
            b = (*b).next;
            if b.is_null() {
                *tail = a;
                break;
            }
        }
    }
    head
}

/*
 * Combine final list merge with restoration of standard doubly-linked
 * list structure.  This approach duplicates code from merge(), but
 * runs faster than the tidier alternatives of either a separate final
 * prev-link restoration pass, or maintaining the prev links
 * throughout.
 */
unsafe fn merge_final(
    priv_: *mut core::ffi::c_void,
    cmp: list_cmp_func_t,
    head: *mut list_head,
    mut a: *mut list_head,
    mut b: *mut list_head,
) {
    let mut tail = head;

    loop {
        /* if equal, take 'a' -- important for sort stability */
        if cmp(priv_, a, b) <= 0 {
            (*tail).next = a;
            (*a).prev = tail;
            tail = a;
            a = (*a).next;
            if a.is_null() {
                break;
            }
        } else {
            (*tail).next = b;
            (*b).prev = tail;
            tail = b;
            b = (*b).next;
            if b.is_null() {
                b = a;
                break;
            }
        }
    }

    /* Finish linking remainder of list b on to tail */
    (*tail).next = b;
    loop {
        (*b).prev = tail;
        tail = b;
        b = (*b).next;
        if b.is_null() {
            break;
        }
    }

    /* And the final links to make a circular doubly-linked list */
    (*tail).next = head;
    (*head).prev = tail;
}

/**
 * list_sort - sort a list
 * @priv: private data, opaque to list_sort(), passed to @cmp
 * @head: the list to sort
 * @cmp: the elements comparison function
 *
 * The comparison function @cmp must return > 0 if @a should sort after
 * @b ("@a > @b" if you want an ascending sort), and <= 0 if @a should
 * sort before @b *or* their original order should be preserved. It is
 * always called with the element that came first in the input in @a,
 * and list_sort is a stable sort.
 */
pub unsafe fn list_sort(
    priv_: *mut core::ffi::c_void,
    head: *mut list_head,
    cmp: list_cmp_func_t,
) {
    let mut list = (*head).next;
    let mut pending: *mut list_head = core::ptr::null_mut();
    let mut count: usize = 0;

    if list == (*head).prev {
        return;
    }

    (*(*head).prev).next = core::ptr::null_mut();

    loop {
        let mut bits = count;
        let mut tail: *mut *mut list_head = &mut pending;

        /* Find the least-significant clear bit in count */
        while bits & 1 != 0 {
            tail = &mut (*(*tail)).prev;
            bits >>= 1;
        }

        /* Do the indicated merge */
        if bits != 0 {
            let a = *tail;
            let b = (*a).prev;
            let merged = merge(priv_, cmp, b, a);
            (*merged).prev = (*b).prev;
            *tail = merged;
        }

        /* Move one element from input list to pending */
        (*list).prev = pending;
        pending = list;
        list = (*list).next;
        (*pending).next = core::ptr::null_mut();
        count = count.wrapping_add(1);

        if list.is_null() {
            break;
        }
    }

    /* End of input; merge together all the pending lists. */
    list = pending;
    pending = (*pending).prev;
    loop {
        let next = (*pending).prev;
        if next.is_null() {
            break;
        }
        list = merge(priv_, cmp, pending, list);
        pending = next;
    }
    /* The final merge, rebuilding prev links */
    merge_final(priv_, cmp, head, pending, list);
}

// EXPORT_SYMBOL(list_sort);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
