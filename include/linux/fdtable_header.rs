/* SPDX-License-Identifier: GPL-2.0 */
/*
 * descriptor table internals; you almost certainly want file.h instead.
 */

// Dependencies supplied by other translated headers are intentionally left external.

pub const NR_OPEN_DEFAULT: usize = BITS_PER_LONG;

#[repr(C)]
pub struct fdtable {
    pub max_fds: ::core::ffi::c_uint,
    pub fd: *mut *mut file, /* current fd array */
    pub close_on_exec: *mut ::core::ffi::c_ulong,
    pub open_fds: *mut ::core::ffi::c_ulong,
    pub full_fds_bits: *mut ::core::ffi::c_ulong,
    pub rcu: rcu_head,
}

/*
 * Open file table structure
 */
#[repr(C)]
pub struct files_struct {
    /*
     * read mostly part
     */
    pub count: atomic_t,
    pub resize_in_progress: bool,
    pub resize_wait: wait_queue_head_t,

    pub fdt: *mut fdtable,
    pub fdtab: fdtable,
    /*
     * written part on a separate cache line in SMP
     */
    pub file_lock: spinlock_t,
    pub next_fd: ::core::ffi::c_uint,
    pub close_on_exec_init: [::core::ffi::c_ulong; 1],
    pub open_fds_init: [::core::ffi::c_ulong; 1],
    pub full_fds_bits_init: [::core::ffi::c_ulong; 1],
    pub fd_array: [*mut file; NR_OPEN_DEFAULT],
}

pub struct file_operations;
pub struct vfsmount;
pub struct dentry;

#[inline]
pub unsafe fn rcu_dereference_check_fdtable(
    files: *const files_struct,
    fdtfd: *mut fdtable,
) -> *mut fdtable {
    rcu_dereference_check(fdtfd, lockdep_is_held(&(*files).file_lock))
}

#[inline]
pub unsafe fn files_fdtable(files: *const files_struct) -> *mut fdtable {
    rcu_dereference_check_fdtable(files, (*files).fdt)
}

/*
 * The caller must ensure that fd table isn't shared or hold rcu or file lock
 */
#[inline]
pub unsafe fn files_lookup_fd_raw(
    files: *mut files_struct,
    fd: ::core::ffi::c_uint,
) -> *mut file {
    let fdt: *mut fdtable = rcu_dereference_raw((*files).fdt);
    let mask: ::core::ffi::c_ulong = array_index_mask_nospec(fd, (*fdt).max_fds);
    let needs_masking: *mut file;

    /*
     * 'mask' is zero for an out-of-bounds fd, all ones for ok.
     * 'fd&mask' is 'fd' for ok, or 0 for out of bounds.
     *
     * Accessing fdt->fd[0] is ok, but needs masking of the result.
     */
    needs_masking = rcu_dereference_raw(*(*fdt).fd.add((fd as usize & mask as usize)));
    (mask & (needs_masking as ::core::ffi::c_ulong)) as *mut file
}

#[inline]
pub unsafe fn files_lookup_fd_locked(
    files: *mut files_struct,
    fd: ::core::ffi::c_uint,
) -> *mut file {
    RCU_LOCKDEP_WARN(
        !lockdep_is_held(&(*files).file_lock),
        "suspicious rcu_dereference_check() usage",
    );
    files_lookup_fd_raw(files, fd)
}

#[inline]
pub unsafe fn close_on_exec(fd: ::core::ffi::c_uint, files: *const files_struct) -> bool {
    test_bit(fd, (*files_fdtable(files)).close_on_exec)
}

pub struct task_struct;

extern "C" {
    pub fn put_files_struct(fs: *mut files_struct);
    pub fn unshare_files() -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct fd_range {
    pub from: ::core::ffi::c_uint,
    pub to: ::core::ffi::c_uint,
}

extern "C" {
    pub fn dup_fd(files: *mut files_struct, range: *mut fd_range) -> *mut files_struct;
    pub fn do_close_on_exec(files: *mut files_struct);
    pub fn iterate_fd(
        files: *mut files_struct,
        n: ::core::ffi::c_uint,
        f: Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *mut file, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
        p: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn close_fd(fd: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn file_close_fd(fd: ::core::ffi::c_uint) -> *mut file;
    pub static mut files_cachep: *mut kmem_cache;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
