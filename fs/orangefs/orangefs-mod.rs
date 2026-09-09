// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * Changes by Acxiom Corporation to add proc file handler for pvfs2 client
 * parameters, Copyright Acxiom Corporation, 2005.
 *
 * See COPYING in top-level directory.
 */

// Declarations supplied by protocol.h, orangefs-kernel.h,
// orangefs-debugfs.h, and orangefs-sysfs.h are external dependencies.

/* ORANGEFS_VERSION is a ./configure define. */
#[cfg(not(ORANGEFS_VERSION))]
const ORANGEFS_VERSION: *const u8 = b"upstream\0".as_ptr();

/* global variables declared here */
#[no_mangle]
pub static mut orangefs_stats: orangefs_stats = orangefs_stats::default();

/* the size of the hash tables for ops in progress */
#[no_mangle]
pub static mut hash_table_size: ::core::ffi::c_int = 509;

static mut module_parm_debug_mask: ::core::ffi::c_ulong = 0;
#[no_mangle]
pub static mut orangefs_gossip_debug_mask: u64 = 0;
#[no_mangle]
pub static mut op_timeout_secs: ::core::ffi::c_int = ORANGEFS_DEFAULT_OP_TIMEOUT_SECS;
#[no_mangle]
pub static mut slot_timeout_secs: ::core::ffi::c_int = ORANGEFS_DEFAULT_SLOT_TIMEOUT_SECS;
#[no_mangle]
pub static mut orangefs_cache_timeout_msecs: ::core::ffi::c_int = 500;
#[no_mangle]
pub static mut orangefs_dcache_timeout_msecs: ::core::ffi::c_int = 50;
#[no_mangle]
pub static mut orangefs_getattr_timeout_msecs: ::core::ffi::c_int = 50;

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("ORANGEFS Development Team");
// MODULE_DESCRIPTION("The Linux Kernel VFS interface to ORANGEFS");
// MODULE_PARM_DESC(module_parm_debug_mask, "debugging level (see orangefs-debug.h for values)");
// MODULE_PARM_DESC(op_timeout_secs, "Operation timeout in seconds");
// MODULE_PARM_DESC(slot_timeout_secs, "Slot timeout in seconds");
// MODULE_PARM_DESC(hash_table_size, "size of hash table for operations in progress");

static mut orangefs_fs_type: file_system_type = file_system_type {
    name: b"pvfs2\0".as_ptr() as *const _,
    init_fs_context: Some(orangefs_init_fs_context),
    parameters: orangefs_fs_param_spec,
    kill_sb: Some(orangefs_kill_sb),
    owner: THIS_MODULE,
};

// module_param(hash_table_size, int, 0);
// module_param(module_parm_debug_mask, ulong, 0644);
// module_param(op_timeout_secs, int, 0);
// module_param(slot_timeout_secs, int, 0);

/* Blocks non-priority requests from being queued for servicing. */
static mut orangefs_request_mutex: mutex = DEFINE_MUTEX!();

/* hash table for storing operations waiting for matching downcall */
#[no_mangle]
pub static mut orangefs_htable_ops_in_progress: *mut list_head = core::ptr::null_mut();
static mut orangefs_htable_ops_in_progress_lock: spinlock_t = DEFINE_SPINLOCK!();

/* list for queueing upcall operations */
static mut orangefs_request_list: list_head = LIST_HEAD_INIT!();

/* used to protect the above orangefs_request_list */
static mut orangefs_request_list_lock: spinlock_t = DEFINE_SPINLOCK!();

/* used for incoming request notification */
static mut orangefs_request_list_waitq: wait_queue_head = DECLARE_WAIT_QUEUE_HEAD!();

