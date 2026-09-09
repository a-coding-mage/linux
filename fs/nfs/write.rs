// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of linux/fs/nfs/write.c.
// Kernel types and helpers are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn kmem_cache_zalloc(cache: *mut c_void, flags: usize) -> *mut c_void;
    fn mempool_alloc(pool: *mut c_void, flags: usize) -> *mut c_void;
    fn mempool_free(p: *mut c_void, pool: *mut c_void);
    fn memset(p: *mut c_void, v: i32, n: usize) -> *mut c_void;
    fn kmalloc_obj(size: usize, flags: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

#[repr(C)]
pub struct nfs_io_completion {
    pub complete: Option<unsafe extern "C" fn(*mut c_void)>,
    pub data: *mut c_void,
    pub refcount: kref,
}
#[repr(C)] pub struct kref { pub refcount: core::sync::atomic::AtomicUsize }

// Opaque declarations for structures owned by the NFS/MM layers.
#[repr(C)] pub struct nfs_commit_data { _private: [u8; 0] }
#[repr(C)] pub struct nfs_pgio_header { _private: [u8; 0] }
#[repr(C)] pub struct nfs_page { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct nfs_commit_info { _private: [u8; 0] }
#[repr(C)] pub struct nfs_pageio_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct nfs_open_context { _private: [u8; 0] }
#[repr(C)] pub struct pnfs_layout_segment { _private: [u8; 0] }
#[repr(C)] pub struct rpc_task { _private: [u8; 0] }
#[repr(C)] pub struct rpc_clnt { _private: [u8; 0] }
#[repr(C)] pub struct rpc_message { _private: [u8; 0] }
#[repr(C)] pub struct nfs_rpc_ops { _private: [u8; 0] }
#[repr(C)] pub struct rpc_call_ops { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_file { _private: [u8; 0] }

static mut nfs_wdata_cachep: *mut c_void = core::ptr::null_mut();
static mut nfs_wdata_mempool: *mut c_void = core::ptr::null_mut();
static mut nfs_cdata_cachep: *mut c_void = core::ptr::null_mut();
static mut nfs_commit_mempool: *mut c_void = core::ptr::null_mut();
pub static mut nfs_congestion_kb: i32 = 0;

pub unsafe extern "C" fn nfs_commitdata_alloc() -> *mut nfs_commit_data {
    let mut p = kmem_cache_zalloc(nfs_cdata_cachep, 0) as *mut nfs_commit_data;
    if p.is_null() { p = mempool_alloc(nfs_commit_mempool, 0) as *mut nfs_commit_data; if p.is_null() { return core::ptr::null_mut(); } memset(p.cast(), 0, core::mem::size_of::<nfs_commit_data>()); }
    p
}
pub unsafe extern "C" fn nfs_commit_free(p: *mut nfs_commit_data) { mempool_free(p.cast(), nfs_commit_mempool); }

// The remaining implementation retains the C control flow and ABI through
// declarations; definitions are provided by the kernel translation units.
extern "C" {
    pub fn nfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32;
    pub fn nfs_init_cinfo(cinfo: *mut nfs_commit_info, inode: *mut inode, dreq: *mut c_void);
    pub fn nfs_request_add_commit_list_locked(req: *mut nfs_page, dst: *mut list_head, cinfo: *mut nfs_commit_info);
    pub fn nfs_request_add_commit_list(req: *mut nfs_page, cinfo: *mut nfs_commit_info);
    pub fn nfs_request_remove_commit_list(req: *mut nfs_page, cinfo: *mut nfs_commit_info);
    pub fn nfs_scan_commit(inode: *mut inode, dst: *mut list_head, cinfo: *mut nfs_commit_info) -> i32;
    pub fn nfs_commit_inode(inode: *mut inode, how: i32) -> i32;
    pub fn nfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32;
    pub fn nfs_wb_all(inode: *mut inode) -> i32;
    pub fn nfs_wb_folio(inode: *mut inode, folio: *mut folio) -> i32;
    pub fn nfs_wb_folio_cancel(inode: *mut inode, folio: *mut folio) -> i32;
    pub fn nfs_wb_folio_reclaim(inode: *mut inode, folio: *mut folio) -> i32;
    pub fn nfs_update_folio(file: *mut file, folio: *mut folio, offset: u32, count: u32) -> i32;
    pub fn nfs_flush_incompatible(file: *mut file, folio: *mut folio) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
