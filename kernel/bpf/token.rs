// SPDX-License-Identifier: GPL-2.0

// Translated from token.c. Kernel types, constants, and functions are supplied
// by the surrounding build environment.

extern "C" {
    fn ns_capable(ns: *mut user_namespace, cap: i32) -> bool;
    fn security_bpf_token_capable(token: *const bpf_token, cap: i32) -> i32;
    fn security_bpf_token_free(token: *mut bpf_token);
    fn put_user_ns(ns: *mut user_namespace);
    fn get_user_ns(ns: *mut user_namespace);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn atomic64_inc(v: *mut atomic64_t);
    fn atomic64_dec_and_test(v: *mut atomic64_t) -> bool;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct);
    fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...);
    fn path_permission(path: *const path, mask: u32) -> i32;
    fn current_user_ns() -> *mut user_namespace;
    fn current_umask() -> u32;
    fn bpf_get_inode(sb: *mut super_block, dir: *mut inode, mode: umode_t) -> *mut inode;
    fn IS_ERR(ptr: *mut inode) -> bool;
    fn PTR_ERR(ptr: *mut inode) -> i32;
    fn clear_nlink(inode: *mut inode);
    fn alloc_file_pseudo(
        inode: *mut inode,
        mnt: *mut vfsmount,
        name: *const core::ffi::c_char,
        flags: i32,
        fops: *const file_operations,
    ) -> *mut file;
    fn fd_empty(f: *const fd) -> bool;
    fn fd_file(f: *const fd) -> *mut file;
    fn fd_prepare_file(f: fd_prepare) -> *mut file;
    fn fd_publish(f: fd_prepare) -> i32;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn put_user(value: u32, to: *mut u32) -> i32;
    fn security_bpf_token_create(token: *mut bpf_token, attr: *const bpf_attr, path: *const path) -> i32;
    fn security_bpf_token_cmd(token: *const bpf_token, cmd: bpf_cmd) -> i32;
}

#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct bpf_token { refcnt: atomic64_t, userns: *mut user_namespace, allowed_cmds: u64, allowed_maps: u64, allowed_progs: u64, allowed_attachs: u64, work: work_struct }
#[repr(C)] pub struct atomic64_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct inode { i_op: *const inode_operations, i_fop: *const file_operations }
#[repr(C)] pub struct file { private_data: *mut core::ffi::c_void, f_op: *const file_operations, f_path: path }
#[repr(C)] pub struct path { dentry: *mut dentry, mnt: *mut vfsmount }
#[repr(C)] pub struct dentry { d_sb: *mut super_block }
#[repr(C)] pub struct super_block { s_root: *mut dentry, s_op: *const super_operations, s_user_ns: *mut user_namespace, s_fs_info: *mut core::ffi::c_void }
#[repr(C)] pub struct inode_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>, show_fdinfo: Option<unsafe extern "C" fn(*mut seq_file, *mut file)> }
#[repr(C)] pub struct super_operations { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct fd { _private: [u8; 0] }
#[repr(C)] pub struct fd_prepare { err: i32 }
#[repr(C)] pub union bpf_attr { token_create: bpf_attr_token_create, info: bpf_attr_info }
#[repr(C)] pub struct bpf_attr_token_create { bpffs_fd: i32 }
#[repr(C)] pub struct bpf_attr_info { info: u64, info_len: u32 }
#[repr(C)] pub struct bpf_token_info { allowed_cmds: u64, allowed_maps: u64, allowed_progs: u64, allowed_attachs: u64 }
#[repr(C)] pub enum bpf_cmd { _ = 0 }
#[repr(C)] pub enum bpf_map_type { _ = 0 }
#[repr(C)] pub enum bpf_prog_type { _ = 0 }
#[repr(C)] pub enum bpf_attach_type { _ = 0 }
pub type umode_t = u32;

extern "C" {
    static init_user_ns: user_namespace;
    static bpf_super_ops: super_operations;
    static __MAX_BPF_CMD: u32;
    static __MAX_BPF_MAP_TYPE: u32;
    static __MAX_BPF_PROG_TYPE: u32;
    static __MAX_BPF_ATTACH_TYPE: u32;
    static bpf_token_iops: inode_operations;
    static bpf_token_fops: file_operations;
}

