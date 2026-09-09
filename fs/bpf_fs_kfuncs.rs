// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Google LLC. */

// Linux kernel dependencies are supplied by the surrounding translation unit.

extern "C" {
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn fput(file: *mut file);
    fn d_path(path: *const path, buf: *mut c_char, size: usize) -> *mut c_char;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn __bpf_dynptr_size(value: *const bpf_dynptr_kern) -> u32;
    fn __bpf_dynptr_data_rw(value: *mut bpf_dynptr_kern, size: u32) -> *mut c_void;
    fn __bpf_dynptr_data(value: *const bpf_dynptr_kern, size: u32) -> *const c_void;
    fn __vfs_getxattr(dentry: *mut dentry, inode: *mut inode, name: *const c_char,
                      value: *mut c_void, size: u32) -> c_int;
    fn file_dentry(file: *mut file) -> *mut dentry;
    fn inode_permission(idmap: *const mnt_idmap, inode: *mut inode, mask: c_int) -> c_int;
    fn __vfs_setxattr(idmap: *const mnt_idmap, dentry: *mut dentry, inode: *mut inode,
                      name: *const c_char, value: *const c_void, size: u32, flags: c_int) -> c_int;
    fn fsnotify_xattr(dentry: *mut dentry);
    fn __vfs_removexattr(idmap: *const mnt_idmap, dentry: *mut dentry, name: *const c_char) -> c_int;
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn kernfs_xattr_get(kn: *mut kernfs_node, name: *const c_char, value: *mut c_void, size: u32) -> c_int;
    fn sock_read_xattr(sock: *mut socket, name: *const c_char, value: *mut c_void, size: u32) -> c_int;
    fn d_real_inode(dentry: *mut dentry) -> *mut inode;
    fn bpf_prog_is_binfmt_misc_ops(prog: *const bpf_prog) -> bool;
    fn btf_id_set8_contains(set: *const c_void, id: u32) -> bool;
    fn btf_id_set_contains(set: *const c_void, id: u32) -> bool;
}

extern "C" {
    static XATTR_NAME_BPF_LSM: *const c_char;
    static XATTR_USER_PREFIX: *const c_char;
    static XATTR_NAME_BPF_LSM_LEN: usize;
    static XATTR_USER_PREFIX_LEN: usize;
    static MAY_READ: c_int;
    static MAY_WRITE: c_int;
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct bpf_dynptr { _private: [u8; 0] }
#[repr(C)] pub struct bpf_dynptr_kern { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { pub kn: *mut kernfs_node }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct bpf_prog { pub type_: u32, pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub st_ops: *mut c_void }
type c_char = i8; type c_int = i32; type c_void = core::ffi::c_void;

extern "C" {
    static nop_mnt_idmap: mnt_idmap;
}

pub unsafe fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file { get_task_exe_file(task) }
pub unsafe fn bpf_put_file(file: *mut file) { fput(file); }

pub unsafe fn bpf_path_d_path(p: *const path, buf: *mut c_char, buf__sz: usize) -> c_int {
    if buf__sz == 0 { return -22; }
    let ret = d_path(p, buf, buf__sz);
    // ERR_PTR/PTR_ERR are kernel pointer-error conventions.
    if (ret as usize) >= (usize::MAX - 4095) { return ret as c_int; }
    let len = buf.add(buf__sz).offset_from(ret) as c_int;
    core::ptr::copy(ret, buf, len as usize);
    len
}

unsafe fn match_security_bpf_prefix(name__str: *const c_char) -> bool {
    // strncmp(name__str, XATTR_NAME_BPF_LSM, XATTR_NAME_BPF_LSM_LEN)
    !strncmp(name__str, XATTR_NAME_BPF_LSM, XATTR_NAME_BPF_LSM_LEN)
}

unsafe fn bpf_xattr_read_permission(name: *const c_char, inode: *mut inode) -> c_int {
    if inode.is_null() { return -22; }
    if strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) != 0 && !match_security_bpf_prefix(name) { return -1; }
    inode_permission(&nop_mnt_idmap, inode, MAY_READ)
}

pub unsafe fn bpf_get_dentry_xattr(dentry: *mut dentry, name: *const c_char, value_p: *mut bpf_dynptr) -> c_int {
    let value_ptr = value_p as *mut bpf_dynptr_kern;
    let inode = d_inode(dentry);
    let value_len = __bpf_dynptr_size(value_ptr);
    let value = __bpf_dynptr_data_rw(value_ptr, value_len);
    if value.is_null() { return -22; }
    let ret = bpf_xattr_read_permission(name, inode);
    if ret != 0 { return ret; }
    __vfs_getxattr(dentry, inode, name, value, value_len)
}

pub unsafe fn bpf_get_file_xattr(file: *mut file, name: *const c_char, value_p: *mut bpf_dynptr) -> c_int {
    bpf_get_dentry_xattr(file_dentry(file), name, value_p)
}

unsafe fn bpf_xattr_write_permission(name: *const c_char, inode: *mut inode) -> c_int {
    if inode.is_null() { return -22; }
    if !match_security_bpf_prefix(name) { return -1; }
    inode_permission(&nop_mnt_idmap, inode, MAY_WRITE)
}

