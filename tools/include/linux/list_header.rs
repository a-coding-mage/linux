/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of include/linux/list.h.
 *
 * Original C dependencies:
 * - <linux/types.h>
 * - <linux/poison.h>
 * - <linux/kernel.h>
 * - <linux/compiler.h>
 */

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

unsafe extern "C" {
    pub static mut LIST_POISON1: *mut c_void;
    pub static mut LIST_POISON2: *mut c_void;
}

/* WRITE_ONCE from <linux/compiler.h>. */
#[inline(always)]
unsafe fn WRITE_ONCE<T>(dst: *mut T, val: T) {
    unsafe {
        ptr::write_volatile(dst, val);
    }
}

/*
 * Simple doubly linked list implementation.
 *
 * Some of the internal functions ("__xxx") are useful when
 * manipulating whole lists rather than single entries, as
 * sometimes we already know the next/prev entries and we can
 * generate better code by using them directly rather than
 * using the generic single-entry routines.
 */

#[macro_export]
macro_rules! LIST_HEAD_INIT {
    ($name:expr) => {
        $crate::list_head {
            next: core::ptr::addr_of_mut!($name),
            prev: core::ptr::addr_of_mut!($name),
        }
    };
}

#[macro_export]
macro_rules! LIST_HEAD {
    ($name:ident) => {
        let mut $name = $crate::LIST_HEAD_INIT!($name);
    };
}

#[inline]
pub unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

/*
 * Insert a new entry between two known consecutive entries.
 *
 * This is only for internal list manipulation where we know
 * the prev/next entries already!
 *
 * C conditional:
 * #ifndef CONFIG_DEBUG_LIST: inline implementation below.
 * #else: extern void __list_add(...)
 */
#[inline]
pub unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = new;
        (*new).next = next;
        (*new).prev = prev;
        (*prev).next = new;
    }
}

/**
 * list_add - add a new entry
 * @new: new entry to be added
 * @head: list head to add it after
 *
 * Insert a new entry after the specified head.
 * This is good for implementing stacks.
 */
#[inline]
pub unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, head, (*head).next);
    }
}

/**
 * list_add_tail - add a new entry
 * @new: new entry to be added
 * @head: list head to add it before
 *
 * Insert a new entry before the specified head.
 * This is useful for implementing queues.
 */
#[inline]
pub unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_add(new, (*head).prev, head);
    }
}

/*
 * Delete a list entry by making the prev/next entries
 * point to each other.
 *
 * This is only for internal list manipulation where we know
 * the prev/next entries already!
 */
#[inline]
pub unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    unsafe {
        (*next).prev = prev;
        WRITE_ONCE(ptr::addr_of_mut!((*prev).next), next);
    }
}

/**
 * list_del - deletes entry from list.
 * @entry: the element to delete from the list.
 * Note: list_empty() on entry does not return true after this, the entry is
 * in an undefined state.
 *
 * C conditional:
 * #ifndef CONFIG_DEBUG_LIST: inline implementations below.
 * #else: extern void __list_del_entry(...); extern void list_del(...);
 */
#[inline]
pub unsafe fn __list_del_entry(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
    }
}

#[inline]
pub unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
        (*entry).next = LIST_POISON1 as *mut list_head;
        (*entry).prev = LIST_POISON2 as *mut list_head;
    }
}

/**
 * list_replace - replace old entry by new one
 * @old : the element to be replaced
 * @new : the new element to insert
 *
 * If @old was empty, it will be overwritten.
 */
#[inline]
pub unsafe fn list_replace(old: *mut list_head, new: *mut list_head) {
    unsafe {
        (*new).next = (*old).next;
        (*(*new).next).prev = new;
        (*new).prev = (*old).prev;
        (*(*new).prev).next = new;
    }
}

#[inline]
pub unsafe fn list_replace_init(old: *mut list_head, new: *mut list_head) {
    unsafe {
        list_replace(old, new);
        INIT_LIST_HEAD(old);
    }
}

/**
 * list_del_init - deletes entry from list and reinitialize it.
 * @entry: the element to delete from the list.
 */
#[inline]
pub unsafe fn list_del_init(entry: *mut list_head) {
    unsafe {
        __list_del_entry(entry);
        INIT_LIST_HEAD(entry);
    }
}

/**
 * list_move - delete from one list and add as another's head
 * @list: the entry to move
 * @head: the head that will precede our entry
 */
#[inline]
pub unsafe fn list_move(list: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_del_entry(list);
        list_add(list, head);
    }
}

/**
 * list_move_tail - delete from one list and add as another's tail
 * @list: the entry to move
 * @head: the head that will follow our entry
 */
#[inline]
pub unsafe fn list_move_tail(list: *mut list_head, head: *mut list_head) {
    unsafe {
        __list_del_entry(list);
        list_add_tail(list, head);
    }
}

