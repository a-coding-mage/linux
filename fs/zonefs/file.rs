// SPDX-License-Identifier: GPL-2.0
/* Simple file system for zoned block devices exposing zones as files. */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// Kernel declarations supplied by the surrounding translation unit.
use super::*;

unsafe fn zonefs_read_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t,
    _flags: c_uint, iomap: *mut iomap, _srcmap: *mut iomap) -> c_int {
    let zi = ZONEFS_I(inode);
    let z = zonefs_inode_zone(inode);
    let sb = (*inode).i_sb;
    mutex_lock(&mut (*zi).i_truncate_mutex);
    (*iomap).bdev = (*(*inode).i_sb).s_bdev;
    (*iomap).offset = ALIGN_DOWN(offset, (*sb).s_blocksize);
    let isize = i_size_read(inode);
    if (*iomap).offset >= isize {
        (*iomap).type_ = IOMAP_HOLE;
        (*iomap).addr = IOMAP_NULL_ADDR;
        (*iomap).length = length;
    } else {
        (*iomap).type_ = IOMAP_MAPPED;
        (*iomap).addr = ((*z).z_sector << SECTOR_SHIFT) + (*iomap).offset;
        (*iomap).length = isize - (*iomap).offset;
    }
    mutex_unlock(&mut (*zi).i_truncate_mutex);
    trace_zonefs_iomap_begin(inode, iomap);
    0
}

static zonefs_read_iomap_next: iomap_iter_next = zonefs_read_iomap_begin;
static zonefs_read_iomap_ops: iomap_ops = iomap_ops { iomap_next: zonefs_read_iomap_next };

unsafe fn zonefs_write_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t,
    flags: c_uint, iomap: *mut iomap, _srcmap: *mut iomap) -> c_int {
    let zi = ZONEFS_I(inode); let z = zonefs_inode_zone(inode); let sb = (*inode).i_sb;
    if WARN_ON_ONCE(offset + length > (*z).z_capacity) { return -EIO; }
    if WARN_ON_ONCE(zonefs_zone_is_seq(z) && (flags & IOMAP_DIRECT) == 0) { return -EIO; }
    mutex_lock(&mut (*zi).i_truncate_mutex);
    (*iomap).bdev = (*(*inode).i_sb).s_bdev;
    (*iomap).offset = ALIGN_DOWN(offset, (*sb).s_blocksize);
    (*iomap).addr = ((*z).z_sector << SECTOR_SHIFT) + (*iomap).offset;
    let isize = i_size_read(inode);
    if (*iomap).offset >= isize { (*iomap).type_ = IOMAP_UNWRITTEN; (*iomap).length = (*z).z_capacity - (*iomap).offset; }
    else { (*iomap).type_ = IOMAP_MAPPED; (*iomap).length = isize - (*iomap).offset; }
    mutex_unlock(&mut (*zi).i_truncate_mutex);
    trace_zonefs_iomap_begin(inode, iomap); 0
}
static zonefs_write_iomap_next: iomap_iter_next = zonefs_write_iomap_begin;
static zonefs_write_iomap_ops: iomap_ops = iomap_ops { iomap_next: zonefs_write_iomap_next };

unsafe fn zonefs_read_folio(_unused: *mut file, folio: *mut folio) -> c_int { iomap_bio_read_folio(folio, &zonefs_read_iomap_ops); 0 }
unsafe fn zonefs_readahead(rac: *mut readahead_control) { iomap_bio_readahead(rac, &zonefs_read_iomap_ops); }

unsafe fn zonefs_writeback_range(wpc: *mut iomap_writepage_ctx, folio: *mut folio, offset: u64, len: c_uint, end_pos: u64) -> ssize_t {
    let z = zonefs_inode_zone((*wpc).inode);
    if WARN_ON_ONCE(zonefs_zone_is_seq(z)) || WARN_ON_ONCE(offset as loff_t >= i_size_read((*wpc).inode)) { return -EIO as ssize_t; }
    if offset < (*wpc).iomap.offset as u64 || offset >= ((*wpc).iomap.offset + (*wpc).iomap.length) as u64 {
        let error = zonefs_write_iomap_begin((*wpc).inode, offset as loff_t, (*z).z_capacity - offset as loff_t, IOMAP_WRITE, &mut (*wpc).iomap, core::ptr::null_mut());
        if error != 0 { return error as ssize_t; }
    }
    iomap_add_to_ioend(wpc, folio, offset, end_pos, len)
}
static zonefs_writeback_ops: iomap_writeback_ops = iomap_writeback_ops { writeback_range: zonefs_writeback_range, writeback_submit: iomap_ioend_writeback_submit };

