/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/dcache.h. External kernel types and functions are
// intentionally referenced but not implemented here.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Build-time configuration from the C header is preserved by these aliases.
#[cfg(target_endian = "little")]
pub type HashLen = (u32, u32); // hash, len
#[cfg(not(target_endian = "little"))]
pub type HashLen = (u32, u32); // len, hash

#[repr(C)]
pub union QstrHashLen { pub fields: HashLen, pub hash_len: u64 }

#[repr(C)]
pub struct qstr { pub hash_len: QstrHashLen, pub name: *const u8 }

#[cfg(target_pointer_width = "64")]
pub const DNAME_INLINE_WORDS: usize = 5;
#[cfg(all(target_pointer_width = "32", feature = "smp"))]
pub const DNAME_INLINE_WORDS: usize = 9;
#[cfg(all(target_pointer_width = "32", not(feature = "smp")))]
pub const DNAME_INLINE_WORDS: usize = 11;
pub const DNAME_INLINE_LEN: usize = DNAME_INLINE_WORDS * core::mem::size_of::<usize>();

#[repr(C)]
pub union shortname_store {
    pub string: [u8; DNAME_INLINE_LEN],
    pub words: [usize; DNAME_INLINE_WORDS],
}

#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_bl_node { pub next: *mut hlist_bl_node, pub pprev: *mut *mut hlist_bl_node }
#[repr(C)] pub struct lockref { pub lock: c_ulong, pub count: c_int }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct seqcount_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct seqlock_t { _private: [u8; 0] }
#[repr(C)] pub struct completion_list { _private: [u8; 0] }

#[repr(C)]
pub struct dentry {
    pub d_flags: c_uint, pub d_seq: seqcount_spinlock_t, pub d_hash: hlist_bl_node,
    pub d_parent: *mut dentry, pub d_name: qstr, pub d_inode: *mut inode,
    pub d_shortname: shortname_store, pub d_op: *const dentry_operations,
    pub d_sb: *mut super_block, pub d_time: c_ulong, pub d_fsdata: *mut c_void,
    pub d_lockref: lockref, pub d_lru: list_head, pub d_sib: hlist_node,
    pub d_children: hlist_head, pub d_alias: DentryAlias,
}

#[repr(C)] pub union DentryAlias { pub d_alias: hlist_node, pub d_in_lookup_hash: hlist_bl_node, pub d_rcu: rcu_head, pub waiters: *mut completion_list }

#[repr(C)] #[derive(Copy, Clone)] pub enum dentry_d_lock_class { DENTRY_D_LOCK_NORMAL, DENTRY_D_LOCK_NESTED }
#[repr(C)] #[derive(Copy, Clone)] pub enum d_real_type { D_REAL_DATA, D_REAL_METADATA }

#[repr(C)]
pub struct dentry_operations {
    pub d_revalidate: Option<unsafe extern "C" fn(*mut inode, *const qstr, *mut dentry, c_uint) -> c_int>,
    pub d_weak_revalidate: Option<unsafe extern "C" fn(*mut dentry, c_uint) -> c_int>,
    pub d_hash: Option<unsafe extern "C" fn(*const dentry, *mut qstr) -> c_int>,
    pub d_compare: Option<unsafe extern "C" fn(*const dentry, c_uint, *const c_char, *const qstr) -> c_int>,
    pub d_delete: Option<unsafe extern "C" fn(*const dentry) -> c_int>, pub d_init: Option<unsafe extern "C" fn(*mut dentry) -> c_int>,
    pub d_release: Option<unsafe extern "C" fn(*mut dentry)>, pub d_prune: Option<unsafe extern "C" fn(*mut dentry)>,
    pub d_iput: Option<unsafe extern "C" fn(*mut dentry, *mut inode)>, pub d_dname: Option<unsafe extern "C" fn(*mut dentry, *mut c_char, c_int) -> *mut c_char>,
    pub d_automount: Option<unsafe extern "C" fn(*mut path) -> *mut vfsmount>, pub d_manage: Option<unsafe extern "C" fn(*const path, bool) -> c_int>,
    pub d_real: Option<unsafe extern "C" fn(*mut dentry, d_real_type) -> *mut dentry>, pub d_unalias_trylock: Option<unsafe extern "C" fn(*const dentry) -> bool>,
    pub d_unalias_unlock: Option<unsafe extern "C" fn(*const dentry)>,
}

