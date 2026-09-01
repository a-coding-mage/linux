/* SPDX-License-Identifier: GPL-2.0 */

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct list {
    pub next: *mut list,
    pub prev: *mut list,
}

#[inline]
pub unsafe fn list_init(list: *mut list) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
pub unsafe fn list_empty(list: *mut list) -> ::std::os::raw::c_int {
    unsafe { ((*list).next == list) as ::std::os::raw::c_int }
}

#[inline]
pub unsafe fn list_insert(link: *mut list, new_link: *mut list) {
    unsafe {
        (*new_link).prev = (*link).prev;
        (*new_link).next = link;
        (*(*new_link).prev).next = new_link;
        (*(*new_link).next).prev = new_link;
    }
}

#[inline]
pub unsafe fn list_append(list: *mut list, new_link: *mut list) {
    unsafe {
        list_insert(list as *mut list, new_link);
    }
}

#[inline]
pub unsafe fn list_prepend(list: *mut list, new_link: *mut list) {
    unsafe {
        list_insert((*list).next, new_link);
    }
}

#[inline]
pub unsafe fn list_remove(link: *mut list) {
    unsafe {
        (*(*link).prev).next = (*link).next;
        (*(*link).next).prev = (*link).prev;
    }
}

#[macro_export]
macro_rules! list_entry {
    ($link:expr, $type:ty, $member:tt) => {
        (($link as *mut u8).wrapping_sub(::std::mem::offset_of!($type, $member)) as *mut $type)
    };
}

#[macro_export]
macro_rules! list_head {
    ($list:expr, $type:ty, $member:tt) => {
        $crate::list_entry!((*($list)).next, $type, $member)
    };
}

#[macro_export]
macro_rules! list_tail {
    ($list:expr, $type:ty, $member:tt) => {
        $crate::list_entry!((*($list)).prev, $type, $member)
    };
}

#[macro_export]
macro_rules! list_next {
    ($elm:expr, $type:ty, $member:tt) => {
        $crate::list_entry!((*($elm)).$member.next, $type, $member)
    };
}

/*
 * C's list_next and list_for_each_entry macros use typeof(*elm/pos). Rust has
 * no direct macro equivalent for deriving that container type from a raw
 * pointer expression, so the Rust macros preserve the pointer arithmetic and
 * require the container type as an explicit macro argument.
 */
#[macro_export]
macro_rules! list_for_each_entry {
    ($pos:ident, $list:expr, $type:ty, $member:tt, $body:block) => {{
        $pos = $crate::list_head!($list, $type, $member);
        while ::std::ptr::addr_of_mut!((*$pos).$member) != ($list) {
            $body
            $pos = $crate::list_next!($pos, $type, $member);
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
