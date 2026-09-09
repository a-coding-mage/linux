// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies correspond to the C headers included by the original source.

/* Call a function for every entry in a shortform directory. */
unsafe fn xchk_dir_walk_sf(
    sc: *mut xfs_scrub,
    dp: *mut xfs_inode,
    dirent_fn: xchk_dirent_fn,
    priv_: *mut c_void,
) -> c_int {
    let mut name = xfs_name { name: ".", len: 1, type_: XFS_DIR3_FT_DIR };
    let mp = (*dp).i_mount;
    let geo = (*mp).m_dir_geo;
    let mut sfep: *mut xfs_dir2_sf_entry;
    let sfp = (*dp).i_df.if_data as *mut xfs_dir2_sf_hdr;
    let mut ino: xfs_ino_t;
    let mut dapos: xfs_dir2_dataptr_t;
    let mut error: c_int;

    ASSERT((*dp).i_df.if_bytes == (*dp).i_disk_size);
    ASSERT(!sfp.is_null());

    dapos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk, (*geo).data_entry_offset);
    error = dirent_fn(sc, dp, dapos, &mut name, I_INO(dp), priv_);
    if error != 0 { return error; }

    dapos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk,
        (*geo).data_entry_offset + xfs_dir2_data_entsize(mp, (".".len() - 1) as _));
    ino = xfs_dir2_sf_get_parent_ino(sfp);
    name.name = "..";
    name.len = 2;
    error = dirent_fn(sc, dp, dapos, &mut name, ino, priv_);
    if error != 0 { return error; }

    sfep = xfs_dir2_sf_firstentry(sfp);
    for _ in 0..(*sfp).count {
        dapos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk,
            xfs_dir2_sf_get_offset(sfep));
        ino = xfs_dir2_sf_get_ino(mp, sfp, sfep);
        name.name = (*sfep).name;
        name.len = (*sfep).namelen;
        name.type_ = xfs_dir2_sf_get_ftype(mp, sfep);
        error = dirent_fn(sc, dp, dapos, &mut name, ino, priv_);
        if error != 0 { return error; }
        sfep = xfs_dir2_sf_nextentry(mp, sfp, sfep);
    }
    0
}

/* Call a function for every entry in a block directory. */
unsafe fn xchk_dir_walk_block(sc: *mut xfs_scrub, dp: *mut xfs_inode,
    dirent_fn: xchk_dirent_fn, priv_: *mut c_void) -> c_int {
    let mp = (*dp).i_mount;
    let geo = (*mp).m_dir_geo;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error = xfs_dir3_block_read((*sc).tp, dp, I_INO(dp), &mut bp);
    if error != 0 { return error; }
    let end = xfs_dir3_data_end_offset(geo, (*bp).b_addr);
    let mut off = (*geo).data_entry_offset;
    while off < end {
        let mut name = xfs_name::default();
        let dup = ((*bp).b_addr as *mut u8).add(off as usize) as *mut xfs_dir2_data_unused;
        let dep = dup as *mut xfs_dir2_data_entry;
        let next_off;
        if be16_to_cpu((*dup).freetag) == XFS_DIR2_DATA_FREE_TAG {
            next_off = off + be16_to_cpu((*dup).length);
            off = next_off; continue;
        }
        next_off = off + xfs_dir2_data_entsize(mp, (*dep).namelen);
        if next_off > end { break; }
        let dapos = xfs_dir2_db_off_to_dataptr(geo, (*geo).datablk, off);
        let ino = be64_to_cpu((*dep).inumber);
        name.name = (*dep).name;
        name.len = (*dep).namelen;
        name.type_ = xfs_dir2_data_get_ftype(mp, dep);
        error = dirent_fn(sc, dp, dapos, &mut name, ino, priv_);
        if error != 0 { break; }
        off = next_off;
    }
    xfs_trans_brelse((*sc).tp, bp);
    error
}

/* Read a leaf-format directory buffer. */
unsafe fn xchk_read_leaf_dir_buf(tp: *mut xfs_trans, dp: *mut xfs_inode,
    geo: *mut xfs_da_geometry, curoff: *mut xfs_dir2_off_t,
    bpp: *mut *mut xfs_buf) -> c_int {
    let mut icur = xfs_iext_cursor::default();
    let mut map = xfs_bmbt_irec::default();
    let ifp = xfs_ifork_ptr(dp, XFS_DATA_FORK);
    *bpp = core::ptr::null_mut();
    let last_da = xfs_dir2_byte_to_da(geo, XFS_DIR2_LEAF_OFFSET);
    let map_off = xfs_dir2_db_to_da(geo, xfs_dir2_byte_to_db(geo, *curoff));
    if !xfs_iext_lookup_extent(dp, ifp, map_off, &mut icur, &mut map) { return 0; }
    if map.br_startoff >= last_da { return 0; }
    xfs_trim_extent(&mut map, map_off, last_da - map_off);
    let new_off = xfs_dir2_da_to_byte(geo, map.br_startoff);
    if new_off > *curoff { *curoff = new_off; }
    xfs_dir3_data_read(tp, dp, I_INO(dp), map.br_startoff, 0, bpp)
}

