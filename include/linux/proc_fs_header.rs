/* SPDX-License-Identifier: GPL-2.0 */
/* The proc filesystem constants/structures. */

// Dependencies supplied by other translated headers are intentionally external.

pub struct proc_dir_entry;
pub struct seq_file;
pub struct seq_operations;

pub const PROC_ENTRY_PERMANENT: u32 = if cfg!(feature = "module") { 0 } else { 1u32 << 0 };
pub const PROC_ENTRY_proc_read_iter: u32 = 1u32 << 1;
pub const PROC_ENTRY_proc_compat_ioctl: u32 = 1u32 << 2;
pub const PROC_ENTRY_proc_lseek: u32 = 1u32 << 3;
pub const PROC_ENTRY_FORCE_LOOKUP: u32 = 1u32 << 7;

#[repr(C)]
pub struct proc_ops {
    pub proc_flags: core::ffi::c_uint,
    pub proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> core::ffi::c_int>,
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut core::ffi::c_char, usize, *mut loff_t) -> ssize_t>,
    pub proc_read_iter: Option<unsafe extern "C" fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
    pub proc_write: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> ssize_t>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, loff_t, core::ffi::c_int) -> loff_t>,
    pub proc_release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> core::ffi::c_int>,
    pub proc_poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table_struct) -> __poll_t>,
    pub proc_ioctl: Option<unsafe extern "C" fn(*mut file, core::ffi::c_uint, core::ffi::c_ulong) -> core::ffi::c_long>,
    #[cfg(feature = "compat")]
    pub proc_compat_ioctl: Option<unsafe extern "C" fn(*mut file, core::ffi::c_uint, core::ffi::c_ulong) -> core::ffi::c_long>,
    pub proc_mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> core::ffi::c_int>,
    pub proc_get_unmapped_area: Option<unsafe extern "C" fn(*mut file, core::ffi::c_ulong, core::ffi::c_ulong, core::ffi::c_ulong, core::ffi::c_ulong) -> core::ffi::c_ulong>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum proc_hidepid {
    HIDEPID_OFF = 0,
    HIDEPID_NO_ACCESS = 1,
    HIDEPID_INVISIBLE = 2,
    HIDEPID_NOT_PTRACEABLE = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum proc_pidonly {
    PROC_PIDONLY_OFF = 0,
    PROC_PIDONLY_ON = 1,
}

#[repr(C)]
pub struct proc_fs_info {
    pub pid_ns: *mut pid_namespace,
    pub pid_gid: kgid_t,
    pub mounter_cred: *const cred,
    pub hide_pid: proc_hidepid,
    pub pidonly: proc_pidonly,
    pub rcu: rcu_head,
}

#[inline]
pub unsafe fn proc_sb_info(sb: *mut super_block) -> *mut proc_fs_info {
    (*sb).s_fs_info as *mut proc_fs_info
}

pub type proc_write_t = unsafe extern "C" fn(*mut file, *mut core::ffi::c_char, usize) -> core::ffi::c_int;

#[cfg(feature = "proc_fs")]
extern "C" {
    pub fn proc_root_init();
    pub fn proc_flush_pid(pid: *mut pid);
    pub fn proc_symlink(name: *const core::ffi::c_char, parent: *mut proc_dir_entry, dest: *const core::ffi::c_char) -> *mut proc_dir_entry;
    pub fn _proc_mkdir(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, data: *mut core::ffi::c_void, force_lookup: bool) -> *mut proc_dir_entry;
    pub fn proc_mkdir(name: *const core::ffi::c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    pub fn proc_mkdir_data(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_mkdir_mode(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    pub fn proc_create_mount_point(name: *const core::ffi::c_char) -> *mut proc_dir_entry;
    pub fn proc_create_seq_private(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, ops: *const seq_operations, state_size: core::ffi::c_uint, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create_single_data(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> core::ffi::c_int>, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create_data(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, ops: *const proc_ops, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, ops: *const proc_ops) -> *mut proc_dir_entry;
    pub fn proc_set_size(de: *mut proc_dir_entry, size: loff_t);
    pub fn proc_set_user(de: *mut proc_dir_entry, uid: kuid_t, gid: kgid_t);
    pub fn proc_get_parent_data(inode: *const inode) -> *mut core::ffi::c_void;
    pub fn proc_remove(de: *mut proc_dir_entry);
    pub fn remove_proc_entry(name: *const core::ffi::c_char, parent: *mut proc_dir_entry);
    pub fn remove_proc_subtree(name: *const core::ffi::c_char, parent: *mut proc_dir_entry) -> core::ffi::c_int;
    pub fn proc_create_net_data(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, ops: *const seq_operations, state_size: core::ffi::c_uint, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create_net_single(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> core::ffi::c_int>, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create_net_data_write(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, ops: *const seq_operations, write: proc_write_t, state_size: core::ffi::c_uint, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn proc_create_net_single_write(name: *const core::ffi::c_char, mode: umode_t, parent: *mut proc_dir_entry, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> core::ffi::c_int>, write: proc_write_t, data: *mut core::ffi::c_void) -> *mut proc_dir_entry;
    pub fn tgid_pidfd_to_pid(file: *const file) -> *mut pid;
    pub fn bpf_iter_init_seq_net(priv_data: *mut core::ffi::c_void, aux: *mut bpf_iter_aux_info) -> core::ffi::c_int;
    pub fn bpf_iter_fini_seq_net(priv_data: *mut core::ffi::c_void);
    pub fn arch_report_meminfo(m: *mut seq_file);
    pub fn arch_proc_pid_thread_features(m: *mut seq_file, task: *mut task_struct);
}

#[inline]
pub unsafe fn pde_data(inode: *const inode) -> *mut core::ffi::c_void { (*inode).i_private }

pub struct bpf_iter_aux_info;

#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_root_init() {}
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_flush_pid(_pid: *mut pid) {}

#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_symlink(_name: *const core::ffi::c_char, _parent: *mut proc_dir_entry, _dest: *const core::ffi::c_char) -> *mut proc_dir_entry { core::ptr::null_mut() }

#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_mkdir(_name: *const core::ffi::c_char, _parent: *mut proc_dir_entry) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_create_mount_point(_name: *const core::ffi::c_char) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn _proc_mkdir(_name: *const core::ffi::c_char, _mode: umode_t, _parent: *mut proc_dir_entry, _data: *mut core::ffi::c_void, _force_lookup: bool) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_mkdir_data(_name: *const core::ffi::c_char, _mode: umode_t, _parent: *mut proc_dir_entry, _data: *mut core::ffi::c_void) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_mkdir_mode(_name: *const core::ffi::c_char, _mode: umode_t, _parent: *mut proc_dir_entry) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_create(_name: *const core::ffi::c_char, _mode: umode_t, _parent: *mut proc_dir_entry, _ops: *const proc_ops) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_create_data(_name: *const core::ffi::c_char, _mode: umode_t, _parent: *mut proc_dir_entry, _ops: *const proc_ops, _data: *mut core::ffi::c_void) -> *mut proc_dir_entry { core::ptr::null_mut() }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_set_size(_de: *mut proc_dir_entry, _size: loff_t) {}
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_set_user(_de: *mut proc_dir_entry, _uid: kuid_t, _gid: kgid_t) {}
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_remove(_de: *mut proc_dir_entry) {}
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn remove_proc_entry(_name: *const core::ffi::c_char, _parent: *mut proc_dir_entry) {}
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn remove_proc_subtree(_name: *const core::ffi::c_char, _parent: *mut proc_dir_entry) -> core::ffi::c_int { 0 }
#[cfg(not(feature = "proc_fs"))]
pub unsafe fn proc_get_parent_data(_inode: *const inode) -> *mut core::ffi::c_void { core::ptr::null_mut() }

#[cfg(all(feature = "proc_fs", feature = "proc_pid_arch_status"))]
extern "C" { pub fn proc_pid_arch_status(m: *mut seq_file, ns: *mut pid_namespace, pid: *mut pid, task: *mut task_struct) -> core::ffi::c_int; }

pub struct net;
pub struct ns_common;

pub unsafe extern "C" fn proc_net_mkdir(net: *mut net, name: *const core::ffi::c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry {
    _proc_mkdir(name, 0, parent, net.cast(), true)
}

extern "C" {
    pub fn open_related_ns(ns: *mut ns_common, get_ns: Option<unsafe extern "C" fn(*mut ns_common) -> *mut ns_common>) -> core::ffi::c_int;
    pub fn proc_ns_file(file: *const file) -> bool;
}

#[inline]
pub unsafe fn proc_pid_ns(sb: *mut super_block) -> *mut pid_namespace { proc_sb_info(sb).as_ref().unwrap().pid_ns }

#[cfg(all(feature = "proc_fs", not(feature = "module")))]
extern "C" { pub fn impl_proc_make_permanent(pde: *mut proc_dir_entry); }

#[inline]
pub unsafe fn proc_make_permanent(pde: *mut proc_dir_entry) {
    #[cfg(all(feature = "proc_fs", not(feature = "module")))]
    impl_proc_make_permanent(pde);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
