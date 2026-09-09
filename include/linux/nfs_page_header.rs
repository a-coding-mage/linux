/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/nfs_page.h
 *
 * NFS page cache wrapper.
 *
 * C header dependencies are supplied by other translated units.
 */

use core::ffi::c_void;

pub const PG_BUSY: u32 = 0;
pub const PG_MAPPED: u32 = 1;
pub const PG_FOLIO: u32 = 2;
pub const PG_CLEAN: u32 = 3;
pub const PG_COMMIT_TO_DS: u32 = 4;
pub const PG_INODE_REF: u32 = 5;
pub const PG_HEADLOCK: u32 = 6;
pub const PG_TEARDOWN: u32 = 7;
pub const PG_UNLOCKPAGE: u32 = 8;
pub const PG_UPTODATE: u32 = 9;
pub const PG_READ_FAILED: u32 = 10;
pub const PG_WB_END: u32 = 11;
pub const PG_REMOVE: u32 = 12;
pub const PG_CONTENDED1: u32 = 13;
pub const PG_CONTENDED2: u32 = 14;

pub const NFS_PAGEIO_DESCRIPTOR_MIRROR_MAX: u32 = 16;

#[repr(C)] pub struct nfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct page { pub mapping: *mut address_space }
#[repr(C)] pub struct folio { pub mapping: *mut address_space }
#[repr(C)] pub struct address_space { pub host: *mut inode }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct nfs_lock_context { pub open_context: *mut nfs_open_context }
#[repr(C)] pub struct nfs_open_context { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct nfs_write_verifier { _private: [u8; 0] }
#[repr(C)] pub struct nfs_pgio_header { _private: [u8; 0] }
#[repr(C)] pub struct rpc_task { _private: [u8; 0] }
#[repr(C)] pub struct rpc_message { _private: [u8; 0] }
#[repr(C)] pub struct nfs_rpc_ops { _private: [u8; 0] }
#[repr(C)] pub struct rpc_task_setup { _private: [u8; 0] }
#[repr(C)] pub struct rpc_call_ops { _private: [u8; 0] }
#[repr(C)] pub struct nfs_pgio_completion_ops { _private: [u8; 0] }
#[repr(C)] pub struct pnfs_layout_segment { _private: [u8; 0] }
#[repr(C)] pub struct nfs_io_completion { _private: [u8; 0] }
#[repr(C)] pub struct nfs_direct_req { _private: [u8; 0] }
#[repr(C)] pub struct nfs_commit_info { _private: [u8; 0] }

pub type pgoff_t = usize;
pub type loff_t = i64;
pub type u32_t = u32;

#[repr(C)]
pub union nfs_page_wb_page {
    pub wb_page: *mut page,
    pub wb_folio: *mut folio,
}

#[repr(C)]
pub struct nfs_page {
    pub wb_list: list_head,
    pub wb_page_or_folio: nfs_page_wb_page,
    pub wb_lock_context: *mut nfs_lock_context,
    pub wb_index: pgoff_t,
    pub wb_offset: u32,
    pub wb_pgbase: u32,
    pub wb_bytes: u32,
    pub wb_kref: kref,
    pub wb_flags: usize,
    pub wb_verf: nfs_write_verifier,
    pub wb_this_page: *mut nfs_page,
    pub wb_head: *mut nfs_page,
    pub wb_nio: u16,
}

#[repr(C)]
pub struct nfs_pgio_mirror {
    pub pg_list: list_head,
    pub pg_bytes_written: usize,
    pub pg_count: usize,
    pub pg_bsize: usize,
    pub pg_base: u32,
    pub pg_recoalesce: u8,
}

pub type nfs_pageio_init_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor, *mut nfs_page);
pub type nfs_pageio_test_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor, *mut nfs_page, *mut nfs_page) -> usize;
pub type nfs_pageio_doio_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor) -> i32;
pub type nfs_pageio_mirror_count_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor, *mut nfs_page) -> u32;
pub type nfs_pageio_cleanup_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor);
pub type nfs_pageio_get_mirror_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor, u32) -> *mut nfs_pgio_mirror;
pub type nfs_pageio_set_mirror_fn = unsafe extern "C" fn(*mut nfs_pageio_descriptor, u32) -> u32;

