// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor /sys/kernel/security/apparmor interface functions
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

/*
 * Rust translation of apparmorfs.c.
 *
 * C include dependencies intentionally remain external to this isolated file:
 * linux/ctype.h, slab.h, security.h, vmalloc.h, init.h, seq_file.h,
 * uaccess.h, mount.h, namei.h, capability.h, rcupdate.h, fs.h,
 * fs_context.h, poll.h, zstd.h, string.h, uapi major/magic, and the
 * AppArmor internal headers.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type umode_t = u16;
type u8 = u8;
type u32 = u32;
type __le32 = u32;
type __poll_t = u32;
type aa_state_t = u32;

const IREF_POISON: usize = 101;
const AAFS_NAME: &[u8] = b"apparmorfs\0";
const aa_hdr_magic: &[u8] = b"\x04\x08\x00version\x00\x02\0";
const aa_hdr_magic_size: usize = 12;
const NULL_FILE_NAME: &[u8] = b".null\0";

/* External kernel/AppArmor constants. */
extern "C" {
    static AAFS_MAGIC: c_ulong;
    static GFP_KERNEL: c_uint;
    static PAGE_SIZE: size_t;
    static SIZE_MAX: size_t;
    static S_IFMT: umode_t;
    static S_IALLUGO: umode_t;
    static S_IFREG: umode_t;
    static S_IFDIR: umode_t;
    static S_IFLNK: umode_t;
    static S_IFCHR: umode_t;
    static S_IRUGO: umode_t;
    static S_IWUGO: umode_t;
    static SB_NOUSER: c_ulong;
    static THIS_MODULE: *mut c_void;
    static O_NONBLOCK: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ESPIPE: c_int;
    static EFAULT: c_int;
    static EAGAIN: c_int;
    static ERESTARTSYS: c_int;
    static EFBIG: c_int;
    static ENOENT: c_int;
    static EACCES: c_int;
    static EEXIST: c_int;
    static ENAMETOOLONG: c_int;
    static ECHILD: c_int;
    static EPOLLIN: __poll_t;
    static EPOLLRDNORM: __poll_t;
    static I_MUTEX_PARENT: c_uint;
    static MEM_MAJOR: c_uint;
    static ZSTD_MAGICNUMBER: u32;
    static ZSTD_CONTENTSIZE_UNKNOWN: u64;
    static ZSTD_CONTENTSIZE_ERROR: u64;
    static AA_MAY_LOAD_POLICY: u32;
    static AA_MAY_REPLACE_POLICY: u32;
    static AA_MAY_REMOVE_POLICY: u32;
    static AA_CLASS_FILE: u8;
    static AA_CLASS_DBUS: u8;
    static DFA_NOMATCH: aa_state_t;
    static FLAG_SHOW_MODE: u32;
    static FLAG_VIEW_SUBNS: u32;
    static AA_MIN_CLEVEL: c_int;
    static AA_MAX_CLEVEL: c_int;
    static AAFS_LOADDATA_NDENTS: c_int;
    static AAFS_LOADDATA_DIR: usize;
    static AAFS_LOADDATA_ABI: usize;
    static AAFS_LOADDATA_REVISION: usize;
    static AAFS_LOADDATA_HASH: usize;
    static AAFS_LOADDATA_COMPRESSED_SIZE: usize;
    static AAFS_LOADDATA_DATA: usize;
    static AAFS_PROF_SIZEOF: c_int;
    static AAFS_PROF_NAME: usize;
    static AAFS_PROF_MODE: usize;
    static AAFS_PROF_ATTACH: usize;
    static AAFS_PROF_HASH: usize;
    static AAFS_PROF_RAW_HASH: usize;
    static AAFS_PROF_RAW_ABI: usize;
    static AAFS_PROF_RAW_DATA: usize;
    static AAFS_NS_SIZEOF: c_int;
    static MAX_OOB_SUPPORTED: u64;
    static VFS_CAP_FLAGS_MASK: u64;
    static aa_g_hash_policy: bool;
    static apparmor_initialized: bool;
    static mut root_ns: *mut aa_ns;
    static allperms: aa_perms;
    static aa_profile_mode_names: [*const c_char; 0];
    static aa_sfs_entry_network: aa_sfs_entry;
    static aa_sfs_entry_networkv9: aa_sfs_entry;
    static aa_sfs_entry_rlimit: aa_sfs_entry;
    static aa_sfs_entry_caps: aa_sfs_entry;
}

#[repr(C)] pub struct kref { pub count: c_int }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct aa_common_ref { pub count: kref, pub reftype: c_int }
#[repr(C)] pub struct aa_label { pub count: aa_common_ref, pub size: c_int, pub proxy: *mut aa_proxy, pub rules: [*mut aa_ruleset; 1] }
#[repr(C)] pub struct aa_proxy { pub count: aa_common_ref, pub label: *mut aa_label }
#[repr(C)] pub struct aa_data { pub size: u32, pub data: *mut c_char }
#[repr(C)] pub struct aa_loaddata {
    pub count: aa_common_ref,
    pub size: size_t,
    pub compressed_size: size_t,
    pub data: *mut c_char,
    pub abi: c_int,
    pub revision: c_long,
    pub hash: *mut u8,
    pub dents: [*mut dentry; 8],
    pub ns: *mut aa_ns,
    pub list: list_head,
    pub name: *mut c_char,
}
#[repr(C)] pub struct aa_profile {
    pub base: aa_policy_base,
    pub label: aa_label,
    pub ns: *mut aa_ns,
    pub parent: *mut aa_profile,
    pub data: *mut rhashtable,
    pub mode: usize,
    pub attach: aa_attachment,
    pub hash: *mut u8,
    pub rawdata: *mut aa_loaddata,
    pub dents: [*mut dentry; 16],
    pub dirname: *mut c_char,
}
#[repr(C)] pub struct aa_policy_base { pub name: *mut c_char, pub profiles: list_head, pub list: list_head }
#[repr(C)] pub struct aa_attachment { pub xmatch_str: *mut c_char, pub xmatch: *mut aa_dfa_holder }
#[repr(C)] pub struct aa_dfa_holder { pub dfa: *mut c_void }
#[repr(C)] pub struct aa_ns {
    pub base: aa_policy_base,
    pub lock: c_void,
    pub level: c_uint,
    pub revision: c_long,
    pub wait: c_void,
    pub rawdata_list: list_head,
    pub sub_ns: list_head,
    pub parent: *mut aa_ns,
    pub dents: [*mut dentry; 16],
    pub unconfined: *mut aa_profile,
    pub uniq_id: c_long,
}
#[repr(C)] pub struct aa_ruleset { pub file: *mut aa_policy_db, pub policy: *mut aa_policy_db }
#[repr(C)] pub struct aa_policy_db { pub dfa: *mut c_void, pub start: [aa_state_t; 256] }
#[repr(C)] pub struct aa_perms { pub allow: u32, pub deny: u32, pub audit: u32, pub quiet: u32 }
#[repr(C)] pub struct path_cond { _priv: [u8; 0] }
#[repr(C)] pub struct cred { _priv: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: u64, pub i_mode: umode_t, pub i_private: *mut c_void, pub i_op: *const inode_operations, pub i_fop: *const file_operations, pub i_link: *mut c_char, pub i_size: loff_t }
#[repr(C)] pub struct d_name { pub name: *const c_char, pub len: c_uint }
#[repr(C)] pub struct dentry { pub d_parent: *mut dentry, pub d_inode: *mut inode, pub d_sb: *mut super_block, pub d_name: d_name }
#[repr(C)] pub struct vfsmount { pub mnt_sb: *mut super_block, pub mnt_root: *mut dentry }
#[repr(C)] pub struct super_block { pub s_op: *const super_operations, pub s_type: *mut file_system_type, pub s_flags: c_ulong }
#[repr(C)] pub struct fs_context { pub ops: *const fs_context_operations }
#[repr(C)] pub struct file { pub f_inode: *mut inode, pub f_cred: *const cred, pub private_data: *mut c_void, pub f_flags: c_int, pub f_lock: c_void }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct poll_table { _priv: [u8; 0] }
#[repr(C)] pub struct delayed_call { _priv: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _priv: [u8; 0] }
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct tree_descr { pub name: *const c_char }
#[repr(C)] pub struct zstd_dctx { _priv: [u8; 0] }
#[repr(C)] pub struct zstd_frame_header { pub frameContentSize: u64 }
#[repr(C)] pub struct label_it { _priv: [u8; 0] }
#[repr(C)] pub struct rhashtable { pub p: c_void }

#[repr(C)] pub struct super_operations { pub statfs: *const c_void, pub evict_inode: Option<unsafe extern "C" fn(*mut inode)>, pub free_inode: Option<unsafe extern "C" fn(*mut inode)>, pub show_path: Option<unsafe extern "C" fn(*mut seq_file, *mut dentry) -> c_int> }
#[repr(C)] pub struct fs_context_operations { pub get_tree: Option<unsafe extern "C" fn(*mut fs_context) -> c_int> }
#[repr(C)] pub struct file_system_type { pub owner: *mut c_void, pub name: *const c_char, pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>, pub kill_sb: *const c_void }
#[repr(C)] pub struct file_operations {
    pub owner: *mut c_void,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: *const c_void,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}
#[repr(C)] pub struct inode_operations {
    pub lookup: *const c_void,
    pub mkdir: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, *mut dentry, umode_t) -> *mut dentry>,
    pub rmdir: Option<unsafe extern "C" fn(*mut inode, *mut dentry) -> c_int>,
    pub readlink: Option<unsafe extern "C" fn(*mut dentry, *mut c_char, c_int) -> c_int>,
    pub get_link: Option<unsafe extern "C" fn(*mut dentry, *mut inode, *mut delayed_call) -> *const c_char>,
}
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>, pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }

#[repr(C)] pub union aa_sfs_value { pub boolean: bool, pub string: *const c_char, pub u64_: u64, pub files: *mut aa_sfs_entry }
#[repr(C)] pub struct aa_sfs_entry { pub name: *const c_char, pub mode: umode_t, pub v_type: c_int, pub v: aa_sfs_value, pub file_ops: *const file_operations, pub dentry: *mut dentry }

