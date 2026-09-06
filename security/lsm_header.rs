// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LSM functions
 */

// C dependencies:
// #include <linux/printk.h>
// #include <linux/lsm_hooks.h>
// #include <linux/lsm_count.h>

/* LSM debugging */
unsafe extern "C" {
    pub static mut lsm_debug: bool;
}

macro_rules! lsm_pr {
    ($($arg:tt)*) => {
        pr_info!($($arg)*)
    };
}

macro_rules! lsm_pr_cont {
    ($($arg:tt)*) => {
        pr_cont!($($arg)*)
    };
}

macro_rules! lsm_pr_dbg {
    ($($arg:tt)*) => {{
        if unsafe { lsm_debug } {
            pr_info!($($arg)*);
        }
    }};
}

/* List of configured LSMs */
unsafe extern "C" {
    pub static mut lsm_active_cnt: ::core::ffi::c_uint;
    pub static mut lsm_idlist: [*const lsm_id; LSM_COUNT];
}

/* LSM blob configuration */
unsafe extern "C" {
    pub static mut blob_sizes: lsm_blob_sizes;
}

/* LSM blob caches */
unsafe extern "C" {
    pub static mut lsm_file_cache: *mut kmem_cache;
    pub static mut lsm_backing_file_cache: *mut kmem_cache;
    pub static mut lsm_inode_cache: *mut kmem_cache;
}

/* LSM blob allocators */
unsafe extern "C" {
    pub fn lsm_cred_alloc(cred: *mut cred, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn lsm_task_alloc(task: *mut task_struct) -> ::core::ffi::c_int;
}

/* LSM framework initializers */

// C conditional:
// #ifdef CONFIG_SECURITYFS
unsafe extern "C" {
    pub fn securityfs_init() -> ::core::ffi::c_int;
}
// #else
#[inline]
pub fn securityfs_init_disabled() -> ::core::ffi::c_int {
    0
}
// #endif /* CONFIG_SECURITYFS */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
