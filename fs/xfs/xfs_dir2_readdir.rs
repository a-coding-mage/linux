// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

/* Directory file type support functions. */
static mut XFS_DIR3_FILETYPE_TABLE: [u8; 9] = [
    DT_UNKNOWN, DT_REG, DT_DIR, DT_CHR, DT_BLK,
    DT_FIFO, DT_SOCK, DT_LNK, DT_WHT,
];

pub unsafe fn xfs_dir3_get_dtype(mp: *mut xfs_mount, filetype: u8) -> u8 {
    if !xfs_has_ftype(mp) { return DT_UNKNOWN; }
    if filetype >= XFS_DIR3_FT_MAX { return DT_UNKNOWN; }
    XFS_DIR3_FILETYPE_TABLE[filetype as usize]
}

unsafe fn xfs_dir2_sf_getdents(args: *mut xfs_da_args, ctx: *mut dir_context) -> i32 {
    let mut i: i32;
    let dp = (*args).dp;
    let mp = (*dp).i_mount;
    let mut off: xfs_dir2_dataptr_t;
    let mut sfep: *mut xfs_dir2_sf_entry_t;
    let sfp = (*dp).i_df.if_data as *mut xfs_dir2_sf_hdr;
    let dot_offset: xfs_dir2_dataptr_t;
    let dotdot_offset: xfs_dir2_dataptr_t;
    let mut ino: xfs_ino_t;
    let geo = (*args).geo;

    ASSERT((*dp).i_df.if_format == XFS_DINODE_FMT_LOCAL);
    ASSERT((*dp).i_df.if_bytes == (*dp).i_disk_size);
    ASSERT(!sfp.is_null());
    if xfs_dir2_dataptr_to_db(geo, (*ctx).pos) > (*geo).datablk { return 0; }
    dot_offset = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk, (*geo).data_entry_offset);
    dotdot_offset = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk,
        (*geo).data_entry_offset + xfs_dir2_data_entsize(mp, (".".len()) as usize));
    if (*ctx).pos <= dot_offset {
        (*ctx).pos = dot_offset & 0x7fffffff;
        if !dir_emit(ctx, "." as *const str as *const i8, 1, I_INO(dp), DT_DIR) { return 0; }
    }
    if (*ctx).pos <= dotdot_offset {
        ino = xfs_dir2_sf_get_parent_ino(sfp);
        (*ctx).pos = dotdot_offset & 0x7fffffff;
        if !dir_emit(ctx, ".." as *const str as *const i8, 2, ino, DT_DIR) { return 0; }
    }
    sfep = xfs_dir2_sf_firstentry(sfp);
    i = 0;
    while i < (*sfp).count {
        let filetype: u8;
        off = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk, xfs_dir2_sf_get_offset(sfep));
        if (*ctx).pos > off {
            sfep = xfs_dir2_sf_nextentry(mp, sfp, sfep);
            i += 1;
            continue;
        }
        ino = xfs_dir2_sf_get_ino(mp, sfp, sfep);
        filetype = xfs_dir2_sf_get_ftype(mp, sfep);
        (*ctx).pos = off & 0x7fffffff;
        if XFS_IS_CORRUPT((*dp).i_mount, !xfs_dir2_namecheck((*sfep).name, (*sfep).namelen)) {
            xfs_dirattr_mark_sick(dp, XFS_DATA_FORK); return -EFSCORRUPTED;
        }
        if !dir_emit(ctx, (*sfep).name as *const i8, (*sfep).namelen, ino, xfs_dir3_get_dtype(mp, filetype)) { return 0; }
        sfep = xfs_dir2_sf_nextentry(mp, sfp, sfep);
        i += 1;
    }
    (*ctx).pos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk + 1, 0) & 0x7fffffff;
    0
}

