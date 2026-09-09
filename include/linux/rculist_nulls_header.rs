/* SPDX-License-Identifier: GPL-2.0 */
/* C header guard and __KERNEL__ conditional preserved as source intent. */
/* Dependencies: linux/list_nulls.h and linux/rcupdate.h. */

/**
 * hlist_nulls_del_init_rcu - deletes entry from hash list with re-initialization
 * @n: the element to delete from the hash list.
 *
 * Note: hlist_nulls_unhashed() on the node return true after this. It is
 * useful for RCU based read lockfree traversal if the writer side
 * must know if the list entry is still hashed or already unhashed.
 */
#[inline]
pub unsafe fn hlist_nulls_del_init_rcu(n: *mut hlist_nulls_node) {
    if !hlist_nulls_unhashed(n) {
        __hlist_nulls_del(n);
        core::ptr::write_volatile(&mut (*n).pprev, core::ptr::null_mut());
    }
}

#[macro_export]
macro_rules! hlist_nulls_first_rcu {
    ($head:expr) => { unsafe { (*($head).first) } };
}

#[macro_export]
macro_rules! hlist_nulls_next_rcu {
    ($node:expr) => { unsafe { (*($node).next) } };
}

#[macro_export]
macro_rules! hlist_nulls_pprev_rcu {
    ($node:expr) => { unsafe { (*($node).pprev) } };
}

/** hlist_nulls_del_rcu - deletes entry from hash list without re-initialization. */
#[inline]
pub unsafe fn hlist_nulls_del_rcu(n: *mut hlist_nulls_node) {
    __hlist_nulls_del(n);
    core::ptr::write_volatile(&mut (*n).pprev, LIST_POISON2);
}

#[inline]
pub unsafe fn hlist_nulls_add_head_rcu(
    n: *mut hlist_nulls_node,
    h: *mut hlist_nulls_head,
) {
    let first = (*h).first;
    core::ptr::write_volatile(&mut (*n).next, first);
    core::ptr::write_volatile(&mut (*n).pprev, &mut (*h).first);
    rcu_assign_pointer(hlist_nulls_first_rcu!(h), n);
    if !is_a_nulls(first) {
        core::ptr::write_volatile(&mut (*first).pprev, &mut (*n).next);
    }
}

#[inline]
pub unsafe fn hlist_nulls_add_tail_rcu(
    n: *mut hlist_nulls_node,
    h: *mut hlist_nulls_head,
) {
    let mut i = (*h).first;
    let mut last: *mut hlist_nulls_node = core::ptr::null_mut();
    while !is_a_nulls(i) {
        last = i;
        i = (*i).next;
    }
    if !last.is_null() {
        core::ptr::write_volatile(&mut (*n).next, (*last).next);
        core::ptr::write_volatile(&mut (*n).pprev, &mut (*last).next);
        rcu_assign_pointer(hlist_nulls_next_rcu!(last), n);
    } else {
        hlist_nulls_add_head_rcu(n, h);
    }
}

/* after that hlist_nulls_del will work */
#[inline]
pub unsafe fn hlist_nulls_add_fake(n: *mut hlist_nulls_node) {
    core::ptr::write_volatile(&mut (*n).pprev, &mut (*n).next);
    core::ptr::write_volatile(&mut (*n).next, NULLS_MARKER(core::ptr::null_mut()));
}

#[inline]
pub unsafe fn hlist_nulls_replace_rcu(
    old: *mut hlist_nulls_node,
    new: *mut hlist_nulls_node,
) {
    let next = (*old).next;
    core::ptr::write_volatile(&mut (*new).next, next);
    core::ptr::write_volatile(&mut (*new).pprev, (*old).pprev);
    rcu_assign_pointer(hlist_nulls_pprev_rcu!(new), new);
    if !is_a_nulls(next) {
        core::ptr::write_volatile(&mut (*next).pprev, &mut (*new).next);
    }
}

#[inline]
pub unsafe fn hlist_nulls_replace_init_rcu(
    old: *mut hlist_nulls_node,
    new: *mut hlist_nulls_node,
) {
    hlist_nulls_replace_rcu(old, new);
    core::ptr::write_volatile(&mut (*old).pprev, core::ptr::null_mut());
}

/* RCU traversal macros. `member` and cursor typing follow hlist_nulls_entry. */
#[macro_export]
macro_rules! hlist_nulls_for_each_entry_rcu {
    ($tpos:ident, $pos:ident, $head:expr, $member:tt) => {
        for $pos in unsafe { rcu_dereference_raw(hlist_nulls_first_rcu!($head)) } {
            if is_a_nulls($pos) { break; }
            $tpos = hlist_nulls_entry($pos, $member);
            $pos = unsafe { rcu_dereference_raw(hlist_nulls_next_rcu!($pos)) };
        }
    };
}

#[macro_export]
macro_rules! hlist_nulls_for_each_entry_safe {
    ($tpos:ident, $pos:ident, $head:expr, $member:tt) => {
        for $pos in unsafe { rcu_dereference_raw(hlist_nulls_first_rcu!($head)) } {
            if is_a_nulls($pos) { break; }
            $tpos = hlist_nulls_entry($pos, $member);
            $pos = unsafe { rcu_dereference_raw(hlist_nulls_next_rcu!($pos)) };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