pub unsafe fn bpf_set_dentry_xattr_locked(dentry: *mut dentry, name: *const c_char, value_p: *const bpf_dynptr, flags: c_int) -> c_int {
    let value_ptr = value_p as *const bpf_dynptr_kern;
    let inode = d_inode(dentry);
    let value_len = __bpf_dynptr_size(value_ptr);
    let value = __bpf_dynptr_data(value_ptr, value_len);
    if value.is_null() { return -22; }
    let ret = bpf_xattr_write_permission(name, inode);
    if ret != 0 { return ret; }
    let ret = __vfs_setxattr(&nop_mnt_idmap, dentry, inode, name, value, value_len, flags);
    if ret == 0 { fsnotify_xattr(dentry); }
    ret
}

pub unsafe fn bpf_remove_dentry_xattr_locked(dentry: *mut dentry, name: *const c_char) -> c_int {
    let inode = d_inode(dentry);
    let ret = bpf_xattr_write_permission(name, inode);
    if ret != 0 { return ret; }
    let ret = __vfs_removexattr(&nop_mnt_idmap, dentry, name);
    if ret == 0 { fsnotify_xattr(dentry); }
    ret
}

pub unsafe fn bpf_set_dentry_xattr(dentry: *mut dentry, name: *const c_char, value_p: *const bpf_dynptr, flags: c_int) -> c_int {
    let inode = d_inode(dentry); if inode.is_null() { return -22; }
    inode_lock(inode); let ret = bpf_set_dentry_xattr_locked(dentry, name, value_p, flags); inode_unlock(inode); ret
}
pub unsafe fn bpf_remove_dentry_xattr(dentry: *mut dentry, name: *const c_char) -> c_int {
    let inode = d_inode(dentry); if inode.is_null() { return -22; }
    inode_lock(inode); let ret = bpf_remove_dentry_xattr_locked(dentry, name); inode_unlock(inode); ret
}

#[cfg(feature = "CONFIG_CGROUPS")]
pub unsafe fn bpf_cgroup_read_xattr(cgroup: *mut cgroup, name: *const c_char, value_p: *mut bpf_dynptr) -> c_int {
    if strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) != 0 { return -1; }
    let ptr = value_p as *mut bpf_dynptr_kern; let len = __bpf_dynptr_size(ptr); let value = __bpf_dynptr_data_rw(ptr, len);
    if value.is_null() { return -22; } kernfs_xattr_get((*cgroup).kn, name, value, len)
}

#[cfg(feature = "CONFIG_NET")]
pub unsafe fn bpf_sock_read_xattr(sock: *mut socket, name: *const c_char, value_p: *mut bpf_dynptr) -> c_int {
    if strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) != 0 { return -1; }
    let ptr = value_p as *mut bpf_dynptr_kern; let len = __bpf_dynptr_size(ptr); let value = __bpf_dynptr_data_rw(ptr, len);
    if value.is_null() { return -22; } sock_read_xattr(sock, name, value, len)
}

pub unsafe fn bpf_real_data_inode(file: *mut file) -> *mut inode { d_real_inode(file_dentry(file)) }

pub const BPF_PROG_TYPE_LSM: u32 = 29;
pub const BPF_PROG_TYPE_STRUCT_OPS: u32 = 26;
pub const EACCES: c_int = 13;

// BTF_KFUNCS_START(bpf_fs_kfunc_set_ids)
// BTF_ID_FLAGS entries: bpf_get_task_exe_file (KF_ACQUIRE | KF_RET_NULL),
// bpf_put_file (KF_RELEASE), bpf_path_d_path, bpf_get_dentry_xattr (KF_SLEEPABLE),
// bpf_get_file_xattr (KF_SLEEPABLE), bpf_set_dentry_xattr (KF_SLEEPABLE),
// bpf_remove_dentry_xattr (KF_SLEEPABLE), bpf_real_data_inode (KF_SLEEPABLE | KF_RET_NULL),
// and, under CONFIG_NET, bpf_sock_read_xattr (KF_RCU).
// BTF_KFUNCS_END(bpf_fs_kfunc_set_ids)
// BTF_SET_START(bpf_fs_kfunc_lsm_only_ids): bpf_set_dentry_xattr, bpf_remove_dentry_xattr.

pub unsafe fn bpf_fs_kfuncs_filter(prog: *const bpf_prog, kfunc_id: u32) -> c_int {
    if !btf_id_set8_contains(core::ptr::null(), kfunc_id) { return 0; }
    if (*prog).type_ == BPF_PROG_TYPE_LSM { return 0; }
    if (*prog).type_ != BPF_PROG_TYPE_STRUCT_OPS { return -EACCES; }
    // ->st_ops is unset during the cfg pass; enforced once it is set.
    if (*prog).aux.is_null() || (*(*prog).aux).st_ops.is_null() { return 0; }
    if bpf_prog_is_binfmt_misc_ops(prog) && !btf_id_set_contains(core::ptr::null(), kfunc_id) { return 0; }
    -EACCES
}

// bpf_[set|remove]_dentry_xattr.* hooks have KF_SLEEPABLE and are available
// only to sleepable hooks with dentry arguments. Hooks listed below already
// hold the d_inode lock and use the locked variants.
// BTF_SET_START(d_inode_locked_hooks)
// bpf_lsm_inode_post_removexattr, bpf_lsm_inode_post_setattr,
// bpf_lsm_inode_post_setxattr, bpf_lsm_inode_removexattr, bpf_lsm_inode_rmdir,
// bpf_lsm_inode_setattr, bpf_lsm_inode_setxattr, bpf_lsm_inode_unlink.
// Under CONFIG_SECURITY_PATH: bpf_lsm_path_unlink, bpf_lsm_path_rmdir.
// BTF_SET_END(d_inode_locked_hooks)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
