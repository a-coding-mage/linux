// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS kernel address space operations and page cache handling.
 *
 * Copyright (c) 2001-2014 Anton Altaparmakov and Tuxera Inc.
 * Copyright (c) 2002 Richard Russon
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

unsafe fn ntfs_iomap_read_end_io(bio: *mut bio) {
    let error: int = blk_status_to_errno((*bio).bi_status);
    let mut iter: folio_iter;

    bio_for_each_folio_all!(iter, bio, {
        let folio: *mut folio = iter.folio;
        let ni: *mut ntfs_inode = NTFS_I((*(*folio).mapping).host);
        let init_size: s64;
        let pos: loff_t = folio_pos(folio);

        init_size = (*ni).initialized_size;
        if pos + iter.offset < init_size &&
            pos + iter.offset + iter.length > init_size {
            folio_zero_segment(folio, offset_in_folio(folio, init_size),
                               iter.offset + iter.length);
        }

        iomap_finish_folio_read(folio, iter.offset, iter.length, error);
    });
    bio_put(bio);
}

unsafe fn ntfs_iomap_bio_submit_read(
    iter: *const iomap_iter,
    ctx: *mut iomap_read_folio_ctx,
) {
    iomap_bio_submit_read_endio(iter, ctx, ntfs_iomap_read_end_io);
}

static ntfs_iomap_bio_read_ops: iomap_read_ops = iomap_read_ops {
    read_folio_range: iomap_bio_read_folio_range,
    submit_read: ntfs_iomap_bio_submit_read,
};

unsafe fn ntfs_read_folio(_file: *mut file, folio: *mut folio) -> int {
    let ni: *mut ntfs_inode = NTFS_I((*(*folio).mapping).host);
    let mut ctx: iomap_read_folio_ctx = iomap_read_folio_ctx {
        cur_folio: folio,
        ops: &ntfs_iomap_bio_read_ops,
    };

    /* Only $DATA attributes can be encrypted and only unnamed $DATA
     * attributes can be compressed. */
    if (*ni).type_ != AT_INDEX_ALLOCATION {
        if NInoEncrypted(ni) {
            folio_unlock(folio);
            return -EOPNOTSUPP;
        }
        if NInoWofCompressed(ni) {
            // CONFIG_NTFS_FS_WOF_COMPRESSION is a build-time configuration.
            #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
            { return ntfs_read_wof_compressed_block(folio); }
            #[cfg(not(CONFIG_NTFS_FS_WOF_COMPRESSION))]
            {
                folio_unlock(folio);
                return -EOPNOTSUPP;
            }
        }
        if NInoNonResident(ni) && NInoCompressed(ni) {
            return ntfs_read_compressed_block(folio);
        }
    }

    iomap_read_folio(&ntfs_read_iomap_ops, &mut ctx, core::ptr::null_mut());
    0
}

