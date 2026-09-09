// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * iomap callack functions
 *
 * Copyright (C) 2026 Namjae Jeon <linkinjeon@kernel.org>
 */

// Linux and exFAT declarations are supplied by the surrounding translation unit.

unsafe fn exfat_file_write_dio_end_io(iocb: *mut kiocb, size: ssize_t,
                                      error: i32, _flags: u32) -> i32 {
    let inode = file_inode((*iocb).ki_filp);
    if error != 0 { return error; }
    if size != 0 && i_size_read(inode) < (*iocb).ki_pos + size {
        i_size_write(inode, (*iocb).ki_pos + size);
        mark_inode_dirty(inode);
    }
    0
}

pub static exfat_write_dio_ops: iomap_dio_ops = iomap_dio_ops {
    end_io: Some(exfat_file_write_dio_end_io),
};

unsafe fn __exfat_iomap_begin(inode: *mut inode, offset: loff_t, mut length: loff_t,
                              flags: u32, iomap: *mut iomap, may_alloc: bool) -> i32 {
    let sb = (*inode).i_sb;
    let sbi = EXFAT_SB(sb);
    let ei = EXFAT_I(inode);
    let mut cluster: u32 = 0;
    let mut num_clusters: u32;
    let mut cluster_offset: loff_t;
    let mut cluster_length: loff_t;
    let err: i32;
    let mut balloc = false;

    if !may_alloc {
        if i_size_read(inode) <= offset {
            (*iomap).type_ = IOMAP_HOLE;
            (*iomap).addr = IOMAP_NULL_ADDR;
            (*iomap).offset = offset;
            (*iomap).length = length;
            return 0;
        }
        if offset + length > i_size_read(inode) {
            length = round_up(i_size_read(inode), i_blocksize(inode)) - offset;
        }
    }

    num_clusters = exfat_bytes_to_cluster_round_up(sbi, offset + length)
        - exfat_bytes_to_cluster(sbi, offset);
    mutex_lock(&mut (*sbi).s_lock);
    (*iomap).bdev = (*inode).i_sb.s_bdev;
    (*iomap).offset = offset;
    err = exfat_map_cluster(inode, exfat_bytes_to_cluster(sbi, offset),
                            &mut cluster, &mut num_clusters, may_alloc, &mut balloc);
    if err != 0 { mutex_unlock(&mut (*sbi).s_lock); return err; }

    cluster_offset = exfat_cluster_offset(sbi, offset);
    cluster_length = exfat_cluster_to_bytes(sbi, num_clusters);
    (*iomap).length = min_t(length, cluster_length - cluster_offset);
    (*iomap).addr = exfat_cluster_to_phys_bytes(sbi, cluster) + cluster_offset;
    (*iomap).type_ = IOMAP_MAPPED;
    (*iomap).flags = IOMAP_F_MERGED;

    if may_alloc || (flags & IOMAP_ZERO) != 0 {
        if balloc { (*iomap).flags |= IOMAP_F_NEW; }
        else if (*iomap).offset + (*iomap).length >= (*ei).valid_size {
            /* Zero the page-cache tail between valid_size and this write. */
            (*iomap).flags |= IOMAP_F_ZERO_TAIL;
        }
    } else if offset >= (*ei).valid_size {
        (*iomap).type_ = if (flags & IOMAP_REPORT) != 0 { IOMAP_HOLE } else { IOMAP_UNWRITTEN };
    } else if offset + (*iomap).length > (*ei).valid_size {
        if (flags & IOMAP_REPORT) != 0 {
            (*iomap).length = (*ei).valid_size - (*iomap).offset;
        } else {
            (*iomap).length = round_up((*ei).valid_size, i_blocksize(inode)) - (*iomap).offset;
        }
    }
    (*iomap).flags |= IOMAP_F_MERGED;
    mutex_unlock(&mut (*sbi).s_lock);
    0
}

unsafe fn exfat_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t,
                            flags: u32, iomap: *mut iomap, _srcmap: *mut iomap) -> i32 {
    __exfat_iomap_begin(inode, offset, length, flags, iomap, false)
}

unsafe fn exfat_write_iomap_begin(inode: *mut inode, offset: loff_t, length: loff_t,
                                 flags: u32, iomap: *mut iomap, _srcmap: *mut iomap) -> i32 {
    __exfat_iomap_begin(inode, offset, length, flags, iomap, true)
}