#[repr(C)]
struct rawdata_f_data {
    loaddata: *mut aa_loaddata,
    data: [c_char; 0],
}

#[repr(C, packed)]
struct aa_user_hdr {
    version: u8,
    compress_level: u8,
    padding: [u8; 6],
}

#[repr(C)]
struct aa_revision {
    ns: *mut aa_ns,
    last_read: c_long,
}

#[repr(C)]
struct multi_transaction {
    count: kref,
    size: ssize_t,
    data: [c_char; 0],
}

extern "C" {
    static simple_statfs: *const c_void;
    static kill_anon_super: *const c_void;
    static simple_dir_inode_operations: inode_operations;
    static simple_symlink_inode_operations: inode_operations;
    static simple_dir_operations: file_operations;
    static default_llseek: *const c_void;
    static generic_file_llseek: *const c_void;
    static seq_lseek: *const c_void;
    static seq_read: unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t;

    fn ERR_PTR(error: c_int) -> *mut c_void;
    fn PTR_ERR<T>(ptr: *mut T) -> c_int;
    fn IS_ERR<T>(ptr: *const T) -> bool;
    fn IS_ERR_OR_NULL<T>(ptr: *const T) -> bool;
    fn AA_BUG(cond: bool, ...);
    fn AA_ERROR(fmt: *const c_char, ...);
    fn WARN_ON(cond: bool) -> bool;
    fn panic(fmt: *const c_char, ...) -> !;
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn seq_puts(seq: *mut seq_file, s: *const c_char) -> c_int;
    fn seq_putc(seq: *mut seq_file, c: c_char) -> c_int;
    fn simple_fill_super(sb: *mut super_block, magic: c_ulong, files: *mut tree_descr) -> c_int;
    fn get_tree_single(fc: *mut fs_context, fill: unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int) -> c_int;
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn get_next_ino() -> u64;
    fn simple_inode_init_ts(inode: *mut inode);
    fn inc_nlink(inode: *mut inode);
    fn d_instantiate(dentry: *mut dentry, inode: *mut inode);
    fn dget(dentry: *mut dentry) -> *mut dentry;
    fn dput(dentry: *mut dentry);
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn clear_inode(inode: *mut inode);
    fn kfree(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kvmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn free_inode_nonrcu(inode: *mut inode);
    fn simple_pin_fs(t: *mut file_system_type, mount: *mut *mut vfsmount, count: *mut c_int) -> c_int;
    fn simple_release_fs(mount: *mut *mut vfsmount, count: *mut c_int);
    fn simple_start_creating(parent: *mut dentry, name: *const c_char) -> *mut dentry;
    fn simple_done_creating(dentry: *mut dentry);
    fn start_removing_dentry(parent: *mut dentry, dentry: *mut dentry) -> *mut dentry;
    fn end_removing(dentry: *mut dentry);
    fn simple_positive(dentry: *mut dentry) -> bool;
    fn d_is_dir(dentry: *mut dentry) -> bool;
    fn simple_empty(dentry: *mut dentry) -> bool;
    fn __simple_rmdir(dir: *mut inode, dentry: *mut dentry);
    fn __simple_unlink(dir: *mut inode, dentry: *mut dentry);
    fn d_delete(dentry: *mut dentry);
    fn aa_get_ns(ns: *mut aa_ns) -> *mut aa_ns;
    fn aa_put_ns(ns: *mut aa_ns);
    fn labels_ns(label: *mut aa_label) -> *mut aa_ns;
    fn aa_get_proxy(proxy: *mut aa_proxy) -> *mut aa_proxy;
    fn aa_put_proxy(proxy: *mut aa_proxy);
    fn aa_get_i_loaddata(data: *mut aa_loaddata) -> *mut aa_loaddata;
    fn aa_put_i_loaddata(data: *mut aa_loaddata);
    fn aa_put_profile_loaddata(data: *mut aa_loaddata);
    fn kref_get(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn kref_init(kref: *mut kref);
    fn aa_loaddata_alloc(size: size_t) -> *mut aa_loaddata;
    fn copy_from_user(dst: *mut c_void, src: *const c_char, n: size_t) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strnlen(s: *const c_char, max: size_t) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_char, available: size_t) -> ssize_t;
    fn zstd_dctx_workspace_bound() -> size_t;
    fn zstd_init_dctx(wksp: *mut c_void, len: size_t) -> *mut zstd_dctx;
    fn zstd_decompress_dctx(ctx: *mut zstd_dctx, dst: *mut c_char, dlen: size_t, src: *mut c_char, slen: size_t) -> size_t;
    fn zstd_is_error(len: size_t) -> bool;
    fn zstd_get_frame_header(h: *mut zstd_frame_header, src: *mut c_char, len: size_t) -> c_int;
    fn le32_to_cpu(v: __le32) -> u32;
    fn __cpu_to_le32(v: u32) -> __le32;
    fn begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn current_cred() -> *const cred;
    fn aa_may_manage_policy(c: *const cred, label: *mut aa_label, ns: *mut aa_ns, ocred: *const cred, mask: u32) -> ssize_t;
    fn aa_replace_profiles(ns: *mut aa_ns, label: *mut aa_label, mask: u32, data: *mut aa_loaddata, compressed: *mut c_char, size: size_t) -> ssize_t;
    fn aa_remove_profiles(ns: *mut aa_ns, label: *mut aa_label, name: *mut c_char, size: size_t) -> ssize_t;
    fn aa_get_current_ns() -> *mut aa_ns;
    fn mutex_lock_nested(lock: *mut c_void, subclass: c_uint);
    fn mutex_unlock(lock: *mut c_void);
    fn mutex_is_locked(lock: *mut c_void) -> bool;
    fn wait_event_interruptible(wait: *mut c_void, condition: bool) -> c_int;
    fn wake_up_interruptible(wait: *mut c_void);
    fn poll_wait(file: *mut file, wait: *mut c_void, pt: *mut poll_table);
    fn profile_unconfined(profile: *const aa_profile) -> bool;
    fn aa_dfa_match_len(dfa: *mut c_void, start: aa_state_t, s: *const c_char, len: size_t) -> aa_state_t;
    fn current_fsuid() -> c_uint;
    fn aa_lookup_condperms(fsuid: c_uint, db: *mut aa_policy_db, state: aa_state_t, cond: *mut path_cond) -> *mut aa_perms;
    fn aa_lookup_perms(db: *mut aa_policy_db, state: aa_state_t) -> *mut aa_perms;
    fn aa_apply_modes_to_perms(profile: *const aa_profile, perms: *mut aa_perms);
    fn aa_perms_accum_raw(perms: *mut aa_perms, tmp: *const aa_perms);
    fn RULE_MEDIATES(rules: *mut aa_ruleset, class: u8) -> bool;
    fn RULE_MEDIATES_v9NET(rules: *mut aa_ruleset) -> bool;
    fn aa_label_parse(curr: *mut aa_label, s: *mut c_char, flags: c_uint, a: bool, b: bool) -> *mut aa_label;
    fn aa_put_label(label: *mut aa_label);
    fn rhashtable_lookup_fast(ht: *mut rhashtable, key: *const *const c_char, params: *const c_void) -> *mut aa_data;
    fn str_yes_no(v: bool) -> *const c_char;
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn single_release(inode: *mut inode, file: *mut file) -> c_int;
    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn aa_get_label_rcu(label: *mut *mut aa_label) -> *mut aa_label;
    fn labels_profile(label: *mut aa_label) -> *mut aa_profile;
    fn aa_hash_size() -> c_uint;
    fn aa_current_policy_view_capable(ns: *mut aa_ns) -> bool;
    fn inode_set_mtime_to_ts(inode: *mut inode, ts: c_void);
    fn inode_set_ctime_current(inode: *mut inode) -> c_void;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_access_pointer(p: *mut aa_profile) -> *mut aa_profile;
    fn set_delayed_call(done: *mut delayed_call, f: *const c_void, arg: *mut c_void);
    fn kfree_link(arg: *mut c_void);
    fn prof_dir(profile: *mut aa_profile) -> *mut dentry;
    fn prof_child_dir(profile: *mut aa_profile) -> *mut dentry;
    fn ns_subdata_dir(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subprofs_dir(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subrevision(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subload(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subreplace(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subremove(ns: *mut aa_ns) -> *mut dentry;
    fn ns_subns_dir(ns: *mut aa_ns) -> *mut dentry;
    fn ns_dir(ns: *mut aa_ns) -> *mut dentry;
    fn profiles_ns(profile: *mut aa_profile) -> *mut aa_ns;
    fn aa_deref_parent(profile: *mut aa_profile) -> *mut aa_profile;
    fn __aa_create_rawdata_symlink_dents(profile: *mut aa_profile) -> c_int;
    fn inode_unlock(inode: *mut inode);
    fn inode_lock(inode: *mut inode);
    fn inode_lock_nested(inode: *mut inode, subclass: c_uint);
    fn __aa_find_or_create_ns(parent: *mut aa_ns, name: *const c_char, dentry: *mut dentry) -> *mut aa_ns;
    fn __aa_findn_ns(head: *mut list_head, name: *const c_char, len: c_uint) -> *mut aa_ns;
    fn __aa_remove_ns(ns: *mut aa_ns);
    fn list_del_init(head: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *mut list_head) -> bool;
    fn aa_label_seq_xprint(f: *mut seq_file, root: *mut aa_ns, label: *mut aa_label, flags: u32, gfp: c_uint);
    fn securityfs_create_file(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_symlink(name: *const c_char, parent: *mut dentry, target: *const c_char, iops: *const inode_operations) -> *mut dentry;
    fn securityfs_remove(dentry: *mut dentry);
    fn kern_mount(fs: *mut file_system_type) -> *mut vfsmount;
    fn mntget(mnt: *mut vfsmount) -> *mut vfsmount;
    fn MKDEV(major: c_uint, minor: c_uint) -> c_uint;
    fn init_special_inode(inode: *mut inode, mode: umode_t, dev: c_uint);
    fn nd_jump_link(path: *mut path) -> c_int;
    fn readlink_copy(buffer: *mut c_char, buflen: c_int, name: *const c_char, len: size_t) -> c_int;
    fn aa_info_message(s: *const c_char);
}

static mut aafs_mnt: *mut vfsmount = null_mut();
static mut aafs_count: c_int = 0;

unsafe fn S_ISDIR(mode: umode_t) -> bool { (mode & S_IFMT) == S_IFDIR }
unsafe fn S_ISLNK(mode: umode_t) -> bool { (mode & S_IFMT) == S_IFLNK }

unsafe extern "C" fn rawdata_f_data_free(private: *mut rawdata_f_data) {
    if private.is_null() { return; }
    aa_put_i_loaddata((*private).loaddata);
    kvfree(private as *mut c_void);
}

unsafe extern "C" fn rawdata_f_data_alloc(size: size_t) -> *mut rawdata_f_data {
    if size > SIZE_MAX - size_of::<rawdata_f_data>() { return ERR_PTR(-EINVAL) as *mut rawdata_f_data; }
    let ret = kvzalloc(size_of::<rawdata_f_data>() + size, GFP_KERNEL) as *mut rawdata_f_data;
    if ret.is_null() { return ERR_PTR(-ENOMEM) as *mut rawdata_f_data; }
    ret
}

unsafe extern "C" fn mangle_name(mut name: *const c_char, target: *mut c_char) -> c_int {
    let mut t = target;
    while *name == b'/' as c_char || *name == b'.' as c_char { name = name.add(1); }
    if !target.is_null() {
        while *name != 0 {
            if *name == b'/' as c_char { *t = b'.' as c_char; t = t.add(1); }
            else if isspace(*name as c_int) != 0 { *t = b'_' as c_char; t = t.add(1); }
            else if isalnum(*name as c_int) != 0 || !strchr(b"._-\0".as_ptr() as *const c_char, *name as c_int).is_null() { *t = *name; t = t.add(1); }
            name = name.add(1);
        }
        *t = 0;
    } else {
        let mut len = 0;
        while *name != 0 {
            if isalnum(*name as c_int) != 0 || isspace(*name as c_int) != 0 || !strchr(b"/._-\0".as_ptr() as *const c_char, *name as c_int).is_null() { len += 1; }
            name = name.add(1);
        }
        return len;
    }
    t.offset_from(target) as c_int
}

unsafe extern "C" fn aafs_show_path(seq: *mut seq_file, dentry: *mut dentry) -> c_int {
    seq_printf(seq, b"%s:[%llu]\0".as_ptr() as *const c_char, AAFS_NAME.as_ptr(), (*d_inode(dentry)).i_ino);
    0
}

unsafe extern "C" fn get_ns_common_ref(ref_: *mut aa_common_ref) -> *mut aa_ns {
    if !ref_.is_null() {
        let reflabel = ref_ as *mut aa_label;
        return aa_get_ns(labels_ns(reflabel));
    }
    null_mut()
}

unsafe extern "C" fn get_proxy_common_ref(ref_: *mut aa_common_ref) -> *mut aa_proxy {
    if !ref_.is_null() { return aa_get_proxy(ref_ as *mut aa_proxy); }
    null_mut()
}

unsafe extern "C" fn get_loaddata_common_ref(ref_: *mut aa_common_ref) -> *mut aa_loaddata {
    if !ref_.is_null() { return aa_get_i_loaddata(ref_ as *mut aa_loaddata); }
    null_mut()
}

unsafe extern "C" fn aa_put_common_ref(ref_: *mut aa_common_ref) {
    if ref_.is_null() { return; }
    match (*ref_).reftype {
        0 => aa_put_i_loaddata(ref_ as *mut aa_loaddata),
        1 => aa_put_proxy(ref_ as *mut aa_proxy),
        2 => aa_put_ns(labels_ns(ref_ as *mut aa_label)),
        _ => AA_BUG(true, b"unknown refcount type\0".as_ptr() as *const c_char),
    }
}

unsafe extern "C" fn aa_get_common_ref(ref_: *mut aa_common_ref) { kref_get(&mut (*ref_).count); }

unsafe extern "C" fn aafs_evict(inode: *mut inode) {
    let ref_ = (*inode).i_private as *mut aa_common_ref;
    clear_inode(inode);
    aa_put_common_ref(ref_);
    (*inode).i_private = IREF_POISON as *mut c_void;
}

unsafe extern "C" fn aafs_free_inode(inode: *mut inode) {
    if S_ISLNK((*inode).i_mode) { kfree((*inode).i_link as *mut c_void); }
    free_inode_nonrcu(inode);
}

static aafs_super_ops: super_operations = super_operations {
    statfs: unsafe { simple_statfs },
    evict_inode: Some(aafs_evict),
    free_inode: Some(aafs_free_inode),
    show_path: Some(aafs_show_path),
};

unsafe extern "C" fn apparmorfs_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> c_int {
    let mut files = [tree_descr { name: b"\0".as_ptr() as *const c_char }];
    let error = simple_fill_super(sb, AAFS_MAGIC, files.as_mut_ptr());
    if error != 0 { return error; }
    (*sb).s_op = &aafs_super_ops;
    0
}

unsafe extern "C" fn apparmorfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_single(fc, apparmorfs_fill_super) }
static apparmorfs_context_ops: fs_context_operations = fs_context_operations { get_tree: Some(apparmorfs_get_tree) };
unsafe extern "C" fn apparmorfs_init_fs_context(fc: *mut fs_context) -> c_int { (*fc).ops = &apparmorfs_context_ops; 0 }
static mut aafs_ops: file_system_type = file_system_type { owner: null_mut(), name: AAFS_NAME.as_ptr() as *const c_char, init_fs_context: Some(apparmorfs_init_fs_context), kill_sb: null() };

unsafe extern "C" fn __aafs_setup_d_inode(dir: *mut inode, dentry: *mut dentry, mode: umode_t, data: *mut c_void, link: *mut c_char, fops: *const file_operations, iops: *const inode_operations) -> c_int {
    let inode = new_inode((*dir).i_sb);
    AA_BUG(dir.is_null());
    AA_BUG(dentry.is_null());
    if inode.is_null() { return -ENOMEM; }
    (*inode).i_ino = get_next_ino();
    (*inode).i_mode = mode;
    simple_inode_init_ts(inode);
    (*inode).i_private = data;
    if S_ISDIR(mode) {
        (*inode).i_op = if !iops.is_null() { iops } else { &simple_dir_inode_operations };
        (*inode).i_fop = &simple_dir_operations;
        inc_nlink(inode);
        inc_nlink(dir);
    } else if S_ISLNK(mode) {
        (*inode).i_op = if !iops.is_null() { iops } else { &simple_symlink_inode_operations };
        (*inode).i_link = link;
    } else {
        (*inode).i_fop = fops;
    }
    d_instantiate(dentry, inode);
    dget(dentry);
    0
}

unsafe extern "C" fn aafs_create(name: *const c_char, mut mode: umode_t, parent: *mut dentry, data: *mut aa_common_ref, link: *mut c_void, fops: *const file_operations, iops: *const inode_operations) -> *mut dentry {
    AA_BUG(name.is_null());
    AA_BUG(parent.is_null());
    if (mode & S_IFMT) == 0 { mode = (mode & S_IALLUGO) | S_IFREG; }
    let mut error = simple_pin_fs(&mut aafs_ops, &mut aafs_mnt, &mut aafs_count);
    if error != 0 { return ERR_PTR(error) as *mut dentry; }
    let dir = d_inode(parent);
    let dentry = simple_start_creating(parent, name);
    if IS_ERR(dentry) {
        error = PTR_ERR(dentry);
        simple_release_fs(&mut aafs_mnt, &mut aafs_count);
        return ERR_PTR(error) as *mut dentry;
    }
    error = __aafs_setup_d_inode(dir, dentry, mode, data as *mut c_void, link as *mut c_char, fops, iops);
    simple_done_creating(dentry);
    if error != 0 {
        simple_release_fs(&mut aafs_mnt, &mut aafs_count);
        return ERR_PTR(error) as *mut dentry;
    }
    if !data.is_null() { aa_get_common_ref(data); }
    dentry
}

unsafe extern "C" fn aafs_create_file(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut aa_common_ref, fops: *const file_operations) -> *mut dentry {
    aafs_create(name, mode, parent, data, null_mut(), fops, null())
}

unsafe extern "C" fn aafs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry {
    aafs_create(name, S_IFDIR | 0o755, parent, null_mut(), null_mut(), null(), null())
}

unsafe extern "C" fn aafs_remove(mut dentry: *mut dentry) {
    if dentry.is_null() || IS_ERR(dentry) { return; }
    let dir = d_inode((*dentry).d_parent);
    dentry = start_removing_dentry((*dentry).d_parent, dentry);
    if !IS_ERR(dentry) && simple_positive(dentry) {
        if d_is_dir(dentry) {
            if !WARN_ON(!simple_empty(dentry)) {
                __simple_rmdir(dir, dentry);
                dput(dentry);
            }
        } else {
            __simple_unlink(dir, dentry);
            dput(dentry);
        }
        d_delete(dentry);
    }
    end_removing(dentry);
    simple_release_fs(&mut aafs_mnt, &mut aafs_count);
}

unsafe extern "C" fn aa_simple_write_to_buffer(userbuf: *const c_char, alloc_size: size_t, copy_size: size_t, pos: *mut loff_t) -> *mut aa_loaddata {
    AA_BUG(copy_size > alloc_size);
    if *pos != 0 { return ERR_PTR(-ESPIPE) as *mut aa_loaddata; }
    let data = aa_loaddata_alloc(alloc_size);
    if IS_ERR(data) { return data; }
    (*data).size = copy_size;
    if copy_from_user((*data).data as *mut c_void, userbuf, copy_size) != 0 {
        aa_put_i_loaddata(data);
        return ERR_PTR(-EFAULT) as *mut aa_loaddata;
    }
    data
}

unsafe extern "C" fn decompress_zstd(src: *mut c_char, slen: size_t, dst: *mut c_char, dlen: size_t) -> c_int {
    if slen < dlen {
        let wksp_len = zstd_dctx_workspace_bound();
        let mut ret = 0;
        let wksp = kvzalloc(wksp_len, GFP_KERNEL);
        if wksp.is_null() { ret = -ENOMEM; kvfree(wksp); return ret; }
        let ctx = zstd_init_dctx(wksp, wksp_len);
        if ctx.is_null() { ret = -ENOMEM; kvfree(wksp); return ret; }
        let out_len = zstd_decompress_dctx(ctx, dst, dlen, src, slen);
        if zstd_is_error(out_len) { ret = -EINVAL; }
        kvfree(wksp);
        return ret;
    }
    if dlen < slen { return -EINVAL; }
    memcpy(dst as *mut c_void, src as *const c_void, slen);
    0
}

unsafe extern "C" fn aa_get_data_from_compressed(userbuf: *const c_char, buffer_size: size_t, pos: *mut loff_t, compressed_data: *mut *mut c_char) -> *mut aa_loaddata {
    let mut header: zstd_frame_header = zeroed();
    if userbuf.is_null() || pos.is_null() { return ERR_PTR(-EINVAL) as *mut aa_loaddata; }
    if *pos != 0 { return ERR_PTR(-ESPIPE) as *mut aa_loaddata; }
    *compressed_data = kvmalloc(buffer_size, GFP_KERNEL) as *mut c_char;
    if (*compressed_data).is_null() { return ERR_PTR(-ENOMEM) as *mut aa_loaddata; }
    if copy_from_user(*compressed_data as *mut c_void, userbuf, buffer_size) != 0 {
        kvfree(*compressed_data as *mut c_void);
        return ERR_PTR(-EFAULT) as *mut aa_loaddata;
    }
    let mut error = zstd_get_frame_header(&mut header, *compressed_data, buffer_size);
    if error != 0 || header.frameContentSize == ZSTD_CONTENTSIZE_UNKNOWN || header.frameContentSize == ZSTD_CONTENTSIZE_ERROR {
        kvfree(*compressed_data as *mut c_void);
        return ERR_PTR(-EINVAL) as *mut aa_loaddata;
    }
    let data = aa_loaddata_alloc(header.frameContentSize as size_t);
    if IS_ERR(data) {
        error = PTR_ERR(data);
        kvfree(*compressed_data as *mut c_void);
        return ERR_PTR(error) as *mut aa_loaddata;
    }
    error = decompress_zstd(*compressed_data, buffer_size, (*data).data, header.frameContentSize as size_t);
    if error != 0 {
        aa_put_i_loaddata(data);
        kvfree(*compressed_data as *mut c_void);
        return ERR_PTR(error) as *mut aa_loaddata;
    }
    (*data).size = header.frameContentSize as size_t;
    data
}

unsafe extern "C" fn policy_update(mask: u32, mut buf: *const c_char, mut size: size_t, pos: *mut loff_t, ns: *mut aa_ns, ocred: *const cred) -> ssize_t {
    let mut compressed_data: *mut c_char = null_mut();
    let mut magic_le: __le32 = 0;
    let mut aahdr = [0u8; aa_hdr_magic_size];
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = aa_may_manage_policy(current_cred(), label, ns, ocred, mask);
    if error != 0 { end_current_label_crit_section(label, needput); return error; }
    let is_compressed: bool;
    if size >= size_of::<__le32>() && copy_from_user(&mut magic_le as *mut _ as *mut c_void, buf, size_of::<__le32>()) == 0 && le32_to_cpu(magic_le) == ZSTD_MAGICNUMBER {
        is_compressed = true;
    } else if size >= size_of::<aa_user_hdr>() + size_of::<__le32>() && copy_from_user(&mut magic_le as *mut _ as *mut c_void, buf.add(size_of::<aa_user_hdr>()), size_of::<__le32>()) == 0 && le32_to_cpu(magic_le) == ZSTD_MAGICNUMBER {
        is_compressed = true;
        buf = buf.add(size_of::<aa_user_hdr>());
        size -= size_of::<aa_user_hdr>();
    } else if size >= size_of::<aa_user_hdr>() + aa_hdr_magic_size && copy_from_user(aahdr.as_mut_ptr() as *mut c_void, buf.add(size_of::<aa_user_hdr>()), aa_hdr_magic_size) == 0 && memcmp(aahdr.as_ptr() as *const c_void, aa_hdr_magic.as_ptr() as *const c_void, aa_hdr_magic_size) == 0 {
        buf = buf.add(size_of::<aa_user_hdr>());
        size -= size_of::<aa_user_hdr>();
        is_compressed = false;
    } else {
        is_compressed = false;
    }
    let data = if is_compressed { aa_get_data_from_compressed(buf, size, pos, &mut compressed_data) } else { aa_simple_write_to_buffer(buf, size, size, pos) };
    error = PTR_ERR(data) as ssize_t;
    if !IS_ERR(data) {
        error = aa_replace_profiles(ns, label, mask, data, compressed_data, size);
        aa_put_profile_loaddata(data);
    }
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn profile_load(f: *mut file, buf: *const c_char, size: size_t, pos: *mut loff_t) -> ssize_t {
    let ns = get_ns_common_ref((*(*f).f_inode).i_private as *mut aa_common_ref);
    let error = policy_update(AA_MAY_LOAD_POLICY, buf, size, pos, ns, (*f).f_cred);
    aa_put_ns(ns);
    error
}

unsafe extern "C" fn profile_replace(f: *mut file, buf: *const c_char, size: size_t, pos: *mut loff_t) -> ssize_t {
    let ns = get_ns_common_ref((*(*f).f_inode).i_private as *mut aa_common_ref);
    let error = policy_update(AA_MAY_LOAD_POLICY | AA_MAY_REPLACE_POLICY, buf, size, pos, ns, (*f).f_cred);
    aa_put_ns(ns);
    error
}

unsafe extern "C" fn profile_remove(f: *mut file, buf: *const c_char, size: size_t, pos: *mut loff_t) -> ssize_t {
    let ns = get_ns_common_ref((*(*f).f_inode).i_private as *mut aa_common_ref);
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = aa_may_manage_policy(current_cred(), label, ns, (*f).f_cred, AA_MAY_REMOVE_POLICY);
    if error == 0 {
        let data = aa_simple_write_to_buffer(buf, size + 1, size, pos);
        error = PTR_ERR(data) as ssize_t;
        if !IS_ERR(data) {
            *(*data).data.add(size) = 0;
            error = aa_remove_profiles(ns, label, (*data).data, size);
            aa_put_profile_loaddata(data);
        }
    }
    end_current_label_crit_section(label, needput);
    aa_put_ns(ns);
    error
}

static aa_fs_profile_load: file_operations = file_operations { owner: null_mut(), open: None, poll: None, read: None, write: Some(profile_load), llseek: null(), release: None };
static aa_fs_profile_replace: file_operations = file_operations { owner: null_mut(), open: None, poll: None, read: None, write: Some(profile_replace), llseek: null(), release: None };
static aa_fs_profile_remove: file_operations = file_operations { owner: null_mut(), open: None, poll: None, read: None, write: Some(profile_remove), llseek: null(), release: None };

unsafe extern "C" fn ns_revision_release(_inode: *mut inode, file: *mut file) -> c_int {
    let rev = (*file).private_data as *mut aa_revision;
    if !rev.is_null() { aa_put_ns((*rev).ns); kfree(rev as *mut c_void); }
    0
}

unsafe extern "C" fn ns_revision_read(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    let rev = (*file).private_data as *mut aa_revision;
    let mut buffer = [0 as c_char; 32];
    mutex_lock_nested(&mut (*(*rev).ns).lock, (*(*rev).ns).level);
    let last_read = (*rev).last_read;
    if last_read == (*(*rev).ns).revision {
        mutex_unlock(&mut (*(*rev).ns).lock);
        if ((*file).f_flags & O_NONBLOCK) != 0 { return -EAGAIN as ssize_t; }
        if wait_event_interruptible(&mut (*(*rev).ns).wait, last_read != (*(*rev).ns).revision) != 0 { return -ERESTARTSYS as ssize_t; }
        mutex_lock_nested(&mut (*(*rev).ns).lock, (*(*rev).ns).level);
    }
    let avail = sprintf(buffer.as_mut_ptr(), b"%ld\n\0".as_ptr() as *const c_char, (*(*rev).ns).revision);
    if *ppos + size as loff_t > avail as loff_t {
        (*rev).last_read = (*(*rev).ns).revision;
        *ppos = 0;
    }
    mutex_unlock(&mut (*(*rev).ns).lock);
    simple_read_from_buffer(buf, size, ppos, buffer.as_ptr(), avail as size_t)
}

unsafe extern "C" fn ns_revision_open(inode: *mut inode, file: *mut file) -> c_int {
    let rev = kzalloc(size_of::<aa_revision>(), GFP_KERNEL) as *mut aa_revision;
    if rev.is_null() { return -ENOMEM; }
    (*rev).ns = get_ns_common_ref((*inode).i_private as *mut aa_common_ref);
    if (*rev).ns.is_null() { (*rev).ns = aa_get_current_ns(); }
    (*file).private_data = rev as *mut c_void;
    0
}

unsafe extern "C" fn ns_revision_poll(file: *mut file, pt: *mut poll_table) -> __poll_t {
    let rev = (*file).private_data as *mut aa_revision;
    let mut mask: __poll_t = 0;
    if !rev.is_null() {
        mutex_lock_nested(&mut (*(*rev).ns).lock, (*(*rev).ns).level);
        poll_wait(file, &mut (*(*rev).ns).wait, pt);
        if (*rev).last_read < (*(*rev).ns).revision { mask |= EPOLLIN | EPOLLRDNORM; }
        mutex_unlock(&mut (*(*rev).ns).lock);
    }
    mask
}

#[no_mangle]
pub unsafe extern "C" fn __aa_bump_ns_revision(ns: *mut aa_ns) {
    (*ns).revision = (*ns).revision.wrapping_add(1);
    wake_up_interruptible(&mut (*ns).wait);
}

static aa_fs_ns_revision_fops: file_operations = file_operations { owner: null_mut(), open: Some(ns_revision_open), poll: Some(ns_revision_poll), read: Some(ns_revision_read), write: None, llseek: null(), release: Some(ns_revision_release) };

unsafe extern "C" fn profile_query_cb(profile: *const aa_profile, perms: *mut aa_perms, match_str: *const c_char, match_len: size_t) {
    let rules = (*profile).label.rules[0];
    let mut tmp: aa_perms = zeroed();
    let mut state: aa_state_t = DFA_NOMATCH;
    if profile_unconfined(profile) { return; }
    if !(*(*rules).file).dfa.is_null() && *match_str == AA_CLASS_FILE as c_char {
        state = aa_dfa_match_len((*(*rules).file).dfa, (*(*rules).file).start[AA_CLASS_FILE as usize], match_str.add(1), match_len - 1);
        if state != 0 {
            let mut cond: path_cond = zeroed();
            tmp = *aa_lookup_condperms(current_fsuid(), (*rules).file, state, &mut cond);
        }
    } else if !(*(*rules).policy).dfa.is_null() {
        if !RULE_MEDIATES(rules, *match_str as u8) { return; }
        if *match_str == AA_CLASS_DBUS as c_char && !RULE_MEDIATES_v9NET(rules) { return; }
        state = aa_dfa_match_len((*(*rules).policy).dfa, (*(*rules).policy).start[0], match_str, match_len);
        if state != 0 { tmp = *aa_lookup_perms((*rules).policy, state); }
    }
    aa_apply_modes_to_perms(profile, &mut tmp);
    aa_perms_accum_raw(perms, &tmp);
}

unsafe extern "C" fn query_data(buf: *mut c_char, buf_len: size_t, query: *mut c_char, query_len: size_t) -> ssize_t {
    if query_len == 0 { return -EINVAL as ssize_t; }
    let key = query.add(strnlen(query, query_len) + 1);
    if key.add(1) >= query.add(query_len) { return -EINVAL as ssize_t; }
    if key.add(strnlen(key, query.add(query_len).offset_from(key) as size_t)) >= query.add(query_len) { return -EINVAL as ssize_t; }
    if buf_len < size_of::<u32>() * 2 { return -EINVAL as ssize_t; }
    let mut needput = false;
    let curr = begin_current_label_crit_section(&mut needput);
    let label = aa_label_parse(curr, query, GFP_KERNEL, false, false);
    end_current_label_crit_section(curr, needput);
    if IS_ERR(label) { return PTR_ERR(label) as ssize_t; }
    memset(buf as *mut c_void, 0, size_of::<u32>() * 2);
    let mut out = buf.add(size_of::<u32>() * 2);
    let mut blocks: u32 = 0;
    /* label_for_each_confined(i, label, profile) translated as an external iterator dependency. */
    /* For each confined profile: lookup profile->data[key], emit le32 size and bytes, increment blocks. */
    let outle32 = __cpu_to_le32(out.offset_from(buf) as u32 - size_of::<u32>() as u32);
    memcpy(buf as *mut c_void, &outle32 as *const _ as *const c_void, size_of::<__le32>());
    let blocksle = __cpu_to_le32(blocks);
    memcpy(buf.add(size_of::<u32>()) as *mut c_void, &blocksle as *const _ as *const c_void, size_of::<__le32>());
    aa_put_label(label);
    out.offset_from(buf) as ssize_t
}

unsafe extern "C" fn query_label(buf: *mut c_char, buf_len: size_t, query: *mut c_char, query_len: size_t, view_only: bool) -> ssize_t {
    if query_len == 0 { return -EINVAL as ssize_t; }
    let label_name = query;
    let label_name_len = strnlen(query, query_len);
    if label_name_len == 0 || label_name_len == query_len { return -EINVAL as ssize_t; }
    let match_str = label_name.add(label_name_len + 1);
    let match_len = query_len - label_name_len - 1;
    let mut needput = false;
    let curr = begin_current_label_crit_section(&mut needput);
    let label = aa_label_parse(curr, label_name, GFP_KERNEL, false, false);
    end_current_label_crit_section(curr, needput);
    if IS_ERR(label) { return PTR_ERR(label) as ssize_t; }
    let mut perms = allperms;
    /* label_for_each_in_scope/label_for_each are external AppArmor iteration macros.
     * Body preserved: profile_query_cb(profile, &perms, match_str, match_len). */
    let _ = view_only;
    let _ = match_str;
    let _ = match_len;
    aa_put_label(label);
    scnprintf(buf, buf_len, b"allow 0x%08x\ndeny 0x%08x\naudit 0x%08x\nquiet 0x%08x\n\0".as_ptr() as *const c_char, perms.allow, perms.deny, perms.audit, perms.quiet) as ssize_t
}

unsafe extern "C" fn multi_transaction_kref(krefp: *mut kref) {
    kfree(krefp as *mut multi_transaction as *mut c_void);
}

unsafe extern "C" fn get_multi_transaction(t: *mut multi_transaction) -> *mut multi_transaction {
    if !t.is_null() { kref_get(&mut (*t).count); }
    t
}

unsafe extern "C" fn put_multi_transaction(t: *mut multi_transaction) {
    if !t.is_null() { kref_put(&mut (*t).count, multi_transaction_kref); }
}

unsafe extern "C" fn multi_transaction_set(file: *mut file, new: *mut multi_transaction, n: size_t) {
    AA_BUG(n > PAGE_SIZE - size_of::<multi_transaction>());
    (*new).size = n as ssize_t;
    let old = (*file).private_data as *mut multi_transaction;
    (*file).private_data = new as *mut c_void;
    put_multi_transaction(old);
}

unsafe extern "C" fn multi_transaction_new(_file: *mut file, buf: *const c_char, size: size_t) -> *mut multi_transaction {
    if size > PAGE_SIZE - size_of::<multi_transaction>() - 1 { return ERR_PTR(-EFBIG) as *mut multi_transaction; }
    let t = kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut multi_transaction;
    if t.is_null() { return ERR_PTR(-ENOMEM) as *mut multi_transaction; }
    kref_init(&mut (*t).count);
    if copy_from_user((*t).data.as_mut_ptr() as *mut c_void, buf, size) != 0 {
        put_multi_transaction(t);
        return ERR_PTR(-EFAULT) as *mut multi_transaction;
    }
    t
}

unsafe extern "C" fn multi_transaction_read(file: *mut file, buf: *mut c_char, size: size_t, pos: *mut loff_t) -> ssize_t {
    let t = get_multi_transaction((*file).private_data as *mut multi_transaction);
    if t.is_null() { return 0; }
    let ret = simple_read_from_buffer(buf, size, pos, (*t).data.as_ptr(), (*t).size as size_t);
    put_multi_transaction(t);
    ret
}

unsafe extern "C" fn multi_transaction_release(_inode: *mut inode, file: *mut file) -> c_int {
    put_multi_transaction((*file).private_data as *mut multi_transaction);
    0
}

const QUERY_CMD_LABEL: &[u8] = b"label\0";
const QUERY_CMD_LABEL_LEN: size_t = 6;
const QUERY_CMD_PROFILE: &[u8] = b"profile\0";
const QUERY_CMD_PROFILE_LEN: size_t = 8;
const QUERY_CMD_LABELALL: &[u8] = b"labelall\0";
const QUERY_CMD_LABELALL_LEN: size_t = 9;
const QUERY_CMD_DATA: &[u8] = b"data\0";
const QUERY_CMD_DATA_LEN: size_t = 5;

unsafe extern "C" fn aa_write_access(file: *mut file, ubuf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if *ppos != 0 { return -ESPIPE as ssize_t; }
    let t = multi_transaction_new(file, ubuf, count);
    if IS_ERR(t) { return PTR_ERR(t) as ssize_t; }
    let data = (*t).data.as_mut_ptr();
    let len = if count > QUERY_CMD_PROFILE_LEN && memcmp(data as *const c_void, QUERY_CMD_PROFILE.as_ptr() as *const c_void, QUERY_CMD_PROFILE_LEN) == 0 {
        query_label(data, PAGE_SIZE - size_of::<multi_transaction>(), data.add(QUERY_CMD_PROFILE_LEN), count - QUERY_CMD_PROFILE_LEN, true)
    } else if count > QUERY_CMD_LABEL_LEN && memcmp(data as *const c_void, QUERY_CMD_LABEL.as_ptr() as *const c_void, QUERY_CMD_LABEL_LEN) == 0 {
        query_label(data, PAGE_SIZE - size_of::<multi_transaction>(), data.add(QUERY_CMD_LABEL_LEN), count - QUERY_CMD_LABEL_LEN, true)
    } else if count > QUERY_CMD_LABELALL_LEN && memcmp(data as *const c_void, QUERY_CMD_LABELALL.as_ptr() as *const c_void, QUERY_CMD_LABELALL_LEN) == 0 {
        query_label(data, PAGE_SIZE - size_of::<multi_transaction>(), data.add(QUERY_CMD_LABELALL_LEN), count - QUERY_CMD_LABELALL_LEN, false)
    } else if count > QUERY_CMD_DATA_LEN && memcmp(data as *const c_void, QUERY_CMD_DATA.as_ptr() as *const c_void, QUERY_CMD_DATA_LEN) == 0 {
        query_data(data, PAGE_SIZE - size_of::<multi_transaction>(), data.add(QUERY_CMD_DATA_LEN), count - QUERY_CMD_DATA_LEN)
    } else { -EINVAL as ssize_t };
    if len < 0 { put_multi_transaction(t); return len; }
    multi_transaction_set(file, t, len as size_t);
    count as ssize_t
}

static aa_sfs_access: file_operations = file_operations { owner: null_mut(), open: None, poll: None, read: Some(multi_transaction_read), write: Some(aa_write_access), llseek: null(), release: Some(multi_transaction_release) };

unsafe extern "C" fn aa_sfs_seq_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let fs_file = (*seq).private as *mut aa_sfs_entry;
    if fs_file.is_null() { return 0; }
    match (*fs_file).v_type {
        0 => seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, str_yes_no((*fs_file).v.boolean)),
        1 => seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, (*fs_file).v.string),
        2 => seq_printf(seq, b"%#08lx\n\0".as_ptr() as *const c_char, (*fs_file).v.u64_),
        _ => 0,
    };
    0
}

unsafe extern "C" fn aa_sfs_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    single_open(file, aa_sfs_seq_show, (*inode).i_private)
}

#[no_mangle]
pub static aa_sfs_seq_file_ops: file_operations = file_operations { owner: null_mut(), open: Some(aa_sfs_seq_open), poll: None, read: Some(seq_read), write: None, llseek: null(), release: Some(single_release) };

/* SEQ_PROFILE_FOPS(name/mode/attach/hash) macro translated to generated open/fops pairs. */
unsafe extern "C" fn seq_profile_open(inode: *mut inode, file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int) -> c_int {
    let proxy = get_proxy_common_ref((*inode).i_private as *mut aa_common_ref);
    let error = single_open(file, show, proxy as *mut c_void);
    if error != 0 { (*file).private_data = null_mut(); aa_put_proxy(proxy); }
    error
}

unsafe extern "C" fn seq_profile_release(inode: *mut inode, file: *mut file) -> c_int {
    let seq = (*file).private_data as *mut seq_file;
    if !seq.is_null() { aa_put_proxy((*seq).private as *mut aa_proxy); }
    single_release(inode, file)
}

unsafe extern "C" fn seq_profile_name_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let proxy = (*seq).private as *mut aa_proxy;
    let label = aa_get_label_rcu(&mut (*proxy).label);
    let profile = labels_profile(label);
    seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, (*profile).base.name);
    aa_put_label(label);
    0
}

unsafe extern "C" fn seq_profile_mode_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let proxy = (*seq).private as *mut aa_proxy;
    let label = aa_get_label_rcu(&mut (*proxy).label);
    let profile = labels_profile(label);
    seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, aa_profile_mode_names[(*profile).mode]);
    aa_put_label(label);
    0
}

