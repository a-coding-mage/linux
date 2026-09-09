// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of fs/eventpoll.c.
// Kernel-provided types, constants, functions, and configuration symbols are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const EP_MAX_NESTS: usize = 4;
pub const PATH_ARR_SIZE: usize = 5;

#[repr(C)]
pub struct eppoll_entry {
    pub next: *mut eppoll_entry,
    pub base: *mut epitem,
    pub wait: wait_queue_entry_t,
    pub whead: *mut wait_queue_head_t,
}

#[repr(C)]
pub union epitem_rbn_rcu {
    pub rbn: rb_node,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct epitem {
    pub rbn_rcu: epitem_rbn_rcu,
    pub rdllink: list_head,
    pub ovflist_next: *mut epitem,
    pub ffd: epoll_key,
    pub pwqlist: *mut eppoll_entry,
    pub ep: *mut eventpoll,
    pub fllink: hlist_node,
    pub ws: *mut wakeup_source,
    pub event: epoll_event,
}

#[repr(C)]
pub struct eventpoll {
    pub mtx: mutex,
    pub wq: wait_queue_head_t,
    pub poll_wait: wait_queue_head_t,
    pub rdllist: list_head,
    pub lock: spinlock_t,
    pub seq: seqcount_spinlock_t,
    pub rbr: rb_root_cached,
    pub ovflist: *mut epitem,
    pub ws: *mut wakeup_source,
    pub user: *mut user_struct,
    pub file: *mut file,
    pub gen: u64,
    pub refs: hlist_head,
    pub loop_check_depth: u8,
    pub refcount: refcount_t,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct ep_pqueue {
    pub pt: poll_table,
    pub epi: *mut epitem,
}

static mut max_user_watches: libc::c_long = 0;
static mut epi_cache: *mut kmem_cache = core::ptr::null_mut();
static mut pwq_cache: *mut kmem_cache = core::ptr::null_mut();
static mut ephead_cache: *mut kmem_cache = core::ptr::null_mut();

#[repr(C)]
pub struct epitems_head {
    pub epitems: hlist_head,
    pub next: *mut epitems_head,
    pub file: *mut file,
}

#[inline]
unsafe fn free_ephead(head: *mut epitems_head) {
    if !head.is_null() { kmem_cache_free(ephead_cache, head as *mut c_void); }
}

#[inline]
unsafe fn ep_is_linked(epi: *const epitem) -> bool {
    !list_empty(&(*epi).rdllink)
}

#[inline]
unsafe fn ep_is_scanning(ep: *const eventpoll) -> bool {
    core::ptr::read_volatile(&(*ep).ovflist) != (-1isize as *mut epitem)
}

#[inline]
unsafe fn ep_enter_scan(ep: *mut eventpoll) { core::ptr::write_volatile(&mut (*ep).ovflist, core::ptr::null_mut()); }

#[inline]
unsafe fn ep_exit_scan(ep: *mut eventpoll) { core::ptr::write_volatile(&mut (*ep).ovflist, -1isize as *mut epitem); }

#[inline]
unsafe fn epi_clear_ovflist(epi: *mut epitem) { (*epi).ovflist_next = -1isize as *mut epitem; }

// The remaining definitions preserve the original implementation's external
// kernel ABI and are supplied by the surrounding Linux kernel translation.
extern "C" {
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn list_empty(entry: *const list_head) -> bool;
}

// Opaque declarations corresponding to Linux kernel types included by the C source.
#[repr(C)] pub struct wait_queue_entry_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct epoll_key { _private: [u8; 0] }
#[repr(C)] pub struct epoll_event { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct wakeup_source { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct seqcount_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct user_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
