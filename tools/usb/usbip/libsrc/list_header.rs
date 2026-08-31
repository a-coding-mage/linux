/* SPDX-License-Identifier: GPL-2.0 */

/* Stripped down implementation of linked list taken
 * from the Linux Kernel.
 */

/*
 * Simple doubly linked list implementation.
 *
 * Some of the internal functions ("__xxx") are useful when
 * manipulating whole lists rather than single entries, as
 * sometimes we already know the next/prev entries and we can
 * generate better code by using them directly rather than
 * using the generic single-entry routines.
 */

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

macro_rules! LIST_HEAD_INIT {
    ($name:expr) => {
        list_head {
            next: &mut $name as *mut list_head,
            prev: &mut $name as *mut list_head,
        }
    };
}

macro_rules! LIST_HEAD {
    ($name:ident) => {
        let mut $name: list_head = LIST_HEAD_INIT!($name);
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
 */
#[inline]
pub unsafe fn __list_add(
    new: *mut list_head,
    prev: *mut list_head,
    next: *mut list_head,
) {
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
        (*prev).next = next;
    }
}

pub const POISON_POINTER_DELTA: usize = 0;
pub const LIST_POISON1: *mut core::ffi::c_void =
    (0x00100100usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;
pub const LIST_POISON2: *mut core::ffi::c_void =
    (0x00200200usize + POISON_POINTER_DELTA) as *mut core::ffi::c_void;

#[inline]
pub unsafe fn __list_del_entry(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
    }
}

/**
 * list_del - deletes entry from list.
 * @entry: the element to delete from the list.
 * Note: list_empty() on entry does not return true after this, the entry is
 * in an undefined state.
 */
#[inline]
pub unsafe fn list_del(entry: *mut list_head) {
    unsafe {
        __list_del((*entry).prev, (*entry).next);
        (*entry).next = LIST_POISON1 as *mut list_head;
        (*entry).prev = LIST_POISON2 as *mut list_head;
    }
}

/**
 * list_entry - get the struct for this entry
 * @ptr:	the &struct list_head pointer.
 * @type:	the type of the struct this is embedded in.
 * @member:	the name of the list_head within the struct.
 */
macro_rules! list_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

/**
 * list_for_each	-	iterate over a list
 * @pos:	the &struct list_head to use as a loop cursor.
 * @head:	the head for your list.
 */
macro_rules! list_for_each {
    ($pos:ident, $head:expr, $body:block) => {{
        $pos = unsafe { (*$head).next };
        while $pos != $head {
            $body
            $pos = unsafe { (*$pos).next };
        }
    }};
}

/**
 * list_for_each_safe - iterate over a list safe against removal of list entry
 * @pos:	the &struct list_head to use as a loop cursor.
 * @n:		another &struct list_head to use as temporary storage
 * @head:	the head for your list.
 */
macro_rules! list_for_each_safe {
    ($pos:ident, $n:ident, $head:expr, $body:block) => {{
        $pos = unsafe { (*$head).next };
        $n = unsafe { (*$pos).next };
        while $pos != $head {
            $body
            $pos = $n;
            $n = unsafe { (*$pos).next };
        }
    }};
}

macro_rules! offsetof {
    ($type:ty, $member:ident) => {{
        let uninit = core::mem::MaybeUninit::<$type>::uninit();
        let base = uninit.as_ptr();
        unsafe { core::ptr::addr_of!((*base).$member) as usize - base as usize }
    }};
}

/**
 * container_of - cast a member of a structure out to the containing structure
 * @ptr:	the pointer to the member.
 * @type:	the type of the container struct this is embedded in.
 * @member:	the name of the member within the struct.
 *
 */
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let __mptr = $ptr;
        (__mptr as *mut u8).wrapping_sub(offsetof!($type, $member)) as *mut $type
    }};
}