/* Call a function for every entry in a leaf directory. */
unsafe fn xchk_dir_walk_leaf(sc: *mut xfs_scrub, dp: *mut xfs_inode,
    dirent_fn: xchk_dirent_fn, priv_: *mut c_void) -> c_int {
    let mp = (*dp).i_mount; let geo = (*mp).m_dir_geo;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut curoff: xfs_dir2_off_t = 0; let mut offset: u32 = 0; let mut error = 0;
    while curoff < XFS_DIR2_LEAF_OFFSET {
        if bp.is_null() || offset >= (*geo).blksize {
            if !bp.is_null() { xfs_trans_brelse((*sc).tp, bp); bp = core::ptr::null_mut(); }
            error = xchk_read_leaf_dir_buf((*sc).tp, dp, geo, &mut curoff, &mut bp);
            if error != 0 || bp.is_null() { break; }
            offset = (*geo).data_entry_offset; curoff += (*geo).data_entry_offset;
        }
        let dup = ((*bp).b_addr as *mut u8).add(offset as usize) as *mut xfs_dir2_data_unused;
        if be16_to_cpu((*dup).freetag) == XFS_DIR2_DATA_FREE_TAG {
            let length = be16_to_cpu((*dup).length); offset += length; curoff += length; continue;
        }
        let dep = dup as *mut xfs_dir2_data_entry; let length = xfs_dir2_data_entsize(mp, (*dep).namelen);
        let mut name = xfs_name::default(); let dapos = xfs_dir2_byte_to_dataptr(curoff) & 0x7fffffff;
        let ino = be64_to_cpu((*dep).inumber); name.name = (*dep).name; name.len = (*dep).namelen; name.type_ = xfs_dir2_data_get_ftype(mp, dep);
        error = dirent_fn(sc, dp, dapos, &mut name, ino, priv_); if error != 0 { break; }
        offset += length; curoff += length;
    }
    if !bp.is_null() { xfs_trans_brelse((*sc).tp, bp); } error
}

/* Call a function for every entry in a directory. */
pub unsafe fn xchk_dir_walk(sc: *mut xfs_scrub, dp: *mut xfs_inode,
    dirent_fn: xchk_dirent_fn, priv_: *mut c_void) -> c_int {
    let mut args = xfs_da_args { dp, geo: (*(*dp).i_mount).m_dir_geo, trans: (*sc).tp,
        owner: I_INO(dp), ..Default::default() };
    let mut error = 0;
    if xfs_is_shutdown((*dp).i_mount) { return -EIO; }
    ASSERT(S_ISDIR(VFS_I(dp).i_mode)); xfs_assert_ilocked(dp, XFS_ILOCK_SHARED | XFS_ILOCK_EXCL);
    match xfs_dir2_format(&mut args, &mut error) {
        XFS_DIR2_FMT_SF => xchk_dir_walk_sf(sc, dp, dirent_fn, priv_),
        XFS_DIR2_FMT_BLOCK => xchk_dir_walk_block(sc, dp, dirent_fn, priv_),
        XFS_DIR2_FMT_LEAF | XFS_DIR2_FMT_NODE => xchk_dir_walk_leaf(sc, dp, dirent_fn, priv_),
        _ => error,
    }
}

/* Look up the inode number for an exact name in a directory. */
pub unsafe fn xchk_dir_lookup(sc: *mut xfs_scrub, dp: *mut xfs_inode,
    name: *const xfs_name, ino: *mut xfs_ino_t) -> c_int {
    let mut args = xfs_da_args { dp, geo: (*(*dp).i_mount).m_dir_geo, trans: (*sc).tp,
        name: (*name).name, namelen: (*name).len, filetype: (*name).type_,
        hashval: xfs_dir2_hashname((*dp).i_mount, name), whichfork: XFS_DATA_FORK,
        op_flags: XFS_DA_OP_OKNOENT, owner: I_INO(dp), ..Default::default() };
    if xfs_is_shutdown((*dp).i_mount) { return -EIO; }
    if dp == (*sc).tempip { args.owner = I_INO((*sc).ip); }
    ASSERT(S_ISDIR(VFS_I(dp).i_mode)); xfs_assert_ilocked(dp, XFS_ILOCK_SHARED | XFS_ILOCK_EXCL);
    let error = xfs_dir_lookup_args(&mut args); if error == 0 { *ino = args.inumber; } error
}

static unsafe fn xchk_dir_trylock_both(sc: *mut xfs_scrub, ip: *mut xfs_inode) -> c_uint {
    if !xchk_ilock_nowait(sc, XFS_IOLOCK_EXCL) { return 0; }
    if !xfs_ilock_nowait(ip, XFS_IOLOCK_SHARED) { xchk_iunlock(sc, XFS_IOLOCK_EXCL); return 0; }
    xchk_ilock(sc, XFS_ILOCK_EXCL);
    if !xfs_ilock_nowait(ip, XFS_ILOCK_EXCL) {
        xchk_iunlock(sc, XFS_ILOCK_EXCL); xfs_iunlock(ip, XFS_IOLOCK_SHARED); xchk_iunlock(sc, XFS_IOLOCK_EXCL); return 0;
    }
    XFS_IOLOCK_SHARED | XFS_ILOCK_EXCL
}

pub unsafe fn xchk_dir_trylock_for_pptrs(sc: *mut xfs_scrub, ip: *mut xfs_inode,
    lockmode: *mut c_uint) -> c_int {
    let mut error = 0; ASSERT((*sc).ilock_flags == 0);
    for _ in 0..HZ { *lockmode = xchk_dir_trylock_both(sc, ip); if *lockmode != 0 { return 0; }
        if xchk_should_terminate(sc, &mut error) { return error; } delay(1); }
    if (*sc).flags & XCHK_TRY_HARDER != 0 { xchk_set_incomplete(sc); return -ETIMEDOUT; }
    -EDEADLOCK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
