/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// linux/sched.h, linux/path.h, linux/spinlock.h, linux/seqlock.h,
// and linux/vfsdebug.h.

#[repr(C)]
pub struct fs_struct {
    pub users: ::core::ffi::c_int,
    pub seq: seqlock_t,
    pub umask: ::core::ffi::c_int,
    pub in_exec: ::core::ffi::c_int,
    pub root: path,
    pub pwd: path,
}

unsafe extern "C" {
    pub static mut fs_cachep: *mut kmem_cache;
    pub static mut userspace_init_fs: *mut fs_struct;

    pub fn exit_fs(task: *mut task_struct);
    pub fn set_fs_root(fs: *mut fs_struct, path: *const path);
    pub fn set_fs_pwd(fs: *mut fs_struct, path: *const path);
    pub fn copy_fs_struct(fs: *mut fs_struct) -> *mut fs_struct;
    pub fn free_fs_struct(fs: *mut fs_struct);
    pub fn unshare_fs_struct() -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn get_fs_root(fs: *mut fs_struct, root: *mut path) {
    read_seqlock_excl(&mut (*fs).seq);
    *root = (*fs).root;
    path_get(root);
    read_sequnlock_excl(&mut (*fs).seq);
}

#[inline]
pub unsafe fn get_fs_pwd(fs: *mut fs_struct, pwd: *mut path) {
    read_seqlock_excl(&mut (*fs).seq);
    *pwd = (*fs).pwd;
    path_get(pwd);
    read_sequnlock_excl(&mut (*fs).seq);
}

unsafe extern "C" {
    pub fn switch_fs_struct(new_fs: *mut fs_struct) -> *mut fs_struct;
    pub fn current_chrooted() -> bool;
    pub fn init_userspace_fs();
}

#[inline]
pub unsafe fn current_umask() -> ::core::ffi::c_int {
    (*current).fs.as_ref().unwrap().umask
}

/*
 * Temporarily use userspace_init_fs for path resolution in kthreads.
 * Callers should use scoped_with_init_fs() which automatically
 * restores the original fs_struct at scope exit.
 */
#[inline]
pub unsafe fn __override_init_fs() -> *mut fs_struct {
    let old_fs = (*current).fs;
    WRITE_ONCE(&mut (*current).fs, userspace_init_fs);
    old_fs
}

#[inline]
pub unsafe fn __revert_init_fs(old_fs: *mut fs_struct) {
    VFS_WARN_ON_ONCE((*current).fs != userspace_init_fs);
    WRITE_ONCE(&mut (*current).fs, old_fs);
}

// C DEFINE_CLASS(__override_init_fs, struct fs_struct *,
//                __revert_init_fs(_T), __override_init_fs(), void)

// C macro: scoped_with_init_fs() expands to
// scoped_class(__override_init_fs, __UNIQUE_ID(label)).


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
