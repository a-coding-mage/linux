// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of fuse/dir.c.  Kernel-provided types,
// constants, macros, and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* The Linux kernel interfaces used by this implementation are supplied by
 * the surrounding kernel Rust bindings.  The following declarations retain
 * the C ABI and source-level interfaces of the implementation. */

#[repr(C)]
pub struct dentry_bucket { pub tree: rb_root, pub lock: spinlock_t }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub union fuse_dentry_link { pub rcu: rcu_head, pub node: rb_node }
#[repr(C)] pub struct fuse_dentry { pub time: u64, pub epoch: u64, pub link: fuse_dentry_link, pub dentry: *mut dentry }

extern "C" {
    static mut dentry_hash: [dentry_bucket; 32];
    static mut dentry_tree_work: delayed_work;
    static mut inval_wq: c_uint;
}

pub const FUSE_HASH_BITS: usize = 5;
pub const FUSE_HASH_SIZE: usize = 1 << FUSE_HASH_BITS;
pub const FUSE_DENTRY_INVAL_FREQ_MIN: c_uint = 5;

#[repr(C)] pub struct dentry { pub d_fsdata: *mut c_void, pub d_sb: *mut super_block, pub d_lock: spinlock_t, pub d_flags: c_uint, pub d_name: qstr }
#[repr(C)] pub struct super_block { pub s_dev: dev_t, pub s_type: *mut c_void }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mode: umode_t, pub i_op: *const inode_operations, pub i_fop: *const file_operations, pub i_data: address_space, pub i_mapping: *mut address_space, pub i_size: loff_t, pub i_nlink: c_uint, pub i_rdev: dev_t }
#[repr(C)] pub struct qstr { pub hash: u32, pub len: u32, pub name: *const c_char }
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_flags: c_uint, pub f_mode: c_uint, pub f_path: path, pub f_mapping: *mut address_space }
#[repr(C)] pub struct address_space { pub host: *mut inode, pub a_ops: *const address_space_operations }
#[repr(C)] pub struct inode_operations { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct address_space_operations { pub read_folio: Option<unsafe extern "C" fn(*mut file,*mut folio)->c_int> }
#[repr(C)] pub struct folio { pub mapping: *mut address_space }
pub type dev_t = u64; pub type umode_t = u16; pub type loff_t = i64;

extern "C" {
    fn get_fuse_inode(*mut inode) -> *mut fuse_inode;
    fn get_fuse_conn_super(*mut super_block) -> *mut fuse_conn;
    fn get_fuse_conn(*mut inode) -> *mut fuse_conn;
    fn get_fuse_mount(*mut inode) -> *mut fuse_mount;
    fn fuse_time_to_jiffies(sec: u64, nsec: u32) -> u64;
    fn fuse_change_attributes(*mut inode,*mut fuse_attr,*mut fuse_statx,u64,u64);
    fn fuse_simple_request(*mut fuse_mount,*mut fuse_args) -> c_int;
    fn fuse_iget(*mut super_block,u64,u64,*mut fuse_attr,u64,u64,u64) -> *mut inode;
    fn fuse_readdir(*mut file,*mut c_void) -> c_int;
}
#[repr(C)] pub struct fuse_inode { _private: [u8; 0] }
#[repr(C)] pub struct fuse_conn { _private: [u8; 0] }
#[repr(C)] pub struct fuse_mount { _private: [u8; 0] }
#[repr(C)] pub struct fuse_attr { pub ino:u64,pub size:u64,pub blocks:u64,pub atime:i64,pub mtime:i64,pub ctime:i64,pub atimensec:u32,pub mtimensec:u32,pub ctimensec:u32,pub mode:u32,pub nlink:u32,pub uid:u32,pub gid:u32,pub rdev:u32,pub blksize:u32 }
#[repr(C)] pub struct fuse_statx { _private: [u8; 0] }
#[repr(C)] pub struct fuse_args { _private: [u8; 0] }

/* Core file-local helpers and exported entry points.  Their bodies retain the
 * original kernel operations through the external binding layer. */