pub const DCACHE_OP_HASH: c_uint = 1 << 0; pub const DCACHE_OP_COMPARE: c_uint = 1 << 1; pub const DCACHE_OP_REVALIDATE: c_uint = 1 << 2; pub const DCACHE_OP_DELETE: c_uint = 1 << 3;
pub const DCACHE_OP_PRUNE: c_uint = 1 << 4; pub const DCACHE_DISCONNECTED: c_uint = 1 << 5; pub const DCACHE_REFERENCED: c_uint = 1 << 6; pub const DCACHE_DONTCACHE: c_uint = 1 << 7;
pub const DCACHE_CANT_MOUNT: c_uint = 1 << 8; pub const DCACHE_LOOKUP_WAITERS: c_uint = 1 << 9; pub const DCACHE_SHRINK_LIST: c_uint = 1 << 10; pub const DCACHE_OP_WEAK_REVALIDATE: c_uint = 1 << 11;
pub const DCACHE_NFSFS_RENAMED: c_uint = 1 << 12; pub const DCACHE_FSNOTIFY_PARENT_WATCHED: c_uint = 1 << 13; pub const DCACHE_DENTRY_KILLED: c_uint = 1 << 14; pub const DCACHE_MOUNTED: c_uint = 1 << 15;
pub const DCACHE_NEED_AUTOMOUNT: c_uint = 1 << 16; pub const DCACHE_MANAGE_TRANSIT: c_uint = 1 << 17; pub const DCACHE_LRU_LIST: c_uint = 1 << 18; pub const DCACHE_ENTRY_TYPE: c_uint = 7 << 19;
pub const DCACHE_MISS_TYPE: c_uint = 0; pub const DCACHE_WHITEOUT_TYPE: c_uint = 1 << 19; pub const DCACHE_DIRECTORY_TYPE: c_uint = 2 << 19; pub const DCACHE_AUTODIR_TYPE: c_uint = 3 << 19;
pub const DCACHE_REGULAR_TYPE: c_uint = 4 << 19; pub const DCACHE_SPECIAL_TYPE: c_uint = 5 << 19; pub const DCACHE_SYMLINK_TYPE: c_uint = 6 << 19; pub const DCACHE_NOKEY_NAME: c_uint = 1 << 22;
pub const DCACHE_OP_REAL: c_uint = 1 << 23; pub const DCACHE_PAR_LOOKUP: c_uint = 1 << 24; pub const DCACHE_DENTRY_CURSOR: c_uint = 1 << 25; pub const DCACHE_NORCU: c_uint = 1 << 26; pub const DCACHE_PERSISTENT: c_uint = 1 << 27;
pub const DCACHE_MANAGED_DENTRY: c_uint = DCACHE_MOUNTED | DCACHE_NEED_AUTOMOUNT | DCACHE_MANAGE_TRANSIT;

#[repr(C)] pub struct name_snapshot { pub name: qstr, pub inline_name: shortname_store }