#[inline]
pub unsafe fn list_is_first(list: *const list_head, head: *const list_head) -> i32 {
    unsafe { ((*list).prev == head as *mut list_head) as i32 }
}

#[inline]
pub unsafe fn list_is_last(list: *const list_head, head: *const list_head) -> i32 {
    unsafe { ((*list).next == head as *mut list_head) as i32 }
}

#[inline]
pub unsafe fn list_empty(head: *const list_head) -> i32 {
    unsafe { ((*head).next == head as *mut list_head) as i32 }
}

#[inline]
pub unsafe fn list_empty_careful(head: *const list_head) -> i32 {
    unsafe {
        let next: *mut list_head = (*head).next;
        ((next == head as *mut list_head) && (next == (*head).prev)) as i32
    }
}

#[inline]
pub unsafe fn list_rotate_left(head: *mut list_head) {
    unsafe {
        let first: *mut list_head;

        if list_empty(head) == 0 {
            first = (*head).next;
            list_move_tail(first, head);
        }
    }
}

#[inline]
pub unsafe fn list_is_singular(head: *const list_head) -> i32 {
    unsafe { ((list_empty(head) == 0) && ((*head).next == (*head).prev)) as i32 }
}

#[inline]
pub unsafe fn __list_cut_position(
    list: *mut list_head,
    head: *mut list_head,
    entry: *mut list_head,
) {
    unsafe {
        let new_first: *mut list_head = (*entry).next;
        (*list).next = (*head).next;
        (*(*list).next).prev = list;
        (*list).prev = entry;
        (*entry).next = list;
        (*head).next = new_first;
        (*new_first).prev = head;
    }
}

#[inline]
pub unsafe fn list_cut_position(list: *mut list_head, head: *mut list_head, entry: *mut list_head) {
    unsafe {
        if list_empty(head) != 0 {
            return;
        }
        if list_is_singular(head) != 0 && ((*head).next != entry && head != entry) {
            return;
        }
        if entry == head {
            INIT_LIST_HEAD(list);
        } else {
            __list_cut_position(list, head, entry);
        }
    }
}

#[inline]
pub unsafe fn __list_splice(list: *const list_head, prev: *mut list_head, next: *mut list_head) {
    unsafe {
        let first: *mut list_head = (*list).next;
        let last: *mut list_head = (*list).prev;

        (*first).prev = prev;
        (*prev).next = first;

        (*last).next = next;
        (*next).prev = last;
    }
}

#[inline]
pub unsafe fn list_splice(list: *const list_head, head: *mut list_head) {
    unsafe {
        if list_empty(list) == 0 {
            __list_splice(list, head, (*head).next);
        }
    }
}

#[inline]
pub unsafe fn list_splice_tail(list: *mut list_head, head: *mut list_head) {
    unsafe {
        if list_empty(list) == 0 {
            __list_splice(list, (*head).prev, head);
        }
    }
}

#[inline]
pub unsafe fn list_splice_init(list: *mut list_head, head: *mut list_head) {
    unsafe {
        if list_empty(list) == 0 {
            __list_splice(list, head, (*head).next);
            INIT_LIST_HEAD(list);
        }
    }
}

#[inline]
pub unsafe fn list_splice_tail_init(list: *mut list_head, head: *mut list_head) {
    unsafe {
        if list_empty(list) == 0 {
            __list_splice(list, (*head).prev, head);
            INIT_LIST_HEAD(list);
        }
    }
}

/*
 * list_entry/list_first_entry/list_last_entry/list_next_entry/list_prev_entry
 * depend on the C container_of(), typeof(), and member designator macros.
 * The Rust forms below preserve the call surface by delegating to a required
 * external container_of! macro where a local source-level mapping is possible.
 */
#[macro_export]
macro_rules! list_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

#[macro_export]
macro_rules! list_first_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        list_entry!((*($ptr)).next, $type, $member)
    };
}

#[macro_export]
macro_rules! list_last_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        list_entry!((*($ptr)).prev, $type, $member)
    };
}

#[macro_export]
macro_rules! list_first_entry_or_null {
    ($ptr:expr, $type:ty, $member:ident) => {
        if list_empty($ptr) == 0 {
            list_first_entry!($ptr, $type, $member)
        } else {
            core::ptr::null_mut()
        }
    };
}

#[macro_export]
macro_rules! list_last_entry_or_null {
    ($ptr:expr, $type:ty, $member:ident) => {
        if list_empty($ptr) == 0 {
            list_last_entry!($ptr, $type, $member)
        } else {
            core::ptr::null_mut()
        }
    };
}