#[repr(C)]
pub struct nfs_pageio_ops {
    pub pg_init: Option<nfs_pageio_init_fn>,
    pub pg_test: Option<nfs_pageio_test_fn>,
    pub pg_doio: Option<nfs_pageio_doio_fn>,
    pub pg_get_mirror_count: Option<nfs_pageio_mirror_count_fn>,
    pub pg_cleanup: Option<nfs_pageio_cleanup_fn>,
    pub pg_get_mirror: Option<nfs_pageio_get_mirror_fn>,
    pub pg_set_mirror: Option<nfs_pageio_set_mirror_fn>,
}

#[repr(C)]
pub struct nfs_rw_ops {
    pub rw_alloc_header: Option<unsafe extern "C" fn() -> *mut nfs_pgio_header>,
    pub rw_free_header: Option<unsafe extern "C" fn(*mut nfs_pgio_header)>,
    pub rw_done: Option<unsafe extern "C" fn(*mut rpc_task, *mut nfs_pgio_header, *mut inode) -> i32>,
    pub rw_result: Option<unsafe extern "C" fn(*mut rpc_task, *mut nfs_pgio_header)>,
    pub rw_initiate: Option<unsafe extern "C" fn(*mut nfs_pgio_header, *mut rpc_message, *const nfs_rpc_ops, *mut rpc_task_setup, i32)>,
}

#[repr(C)]
pub struct nfs_pageio_descriptor {
    pub pg_inode: *mut inode,
    pub pg_ops: *const nfs_pageio_ops,
    pub pg_rw_ops: *const nfs_rw_ops,
    pub pg_ioflags: i32,
    pub pg_error: i32,
    pub pg_rpc_callops: *const rpc_call_ops,
    pub pg_completion_ops: *const nfs_pgio_completion_ops,
    pub pg_lseg: *mut pnfs_layout_segment,
    pub pg_io_completion: *mut nfs_io_completion,
    pub pg_dreq: *mut nfs_direct_req,
    /* CONFIG_NFS_FSCACHE conditionally supplies this field. */
    pub pg_netfs: *mut c_void,
    pub pg_bsize: u32,
    pub pg_mirror_count: u32,
    pub pg_mirrors: *mut nfs_pgio_mirror,
    pub pg_mirrors_static: [nfs_pgio_mirror; 1],
    pub pg_mirrors_dynamic: *mut nfs_pgio_mirror,
    pub pg_mirror_idx: u32,
    pub pg_maxretrans: u16,
    pub pg_moreio: u8,
}

extern "C" {
    pub fn nfs_page_create_from_page(ctx: *mut nfs_open_context, page: *mut page, pgbase: u32, offset: loff_t, count: u32) -> *mut nfs_page;
    pub fn nfs_page_create_from_folio(ctx: *mut nfs_open_context, folio: *mut folio, offset: u32, count: u32) -> *mut nfs_page;
    pub fn nfs_release_request(req: *mut nfs_page);
    pub fn nfs_pageio_init(desc: *mut nfs_pageio_descriptor, inode: *mut inode, pg_ops: *const nfs_pageio_ops, compl_ops: *const nfs_pgio_completion_ops, rw_ops: *const nfs_rw_ops, bsize: usize, how: i32);
    pub fn nfs_pageio_add_request(desc: *mut nfs_pageio_descriptor, req: *mut nfs_page) -> i32;
    pub fn nfs_pageio_resend(desc: *mut nfs_pageio_descriptor, hdr: *mut nfs_pgio_header) -> i32;
    pub fn nfs_pageio_complete(desc: *mut nfs_pageio_descriptor);
    pub fn nfs_pageio_cond_complete(desc: *mut nfs_pageio_descriptor, index: pgoff_t);
    pub fn nfs_generic_pg_test(desc: *mut nfs_pageio_descriptor, prev: *mut nfs_page, req: *mut nfs_page) -> usize;
    pub fn nfs_unlock_request(req: *mut nfs_page);
    pub fn nfs_unlock_and_release_request(req: *mut nfs_page);
    pub fn nfs_join_page_group(head: *mut nfs_page, cinfo: *mut nfs_commit_info, inode: *mut inode);
    pub fn nfs_page_group_lock(req: *mut nfs_page) -> i32;
    pub fn nfs_page_group_unlock(req: *mut nfs_page);
    pub fn nfs_page_group_sync_on_bit(req: *mut nfs_page, bit: u32) -> bool;
    pub fn nfs_page_group_sync_on_bit_locked(req: *mut nfs_page, bit: u32) -> bool;
    pub fn nfs_page_set_headlock(req: *mut nfs_page) -> i32;
    pub fn nfs_page_clear_headlock(req: *mut nfs_page);
    pub fn nfs_async_iocounter_wait(task: *mut rpc_task, lock_context: *mut nfs_lock_context) -> bool;
}

