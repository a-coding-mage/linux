/* SPDX-License-Identifier: GPL-2.0 */

/*
 * RCU-protected bl list version. See include/linux/list_bl.h.
 *
 * The C headers linux/list_bl.h and linux/rcupdate.h provide the referenced
 * types, constants, and operations.
 */

/* return the first ptr or next element in an RCU protected list */
#[macro_export]
macro_rules! hlist_bl_first_rcu {
    ($head:expr) => {
        (*(&mut (*$head).first as *mut _ as *mut *mut hlist_bl_node))
    };
}

#[macro_export]
macro_rules! hlist_bl_next_rcu {
    ($node:expr) => {
        (*(&mut (*$node).next as *mut _ as *mut *mut hlist_bl_node))
    };
}

#[inline]
pub unsafe fn hlist_bl_set_first_rcu(h: *mut hlist_bl_head, n: *mut hlist_bl_node) {
    LIST_BL_BUG_ON((n as usize & LIST_BL_LOCKMASK) != 0);
    LIST_BL_BUG_ON(((*h).first as usize & LIST_BL_LOCKMASK) != LIST_BL_LOCKMASK);
    rcu_assign_pointer(
        hlist_bl_first_rcu!(h),
        (n as usize | LIST_BL_LOCKMASK) as *mut hlist_bl_node,
    );
}

#[inline]
pub unsafe fn hlist_bl_first_rcu_dereference(
    head: *mut hlist_bl_head,
) -> *mut hlist_bl_node {
    ((rcu_dereference_check(
        hlist_bl_first_rcu!(head),
        hlist_bl_is_locked(head),
    ) as usize
        & !LIST_BL_LOCKMASK) as *mut hlist_bl_node)
}

/**
 * hlist_bl_del_rcu - deletes entry from hash list without re-initialization
 * @n: the element to delete from the hash list.
 *
 * Note: hlist_bl_unhashed() on entry does not return true after this,
 * the entry is in an undefined state. It is useful for RCU based
 * lockfree traversal.
 *
 * In particular, it means that we can not poison the forward
 * pointers that may still be used for walking the hash list.
 */
#[inline]
pub unsafe fn hlist_bl_del_rcu(n: *mut hlist_bl_node) {
    __hlist_bl_del(n);
    (*n).pprev = LIST_POISON2;
}

/**
 * hlist_bl_add_head_rcu - add an element to an hlist_bl while permitting
 * racing traversals.
 */
#[inline]
pub unsafe fn hlist_bl_add_head_rcu(n: *mut hlist_bl_node, h: *mut hlist_bl_head) {
    let first: *mut hlist_bl_node;

    /* don't need hlist_bl_first_rcu* because we're under lock */
    first = hlist_bl_first(h);

    (*n).next = first;
    if !first.is_null() {
        (*first).pprev = &mut (*n).next;
    }
    (*n).pprev = &mut (*h).first;

    /* need _rcu because we can have concurrent lock free readers */
    hlist_bl_set_first_rcu(h, n);
}

/**
 * hlist_bl_for_each_entry_rcu - iterate over rcu list of given type
 *
 * This preserves the C traversal as a Rust macro; `$tpos` and `$pos` are
 * caller-owned mutable cursor expressions.
 */
#[macro_export]
macro_rules! hlist_bl_for_each_entry_rcu {
    ($tpos:ident, $pos:ident, $head:expr, $member:ident, $ty:ty) => {
        for $pos in std::iter::successors(
            Some(unsafe { hlist_bl_first_rcu_dereference($head) }),
            |p| unsafe {
                if p.is_null() {
                    None
                } else {
                    Some(rcu_dereference_raw(hlist_bl_next_rcu!(*p)))
                }
            },
        ) {
            if $pos.is_null() {
                break;
            }
            $tpos = unsafe { hlist_bl_entry($pos, std::mem::MaybeUninit::<$ty>::uninit(), $member) };
        }
    };
}

#[macro_export]
macro_rules! hlist_bl_for_each_entry_continue_rcu {
    ($tpos:ident, $pos:ident, $member:ident, $ty:ty) => {
        for $pos in std::iter::successors(
            Some(unsafe { rcu_dereference_raw(hlist_bl_next_rcu!(&mut (*$tpos).$member)) }),
            |p| unsafe {
                if p.is_null() { None } else { Some(rcu_dereference_raw(hlist_bl_next_rcu!(*p))) }
            },
        ) {
            if $pos.is_null() { break; }
            $tpos = unsafe { hlist_bl_entry($pos, std::mem::MaybeUninit::<$ty>::uninit(), $member) };
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
