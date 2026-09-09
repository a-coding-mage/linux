/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux translation are intentionally
// left external: container_of, READ_ONCE, WRITE_ONCE, and LIST_POISON2.

#[repr(C)]
pub struct hlist_nulls_head {
    pub first: *mut hlist_nulls_node,
}

#[repr(C)]
pub struct hlist_nulls_node {
    pub next: *mut hlist_nulls_node,
    pub pprev: *mut *mut hlist_nulls_node,
}

#[inline]
pub const unsafe fn NULLS_MARKER(value: isize) -> usize {
    1usize | ((value as usize) << 1)
}

#[inline]
pub unsafe fn INIT_HLIST_NULLS_HEAD(ptr: *mut hlist_nulls_head, nulls: isize) {
    (*ptr).first = NULLS_MARKER(nulls) as *mut hlist_nulls_node;
}

#[inline]
pub const unsafe fn HLIST_NULLS_HEAD_INIT(nulls: isize) -> hlist_nulls_head {
    hlist_nulls_head {
        first: NULLS_MARKER(nulls) as *mut hlist_nulls_node,
    }
}

#[macro_export]
macro_rules! hlist_nulls_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

#[macro_export]
macro_rules! hlist_nulls_entry_safe {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let ____ptr = $ptr;
        if !is_a_nulls(____ptr) {
            hlist_nulls_entry!(____ptr, $type, $member)
        } else {
            core::ptr::null_mut()
        }
    }};
}

#[inline]
pub unsafe fn is_a_nulls(ptr: *const hlist_nulls_node) -> i32 {
    ((ptr as usize) & 1) as i32
}

#[inline]
pub unsafe fn get_nulls_value(ptr: *const hlist_nulls_node) -> usize {
    (ptr as usize) >> 1
}

#[inline]
pub unsafe fn hlist_nulls_unhashed(h: *const hlist_nulls_node) -> i32 {
    ((*h).pprev.is_null()) as i32
}

#[inline]
pub unsafe fn hlist_nulls_unhashed_lockless(h: *const hlist_nulls_node) -> i32 {
    (READ_ONCE!((*h).pprev).is_null()) as i32
}

#[inline]
pub unsafe fn hlist_nulls_empty(h: *const hlist_nulls_head) -> i32 {
    is_a_nulls(READ_ONCE!((*h).first))
}

#[inline]
pub unsafe fn hlist_nulls_add_head(n: *mut hlist_nulls_node, h: *mut hlist_nulls_head) {
    let first = (*h).first;
    (*n).next = first;
    WRITE_ONCE!((*n).pprev, &mut (*h).first as *mut *mut hlist_nulls_node);
    (*h).first = n;
    if is_a_nulls(first) == 0 {
        WRITE_ONCE!((*first).pprev, &mut (*n).next as *mut *mut hlist_nulls_node);
    }
}

#[inline]
pub unsafe fn __hlist_nulls_del(n: *mut hlist_nulls_node) {
    let next = (*n).next;
    let pprev = (*n).pprev;
    WRITE_ONCE!(*pprev, next);
    if is_a_nulls(next) == 0 {
        WRITE_ONCE!((*next).pprev, pprev);
    }
}

#[inline]
pub unsafe fn hlist_nulls_del(n: *mut hlist_nulls_node) {
    __hlist_nulls_del(n);
    WRITE_ONCE!((*n).pprev, LIST_POISON2);
}

#[macro_export]
macro_rules! hlist_nulls_for_each_entry {
    ($tpos:ident, $pos:ident, $head:expr, $member:ident) => {
        for $pos in unsafe { (*$head).first } {
            if unsafe { is_a_nulls($pos) } != 0 {
                break;
            }
            let $tpos = unsafe { hlist_nulls_entry!($pos, _, $member) };
            let _ = &$tpos;
            $pos = unsafe { (*$pos).next };
        }
    };
}

#[macro_export]
macro_rules! hlist_nulls_for_each_entry_from {
    ($tpos:ident, $pos:ident, $member:ident) => {
        while unsafe { is_a_nulls($pos) } == 0 {
            let $tpos = unsafe { hlist_nulls_entry!($pos, _, $member) };
            let _ = &$tpos;
            $pos = unsafe { (*$pos).next };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