extern "C" {
    pub static empty_name: qstr; pub static slash_name: qstr; pub static dotdot_name: qstr; pub static mut rename_lock: seqlock_t;
    pub fn d_instantiate(*mut dentry, *mut inode); pub fn d_instantiate_new(*mut dentry, *mut inode); pub fn __d_drop(*mut dentry); pub fn d_drop(*mut dentry); pub fn d_delete(*mut dentry);
    pub fn d_alloc(*mut dentry, *const qstr) -> *mut dentry; pub fn d_alloc_anon(*mut super_block) -> *mut dentry; pub fn d_alloc_parallel(*mut dentry, *const qstr) -> *mut dentry; pub fn d_splice_alias(*mut inode, *mut dentry) -> *mut dentry; pub fn d_splice_alias_ops(*mut inode, *mut dentry, *const dentry_operations) -> *mut dentry; pub fn d_add_ci(*mut dentry, *mut inode, *mut qstr) -> *mut dentry;
    pub fn d_same_name(*const dentry, *const dentry, *const qstr) -> bool; pub fn d_find_any_alias(*mut inode) -> *mut dentry; pub fn d_obtain_alias(*mut inode) -> *mut dentry; pub fn d_obtain_root(*mut inode) -> *mut dentry; pub fn shrink_dcache_sb(*mut super_block); pub fn shrink_dcache_parent(*mut dentry); pub fn d_invalidate(*mut dentry);
    pub fn d_make_root(*mut inode) -> *mut dentry; pub fn d_mark_tmpfile(*mut file, *mut inode); pub fn d_mark_tmpfile_name(*mut file, *const qstr) -> c_int; pub fn d_tmpfile(*mut file, *mut inode);
    pub fn d_find_alias(*mut inode) -> *mut dentry; pub fn d_prune_aliases(*mut inode); pub fn __move_to_shrink_list(*mut dentry, *mut list_head) -> bool; pub fn shrink_dentry_list(*mut list_head); pub fn d_find_alias_rcu(*mut inode) -> *mut dentry; pub fn path_has_submounts(*const path) -> c_int; pub fn d_rehash(*mut dentry); pub fn d_add(*mut dentry, *mut inode); pub fn d_move(*mut dentry, *mut dentry); pub fn d_exchange(*mut dentry, *mut dentry); pub fn d_ancestor(*mut dentry, *mut dentry) -> *mut dentry; pub fn d_lookup(*const dentry, *const qstr) -> *mut dentry;
    pub fn d_parent_ino(*mut dentry) -> u64; pub fn dynamic_dname(*mut c_char, c_int, *const c_char, ...) -> *mut c_char; pub fn __d_path(*const path, *const path, *mut c_char, c_int) -> *mut c_char; pub fn d_absolute_path(*const path, *mut c_char, c_int) -> *mut c_char; pub fn d_path(*const path, *mut c_char, c_int) -> *mut c_char; pub fn dentry_path_raw(*const dentry, *mut c_char, c_int) -> *mut c_char; pub fn dentry_path(*const dentry, *mut c_char, c_int) -> *mut c_char;
    pub fn dget_parent(*mut dentry) -> *mut dentry; pub fn __d_lookup_unhash_wake(*mut dentry); pub fn dput(*mut dentry); pub fn vfs_pressure_ratio(c_ulong) -> c_ulong; pub fn take_dentry_name_snapshot(*mut name_snapshot, *mut dentry); pub fn release_dentry_name_snapshot(*mut name_snapshot); pub fn set_default_d_op(*mut super_block, *const dentry_operations); pub fn d_make_persistent(*mut dentry, *mut inode) -> *mut dentry; pub fn d_make_discardable(*mut dentry);
}