// DEFINE_IOMAP_ITER_NEXT(exfat_iomap_next, exfat_iomap_begin)
unsafe fn exfat_iomap_next(iter: *mut iomap_iter) -> i32 { exfat_iomap_begin((*iter).inode, (*iter).pos, (*iter).length, (*iter).flags, &mut (*iter).iomap, &mut (*iter).srcmap) }

pub static exfat_iomap_ops: iomap_ops = iomap_ops { iomap_next: Some(exfat_iomap_next) };

unsafe fn exfat_write_iomap_end(inode: *mut inode, pos: loff_t, _length: loff_t,
                                written: ssize_t, _flags: u32, iomap: *mut iomap) -> ssize_t {
    let ei = EXFAT_I(inode);
    let mut dirtied = false;
    let mut end: loff_t;
    if written == 0 { return 0; }
    end = pos + written;
    if (*ei).valid_size < end { (*ei).valid_size = end; dirtied = true; }
    if ((*iomap).flags & IOMAP_F_ZERO_TAIL) != 0 { end = round_up(end, i_blocksize(inode)); }
    if (*ei).zeroed_size < end { (*ei).zeroed_size = end; }
    if dirtied || ((*iomap).flags & IOMAP_F_SIZE_CHANGED) != 0 { mark_inode_dirty(inode); }
    written
}

// DEFINE_IOMAP_ITER_NEXT_END(exfat_write_iomap_next, exfat_write_iomap_begin, exfat_write_iomap_end)
unsafe fn exfat_write_iomap_next(iter: *mut iomap_iter) -> i32 {
    exfat_write_iomap_begin((*iter).inode, (*iter).pos, (*iter).length, (*iter).flags, &mut (*iter).iomap, &mut (*iter).srcmap)
}
pub static exfat_write_iomap_ops: iomap_ops = iomap_ops { iomap_next: Some(exfat_write_iomap_next) };

unsafe fn exfat_writeback_range(wpc: *mut iomap_writepage_ctx, folio: *mut folio,
                                offset: u64, len: u32, end_pos: u64) -> ssize_t {
    if offset < (*wpc).iomap.offset || offset >= (*wpc).iomap.offset + (*wpc).iomap.length {
        let error = __exfat_iomap_begin((*wpc).inode, offset as loff_t, len as loff_t, 0, &mut (*wpc).iomap, false);
        if error != 0 { return error as ssize_t; }
    }
    iomap_add_to_ioend(wpc, folio, offset, end_pos, len)
}

pub static exfat_writeback_ops: iomap_writeback_ops = iomap_writeback_ops {
    writeback_range: Some(exfat_writeback_range),
    writeback_submit: Some(iomap_ioend_writeback_submit),
};

unsafe fn exfat_iomap_read_end_io(bio: *mut bio) {
    let error = blk_status_to_errno((*bio).bi_status);
    let mut iter: folio_iter;
    bio_for_each_folio_all!(iter, bio, {
        let folio = iter.folio;
        let ei = EXFAT_I((*(*folio).mapping).host);
        let valid_size = (*ei).valid_size;
        let pos = folio_pos(folio);
        if pos + iter.offset as loff_t < valid_size && pos + iter.offset as loff_t + iter.length as loff_t > valid_size {
            folio_zero_segment(folio, offset_in_folio(folio, valid_size), iter.offset + iter.length);
        }
        iomap_finish_folio_read(folio, iter.offset, iter.length, error);
    });
    bio_put(bio);
}

unsafe fn exfat_iomap_bio_submit_read(iter: *const iomap_iter, ctx: *mut iomap_read_folio_ctx) {
    iomap_bio_submit_read_endio(iter, ctx, Some(exfat_iomap_read_end_io));
}

pub static exfat_iomap_bio_read_ops: iomap_read_ops = iomap_read_ops {
    read_folio_range: Some(iomap_bio_read_folio_range),
    submit_read: Some(exfat_iomap_bio_submit_read),
};

pub unsafe fn exfat_iomap_swap_activate(sis: *mut swap_info_struct, file: *mut file,
                                        span: *mut sector_t) -> i32 {
    iomap_swapfile_activate(sis, file, span, &exfat_iomap_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