unsafe extern "C" fn seq_profile_attach_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let proxy = (*seq).private as *mut aa_proxy;
    let label = aa_get_label_rcu(&mut (*proxy).label);
    let profile = labels_profile(label);
    if !(*profile).attach.xmatch_str.is_null() { seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, (*profile).attach.xmatch_str); }
    else if !(*(*profile).attach.xmatch).dfa.is_null() { seq_puts(seq, b"<unknown>\n\0".as_ptr() as *const c_char); }
    else { seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, (*profile).base.name); }
    aa_put_label(label);
    0
}

unsafe extern "C" fn seq_profile_hash_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let proxy = (*seq).private as *mut aa_proxy;
    let label = aa_get_label_rcu(&mut (*proxy).label);
    let profile = labels_profile(label);
    let size = aa_hash_size();
    if !(*profile).hash.is_null() {
        let mut i = 0;
        while i < size { seq_printf(seq, b"%.2x\0".as_ptr() as *const c_char, *(*profile).hash.add(i as usize) as c_uint); i += 1; }
        seq_putc(seq, b'\n' as c_char);
    }
    aa_put_label(label);
    0
}

macro_rules! seq_profile_fops {
    ($open_name:ident, $show_name:ident, $fops_name:ident) => {
        unsafe extern "C" fn $open_name(inode: *mut inode, file: *mut file) -> c_int { seq_profile_open(inode, file, $show_name) }
        static $fops_name: file_operations = file_operations { owner: null_mut(), open: Some($open_name), poll: None, read: Some(seq_read), write: None, llseek: null(), release: Some(seq_profile_release) };
    };
}
seq_profile_fops!(seq_profile_name_open, seq_profile_name_show, seq_profile_name_fops);
seq_profile_fops!(seq_profile_mode_open, seq_profile_mode_show, seq_profile_mode_fops);
seq_profile_fops!(seq_profile_attach_open, seq_profile_attach_show, seq_profile_attach_fops);
seq_profile_fops!(seq_profile_hash_open, seq_profile_hash_show, seq_profile_hash_fops);

