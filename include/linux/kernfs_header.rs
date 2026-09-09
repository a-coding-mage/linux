/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/kernfs.h. Kernel dependencies are supplied externally. */

use core::ffi::{c_char, c_int, c_void};

/* Opaque types supplied by other kernel headers. */
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct ns_common { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_operations_struct { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct file_system_type { _private: [u8; 0] }
#[repr(C)] pub struct poll_table_struct { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_root { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct kernfs_open_node { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_iattrs { _private: [u8; 0] }

pub type loff_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type umode_t = u16;
pub type ino_t = u64;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type __poll_t = u32;

pub const NR_KERNFS_LOCK_BITS: usize = 1; /* CONFIG_SMP selects the ilog2(NR_CPUS) expression. */
pub const NR_KERNFS_LOCKS: usize = 1usize << NR_KERNFS_LOCK_BITS;

#[repr(C)]
pub struct kernfs_global_locks { pub node_mutex: [mutex; NR_KERNFS_LOCKS] }

#[repr(C)]
#[derive(Clone, Copy)]
pub enum kernfs_node_type { KERNFS_DIR = 0x0001, KERNFS_FILE = 0x0002, KERNFS_LINK = 0x0004 }
pub const KERNFS_TYPE_MASK: u16 = 0x000f;
pub const KERNFS_FLAG_MASK: u16 = !KERNFS_TYPE_MASK;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum kernfs_node_flag {
    KERNFS_ACTIVATED = 0x0010, KERNFS_NS = 0x0020, KERNFS_HAS_SEQ_SHOW = 0x0040,
    KERNFS_HAS_MMAP = 0x0080, KERNFS_LOCKDEP = 0x0100, KERNFS_HIDDEN = 0x0200,
    KERNFS_SUICIDAL = 0x0400, KERNFS_SUICIDED = 0x0800, KERNFS_EMPTY_DIR = 0x1000,
    KERNFS_HAS_RELEASE = 0x2000, KERNFS_REMOVING = 0x4000,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub enum kernfs_root_flag {
    KERNFS_ROOT_CREATE_DEACTIVATED = 0x0001, KERNFS_ROOT_EXTRA_OPEN_PERM_CHECK = 0x0002,
    KERNFS_ROOT_SUPPORT_EXPORTOP = 0x0004, KERNFS_ROOT_SUPPORT_USER_XATTR = 0x0008,
    KERNFS_ROOT_INVARIANT_PARENT = 0x0010,
}

#[repr(C)] pub struct kernfs_elem_dir { pub subdirs: usize, pub children: rb_root, pub root: *mut kernfs_root, pub rev: usize }
#[repr(C)] pub struct kernfs_elem_symlink { pub target_kn: *mut kernfs_node }
#[repr(C)] pub struct kernfs_elem_attr { pub ops: *const kernfs_ops, pub open: *mut kernfs_open_node, pub size: loff_t, pub notify_next: *mut kernfs_node }
#[repr(C)] pub union kernfs_node_elem { pub dir: kernfs_elem_dir, pub symlink: kernfs_elem_symlink, pub attr: kernfs_elem_attr }

#[repr(C)]
pub struct kernfs_node {
    pub count: atomic_t, pub active: atomic_t,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)] pub dep_map: lockdep_map,
    pub __parent: *mut kernfs_node, pub name: *const c_char, pub rb: rb_node,
    pub ns: *const ns_common, pub hash: u32, pub flags: u16, pub mode: umode_t,
    pub elem: kernfs_node_elem, pub id: u64, pub priv_: *mut c_void,
    pub iattr: *mut kernfs_iattrs, pub rcu: rcu_head,
}

#[repr(C)]
pub struct kernfs_syscall_ops {
    pub show_options: Option<unsafe extern "C" fn(*mut seq_file, *mut kernfs_root) -> c_int>,
    pub mkdir: Option<unsafe extern "C" fn(*mut kernfs_node, *const c_char, umode_t) -> c_int>,
    pub rmdir: Option<unsafe extern "C" fn(*mut kernfs_node) -> c_int>,
    pub rename: Option<unsafe extern "C" fn(*mut kernfs_node, *mut kernfs_node, *const c_char) -> c_int>,
    pub show_path: Option<unsafe extern "C" fn(*mut seq_file, *mut kernfs_node, *mut kernfs_root) -> c_int>,
}

#[repr(C)]
pub struct kernfs_open_file {
    pub kn: *mut kernfs_node, pub file: *mut file, pub seq_file: *mut seq_file, pub priv_: *mut c_void,
    pub mutex: mutex, pub prealloc_mutex: mutex, pub event: c_int, pub list: list_head, pub prealloc_buf: *mut c_char,
    pub atomic_write_len: size_t, pub mmapped: bool, pub released: bool, pub vm_ops: *const vm_operations_struct,
}

#[repr(C)]
pub struct kernfs_ops {
    pub open: Option<unsafe extern "C" fn(*mut kernfs_open_file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut kernfs_open_file)>,
    pub seq_show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
    pub seq_start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>,
    pub seq_next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>,
    pub seq_stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut c_char, size_t, loff_t) -> ssize_t>,
    pub atomic_write_len: size_t, pub prealloc: bool,
    pub write: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut c_char, size_t, loff_t) -> ssize_t>,
    pub poll: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut poll_table_struct) -> __poll_t>,
    pub mmap: Option<unsafe extern "C" fn(*mut kernfs_open_file, *mut vm_area_struct) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut kernfs_open_file, loff_t, c_int) -> loff_t>,
}