unsafe fn xfs_dir2_block_getdents(args: *mut xfs_da_args, ctx: *mut dir_context, lock_mode: *mut u32) -> i32 {
    let dp = (*args).dp;
    let geo = (*args).geo;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error: i32;
    let wantoff: i32;
    let mut cook: xfs_off_t;
    let mut offset: u32;
    let mut next_offset: u32;
    let end: u32;
    if xfs_dir2_dataptr_to_db(geo, (*ctx).pos) > (*geo).datablk { return 0; }
    error = xfs_dir3_block_read((*args).trans, dp, (*args).owner, &mut bp);
    if error != 0 { return error; }
    xfs_iunlock(dp, *lock_mode); *lock_mode = 0;
    wantoff = xfs_dir2_dataptr_to_off(geo, (*ctx).pos);
    xfs_dir3_data_check(dp, bp);
    end = xfs_dir3_data_end_offset(geo, (*bp).b_addr);
    offset = (*geo).data_entry_offset;
    while offset < end {
        let dup = ((*bp).b_addr as *mut u8).add(offset as usize) as *mut xfs_dir2_data_unused;
        let dep = dup as *mut xfs_dir2_data_entry;
        let filetype: u8;
        if be16_to_cpu((*dup).freetag) == XFS_DIR2_DATA_FREE_TAG {
            next_offset = offset + be16_to_cpu((*dup).length) as u32; offset = next_offset; continue;
        }
        next_offset = offset + xfs_dir2_data_entsize((*dp).i_mount, (*dep).namelen) as u32;
        if (offset as i32) < wantoff { offset = next_offset; continue; }
        cook = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk, offset);
        (*ctx).pos = cook & 0x7fffffff;
        filetype = xfs_dir2_data_get_ftype((*dp).i_mount, dep);
        if XFS_IS_CORRUPT((*dp).i_mount, !xfs_dir2_namecheck((*dep).name, (*dep).namelen)) {
            xfs_dirattr_mark_sick(dp, XFS_DATA_FORK); error = -EFSCORRUPTED; break;
        }
        if !dir_emit(ctx, (*dep).name as *const i8, (*dep).namelen, be64_to_cpu((*dep).inumber), xfs_dir3_get_dtype((*dp).i_mount, filetype)) { break; }
        offset = next_offset;
    }
    if offset >= end { (*ctx).pos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk + 1, 0) & 0x7fffffff; }
    xfs_trans_brelse((*args).trans, bp); error
}

unsafe fn xfs_dir2_leaf_readbuf(args: *mut xfs_da_args, bufsize: usize, cur_off: *mut xfs_dir2_off_t, ra_blk: *mut xfs_dablk_t, bpp: *mut *mut xfs_buf) -> i32 {
    let dp = (*args).dp; let geo = (*args).geo; let ifp = xfs_ifork_ptr(dp, XFS_DATA_FORK);
    let mut bp: *mut xfs_buf = core::ptr::null_mut(); let mut map: xfs_bmbt_irec = core::mem::zeroed();
    let mut icur: xfs_iext_cursor = core::mem::zeroed(); let mut error = xfs_iread_extents((*args).trans, dp, XFS_DATA_FORK);
    if error != 0 { *bpp = bp; return error; }
    let last_da = xfs_dir2_byte_to_da(geo, XFS_DIR2_LEAF_OFFSET);
    let map_off = xfs_dir2_db_to_da(geo, xfs_dir2_byte_to_db(geo, *cur_off));
    if !xfs_iext_lookup_extent(dp, ifp, map_off, &mut icur, &mut map) || map.br_startoff >= last_da { *bpp = bp; return 0; }
    xfs_trim_extent(&mut map, map_off, last_da - map_off);
    let new_off = xfs_dir2_da_to_byte(geo, map.br_startoff); if new_off > *cur_off { *cur_off = new_off; }
    error = xfs_dir3_data_read((*args).trans, dp, (*args).owner, map.br_startoff, 0, &mut bp); if error != 0 { *bpp = bp; return error; }
    let mut ra_want = howmany(bufsize + (*geo).blksize, 1usize << (*geo).fsblog);
    if *ra_blk < last_da { if *ra_blk == 0 { *ra_blk = map.br_startoff; }
        let mut next_ra = map.br_startoff + (*geo).fsbcount;
        if next_ra < last_da { if map.br_blockcount < (*geo).fsbcount && !xfs_iext_next_extent(ifp, &mut icur, &mut map) { *ra_blk = last_da; } else if map.br_startoff < last_da {
            xfs_trim_extent(&mut map, next_ra, last_da - next_ra); let mut plug: blk_plug = core::mem::zeroed(); blk_start_plug(&mut plug);
            while ra_want > 0 { next_ra = roundup(map.br_startoff, (*geo).fsbcount); while ra_want > 0 && next_ra < map.br_startoff + map.br_blockcount { if next_ra >= last_da { *ra_blk = last_da; break; } if next_ra > *ra_blk { xfs_dir3_data_readahead(dp, next_ra, XFS_DABUF_MAP_HOLE_OK); *ra_blk = next_ra; } ra_want -= (*geo).fsbcount as usize; next_ra += (*geo).fsbcount; } if !xfs_iext_next_extent(ifp, &mut icur, &mut map) { *ra_blk = last_da; break; } }
            blk_finish_plug(&mut plug);
        }}
    }
    *bpp = bp; error
}

