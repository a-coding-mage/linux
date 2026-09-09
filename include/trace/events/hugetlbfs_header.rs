/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the Linux hugetlbfs trace-event header. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_char;

/* Supplied by the kernel tracepoint and filesystem definitions. */
pub enum inode {}
pub enum dentry {}
pub enum iattr {}

pub type u64_ = u64;
pub type __u16 = u16;
pub type loff_t = i64;
pub type blkcnt_t = u64;
pub type dev_t = u64;

#[repr(C)]
pub struct hugetlbfs_alloc_inode_entry {
    pub ino: u64_,
    pub dir: u64_,
    pub dev: dev_t,
    pub mode: __u16,
}

#[repr(C)]
pub struct hugetlbfs__inode_entry {
    pub ino: u64_,
    pub size: loff_t,
    pub blocks: blkcnt_t,
    pub dev: dev_t,
    pub nlink: u32,
    pub seals: u32,
    pub mode: __u16,
}

#[repr(C)]
pub struct hugetlbfs_setattr_entry {
    pub ino: u64_,
    pub old_size: loff_t,
    pub ia_size: loff_t,
    pub dev: dev_t,
    pub d_len: u32,
    /* __string(d_name, dentry->d_name.name) */
    pub d_name: *const c_char,
    pub ia_valid: u32,
    pub ia_mode: u32,
}

#[repr(C)]
pub struct hugetlbfs_fallocate_entry {
    pub ino: u64_,
    pub offset: loff_t,
    pub len: loff_t,
    pub size: loff_t,
    pub dev: dev_t,
    pub mode: i32,
    pub ret: i32,
}

/*
 * TRACE_EVENT(hugetlbfs_alloc_inode):
 * TP_PROTO(struct inode *inode, struct inode *dir, int mode)
 * TP_fast_assign:
 *   dev = inode->i_sb->s_dev; ino = inode->i_ino;
 *   dir = dir ? dir->i_ino : 0; entry->mode = mode;
 * TP_printk("dev %d,%d ino %llu dir %llu mode 0%o", MAJOR(dev), MINOR(dev),
 *           ino, dir, mode)
 */
pub const HUGETLBFS_ALLOC_INODE_EVENT: &str = "hugetlbfs_alloc_inode";

/*
 * DECLARE_EVENT_CLASS(hugetlbfs__inode):
 * TP_PROTO(struct inode *inode)
 * TP_fast_assign:
 *   dev = inode->i_sb->s_dev; ino = inode->i_ino; mode = inode->i_mode;
 *   size = inode->i_size; nlink = inode->i_nlink;
 *   seals = HUGETLBFS_I(inode)->seals; blocks = inode->i_blocks;
 * TP_printk("dev %d,%d ino %llu mode 0%o size %lld nlink %u seals %u blocks %llu", ...)
 */
pub const HUGETLBFS_INODE_EVENT_CLASS: &str = "hugetlbfs__inode";

/* DEFINE_EVENT(hugetlbfs__inode, hugetlbfs_evict_inode, struct inode *inode) */
pub const HUGETLBFS_EVICT_INODE_EVENT: &str = "hugetlbfs_evict_inode";

/* DEFINE_EVENT(hugetlbfs__inode, hugetlbfs_free_inode, struct inode *inode) */
pub const HUGETLBFS_FREE_INODE_EVENT: &str = "hugetlbfs_free_inode";

/*
 * TRACE_EVENT(hugetlbfs_setattr):
 * TP_PROTO(struct inode *inode, struct dentry *dentry, struct iattr *attr)
 * TP_fast_assign copies dev, ino, dentry->d_name.len, dentry->d_name.name,
 * attr->ia_valid, attr->ia_mode, inode->i_size, and attr->ia_size.
 * TP_printk("dev %d,%d ino %llu name %.*s valid %#x mode 0%o old_size %lld size %lld", ...)
 */
pub const HUGETLBFS_SETATTR_EVENT: &str = "hugetlbfs_setattr";

/*
 * TRACE_EVENT(hugetlbfs_fallocate):
 * TP_PROTO(struct inode *inode, int mode, loff_t offset, loff_t len, int ret)
 * TP_fast_assign copies dev, ino, mode, offset, len, inode->i_size, and ret.
 * TP_printk("dev %d,%d ino %llu mode 0%o offset %lld len %lld size %lld ret %d", ...)
 */
pub const HUGETLBFS_FALLOCATE_EVENT: &str = "hugetlbfs_fallocate";

/* TRACE_HEADER_MULTI_READ and trace/define_trace.h are build-time directives. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
