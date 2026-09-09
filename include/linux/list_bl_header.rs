/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/list.h, linux/bit_spinlock.h

#[cfg(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK"))]
pub const LIST_BL_LOCKMASK: usize = 1;
#[cfg(not(any(feature = "CONFIG_SMP", feature = "CONFIG_DEBUG_SPINLOCK")))]
pub const LIST_BL_LOCKMASK: usize = 0;

#[repr(C)]
pub struct hlist_bl_head {
    pub first: *mut hlist_bl_node,
}

#[repr(C)]
pub struct hlist_bl_node {
    pub next: *mut hlist_bl_node,
    pub pprev: *mut *mut hlist_bl_node,
}

#[inline]
pub unsafe fn INIT_HLIST_BL_HEAD(ptr: *mut hlist_bl_head) {
    (*ptr).first = core::ptr::null_mut();
}

#[inline]
pub unsafe fn INIT_HLIST_BL_NODE(h: *mut hlist_bl_node) {
    (*h).next = core::ptr::null_mut();
    (*h).pprev = core::ptr::null_mut();
}

// Equivalent to the kernel's container_of(ptr, type, member) macro.
#[macro_export]
macro_rules! hlist_bl_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

#[inline]
pub unsafe fn hlist_bl_unhashed(h: *const hlist_bl_node) -> bool {
    (*h).pprev.is_null()
}

#[inline]
pub unsafe fn hlist_bl_first(h: *mut hlist_bl_head) -> *mut hlist_bl_node {
    ((*h).first as usize & !LIST_BL_LOCKMASK) as *mut hlist_bl_node
}

#[inline]
pub unsafe fn hlist_bl_set_first(h: *mut hlist_bl_head, n: *mut hlist_bl_node) {
    // LIST_BL_BUG_ON((unsigned long)n & LIST_BL_LOCKMASK);
    // LIST_BL_BUG_ON(((unsigned long)h->first & LIST_BL_LOCKMASK) != LIST_BL_LOCKMASK);
    (*h).first = ((n as usize) | LIST_BL_LOCKMASK) as *mut hlist_bl_node;
}

#[inline]
pub unsafe fn hlist_bl_empty(h: *const hlist_bl_head) -> bool {
    let first = core::ptr::read_volatile(&(*h).first);
    (first as usize & !LIST_BL_LOCKMASK) == 0
}

#[inline]
pub unsafe fn hlist_bl_add_head(n: *mut hlist_bl_node, h: *mut hlist_bl_head) {
    let first = hlist_bl_first(h);
    (*n).next = first;
    if !first.is_null() {
        (*first).pprev = &mut (*n).next;
    }
    (*n).pprev = &mut (*h).first;
    hlist_bl_set_first(h, n);
}

#[inline]
pub unsafe fn hlist_bl_add_before(n: *mut hlist_bl_node, next: *mut hlist_bl_node) {
    let pprev = (*next).pprev;
    (*n).pprev = pprev;
    (*n).next = next;
    (*next).pprev = &mut (*n).next;
    let old = core::ptr::read_volatile(pprev);
    core::ptr::write_volatile(pprev, ((n as usize) | (old as usize & LIST_BL_LOCKMASK)) as *mut hlist_bl_node);
}

#[inline]
pub unsafe fn hlist_bl_add_behind(n: *mut hlist_bl_node, prev: *mut hlist_bl_node) {
    (*n).next = (*prev).next;
    (*n).pprev = &mut (*prev).next;
    (*prev).next = n;
    if !(*n).next.is_null() {
        (*(*n).next).pprev = &mut (*n).next;
    }
}

#[inline]
pub unsafe fn __hlist_bl_del(n: *mut hlist_bl_node) {
    let next = (*n).next;
    let pprev = (*n).pprev;
    // LIST_BL_BUG_ON((unsigned long)n & LIST_BL_LOCKMASK);
    let old = core::ptr::read_volatile(pprev);
    core::ptr::write_volatile(pprev, ((next as usize) | (old as usize & LIST_BL_LOCKMASK)) as *mut hlist_bl_node);
    if !next.is_null() {
        (*next).pprev = pprev;
    }
}

#[inline]
pub unsafe fn hlist_bl_del(n: *mut hlist_bl_node) {
    __hlist_bl_del(n);
    (*n).next = LIST_POISON1;
    (*n).pprev = LIST_POISON2;
}

#[inline]
pub unsafe fn hlist_bl_del_init(n: *mut hlist_bl_node) {
    if !hlist_bl_unhashed(n) {
        __hlist_bl_del(n);
        INIT_HLIST_BL_NODE(n);
    }
}

extern "C" {
    pub fn bit_spin_lock(bit: u32, addr: *mut usize);
    pub fn __bit_spin_unlock(bit: u32, addr: *mut usize);
    pub fn bit_spin_is_locked(bit: u32, addr: *mut usize) -> bool;
}

#[inline]
pub unsafe fn hlist_bl_lock(b: *mut hlist_bl_head) {
    bit_spin_lock(0, b as *mut usize);
}

#[inline]
pub unsafe fn hlist_bl_unlock(b: *mut hlist_bl_head) {
    __bit_spin_unlock(0, b as *mut usize);
}

#[inline]
pub unsafe fn hlist_bl_is_locked(b: *mut hlist_bl_head) -> bool {
    bit_spin_is_locked(0, b as *mut usize)
}

// The C iteration macros use typeof(*tpos) and container_of; callers should
// provide equivalent cursor declarations and the surrounding container_of!.
#[macro_export]
macro_rules! hlist_bl_for_each_entry {
    ($tpos:ident, $pos:ident, $head:expr, $member:ident) => {
        for $pos in core::iter::successors(
            unsafe { Some(hlist_bl_first($head)) },
            |p| unsafe { if (**p).next.is_null() { None } else { Some((**p).next) } },
        ) {
            if $pos.is_null() { break; }
            let $tpos = hlist_bl_entry!($pos, _, $member);
            let _ = &$tpos;
        }
    };
}

#[macro_export]
macro_rules! hlist_bl_for_each_entry_safe {
    ($tpos:ident, $pos:ident, $n:ident, $head:expr, $member:ident) => {
        for $pos in core::iter::successors(
            unsafe { Some(hlist_bl_first($head)) },
            |p| unsafe { if (**p).next.is_null() { None } else { Some((**p).next) } },
        ) {
            if $pos.is_null() { break; }
            let $n = unsafe { (*$pos).next };
            let $tpos = hlist_bl_entry!($pos, _, $member);
            let _ = (&$tpos, &$n);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
