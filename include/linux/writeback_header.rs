/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of include/linux/writeback.h. */

// External kernel types, constants, and functions are supplied by other headers.

pub const DIRTY_SCOPE: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum writeback_sync_modes { WB_SYNC_NONE, WB_SYNC_ALL }

#[repr(C)]
pub struct writeback_control {
    pub nr_to_write: ::core::ffi::c_long,
    pub pages_skipped: ::core::ffi::c_long,
    pub range_start: i64,
    pub range_end: i64,
    pub sync_mode: writeback_sync_modes,
    pub for_kupdate: u32,
    pub for_background: u32,
    pub tagged_writepages: u32,
    pub range_cyclic: u32,
    pub for_sync: u32,
    pub unpinned_netfs_wb: u32,
    pub no_cgroup_owner: u32,
    pub fbatch: folio_batch,
    pub index: usize,
    pub saved_err: i32,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb: *mut bdi_writeback,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub inode: *mut inode,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_id: i32,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_lcand_id: i32,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_tcand_id: i32,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_bytes: usize,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_lcand_bytes: usize,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub wb_tcand_bytes: usize,
}

#[inline]
pub unsafe fn wbc_to_write_flags(wbc: *mut writeback_control) -> blk_opf_t {
    let mut flags: blk_opf_t = 0;
    if (*wbc).sync_mode == writeback_sync_modes::WB_SYNC_ALL {
        flags |= REQ_SYNC;
    } else if (*wbc).for_kupdate != 0 || (*wbc).for_background != 0 {
        flags |= REQ_BACKGROUND;
    }
    flags
}

#[repr(C)]
pub struct wb_domain {
    pub lock: spinlock_t,
    pub completions: fprop_global,
    pub period_timer: timer_list,
    pub period_time: usize,
    pub dirty_limit_tstamp: usize,
    pub dirty_limit: usize,
}

#[inline]
pub unsafe fn wb_domain_size_changed(dom: *mut wb_domain) {
    spin_lock(&mut (*dom).lock);
    (*dom).dirty_limit_tstamp = jiffies;
    (*dom).dirty_limit = 0;
    spin_unlock(&mut (*dom).lock);
}

#[inline]
pub unsafe fn wbc_to_tag(wbc: *mut writeback_control) -> xa_mark_t {
    if (*wbc).sync_mode == writeback_sync_modes::WB_SYNC_ALL || (*wbc).tagged_writepages != 0 {
        PAGECACHE_TAG_TOWRITE
    } else { PAGECACHE_TAG_DIRTY }
}

pub unsafe extern "C" {
    pub fn writeback_inodes_sb(sb: *mut super_block, reason: wb_reason);
    pub fn writeback_inodes_sb_nr(sb: *mut super_block, nr: usize, reason: wb_reason);
    pub fn try_to_writeback_inodes_sb(sb: *mut super_block, reason: wb_reason);
    pub fn sync_inodes_sb(sb: *mut super_block);
    pub fn wakeup_flusher_threads(reason: wb_reason);
    pub fn wakeup_flusher_threads_bdi(bdi: *mut backing_dev_info, reason: wb_reason);
    pub fn inode_wait_for_writeback(inode: *mut inode);
    pub fn inode_io_list_del(inode: *mut inode);
    pub fn node_dirty_ok(pgdat: *mut pglist_data) -> bool;
    pub fn wb_domain_init(dom: *mut wb_domain, gfp: gfp_t) -> i32;
    pub fn global_dirty_limits(pbackground: *mut usize, pdirty: *mut usize);
    pub fn wb_calc_thresh(wb: *mut bdi_writeback, thresh: usize) -> usize;
    pub fn cgwb_calc_thresh(wb: *mut bdi_writeback) -> usize;
    pub fn wb_update_bandwidth(wb: *mut bdi_writeback);
    pub fn balance_dirty_pages_ratelimited(mapping: *mut address_space);
    pub fn balance_dirty_pages_ratelimited_flags(mapping: *mut address_space, flags: u32) -> i32;
    pub fn wb_over_bg_thresh(wb: *mut bdi_writeback) -> bool;
    pub fn writeback_iter(mapping: *mut address_space, wbc: *mut writeback_control, folio: *mut folio, error: *mut i32) -> *mut folio;
    pub fn do_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32;
    pub fn writeback_set_ratelimit();
    pub fn tag_pages_for_writeback(mapping: *mut address_space, start: usize, end: usize);
    pub fn filemap_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
    pub fn folio_redirty_for_writepage(wbc: *mut writeback_control, folio: *mut folio) -> bool;
    pub fn redirty_page_for_writepage(wbc: *mut writeback_control, page: *mut page) -> bool;
    pub fn sb_mark_inode_writeback(inode: *mut inode);
    pub fn sb_clear_inode_writeback(inode: *mut inode);
}