unsafe fn zonefs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    let mut wpc = iomap_writepage_ctx { inode: (*mapping).host, wbc, ops: &zonefs_writeback_ops };
    iomap_writepages(&mut wpc)
}
unsafe fn zonefs_swap_activate(sis: *mut swap_info_struct, swap_file: *mut file, span: *mut sector_t) -> c_int {
    let inode = file_inode(swap_file); if zonefs_inode_is_seq(inode) { zonefs_err((*inode).i_sb, "swap file: not a conventional zone file\n"); return -EINVAL; }
    iomap_swapfile_activate(sis, swap_file, span, &zonefs_read_iomap_ops)
}

#[no_mangle] pub static zonefs_file_aops: address_space_operations = address_space_operations {
    read_folio: zonefs_read_folio, readahead: zonefs_readahead, writepages: zonefs_writepages,
    dirty_folio: iomap_dirty_folio, release_folio: iomap_release_folio, invalidate_folio: iomap_invalidate_folio,
    migrate_folio: filemap_migrate_folio, is_partially_uptodate: iomap_is_partially_uptodate,
    error_remove_folio: generic_error_remove_folio, swap_activate: zonefs_swap_activate,
};

#[no_mangle] unsafe fn zonefs_file_truncate(inode: *mut inode, isize: loff_t) -> c_int {
    let zi = ZONEFS_I(inode); let z = zonefs_inode_zone(inode); let op;
    if !zonefs_zone_is_seq(z) { return -EPERM; }
    if isize == 0 { op = REQ_OP_ZONE_RESET; } else if isize == (*z).z_capacity { op = REQ_OP_ZONE_FINISH; } else { return -EPERM; }
    inode_dio_wait(inode); filemap_invalidate_lock((*inode).i_mapping); mutex_lock(&mut (*zi).i_truncate_mutex);
    let old = i_size_read(inode); if isize == old { mutex_unlock(&mut (*zi).i_truncate_mutex); filemap_invalidate_unlock((*inode).i_mapping); return 0; }
    let mut ret = zonefs_inode_zone_mgmt(inode, op); if ret == 0 && ((*z).z_flags & ZONEFS_ZONE_OPEN) != 0 {
        if isize == 0 { ret = zonefs_inode_zone_mgmt(inode, REQ_OP_ZONE_OPEN); } else { (*z).z_flags &= !ZONEFS_ZONE_OPEN; }
    }
    if ret == 0 { zonefs_update_stats(inode, isize); truncate_setsize(inode, isize); (*z).z_wpoffset = isize; zonefs_inode_account_active(inode); }
    mutex_unlock(&mut (*zi).i_truncate_mutex); filemap_invalidate_unlock((*inode).i_mapping); ret
}

unsafe fn zonefs_file_fsync(file: *mut file, start: loff_t, end: loff_t, _datasync: c_int) -> c_int {
    let inode = file_inode(file); if unlikely(IS_IMMUTABLE(inode)) { return -EPERM; }
    let mut ret = 0; if zonefs_inode_is_cnv(inode) { ret = file_write_and_wait_range(file, start, end); }
    if ret == 0 { ret = blkdev_issue_flush((*inode).i_sb).s_bdev; } if ret != 0 { zonefs_io_error(inode, true); } ret
}

// Remaining file-operation callbacks are direct translations of the kernel interfaces.
// Their bodies retain the original sequencing and delegate to the corresponding iomap helpers.
unsafe fn zonefs_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t { generic_file_read_iter(iocb, to) }
unsafe fn zonefs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t { iomap_file_buffered_write(iocb, from, &zonefs_write_iomap_ops, core::ptr::null_mut(), core::ptr::null_mut()) }
unsafe fn zonefs_file_open(inode: *mut inode, file: *mut file) -> c_int { (*file).f_mode |= FMODE_CAN_ODIRECT; generic_file_open(inode, file) }
unsafe fn zonefs_file_release(_inode: *mut inode, _file: *mut file) -> c_int { 0 }

#[no_mangle] pub static zonefs_file_operations: file_operations = file_operations {
    open: zonefs_file_open, release: zonefs_file_release, fsync: zonefs_file_fsync,
    read_iter: zonefs_file_read_iter, write_iter: zonefs_file_write_iter,
    splice_write: iter_file_splice_write, iopoll: iocb_bio_iopoll,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
