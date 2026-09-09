/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by other headers are intentionally left as
// external Rust names, as in the original C header.
pub struct coredump_params;
pub struct pid;
pub struct file;
pub struct task_struct;
pub struct dentry_operations;

// `gfp_t` and `GFP_KERNEL` are supplied by linux/gfp_types.h.
extern "C" {
    pub fn pidfs_alloc_file(pid: *mut pid, flags: ::core::ffi::c_uint) -> *mut file;
    pub fn pidfs_init();
    pub fn pidfs_prepare_pid(pid: *mut pid);
    pub fn pidfs_add_pid(pid: *mut pid) -> ::core::ffi::c_int;
    pub fn pidfs_remove_pid(pid: *mut pid);
    pub fn pidfs_exit(tsk: *mut task_struct);

    // Conditional on CONFIG_COREDUMP in the C build.
    pub fn pidfs_coredump(cprm: *const coredump_params);

    pub static pidfs_dentry_operations: dentry_operations;
    pub fn pidfs_register_pid_gfp(pid: *mut pid, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn pidfs_free_pid(pid: *mut pid);
}

/**
 * pidfs_register_pid - register a struct pid in pidfs
 * @pid: pid to pin
 *
 * Register a struct pid in pidfs.
 *
 * Return: On success zero, on error a negative error code is returned.
 */
#[inline]
pub unsafe fn pidfs_register_pid(pid: *mut pid) -> ::core::ffi::c_int {
    pidfs_register_pid_gfp(pid, GFP_KERNEL)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
