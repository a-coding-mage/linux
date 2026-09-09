// SPDX-License-Identifier: GPL-2.0-only
/*
 * truncate.c
 *
 * PURPOSE
 *\tTruncate handling routines for the OSTA-UDF(tm) filesystem.
 *
 * COPYRIGHT
 *  (C) 1999-2004 Ben Fennema
 *  (C) 1999 Stelias Computing Inc
 *
 * HISTORY
 *
 *  02/24/99 blf  Created.
 *
 */

unsafe fn extent_trunc(
    inode: *mut inode,
    epos: *mut extent_position,
    eloc: *mut kernel_lb_addr,
    mut etype: i8,
    elen: u32,
    mut nelen: u32,
) {
    let mut neloc: kernel_lb_addr = core::mem::zeroed();
    let last_block = (elen + (*(*inode).i_sb).s_blocksize - 1)
        >> (*(*inode).i_sb).s_blocksize_bits;
    let first_block = (nelen + (*(*inode).i_sb).s_blocksize - 1)
        >> (*(*inode).i_sb).s_blocksize_bits;

    if nelen != 0 {
        if etype == (EXT_NOT_RECORDED_ALLOCATED >> 30) as i8 {
            udf_free_blocks((*inode).i_sb, inode, eloc, 0, last_block);
            etype = (EXT_NOT_RECORDED_NOT_ALLOCATED >> 30) as i8;
        } else {
            neloc = *eloc;
        }
        nelen = ((etype as u32) << 30) | nelen;
    }

    if elen != nelen {
        udf_write_aext(inode, epos, &mut neloc, nelen, 0);
        if last_block > first_block {
            if etype == (EXT_RECORDED_ALLOCATED >> 30) as i8 {
                mark_inode_dirty(inode);
            }
            if etype != (EXT_NOT_RECORDED_NOT_ALLOCATED >> 30) as i8 {
                udf_free_blocks((*inode).i_sb, inode, eloc, first_block,
                                last_block - first_block);
            }
        }
    }
}

/*
 * Truncate the last extent to match i_size. This function assumes
 * that preallocation extent is already truncated.
 */
unsafe fn udf_truncate_tail_extent(inode: *mut inode) {
    let mut epos: extent_position = core::mem::zeroed();
    let mut eloc: kernel_lb_addr = core::mem::zeroed();
    let mut elen: u32 = 0;
    let mut nelen: u32;
    let mut lbcount: u64 = 0;
    let mut etype: i8 = -1;
    let mut netype: i8 = 0;
    let mut adsize: i32;
    let iinfo = UDF_I(inode);
    let mut ret: i32;

    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB ||
       (*inode).i_size == (*iinfo).i_lenExtents { return; }
    if (*inode).i_nlink == 0 { return; }

    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_SHORT {
        adsize = core::mem::size_of::<short_ad>() as i32;
    } else if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_LONG {
        adsize = core::mem::size_of::<long_ad>() as i32;
    } else { BUG(); }

    loop {
        ret = udf_next_aext(inode, &mut epos, &mut eloc, &mut elen, &mut netype, 1);
        if ret <= 0 { break; }
        etype = netype;
        lbcount += elen as u64;
        if lbcount > (*inode).i_size as u64 {
            if lbcount - (*inode).i_size as u64 >= (*(*inode).i_sb).s_blocksize as u64 {
                udf_warn((*inode).i_sb, "Too long extent after EOF in inode %u: i_size: %lld lbcount: %lld extent %u+%u\\n",
                    (*inode).i_ino as u32, (*inode).i_size, lbcount,
                    eloc.logicalBlockNum as u32, elen);
            }
            nelen = elen - (lbcount - (*inode).i_size as u64) as u32;
            (*(&mut epos)).offset -= adsize;
            extent_trunc(inode, &mut epos, &mut eloc, etype, elen, nelen);
            epos.offset += adsize;
            if udf_next_aext(inode, &mut epos, &mut eloc, &mut elen, &mut netype, 1) > 0 {
                udf_err((*inode).i_sb, "Extent after EOF in inode %u\\n", (*inode).i_ino as u32);
            }
            break;
        }
    }
    if ret >= 0 { (*iinfo).i_lenExtents = (*inode).i_size; }
    brelse(epos.bh);
}

unsafe fn udf_discard_prealloc(inode: *mut inode) {
    let mut epos: extent_position = core::mem::zeroed();
    let mut prev_epos: extent_position = core::mem::zeroed();
    let mut eloc: kernel_lb_addr = core::mem::zeroed();
    let mut elen: u32 = 0;
    let mut lbcount: u64 = 0;
    let mut etype: i8 = -1;
    let iinfo = UDF_I(inode);
    let bsize = i_blocksize(inode);
    let mut tmpetype: i8 = -1;
    let mut ret: i32;

    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB ||
       ALIGN((*inode).i_size, bsize) == ALIGN((*iinfo).i_lenExtents, bsize) { return; }
    epos.block = (*iinfo).i_location;
    loop {
        ret = udf_next_aext(inode, &mut epos, &mut eloc, &mut elen, &mut tmpetype, 0);
        if ret < 0 { break; }
        if ret == 0 { break; }
        brelse(prev_epos.bh);
        prev_epos = epos;
        if !prev_epos.bh.is_null() { get_bh(prev_epos.bh); }
        ret = udf_next_aext(inode, &mut epos, &mut eloc, &mut elen, &mut etype, 1);
        if ret < 0 { break; }
        lbcount += elen as u64;
    }
    if etype == (EXT_NOT_RECORDED_ALLOCATED >> 30) as i8 {
        lbcount -= elen as u64;
        udf_delete_aext(inode, prev_epos, core::ptr::null_mut());
        udf_free_blocks((*inode).i_sb, inode, &mut eloc, 0, DIV_ROUND_UP(elen, bsize));
    }
    (*iinfo).i_lenExtents = lbcount;
    brelse(epos.bh);
    brelse(prev_epos.bh);
}

