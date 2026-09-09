/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const ICQ_EXITED: u32 = 1 << 2;
pub const ICQ_DESTROYED: u32 = 1 << 3;

/*
 * An io_cq (icq) is association between an io_context (ioc) and a
 * request_queue (q).  This is used by elevators which need to track
 * information per ioc - q pair.
 *
 * Elevator can request use of icq by setting elevator_type->icq_size and
 * ->icq_align.  Both size and align must be larger than that of struct
 * io_cq and elevator can use the tail area for private information.
 *
 * As icq's are linked from both ioc and q, the locking rules are a bit
 * complex; see the original C header for the complete locking contract.
 */
#[repr(C)]
pub union IoCqNode {
    pub q_node: core::mem::ManuallyDrop<list_head>,
    pub __rcu_icq_cache: core::mem::ManuallyDrop<*mut kmem_cache>,
}

#[repr(C)]
pub union IoCqIocNode {
    pub ioc_node: core::mem::ManuallyDrop<hlist_node>,
    pub __rcu_head: core::mem::ManuallyDrop<rcu_head>,
}

#[repr(C)]
pub struct io_cq {
    pub q: *mut request_queue,
    pub ioc: *mut io_context,
    // Anonymous C union containing q_node / __rcu_icq_cache.
    pub q_node: IoCqNode,
    pub ioc_node: IoCqIocNode,
    pub flags: core::ffi::c_uint,
}

/*
 * I/O subsystem state of the associated processes.  It is refcounted
 * and kmalloc'ed. These could be shared between processes.
 */
#[repr(C)]
pub struct io_context {
    pub refcount: atomic_long_t,
    pub active_ref: atomic_t,
    pub ioprio: core::ffi::c_ushort,
    #[cfg(CONFIG_BLK_ICQ)]
    pub lock: spinlock_t,
    #[cfg(CONFIG_BLK_ICQ)]
    pub icq_tree: radix_tree_root,
    #[cfg(CONFIG_BLK_ICQ)]
    pub icq_hint: *mut io_cq,
    #[cfg(CONFIG_BLK_ICQ)]
    pub icq_list: hlist_head,
    #[cfg(CONFIG_BLK_ICQ)]
    pub release_work: work_struct,
}

pub struct task_struct;

#[cfg(CONFIG_BLOCK)]
extern "C" {
    pub fn put_io_context(ioc: *mut io_context);
    pub fn exit_io_context(task: *mut task_struct);
    pub fn __copy_io(clone_flags: u64, tsk: *mut task_struct) -> core::ffi::c_int;
}

#[cfg(CONFIG_BLOCK)]
#[inline]
pub unsafe fn copy_io(clone_flags: u64, tsk: *mut task_struct) -> core::ffi::c_int {
    if (*current).io_context.is_null() {
        return 0;
    }
    __copy_io(clone_flags, tsk)
}

#[cfg(not(CONFIG_BLOCK))]
#[inline]
pub unsafe fn put_io_context(_ioc: *mut io_context) {}

#[cfg(not(CONFIG_BLOCK))]
#[inline]
pub unsafe fn exit_io_context(_task: *mut task_struct) {}

#[cfg(not(CONFIG_BLOCK))]
#[inline]
pub unsafe fn copy_io(_clone_flags: u64, _tsk: *mut task_struct) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