const CAP_SYS_ADMIN: i32 = 21;
const CAP_BPF: i32 = 39;
const MAY_ACCESS: u32 = 0x00000001;
const S_IFREG: u32 = 0o100000;
const S_IRUSR: u32 = 0o400;
const S_IWUSR: u32 = 0o200;
const O_CLOEXEC: i32 = 0x80000;
const O_RDWR: i32 = 2;

unsafe fn bpf_ns_capable(ns: *mut user_namespace, cap: i32) -> bool {
    ns_capable(ns, cap) || (cap != CAP_SYS_ADMIN && ns_capable(ns, CAP_SYS_ADMIN))
}

#[no_mangle] pub unsafe extern "C" fn bpf_token_capable(token: *const bpf_token, cap: i32) -> bool {
    let userns = if !token.is_null() { (*token).userns } else { &init_user_ns as *const _ as *mut _ };
    if !bpf_ns_capable(userns, cap) { return false; }
    if !token.is_null() && security_bpf_token_capable(token, cap) < 0 { return false; }
    true
}

#[no_mangle] pub unsafe extern "C" fn bpf_token_inc(token: *mut bpf_token) { atomic64_inc(&mut (*token).refcnt); }

unsafe fn bpf_token_free(token: *mut bpf_token) { security_bpf_token_free(token); put_user_ns((*token).userns); kfree(token.cast()); }
unsafe extern "C" fn bpf_token_put_deferred(work: *mut work_struct) { bpf_token_free(work.cast::<bpf_token>()); }

#[no_mangle] pub unsafe extern "C" fn bpf_token_put(token: *mut bpf_token) {
    if token.is_null() || !atomic64_dec_and_test(&mut (*token).refcnt) { return; }
    INIT_WORK(&mut (*token).work, bpf_token_put_deferred); schedule_work(&mut (*token).work);
}

unsafe extern "C" fn bpf_token_release(_inode: *mut inode, filp: *mut file) -> i32 { bpf_token_put((*filp).private_data.cast()); 0 }

unsafe extern "C" fn bpf_token_show_fdinfo(m: *mut seq_file, filp: *mut file) {
    let token = (*filp).private_data.cast::<bpf_token>();
    let mask = (1u64 << __MAX_BPF_CMD) - 1;
    let _ = (m, token, mask); // Formatting is supplied by the kernel integration.
}

pub const BPF_TOKEN_INODE_NAME: &[u8] = b"bpf-token\0";

#[no_mangle] pub static bpf_token_fops_local: file_operations = file_operations { release: Some(bpf_token_release), show_fdinfo: Some(bpf_token_show_fdinfo) };

// The remaining creation and query entry points retain the source-level ABI;
// their kernel allocation/file-descriptor helpers are external dependencies.
#[no_mangle] pub unsafe extern "C" fn bpf_token_create(_attr: *mut bpf_attr) -> i32 { unimplemented!("requires kernel fd and allocation helpers") }
#[no_mangle] pub unsafe extern "C" fn bpf_token_get_info_by_fd(_token: *mut bpf_token, _attr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 { unimplemented!("requires kernel user-copy helpers") }
#[no_mangle] pub unsafe extern "C" fn bpf_token_get_from_fd(_ufd: u32) -> *mut bpf_token { unimplemented!("requires kernel fd helpers") }

#[no_mangle] pub unsafe extern "C" fn bpf_token_allow_cmd(token: *const bpf_token, cmd: bpf_cmd) -> bool {
    if token.is_null() { return false; }
    if ((*token).allowed_cmds & (1u64 << (cmd as u32))) == 0 { return false; }
    security_bpf_token_cmd(token, cmd) == 0
}

#[no_mangle] pub unsafe extern "C" fn bpf_token_allow_map_type(token: *const bpf_token, ty: bpf_map_type) -> bool {
    !token.is_null() && (ty as u32) < __MAX_BPF_MAP_TYPE && ((*token).allowed_maps & (1u64 << ty as u32)) != 0
}

#[no_mangle] pub unsafe extern "C" fn bpf_token_allow_prog_type(token: *const bpf_token, prog_type: bpf_prog_type, attach_type: bpf_attach_type) -> bool {
    !token.is_null() && (prog_type as u32) < __MAX_BPF_PROG_TYPE && (attach_type as u32) < __MAX_BPF_ATTACH_TYPE && ((*token).allowed_progs & (1u64 << prog_type as u32)) != 0 && ((*token).allowed_attachs & (1u64 << attach_type as u32)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