unsafe fn orangefs_init() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut i: u32 = 0;

    if op_timeout_secs < 0 { op_timeout_secs = 0; }
    if slot_timeout_secs < 0 { slot_timeout_secs = 0; }

    ret = op_cache_initialize();
    if ret < 0 { return ret; }

    ret = orangefs_inode_cache_initialize();
    if ret < 0 { op_cache_finalize(); return ret; }

    orangefs_htable_ops_in_progress = kzalloc_objs::<list_head>(hash_table_size);
    if orangefs_htable_ops_in_progress.is_null() {
        ret = -ENOMEM;
        orangefs_inode_cache_finalize();
        op_cache_finalize();
        return ret;
    }

    for i in 0..(hash_table_size as u32) {
        INIT_LIST_HEAD!(&mut *orangefs_htable_ops_in_progress.add(i as usize));
    }

    ret = fsid_key_table_initialize();
    if ret < 0 {
        kfree(orangefs_htable_ops_in_progress);
        orangefs_inode_cache_finalize();
        op_cache_finalize();
        return ret;
    }

    ret = orangefs_prepare_debugfs_help_string(1);
    if ret != 0 {
        fsid_key_table_finalize();
        kfree(orangefs_htable_ops_in_progress);
        orangefs_inode_cache_finalize();
        op_cache_finalize();
        return ret;
    }

    orangefs_debugfs_init(module_parm_debug_mask);
    ret = orangefs_sysfs_init();
    if ret != 0 {
        orangefs_debugfs_cleanup();
        fsid_key_table_finalize();
        kfree(orangefs_htable_ops_in_progress);
        orangefs_inode_cache_finalize();
        op_cache_finalize();
        return ret;
    }

    ret = orangefs_dev_init();
    if ret < 0 {
        gossip_err!("%s: could not initialize device subsystem %d!\n", __func__, ret);
        orangefs_sysfs_exit();
        orangefs_debugfs_cleanup();
        fsid_key_table_finalize();
        kfree(orangefs_htable_ops_in_progress);
        orangefs_inode_cache_finalize();
        op_cache_finalize();
        return ret;
    }

    ret = register_filesystem(&mut orangefs_fs_type);
    if ret == 0 {
        pr_info!("%s: module version %s loaded\n", __func__, ORANGEFS_VERSION);
        return ret;
    }

    orangefs_dev_cleanup();
    orangefs_sysfs_exit();
    orangefs_debugfs_cleanup();
    fsid_key_table_finalize();
    kfree(orangefs_htable_ops_in_progress);
    orangefs_inode_cache_finalize();
    op_cache_finalize();
    ret
}

unsafe fn orangefs_exit() {
    let mut i = 0;
    gossip_debug!(GOSSIP_INIT_DEBUG, "orangefs: orangefs_exit called\n");
    unregister_filesystem(&mut orangefs_fs_type);
    orangefs_debugfs_cleanup();
    orangefs_sysfs_exit();
    fsid_key_table_finalize();
    orangefs_dev_cleanup();
    BUG_ON!(!list_empty(&orangefs_request_list));
    for i in 0..hash_table_size { BUG_ON!(!list_empty(&*orangefs_htable_ops_in_progress.add(i as usize))); }
    orangefs_inode_cache_finalize();
    op_cache_finalize();
    kfree(orangefs_htable_ops_in_progress);
    pr_info!("orangefs: module version %s unloaded\n", ORANGEFS_VERSION);
}

/* Walk operations in progress and mark them as purged. */
#[no_mangle]
pub unsafe extern "C" fn purge_inprogress_ops() {
    for i in 0..hash_table_size {
        let mut op: *mut orangefs_kernel_op_s;
        let mut next: *mut orangefs_kernel_op_s;
        spin_lock(&mut orangefs_htable_ops_in_progress_lock);
        list_for_each_entry_safe!(op, next, &mut *orangefs_htable_ops_in_progress.add(i as usize), list, {
            set_op_state_purged(op);
            gossip_debug!(GOSSIP_DEV_DEBUG, "%s: op:%s: op_state:%d: process:%s:\n",
                __func__, get_opname_string(op), (*op).op_state, (*current).comm);
        });
        spin_unlock(&mut orangefs_htable_ops_in_progress_lock);
    }
}

// module_init(orangefs_init);
// module_exit(orangefs_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