pub const BDP_ASYNC: u32 = 0x0001;
pub const MIN_WRITEBACK_PAGES: usize = 4096usize >> (PAGE_SHIFT - 10);

pub static mut dirty_writeback_interval: u32 = 0;
pub static mut dirty_expire_interval: u32 = 0;
pub static mut global_wb_domain: wb_domain = unsafe { ::core::mem::zeroed() };

pub const dirty_throttle_leaks: *mut i32 = core::ptr::null_mut(); // per-CPU declaration

#[repr(C)]
pub struct dirty_throttle_control {
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub dom: *mut wb_domain,
    #[cfg(CONFIG_CGROUP_WRITEBACK)]
    pub gdtc: *mut dirty_throttle_control,
    pub wb: *mut bdi_writeback,
    pub wb_completions: *mut fprop_local_percpu,
    pub avail: usize,
    pub dirty: usize,
    pub thresh: usize,
    pub bg_thresh: usize,
    pub limit: usize,
    pub wb_dirty: usize,
    pub wb_thresh: usize,
    pub wb_bg_thresh: usize,
    pub pos_ratio: usize,
    pub freerun: bool,
    pub dirty_exceeded: bool,
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
pub unsafe extern "C" {
    pub fn __inode_attach_wb(inode: *mut inode, folio: *mut folio);
    pub fn wbc_detach_inode(wbc: *mut writeback_control);
    pub fn wbc_account_cgroup_owner(wbc: *mut writeback_control, folio: *mut folio, bytes: usize);
    pub fn cgroup_writeback_by_id(bdi_id: u64, memcg_id: i32, reason: wb_reason, done: *mut wb_completion) -> i32;
    pub fn cgroup_writeback_umount(sb: *mut super_block);
    pub fn cleanup_offline_cgwb(wb: *mut bdi_writeback) -> bool;
    pub fn wbc_attach_fdatawrite_inode(wbc: *mut writeback_control, inode: *mut inode);
    pub fn inode_switch_wbs_work_fn(work: *mut work_struct);
}

#[cfg(CONFIG_CGROUP_WRITEBACK)]
#[inline]
pub unsafe fn inode_attach_wb(inode: *mut inode, folio: *mut folio) {
    if (*inode).i_wb.is_null() { __inode_attach_wb(inode, folio); }
}
#[cfg(CONFIG_CGROUP_WRITEBACK)]
#[inline] pub unsafe fn inode_detach_wb(_: *mut inode) {}
#[cfg(CONFIG_CGROUP_WRITEBACK)]
#[inline] pub unsafe fn wbc_init_bio(_: *mut writeback_control, _: *mut bio) {}

#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn inode_attach_wb(_: *mut inode, _: *mut folio) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn inode_detach_wb(_: *mut inode) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn wbc_attach_fdatawrite_inode(_: *mut writeback_control, _: *mut inode) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn wbc_detach_inode(_: *mut writeback_control) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn wbc_init_bio(_: *mut writeback_control, _: *mut bio) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn wbc_account_cgroup_owner(_: *mut writeback_control, _: *mut folio, _: usize) {}
#[cfg(not(CONFIG_CGROUP_WRITEBACK))]
#[inline] pub unsafe fn cgroup_writeback_umount(_: *mut super_block) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
