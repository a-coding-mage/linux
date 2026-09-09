/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/list.h.  Types and low-level primitives are
// supplied by the surrounding kernel translation.

extern "C" {
    pub fn __list_add_valid_or_report(new: *mut list_head, prev: *mut list_head, next: *mut list_head) -> bool;
    pub fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool;
}

#[allow(non_camel_case_types)]
pub type size_t = usize;

#[inline(always)]
pub unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list; (*list).prev = list;
}

#[inline(always)]
pub unsafe fn __list_add_valid(new: *mut list_head, prev: *mut list_head, next: *mut list_head) -> bool {
    // CONFIG_LIST_HARDENED / CONFIG_DEBUG_LIST are build-time conditions.
    if (*next).prev == prev && (*prev).next == next && new != prev && new != next { true }
    else { __list_add_valid_or_report(new, prev, next) }
}

#[inline(always)]
pub unsafe fn __list_del_entry_valid(entry: *mut list_head) -> bool {
    if (*entry).prev.next == entry && (*entry).next.prev == entry { true }
    else { __list_del_entry_valid_or_report(entry) }
}

#[inline(always)] pub unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    if !__list_add_valid(new, prev, next) { return; }
    (*next).prev = new; (*new).next = next; (*new).prev = prev; (*prev).next = new;
}
#[inline(always)] pub unsafe fn list_add(new: *mut list_head, head: *mut list_head) { __list_add(new, head, (*head).next); }
#[inline(always)] pub unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) { __list_add(new, (*head).prev, head); }
#[inline(always)] pub unsafe fn list_add_tail_release(new: *mut list_head, head: *mut list_head) { __list_add(new, (*head).prev, head); }
#[inline(always)] pub unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) { (*next).prev = prev; (*prev).next = next; }
#[inline(always)] pub unsafe fn __list_del_clearprev(entry: *mut list_head) { __list_del((*entry).prev, (*entry).next); (*entry).prev = core::ptr::null_mut(); }
#[inline(always)] pub unsafe fn __list_del_entry(entry: *mut list_head) { if __list_del_entry_valid(entry) { __list_del((*entry).prev, (*entry).next); } }
#[inline(always)] pub unsafe fn list_del(entry: *mut list_head) { __list_del_entry(entry); (*entry).next = LIST_POISON1; (*entry).prev = LIST_POISON2; }
#[inline(always)] pub unsafe fn list_replace(old: *mut list_head, new: *mut list_head) { (*new).next=(*old).next; (*(*new).next).prev=new; (*new).prev=(*old).prev; (*(*new).prev).next=new; }
#[inline(always)] pub unsafe fn list_replace_init(old: *mut list_head, new: *mut list_head) { list_replace(old,new); INIT_LIST_HEAD(old); }
#[inline(always)] pub unsafe fn list_del_init(entry: *mut list_head) { __list_del_entry(entry); INIT_LIST_HEAD(entry); }
#[inline(always)] pub unsafe fn list_move(list: *mut list_head, head: *mut list_head) { __list_del_entry(list); list_add(list,head); }
#[inline(always)] pub unsafe fn list_move_tail(list: *mut list_head, head: *mut list_head) { __list_del_entry(list); list_add_tail(list,head); }
#[inline(always)] pub unsafe fn list_is_first(list:*const list_head,head:*const list_head)->bool { (*list).prev==head }
#[inline(always)] pub unsafe fn list_is_last(list:*const list_head,head:*const list_head)->bool { (*list).next==head }
#[inline(always)] pub unsafe fn list_is_head(list:*const list_head,head:*const list_head)->bool { list==head }
#[inline(always)] pub unsafe fn list_empty(head:*const list_head)->bool { (*head).next==head as *mut _ }
#[inline(always)] pub unsafe fn list_is_singular(head:*const list_head)->bool { !list_empty(head) && (*head).next==(*head).prev }
#[inline(always)] pub unsafe fn list_rotate_left(head:*mut list_head) { if !list_empty(head) { list_move_tail((*head).next,head); } }
#[inline(always)] pub unsafe fn list_count_nodes(head:*mut list_head)->size_t { let mut n=0; let mut p=(*head).next; while p!=head { n+=1; p=(*p).next; } n }

// The following macros retain the source interface; `container_of!` is supplied
// by the translated kernel support code.
#[macro_export] macro_rules! LIST_HEAD_INIT { ($name:expr) => { [$name as *mut _, $name as *mut _] }; }
#[macro_export] macro_rules! LIST_HEAD { ($name:ident) => { let mut $name: list_head = unsafe { core::mem::zeroed() }; unsafe { $crate::INIT_LIST_HEAD(&mut $name); } }; }
#[macro_export] macro_rules! list_entry { ($ptr:expr,$ty:ty,$member:ident) => { container_of!($ptr,$ty,$member) }; }
#[macro_export] macro_rules! list_first_entry { ($ptr:expr,$ty:ty,$member:ident) => { list_entry!(unsafe{(*$ptr).next},$ty,$member) }; }
#[macro_export] macro_rules! list_last_entry { ($ptr:expr,$ty:ty,$member:ident) => { list_entry!(unsafe{(*$ptr).prev},$ty,$member) }; }
#[macro_export] macro_rules! list_for_each { ($pos:ident,$head:expr,$body:block) => {{ let mut $pos=unsafe{(*$head).next}; while !unsafe{list_is_head($pos,$head)} { $body; $pos=unsafe{(*$pos).next}; } }}; }

// Hlist operations.
#[inline(always)] pub unsafe fn INIT_HLIST_HEAD(ptr:*mut hlist_head) { (*ptr).first=core::ptr::null_mut(); }
#[inline(always)] pub unsafe fn INIT_HLIST_NODE(h:*mut hlist_node) { (*h).next=core::ptr::null_mut(); (*h).pprev=core::ptr::null_mut(); }
#[inline(always)] pub unsafe fn hlist_unhashed(h:*const hlist_node)->bool { (*h).pprev.is_null() }
#[inline(always)] pub unsafe fn hlist_empty(h:*const hlist_head)->bool { (*h).first.is_null() }
#[inline(always)] pub unsafe fn __hlist_del(n:*mut hlist_node) { let next=(*n).next; let pprev=(*n).pprev; *pprev=next; if !next.is_null(){(*next).pprev=pprev;} }
#[inline(always)] pub unsafe fn hlist_del(n:*mut hlist_node) { __hlist_del(n); (*n).next=LIST_POISON1; (*n).pprev=LIST_POISON2 as *mut _; }
#[inline(always)] pub unsafe fn hlist_del_init(n:*mut hlist_node) { if !hlist_unhashed(n){__hlist_del(n);INIT_HLIST_NODE(n);} }
#[inline(always)] pub unsafe fn hlist_add_head(n:*mut hlist_node,h:*mut hlist_head){let f=(*h).first;(*n).next=f;if !f.is_null(){(*f).pprev=&mut (*n).next;}(*h).first=n;(*n).pprev=&mut (*h).first;}
#[inline(always)] pub unsafe fn hlist_count_nodes(h:*mut hlist_head)->size_t {let mut n=0;let mut p=(*h).first;while !p.is_null(){n+=1;p=(*p).next;}n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
