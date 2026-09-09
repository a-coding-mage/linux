// SPDX-License-Identifier: GPL-2.0
/* User-mappable watch queue
 *
 * Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * See Documentation/core-api/watch_queue.rst
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub struct watch_type_filter {
    pub type_: enum_watch_notification_type,
    pub subtype_filter: [__u32; 1], /* Bitmask of subtypes to filter on */
    pub info_filter: __u32,         /* Filter on watch_notification::info */
    pub info_mask: __u32,           /* Mask of relevant bits in info_filter */
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub union watch_filter__bindgen_ty_1 {
    pub rcu: rcu_head,
    /* Bitmask of accepted types */
    pub type_filter: [c_ulong; WATCH_TYPE__NR as usize],
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub struct watch_filter {
    pub __bindgen_anon_1: watch_filter__bindgen_ty_1,
    pub nr_filters: u32, /* Number of filters */
    pub filters: [watch_type_filter; 0],
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub struct watch_queue {
    pub rcu: rcu_head,
    pub filter: *mut watch_filter,
    pub pipe: *mut pipe_inode_info, /* Pipe we use as a buffer, NULL if queue closed */
    pub watches: hlist_head,        /* Contributory watches */
    pub notes: *mut *mut page,      /* Preallocated notifications */
    pub notes_bitmap: *mut c_ulong, /* Allocation bitmap for notes */
    pub usage: kref,                /* Object usage count */
    pub lock: spinlock_t,
    pub nr_notes: c_uint,  /* Number of notes */
    pub nr_pages: c_uint,  /* Number of pages in notes[] */
}

/* Representation of a watch on an object. */
#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub union watch__bindgen_ty_1 {
    pub rcu: rcu_head,
    pub info_id: u32, /* ID to be OR'd in to info field */
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub struct watch {
    pub __bindgen_anon_1: watch__bindgen_ty_1,
    pub queue: *mut watch_queue,       /* Queue to post events to */
    pub queue_node: hlist_node,        /* Link in queue->watches */
    pub watch_list: *mut watch_list,
    pub list_node: hlist_node,         /* Link in watch_list->watchers */
    pub cred: *const cred,             /* Creds of the owner of the watch */
    pub private: *mut c_void,          /* Private data for the watched object */
    pub id: u64,                       /* Internal identifier */
    pub usage: kref,                   /* Object usage count */
}

/* List of watches on an object. */
#[cfg(CONFIG_WATCH_QUEUE)]
#[repr(C)]
pub struct watch_list {
    pub rcu: rcu_head,
    pub watchers: hlist_head,
    pub release_watch: Option<unsafe extern "C" fn(*mut watch)>,
    pub lock: spinlock_t,
}

#[cfg(CONFIG_WATCH_QUEUE)]
extern "C" {
    pub fn __post_watch_notification(
        wlist: *mut watch_list,
        n: *mut watch_notification,
        cred: *const cred,
        id: u64,
    );
    pub fn get_watch_queue(which: c_int) -> *mut watch_queue;
    pub fn put_watch_queue(queue: *mut watch_queue);
    pub fn init_watch(watch: *mut watch, queue: *mut watch_queue);
    pub fn add_watch_to_object(watch: *mut watch, wlist: *mut watch_list) -> c_int;
    pub fn remove_watch_from_object(
        wlist: *mut watch_list,
        queue: *mut watch_queue,
        id: u64,
        is_id: bool,
    ) -> c_int;
    pub fn watch_queue_set_size(pipe: *mut pipe_inode_info, nr_notes: c_uint) -> c_long;
    pub fn watch_queue_set_filter(
        pipe: *mut pipe_inode_info,
        filter: *mut watch_notification_filter,
    ) -> c_long;
    pub fn watch_queue_init(pipe: *mut pipe_inode_info) -> c_int;
    pub fn watch_queue_clear(pipe: *mut pipe_inode_info);
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[inline]
pub unsafe fn init_watch_list(
    wlist: *mut watch_list,
    release_watch: Option<unsafe extern "C" fn(*mut watch)>,
) {
    INIT_HLIST_HEAD(core::ptr::addr_of_mut!((*wlist).watchers));
    spin_lock_init(core::ptr::addr_of_mut!((*wlist).lock));
    (*wlist).release_watch = release_watch;
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[inline]
pub unsafe fn post_watch_notification(
    wlist: *mut watch_list,
    n: *mut watch_notification,
    cred: *const cred,
    id: u64,
) {
    if unlikely(!wlist.is_null()) {
        __post_watch_notification(wlist, n, cred, id);
    }
}

#[cfg(CONFIG_WATCH_QUEUE)]
#[inline]
pub unsafe fn remove_watch_list(wlist: *mut watch_list, id: u64) {
    if !wlist.is_null() {
        remove_watch_from_object(wlist, core::ptr::null_mut(), id, true);
        kfree_rcu(wlist, rcu);
    }
}

/// Calculate the information part of the size of a watch record, given the
/// structure size.
#[inline]
pub const fn watch_sizeof<const STRUCT_SIZE: usize>() -> usize {
    STRUCT_SIZE << WATCH_INFO_LENGTH__SHIFT
}

#[cfg(not(CONFIG_WATCH_QUEUE))]
#[inline]
pub unsafe fn watch_queue_init(pipe: *mut pipe_inode_info) -> c_int {
    let _ = pipe;
    -ENOPKG
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