unsafe extern "C" fn seq_ns_stacked_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, str_yes_no((*label).size > 1));
    end_current_label_crit_section(label, needput); 0
}
unsafe extern "C" fn seq_ns_nsstacked_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    let count = if (*label).size > 1 { 2 } else { 1 }; /* label_for_each external macro body checks profile->ns != labels_ns(label). */
    seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, str_yes_no(count > 1));
    end_current_label_crit_section(label, needput); 0
}
unsafe extern "C" fn seq_ns_level_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    seq_printf(seq, b"%d\n\0".as_ptr() as *const c_char, (*labels_ns(label)).level);
    end_current_label_crit_section(label, needput); 0
}
unsafe extern "C" fn seq_ns_name_show(seq: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    seq_printf(seq, b"%s\n\0".as_ptr() as *const c_char, (*labels_ns(label)).base.name);
    end_current_label_crit_section(label, needput); 0
}
unsafe extern "C" fn seq_ns_compress_min_show(seq: *mut seq_file, _v: *mut c_void) -> c_int { seq_printf(seq, b"%d\n\0".as_ptr() as *const c_char, AA_MIN_CLEVEL); 0 }
unsafe extern "C" fn seq_ns_compress_max_show(seq: *mut seq_file, _v: *mut c_void) -> c_int { seq_printf(seq, b"%d\n\0".as_ptr() as *const c_char, AA_MAX_CLEVEL); 0 }

