/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations corresponding to <stddef.h> and "list_types.h" are
// supplied by the surrounding translation unit.

#[allow(unused_macros)]
macro_rules! __same_type {
    ($a:expr, $b:expr) => { false };
}

#[allow(unused_macros)]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr = ($ptr as *mut u8);
        unsafe { __mptr.sub(core::mem::offset_of!($type, $member)) as *mut $type }
    }};
}

pub const LIST_POISON1: *mut core::ffi::c_void = 0x100usize as *mut core::ffi::c_void;
pub const LIST_POISON2: *mut core::ffi::c_void = 0x122usize as *mut core::ffi::c_void;

/*
 * Circular doubly linked list implementation.
 *
 * Some of the internal functions ("__xxx") are useful when
 * manipulating whole lists rather than single entries, as
 * sometimes we already know the next/prev entries and we can
 * generate better code by using them directly rather than
 * using the generic single-entry routines.
 */

#[allow(unused_macros)]
macro_rules! LIST_HEAD_INIT { ($name:expr) => { $crate::list_head { next: &mut $name, prev: &mut $name } }; }
#[allow(unused_macros)]
macro_rules! LIST_HEAD { ($name:ident) => { let mut $name = $crate::list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }; $name.next = &mut $name; $name.prev = &mut $name; }; }

pub unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

pub unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

pub unsafe fn list_add(new: *mut list_head, head: *mut list_head) { __list_add(new, head, (*head).next); }
pub unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) { __list_add(new, (*head).prev, head); }
pub unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) { (*next).prev = prev; (*prev).next = next; }
pub unsafe fn __list_del_entry(entry: *mut list_head) { __list_del((*entry).prev, (*entry).next); }

pub unsafe fn list_del(entry: *mut list_head) {
    __list_del_entry(entry);
    (*entry).next = LIST_POISON1 as *mut list_head;
    (*entry).prev = LIST_POISON2 as *mut list_head;
}

pub unsafe fn list_replace(old: *mut list_head, new: *mut list_head) {
    (*new).next = (*old).next;
    (*new).next.as_mut().unwrap().prev = new;
    (*new).prev = (*old).prev;
    (*new).prev.as_mut().unwrap().next = new;
}
pub unsafe fn list_replace_init(old: *mut list_head, new: *mut list_head) { list_replace(old, new); INIT_LIST_HEAD(old); }
pub unsafe fn list_move(list: *mut list_head, head: *mut list_head) { __list_del_entry(list); list_add(list, head); }
pub unsafe fn list_move_tail(list: *mut list_head, head: *mut list_head) { __list_del_entry(list); list_add_tail(list, head); }
pub unsafe fn list_is_first(list: *const list_head, head: *const list_head) -> i32 { ((*list).prev == head as *mut list_head) as i32 }
pub unsafe fn list_is_last(list: *const list_head, head: *const list_head) -> i32 { ((*list).next == head as *mut list_head) as i32 }
pub unsafe fn list_is_head(list: *const list_head, head: *const list_head) -> i32 { (list == head) as i32 }
pub unsafe fn list_empty(head: *const list_head) -> i32 { ((*head).next == head as *mut list_head) as i32 }

#[allow(unused_macros)]
macro_rules! list_entry { ($ptr:expr, $type:ty, $member:tt) => { container_of!($ptr, $type, $member) }; }
#[allow(unused_macros)]
macro_rules! list_first_entry { ($ptr:expr, $type:ty, $member:tt) => { list_entry!((*$ptr).next, $type, $member) }; }
#[allow(unused_macros)]
macro_rules! list_last_entry { ($ptr:expr, $type:ty, $member:tt) => { list_entry!((*$ptr).prev, $type, $member) }; }
#[allow(unused_macros)]
macro_rules! list_next_entry { ($pos:expr, $member:tt) => { list_entry!((*$pos).$member.next, _, $member) }; }
#[allow(unused_macros)]
macro_rules! list_prev_entry { ($pos:expr, $member:tt) => { list_entry!((*$pos).$member.prev, _, $member) }; }
#[allow(unused_macros)]
macro_rules! list_entry_is_head { ($pos:expr, $head:expr, $member:tt) => { (&(*$pos).$member as *const _) == ($head as *const _) }; }
#[allow(unused_macros)]
macro_rules! list_for_each_entry { ($pos:ident, $head:expr, $member:tt) => { for _ in 0.. { if list_entry_is_head!($pos, $head, $member) { break; } } }; }
#[allow(unused_macros)]
macro_rules! list_for_each_entry_reverse { ($pos:ident, $head:expr, $member:tt) => { for _ in 0.. { if list_entry_is_head!($pos, $head, $member) { break; } } }; }
#[allow(unused_macros)]
macro_rules! list_for_each_entry_safe { ($pos:ident, $n:ident, $head:expr, $member:tt) => { for _ in 0.. { if list_entry_is_head!($pos, $head, $member) { break; } } }; }

/* Double linked lists with a single pointer list head. */
#[allow(unused_macros)]
macro_rules! HLIST_HEAD_INIT { () => { hlist_head { first: core::ptr::null_mut() } }; }
pub unsafe fn INIT_HLIST_HEAD(ptr: *mut hlist_head) { (*ptr).first = core::ptr::null_mut(); }
pub unsafe fn INIT_HLIST_NODE(h: *mut hlist_node) { (*h).next = core::ptr::null_mut(); (*h).pprev = core::ptr::null_mut(); }
pub unsafe fn hlist_unhashed(h: *const hlist_node) -> i32 { ((*h).pprev.is_null()) as i32 }
pub unsafe fn __hlist_del(n: *mut hlist_node) { let next = (*n).next; let pprev = (*n).pprev; *pprev = next; if !next.is_null() { (*next).pprev = pprev; } }
pub unsafe fn hlist_del(n: *mut hlist_node) { __hlist_del(n); (*n).next = LIST_POISON1 as *mut hlist_node; (*n).pprev = LIST_POISON2 as *mut *mut hlist_node; }
pub unsafe fn hlist_del_init(n: *mut hlist_node) { if hlist_unhashed(n) == 0 { __hlist_del(n); INIT_HLIST_NODE(n); } }
pub unsafe fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head) { let first = (*h).first; (*n).next = first; if !first.is_null() { (*first).pprev = &mut (*n).next; } (*h).first = n; (*n).pprev = &mut (*h).first; }

#[allow(unused_macros)]
macro_rules! hlist_entry { ($ptr:expr, $type:ty, $member:tt) => { container_of!($ptr, $type, $member) }; }
#[allow(unused_macros)]
macro_rules! hlist_entry_safe { ($ptr:expr, $type:ty, $member:tt) => { if !($ptr).is_null() { hlist_entry!($ptr, $type, $member) } else { core::ptr::null_mut() } }; }
#[allow(unused_macros)]
macro_rules! hlist_for_each_entry { ($pos:ident, $head:expr, $member:tt) => { while !$pos.is_null() { $pos = hlist_entry_safe!((*$pos).$member.next, _, $member); } }; }
#[allow(unused_macros)]
macro_rules! hlist_for_each_entry_safe { ($pos:ident, $n:ident, $head:expr, $member:tt) => { while !$pos.is_null() { $n = (*$pos).$member.next; $pos = hlist_entry_safe!($n, _, $member); } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