#[macro_export]
macro_rules! list_next_entry {
    ($pos:expr, $type:ty, $member:ident) => {
        list_entry!((*($pos)).$member.next, $type, $member)
    };
}

#[macro_export]
macro_rules! list_prev_entry {
    ($pos:expr, $type:ty, $member:ident) => {
        list_entry!((*($pos)).$member.prev, $type, $member)
    };
}

/* C for-loop iteration macros translated as callable macro skeletons. */
#[macro_export]
macro_rules! list_for_each {
    ($pos:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).next;
        while $pos != $head {
            $body
            $pos = (*$pos).next;
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_prev {
    ($pos:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).prev;
        while $pos != $head {
            $body
            $pos = (*$pos).prev;
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_safe {
    ($pos:ident, $n:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).next;
        $n = (*$pos).next;
        while $pos != $head {
            $body
            $pos = $n;
            $n = (*$pos).next;
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_prev_safe {
    ($pos:ident, $n:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).prev;
        $n = (*$pos).prev;
        while $pos != $head {
            $body
            $pos = $n;
            $n = (*$pos).prev;
        }
    }};
}

/* The list_for_each_entry* macros require C typeof(*pos); Rust callers pass $type. */
#[macro_export]
macro_rules! list_for_each_entry {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_first_entry!($head, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = list_next_entry!($pos, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_reverse {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_last_entry!($head, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = list_prev_entry!($pos, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_prepare_entry {
    ($pos:expr, $head:expr, $type:ty, $member:ident) => {
        if !($pos).is_null() {
            $pos
        } else {
            list_entry!($head, $type, $member)
        }
    };
}

#[macro_export]
macro_rules! list_for_each_entry_continue {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_next_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = list_next_entry!($pos, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_continue_reverse {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_prev_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = list_prev_entry!($pos, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_from {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = list_next_entry!($pos, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_safe {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_first_entry!($head, $type, $member);
        $n = list_next_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = $n;
            $n = list_next_entry!($n, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_safe_continue {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_next_entry!($pos, $type, $member);
        $n = list_next_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = $n;
            $n = list_next_entry!($n, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_safe_from {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $n = list_next_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = $n;
            $n = list_next_entry!($n, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_for_each_entry_safe_reverse {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = list_last_entry!($head, $type, $member);
        $n = list_prev_entry!($pos, $type, $member);
        while core::ptr::addr_of_mut!((*$pos).$member) != $head {
            $body
            $pos = $n;
            $n = list_prev_entry!($n, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! list_safe_reset_next {
    ($pos:expr, $n:ident, $type:ty, $member:ident) => {
        $n = list_next_entry!($pos, $type, $member);
    };
}

/*
 * Double linked lists with a single pointer list head.
 * Mostly useful for hash tables where the two pointer list head is
 * too wasteful.
 * You lose the ability to access the tail in O(1).
 */

#[macro_export]
macro_rules! HLIST_HEAD_INIT {
    () => {
        $crate::hlist_head {
            first: core::ptr::null_mut(),
        }
    };
}

#[macro_export]
macro_rules! HLIST_HEAD {
    ($name:ident) => {
        let mut $name = $crate::HLIST_HEAD_INIT!();
    };
}

#[macro_export]
macro_rules! INIT_HLIST_HEAD {
    ($ptr:expr) => {
        (*($ptr)).first = core::ptr::null_mut();
    };
}

#[inline]
pub unsafe fn INIT_HLIST_NODE(h: *mut hlist_node) {
    unsafe {
        (*h).next = ptr::null_mut();
        (*h).pprev = ptr::null_mut();
    }
}

#[inline]
pub unsafe fn hlist_unhashed(h: *const hlist_node) -> i32 {
    unsafe { ((*h).pprev.is_null()) as i32 }
}

#[inline]
pub unsafe fn hlist_empty(h: *const hlist_head) -> i32 {
    unsafe { ((*h).first.is_null()) as i32 }
}

#[inline]
pub unsafe fn __hlist_del(n: *mut hlist_node) {
    unsafe {
        let next: *mut hlist_node = (*n).next;
        let pprev: *mut *mut hlist_node = (*n).pprev;

        WRITE_ONCE(pprev, next);
        if !next.is_null() {
            (*next).pprev = pprev;
        }
    }
}

#[inline]
pub unsafe fn hlist_del(n: *mut hlist_node) {
    unsafe {
        __hlist_del(n);
        (*n).next = LIST_POISON1 as *mut hlist_node;
        (*n).pprev = LIST_POISON2 as *mut *mut hlist_node;
    }
}

#[inline]
pub unsafe fn hlist_del_init(n: *mut hlist_node) {
    unsafe {
        if hlist_unhashed(n) == 0 {
            __hlist_del(n);
            INIT_HLIST_NODE(n);
        }
    }
}

#[inline]
pub unsafe fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head) {
    unsafe {
        let first: *mut hlist_node = (*h).first;
        (*n).next = first;
        if !first.is_null() {
            (*first).pprev = ptr::addr_of_mut!((*n).next);
        }
        (*h).first = n;
        (*n).pprev = ptr::addr_of_mut!((*h).first);
    }
}

/* next must be != NULL */
#[inline]
pub unsafe fn hlist_add_before(n: *mut hlist_node, next: *mut hlist_node) {
    unsafe {
        (*n).pprev = (*next).pprev;
        (*n).next = next;
        (*next).pprev = ptr::addr_of_mut!((*n).next);
        *(*n).pprev = n;
    }
}

#[inline]
pub unsafe fn hlist_add_behind(n: *mut hlist_node, prev: *mut hlist_node) {
    unsafe {
        (*n).next = (*prev).next;
        (*prev).next = n;
        (*n).pprev = ptr::addr_of_mut!((*prev).next);

        if !(*n).next.is_null() {
            (*(*n).next).pprev = ptr::addr_of_mut!((*n).next);
        }
    }
}

/* after that we'll appear to be on some hlist and hlist_del will work */
#[inline]
pub unsafe fn hlist_add_fake(n: *mut hlist_node) {
    unsafe {
        (*n).pprev = ptr::addr_of_mut!((*n).next);
    }
}

#[inline]
pub unsafe fn hlist_fake(h: *mut hlist_node) -> bool {
    unsafe { (*h).pprev == ptr::addr_of_mut!((*h).next) }
}

/*
 * Move a list from one list head to another. Fixup the pprev
 * reference of the first entry if it exists.
 */
#[inline]
pub unsafe fn hlist_move_list(old: *mut hlist_head, new: *mut hlist_head) {
    unsafe {
        (*new).first = (*old).first;
        if !(*new).first.is_null() {
            (*(*new).first).pprev = ptr::addr_of_mut!((*new).first);
        }
        (*old).first = ptr::null_mut();
    }
}

#[macro_export]
macro_rules! hlist_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

#[macro_export]
macro_rules! hlist_for_each {
    ($pos:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).first;
        while !$pos.is_null() {
            $body
            $pos = (*$pos).next;
        }
    }};
}

#[macro_export]
macro_rules! hlist_for_each_safe {
    ($pos:ident, $n:ident, $head:expr, $body:block) => {{
        $pos = (*($head)).first;
        while !$pos.is_null() && {
            $n = (*$pos).next;
            true
        } {
            $body
            $pos = $n;
        }
    }};
}

#[macro_export]
macro_rules! hlist_entry_safe {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let ____ptr = $ptr;
        if !____ptr.is_null() {
            hlist_entry!(____ptr, $type, $member)
        } else {
            core::ptr::null_mut()
        }
    }};
}

#[macro_export]
macro_rules! hlist_for_each_entry {
    ($pos:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = hlist_entry_safe!((*($head)).first, $type, $member);
        while !$pos.is_null() {
            $body
            $pos = hlist_entry_safe!((*$pos).$member.next, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! hlist_for_each_entry_continue {
    ($pos:ident, $type:ty, $member:ident, $body:block) => {{
        $pos = hlist_entry_safe!((*$pos).$member.next, $type, $member);
        while !$pos.is_null() {
            $body
            $pos = hlist_entry_safe!((*$pos).$member.next, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! hlist_for_each_entry_from {
    ($pos:ident, $type:ty, $member:ident, $body:block) => {{
        while !$pos.is_null() {
            $body
            $pos = hlist_entry_safe!((*$pos).$member.next, $type, $member);
        }
    }};
}

#[macro_export]
macro_rules! hlist_for_each_entry_safe {
    ($pos:ident, $n:ident, $head:expr, $type:ty, $member:ident, $body:block) => {{
        $pos = hlist_entry_safe!((*($head)).first, $type, $member);
        while !$pos.is_null() && {
            $n = (*$pos).$member.next;
            true
        } {
            $body
            $pos = hlist_entry_safe!($n, $type, $member);
        }
    }};
}

/**
 * list_del_range - deletes range of entries from list.
 * @begin: first element in the range to delete from the list.
 * @end: last element in the range to delete from the list.
 * Note: list_empty on the range of entries does not return true after this,
 * the entries is in an undefined state.
 */
#[inline]
pub unsafe fn list_del_range(begin: *mut list_head, end: *mut list_head) {
    unsafe {
        (*(*begin).prev).next = (*end).next;
        (*(*end).next).prev = (*begin).prev;
    }
}

#[macro_export]
macro_rules! list_for_each_from {
    ($pos:ident, $head:expr, $body:block) => {{
        while $pos != $head {
            $body
            $pos = (*$pos).next;
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