#[inline] pub unsafe fn IS_ROOT(x: *const dentry) -> bool { (*x).d_parent == x as *mut dentry }
#[inline] pub unsafe fn d_count(x: *const dentry) -> c_int { (*x).d_lockref.count }
#[inline] pub unsafe fn dget_dlock(x: *mut dentry) -> *mut dentry { (*x).d_lockref.count = (*x).d_lockref.count.wrapping_add(1); x }
#[inline] pub unsafe fn dget(x: *mut dentry) -> *mut dentry { if !x.is_null() { lockref_get(&mut (*x).d_lockref); } x }
#[inline] pub unsafe fn d_unhashed(_: *const dentry) -> c_int { 0 }
#[inline] pub unsafe fn d_unlinked(x: *const dentry) -> bool { d_unhashed(x) != 0 && !IS_ROOT(x) }
#[inline] pub unsafe fn cant_mount(x: *const dentry) -> c_int { ((*x).d_flags & DCACHE_CANT_MOUNT) as c_int }
#[inline] pub unsafe fn d_in_lookup(x: *const dentry) -> c_int { ((*x).d_flags & DCACHE_PAR_LOOKUP) as c_int }
#[inline] pub unsafe fn d_lookup_done(x: *mut dentry) { if d_in_lookup(x) != 0 { __d_lookup_unhash_wake(x) } }
#[inline] pub unsafe fn d_managed(x: *const dentry) -> bool { (*x).d_flags & DCACHE_MANAGED_DENTRY != 0 }
#[inline] pub unsafe fn d_mountpoint(x: *const dentry) -> bool { (*x).d_flags & DCACHE_MOUNTED != 0 }
#[inline] pub unsafe fn __d_entry_type(x: *const dentry) -> c_uint { (*x).d_flags & DCACHE_ENTRY_TYPE }
#[inline] pub unsafe fn d_is_miss(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_MISS_TYPE }
#[inline] pub unsafe fn d_is_whiteout(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_WHITEOUT_TYPE }
#[inline] pub unsafe fn d_can_lookup(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_DIRECTORY_TYPE }
#[inline] pub unsafe fn d_is_autodir(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_AUTODIR_TYPE }
#[inline] pub unsafe fn d_is_dir(x: *const dentry) -> bool { d_can_lookup(x) || d_is_autodir(x) }
#[inline] pub unsafe fn d_is_symlink(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_SYMLINK_TYPE }
#[inline] pub unsafe fn d_is_reg(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_REGULAR_TYPE }
#[inline] pub unsafe fn d_is_special(x: *const dentry) -> bool { __d_entry_type(x) == DCACHE_SPECIAL_TYPE }
#[inline] pub unsafe fn d_is_file(x: *const dentry) -> bool { d_is_reg(x) || d_is_special(x) }
#[inline] pub unsafe fn d_is_negative(x: *const dentry) -> bool { d_is_miss(x) }
#[inline] pub unsafe fn d_flags_negative(flags: c_uint) -> bool { flags & DCACHE_ENTRY_TYPE == DCACHE_MISS_TYPE }
#[inline] pub unsafe fn d_is_positive(x: *const dentry) -> bool { !d_is_negative(x) }
#[inline] pub unsafe fn d_really_is_negative(x: *const dentry) -> bool { (*x).d_inode.is_null() }
#[inline] pub unsafe fn d_really_is_positive(x: *const dentry) -> bool { !(*x).d_inode.is_null() }
#[inline] pub unsafe fn simple_positive(x: *const dentry) -> bool { d_really_is_positive(x) && d_unhashed(x) == 0 }
#[inline] pub unsafe fn d_inode(x: *const dentry) -> *mut inode { (*x).d_inode }
#[inline] pub unsafe fn d_inode_rcu(x: *const dentry) -> *mut inode { (*x).d_inode }
#[inline] pub unsafe fn d_backing_inode(x: *const dentry) -> *mut inode { (*x).d_inode }
#[inline] pub unsafe fn d_real(x: *mut dentry, ty: d_real_type) -> *mut dentry { if (*x).d_flags & DCACHE_OP_REAL != 0 { ((*x).d_op).as_ref().unwrap().d_real.unwrap()(x, ty) } else { x } }
#[inline] pub unsafe fn d_real_inode(x: *const dentry) -> *mut inode { d_inode(d_real(x as *mut dentry, d_real_type::D_REAL_DATA)) }
#[inline] pub unsafe fn d_first_child(_: *const dentry) -> *mut dentry { core::ptr::null_mut() }
#[inline] pub unsafe fn d_next_sibling(_: *const dentry) -> *mut dentry { core::ptr::null_mut() }

extern "C" { fn lockref_get(*mut lockref); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