unsafe fn udf_update_alloc_ext_desc(inode: *mut inode, epos: *mut extent_position, lenalloc: u32) {
    let sb = (*inode).i_sb;
    let sbi = UDF_SB(sb);
    let aed = (*(*epos).bh).b_data as *mut allocExtDesc;
    let mut len = core::mem::size_of::<allocExtDesc>() as i32;
    (*aed).lengthAllocDescs = cpu_to_le32(lenalloc);
    if !UDF_QUERY_FLAG(sb, UDF_FLAG_STRICT) || (*sbi).s_udfrev >= 0x0201 { len += lenalloc as i32; }
    udf_update_tag((*(*epos).bh).b_data, len);
    mmb_mark_buffer_dirty((*epos).bh, &mut (*UDF_I(inode)).i_metadata_bhs);
}

/* Truncate extents of inode to inode->i_size. */
unsafe fn udf_truncate_extents(inode: *mut inode) -> i32 {
    let mut epos: extent_position = core::mem::zeroed();
    let mut eloc: kernel_lb_addr = core::mem::zeroed();
    let mut neloc: kernel_lb_addr = core::mem::zeroed();
    let mut elen: u32 = 0;
    let mut nelen: u32 = 0;
    let mut indirect_ext_len: u32 = 0;
    let mut lenalloc: u32;
    let mut etype: i8 = 0;
    let sb = (*inode).i_sb;
    let first_block = (*inode).i_size >> (*sb).s_blocksize_bits;
    let mut offset: i32 = 0;
    let mut byte_offset: i64;
    let adsize: i32;
    let iinfo = UDF_I(inode);
    let mut ret: i32 = 0;
    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_SHORT { adsize = core::mem::size_of::<short_ad>() as i32; }
    else if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_LONG { adsize = core::mem::size_of::<long_ad>() as i32; }
    else { BUG(); adsize = 0; }
    ret = inode_bmap(inode, first_block, &mut epos, &mut eloc, &mut elen, &mut offset, &mut etype);
    if ret < 0 { return ret; }
    byte_offset = ((offset as i64) << (*sb).s_blocksize_bits) +
        ((*inode).i_size & ((*sb).s_blocksize - 1));
    if ret == 0 { WARN_ON(byte_offset); return 0; }
    epos.offset -= adsize;
    extent_trunc(inode, &mut epos, &mut eloc, etype, elen, byte_offset as u32);
    epos.offset += adsize;
    lenalloc = if byte_offset != 0 { epos.offset as u32 } else { (epos.offset - adsize) as u32 };
    if epos.bh.is_null() { lenalloc -= udf_file_entry_alloc_offset(inode) as u32; }
    else { lenalloc -= core::mem::size_of::<allocExtDesc>() as u32; }
    while { ret = udf_current_aext(inode, &mut epos, &mut eloc, &mut elen, &mut etype, 0); ret > 0 } {
        if etype == (EXT_NEXT_EXTENT_ALLOCDESCS >> 30) as i8 {
            udf_write_aext(inode, &mut epos, &mut neloc, nelen, 0);
            if indirect_ext_len != 0 { BUG_ON(epos.bh.is_null()); udf_free_blocks(sb, core::ptr::null_mut(), &mut epos.block, 0, indirect_ext_len); }
            else if epos.bh.is_null() { (*iinfo).i_lenAlloc = lenalloc; mark_inode_dirty(inode); }
            else { udf_update_alloc_ext_desc(inode, &mut epos, lenalloc); }
            brelse(epos.bh);
            epos.offset = core::mem::size_of::<allocExtDesc>() as i32;
            epos.block = eloc;
            epos.bh = sb_bread(sb, udf_get_lb_pblock(sb, &mut eloc, 0));
            if epos.bh.is_null() { return -EIO; }
            indirect_ext_len = if elen != 0 { (elen + (*sb).s_blocksize - 1) >> (*sb).s_blocksize_bits } else { 1 };
        } else { extent_trunc(inode, &mut epos, &mut eloc, etype, elen, 0); epos.offset += adsize; }
    }
    if ret < 0 { brelse(epos.bh); return ret; }
    if indirect_ext_len != 0 { BUG_ON(epos.bh.is_null()); udf_free_blocks(sb, core::ptr::null_mut(), &mut epos.block, 0, indirect_ext_len); }
    else if epos.bh.is_null() { (*iinfo).i_lenAlloc = lenalloc; mark_inode_dirty(inode); }
    else { udf_update_alloc_ext_desc(inode, &mut epos, lenalloc); }
    (*iinfo).i_lenExtents = (*inode).i_size;
    brelse(epos.bh);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