unsafe fn xfs_dir2_leaf_getdents(args: *mut xfs_da_args, ctx: *mut dir_context, mut bufsize: usize, lock_mode: *mut u32) -> i32 {
    let dp = (*args).dp; let mp = (*dp).i_mount; let geo = (*args).geo; let mut bp: *mut xfs_buf = core::ptr::null_mut(); let mut rablk: xfs_dablk_t = 0; let mut curoff = xfs_dir2_dataptr_to_byte((*ctx).pos); let mut offset: u32 = 0; let mut error = 0;
    if (*ctx).pos >= XFS_DIR2_MAX_DATAPTR { return 0; }
    while curoff < XFS_DIR2_LEAF_OFFSET { let mut length: i32; if bp.is_null() || offset >= (*geo).blksize { if !bp.is_null() { xfs_trans_brelse((*args).trans, bp); bp = core::ptr::null_mut(); } if *lock_mode == 0 { *lock_mode = xfs_ilock_data_map_shared(dp); } error = xfs_dir2_leaf_readbuf(args, bufsize, &mut curoff, &mut rablk, &mut bp); if error != 0 || bp.is_null() { break; } xfs_iunlock(dp, *lock_mode); *lock_mode = 0; xfs_dir3_data_check(dp, bp); offset = (*geo).data_entry_offset; let byteoff = xfs_dir2_byte_to_off(geo, curoff); if byteoff == 0 { curoff += (*geo).data_entry_offset; } else { while offset < byteoff { let dup = ((*bp).b_addr as *mut u8).add(offset as usize) as *mut xfs_dir2_data_unused; if be16_to_cpu((*dup).freetag) == XFS_DIR2_DATA_FREE_TAG { length = be16_to_cpu((*dup).length) as i32; offset += length as u32; } else { let dep = dup as *mut xfs_dir2_data_entry; length = xfs_dir2_data_entsize(mp, (*dep).namelen) as i32; offset += length as u32; } } curoff = xfs_dir2_db_off_to_byte(geo, xfs_dir2_byte_to_db(geo, curoff), offset); if offset >= (*geo).blksize { continue; } } }
        let dup = ((*bp).b_addr as *mut u8).add(offset as usize) as *mut xfs_dir2_data_unused; if be16_to_cpu((*dup).freetag) == XFS_DIR2_DATA_FREE_TAG { length = be16_to_cpu((*dup).length) as i32; offset += length as u32; curoff += length as i64; continue; }
        let dep = dup as *mut xfs_dir2_data_entry; length = xfs_dir2_data_entsize(mp, (*dep).namelen) as i32; let filetype = xfs_dir2_data_get_ftype(mp, dep); (*ctx).pos = xfs_dir2_byte_to_dataptr(curoff) & 0x7fffffff;
        if XFS_IS_CORRUPT((*dp).i_mount, !xfs_dir2_namecheck((*dep).name, (*dep).namelen)) { xfs_dirattr_mark_sick(dp, XFS_DATA_FORK); error = -EFSCORRUPTED; break; }
        if !dir_emit(ctx, (*dep).name as *const i8, (*dep).namelen, be64_to_cpu((*dep).inumber), xfs_dir3_get_dtype((*dp).i_mount, filetype)) { break; }
        offset += length as u32; curoff += length as i64; bufsize = if bufsize > length as usize { bufsize - length as usize } else { 0 };
    }
    if curoff > xfs_dir2_dataptr_to_byte(XFS_DIR2_MAX_DATAPTR) { (*ctx).pos = XFS_DIR2_MAX_DATAPTR & 0x7fffffff; } else { (*ctx).pos = xfs_dir2_byte_to_dataptr(curoff) & 0x7fffffff; } if !bp.is_null() { xfs_trans_brelse((*args).trans, bp); } error
}

pub unsafe fn xfs_readdir(tp: *mut xfs_trans, dp: *mut xfs_inode, ctx: *mut dir_context, bufsize: usize) -> i32 {
    let mut args: xfs_da_args = core::mem::zeroed(); let mut lock_mode: u32; let mut error: i32;
    trace_xfs_readdir(dp); if xfs_is_shutdown((*dp).i_mount) || xfs_ifork_zapped(dp, XFS_DATA_FORK) { return -EIO; }
    ASSERT(S_ISDIR((*VFS_I(dp)).i_mode)); xfs_assert_ilocked(dp, XFS_IOLOCK_SHARED | XFS_IOLOCK_EXCL); XFS_STATS_INC((*dp).i_mount, xs_dir_getdents);
    args.dp = dp; args.geo = (*dp).i_mount->m_dir_geo; args.trans = tp; args.owner = I_INO(dp);
    if (*dp).i_df.if_format == XFS_DINODE_FMT_LOCAL { return xfs_dir2_sf_getdents(&mut args, ctx); }
    lock_mode = xfs_ilock_data_map_shared(dp); match xfs_dir2_format(&mut args, &mut error) { XFS_DIR2_FMT_BLOCK => error = xfs_dir2_block_getdents(&mut args, ctx, &mut lock_mode), XFS_DIR2_FMT_LEAF | XFS_DIR2_FMT_NODE => error = xfs_dir2_leaf_getdents(&mut args, ctx, bufsize, &mut lock_mode), _ => {} }
    if lock_mode != 0 { xfs_iunlock(dp, lock_mode); } error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