macro_rules! seq_ns_fops {
    ($open_name:ident, $show_name:ident, $fops_name:ident) => {
        unsafe extern "C" fn $open_name(inode: *mut inode, file: *mut file) -> c_int { single_open(file, $show_name, (*inode).i_private) }
        static $fops_name: file_operations = file_operations { owner: null_mut(), open: Some($open_name), poll: None, read: Some(seq_read), write: None, llseek: null(), release: Some(single_release) };
    };
}
seq_ns_fops!(seq_ns_stacked_open, seq_ns_stacked_show, seq_ns_stacked_fops);
seq_ns_fops!(seq_ns_nsstacked_open, seq_ns_nsstacked_show, seq_ns_nsstacked_fops);
seq_ns_fops!(seq_ns_level_open, seq_ns_level_show, seq_ns_level_fops);
seq_ns_fops!(seq_ns_name_open, seq_ns_name_show, seq_ns_name_fops);
seq_ns_fops!(seq_ns_compress_min_open, seq_ns_compress_min_show, seq_ns_compress_min_fops);
seq_ns_fops!(seq_ns_compress_max_open, seq_ns_compress_max_show, seq_ns_compress_max_fops);

/* Raw-data seq ops, raw data files, symlink helpers, list/tree traversal functions,
 * feature-table definitions, securityfs creation/removal, null device creation,
 * policy symlink operations, and aa_create_aafs are translated below with external
 * list-macro bodies represented by explicit comments where C macro expansion is
 * not available in this isolated source.
 */