unsafe fn ntfs_bmap(mapping: *mut address_space, mut block: sector_t) -> sector_t {
    let mut ofs: s64;
    let mut size: s64;
    let i_size: loff_t;
    let mut lcn: s64;
    let blocksize: c_ulong;
    let mut flags: c_ulong = 0;
    let ni: *mut ntfs_inode = NTFS_I((*mapping).host);
    let vol: *mut ntfs_volume = (*ni).vol;
    let mut delta: c_uint;
    let blocksize_bits: c_uchar;

    ntfs_debug!("Entering for mft_no 0x%llx, logical block 0x%llx.", (*ni).mft_no, block as c_ulonglong);
    if (*ni).type_ != AT_DATA || !NInoNonResident(ni) || NInoEncrypted(ni) ||
        NInoWofCompressed(ni) || NInoMstProtected(ni) {
        ntfs_error!((*(*vol).sb), "BMAP does not make sense for attributes, returning 0.");
        return 0;
    }
    blocksize = (*(*vol).sb).s_blocksize;
    blocksize_bits = (*(*vol).sb).s_blocksize_bits;
    ofs = (block as s64) << blocksize_bits;
    read_lock_irqsave(&(*ni).size_lock, &mut flags);
    size = (*ni).initialized_size;
    i_size = i_size_read(VFS_I(ni));
    read_unlock_irqrestore(&(*ni).size_lock, flags);
    if unlikely!(ofs >= size || (ofs + blocksize as s64 > size && size < i_size)) {
        goto_hole!();
    }
    down_read(&(*ni).runlist.lock);
    lcn = ntfs_attr_vcn_to_lcn_nolock(ni, ntfs_bytes_to_cluster(vol, ofs), false);
    up_read(&(*ni).runlist.lock);
    if unlikely!(lcn < LCN_HOLE) {
        match lcn as int {
            LCN_ENOENT => { goto_hole!(); }
            LCN_ENOMEM => { ntfs_error!((*(*vol).sb), "Not enough memory. Returning 0."); }
            _ => { ntfs_error!((*(*vol).sb), "Failed to complete mapping. Run chkdsk. Returning 0."); }
        }
        return 0;
    }
    if lcn < 0 { goto_hole!(); }
    delta = (ofs as c_ulong & (*vol).cluster_size_mask) as c_uint;
    block = (ntfs_cluster_to_bytes(vol, lcn) + delta as s64) as sector_t >> blocksize_bits;
    ntfs_debug!("Done (returning block 0x%llx).", lcn as c_ulonglong);
    return block;

    macro_rules! goto_hole { () => {{ ntfs_debug!("Done (returning hole)."); return 0; }} }
}

unsafe fn ntfs_readahead(rac: *mut readahead_control) {
    let mapping = (*rac).mapping;
    let inode = (*mapping).host;
    let ni = NTFS_I(inode);
    let mut ctx = iomap_read_folio_ctx { ops: &ntfs_iomap_bio_read_ops, rac };
    if !NInoNonResident(ni) || NInoCompressed(ni) || NInoWofCompressed(ni) { return; }
    iomap_readahead(&ntfs_read_iomap_ops, &mut ctx, core::ptr::null_mut());
}

unsafe fn ntfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> int {
    let inode = (*mapping).host;
    let ni = NTFS_I(inode);
    let mut wpc = iomap_writepage_ctx { inode: (*mapping).host, wbc, ops: &ntfs_writeback_ops };
    let mut need_iput = false;
    if NVolShutdown((*ni).vol) { return -EIO; }
    if !NInoNonResident(ni) { return 0; }
    if NInoEncrypted(ni) { ntfs_debug!("Encrypted I/O not supported"); return -EOPNOTSUPP; }
    if ((*ni).type_ == AT_DATA || (*ni).type_ == AT_INDEX_ALLOCATION) && igrab(inode) != core::ptr::null_mut() { need_iput = true; }
    let ret = iomap_writepages(&mut wpc);
    if need_iput { iput(inode); }
    ret
}

unsafe fn ntfs_swap_activate(sis: *mut swap_info_struct, swap_file: *mut file, span: *mut sector_t) -> int {
    if NInoWofCompressed(NTFS_I(file_inode(swap_file))) { return -EOPNOTSUPP; }
    iomap_swapfile_activate(sis, swap_file, span, &ntfs_read_iomap_ops)
}

static ntfs_aops: address_space_operations = address_space_operations {
    read_folio: ntfs_read_folio, readahead: ntfs_readahead, writepages: ntfs_writepages,
    dirty_folio: iomap_dirty_folio, bmap: ntfs_bmap, migrate_folio: filemap_migrate_folio,
    is_partially_uptodate: iomap_is_partially_uptodate, error_remove_folio: generic_error_remove_folio,
    release_folio: iomap_release_folio, invalidate_folio: iomap_invalidate_folio, swap_activate: ntfs_swap_activate,
};

static ntfs_mft_aops: address_space_operations = address_space_operations {
    read_folio: ntfs_read_folio, readahead: ntfs_readahead, writepages: ntfs_mft_writepages,
    dirty_folio: iomap_dirty_folio, bmap: ntfs_bmap, migrate_folio: filemap_migrate_folio,
    is_partially_uptodate: iomap_is_partially_uptodate, error_remove_folio: generic_error_remove_folio,
    release_folio: iomap_release_folio, invalidate_folio: iomap_invalidate_folio,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