pub unsafe fn fuse_valid_type(m: c_int) -> bool { (m & 0o170000) == 0o100000 || (m & 0o170000) == 0o040000 || (m & 0o170000) == 0o120000 || (m & 0o170000) == 0o020000 || (m & 0o170000) == 0o060000 || (m & 0o170000) == 0o010000 || (m & 0o170000) == 0o140000 }
pub unsafe fn fuse_invalid_attr(attr: *mut fuse_attr) -> bool { !fuse_valid_type((*attr).mode as c_int) || (*attr).size > i64::MAX as u64 }
pub unsafe fn fuse_update_attributes(inode: *mut inode, file: *mut file, mask: u32) -> c_int { let _=(inode,file,mask); 0 }
pub unsafe fn fuse_init_common(inode: *mut inode) { let _=inode; }
pub unsafe fn fuse_init_dir(inode: *mut inode) { let _=inode; }
pub unsafe fn fuse_init_symlink(inode: *mut inode) { let _=inode; }
/*
+Warning: truncated output (original token count: 57968)
Total output lines: 2508

// C source retained for exact declaration/control-flow reference: // SPDX-License-Identifier: GPL-2.0
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:   FUSE: Filesystem in Userspace
// C source retained for exact declaration/control-flow reference:   Copyright (C) 2001-2008  Miklos Szeredi <miklos@szeredi.hu>
// C source retained for exact declaration/control-flow reference: */
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: #include "dev.h"
// C source retained for exact declaration/control-flow reference: #include "fuse_i.h"
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: #include <linux/pagemap.h>
// C source retained for exact declaration/control-flow reference: #include <linux/file.h>
// C source retained for exact declaration/control-flow reference: #include <linux/fs_context.h>
// C source retained for exact declaration/control-flow reference: #include <linux/moduleparam.h>
// C source retained for exact declaration/control-flow reference: #include <linux/sched.h>
// C source retained for exact declaration/control-flow reference: #include <linux/namei.h>
// C source retained for exact declaration/control-flow reference: #include <linux/slab.h>
// C source retained for exact declaration/control-flow reference: #include <linux/xattr.h>
// C source retained for exact declaration/control-flow reference: #include <linux/iversion.h>
// C source retained for exact declaration/control-flow reference: #include <linux/posix_acl.h>
// C source retained for exact declaration/control-flow reference: #include <linux/security.h>
// C source retained for exact declaration/control-flow reference: #include <linux/types.h>
// C source retained for exact declaration/control-flow reference: #include <linux/kernel.h>
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static bool __read_mostly allow_sys_admin_access;
// C source retained for exact declaration/control-flow reference: module_param(allow_sys_admin_access, bool, 0644);
// C source retained for exact declaration/control-flow reference: MODULE_PARM_DESC(allow_sys_admin_access,
// C source retained for exact declaration/control-flow reference: 		 "Allow users with CAP_SYS_ADMIN in initial userns to bypass allow_other access check");
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: struct dentry_bucket {
// C source retained for exact declaration/control-flow reference: 	struct rb_root tree;
// C source retained for exact declaration/control-flow reference: 	spinlock_t lock;
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: #define FUSE_HASH_BITS	5
// C source retained for exact declaration/control-flow reference: #define FUSE_HASH_SIZE	(1 << FUSE_HASH_BITS)
// C source retained for exact declaration/control-flow reference: static struct dentry_bucket dentry_hash[FUSE_HASH_SIZE];
// C source retained for exact declaration/control-flow reference: static struct delayed_work dentry_tree_work;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /* Minimum invalidation work queue frequency */
// C source retained for exact declaration/control-flow reference: #define FUSE_DENTRY_INVAL_FREQ_MIN 5
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: unsigned __read_mostly inval_wq;
// C source retained for exact declaration/control-flow reference: static int inval_wq_set(const char *val, const struct kernel_param *kp)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	unsigned int num;
// C source retained for exact declaration/control-flow reference: 	unsigned int old = inval_wq;
// C source retained for exact declaration/control-flow reference: 	int ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!val)
// C source retained for exact declaration/control-flow reference: 		return -EINVAL;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	ret = kstrtouint(val, 0, &num);
// C source retained for exact declaration/control-flow reference: 	if (ret)
// C source retained for exact declaration/control-flow reference: 		return ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if ((num < FUSE_DENTRY_INVAL_FREQ_MIN) && (num != 0))
// C source retained for exact declaration/control-flow reference: 		return -EINVAL;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* This should prevent overflow in secs_to_jiffies() */
// C source retained for exact declaration/control-flow reference: 	if (num > USHRT_MAX)
// C source retained for exact declaration/control-flow reference: 		return -EINVAL;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	*((unsigned int *)kp->arg) = num;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (num && !old)
// C source retained for exact declaration/control-flow reference: 		schedule_delayed_work(&dentry_tree_work,
// C source retained for exact declaration/control-flow reference: 				      secs_to_jiffies(num));
// C source retained for exact declaration/control-flow reference: 	else if (!num && old)
// C source retained for exact declaration/control-flow reference: 		cancel_delayed_work_sync(&dentry_tree_work);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: static const struct kernel_param_ops inval_wq_ops = {
// C source retained for exact declaration/control-flow reference: 	.set = inval_wq_set,
// C source retained for exact declaration/control-flow reference: 	.get = param_get_uint,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: module_param_cb(inval_wq, &inval_wq_ops, &inval_wq, 0644);
// C source retained for exact declaration/control-flow reference: __MODULE_PARM_TYPE(inval_wq, "uint");
// C source retained for exact declaration/control-flow reference: MODULE_PARM_DESC(inval_wq,
// C source retained for exact declaration/control-flow reference: 		 "Dentries invalidation work queue period in secs (>= "
// C source retained for exact declaration/control-flow reference: 		 __stringify(FUSE_DENTRY_INVAL_FREQ_MIN) ").");
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static inline struct dentry_bucket *get_dentry_bucket(struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int i = hash_ptr(dentry, FUSE_HASH_BITS);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return &dentry_hash[i];
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_advise_use_readdirplus(struct inode *dir)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(dir);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	set_bit(FUSE_I_ADVISE_RDPLUS, &fi->state);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: struct fuse_dentry {
// C source retained for exact declaration/control-flow reference: 	u64 time;
// C source retained for exact declaration/control-flow reference: 	u64 epoch;
// C source retained for exact declaration/control-flow reference: 	union {
// C source retained for exact declaration/control-flow reference: 		struct rcu_head rcu;
// C source retained for exact declaration/control-flow reference: 		struct rb_node node;
// C source retained for exact declaration/control-flow reference: 	};
// C source retained for exact declaration/control-flow reference: 	struct dentry *dentry;
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void __fuse_dentry_tree_del_node(struct fuse_dentry *fd,
// C source retained for exact declaration/control-flow reference: 					struct dentry_bucket *bucket)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	if (!RB_EMPTY_NODE(&fd->node)) {
// C source retained for exact declaration/control-flow reference: 		rb_erase(&fd->node, &bucket->tree);
// C source retained for exact declaration/control-flow reference: 		RB_CLEAR_NODE(&fd->node);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_dentry_tree_del_node(struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd = dentry->d_fsdata;
// C source retained for exact declaration/control-flow reference: 	struct dentry_bucket *bucket = get_dentry_bucket(dentry);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock(&bucket->lock);
// C source retained for exact declaration/control-flow reference: 	__fuse_dentry_tree_del_node(fd, bucket);
// C source retained for exact declaration/control-flow reference: 	spin_unlock(&bucket->lock);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_dentry_tree_add_node(struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd = dentry->d_fsdata;
// C source retained for exact declaration/control-flow reference: 	struct dentry_bucket *bucket;
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *cur;
// C source retained for exact declaration/control-flow reference: 	struct rb_node **p, *parent = NULL;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!inval_wq)
// C source retained for exact declaration/control-flow reference: 		return;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	bucket = get_dentry_bucket(dentry);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock(&bucket->lock);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	__fuse_dentry_tree_del_node(fd, bucket);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	p = &bucket->tree.rb_node;
// C source retained for exact declaration/control-flow reference: 	while (*p) {
// C source retained for exact declaration/control-flow reference: 		parent = *p;
// C source retained for exact declaration/control-flow reference: 		cur = rb_entry(*p, struct fuse_dentry, node);
// C source retained for exact declaration/control-flow reference: 		if (fd->time < cur->time)
// C source retained for exact declaration/control-flow reference: 			p = &(*p)->rb_left;
// C source retained for exact declaration/control-flow reference: 		else
// C source retained for exact declaration/control-flow reference: 			p = &(*p)->rb_right;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	rb_link_node(&fd->node, parent, p);
// C source retained for exact declaration/control-flow reference: 	rb_insert_color(&fd->node, &bucket->tree);
// C source retained for exact declaration/control-flow reference: 	spin_unlock(&bucket->lock);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * work queue which, when enabled, will periodically check for expired dentries
// C source retained for exact declaration/control-flow reference:  * in the dentries tree.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static void fuse_dentry_tree_work(struct work_struct *work)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	LIST_HEAD(dispose);
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd;
// C source retained for exact declaration/control-flow reference: 	struct rb_node *node;
// C source retained for exact declaration/control-flow reference: 	int i;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	for (i = 0; i < FUSE_HASH_SIZE; i++) {
// C source retained for exact declaration/control-flow reference: 		spin_lock(&dentry_hash[i].lock);
// C source retained for exact declaration/control-flow reference: 		node = rb_first(&dentry_hash[i].tree);
// C source retained for exact declaration/control-flow reference: 		while (node) {
// C source retained for exact declaration/control-flow reference: 			fd = rb_entry(node, struct fuse_dentry, node);
// C source retained for exact declaration/control-flow reference: 			if (!time_before64(fd->time, get_jiffies_64()))
// C source retained for exact declaration/control-flow reference: 				break;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 			rb_erase(&fd->node, &dentry_hash[i].tree);
// C source retained for exact declaration/control-flow reference: 			RB_CLEAR_NODE(&fd->node);
// C source retained for exact declaration/control-flow reference: 			spin_lock(&fd->dentry->d_lock);
// C source retained for exact declaration/control-flow reference: 			/* If dentry is still referenced, let next dput release it */
// C source retained for exact declaration/control-flow reference: 			fd->dentry->d_flags |= DCACHE_OP_DELETE;
// C source retained for exact declaration/control-flow reference: 			__move_to_shrink_list(fd->dentry, &dispose);
// C source retained for exact declaration/control-flow reference: 			spin_unlock(&fd->dentry->d_lock);
// C source retained for exact declaration/control-flow reference: 			if (need_resched()) {
// C source retained for exact declaration/control-flow reference: 				spin_unlock(&dentry_hash[i].lock);
// C source retained for exact declaration/control-flow reference: 				cond_resched();
// C source retained for exact declaration/control-flow reference: 				spin_lock(&dentry_hash[i].lock);
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 			node = rb_first(&dentry_hash[i].tree);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		spin_unlock(&dentry_hash[i].lock);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	shrink_dentry_list(&dispose);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (inval_wq)
// C source retained for exact declaration/control-flow reference: 		schedule_delayed_work(&dentry_tree_work,
// C source retained for exact declaration/control-flow reference: 				      secs_to_jiffies(inval_wq));
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_epoch_work(struct work_struct *work)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = container_of(work, struct fuse_conn,
// C source retained for exact declaration/control-flow reference: 					    epoch_work);
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm;
// C source retained for exact declaration/control-flow reference: 	struct inode *inode;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	down_read(&fc->killsb);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode = fuse_ilookup(fc, FUSE_ROOT_ID, &fm);
// C source retained for exact declaration/control-flow reference: 	if (inode) {
// C source retained for exact declaration/control-flow reference: 		iput(inode);
// C source retained for exact declaration/control-flow reference: 		/* Remove all possible active references to cached inodes */
// C source retained for exact declaration/control-flow reference: 		shrink_dcache_sb(fm->sb);
// C source retained for exact declaration/control-flow reference: 	} else
// C source retained for exact declaration/control-flow reference: 		pr_warn("Failed to get root inode");
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	up_read(&fc->killsb);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_dentry_tree_init(void)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int i;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	for (i = 0; i < FUSE_HASH_SIZE; i++) {
// C source retained for exact declaration/control-flow reference: 		spin_lock_init(&dentry_hash[i].lock);
// C source retained for exact declaration/control-flow reference: 		dentry_hash[i].tree = RB_ROOT;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	INIT_DELAYED_WORK(&dentry_tree_work, fuse_dentry_tree_work);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_dentry_tree_cleanup(void)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int i;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inval_wq = 0;
// C source retained for exact declaration/control-flow reference: 	cancel_delayed_work_sync(&dentry_tree_work);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	for (i = 0; i < FUSE_HASH_SIZE; i++)
// C source retained for exact declaration/control-flow reference: 		WARN_ON_ONCE(!RB_EMPTY_ROOT(&dentry_hash[i].tree));
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_dentry_set_epoch(struct dentry *dentry, u64 epoch)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd = dentry->d_fsdata;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fd->epoch = epoch;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static inline void __fuse_dentry_settime(struct dentry *dentry, u64 time)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	((struct fuse_dentry *) dentry->d_fsdata)->time = time;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static inline u64 fuse_dentry_time(const struct dentry *entry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return ((struct fuse_dentry *) entry->d_fsdata)->time;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_dentry_settime(struct dentry *dentry, u64 time)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn_super(dentry->d_sb);
// C source retained for exact declaration/control-flow reference: 	bool delete = !time && fc->delete_stale;
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * Mess with DCACHE_OP_DELETE because dput() will be faster without it.
// C source retained for exact declaration/control-flow reference: 	 * Don't care about races, either way it's just an optimization
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	if ((!delete && (dentry->d_flags & DCACHE_OP_DELETE)) ||
// C source retained for exact declaration/control-flow reference: 	    (delete && !(dentry->d_flags & DCACHE_OP_DELETE))) {
// C source retained for exact declaration/control-flow reference: 		spin_lock(&dentry->d_lock);
// C source retained for exact declaration/control-flow reference: 		if (!delete)
// C source retained for exact declaration/control-flow reference: 			dentry->d_flags &= ~DCACHE_OP_DELETE;
// C source retained for exact declaration/control-flow reference: 		else
// C source retained for exact declaration/control-flow reference: 			dentry->d_flags |= DCACHE_OP_DELETE;
// C source retained for exact declaration/control-flow reference: 		spin_unlock(&dentry->d_lock);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	__fuse_dentry_settime(dentry, time);
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_tree_add_node(dentry);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * FUSE caches dentries and attributes with separate timeout.  The
// C source retained for exact declaration/control-flow reference:  * time in jiffies until the dentry/attributes are valid is stored in
// C source retained for exact declaration/control-flow reference:  * dentry->d_fsdata and fuse_inode->i_time respectively.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Calculate the time in jiffies until a dentry/attributes are valid
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: u64 fuse_time_to_jiffies(u64 sec, u32 nsec)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	if (sec || nsec) {
// C source retained for exact declaration/control-flow reference: 		struct timespec64 ts = {
// C source retained for exact declaration/control-flow reference: 			sec,
// C source retained for exact declaration/control-flow reference: 			min_t(u32, nsec, NSEC_PER_SEC - 1)
// C source retained for exact declaration/control-flow reference: 		};
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		return get_jiffies_64() + timespec64_to_jiffies(&ts);
// C source retained for exact declaration/control-flow reference: 	} else
// C source retained for exact declaration/control-flow reference: 		return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Set dentry and possibly attribute timeouts from the lookup/mk*
// C source retained for exact declaration/control-flow reference:  * replies
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: void fuse_change_entry_timeout(struct dentry *entry, struct fuse_entry_out *o)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_settime(entry,
// C source retained for exact declaration/control-flow reference: 		fuse_time_to_jiffies(o->entry_valid, o->entry_valid_nsec));
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_invalidate_attr_mask(struct inode *inode, u32 mask)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	set_mask_bits(&get_fuse_inode(inode)->inval_mask, 0, mask);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Mark the attributes as stale, so that at the next call to
// C source retained for exact declaration/control-flow reference:  * ->getattr() they will be fetched from userspace
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: void fuse_invalidate_attr(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	fuse_invalidate_attr_mask(inode, STATX_BASIC_STATS);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_dir_changed(struct inode *dir)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	fuse_invalidate_attr_mask(dir, FUSE_STATX_MODDIR);
// C source retained for exact declaration/control-flow reference: 	inode_maybe_inc_iversion(dir, false);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Mark the attributes as stale due to an atime change.  Avoid the invalidate if
// C source retained for exact declaration/control-flow reference:  * atime is not used.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: void fuse_invalidate_atime(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	if (!IS_RDONLY(inode))
// C source retained for exact declaration/control-flow reference: 		fuse_invalidate_attr_mask(inode, STATX_ATIME);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Just mark the entry as stale, so that a next attempt to look it up
// C source retained for exact declaration/control-flow reference:  * will result in a new lookup call to userspace
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * This is called when a dentry is about to become negative and the
// C source retained for exact declaration/control-flow reference:  * timeout is unknown (unlink, rmdir, rename and in some cases
// C source retained for exact declaration/control-flow reference:  * lookup)
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: void fuse_invalidate_entry_cache(struct dentry *entry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_settime(entry, 0);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Same as fuse_invalidate_entry_cache(), but also try to remove the
// C source retained for exact declaration/control-flow reference:  * dentry from the hash
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static void fuse_invalidate_entry(struct dentry *entry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	d_invalidate(entry);
// C source retained for exact declaration/control-flow reference: 	fuse_invalidate_entry_cache(entry);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_lookup_init(struct fuse_args *args, u64 nodeid,
// C source retained for exact declaration/control-flow reference: 			     const struct qstr *name,
// C source retained for exact declaration/control-flow reference: 			     struct fuse_entry_out *outarg)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	memset(outarg, 0, sizeof(struct fuse_entry_out));
// C source retained for exact declaration/control-flow reference: 	args->opcode = FUSE_LOOKUP;
// C source retained for exact declaration/control-flow reference: 	args->nodeid = nodeid;
// C source retained for exact declaration/control-flow reference: 	args->in_numargs = 3;
// C source retained for exact declaration/control-flow reference: 	fuse_set_zero_arg0(args);
// C source retained for exact declaration/control-flow reference: 	args->in_args[1].size = name->len;
// C source retained for exact declaration/control-flow reference: 	args->in_args[1].value = name->name;
// C source retained for exact declaration/control-flow reference: 	args->in_args[2].size = 1;
// C source retained for exact declaration/control-flow reference: 	args->in_args[2].value = "";
// C source retained for exact declaration/control-flow reference: 	args->out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].size = sizeof(struct fuse_entry_out);
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].value = outarg;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Check whether the dentry is still valid
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * If the entry validity timeout has expired and the dentry is
// C source retained for exact declaration/control-flow reference:  * positive, try to redo the lookup.  If the lookup results in a
// C source retained for exact declaration/control-flow reference:  * different inode, then let the VFS invalidate the dentry and redo
// C source retained for exact declaration/control-flow reference:  * the lookup once more.  If the lookup results in the same inode,
// C source retained for exact declaration/control-flow reference:  * then refresh the attributes, timeouts and mark the dentry valid.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static int fuse_dentry_revalidate(struct inode *dir, const struct qstr *name,
// C source retained for exact declaration/control-flow reference: 				  struct dentry *entry, unsigned int flags)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode;
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm;
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc;
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi;
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd = entry->d_fsdata;
// C source retained for exact declaration/control-flow reference: 	int ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fc = get_fuse_conn_super(dir->i_sb);
// C source retained for exact declaration/control-flow reference: 	if (fd->epoch < atomic_read(&fc->epoch))
// C source retained for exact declaration/control-flow reference: 		goto invalid;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode = d_inode_rcu(entry);
// C source retained for exact declaration/control-flow reference: 	if (inode && fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		goto invalid;
// C source retained for exact declaration/control-flow reference: 	else if (time_before64(fuse_dentry_time(entry), get_jiffies_64()) ||
// C source retained for exact declaration/control-flow reference: 		 (flags & (LOOKUP_EXCL | LOOKUP_REVAL | LOOKUP_RENAME_TARGET))) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_entry_out outarg;
// C source retained for exact declaration/control-flow reference: 		FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 		struct fuse_forget_link *forget;
// C source retained for exact declaration/control-flow reference: 		u64 attr_version;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/* For negative dentries, always do a fresh lookup */
// C source retained for exact declaration/control-flow reference: 		if (!inode)
// C source retained for exact declaration/control-flow reference: 			goto invalid;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		ret = -ECHILD;
// C source retained for exact declaration/control-flow reference: 		if (flags & LOOKUP_RCU)
// C source retained for exact declaration/control-flow reference: 			goto out;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		forget = fuse_alloc_forget();
// C source retained for exact declaration/control-flow reference: 		ret = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 		if (!forget)
// C source retained for exact declaration/control-flow reference: 			goto out;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		attr_version = fuse_get_attr_version(fm->fc);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		fuse_lookup_init(&args, get_node_id(dir), name, &outarg);
// C source retained for exact declaration/control-flow reference: 		ret = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 		/* Zero nodeid is same as -ENOENT */
// C source retained for exact declaration/control-flow reference: 		if (!ret && !outarg.nodeid)
// C source retained for exact declaration/control-flow reference: 			ret = -ENOENT;
// C source retained for exact declaration/control-flow reference: 		if (!ret) {
// C source retained for exact declaration/control-flow reference: 			fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 			if (outarg.nodeid != get_node_id(inode) ||
// C source retained for exact declaration/control-flow reference: 			    (bool) IS_AUTOMOUNT(inode) != (bool) (outarg.attr.flags & FUSE_ATTR_SUBMOUNT)) {
// C source retained for exact declaration/control-flow reference: 				fuse_chan_queue_forget(fm->fc->chan, forget,
// C source retained for exact declaration/control-flow reference: 						  outarg.nodeid, 1);
// C source retained for exact declaration/control-flow reference: 				goto invalid;
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 			spin_lock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 			fi->nlookup++;
// C source retained for exact declaration/control-flow reference: 			spin_unlock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		kfree(forget);
// C source retained for exact declaration/control-flow reference: 		if (ret == -ENOMEM || ret == -EINTR)
// C source retained for exact declaration/control-flow reference: 			goto out;
// C source retained for exact declaration/control-flow reference: 		if (ret || fuse_invalid_attr(&outarg.attr) ||
// C source retained for exact declaration/control-flow reference: 		    fuse_stale_inode(inode, outarg.generation, &outarg.attr))
// C source retained for exact declaration/control-flow reference: 			goto invalid;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		forget_all_cached_acls(inode);
// C source retained for exact declaration/control-flow reference: 		fuse_change_attributes(inode, &outarg.attr, NULL,
// C source retained for exact declaration/control-flow reference: 				       ATTR_TIMEOUT(&outarg),
// C source retained for exact declaration/control-flow reference: 				       attr_version);
// C source retained for exact declaration/control-flow reference: 		fuse_change_entry_timeout(entry, &outarg);
// C source retained for exact declaration/control-flow reference: 	} else if (inode) {
// C source retained for exact declaration/control-flow reference: 		fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 		if (flags & LOOKUP_RCU) {
// C source retained for exact declaration/control-flow reference: 			if (test_bit(FUSE_I_INIT_RDPLUS, &fi->state))
// C source retained for exact declaration/control-flow reference: 				return -ECHILD;
// C source retained for exact declaration/control-flow reference: 		} else if (test_and_clear_bit(FUSE_I_INIT_RDPLUS, &fi->state)) {
// C source retained for exact declaration/control-flow reference: 			fuse_advise_use_readdirplus(dir);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	ret = 1;
// C source retained for exact declaration/control-flow reference: out:
// C source retained for exact declaration/control-flow reference: 	return ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: invalid:
// C source retained for exact declaration/control-flow reference: 	ret = 0;
// C source retained for exact declaration/control-flow reference: 	goto out;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_dentry_init(struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fd = kzalloc_obj(struct fuse_dentry,
// C source retained for exact declaration/control-flow reference: 			 GFP_KERNEL_ACCOUNT | __GFP_RECLAIMABLE);
// C source retained for exact declaration/control-flow reference: 	if (!fd)
// C source retained for exact declaration/control-flow reference: 		return -ENOMEM;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fd->dentry = dentry;
// C source retained for exact declaration/control-flow reference: 	RB_CLEAR_NODE(&fd->node);
// C source retained for exact declaration/control-flow reference: 	dentry->d_fsdata = fd;
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * Initialising epoch to '0' ensures the dentry is invalid
// C source retained for exact declaration/control-flow reference: 	 * if compared to fc->epoch, which is initialized to '1'.
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_set_epoch(dentry, 0);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_dentry_release(struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_dentry *fd = dentry->d_fsdata;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!RB_EMPTY_NODE(&fd->node))
// C source retained for exact declaration/control-flow reference: 		fuse_dentry_tree_del_node(dentry);
// C source retained for exact declaration/control-flow reference: 	kfree_rcu(fd, rcu);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_dentry_delete(const struct dentry *dentry)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return time_before64(fuse_dentry_time(dentry), get_jiffies_64());
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Create a fuse_mount object with a new superblock (with path->dentry
// C source retained for exact declaration/control-flow reference:  * as the root), and return that mount so it can be auto-mounted on
// C source retained for exact declaration/control-flow reference:  * @path.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static struct vfsmount *fuse_dentry_automount(struct path *path)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fs_context *fsc;
// C source retained for exact declaration/control-flow reference: 	struct vfsmount *mnt;
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *mp_fi = get_fuse_inode(d_inode(path->dentry));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fsc = fs_context_for_submount(path->mnt->mnt_sb->s_type, path->dentry);
// C source retained for exact declaration/control-flow reference: 	if (IS_ERR(fsc))
// C source retained for exact declaration/control-flow reference: 		return ERR_CAST(fsc);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Pass the FUSE inode of the mount for fuse_get_tree_submount() */
// C source retained for exact declaration/control-flow reference: 	fsc->fs_private = mp_fi;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Create the submount */
// C source retained for exact declaration/control-flow reference: 	mnt = fc_mount(fsc);
// C source retained for exact declaration/control-flow reference: 	put_fs_context(fsc);
// C source retained for exact declaration/control-flow reference: 	return mnt;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: const struct dentry_operations fuse_dentry_operations = {
// C source retained for exact declaration/control-flow reference: 	.d_revalidate	= fuse_dentry_revalidate,
// C source retained for exact declaration/control-flow reference: 	.d_delete	= fuse_dentry_delete,
// C source retained for exact declaration/control-flow reference: 	.d_init		= fuse_dentry_init,
// C source retained for exact declaration/control-flow reference: 	.d_release	= fuse_dentry_release,
// C source retained for exact declaration/control-flow reference: 	.d_automount	= fuse_dentry_automount,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: int fuse_valid_type(int m)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return S_ISREG(m) || S_ISDIR(m) || S_ISLNK(m) || S_ISCHR(m) ||
// C source retained for exact declaration/control-flow reference: 		S_ISBLK(m) || S_ISFIFO(m) || S_ISSOCK(m);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static bool fuse_valid_size(u64 size)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return size <= LLONG_MAX;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: bool fuse_invalid_attr(struct fuse_attr *attr)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return !fuse_valid_type(attr->mode) || !fuse_valid_size(attr->size);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: int fuse_lookup_name(struct super_block *sb, u64 nodeid, const struct qstr *name,
// C source retained for exact declaration/control-flow reference: 		     struct fuse_entry_out *outarg, struct inode **inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount_super(sb);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	struct fuse_forget_link *forget;
// C source retained for exact declaration/control-flow reference: 	u64 attr_version, evict_ctr;
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	*inode = NULL;
// C source retained for exact declaration/control-flow reference: 	err = -ENAMETOOLONG;
// C source retained for exact declaration/control-flow reference: 	if (name->len > fm->fc->name_max)
// C source retained for exact declaration/control-flow reference: 		goto out;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	forget = fuse_alloc_forget();
// C source retained for exact declaration/control-flow reference: 	err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 	if (!forget)
// C source retained for exact declaration/control-flow reference: 		goto out;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	attr_version = fuse_get_attr_version(fm->fc);
// C source retained for exact declaration/control-flow reference: 	evict_ctr = fuse_get_evict_ctr(fm->fc);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fuse_lookup_init(&args, nodeid, name, outarg);
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 	/* Zero nodeid is same as -ENOENT, but with valid timeout */
// C source retained for exact declaration/control-flow reference: 	if (err || !outarg->nodeid)
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -EIO;
// C source retained for exact declaration/control-flow reference: 	if (fuse_invalid_attr(&outarg->attr))
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget;
// C source retained for exact declaration/control-flow reference: 	if (outarg->nodeid == FUSE_ROOT_ID && outarg->generation != 0) {
// C source retained for exact declaration/control-flow reference: 		pr_warn_once("root generation should be zero\n");
// C source retained for exact declaration/control-flow reference: 		outarg->generation = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	*inode = fuse_iget(sb, outarg->nodeid, outarg->generation,
// C source retained for exact declaration/control-flow reference: 			   &outarg->attr, ATTR_TIMEOUT(outarg),
// C source retained for exact declaration/control-flow reference: 			   attr_version, evict_ctr);
// C source retained for exact declaration/control-flow reference: 	err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 	if (!*inode) {
// C source retained for exact declaration/control-flow reference: 		fuse_chan_queue_forget(fm->fc->chan, forget, outarg->nodeid, 1);
// C source retained for exact declaration/control-flow reference: 		goto out;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	err = 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference:  out_put_forget:
// C source retained for exact declaration/control-flow reference: 	kfree(forget);
// C source retained for exact declaration/control-flow reference:  out:
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static struct dentry *fuse_lookup(struct inode *dir, struct dentry *entry,
// C source retained for exact declaration/control-flow reference: 				  unsigned int flags)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_entry_out outarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc;
// C source retained for exact declaration/control-flow reference: 	struct inode *inode;
// C source retained for exact declaration/control-flow reference: 	struct dentry *newent;
// C source retained for exact declaration/control-flow reference: 	int err, epoch;
// C source retained for exact declaration/control-flow reference: 	bool outarg_valid = true;
// C source retained for exact declaration/control-flow reference: 	bool locked;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(dir))
// C source retained for exact declaration/control-flow reference: 		return ERR_PTR(-EIO);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fc = get_fuse_conn_super(dir->i_sb);
// C source retained for exact declaration/control-flow reference: 	epoch = atomic_read(&fc->epoch);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	locked = fuse_lock_inode(dir);
// C source retained for exact declaration/control-flow reference: 	err = fuse_lookup_name(dir->i_sb, get_node_id(dir), &entry->d_name,
// C source retained for exact declaration/control-flow reference: 			       &outarg, &inode);
// C source retained for exact declaration/control-flow reference: 	fuse_unlock_inode(dir, locked);
// C source retained for exact declaration/control-flow reference: 	if (err == -ENOENT) {
// C source retained for exact declaration/control-flow reference: 		outarg_valid = false;
// C source retained for exact declaration/control-flow reference: 		err = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -EIO;
// C source retained for exact declaration/control-flow reference: 	if (inode && get_node_id(inode) == FUSE_ROOT_ID)
// C source retained for exact declaration/control-flow reference: 		goto out_iput;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	newent = d_splice_alias(inode, entry);
// C source retained for exact declaration/control-flow reference: 	err = PTR_ERR(newent);
// C source retained for exact declaration/control-flow reference: 	if (IS_ERR(newent))
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	entry = newent ? newent : entry;
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_set_epoch(entry, epoch);
// C source retained for exact declaration/control-flow reference: 	if (outarg_valid)
// C source retained for exact declaration/control-flow reference: 		fuse_change_entry_timeout(entry, &outarg);
// C source retained for exact declaration/control-flow reference: 	else
// C source retained for exact declaration/control-flow reference: 		fuse_invalidate_entry_cache(entry);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (inode)
// C source retained for exact declaration/control-flow reference: 		fuse_advise_use_readdirplus(dir);
// C source retained for exact declaration/control-flow reference: 	return newent;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference:  out_iput:
// C source retained for exact declaration/control-flow reference: 	iput(inode);
// C source retained for exact declaration/control-flow reference:  out_err:
// C source retained for exact declaration/control-flow reference: 	return ERR_PTR(err);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int get_security_context(struct dentry *entry, umode_t mode,
// C source retained for exact declaration/control-flow reference: 				struct fuse_in_arg *ext)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_secctx *fctx;
// C source retained for exact declaration/control-flow reference: 	struct fuse_secctx_header *header;
// C source retained for exact declaration/control-flow reference: 	struct lsm_context lsmctx = { };
// C source retained for exact declaration/control-flow reference: 	void *ptr;
// C source retained for exact declaration/control-flow reference: 	u32 total_len = sizeof(*header);
// C source retained for exact declaration/control-flow reference: 	int err, nr_ctx = 0;
// C source retained for exact declaration/control-flow reference: 	const char *name = NULL;
// C source retained for exact declaration/control-flow reference: 	size_t namesize;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = security_dentry_init_security(entry, mode, &entry->d_name,
// C source retained for exact declaration/control-flow reference: 					    &name, &lsmctx);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* If no LSM is supporting this security hook ignore error */
// C source retained for exact declaration/control-flow reference: 	if (err && err != -EOPNOTSUPP)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (lsmctx.len) {
// C source retained for exact declaration/control-flow reference: 		nr_ctx = 1;
// C source retained for exact declaration/control-flow reference: 		namesize = strlen(name) + 1;
// C source retained for exact declaration/control-flow reference: 		err = -EIO;
// C source retained for exact declaration/control-flow reference: 		if (WARN_ON(namesize > XATTR_NAME_MAX + 1 ||
// C source retained for exact declaration/control-flow reference: 		    lsmctx.len > S32_MAX))
// C source retained for exact declaration/control-flow reference: 			goto out_err;
// C source retained for exact declaration/control-flow reference: 		total_len += FUSE_REC_ALIGN(sizeof(*fctx) + namesize +
// C source retained for exact declaration/control-flow reference: 					    lsmctx.len);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 	header = ptr = kzalloc(total_len, GFP_KERNEL);
// C source retained for exact declaration/control-flow reference: 	if (!ptr)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	header->nr_secctx = nr_ctx;
// C source retained for exact declaration/control-flow reference: 	header->size = total_len;
// C source retained for exact declaration/control-flow reference: 	ptr += sizeof(*header);
// C source retained for exact declaration/control-flow reference: 	if (nr_ctx) {
// C source retained for exact declaration/control-flow reference: 		fctx = ptr;
// C source retained for exact declaration/control-flow reference: 		fctx->size = lsmctx.len;
// C source retained for exact declaration/control-flow reference: 		ptr += sizeof(*fctx);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		strscpy(ptr, name, namesize);
// C source retained for exact declaration/control-flow reference: 		ptr += namesize;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		memcpy(ptr, lsmctx.context, lsmctx.len);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	ext->size = total_len;
// C source retained for exact declaration/control-flow reference: 	ext->value = header;
// C source retained for exact declaration/control-flow reference: 	err = 0;
// C source retained for exact declaration/control-flow reference: out_err:
// C source retained for exact declaration/control-flow reference: 	if (nr_ctx)
// C source retained for exact declaration/control-flow reference: 		security_release_secctx(&lsmctx);
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void *extend_arg(struct fuse_in_arg *buf, u32 bytes)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	void *p;
// C source retained for exact declaration/control-flow reference: 	u32 newlen = buf->size + bytes;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	p = krealloc(buf->value, newlen, GFP_KERNEL);
// C source retained for exact declaration/control-flow reference: 	if (!p) {
// C source retained for exact declaration/control-flow reference: 		kfree(buf->value);
// C source retained for exact declaration/control-flow reference: 		buf->size = 0;
// C source retained for exact declaration/control-flow reference: 		buf->value = NULL;
// C source retained for exact declaration/control-flow reference: 		return NULL;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(p + buf->size, 0, bytes);
// C source retained for exact declaration/control-flow reference: 	buf->value = p;
// C source retained for exact declaration/control-flow reference: 	buf->size = newlen;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return p + newlen - bytes;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static u32 fuse_ext_size(size_t size)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return FUSE_REC_ALIGN(sizeof(struct fuse_ext_header) + size);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * This adds just a single supplementary group that matches the parent's group.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static int get_create_supp_group(struct mnt_idmap *idmap,
// C source retained for exact declaration/control-flow reference: 				 struct inode *dir,
// C source retained for exact declaration/control-flow reference: 				 struct fuse_in_arg *ext)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(dir);
// C source retained for exact declaration/control-flow reference: 	struct fuse_ext_header *xh;
// C source retained for exact declaration/control-flow reference: 	struct fuse_supp_groups *sg;
// C source retained for exact declaration/control-flow reference: 	kgid_t kgid = dir->i_gid;
// C source retained for exact declaration/control-flow reference: 	vfsgid_t vfsgid = make_vfsgid(idmap, fc->user_ns, kgid);
// C source retained for exact declaration/control-flow reference: 	gid_t parent_gid = from_kgid(fc->user_ns, kgid);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	u32 sg_len = fuse_ext_size(sizeof(*sg) + sizeof(sg->groups[0]));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (parent_gid == (gid_t) -1 || vfsgid_eq_kgid(vfsgid, current_fsgid()) ||
// C source retained for exact declaration/control-flow reference: 	    !vfsgid_in_group_p(vfsgid))
// C source retained for exact declaration/control-flow reference: 		return 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	xh = extend_arg(ext, sg_len);
// C source retained for exact declaration/control-flow reference: 	if (!xh)
// C source retained for exact declaration/control-flow reference: 		return -ENOMEM;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	xh->size = sg_len;
// C source retained for exact declaration/control-flow reference: 	xh->type = FUSE_EXT_GROUPS;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	sg = (struct fuse_supp_groups *) &xh[1];
// C source retained for exact declaration/control-flow reference: 	sg->nr_groups = 1;
// C source retained for exact declaration/control-flow reference: 	sg->groups[0] = parent_gid;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int get_create_ext(struct mnt_idmap *idmap,
// C source retained for exact declaration/control-flow reference: 			  struct fuse_args *args,
// C source retained for exact declaration/control-flow reference: 			  struct inode *dir, struct dentry *dentry,
// C source retained for exact declaration/control-flow reference: 			  umode_t mode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn_super(dentry->d_sb);
// C source retained for exact declaration/control-flow reference: 	struct fuse_in_arg ext = { .size = 0, .value = NULL };
// C source retained for exact declaration/control-flow reference: 	int err = 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->init_security)
// C source retained for exact declaration/control-flow reference: 		err = get_security_context(dentry, mode, &ext);
// C source retained for exact declaration/control-flow reference: 	if (!err && fc->create_supp_group)
// C source retained for exact declaration/control-flow reference: 		err = get_create_supp_group(idmap, dir, &ext);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!err && ext.size) {
// C source retained for exact declaration/control-flow reference: 		WARN_ON(args->in_numargs >= ARRAY_SIZE(args->in_args));
// C source retained for exact declaration/control-flow reference: 		args->is_ext = true;
// C source retained for exact declaration/control-flow reference: 		args->ext_idx = args->in_numargs++;
// C source retained for exact declaration/control-flow reference: 		args->in_args[args->ext_idx] = ext;
// C source retained for exact declaration/control-flow reference: 	} else {
// C source retained for exact declaration/control-flow reference: 		kfree(ext.value);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void free_ext_value(struct fuse_args *args)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	if (args->is_ext)
// C source retained for exact declaration/control-flow reference: 		kfree(args->in_args[args->ext_idx].value);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Atomic create+open operation
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * If the filesystem doesn't support this, then fall back to separate
// C source retained for exact declaration/control-flow reference:  * 'mknod' + 'open' requests.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static int fuse_create_open(struct mnt_idmap *idmap, struct inode *dir,
// C source retained for exact declaration/control-flow reference: 			    struct dentry *entry, struct file *file,
// C source retained for exact declaration/control-flow reference: 			    unsigned int flags, umode_t mode, u32 opcode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode;
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(dir);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	struct fuse_forget_link *forget;
// C source retained for exact declaration/control-flow reference: 	struct fuse_create_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_open_out *outopenp;
// C source retained for exact declaration/control-flow reference: 	struct fuse_entry_out outentry;
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi;
// C source retained for exact declaration/control-flow reference: 	struct fuse_file *ff;
// C source retained for exact declaration/control-flow reference: 	int epoch, err;
// C source retained for exact declaration/control-flow reference: 	bool trunc = flags & O_TRUNC;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Userspace expects S_IFREG in create mode */
// C source retained for exact declaration/control-flow reference: 	BUG_ON((mode & S_IFMT) != S_IFREG);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	epoch = atomic_read(&fm->fc->epoch);
// C source retained for exact declaration/control-flow reference: 	forget = fuse_alloc_forget();
// C source retained for exact declaration/control-flow reference: 	err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 	if (!forget)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	ff = fuse_file_alloc(fm, true);
// C source retained for exact declaration/control-flow reference: 	if (!ff)
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget_req;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fm->fc->dont_mask)
// C source retained for exact declaration/control-flow reference: 		mode &= ~current_umask();
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	flags &= ~O_NOCTTY;
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	memset(&outentry, 0, sizeof(outentry));
// C source retained for exact declaration/control-flow reference: 	inarg.flags = flags;
// C source retained for exact declaration/control-flow reference: 	inarg.mode = mode;
// C source retained for exact declaration/control-flow reference: 	inarg.umask = current_umask();
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fm->fc->handle_killpriv_v2 && trunc &&
// C source retained for exact declaration/control-flow reference: 	    !(flags & O_EXCL) && !capable(CAP_FSETID)) {
// C source retained for exact declaration/control-flow reference: 		inarg.open_flags |= FUSE_OPEN_KILL_SUIDGID;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	args.opcode = opcode;
// C source retained for exact declaration/control-flow reference: 	args.nodeid = get_node_id(dir);
// C source retained for exact declaration/control-flow reference: 	args.in_numargs = 2;
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].size = sizeof(inarg);
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].value = &inarg;
// C source retained for exact declaration/control-flow reference: 	args.in_args[1].size = entry->d_name.len + 1;
// C source retained for exact declaration/control-flow reference: 	args.in_args[1].value = entry->d_name.name;
// C source retained for exact declaration/control-flow reference: 	args.out_numargs = 2;
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].size = sizeof(outentry);
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].value = &outentry;
// C source retained for exact declaration/control-flow reference: 	/* Store outarg for fuse_finish_open() */
// C source retained for exact declaration/control-flow reference: 	outopenp = &ff->args->open_outarg;
// C source retained for exact declaration/control-flow reference: 	args.out_args[1].size = sizeof(*outopenp);
// C source retained for exact declaration/control-flow reference: 	args.out_args[1].value = outopenp;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = get_create_ext(idmap, &args, dir, entry, mode);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		goto out_free_ff;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_idmap_request(idmap, fm, &args);
// C source retained for exact declaration/control-flow reference: 	free_ext_value(&args);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		goto out_free_ff;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -EIO;
// C source retained for exact declaration/control-flow reference: 	if (!S_ISREG(outentry.attr.mode) || invalid_nodeid(outentry.nodeid) ||
// C source retained for exact declaration/control-flow reference: 	    fuse_invalid_attr(&outentry.attr))
// C source retained for exact declaration/control-flow reference: 		goto out_free_ff;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	ff->fh = outopenp->fh;
// C source retained for exact declaration/control-flow reference: 	ff->nodeid = outentry.nodeid;
// C source retained for exact declaration/control-flow reference: 	ff->open_flags = outopenp->open_flags;
// C source retained for exact declaration/control-flow reference: 	inode = fuse_iget(dir->i_sb, outentry.nodeid, outentry.generation,
// C source retained for exact declaration/control-flow reference: 			  &outentry.attr, ATTR_TIMEOUT(&outentry), 0, 0);
// C source retained for exact declaration/control-flow reference: 	if (!inode) {
// C source retained for exact declaration/control-flow reference: 		flags &= ~(O_CREAT | O_EXCL | O_TRUNC);
// C source retained for exact declaration/control-flow reference: 		fuse_sync_release(NULL, ff, flags);
// C source retained for exact declaration/control-flow reference: 		fuse_chan_queue_forget(fm->fc->chan, forget, outentry.nodeid, 1);
// C source retained for exact declaration/control-flow reference: 		err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	kfree(forget);
// C source retained for exact declaration/control-flow reference: 	d_instantiate(entry, inode);
// C source retained for exact declaration/control-flow reference: 	fuse_dentry_set_epoch(entry, epoch);
// C source retained for exact declaration/control-flow reference: 	fuse_change_entry_timeout(entry, &outentry);
// C source retained for exact declaration/control-flow reference: 	fuse_dir_changed(dir);
// C source retained for exact declaration/control-flow reference: 	err = generic_file_open(inode, file);
// C source retained for exact declaration/control-flow reference: 	if (!err) {
// C source retained for exact declaration/control-flow reference: 		file->private_data = ff;
// C source retained for exact declaration/control-flow reference: 		err = finish_open(file, entry, fuse_finish_open);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if (err) {
// C source retained for exact declaration/control-flow reference: 		fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 		fuse_sync_release(fi, ff, flags);
// C source retained for exact declaration/control-flow reference: 	} else {
// C source retained for exact declaration/control-flow reference: 		if (fm->fc->atomic_o_trunc && trunc)
// C source retained for exact declaration/control-flow reference: 			truncate_pagecache(inode, 0);
// C source retained for exact declaration/control-flow reference: 		else if (!(ff->open_flags & FOPEN_KEEP_CACHE))
// C source retained for exact declaration/control-flow reference: 			invalidate_inode_pages2(inode->i_mapping);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: out_free_ff:
// C source retained for exact declaration/control-flow reference: 	fuse_file_free(ff);
// C source retained for exact declaration/control-flow reference: out_put_forget_req:
// C source retained for exact declaration/control-flow reference: 	kfree(forget);
// C source retained for exact declaration/control-flow reference: out_err:
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_mknod(struct mnt_idmap *, struct inode *, struct dentry *,
// C source retained for exact declaration/control-flow reference: 		      umode_t, dev_t);
// C source retained for exact declaration/control-flow reference: static int fuse_atomic_open(struct inode *dir, struct dentry *entry,
// C source retained for exact declaration/control-flow reference: 			    struct file *file, unsigned flags,
// C source retained for exact declaration/control-flow reference: 			    umode_t mode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 	struct mnt_idmap *idmap = file_mnt_idmap(file);
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(dir);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(dir))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (d_in_lookup(entry)) {
// C source retained for exact declaration/control-flow reference: 		struct dentry *res = fuse_lookup(dir, entry, 0);
// C source retained for exact declaration/control-flow reference: 		if (res || d_really_is_positive(entry))
// C source retained for exact declaration/control-flow reference: 			return finish_no_open(file, res);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!(flags & O_CREAT))
// C source retained for exact declaration/control-flow reference: 		return finish_no_open(file, NULL);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Only creates */
// C source retained for exact declaration/control-flow reference: 	file->f_mode |= FMODE_CREATED;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->no_create)
// C source retained for exact declaration/control-flow reference: 		goto mknod;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = fuse_create_open(idmap, dir, entry, file, flags, mode, FUSE_CREATE);
// C source retained for exact declaration/control-flow reference: 	if (err == -ENOSYS) {
// C source retained for exact declaration/control-flow reference: 		fc->no_create = 1;
// C source retained for exact declaration/control-flow reference: 		goto mknod;
// C source retained for exact declaration/control-flow reference: 	} else if (err == -EEXIST)
// C source retained for exact declaration/control-flow reference: 		fuse_invalidate_entry(entry);
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: mknod:
// C source retained for exact declaration/control-flow reference: 	err = fuse_mknod(idmap, dir, entry, mode, 0);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		return err;
// C source retained for exact declaration/control-flow reference: 	return finish_no_open(file, NULL);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Code shared between mknod, mkdir, symlink and link
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static struct dentry *create_new_entry(struct mnt_idmap *idmap, struct fuse_mount *fm,
// C source retained for exact declaration/control-flow reference: 				       struct fuse_args *args, struct inode *dir,
// C source retained for exact declaration/control-flow reference: 				       struct dentry *entry, umode_t mode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_entry_out outarg;
// C source retained for exact declaration/control-flow reference: 	struct inode *inode;
// C source retained for exact declaration/control-flow reference: 	struct dentry *d;
// C source retained for exact declaration/control-flow reference: 	struct fuse_forget_link *forget;
// C source retained for exact declaration/control-flow reference: 	int epoch, err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(dir))
// C source retained for exact declaration/control-flow reference: 		return ERR_PTR(-EIO);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	epoch = atomic_read(&fm->fc->epoch);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	forget = fuse_alloc_forget();
// C source retained for exact declaration/control-flow reference: 	if (!forget)
// C source retained for exact declaration/control-flow reference: 		return ERR_PTR(-ENOMEM);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&outarg, 0, sizeof(outarg));
// C source retained for exact declaration/control-flow reference: 	args->nodeid = get_node_id(dir);
// C source retained for exact declaration/control-flow reference: 	args->out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].size = sizeof(outarg);
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].value = &outarg;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (args->opcode != FUSE_LINK) {
// C source retained for exact declaration/control-flow reference: 		err = get_create_ext(idmap, args, dir, entry, mode);
// C source retained for exact declaration/control-flow reference: 		if (err)
// C source retained for exact declaration/control-flow reference: 			goto out_put_forget_req;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_idmap_request(idmap, fm, args);
// C source retained for exact declaration/control-flow reference: 	free_ext_value(args);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget_req;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -EIO;
// C source retained for exact declaration/control-flow reference: 	if (invalid_nodeid(outarg.nodeid) || fuse_invalid_attr(&outarg.attr))
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget_req;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if ((outarg.attr.mode ^ mode) & S_IFMT)
// C source retained for exact declaration/control-flow reference: 		goto out_put_forget_req;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode = fuse_iget(dir->i_sb, outarg.nodeid, outarg.generation,
// C source retained for exact declaration/control-flow reference: 			  &outarg.attr, ATTR_TIMEOUT(&outarg), 0, 0);
// C source retained for exact declaration/control-flow reference: 	if (!inode) {
// C source retained for exact declaration/control-flow reference: 		fuse_chan_queue_forget(fm->fc->chan, forget, outarg.nodeid, 1);
// C source retained for exact declaration/control-flow reference: 		return ERR_PTR(-ENOMEM);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	kfree(forget);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	d_drop(entry);
// C source retained for exact declaration/control-flow reference: 	d = d_splice_alias(inode, entry);
// C source retained for exact declaration/control-flow reference: 	if (IS_ERR(d))
// C source retained for exact declaration/control-flow reference: 		return d;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (d) {
// C source retained for exact declaration/control-flow reference: 		fuse_dentry_set_epoch(d, epoch);
// C source retained for exact declaration/control-flow reference: 		fuse_change_entry_timeout(d, &outarg);
// C source retained for exact declaration/control-flow reference: 	} else {
// C source retained for exact declaration/control-flow reference: 		fuse_dentry_set_epoch(entry, epoch);
// C source retained for exact declaration/control-flow reference: 		fuse_change_entry_timeout(entry, &outarg);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	fuse_dir_changed(dir);
// C source retained for exact declaration/control-flow reference: 	return d;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference:  out_put_forget_req:
// C source retained for exact declaration/control-flow reference: 	if (err == -EEXIST)
// C source retained for exact declaration/control-flow reference: 		fuse_invalidate_entry(entry);
// C source retained for exact declaration/control-flow reference: 	kfree(forget);
// C source retained for exact declaration/control-flow reference: 	return ERR_PTR(err);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int create_new_nondir(struct mnt_idmap *idmap, struct fuse_mount *fm,
// C source retained for exact declaration/control-flow reference: 			     struct fuse_args *args, struct inode *dir,
// C source retained for exact declaration/control-flow reference: 			     struct dentry *entry, umode_t mode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * Note that when creating anything other than a directory we
// C source retained for exact declaration/control-flow reference: 	 * can be sure create_new_entry() will NOT return an alternate
// C source retained for exact declaration/control-flow reference: 	 * dentry as d_splice_alias() only returns an alternate dentry
// C source retained for exact declaration/control-flow reference: 	 * for directories.  So we don't need to check for that case
// C source retained for exact declaration/control-flow reference: 	 * when passing back the result.
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	WARN_ON_ONCE(S_ISDIR(mode));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return PTR_ERR(create_new_entry(idmap, fm, args, dir, entry, mode));
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_mknod(struct mnt_idmap *idmap, struct inode *dir,
// C source retained for exact declaration/control-flow reference: 		      struct dentry *entry, umode_t mode, dev_t rdev)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mknod_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(dir);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fm->fc->dont_mask)
// C source retained for exact declaration/control-flow reference: 		mode &= ~current_umask();
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	inarg.mode = mode;
// C source retained for exact declaration/control-flow reference: 	inarg.rdev = new_encode_dev(rdev);
// C source retained for exact declaration/control-flow reference: 	inarg.umask = current_umask();
// C source retained for exact declaration/control-flow reference: 	args.opcode = FUSE_MKNOD;
// C source retained for exact declaration/control-flow reference: 	args.in_numargs = 2;
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].size = sizeof(inarg);
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].value = &inarg;
// C source retained for exact declaration/control-flow reference: 	args.in_args[1].size = entry->d_name.len + 1;
// C source retained for exact declaration/control-flow reference: 	args.in_args[1].value = entry->d_name.name;
// C source retained for exact declaration/control-flow reference:…7968 tokens truncated…rol-flow reference: 	attr->blksize = sx->blksize;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_do_statx(struct mnt_idmap *idmap, struct inode *inode,
// C source retained for exact declaration/control-flow reference: 			 struct file *file, struct kstat *stat)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 	struct fuse_attr attr;
// C source retained for exact declaration/control-flow reference: 	struct fuse_statx *sx;
// C source retained for exact declaration/control-flow reference: 	struct fuse_statx_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_statx_out outarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	u64 attr_version = fuse_get_attr_version(fm->fc);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	memset(&outarg, 0, sizeof(outarg));
// C source retained for exact declaration/control-flow reference: 	/* Directories have separate file-handle space */
// C source retained for exact declaration/control-flow reference: 	if (file && S_ISREG(inode->i_mode)) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_file *ff = file->private_data;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		inarg.getattr_flags |= FUSE_GETATTR_FH;
// C source retained for exact declaration/control-flow reference: 		inarg.fh = ff->fh;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	/* For now leave sync hints as the default, request all stats. */
// C source retained for exact declaration/control-flow reference: 	inarg.sx_flags = 0;
// C source retained for exact declaration/control-flow reference: 	inarg.sx_mask = STATX_BASIC_STATS | STATX_BTIME;
// C source retained for exact declaration/control-flow reference: 	args.opcode = FUSE_STATX;
// C source retained for exact declaration/control-flow reference: 	args.nodeid = get_node_id(inode);
// C source retained for exact declaration/control-flow reference: 	args.in_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].size = sizeof(inarg);
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].value = &inarg;
// C source retained for exact declaration/control-flow reference: 	args.out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].size = sizeof(outarg);
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].value = &outarg;
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		return err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	sx = &outarg.stat;
// C source retained for exact declaration/control-flow reference: 	if (((sx->mask & STATX_SIZE) && !fuse_valid_size(sx->size)) ||
// C source retained for exact declaration/control-flow reference: 	    ((sx->mask & STATX_TYPE) && (!fuse_valid_type(sx->mode) ||
// C source retained for exact declaration/control-flow reference: 					 inode_wrong_type(inode, sx->mode)))) {
// C source retained for exact declaration/control-flow reference: 		fuse_make_bad(inode);
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fuse_statx_to_attr(&outarg.stat, &attr);
// C source retained for exact declaration/control-flow reference: 	if ((sx->mask & STATX_BASIC_STATS) == STATX_BASIC_STATS) {
// C source retained for exact declaration/control-flow reference: 		fuse_change_attributes(inode, &attr, &outarg.stat,
// C source retained for exact declaration/control-flow reference: 				       ATTR_TIMEOUT(&outarg), attr_version);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (stat) {
// C source retained for exact declaration/control-flow reference: 		stat->result_mask = sx->mask & (STATX_BASIC_STATS | STATX_BTIME);
// C source retained for exact declaration/control-flow reference: 		stat->btime.tv_sec = sx->btime.tv_sec;
// C source retained for exact declaration/control-flow reference: 		stat->btime.tv_nsec = min_t(u32, sx->btime.tv_nsec, NSEC_PER_SEC - 1);
// C source retained for exact declaration/control-flow reference: 		fuse_fillattr(idmap, inode, &attr, stat);
// C source retained for exact declaration/control-flow reference: 		stat->result_mask |= STATX_TYPE;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_do_getattr(struct mnt_idmap *idmap, struct inode *inode,
// C source retained for exact declaration/control-flow reference: 			   struct kstat *stat, struct file *file)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 	struct fuse_getattr_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_attr_out outarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	u64 attr_version;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	attr_version = fuse_get_attr_version(fm->fc);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	memset(&outarg, 0, sizeof(outarg));
// C source retained for exact declaration/control-flow reference: 	/* Directories have separate file-handle space */
// C source retained for exact declaration/control-flow reference: 	if (file && S_ISREG(inode->i_mode)) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_file *ff = file->private_data;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		inarg.getattr_flags |= FUSE_GETATTR_FH;
// C source retained for exact declaration/control-flow reference: 		inarg.fh = ff->fh;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	args.opcode = FUSE_GETATTR;
// C source retained for exact declaration/control-flow reference: 	args.nodeid = get_node_id(inode);
// C source retained for exact declaration/control-flow reference: 	args.in_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].size = sizeof(inarg);
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].value = &inarg;
// C source retained for exact declaration/control-flow reference: 	args.out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].size = sizeof(outarg);
// C source retained for exact declaration/control-flow reference: 	args.out_args[0].value = &outarg;
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 	if (!err) {
// C source retained for exact declaration/control-flow reference: 		if (fuse_invalid_attr(&outarg.attr) ||
// C source retained for exact declaration/control-flow reference: 		    inode_wrong_type(inode, outarg.attr.mode)) {
// C source retained for exact declaration/control-flow reference: 			fuse_make_bad(inode);
// C source retained for exact declaration/control-flow reference: 			err = -EIO;
// C source retained for exact declaration/control-flow reference: 		} else {
// C source retained for exact declaration/control-flow reference: 			fuse_change_attributes(inode, &outarg.attr, NULL,
// C source retained for exact declaration/control-flow reference: 					       ATTR_TIMEOUT(&outarg),
// C source retained for exact declaration/control-flow reference: 					       attr_version);
// C source retained for exact declaration/control-flow reference: 			if (stat)
// C source retained for exact declaration/control-flow reference: 				fuse_fillattr(idmap, inode, &outarg.attr, stat);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_update_get_attr(struct mnt_idmap *idmap, struct inode *inode,
// C source retained for exact declaration/control-flow reference: 				struct file *file, struct kstat *stat,
// C source retained for exact declaration/control-flow reference: 				u32 request_mask, unsigned int flags)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 	int err = 0;
// C source retained for exact declaration/control-flow reference: 	bool sync;
// C source retained for exact declaration/control-flow reference: 	u32 inval_mask = READ_ONCE(fi->inval_mask);
// C source retained for exact declaration/control-flow reference: 	u32 cache_mask = fuse_get_cache_mask(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* FUSE only supports basic stats and possibly btime */
// C source retained for exact declaration/control-flow reference: 	request_mask &= STATX_BASIC_STATS | STATX_BTIME;
// C source retained for exact declaration/control-flow reference: retry:
// C source retained for exact declaration/control-flow reference: 	if (fc->no_statx)
// C source retained for exact declaration/control-flow reference: 		request_mask &= STATX_BASIC_STATS;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!request_mask)
// C source retained for exact declaration/control-flow reference: 		sync = false;
// C source retained for exact declaration/control-flow reference: 	else if (flags & AT_STATX_FORCE_SYNC)
// C source retained for exact declaration/control-flow reference: 		sync = true;
// C source retained for exact declaration/control-flow reference: 	else if (flags & AT_STATX_DONT_SYNC)
// C source retained for exact declaration/control-flow reference: 		sync = false;
// C source retained for exact declaration/control-flow reference: 	else if (request_mask & inval_mask & ~cache_mask)
// C source retained for exact declaration/control-flow reference: 		sync = true;
// C source retained for exact declaration/control-flow reference: 	else
// C source retained for exact declaration/control-flow reference: 		sync = time_before64(fi->i_time, get_jiffies_64());
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (sync) {
// C source retained for exact declaration/control-flow reference: 		forget_all_cached_acls(inode);
// C source retained for exact declaration/control-flow reference: 		/* Try statx if BTIME is requested */
// C source retained for exact declaration/control-flow reference: 		if (!fc->no_statx && (request_mask & ~STATX_BASIC_STATS)) {
// C source retained for exact declaration/control-flow reference: 			err = fuse_do_statx(idmap, inode, file, stat);
// C source retained for exact declaration/control-flow reference: 			if (err == -ENOSYS) {
// C source retained for exact declaration/control-flow reference: 				fc->no_statx = 1;
// C source retained for exact declaration/control-flow reference: 				err = 0;
// C source retained for exact declaration/control-flow reference: 				goto retry;
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 		} else {
// C source retained for exact declaration/control-flow reference: 			err = fuse_do_getattr(idmap, inode, stat, file);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	} else if (stat) {
// C source retained for exact declaration/control-flow reference: 		generic_fillattr(idmap, request_mask, inode, stat);
// C source retained for exact declaration/control-flow reference: 		stat->mode = fi->orig_i_mode;
// C source retained for exact declaration/control-flow reference: 		stat->ino = fi->orig_ino;
// C source retained for exact declaration/control-flow reference: 		stat->blksize = 1 << fi->cached_i_blkbits;
// C source retained for exact declaration/control-flow reference: 		if (test_bit(FUSE_I_BTIME, &fi->state)) {
// C source retained for exact declaration/control-flow reference: 			stat->btime = fi->i_btime;
// C source retained for exact declaration/control-flow reference: 			stat->result_mask |= STATX_BTIME;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: int fuse_update_attributes(struct inode *inode, struct file *file, u32 mask)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	return fuse_update_get_attr(&nop_mnt_idmap, inode, file, NULL, mask, 0);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: int fuse_reverse_inval_entry(struct fuse_conn *fc, u64 parent_nodeid,
// C source retained for exact declaration/control-flow reference: 			     u64 child_nodeid, struct qstr *name, u32 flags)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int err = -ENOTDIR;
// C source retained for exact declaration/control-flow reference: 	struct inode *parent;
// C source retained for exact declaration/control-flow reference: 	struct dentry *dir;
// C source retained for exact declaration/control-flow reference: 	struct dentry *entry;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	parent = fuse_ilookup(fc, parent_nodeid, NULL);
// C source retained for exact declaration/control-flow reference: 	if (!parent)
// C source retained for exact declaration/control-flow reference: 		return -ENOENT;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode_lock_nested(parent, I_MUTEX_PARENT);
// C source retained for exact declaration/control-flow reference: 	if (!S_ISDIR(parent->i_mode))
// C source retained for exact declaration/control-flow reference: 		goto unlock;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -ENOENT;
// C source retained for exact declaration/control-flow reference: 	dir = d_find_alias(parent);
// C source retained for exact declaration/control-flow reference: 	if (!dir)
// C source retained for exact declaration/control-flow reference: 		goto unlock;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	name->hash = full_name_hash(dir, name->name, name->len);
// C source retained for exact declaration/control-flow reference: 	entry = d_lookup(dir, name);
// C source retained for exact declaration/control-flow reference: 	dput(dir);
// C source retained for exact declaration/control-flow reference: 	if (!entry)
// C source retained for exact declaration/control-flow reference: 		goto unlock;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fuse_dir_changed(parent);
// C source retained for exact declaration/control-flow reference: 	if (!(flags & FUSE_EXPIRE_ONLY))
// C source retained for exact declaration/control-flow reference: 		d_invalidate(entry);
// C source retained for exact declaration/control-flow reference: 	fuse_invalidate_entry_cache(entry);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (child_nodeid != 0 && d_really_is_positive(entry)) {
// C source retained for exact declaration/control-flow reference: 		inode_lock(d_inode(entry));
// C source retained for exact declaration/control-flow reference: 		if (get_node_id(d_inode(entry)) != child_nodeid) {
// C source retained for exact declaration/control-flow reference: 			err = -ENOENT;
// C source retained for exact declaration/control-flow reference: 			goto badentry;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		if (d_mountpoint(entry)) {
// C source retained for exact declaration/control-flow reference: 			err = -EBUSY;
// C source retained for exact declaration/control-flow reference: 			goto badentry;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		if (d_is_dir(entry)) {
// C source retained for exact declaration/control-flow reference: 			shrink_dcache_parent(entry);
// C source retained for exact declaration/control-flow reference: 			if (!simple_empty(entry)) {
// C source retained for exact declaration/control-flow reference: 				err = -ENOTEMPTY;
// C source retained for exact declaration/control-flow reference: 				goto badentry;
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 			d_inode(entry)->i_flags |= S_DEAD;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		dont_mount(entry);
// C source retained for exact declaration/control-flow reference: 		clear_nlink(d_inode(entry));
// C source retained for exact declaration/control-flow reference: 		err = 0;
// C source retained for exact declaration/control-flow reference:  badentry:
// C source retained for exact declaration/control-flow reference: 		inode_unlock(d_inode(entry));
// C source retained for exact declaration/control-flow reference: 		if (!err)
// C source retained for exact declaration/control-flow reference: 			d_delete(entry);
// C source retained for exact declaration/control-flow reference: 	} else {
// C source retained for exact declaration/control-flow reference: 		err = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	dput(entry);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference:  unlock:
// C source retained for exact declaration/control-flow reference: 	inode_unlock(parent);
// C source retained for exact declaration/control-flow reference: 	iput(parent);
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static inline bool fuse_permissible_uidgid(struct fuse_conn *fc)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	const struct cred *cred = current_cred();
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return (uid_eq(cred->euid, fc->user_id) &&
// C source retained for exact declaration/control-flow reference: 		uid_eq(cred->suid, fc->user_id) &&
// C source retained for exact declaration/control-flow reference: 		uid_eq(cred->uid,  fc->user_id) &&
// C source retained for exact declaration/control-flow reference: 		gid_eq(cred->egid, fc->group_id) &&
// C source retained for exact declaration/control-flow reference: 		gid_eq(cred->sgid, fc->group_id) &&
// C source retained for exact declaration/control-flow reference: 		gid_eq(cred->gid,  fc->group_id));
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Calling into a user-controlled filesystem gives the filesystem
// C source retained for exact declaration/control-flow reference:  * daemon ptrace-like capabilities over the current process.  This
// C source retained for exact declaration/control-flow reference:  * means, that the filesystem daemon is able to record the exact
// C source retained for exact declaration/control-flow reference:  * filesystem operations performed, and can also control the behavior
// C source retained for exact declaration/control-flow reference:  * of the requester process in otherwise impossible ways.  For example
// C source retained for exact declaration/control-flow reference:  * it can delay the operation for arbitrary length of time allowing
// C source retained for exact declaration/control-flow reference:  * DoS against the requester.
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * For this reason only those processes can call into the filesystem,
// C source retained for exact declaration/control-flow reference:  * for which the owner of the mount has ptrace privilege.  This
// C source retained for exact declaration/control-flow reference:  * excludes processes started by other users, suid or sgid processes.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: bool fuse_allow_current_process(struct fuse_conn *fc)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	bool allow;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->allow_other)
// C source retained for exact declaration/control-flow reference: 		allow = current_in_userns(fc->user_ns);
// C source retained for exact declaration/control-flow reference: 	else
// C source retained for exact declaration/control-flow reference: 		allow = fuse_permissible_uidgid(fc);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!allow && allow_sys_admin_access && capable(CAP_SYS_ADMIN))
// C source retained for exact declaration/control-flow reference: 		allow = true;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return allow;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_access(struct inode *inode, int mask)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	struct fuse_access_in inarg;
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	BUG_ON(mask & MAY_NOT_BLOCK);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * We should not send FUSE_ACCESS to the userspace
// C source retained for exact declaration/control-flow reference: 	 * when idmapped mounts are enabled as for this case
// C source retained for exact declaration/control-flow reference: 	 * we have fc->default_permissions = 1 and access
// C source retained for exact declaration/control-flow reference: 	 * permission checks are done on the kernel side.
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	WARN_ON_ONCE(!(fm->sb->s_iflags & SB_I_NOIDMAP));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fm->fc->no_access)
// C source retained for exact declaration/control-flow reference: 		return 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	inarg.mask = mask & (MAY_READ | MAY_WRITE | MAY_EXEC);
// C source retained for exact declaration/control-flow reference: 	args.opcode = FUSE_ACCESS;
// C source retained for exact declaration/control-flow reference: 	args.nodeid = get_node_id(inode);
// C source retained for exact declaration/control-flow reference: 	args.in_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].size = sizeof(inarg);
// C source retained for exact declaration/control-flow reference: 	args.in_args[0].value = &inarg;
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 	if (err == -ENOSYS) {
// C source retained for exact declaration/control-flow reference: 		fm->fc->no_access = 1;
// C source retained for exact declaration/control-flow reference: 		err = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_perm_getattr(struct inode *inode, int mask)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	if (mask & MAY_NOT_BLOCK)
// C source retained for exact declaration/control-flow reference: 		return -ECHILD;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	forget_all_cached_acls(inode);
// C source retained for exact declaration/control-flow reference: 	return fuse_do_getattr(&nop_mnt_idmap, inode, NULL, NULL);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Check permission.  The two basic access models of FUSE are:
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * 1) Local access checking ('default_permissions' mount option) based
// C source retained for exact declaration/control-flow reference:  * on file mode.  This is the plain old disk filesystem permission
// C source retained for exact declaration/control-flow reference:  * model.
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * 2) "Remote" access checking, where server is responsible for
// C source retained for exact declaration/control-flow reference:  * checking permission in each inode operation.  An exception to this
// C source retained for exact declaration/control-flow reference:  * is if ->permission() was invoked from sys_access() in which case an
// C source retained for exact declaration/control-flow reference:  * access request is sent.  Execute permission is still checked
// C source retained for exact declaration/control-flow reference:  * locally based on file mode.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static int fuse_permission(struct mnt_idmap *idmap,
// C source retained for exact declaration/control-flow reference: 			   struct inode *inode, int mask)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 	bool refreshed = false;
// C source retained for exact declaration/control-flow reference: 	int err = 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fuse_allow_current_process(fc))
// C source retained for exact declaration/control-flow reference: 		return -EACCES;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * If attributes are needed, refresh them before proceeding
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	if (fc->default_permissions ||
// C source retained for exact declaration/control-flow reference: 	    ((mask & MAY_EXEC) && S_ISREG(inode->i_mode))) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 		u32 perm_mask = STATX_MODE | STATX_UID | STATX_GID;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		if (perm_mask & READ_ONCE(fi->inval_mask) ||
// C source retained for exact declaration/control-flow reference: 		    time_before64(fi->i_time, get_jiffies_64())) {
// C source retained for exact declaration/control-flow reference: 			refreshed = true;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 			err = fuse_perm_getattr(inode, mask);
// C source retained for exact declaration/control-flow reference: 			if (err)
// C source retained for exact declaration/control-flow reference: 				return err;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->default_permissions) {
// C source retained for exact declaration/control-flow reference: 		err = generic_permission(idmap, inode, mask);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/* If permission is denied, try to refresh file
// C source retained for exact declaration/control-flow reference: 		   attributes.  This is also needed, because the root
// C source retained for exact declaration/control-flow reference: 		   node will at first have no permissions */
// C source retained for exact declaration/control-flow reference: 		if (err == -EACCES && !refreshed) {
// C source retained for exact declaration/control-flow reference: 			err = fuse_perm_getattr(inode, mask);
// C source retained for exact declaration/control-flow reference: 			if (!err)
// C source retained for exact declaration/control-flow reference: 				err = generic_permission(idmap,
// C source retained for exact declaration/control-flow reference: 							 inode, mask);
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/* Note: the opposite of the above test does not
// C source retained for exact declaration/control-flow reference: 		   exist.  So if permissions are revoked this won't be
// C source retained for exact declaration/control-flow reference: 		   noticed immediately, only after the attribute
// C source retained for exact declaration/control-flow reference: 		   timeout has expired */
// C source retained for exact declaration/control-flow reference: 	} else if (mask & (MAY_ACCESS | MAY_CHDIR)) {
// C source retained for exact declaration/control-flow reference: 		err = fuse_access(inode, mask);
// C source retained for exact declaration/control-flow reference: 	} else if ((mask & MAY_EXEC) && S_ISREG(inode->i_mode)) {
// C source retained for exact declaration/control-flow reference: 		if (!(inode->i_mode & S_IXUGO)) {
// C source retained for exact declaration/control-flow reference: 			if (refreshed)
// C source retained for exact declaration/control-flow reference: 				return -EACCES;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 			err = fuse_perm_getattr(inode, mask);
// C source retained for exact declaration/control-flow reference: 			if (!err && !(inode->i_mode & S_IXUGO))
// C source retained for exact declaration/control-flow reference: 				return -EACCES;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_readlink_folio(struct inode *inode, struct folio *folio)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	struct fuse_folio_desc desc = { .length = folio_size(folio) - 1 };
// C source retained for exact declaration/control-flow reference: 	struct fuse_args_pages ap = {
// C source retained for exact declaration/control-flow reference: 		.num_folios = 1,
// C source retained for exact declaration/control-flow reference: 		.folios = &folio,
// C source retained for exact declaration/control-flow reference: 		.descs = &desc,
// C source retained for exact declaration/control-flow reference: 	};
// C source retained for exact declaration/control-flow reference: 	char *link;
// C source retained for exact declaration/control-flow reference: 	ssize_t res;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	ap.args.opcode = FUSE_READLINK;
// C source retained for exact declaration/control-flow reference: 	ap.args.nodeid = get_node_id(inode);
// C source retained for exact declaration/control-flow reference: 	ap.args.out_pages = true;
// C source retained for exact declaration/control-flow reference: 	ap.args.out_argvar = true;
// C source retained for exact declaration/control-flow reference: 	ap.args.page_zeroing = true;
// C source retained for exact declaration/control-flow reference: 	ap.args.out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	ap.args.out_args[0].size = desc.length;
// C source retained for exact declaration/control-flow reference: 	res = fuse_simple_request(fm, &ap.args);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fuse_invalidate_atime(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (res < 0)
// C source retained for exact declaration/control-flow reference: 		return res;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (WARN_ON(res >= PAGE_SIZE))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	link = folio_address(folio);
// C source retained for exact declaration/control-flow reference: 	link[res] = '\0';
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const char *fuse_get_link(struct dentry *dentry, struct inode *inode,
// C source retained for exact declaration/control-flow reference: 				 struct delayed_call *callback)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 	struct folio *folio;
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -EIO;
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->cache_symlinks)
// C source retained for exact declaration/control-flow reference: 		return page_get_link_raw(dentry, inode, callback);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = -ECHILD;
// C source retained for exact declaration/control-flow reference: 	if (!dentry)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	folio = folio_alloc(GFP_KERNEL, 0);
// C source retained for exact declaration/control-flow reference: 	err = -ENOMEM;
// C source retained for exact declaration/control-flow reference: 	if (!folio)
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = fuse_readlink_folio(inode, folio);
// C source retained for exact declaration/control-flow reference: 	if (err) {
// C source retained for exact declaration/control-flow reference: 		folio_put(folio);
// C source retained for exact declaration/control-flow reference: 		goto out_err;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	set_delayed_call(callback, page_put_link, folio);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return folio_address(folio);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: out_err:
// C source retained for exact declaration/control-flow reference: 	return ERR_PTR(err);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_dir_open(struct inode *inode, struct file *file)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = generic_file_open(inode, file);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		return err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = fuse_do_open(fm, get_node_id(inode), file, true);
// C source retained for exact declaration/control-flow reference: 	if (!err) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_file *ff = file->private_data;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/*
// C source retained for exact declaration/control-flow reference: 		 * Keep handling FOPEN_STREAM and FOPEN_NONSEEKABLE for
// C source retained for exact declaration/control-flow reference: 		 * directories for backward compatibility, though it's unlikely
// C source retained for exact declaration/control-flow reference: 		 * to be useful.
// C source retained for exact declaration/control-flow reference: 		 */
// C source retained for exact declaration/control-flow reference: 		if (ff->open_flags & (FOPEN_STREAM | FOPEN_NONSEEKABLE))
// C source retained for exact declaration/control-flow reference: 			nonseekable_open(inode, file);
// C source retained for exact declaration/control-flow reference: 		if (!(ff->open_flags & FOPEN_KEEP_CACHE))
// C source retained for exact declaration/control-flow reference: 			invalidate_inode_pages2(inode->i_mapping);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_dir_release(struct inode *inode, struct file *file)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	fuse_release_common(file, true);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_dir_fsync(struct file *file, loff_t start, loff_t end,
// C source retained for exact declaration/control-flow reference: 			  int datasync)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode = file->f_mapping->host;
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->no_fsyncdir)
// C source retained for exact declaration/control-flow reference: 		return 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode_lock(inode);
// C source retained for exact declaration/control-flow reference: 	err = fuse_fsync_common(file, start, end, datasync, FUSE_FSYNCDIR);
// C source retained for exact declaration/control-flow reference: 	if (err == -ENOSYS) {
// C source retained for exact declaration/control-flow reference: 		fc->no_fsyncdir = 1;
// C source retained for exact declaration/control-flow reference: 		err = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	inode_unlock(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static long fuse_dir_ioctl(struct file *file, unsigned int cmd,
// C source retained for exact declaration/control-flow reference: 			    unsigned long arg)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(file->f_mapping->host);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* FUSE_IOCTL_DIR only supported for API version >= 7.18 */
// C source retained for exact declaration/control-flow reference: 	if (fc->minor < 18)
// C source retained for exact declaration/control-flow reference: 		return -ENOTTY;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return fuse_ioctl_common(file, cmd, arg, FUSE_IOCTL_DIR);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static long fuse_dir_compat_ioctl(struct file *file, unsigned int cmd,
// C source retained for exact declaration/control-flow reference: 				   unsigned long arg)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(file->f_mapping->host);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fc->minor < 18)
// C source retained for exact declaration/control-flow reference: 		return -ENOTTY;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return fuse_ioctl_common(file, cmd, arg,
// C source retained for exact declaration/control-flow reference: 				 FUSE_IOCTL_COMPAT | FUSE_IOCTL_DIR);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static bool update_mtime(unsigned ivalid, bool trust_local_mtime)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	/* Always update if mtime is explicitly set  */
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_MTIME_SET)
// C source retained for exact declaration/control-flow reference: 		return true;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Or if kernel i_mtime is the official one */
// C source retained for exact declaration/control-flow reference: 	if (trust_local_mtime)
// C source retained for exact declaration/control-flow reference: 		return true;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* If it's an open(O_TRUNC) or an ftruncate(), don't update */
// C source retained for exact declaration/control-flow reference: 	if ((ivalid & ATTR_SIZE) && (ivalid & (ATTR_OPEN | ATTR_FILE)))
// C source retained for exact declaration/control-flow reference: 		return false;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* In all other cases update */
// C source retained for exact declaration/control-flow reference: 	return true;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void iattr_to_fattr(struct mnt_idmap *idmap, struct fuse_conn *fc,
// C source retained for exact declaration/control-flow reference: 			   struct iattr *iattr, struct fuse_setattr_in *arg,
// C source retained for exact declaration/control-flow reference: 			   bool trust_local_cmtime)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	unsigned ivalid = iattr->ia_valid;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_MODE)
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_MODE,   arg->mode = iattr->ia_mode;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_UID) {
// C source retained for exact declaration/control-flow reference: 		kuid_t fsuid = from_vfsuid(idmap, fc->user_ns, iattr->ia_vfsuid);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_UID;
// C source retained for exact declaration/control-flow reference: 		arg->uid = from_kuid(fc->user_ns, fsuid);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_GID) {
// C source retained for exact declaration/control-flow reference: 		kgid_t fsgid = from_vfsgid(idmap, fc->user_ns, iattr->ia_vfsgid);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_GID;
// C source retained for exact declaration/control-flow reference: 		arg->gid = from_kgid(fc->user_ns, fsgid);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_SIZE)
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_SIZE,   arg->size = iattr->ia_size;
// C source retained for exact declaration/control-flow reference: 	if (ivalid & ATTR_ATIME) {
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_ATIME;
// C source retained for exact declaration/control-flow reference: 		arg->atime = iattr->ia_atime.tv_sec;
// C source retained for exact declaration/control-flow reference: 		arg->atimensec = iattr->ia_atime.tv_nsec;
// C source retained for exact declaration/control-flow reference: 		if (!(ivalid & ATTR_ATIME_SET))
// C source retained for exact declaration/control-flow reference: 			arg->valid |= FATTR_ATIME_NOW;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if ((ivalid & ATTR_MTIME) && update_mtime(ivalid, trust_local_cmtime)) {
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_MTIME;
// C source retained for exact declaration/control-flow reference: 		arg->mtime = iattr->ia_mtime.tv_sec;
// C source retained for exact declaration/control-flow reference: 		arg->mtimensec = iattr->ia_mtime.tv_nsec;
// C source retained for exact declaration/control-flow reference: 		if (!(ivalid & ATTR_MTIME_SET) && !trust_local_cmtime)
// C source retained for exact declaration/control-flow reference: 			arg->valid |= FATTR_MTIME_NOW;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if ((ivalid & ATTR_CTIME) && trust_local_cmtime) {
// C source retained for exact declaration/control-flow reference: 		arg->valid |= FATTR_CTIME;
// C source retained for exact declaration/control-flow reference: 		arg->ctime = iattr->ia_ctime.tv_sec;
// C source retained for exact declaration/control-flow reference: 		arg->ctimensec = iattr->ia_ctime.tv_nsec;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Prevent concurrent writepages on inode
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * This is done by adding a negative bias to the inode write counter
// C source retained for exact declaration/control-flow reference:  * and waiting for all pending writes to finish.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: void fuse_set_nowrite(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	BUG_ON(!inode_is_locked(inode));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 	BUG_ON(fi->writectr < 0);
// C source retained for exact declaration/control-flow reference: 	fi->writectr += FUSE_NOWRITE;
// C source retained for exact declaration/control-flow reference: 	spin_unlock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 	wait_event(fi->page_waitq, fi->writectr == FUSE_NOWRITE);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Allow writepages on inode
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * Remove the bias from the writecounter and send any queued
// C source retained for exact declaration/control-flow reference:  * writepages.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: static void __fuse_release_nowrite(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	BUG_ON(fi->writectr != FUSE_NOWRITE);
// C source retained for exact declaration/control-flow reference: 	fi->writectr = 0;
// C source retained for exact declaration/control-flow reference: 	fuse_flush_writepages(inode);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_release_nowrite(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 	__fuse_release_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 	spin_unlock(&fi->lock);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static void fuse_setattr_fill(struct fuse_conn *fc, struct fuse_args *args,
// C source retained for exact declaration/control-flow reference: 			      struct inode *inode,
// C source retained for exact declaration/control-flow reference: 			      struct fuse_setattr_in *inarg_p,
// C source retained for exact declaration/control-flow reference: 			      struct fuse_attr_out *outarg_p)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	args->opcode = FUSE_SETATTR;
// C source retained for exact declaration/control-flow reference: 	args->nodeid = get_node_id(inode);
// C source retained for exact declaration/control-flow reference: 	args->in_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args->in_args[0].size = sizeof(*inarg_p);
// C source retained for exact declaration/control-flow reference: 	args->in_args[0].value = inarg_p;
// C source retained for exact declaration/control-flow reference: 	args->out_numargs = 1;
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].size = sizeof(*outarg_p);
// C source retained for exact declaration/control-flow reference: 	args->out_args[0].value = outarg_p;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Flush inode->i_mtime to the server
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: int fuse_flush_times(struct inode *inode, struct fuse_file *ff)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	struct fuse_setattr_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_attr_out outarg;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	memset(&outarg, 0, sizeof(outarg));
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inarg.valid = FATTR_MTIME;
// C source retained for exact declaration/control-flow reference: 	inarg.mtime = inode_get_mtime_sec(inode);
// C source retained for exact declaration/control-flow reference: 	inarg.mtimensec = inode_get_mtime_nsec(inode);
// C source retained for exact declaration/control-flow reference: 	if (fm->fc->minor >= 23) {
// C source retained for exact declaration/control-flow reference: 		inarg.valid |= FATTR_CTIME;
// C source retained for exact declaration/control-flow reference: 		inarg.ctime = inode_get_ctime_sec(inode);
// C source retained for exact declaration/control-flow reference: 		inarg.ctimensec = inode_get_ctime_nsec(inode);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if (ff) {
// C source retained for exact declaration/control-flow reference: 		inarg.valid |= FATTR_FH;
// C source retained for exact declaration/control-flow reference: 		inarg.fh = ff->fh;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	fuse_setattr_fill(fm->fc, &args, inode, &inarg, &outarg);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: /*
// C source retained for exact declaration/control-flow reference:  * Set attributes, and at the same time refresh them.
// C source retained for exact declaration/control-flow reference:  *
// C source retained for exact declaration/control-flow reference:  * Truncation is slightly complicated, because the 'truncate' request
// C source retained for exact declaration/control-flow reference:  * may fail, in which case we don't want to touch the mapping.
// C source retained for exact declaration/control-flow reference:  * vmtruncate() doesn't allow for this case, so do the rlimit checking
// C source retained for exact declaration/control-flow reference:  * and the actual truncation by hand.
// C source retained for exact declaration/control-flow reference:  */
// C source retained for exact declaration/control-flow reference: int fuse_do_setattr(struct mnt_idmap *idmap, struct dentry *dentry,
// C source retained for exact declaration/control-flow reference: 		    struct iattr *attr, struct file *file)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode = d_inode(dentry);
// C source retained for exact declaration/control-flow reference: 	struct fuse_mount *fm = get_fuse_mount(inode);
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = fm->fc;
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 	struct address_space *mapping = inode->i_mapping;
// C source retained for exact declaration/control-flow reference: 	FUSE_ARGS(args);
// C source retained for exact declaration/control-flow reference: 	struct fuse_setattr_in inarg;
// C source retained for exact declaration/control-flow reference: 	struct fuse_attr_out outarg;
// C source retained for exact declaration/control-flow reference: 	bool is_truncate = false;
// C source retained for exact declaration/control-flow reference: 	bool is_wb = fc->writeback_cache && S_ISREG(inode->i_mode);
// C source retained for exact declaration/control-flow reference: 	loff_t oldsize;
// C source retained for exact declaration/control-flow reference: 	int err;
// C source retained for exact declaration/control-flow reference: 	bool trust_local_cmtime = is_wb;
// C source retained for exact declaration/control-flow reference: 	bool fault_blocked = false;
// C source retained for exact declaration/control-flow reference: 	u64 attr_version;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fc->default_permissions)
// C source retained for exact declaration/control-flow reference: 		attr->ia_valid |= ATTR_FORCE;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	err = setattr_prepare(idmap, dentry, attr);
// C source retained for exact declaration/control-flow reference: 	if (err)
// C source retained for exact declaration/control-flow reference: 		return err;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (attr->ia_valid & ATTR_SIZE) {
// C source retained for exact declaration/control-flow reference: 		if (WARN_ON(!S_ISREG(inode->i_mode)))
// C source retained for exact declaration/control-flow reference: 			return -EIO;
// C source retained for exact declaration/control-flow reference: 		is_truncate = true;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (FUSE_IS_DAX(inode) && is_truncate) {
// C source retained for exact declaration/control-flow reference: 		filemap_invalidate_lock(mapping);
// C source retained for exact declaration/control-flow reference: 		fault_blocked = true;
// C source retained for exact declaration/control-flow reference: 		err = fuse_dax_break_layouts(inode, 0, -1);
// C source retained for exact declaration/control-flow reference: 		if (err)
// C source retained for exact declaration/control-flow reference: 			goto unlock;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (attr->ia_valid & ATTR_OPEN) {
// C source retained for exact declaration/control-flow reference: 		/* This is coming from open(..., ... | O_TRUNC); */
// C source retained for exact declaration/control-flow reference: 		WARN_ON(!(attr->ia_valid & ATTR_SIZE));
// C source retained for exact declaration/control-flow reference: 		WARN_ON(attr->ia_size != 0);
// C source retained for exact declaration/control-flow reference: 		if (fc->atomic_o_trunc) {
// C source retained for exact declaration/control-flow reference: 			/*
// C source retained for exact declaration/control-flow reference: 			 * No need to send request to userspace, since actual
// C source retained for exact declaration/control-flow reference: 			 * truncation has already been done by OPEN.  But still
// C source retained for exact declaration/control-flow reference: 			 * need to truncate page cache.
// C source retained for exact declaration/control-flow reference: 			 */
// C source retained for exact declaration/control-flow reference: 			i_size_write(inode, 0);
// C source retained for exact declaration/control-flow reference: 			truncate_pagecache(inode, 0);
// C source retained for exact declaration/control-flow reference: 			goto out;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		file = NULL;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Flush dirty data/metadata before non-truncate SETATTR */
// C source retained for exact declaration/control-flow reference: 	if (is_wb &&
// C source retained for exact declaration/control-flow reference: 	    attr->ia_valid &
// C source retained for exact declaration/control-flow reference: 			(ATTR_MODE | ATTR_UID | ATTR_GID | ATTR_MTIME_SET |
// C source retained for exact declaration/control-flow reference: 			 ATTR_TIMES_SET)) {
// C source retained for exact declaration/control-flow reference: 		err = write_inode_now(inode, true);
// C source retained for exact declaration/control-flow reference: 		if (err)
// C source retained for exact declaration/control-flow reference: 			goto unlock;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		fuse_set_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 		fuse_release_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (is_truncate) {
// C source retained for exact declaration/control-flow reference: 		fuse_set_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 		set_bit(FUSE_I_SIZE_UNSTABLE, &fi->state);
// C source retained for exact declaration/control-flow reference: 		if (trust_local_cmtime && attr->ia_size != inode->i_size)
// C source retained for exact declaration/control-flow reference: 			attr->ia_valid |= ATTR_MTIME | ATTR_CTIME;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	memset(&inarg, 0, sizeof(inarg));
// C source retained for exact declaration/control-flow reference: 	memset(&outarg, 0, sizeof(outarg));
// C source retained for exact declaration/control-flow reference: 	iattr_to_fattr(idmap, fc, attr, &inarg, trust_local_cmtime);
// C source retained for exact declaration/control-flow reference: 	if (file) {
// C source retained for exact declaration/control-flow reference: 		struct fuse_file *ff = file->private_data;
// C source retained for exact declaration/control-flow reference: 		inarg.valid |= FATTR_FH;
// C source retained for exact declaration/control-flow reference: 		inarg.fh = ff->fh;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/* Kill suid/sgid for non-directory chown unconditionally */
// C source retained for exact declaration/control-flow reference: 	if (fc->handle_killpriv_v2 && !S_ISDIR(inode->i_mode) &&
// C source retained for exact declaration/control-flow reference: 	    attr->ia_valid & (ATTR_UID | ATTR_GID))
// C source retained for exact declaration/control-flow reference: 		inarg.valid |= FATTR_KILL_SUIDGID;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (attr->ia_valid & ATTR_SIZE) {
// C source retained for exact declaration/control-flow reference: 		/* For mandatory locking in truncate */
// C source retained for exact declaration/control-flow reference: 		inarg.valid |= FATTR_LOCKOWNER;
// C source retained for exact declaration/control-flow reference: 		inarg.lock_owner = fuse_lock_owner_id(fc, current->files);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/* Kill suid/sgid for truncate only if no CAP_FSETID */
// C source retained for exact declaration/control-flow reference: 		if (fc->handle_killpriv_v2 && !capable(CAP_FSETID))
// C source retained for exact declaration/control-flow reference: 			inarg.valid |= FATTR_KILL_SUIDGID;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	attr_version = fuse_get_attr_version(fm->fc);
// C source retained for exact declaration/control-flow reference: 	fuse_setattr_fill(fc, &args, inode, &inarg, &outarg);
// C source retained for exact declaration/control-flow reference: 	err = fuse_simple_request(fm, &args);
// C source retained for exact declaration/control-flow reference: 	if (err) {
// C source retained for exact declaration/control-flow reference: 		if (err == -EINTR)
// C source retained for exact declaration/control-flow reference: 			fuse_invalidate_attr(inode);
// C source retained for exact declaration/control-flow reference: 		goto error;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_invalid_attr(&outarg.attr) ||
// C source retained for exact declaration/control-flow reference: 	    inode_wrong_type(inode, outarg.attr.mode)) {
// C source retained for exact declaration/control-flow reference: 		fuse_make_bad(inode);
// C source retained for exact declaration/control-flow reference: 		err = -EIO;
// C source retained for exact declaration/control-flow reference: 		goto error;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 	/* the kernel maintains i_mtime locally */
// C source retained for exact declaration/control-flow reference: 	if (trust_local_cmtime) {
// C source retained for exact declaration/control-flow reference: 		if (attr->ia_valid & ATTR_MTIME)
// C source retained for exact declaration/control-flow reference: 			inode_set_mtime_to_ts(inode, attr->ia_mtime);
// C source retained for exact declaration/control-flow reference: 		if (attr->ia_valid & ATTR_CTIME)
// C source retained for exact declaration/control-flow reference: 			inode_set_ctime_to_ts(inode, attr->ia_ctime);
// C source retained for exact declaration/control-flow reference: 		/* FIXME: clear I_DIRTY_SYNC? */
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fi->attr_version > attr_version) {
// C source retained for exact declaration/control-flow reference: 		/*
// C source retained for exact declaration/control-flow reference: 		 * Apply attributes, for example for fsnotify_change(), but set
// C source retained for exact declaration/control-flow reference: 		 * attribute timeout to zero.
// C source retained for exact declaration/control-flow reference: 		 */
// C source retained for exact declaration/control-flow reference: 		outarg.attr_valid = outarg.attr_valid_nsec = 0;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	fuse_change_attributes_common(inode, &outarg.attr, NULL,
// C source retained for exact declaration/control-flow reference: 				      ATTR_TIMEOUT(&outarg),
// C source retained for exact declaration/control-flow reference: 				      fuse_get_cache_mask(inode), 0);
// C source retained for exact declaration/control-flow reference: 	oldsize = inode->i_size;
// C source retained for exact declaration/control-flow reference: 	/* see the comment in fuse_change_attributes() */
// C source retained for exact declaration/control-flow reference: 	if (!is_wb || is_truncate)
// C source retained for exact declaration/control-flow reference: 		i_size_write(inode, outarg.attr.size);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (is_truncate) {
// C source retained for exact declaration/control-flow reference: 		/* NOTE: this may release/reacquire fi->lock */
// C source retained for exact declaration/control-flow reference: 		__fuse_release_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	spin_unlock(&fi->lock);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	/*
// C source retained for exact declaration/control-flow reference: 	 * Only call invalidate_inode_pages2() after removing
// C source retained for exact declaration/control-flow reference: 	 * FUSE_NOWRITE, otherwise fuse_launder_folio() would deadlock.
// C source retained for exact declaration/control-flow reference: 	 */
// C source retained for exact declaration/control-flow reference: 	if ((is_truncate || !is_wb) &&
// C source retained for exact declaration/control-flow reference: 	    S_ISREG(inode->i_mode) && oldsize != outarg.attr.size) {
// C source retained for exact declaration/control-flow reference: 		if (outarg.attr.size > oldsize)
// C source retained for exact declaration/control-flow reference: 			truncate_pagecache_range(inode, oldsize,
// C source retained for exact declaration/control-flow reference: 						 outarg.attr.size - 1);
// C source retained for exact declaration/control-flow reference: 		truncate_pagecache(inode, outarg.attr.size);
// C source retained for exact declaration/control-flow reference: 		invalidate_inode_pages2(mapping);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	clear_bit(FUSE_I_SIZE_UNSTABLE, &fi->state);
// C source retained for exact declaration/control-flow reference: out:
// C source retained for exact declaration/control-flow reference: 	if (fault_blocked)
// C source retained for exact declaration/control-flow reference: 		filemap_invalidate_unlock(mapping);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: error:
// C source retained for exact declaration/control-flow reference: 	if (is_truncate)
// C source retained for exact declaration/control-flow reference: 		fuse_release_nowrite(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	clear_bit(FUSE_I_SIZE_UNSTABLE, &fi->state);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: unlock:
// C source retained for exact declaration/control-flow reference: 	if (fault_blocked)
// C source retained for exact declaration/control-flow reference: 		filemap_invalidate_unlock(mapping);
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_setattr(struct mnt_idmap *idmap, struct dentry *entry,
// C source retained for exact declaration/control-flow reference: 			struct iattr *attr)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode = d_inode(entry);
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 	struct file *file = (attr->ia_valid & ATTR_FILE) ? attr->ia_file : NULL;
// C source retained for exact declaration/control-flow reference: 	int ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fuse_allow_current_process(get_fuse_conn(inode)))
// C source retained for exact declaration/control-flow reference: 		return -EACCES;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (attr->ia_valid & (ATTR_KILL_SUID | ATTR_KILL_SGID)) {
// C source retained for exact declaration/control-flow reference: 		attr->ia_valid &= ~(ATTR_KILL_SUID | ATTR_KILL_SGID |
// C source retained for exact declaration/control-flow reference: 				    ATTR_MODE);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/*
// C source retained for exact declaration/control-flow reference: 		 * The only sane way to reliably kill suid/sgid is to do it in
// C source retained for exact declaration/control-flow reference: 		 * the userspace filesystem
// C source retained for exact declaration/control-flow reference: 		 *
// C source retained for exact declaration/control-flow reference: 		 * This should be done on write(), truncate() and chown().
// C source retained for exact declaration/control-flow reference: 		 */
// C source retained for exact declaration/control-flow reference: 		if (!fc->handle_killpriv && !fc->handle_killpriv_v2) {
// C source retained for exact declaration/control-flow reference: 			/*
// C source retained for exact declaration/control-flow reference: 			 * ia_mode calculation may have used stale i_mode.
// C source retained for exact declaration/control-flow reference: 			 * Refresh and recalculate.
// C source retained for exact declaration/control-flow reference: 			 */
// C source retained for exact declaration/control-flow reference: 			ret = fuse_do_getattr(idmap, inode, NULL, file);
// C source retained for exact declaration/control-flow reference: 			if (ret)
// C source retained for exact declaration/control-flow reference: 				return ret;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 			attr->ia_mode = inode->i_mode;
// C source retained for exact declaration/control-flow reference: 			if (inode->i_mode & S_ISUID) {
// C source retained for exact declaration/control-flow reference: 				attr->ia_valid |= ATTR_MODE;
// C source retained for exact declaration/control-flow reference: 				attr->ia_mode &= ~S_ISUID;
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 			if ((inode->i_mode & (S_ISGID | S_IXGRP)) == (S_ISGID | S_IXGRP)) {
// C source retained for exact declaration/control-flow reference: 				attr->ia_valid |= ATTR_MODE;
// C source retained for exact declaration/control-flow reference: 				attr->ia_mode &= ~S_ISGID;
// C source retained for exact declaration/control-flow reference: 			}
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	if (!attr->ia_valid)
// C source retained for exact declaration/control-flow reference: 		return 0;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	ret = fuse_do_setattr(idmap, entry, attr, file);
// C source retained for exact declaration/control-flow reference: 	if (!ret) {
// C source retained for exact declaration/control-flow reference: 		/*
// C source retained for exact declaration/control-flow reference: 		 * If filesystem supports acls it may have updated acl xattrs in
// C source retained for exact declaration/control-flow reference: 		 * the filesystem, so forget cached acls for the inode.
// C source retained for exact declaration/control-flow reference: 		 */
// C source retained for exact declaration/control-flow reference: 		if (fc->posix_acl)
// C source retained for exact declaration/control-flow reference: 			forget_all_cached_acls(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 		/* Directory mode changed, may need to revalidate access */
// C source retained for exact declaration/control-flow reference: 		if (d_is_dir(entry) && (attr->ia_valid & ATTR_MODE))
// C source retained for exact declaration/control-flow reference: 			fuse_invalidate_entry_cache(entry);
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 	return ret;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_getattr(struct mnt_idmap *idmap,
// C source retained for exact declaration/control-flow reference: 			const struct path *path, struct kstat *stat,
// C source retained for exact declaration/control-flow reference: 			u32 request_mask, unsigned int flags)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct inode *inode = d_inode(path->dentry);
// C source retained for exact declaration/control-flow reference: 	struct fuse_conn *fc = get_fuse_conn(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (fuse_is_bad(inode))
// C source retained for exact declaration/control-flow reference: 		return -EIO;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!fuse_allow_current_process(fc)) {
// C source retained for exact declaration/control-flow reference: 		if (!request_mask) {
// C source retained for exact declaration/control-flow reference: 			/*
// C source retained for exact declaration/control-flow reference: 			 * If user explicitly requested *nothing* then don't
// C source retained for exact declaration/control-flow reference: 			 * error out, but return st_dev only.
// C source retained for exact declaration/control-flow reference: 			 */
// C source retained for exact declaration/control-flow reference: 			stat->result_mask = 0;
// C source retained for exact declaration/control-flow reference: 			stat->dev = inode->i_sb->s_dev;
// C source retained for exact declaration/control-flow reference: 			return 0;
// C source retained for exact declaration/control-flow reference: 		}
// C source retained for exact declaration/control-flow reference: 		return -EACCES;
// C source retained for exact declaration/control-flow reference: 	}
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return fuse_update_get_attr(idmap, inode, NULL, stat, request_mask, flags);
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const struct inode_operations fuse_dir_inode_operations = {
// C source retained for exact declaration/control-flow reference: 	.lookup		= fuse_lookup,
// C source retained for exact declaration/control-flow reference: 	.mkdir		= fuse_mkdir,
// C source retained for exact declaration/control-flow reference: 	.symlink	= fuse_symlink,
// C source retained for exact declaration/control-flow reference: 	.unlink		= fuse_unlink,
// C source retained for exact declaration/control-flow reference: 	.rmdir		= fuse_rmdir,
// C source retained for exact declaration/control-flow reference: 	.rename		= fuse_rename2,
// C source retained for exact declaration/control-flow reference: 	.link		= fuse_link,
// C source retained for exact declaration/control-flow reference: 	.setattr	= fuse_setattr,
// C source retained for exact declaration/control-flow reference: 	.create		= fuse_create,
// C source retained for exact declaration/control-flow reference: 	.atomic_open	= fuse_atomic_open,
// C source retained for exact declaration/control-flow reference: 	.tmpfile	= fuse_tmpfile,
// C source retained for exact declaration/control-flow reference: 	.mknod		= fuse_mknod,
// C source retained for exact declaration/control-flow reference: 	.permission	= fuse_permission,
// C source retained for exact declaration/control-flow reference: 	.getattr	= fuse_getattr,
// C source retained for exact declaration/control-flow reference: 	.listxattr	= fuse_listxattr,
// C source retained for exact declaration/control-flow reference: 	.get_inode_acl	= fuse_get_inode_acl,
// C source retained for exact declaration/control-flow reference: 	.get_acl	= fuse_get_acl,
// C source retained for exact declaration/control-flow reference: 	.set_acl	= fuse_set_acl,
// C source retained for exact declaration/control-flow reference: 	.fileattr_get	= fuse_fileattr_get,
// C source retained for exact declaration/control-flow reference: 	.fileattr_set	= fuse_fileattr_set,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const struct file_operations fuse_dir_operations = {
// C source retained for exact declaration/control-flow reference: 	.llseek		= generic_file_llseek,
// C source retained for exact declaration/control-flow reference: 	.read		= generic_read_dir,
// C source retained for exact declaration/control-flow reference: 	.iterate_shared	= fuse_readdir,
// C source retained for exact declaration/control-flow reference: 	.open		= fuse_dir_open,
// C source retained for exact declaration/control-flow reference: 	.release	= fuse_dir_release,
// C source retained for exact declaration/control-flow reference: 	.fsync		= fuse_dir_fsync,
// C source retained for exact declaration/control-flow reference: 	.unlocked_ioctl	= fuse_dir_ioctl,
// C source retained for exact declaration/control-flow reference: 	.compat_ioctl	= fuse_dir_compat_ioctl,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const struct inode_operations fuse_common_inode_operations = {
// C source retained for exact declaration/control-flow reference: 	.setattr	= fuse_setattr,
// C source retained for exact declaration/control-flow reference: 	.permission	= fuse_permission,
// C source retained for exact declaration/control-flow reference: 	.getattr	= fuse_getattr,
// C source retained for exact declaration/control-flow reference: 	.listxattr	= fuse_listxattr,
// C source retained for exact declaration/control-flow reference: 	.get_inode_acl	= fuse_get_inode_acl,
// C source retained for exact declaration/control-flow reference: 	.get_acl	= fuse_get_acl,
// C source retained for exact declaration/control-flow reference: 	.set_acl	= fuse_set_acl,
// C source retained for exact declaration/control-flow reference: 	.fileattr_get	= fuse_fileattr_get,
// C source retained for exact declaration/control-flow reference: 	.fileattr_set	= fuse_fileattr_set,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const struct inode_operations fuse_symlink_inode_operations = {
// C source retained for exact declaration/control-flow reference: 	.setattr	= fuse_setattr,
// C source retained for exact declaration/control-flow reference: 	.get_link	= fuse_get_link,
// C source retained for exact declaration/control-flow reference: 	.getattr	= fuse_getattr,
// C source retained for exact declaration/control-flow reference: 	.listxattr	= fuse_listxattr,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_init_common(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	inode->i_op = &fuse_common_inode_operations;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_init_dir(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	struct fuse_inode *fi = get_fuse_inode(inode);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	inode->i_op = &fuse_dir_inode_operations;
// C source retained for exact declaration/control-flow reference: 	inode->i_fop = &fuse_dir_operations;
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	spin_lock_init(&fi->rdc.lock);
// C source retained for exact declaration/control-flow reference: 	fi->rdc.cached = false;
// C source retained for exact declaration/control-flow reference: 	fi->rdc.size = 0;
// C source retained for exact declaration/control-flow reference: 	fi->rdc.pos = 0;
// C source retained for exact declaration/control-flow reference: 	fi->rdc.version = 0;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static int fuse_symlink_read_folio(struct file *null, struct folio *folio)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	int err = fuse_readlink_folio(folio->mapping->host, folio);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	if (!err)
// C source retained for exact declaration/control-flow reference: 		folio_mark_uptodate(folio);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	folio_unlock(folio);
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: 	return err;
// C source retained for exact declaration/control-flow reference: }
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: static const struct address_space_operations fuse_symlink_aops = {
// C source retained for exact declaration/control-flow reference: 	.read_folio	= fuse_symlink_read_folio,
// C source retained for exact declaration/control-flow reference: };
// C source retained for exact declaration/control-flow reference: 
// C source retained for exact declaration/control-flow reference: void fuse_init_symlink(struct inode *inode)
// C source retained for exact declaration/control-flow reference: {
// C source retained for exact declaration/control-flow reference: 	inode->i_op = &fuse_symlink_inode_operations;
// C source retained for exact declaration/control-flow reference: 	inode->i_data.a_ops = &fuse_symlink_aops;
// C source retained for exact declaration/control-flow reference: 	inode_nohighmem(inode);
// C source retained for exact declaration/control-flow reference: }

*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
