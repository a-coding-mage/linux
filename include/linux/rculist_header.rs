/* SPDX-License-Identifier: GPL-2.0 */
// Translation of the Linux RCU-protected list header.  The original code is
// kernel-only and depends on the declarations supplied by linux/list.h and
// linux/rcupdate.h.

#[inline]
pub unsafe fn INIT_LIST_HEAD_RCU(list: *mut list_head) {
    WRITE_ONCE((*list).next, list);
    WRITE_ONCE((*list).prev, list);
}

#[macro_export]
macro_rules! list_next_rcu { ($list:expr) => { (*((&mut (*$list).next) as *mut *mut list_head)) }; }
#[macro_export]
macro_rules! list_bidir_prev_rcu { ($list:expr) => { (*((&mut (*$list).prev) as *mut *mut list_head)) }; }
#[macro_export]
macro_rules! list_tail_rcu { ($head:expr) => { (*((&mut (*$head).prev) as *mut *mut list_head)) }; }

#[macro_export]
macro_rules! list_for_each_rcu {
    ($pos:ident, $head:expr) => {
        for $pos = rcu_dereference((*$head).next);
             !list_is_head($pos, $head);
             $pos = rcu_dereference((*$pos).next) {}
    };
}

#[inline] pub fn check_arg_count_one<T>(_dummy: T) {}
#[cfg(feature = "CONFIG_PROVE_RCU_LIST")]
#[inline] pub unsafe fn __list_check_rcu<C>(_dummy: C, cond: bool) {
    RCU_LOCKDEP_WARN(!cond && !rcu_read_lock_any_held(), "RCU-list traversed in non-reader section!");
}
#[cfg(not(feature = "CONFIG_PROVE_RCU_LIST"))]
#[inline] pub fn __list_check_rcu<C>(_dummy: C, _cond: bool) {}
#[cfg(feature = "CONFIG_PROVE_RCU_LIST")]
#[inline] pub unsafe fn __list_check_srcu(cond: bool) {
    RCU_LOCKDEP_WARN(!cond, "RCU-list traversed without holding the required lock!");
}
#[cfg(not(feature = "CONFIG_PROVE_RCU_LIST"))]
#[inline] pub fn __list_check_srcu(_cond: bool) {}

#[inline]
pub unsafe fn __list_add_rcu(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    if !__list_add_valid(new, prev, next) { return; }
    (*new).next = next; (*new).prev = prev;
    rcu_assign_pointer(list_next_rcu!(prev), new);
    (*next).prev = new;
}
#[inline] pub unsafe fn list_add_rcu(new: *mut list_head, head: *mut list_head) { __list_add_rcu(new, head, (*head).next); }
#[inline] pub unsafe fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head) { __list_add_rcu(new, (*head).prev, head); }
#[inline] pub unsafe fn list_del_rcu(entry: *mut list_head) { __list_del_entry(entry); (*entry).prev = LIST_POISON2; }
#[inline] pub unsafe fn list_bidir_del_rcu(entry: *mut list_head) { __list_del_entry(entry); }

#[inline]
pub unsafe fn hlist_del_init_rcu(n: *mut hlist_node) {
    if !hlist_unhashed(n) { __hlist_del(n); WRITE_ONCE((*n).pprev, core::ptr::null_mut()); }
}
#[inline]
pub unsafe fn list_replace_rcu(old: *mut list_head, new: *mut list_head) {
    (*new).next = (*old).next; (*new).prev = (*old).prev;
    rcu_assign_pointer(list_next_rcu!((*new).prev), new);
    (*(*new).next).prev = new; (*old).prev = LIST_POISON2;
}
#[inline]
pub unsafe fn __list_splice_rcu(list: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    let first = (*list).next; let last = (*list).prev;
    (*last).next = next; (*first).prev = prev; (*next).prev = last;
    rcu_assign_pointer(list_next_rcu!(prev), first);
}
#[inline] pub unsafe fn list_splice_rcu(list: *mut list_head, head: *mut list_head) { if !list_empty(list) { __list_splice_rcu(list, head, (*head).next); } }
#[inline]
pub unsafe fn __list_splice_init_rcu(list: *mut list_head, prev: *mut list_head, next: *mut list_head, sync: unsafe extern "C" fn()) {
    let first = (*list).next; let last = (*list).prev;
    INIT_LIST_HEAD_RCU(list); sync(); ASSERT_EXCLUSIVE_ACCESS!(*first); ASSERT_EXCLUSIVE_ACCESS!(*last);
    (*last).next = next; rcu_assign_pointer(list_next_rcu!(prev), first);
    (*first).prev = prev; (*next).prev = last;
}
#[inline] pub unsafe fn list_splice_init_rcu(list: *mut list_head, head: *mut list_head, sync: unsafe extern "C" fn()) { if !list_empty(list) { __list_splice_init_rcu(list, head, (*head).next, sync); } }
#[inline] pub unsafe fn list_splice_tail_init_rcu(list: *mut list_head, head: *mut list_head, sync: unsafe extern "C" fn()) { if !list_empty(list) { __list_splice_init_rcu(list, (*head).prev, head, sync); } }

