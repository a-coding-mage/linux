// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and spufs headers are referenced
// here but intentionally not reimplemented.

use core::ffi::c_char;

extern "C" {
    static spufs_context_fops: file_operations;
    static THIS_MODULE: module;

    fn spufs_run_spu(ctx: *mut spufs_context, npc: *mut u32, status: *mut u32) -> c_long;
    fn spufs_create(
        path: *mut path,
        dentry: *mut dentry,
        flags: c_uint,
        mode: umode_t,
        neighbor: *mut file,
    ) -> c_int;
    fn do_notify_spus_active();
    fn spufs_coredump_extra_notes_size(cprm: *mut coredump_params) -> usize;
    fn spufs_coredump_extra_notes_write(
        cprm: *mut coredump_params,
    ) -> c_int;
    fn start_creating_user_path(
        dfd: c_int,
        pathname: *const c_char,
        path: *mut path,
        lookup_flags: c_uint,
    ) -> *mut dentry;
    fn end_creating_path(path: *mut path, dentry: *mut dentry);
}

// Opaque types and kernel scalar aliases are provided by the translated
// kernel headers.
#[allow(non_camel_case_types)]
type c_long = isize;
#[allow(non_camel_case_types)]
type c_uint = u32;
#[allow(non_camel_case_types)]
type c_int = i32;

unsafe extern "C" {
    fn get_user(value: *mut u32, ptr: *const u32) -> c_int;
    fn put_user(value: u32, ptr: *mut u32) -> c_int;
}

// The following declarations correspond to types supplied by spufs.h and
// the Linux headers.
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spufs_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct path {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    pub f_op: *const file_operations,
    _private: [u8; 0],
}
#[repr(C)]
pub struct coredump_params {
    _private: [u8; 0],
}
type umode_t = u16;

#[repr(C)]
pub struct spufs_inode_info {
    pub i_ctx: *mut spufs_context,
}

#[repr(C)]
pub struct spufs_calls {
    pub create: Option<unsafe extern "C" fn(*const c_char, c_uint, umode_t, *mut file) -> c_long>,
    pub spu_run: Option<unsafe extern "C" fn(*mut file, *mut u32, *mut u32) -> c_long>,
    pub notify_spus_active: Option<unsafe extern "C" fn()>,
    pub owner: *const module,
    #[cfg(CONFIG_COREDUMP)]
    pub coredump_extra_notes_size: Option<unsafe extern "C" fn(*mut coredump_params) -> usize>,
    #[cfg(CONFIG_COREDUMP)]
    pub coredump_extra_notes_write: Option<unsafe extern "C" fn(*mut coredump_params) -> c_int>,
}

const EFAULT: c_long = -14;
const EINVAL: c_long = -22;
const AT_FDCWD: c_int = -100;
const LOOKUP_DIRECTORY: c_uint = 0x0000_0002;

unsafe fn file_inode(filp: *mut file) -> *mut spufs_inode_info {
    filp as *mut spufs_inode_info
}

unsafe fn spufs_i(inode: *mut spufs_inode_info) -> *mut spufs_inode_info {
    inode
}

/**
 * sys_spu_run - run code loaded into an SPU
 *
 * @unpc:    next program counter for the SPU
 * @ustatus: status of the SPU
 *
 * This system call transfers the control of execution of a
 * user space thread to an SPU. It will return when the
 * SPU has finished executing or when it hits an error
 * condition and it will be interrupted if a signal needs
 * to be delivered to a handler in user space.
 *
 * The next program counter is set to the passed value
 * before the SPU starts fetching code and the user space
 * pointer gets updated with the new value when returning
 * from kernel space.
 *
 * The status value returned from spu_run reflects the
 * value of the spu_status register after the SPU has stopped.
 */
unsafe extern "C" fn do_spu_run(
    filp: *mut file,
    unpc: *mut u32,
    ustatus: *mut u32,
) -> c_long {
    let mut ret: c_long;
    let i: *mut spufs_inode_info;
    let mut npc: u32 = 0;
    let mut status: u32 = 0;

    ret = EFAULT;
    if get_user(&mut npc, unpc) != 0 {
        return ret;
    }

    /* check if this file was created by spu_create */
    ret = EINVAL;
    if (*filp).f_op != &spufs_context_fops {
        return ret;
    }

    i = spufs_i(file_inode(filp));
    ret = spufs_run_spu((*i).i_ctx, &mut npc, &mut status);

    if put_user(npc, unpc) != 0 {
        ret = EFAULT;
    }

    if !ustatus.is_null() && put_user(status, ustatus) != 0 {
        ret = EFAULT;
    }

    ret
}

unsafe extern "C" fn do_spu_create(
    pathname: *const c_char,
    flags: c_uint,
    mode: umode_t,
    neighbor: *mut file,
) -> c_long {
    let mut path = path { _private: [] };
    let mut dentry: *mut dentry;
    let mut ret: c_int;

    dentry = start_creating_user_path(AT_FDCWD, pathname, &mut path, LOOKUP_DIRECTORY);
    ret = dentry as c_int;
    if !dentry.is_null() {
        ret = spufs_create(&mut path, dentry, flags, mode, neighbor);
        end_creating_path(&mut path, dentry);
    }

    ret as c_long
}

#[no_mangle]
pub static mut spufs_calls: spufs_calls = spufs_calls {
    create: Some(do_spu_create),
    spu_run: Some(do_spu_run),
    notify_spus_active: Some(do_notify_spus_active),
    owner: unsafe { &THIS_MODULE },
    #[cfg(CONFIG_COREDUMP)]
    coredump_extra_notes_size: Some(spufs_coredump_extra_notes_size),
    #[cfg(CONFIG_COREDUMP)]
    coredump_extra_notes_write: Some(spufs_coredump_extra_notes_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