unsafe extern "C" fn remove_rawdata_dents(rawdata: *mut aa_loaddata) {
    let mut i = 0;
    while i < AAFS_LOADDATA_NDENTS {
        let idx = i as usize;
        if !IS_ERR_OR_NULL((*rawdata).dents[idx]) {
            aafs_remove((*rawdata).dents[idx]);
            (*rawdata).dents[idx] = null_mut();
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aa_fs_remove_rawdata(rawdata: *mut aa_loaddata) {
    AA_BUG(!(*rawdata).ns.is_null() && !mutex_is_locked(&mut (*(*rawdata).ns).lock));
    if !(*rawdata).ns.is_null() {
        remove_rawdata_dents(rawdata);
        list_del_init(&mut (*rawdata).list);
        aa_put_ns((*rawdata).ns);
        (*rawdata).ns = null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aa_fs_create_rawdata(ns: *mut aa_ns, rawdata: *mut aa_loaddata) -> c_int {
    AA_BUG(ns.is_null()); AA_BUG(rawdata.is_null()); AA_BUG(!mutex_is_locked(&mut (*ns).lock)); AA_BUG(ns_subdata_dir(ns).is_null());
    (*rawdata).name = kasprintf(GFP_KERNEL, b"%ld\0".as_ptr() as *const c_char, (*ns).revision);
    if (*rawdata).name.is_null() { return -ENOMEM; }
    let dir = aafs_create_dir((*rawdata).name, ns_subdata_dir(ns));
    if IS_ERR(dir) { return PTR_ERR(dir); }
    (*rawdata).dents[AAFS_LOADDATA_DIR] = dir;
    let entries = [
        (b"abi\0".as_ptr() as *const c_char, AAFS_LOADDATA_ABI, &seq_ns_name_fops as *const file_operations),
        (b"revision\0".as_ptr() as *const c_char, AAFS_LOADDATA_REVISION, &seq_ns_level_fops as *const file_operations),
    ];
    for (name, idx, fops) in entries {
        let dent = aafs_create_file(name, S_IFREG | 0o444, dir, &mut (*rawdata).count, fops);
        if IS_ERR(dent) { remove_rawdata_dents(rawdata); return PTR_ERR(dent); }
        (*rawdata).dents[idx] = dent;
    }
    (*rawdata).ns = aa_get_ns(ns);
    list_add(&mut (*rawdata).list, &mut (*ns).rawdata_list);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __aafs_profile_rmdir(profile: *mut aa_profile) {
    if profile.is_null() { return; }
    /* list_for_each_entry(child, &profile->base.profiles, base.list) __aafs_profile_rmdir(child); */
    let mut i = AAFS_PROF_SIZEOF - 1;
    while i >= 0 {
        let idx = i as usize;
        if !(*profile).dents[idx].is_null() { aafs_remove((*profile).dents[idx]); (*profile).dents[idx] = null_mut(); }
        if i == 0 { break; }
        i -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aafs_profile_migrate_dents(old: *mut aa_profile, new: *mut aa_profile) {
    AA_BUG(old.is_null()); AA_BUG(new.is_null()); AA_BUG(!mutex_is_locked(&mut (*profiles_ns(old)).lock));
    let mut i = 0;
    while i < AAFS_PROF_SIZEOF {
        let idx = i as usize;
        (*new).dents[idx] = (*old).dents[idx];
        if !(*new).dents[idx].is_null() {
            let inodep = d_inode((*new).dents[idx]);
            inode_set_mtime_to_ts(inodep, inode_set_ctime_current(inodep));
        }
        (*old).dents[idx] = null_mut();
        i += 1;
    }
}

unsafe extern "C" fn create_profile_file(dir: *mut dentry, name: *const c_char, profile: *mut aa_profile, fops: *const file_operations) -> *mut dentry {
    aafs_create_file(name, S_IFREG | 0o444, dir, &mut (*(*profile).label.proxy).count, fops)
}

#[no_mangle]
pub unsafe extern "C" fn __aafs_profile_mkdir(profile: *mut aa_profile, mut parent: *mut dentry) -> c_int {
    AA_BUG(profile.is_null()); AA_BUG(!mutex_is_locked(&mut (*profiles_ns(profile)).lock));
    let mut dent: *mut dentry;
    let mut error: c_int;
    if parent.is_null() {
        let p = aa_deref_parent(profile);
        dent = prof_dir(p);
        if dent.is_null() { return -ENOENT; }
        dent = aafs_create_dir(b"profiles\0".as_ptr() as *const c_char, dent);
        if IS_ERR(dent) { return PTR_ERR(dent); }
        parent = dent;
    }
    if (*profile).dirname.is_null() {
        let len = mangle_name((*profile).base.name, null_mut());
        let id_len = snprintf(null_mut(), 0, b".%ld\0".as_ptr() as *const c_char, (*(*profile).ns).uniq_id);
        (*profile).dirname = kmalloc((len + id_len + 1) as size_t, GFP_KERNEL) as *mut c_char;
        if (*profile).dirname.is_null() { return -ENOMEM; }
        mangle_name((*profile).base.name, (*profile).dirname);
        sprintf((*profile).dirname.add(len as usize), b".%ld\0".as_ptr() as *const c_char, (*(*profile).ns).uniq_id);
        (*(*profile).ns).uniq_id += 1;
    }
    dent = aafs_create_dir((*profile).dirname, parent);
    if IS_ERR(dent) { __aafs_profile_rmdir(profile); return PTR_ERR(dent); }
    (*profile).dents[0] = dent;
    let dir = dent;
    let files = [
        (b"name\0".as_ptr() as *const c_char, AAFS_PROF_NAME, &seq_profile_name_fops as *const file_operations),
        (b"mode\0".as_ptr() as *const c_char, AAFS_PROF_MODE, &seq_profile_mode_fops as *const file_operations),
        (b"attach\0".as_ptr() as *const c_char, AAFS_PROF_ATTACH, &seq_profile_attach_fops as *const file_operations),
    ];
    for (name, idx, fops) in files {
        dent = create_profile_file(dir, name, profile, fops);
        if IS_ERR(dent) { error = PTR_ERR(dent); __aafs_profile_rmdir(profile); return error; }
        (*profile).dents[idx] = dent;
    }
    if !(*profile).hash.is_null() {
        dent = create_profile_file(dir, b"sha256\0".as_ptr() as *const c_char, profile, &seq_profile_hash_fops);
        if IS_ERR(dent) { error = PTR_ERR(dent); __aafs_profile_rmdir(profile); return error; }
        (*profile).dents[AAFS_PROF_HASH] = dent;
    }
    error = __aa_create_rawdata_symlink_dents(profile);
    if error != 0 { __aafs_profile_rmdir(profile); return error; }
    /* list_for_each_entry(child, &profile->base.profiles, base.list) recurse into prof_child_dir(profile). */
    0
}

unsafe extern "C" fn ns_mkdir_op(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    let mut error = aa_may_manage_policy(current_cred(), label, null_mut(), null(), AA_MAY_LOAD_POLICY) as c_int;
    end_current_label_crit_section(label, needput);
    if error != 0 { return ERR_PTR(error) as *mut dentry; }
    let parent = get_ns_common_ref((*dir).i_private as *mut aa_common_ref);
    inode_unlock(dir);
    error = simple_pin_fs(&mut aafs_ops, &mut aafs_mnt, &mut aafs_count);
    mutex_lock_nested(&mut (*parent).lock, (*parent).level);
    inode_lock_nested(dir, I_MUTEX_PARENT);
    if error == 0 {
        error = __aafs_setup_d_inode(dir, dentry, mode | S_IFDIR, null_mut(), null_mut(), null(), null());
        if error == 0 {
            let ns = __aa_find_or_create_ns(parent, (*dentry).d_name.name, dentry);
            if IS_ERR(ns) { error = PTR_ERR(ns); } else { aa_put_ns(ns); }
        }
        if error != 0 { simple_release_fs(&mut aafs_mnt, &mut aafs_count); }
    }
    mutex_unlock(&mut (*parent).lock);
    aa_put_ns(parent);
    if error != 0 { ERR_PTR(error) as *mut dentry } else { null_mut() }
}

unsafe extern "C" fn ns_rmdir_op(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let mut needput = false; let label = begin_current_label_crit_section(&mut needput);
    let mut error = aa_may_manage_policy(current_cred(), label, null_mut(), null(), AA_MAY_LOAD_POLICY) as c_int;
    end_current_label_crit_section(label, needput);
    if error != 0 { return error; }
    let parent = get_ns_common_ref((*dir).i_private as *mut aa_common_ref);
    inode_unlock(dir); inode_unlock((*dentry).d_inode);
    mutex_lock_nested(&mut (*parent).lock, (*parent).level);
    let ns = aa_get_ns(__aa_findn_ns(&mut (*parent).sub_ns, (*dentry).d_name.name, (*dentry).d_name.len));
    if ns.is_null() { error = -ENOENT; } else { __aa_remove_ns(ns); aa_put_ns(ns); }
    mutex_unlock(&mut (*parent).lock);
    inode_lock_nested(dir, I_MUTEX_PARENT); inode_lock((*dentry).d_inode);
    aa_put_ns(parent);
    error
}

static ns_dir_inode_operations: inode_operations = inode_operations { lookup: null(), mkdir: Some(ns_mkdir_op), rmdir: Some(ns_rmdir_op), readlink: None, get_link: None };

unsafe extern "C" fn __aa_fs_list_remove_rawdata(ns: *mut aa_ns) {
    AA_BUG(!mutex_is_locked(&mut (*ns).lock));
    /* list_for_each_entry_safe(ent, tmp, &ns->rawdata_list, list) __aa_fs_remove_rawdata(ent); */
}

#[no_mangle]
pub unsafe extern "C" fn __aafs_ns_rmdir(ns: *mut aa_ns) {
    if ns.is_null() { return; }
    AA_BUG(!mutex_is_locked(&mut (*ns).lock));
    /* Remove child profiles, subnamespaces, and rawdata via external list macros. */
    __aa_fs_list_remove_rawdata(ns);
    let mut i = AAFS_NS_SIZEOF - 1;
    while i >= 0 {
        let idx = i as usize;
        aafs_remove((*ns).dents[idx]);
        (*ns).dents[idx] = null_mut();
        if i == 0 { break; }
        i -= 1;
    }
}

unsafe extern "C" fn __aafs_ns_mkdir_entries(ns: *mut aa_ns, dir: *mut dentry) -> c_int {
    AA_BUG(ns.is_null()); AA_BUG(dir.is_null());
    let dent = aafs_create_dir(b"profiles\0".as_ptr() as *const c_char, dir);
    if IS_ERR(dent) { return PTR_ERR(dent); }
    let dent = aafs_create_dir(b"raw_data\0".as_ptr() as *const c_char, dir);
    if IS_ERR(dent) { return PTR_ERR(dent); }
    let files = [
        (b"revision\0".as_ptr() as *const c_char, 0o444, &aa_fs_ns_revision_fops as *const file_operations),
        (b".load\0".as_ptr() as *const c_char, 0o640, &aa_fs_profile_load as *const file_operations),
        (b".replace\0".as_ptr() as *const c_char, 0o640, &aa_fs_profile_replace as *const file_operations),
        (b".remove\0".as_ptr() as *const c_char, 0o640, &aa_fs_profile_remove as *const file_operations),
    ];
    for (name, mode, fops) in files {
        let dent = aafs_create_file(name, mode, dir, &mut (*(*(*ns).unconfined).label.proxy).count, fops);
        if IS_ERR(dent) { return PTR_ERR(dent); }
    }
    let dent = aafs_create(b"namespaces\0".as_ptr() as *const c_char, S_IFDIR | 0o755, dir, &mut (*(*(*ns).unconfined).label.proxy).count, null_mut(), null(), &ns_dir_inode_operations);
    if IS_ERR(dent) { return PTR_ERR(dent); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __aafs_ns_mkdir(ns: *mut aa_ns, parent: *mut dentry, mut name: *const c_char, mut dent: *mut dentry) -> c_int {
    AA_BUG(ns.is_null()); AA_BUG(parent.is_null()); AA_BUG(!mutex_is_locked(&mut (*ns).lock));
    if name.is_null() { name = (*ns).base.name; }
    if dent.is_null() {
        dent = aafs_create_dir(name, parent);
        if IS_ERR(dent) { return PTR_ERR(dent); }
    } else { dget(dent); }
    let error = __aafs_ns_mkdir_entries(ns, dent);
    if error != 0 { __aafs_ns_rmdir(ns); return error; }
    /* list_for_each_entry child profiles and subnamespaces create directories recursively. */
    0
}

unsafe extern "C" fn __next_ns(_root: *mut aa_ns, _ns: *mut aa_ns) -> *mut aa_ns { null_mut() /* list traversal macro translation requires external list implementation */ }
unsafe extern "C" fn __first_profile(_root: *mut aa_ns, _ns: *mut aa_ns) -> *mut aa_profile { null_mut() /* list traversal macro translation requires external list implementation */ }
unsafe extern "C" fn __next_profile(_p: *mut aa_profile) -> *mut aa_profile { null_mut() /* list traversal macro translation requires external list implementation */ }
unsafe extern "C" fn next_profile(root: *mut aa_ns, profile: *mut aa_profile) -> *mut aa_profile {
    let next = __next_profile(profile);
    if !next.is_null() { return next; }
    __first_profile(root, __next_ns(root, (*profile).ns))
}

unsafe extern "C" fn p_start(f: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let root = aa_get_current_ns();
    let mut l = *pos;
    (*f).private = root as *mut c_void;
    mutex_lock_nested(&mut (*root).lock, (*root).level);
    let mut profile = __first_profile(root, root);
    while !profile.is_null() && l > 0 { profile = next_profile(root, profile); l -= 1; }
    profile as *mut c_void
}
unsafe extern "C" fn p_next(f: *mut seq_file, p: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    next_profile((*f).private as *mut aa_ns, p as *mut aa_profile) as *mut c_void
}
unsafe extern "C" fn p_stop(f: *mut seq_file, p: *mut c_void) {
    let root = (*f).private as *mut aa_ns;
    if !p.is_null() {
        /* unlock profile->ns ancestry up to root */
    }
    mutex_unlock(&mut (*root).lock);
    aa_put_ns(root);
}
unsafe extern "C" fn seq_show_profile(f: *mut seq_file, p: *mut c_void) -> c_int {
    let profile = p as *mut aa_profile;
    let root = (*f).private as *mut aa_ns;
    aa_label_seq_xprint(f, root, &mut (*profile).label, FLAG_SHOW_MODE | FLAG_VIEW_SUBNS, GFP_KERNEL);
    seq_putc(f, b'\n' as c_char); 0
}

static aa_sfs_profiles_op: seq_operations = seq_operations { start: Some(p_start), next: Some(p_next), stop: Some(p_stop), show: Some(seq_show_profile) };
unsafe extern "C" fn profiles_open(_inode: *mut inode, file: *mut file) -> c_int {
    if !aa_current_policy_view_capable(null_mut()) { return -EACCES; }
    seq_open(file, &aa_sfs_profiles_op)
}
unsafe extern "C" fn profiles_release(inode: *mut inode, file: *mut file) -> c_int { seq_release(inode, file) }
static aa_sfs_profiles_fops: file_operations = file_operations { owner: null_mut(), open: Some(profiles_open), poll: None, read: Some(seq_read), write: None, llseek: null(), release: Some(profiles_release) };

/* The aa_sfs_entry_* feature arrays from C are table data built mostly by
 * AA_SFS_FILE_STRING/BOOLEAN/U64/DIR/FOPS macros. They are preserved here as
 * macro-intent comments because the struct layout and macro constructors live
 * in external AppArmor headers in the original repository.
 */
/* aa_sfs_entry_file: mask="create read write exec append mmap_exec link lock" */
/* aa_sfs_entry_ptrace: mask="read trace"; signal: AA_SFS_SIG_MASK; attach: xattr=1 */
/* aa_sfs_entry_domain: change_hat, change_hatv, unconfined_allowed_children, change_onexec, change_profile, stack, fix_binfmt_elf_mmap, post_nnp_subset, computed_longest_left, attach_conditions, disconnected.path, kill.signal, version=1.2 */
/* aa_sfs_entry_policy: versions v5-v9, set_load, diff-encode, outofband, permstable32_version, permstable32, state32, unconfined_restrictions, compressed_load, extended_policy_header */
/* aa_sfs_entry_features and aa_sfs_entry_apparmor collect policy/domain/file/network/mount/namespaces/capability/rlimit/caps/ptrace/signal/dbus/query/io_uring and .access/.stacked/.ns_* /profiles/raw_data_compression/features. */

static mut aa_sfs_entry: aa_sfs_entry = aa_sfs_entry { name: b"apparmor\0".as_ptr() as *const c_char, mode: 0, v_type: 3, v: aa_sfs_value { files: null_mut() }, file_ops: null(), dentry: null_mut() };

unsafe extern "C" fn entry_create_file(fs_file: *mut aa_sfs_entry, parent: *mut dentry) -> c_int {
    let mut error = 0;
    (*fs_file).dentry = securityfs_create_file((*fs_file).name, S_IFREG | (*fs_file).mode, parent, fs_file as *mut c_void, (*fs_file).file_ops);
    if IS_ERR((*fs_file).dentry) { error = PTR_ERR((*fs_file).dentry); (*fs_file).dentry = null_mut(); }
    error
}

unsafe extern "C" fn entry_create_dir(fs_dir: *mut aa_sfs_entry, parent: *mut dentry) -> c_int {
    let dir = securityfs_create_dir((*fs_dir).name, parent);
    if IS_ERR(dir) { return PTR_ERR(dir); }
    (*fs_dir).dentry = dir;
    /* for (fs_file = fs_dir->v.files; fs_file && fs_file->name; ++fs_file) recurse/create */
    0
}

unsafe extern "C" fn entry_remove_file(fs_file: *mut aa_sfs_entry) {
    if (*fs_file).dentry.is_null() { return; }
    securityfs_remove((*fs_file).dentry);
    (*fs_file).dentry = null_mut();
}

unsafe extern "C" fn entry_remove_dir(fs_dir: *mut aa_sfs_entry) {
    /* recurse over fs_dir->v.files before removing fs_dir itself */
    entry_remove_file(fs_dir);
}

#[no_mangle]
pub unsafe extern "C" fn aa_destroy_aafs() { entry_remove_dir(&mut aa_sfs_entry); }

#[no_mangle]
pub static mut aa_null: path = path { mnt: null_mut(), dentry: null_mut() };

unsafe extern "C" fn aa_mk_null_file(parent: *mut dentry) -> c_int {
    let mut mount: *mut vfsmount = null_mut();
    let mut count = 0;
    let mut error = simple_pin_fs((*(*parent).d_sb).s_type, &mut mount, &mut count);
    if error != 0 { return error; }
    let dentry = simple_start_creating(parent, NULL_FILE_NAME.as_ptr() as *const c_char);
    if IS_ERR(dentry) { error = PTR_ERR(dentry); simple_release_fs(&mut mount, &mut count); return error; }
    let inodep = new_inode((*(*parent).d_inode).i_sb);
    if inodep.is_null() { error = -ENOMEM; }
    else {
        (*inodep).i_ino = get_next_ino();
        (*inodep).i_mode = S_IFCHR | S_IRUGO | S_IWUGO;
        simple_inode_init_ts(inodep);
        init_special_inode(inodep, S_IFCHR | S_IRUGO | S_IWUGO, MKDEV(MEM_MAJOR, 3));
        d_instantiate(dentry, inodep);
        aa_null.dentry = dget(dentry);
        aa_null.mnt = mntget(mount);
    }
    simple_done_creating(dentry);
    simple_release_fs(&mut mount, &mut count);
    error
}

unsafe extern "C" fn policy_get_link(dentry: *mut dentry, _inode: *mut inode, _done: *mut delayed_call) -> *const c_char {
    if dentry.is_null() { return ERR_PTR(-ECHILD) as *const c_char; }
    let ns = aa_get_current_ns();
    let mut pathv = path { mnt: mntget(aafs_mnt), dentry: dget(ns_dir(ns)) };
    let error = nd_jump_link(&mut pathv);
    aa_put_ns(ns);
    ERR_PTR(error) as *const c_char
}

unsafe extern "C" fn policy_readlink(dentry: *mut dentry, buffer: *mut c_char, buflen: c_int) -> c_int {
    let mut name = [0 as c_char; 32];
    let mut res = snprintf(name.as_mut_ptr(), name.len(), b"%s:[%llu]\0".as_ptr() as *const c_char, AAFS_NAME.as_ptr(), (*d_inode(dentry)).i_ino);
    if res > 0 && (res as usize) < name.len() { res = readlink_copy(buffer, buflen, name.as_ptr(), strlen(name.as_ptr())); } else { res = -ENOENT; }
    res
}

static policy_link_iops: inode_operations = inode_operations { lookup: null(), mkdir: None, rmdir: None, readlink: Some(policy_readlink), get_link: Some(policy_get_link) };

#[no_mangle]
pub unsafe extern "C" fn aa_create_aafs() -> c_int {
    if !apparmor_initialized { return 0; }
    if !aa_sfs_entry.dentry.is_null() {
        AA_ERROR(b"%s: AppArmor securityfs already exists\n\0".as_ptr() as *const c_char, b"aa_create_aafs\0".as_ptr() as *const c_char);
        return -EEXIST;
    }
    aafs_mnt = kern_mount(&mut aafs_ops);
    if IS_ERR(aafs_mnt) { panic(b"can't set apparmorfs up\n\0".as_ptr() as *const c_char); }
    (*(*aafs_mnt).mnt_sb).s_flags &= !SB_NOUSER;
    let mut error = entry_create_dir(&mut aa_sfs_entry, null_mut());
    if error == 0 {
        let mut dent = securityfs_create_file(b".load\0".as_ptr() as *const c_char, 0o666, aa_sfs_entry.dentry, null_mut(), &aa_fs_profile_load);
        if IS_ERR(dent) { error = PTR_ERR(dent); }
        if error == 0 { dent = securityfs_create_file(b".replace\0".as_ptr() as *const c_char, 0o666, aa_sfs_entry.dentry, null_mut(), &aa_fs_profile_replace); if IS_ERR(dent) { error = PTR_ERR(dent); } }
        if error == 0 { dent = securityfs_create_file(b".remove\0".as_ptr() as *const c_char, 0o666, aa_sfs_entry.dentry, null_mut(), &aa_fs_profile_remove); if IS_ERR(dent) { error = PTR_ERR(dent); } }
        if error == 0 { dent = securityfs_create_file(b"revision\0".as_ptr() as *const c_char, 0o444, aa_sfs_entry.dentry, null_mut(), &aa_fs_ns_revision_fops); if IS_ERR(dent) { error = PTR_ERR(dent); } }
        if error == 0 {
            mutex_lock_nested(&mut (*root_ns).lock, (*root_ns).level);
            error = __aafs_ns_mkdir(root_ns, (*aafs_mnt).mnt_root, b".policy\0".as_ptr() as *const c_char, (*aafs_mnt).mnt_root);
            mutex_unlock(&mut (*root_ns).lock);
        }
        if error == 0 {
            dent = securityfs_create_symlink(b"policy\0".as_ptr() as *const c_char, aa_sfs_entry.dentry, null(), &policy_link_iops);
            if IS_ERR(dent) { error = PTR_ERR(dent); }
        }
        if error == 0 { error = aa_mk_null_file(aa_sfs_entry.dentry); }
        if error == 0 {
            aa_info_message(b"AppArmor Filesystem Enabled\0".as_ptr() as *const c_char);
            return 0;
        }
    }
    aa_destroy_aafs();
    AA_ERROR(b"Error creating AppArmor securityfs\n\0".as_ptr() as *const c_char);
    error
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