extern "C" {
    fn test_bit(bit: u32, addr: *const usize) -> bool;
    fn test_and_set_bit(bit: u32, addr: *mut usize) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_move_tail(entry: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del_init(entry: *mut list_head);
    fn folio_page(folio: *mut folio, index: usize) -> *mut page;
    fn folio_size(folio: *mut folio) -> usize;
}

pub const PAGE_SHIFT: usize = 0; /* supplied by the pagemap dependency */
pub const PAGE_SIZE: usize = 0; /* supplied by the pagemap dependency */

pub unsafe fn nfs_page_to_folio(req: *const nfs_page) -> *mut folio {
    if test_bit(PG_FOLIO, &(*req).wb_flags) { (*req).wb_page_or_folio.wb_folio } else { core::ptr::null_mut() }
}

pub unsafe fn nfs_page_to_page(req: *const nfs_page, pgbase: usize) -> *mut page {
    let folio = nfs_page_to_folio(req);
    if folio.is_null() { (*req).wb_page_or_folio.wb_page } else { folio_page(folio, pgbase >> PAGE_SHIFT) }
}

pub unsafe fn nfs_page_to_inode(req: *const nfs_page) -> *mut inode {
    let folio = nfs_page_to_folio(req);
    if folio.is_null() { (*(*req).wb_page_or_folio.wb_page).mapping.as_ref().unwrap().host } else { (*folio).mapping.as_ref().unwrap().host }
}

pub unsafe fn nfs_page_max_length(req: *const nfs_page) -> usize {
    let folio = nfs_page_to_folio(req);
    if folio.is_null() { PAGE_SIZE } else { folio_size(folio) }
}

pub unsafe fn nfs_lock_request(req: *mut nfs_page) -> i32 {
    (!test_and_set_bit(PG_BUSY, &mut (*req).wb_flags)) as i32
}

pub unsafe fn nfs_list_add_request(req: *mut nfs_page, head: *mut list_head) { list_add_tail(&mut (*req).wb_list, head); }
pub unsafe fn nfs_list_move_request(req: *mut nfs_page, head: *mut list_head) { list_move_tail(&mut (*req).wb_list, head); }
pub unsafe fn nfs_list_remove_request(req: *mut nfs_page) { if !list_empty(&(*req).wb_list) { list_del_init(&mut (*req).wb_list); } }
pub unsafe fn nfs_list_entry(head: *mut list_head) -> *mut nfs_page { head as *mut nfs_page }
pub unsafe fn req_offset(req: *const nfs_page) -> loff_t { (((*req).wb_index as loff_t) << PAGE_SHIFT) + (*req).wb_offset as loff_t }
pub unsafe fn nfs_req_openctx(req: *mut nfs_page) -> *mut nfs_open_context { (*(*req).wb_lock_context).open_context }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