#[macro_export] macro_rules! list_entry_rcu { ($ptr:expr, $type:ty, $member:tt) => { container_of!(READ_ONCE($ptr), $type, $member) }; }
#[macro_export] macro_rules! list_first_or_null_rcu { ($ptr:expr, $type:ty, $member:tt) => {{ let __ptr = $ptr; let __next = READ_ONCE((*__ptr).next); if likely(__ptr != __next) { list_entry_rcu!(__next, $type, $member) } else { core::ptr::null_mut() } }}; }
#[macro_export] macro_rules! list_next_or_null_rcu { ($head:expr, $ptr:expr, $type:ty, $member:tt) => {{ let __head=$head; let __ptr=$ptr; let __next=READ_ONCE((*__ptr).next); if likely(__next != __head) { list_entry_rcu!(__next, $type, $member) } else { core::ptr::null_mut() } }}; }

// The following C variadic/typeof iteration macros are retained as Rust
// macro forms; their cursor and member expressions preserve the source flow.
#[macro_export] macro_rules! list_for_each_entry_rcu { ($pos:ident,$head:expr,$member:tt $(,$cond:expr)*) => { for $pos = list_entry_rcu!((*$head).next, _, $member); &(*$pos).$member != $head; $pos = list_entry_rcu!((*(*$pos).$member).next, _, $member) {} }; }
#[macro_export] macro_rules! list_for_each_entry_srcu { ($pos:ident,$head:expr,$member:tt,$cond:expr) => { list_for_each_entry_rcu!($pos,$head,$member) }; }
#[macro_export] macro_rules! list_entry_lockless { ($ptr:expr,$type:ty,$member:tt) => { container_of!(READ_ONCE($ptr),$type,$member) }; }
#[macro_export] macro_rules! list_for_each_entry_lockless { ($pos:ident,$head:expr,$member:tt) => { list_for_each_entry_rcu!($pos,$head,$member) }; }
#[macro_export] macro_rules! list_for_each_entry_continue_rcu { ($pos:ident,$head:expr,$member:tt) => { while !list_is_head!(&(*$pos).$member,$head) { $pos=list_entry_rcu!((*(*$pos).$member).next,_, $member); } }; }
#[macro_export] macro_rules! list_for_each_entry_from_rcu { ($pos:ident,$head:expr,$member:tt) => { list_for_each_entry_continue_rcu!($pos,$head,$member) }; }