#[repr(C)] pub struct kernfs_fs_context { pub root: *mut kernfs_root, pub ns_tag: *mut ns_common, pub magic: usize, pub new_sb_created: bool }

extern "C" {
    pub fn kernfs_root_to_node(root: *mut kernfs_root) -> *mut kernfs_node;
    pub fn kernfs_name(kn: *mut kernfs_node, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn kernfs_path_from_node(kn_to: *mut kernfs_node, kn_from: *mut kernfs_node, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn pr_cont_kernfs_name(kn: *mut kernfs_node); pub fn pr_cont_kernfs_path(kn: *mut kernfs_node);
    pub fn kernfs_get_parent(kn: *mut kernfs_node) -> *mut kernfs_node;
    pub fn kernfs_find_and_get_ns(parent: *mut kernfs_node, name: *const c_char, ns: *const ns_common) -> *mut kernfs_node;
    pub fn kernfs_walk_and_get_ns(parent: *mut kernfs_node, path: *const c_char, ns: *const ns_common) -> *mut kernfs_node;
    pub fn kernfs_get(kn: *mut kernfs_node); pub fn kernfs_put(kn: *mut kernfs_node);
    pub fn kernfs_node_from_dentry(dentry: *mut dentry) -> *mut kernfs_node;
    pub fn kernfs_root_from_sb(sb: *mut super_block) -> *mut kernfs_root;
    pub fn kernfs_get_inode(sb: *mut super_block, kn: *mut kernfs_node) -> *mut c_void;
    pub fn kernfs_node_dentry(kn: *mut kernfs_node, sb: *mut super_block) -> *mut dentry;
    pub fn kernfs_create_root(scops: *mut kernfs_syscall_ops, flags: u32, priv_: *mut c_void) -> *mut kernfs_root;
    pub fn kernfs_destroy_root(root: *mut kernfs_root); pub fn kernfs_root_flags(kn: *mut kernfs_node) -> u32;
    pub fn kernfs_create_dir_ns(parent: *mut kernfs_node, name: *const c_char, mode: umode_t, uid: kuid_t, gid: kgid_t, priv_: *mut c_void, ns: *const ns_common) -> *mut kernfs_node;
    pub fn kernfs_create_empty_dir(parent: *mut kernfs_node, name: *const c_char) -> *mut kernfs_node;
    pub fn __kernfs_create_file(parent: *mut kernfs_node, name: *const c_char, mode: umode_t, uid: kuid_t, gid: kgid_t, size: loff_t, ops: *const kernfs_ops, priv_: *mut c_void, ns: *const ns_common, key: *mut lock_class_key) -> *mut kernfs_node;
    pub fn kernfs_create_link(parent: *mut kernfs_node, name: *const c_char, target: *mut kernfs_node) -> *mut kernfs_node;
    pub fn kernfs_activate(kn: *mut kernfs_node); pub fn kernfs_show(kn: *mut kernfs_node, show: bool); pub fn kernfs_remove(kn: *mut kernfs_node);
    pub fn kernfs_break_active_protection(kn: *mut kernfs_node); pub fn kernfs_unbreak_active_protection(kn: *mut kernfs_node); pub fn kernfs_remove_self(kn: *mut kernfs_node) -> bool;
    pub fn kernfs_remove_by_name_ns(parent: *mut kernfs_node, name: *const c_char, ns: *const ns_common) -> c_int;
    pub fn kernfs_rename_ns(kn: *mut kernfs_node, new_parent: *mut kernfs_node, new_name: *const c_char, new_ns: *const ns_common) -> c_int;
    pub fn kernfs_setattr(kn: *mut kernfs_node, iattr: *const iattr) -> c_int; pub fn kernfs_generic_poll(of: *mut kernfs_open_file, pt: *mut poll_table_struct) -> __poll_t; pub fn kernfs_notify(kn: *mut kernfs_node);
    pub fn kernfs_xattr_get(kn: *mut kernfs_node, name: *const c_char, value: *mut c_void, size: size_t) -> c_int;
    pub fn kernfs_xattr_set(kn: *mut kernfs_node, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int;
    pub fn kernfs_super_ns(sb: *mut super_block) -> *const ns_common; pub fn kernfs_get_tree(fc: *mut fs_context) -> c_int; pub fn kernfs_free_fs_context(fc: *mut fs_context); pub fn kernfs_kill_sb(sb: *mut super_block); pub fn kernfs_init();
    pub fn kernfs_find_and_get_node_by_id(root: *mut kernfs_root, id: u64) -> *mut kernfs_node;
}

#[inline] pub unsafe fn kernfs_type(kn: *mut kernfs_node) -> u16 { (*kn).flags & KERNFS_TYPE_MASK }
#[inline] pub unsafe fn kernfs_id_ino(id: u64) -> ino_t { id }
#[inline] pub unsafe fn kernfs_id_gen(id: u64) -> u32 { let _ = id; 1 }
#[inline] pub unsafe fn kernfs_ino(kn: *mut kernfs_node) -> ino_t { kernfs_id_ino((*kn).id) }
#[inline] pub unsafe fn kernfs_gen(kn: *mut kernfs_node) -> ino_t { kernfs_id_gen((*kn).id) as ino_t }
#[inline] pub unsafe fn kernfs_path(kn: *mut kernfs_node, buf: *mut c_char, buflen: size_t) -> c_int { kernfs_path_from_node(kn, core::ptr::null_mut(), buf, buflen) }
#[inline] pub unsafe fn kernfs_find_and_get(kn: *mut kernfs_node, name: *const c_char) -> *mut kernfs_node { kernfs_find_and_get_ns(kn, name, core::ptr::null()) }
#[inline] pub unsafe fn kernfs_walk_and_get(kn: *mut kernfs_node, path: *const c_char) -> *mut kernfs_node { kernfs_walk_and_get_ns(kn, path, core::ptr::null()) }
#[inline] pub unsafe fn kernfs_remove_by_name(parent: *mut kernfs_node, name: *const c_char) -> c_int { kernfs_remove_by_name_ns(parent, name, core::ptr::null()) }
#[inline] pub unsafe fn kernfs_rename(kn: *mut kernfs_node, new_parent: *mut kernfs_node, new_name: *const c_char) -> c_int { kernfs_rename_ns(kn, new_parent, new_name, core::ptr::null()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
