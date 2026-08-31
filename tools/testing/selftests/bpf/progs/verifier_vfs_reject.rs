// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC. */

// C dependencies: vmlinux.h, errno.h, bpf/bpf_helpers.h,
// bpf/bpf_tracing.h, linux/limits.h, bpf_misc.h, bpf_experimental.h.

const PATH_MAX: usize = 4096;
const EACCES: i32 = 13;

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fs_struct {
    pub root: path,
    pub pwd: path,
}

#[repr(C)]
pub struct task_struct {
    pub parent: *mut task_struct,
    pub fs: *mut fs_struct,
}

#[repr(C)]
pub struct callback_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_task_work: callback_head,
    pub f_path: path,
}

pub type ino_t = u64;

#[repr(C)]
pub struct inode {
    pub i_ino: ino_t,
}

#[repr(C)]
pub struct dentry {
    pub d_inode: *mut inode,
}

unsafe extern "C" {
    fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn bpf_put_file(file: *mut file);
    fn bpf_path_d_path(path: *mut path, buf: *mut i8, sz: u32) -> i32;
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

static mut buf: [i8; PATH_MAX] = [0; PATH_MAX];

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("Possibly NULL pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file_kfunc_null() -> i32 {
    let acquired: *mut file;

    /* Can't pass a NULL pointer to bpf_get_task_exe_file(). */
    acquired = unsafe { bpf_get_task_exe_file(core::ptr::null_mut()) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_put_file(acquired) };
    0
}

#[unsafe(link_section = "lsm.s/inode_getxattr")]
// __failure
// __msg("R1 is fp expected STRUCT task_struct")
#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file_kfunc_fp() -> i32 {
    let mut x = core::mem::MaybeUninit::<u64>::uninit();
    let acquired: *mut file;
    let task: *mut task_struct;

    task = x.as_mut_ptr() as *mut task_struct;
    /* Can't pass random frame pointer to bpf_get_task_exe_file(). */
    acquired = unsafe { bpf_get_task_exe_file(task) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_put_file(acquired) };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("R1 must be referenced or trusted")
#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file_kfunc_untrusted() -> i32 {
    let acquired: *mut file;
    let parent: *mut task_struct;

    /* Walking a trusted struct task_struct returned from
     * bpf_get_current_task_btf() yields an untrusted pointer.
     */
    parent = unsafe { (*bpf_get_current_task_btf()).parent };
    /* Can't pass untrusted pointer to bpf_get_task_exe_file(). */
    acquired = unsafe { bpf_get_task_exe_file(parent) };
    if acquired.is_null() {
        return 0;
    }

    unsafe { bpf_put_file(acquired) };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn get_task_exe_file_kfunc_unreleased() -> i32 {
    let acquired: *mut file;

    acquired = unsafe { bpf_get_task_exe_file(bpf_get_current_task_btf()) };
    if acquired.is_null() {
        return 0;
    }

    /* Acquired but never released. */
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("release kfunc bpf_put_file expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn put_file_kfunc_unacquired(file: *mut file) -> i32 {
    /* Can't release an unacquired pointer. */
    unsafe { bpf_put_file(file) };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("Possibly NULL pointer passed to trusted R1")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_null() -> i32 {
    /* Can't pass NULL value to bpf_path_d_path() kfunc. */
    unsafe {
        bpf_path_d_path(
            core::ptr::null_mut(),
            core::ptr::addr_of_mut!(buf) as *mut i8,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    0
}

#[unsafe(link_section = "lsm.s/task_alloc")]
// __failure
// __msg("dereference of modified untrusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_untrusted_from_argument(
    task: *mut task_struct,
) -> i32 {
    let root: *mut path;

    /* Walking a trusted argument typically yields an untrusted
     * pointer. This is one example of that.
     */
    root = unsafe { core::ptr::addr_of_mut!((*(*task).fs).root) };
    unsafe {
        bpf_path_d_path(
            root,
            core::ptr::addr_of_mut!(buf) as *mut i8,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("dereference of modified untrusted_ptr_")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_untrusted_from_current() -> i32 {
    let pwd: *mut path;
    let current: *mut task_struct;

    current = unsafe { bpf_get_current_task_btf() };
    /* Walking a trusted pointer returned from bpf_get_current_task_btf()
     * yields an untrusted pointer.
     */
    pwd = unsafe { core::ptr::addr_of_mut!((*(*current).fs).pwd) };
    unsafe {
        bpf_path_d_path(
            pwd,
            core::ptr::addr_of_mut!(buf) as *mut i8,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("kernel function bpf_path_d_path R1 expected pointer to STRUCT path but R1 has a pointer to STRUCT file")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_type_mismatch(file: *mut file) -> i32 {
    unsafe {
        bpf_path_d_path(
            core::ptr::addr_of_mut!((*file).f_task_work) as *mut path,
            core::ptr::addr_of_mut!(buf) as *mut i8,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    0
}

#[unsafe(link_section = "lsm.s/file_open")]
// __failure
// __msg("invalid access to map value, value_size=4096 off=0 size=8192")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_invalid_buf_sz(file: *mut file) -> i32 {
    /* bpf_path_d_path() enforces a constraint on the buffer size supplied
     * by the BPF LSM program via the __sz annotation. buf here is set to
     * PATH_MAX, so let's ensure that the BPF verifier rejects BPF_PROG_LOAD
     * attempts if the supplied size and the actual size of the buffer
     * mismatches.
     */
    unsafe {
        bpf_path_d_path(
            core::ptr::addr_of_mut!((*file).f_path),
            core::ptr::addr_of_mut!(buf) as *mut i8,
            (PATH_MAX * 2) as u32,
        )
    };
    0
}

#[unsafe(link_section = "fentry/vfs_open")]
// __failure
// __msg("calling kernel function bpf_path_d_path is not allowed")
#[no_mangle]
pub unsafe extern "C" fn path_d_path_kfunc_non_lsm(path: *mut path, _f: *mut file) -> i32 {
    /* Calling bpf_path_d_path() from a non-LSM BPF program isn't permitted.
     */
    unsafe {
        bpf_path_d_path(
            path,
            core::ptr::addr_of_mut!(buf) as *mut i8,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    0
}

#[unsafe(link_section = "lsm.s/inode_rename")]
// __failure
// __msg("invalid mem access 'trusted_ptr_or_null_'")
#[no_mangle]
pub unsafe extern "C" fn inode_rename(
    _old_dir: *mut inode,
    _old_dentry: *mut dentry,
    _new_dir: *mut inode,
    new_dentry: *mut dentry,
    _flags: ::core::ffi::c_uint,
) -> i32 {
    let inode: *mut inode = unsafe { (*new_dentry).d_inode };
    let ino: ino_t;

    ino = unsafe { (*inode).i_ino };
    if ino == 0 {
        return -EACCES;
    }
    0
}

#[unsafe(link_section = "license")]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