#[inline] pub unsafe fn hlist_del_rcu(n:*mut hlist_node) { __hlist_del(n); WRITE_ONCE((*n).pprev, LIST_POISON2); }
#[inline]
pub unsafe fn hlist_replace_rcu(old:*mut hlist_node,new:*mut hlist_node) { let next=(*old).next; (*new).next=next; WRITE_ONCE((*new).pprev,(*old).pprev); rcu_assign_pointer!(*( (*new).pprev as *mut *mut hlist_node),new); if !next.is_null(){WRITE_ONCE((*next).pprev,&mut (*new).next);} WRITE_ONCE((*old).pprev,LIST_POISON2); }
#[inline]
pub unsafe fn hlists_swap_heads_rcu(left:*mut hlist_head,right:*mut hlist_head) { let node1=(*left).first; let node2=(*right).first; rcu_assign_pointer!((*left).first,node2); rcu_assign_pointer!((*right).first,node1); WRITE_ONCE((*node2).pprev,&mut (*left).first); WRITE_ONCE((*node1).pprev,&mut (*right).first); }
#[macro_export] macro_rules! hlist_first_rcu { ($head:expr) => { (*((&mut (*$head).first) as *mut *mut hlist_node)) }; }
#[macro_export] macro_rules! hlist_next_rcu { ($node:expr) => { (*((&mut (*$node).next) as *mut *mut hlist_node)) }; }
#[macro_export] macro_rules! hlist_pprev_rcu { ($node:expr) => { (*(((*$node).pprev) as *mut *mut hlist_node)) }; }
#[inline]
pub unsafe fn hlist_add_head_rcu(n:*mut hlist_node,h:*mut hlist_head) { let first=(*h).first; (*n).next=first; WRITE_ONCE((*n).pprev,&mut (*h).first); rcu_assign_pointer!(hlist_first_rcu!(h),n); if !first.is_null(){WRITE_ONCE((*first).pprev,&mut (*n).next);} }
#[inline]
pub unsafe fn hlist_add_tail_rcu(n:*mut hlist_node,h:*mut hlist_head) { let mut i=(*h).first; let mut last=core::ptr::null_mut(); while !i.is_null(){last=i;i=(*i).next;} if !last.is_null(){(*n).next=(*last).next;WRITE_ONCE((*n).pprev,&mut (*last).next);rcu_assign_pointer!(hlist_next_rcu!(last),n);}else{hlist_add_head_rcu(n,h);} }
#[inline] pub unsafe fn hlist_add_before_rcu(n:*mut hlist_node,next:*mut hlist_node){WRITE_ONCE((*n).pprev,(*next).pprev);(*n).next=next;rcu_assign_pointer!(hlist_pprev_rcu!(n),n);WRITE_ONCE((*next).pprev,&mut (*n).next);}
#[inline] pub unsafe fn hlist_add_behind_rcu(n:*mut hlist_node,prev:*mut hlist_node){(*n).next=(*prev).next;WRITE_ONCE((*n).pprev,&mut (*prev).next);rcu_assign_pointer!(hlist_next_rcu!(prev),n);if !(*n).next.is_null(){WRITE_ONCE((*(*n).next).pprev,&mut (*n).next);}}

#[macro_export] macro_rules! __hlist_for_each_rcu { ($pos:ident,$head:expr) => { for $pos=rcu_dereference(hlist_first_rcu!($head)); !$pos.is_null(); $pos=rcu_dereference(hlist_next_rcu!($pos)) {} }; }
#[macro_export] macro_rules! hlist_for_each_entry_rcu { ($pos:ident,$head:expr,$member:tt $(,$cond:expr)*) => { __hlist_for_each_rcu!($pos,$head) }; }
#[macro_export] macro_rules! hlist_for_each_entry_srcu { ($pos:ident,$head:expr,$member:tt,$cond:expr) => { hlist_for_each_entry_rcu!($pos,$head,$member) }; }
#[macro_export] macro_rules! hlist_for_each_entry_rcu_notrace { ($pos:ident,$head:expr,$member:tt) => { hlist_for_each_entry_rcu!($pos,$head,$member) }; }
#[macro_export] macro_rules! hlist_for_each_entry_rcu_bh { ($pos:ident,$head:expr,$member:tt) => { hlist_for_each_entry_rcu!($pos,$head,$member) }; }
#[macro_export] macro_rules! hlist_for_each_entry_continue_rcu { ($pos:ident,$member:tt) => { while !$pos.is_null() { $pos=rcu_dereference(hlist_next_rcu!(&mut (*$pos).$member)); } }; }
#[macro_export] macro_rules! hlist_for_each_entry_continue_rcu_bh { ($pos:ident,$member:tt) => { hlist_for_each_entry_continue_rcu!($pos,$member) }; }
#[macro_export] macro_rules! hlist_for_each_entry_from_rcu { ($pos:ident,$member:tt) => { while !$pos.is_null() { $pos=rcu_dereference(hlist_next_rcu!(&mut (*$pos).$member)); } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
