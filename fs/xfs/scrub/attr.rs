// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Declarations supplied by the XFS scrub implementation and its headers are
// intentionally referenced here rather than reimplemented.

unsafe fn xchk_xattr_buf_cleanup(priv_: *mut core::ffi::c_void) {
    let ab = priv_ as *mut xchk_xattr_buf;
    kvfree((*ab).freemap as *mut core::ffi::c_void);
    (*ab).freemap = core::ptr::null_mut();
    kvfree((*ab).usedmap as *mut core::ffi::c_void);
    (*ab).usedmap = core::ptr::null_mut();
    kvfree((*ab).value as *mut core::ffi::c_void);
    (*ab).value = core::ptr::null_mut();
    (*ab).value_sz = 0;
    kvfree((*ab).name as *mut core::ffi::c_void);
    (*ab).name = core::ptr::null_mut();
}

unsafe fn xchk_xattr_want_freemap(sc: *mut xfs_scrub) -> bool {
    if (*sc).flags & XCHK_TRY_HARDER != 0 { return true; }
    if (*sc).ip.is_null() { return true; }
    let ifp = xfs_ifork_ptr((*sc).ip, XFS_ATTR_FORK);
    if ifp.is_null() { return false; }
    xfs_ifork_has_extents(ifp)
}

unsafe fn xchk_setup_xattr_buf(sc: *mut xfs_scrub, value_size: usize) -> i32 {
    let bmp_sz = core::mem::size_of::<c_long>() * BITS_TO_LONGS((*(*sc).mp).m_attr_geo.blksize);
    let mut ab = (*sc).buf as *mut xchk_xattr_buf;
    let mut new_val: *mut core::ffi::c_void;

    if !ab.is_null() { return xchk_setup_xattr_buf_resize(sc, ab, value_size); }
    ab = kvzalloc_obj::<xchk_xattr_buf>(XCHK_GFP_FLAGS);
    if ab.is_null() { return -ENOMEM; }
    (*sc).buf = ab as *mut core::ffi::c_void;
    (*sc).buf_cleanup = Some(xchk_xattr_buf_cleanup);
    (*ab).usedmap = kvmalloc(bmp_sz, XCHK_GFP_FLAGS) as *mut c_ulong;
    if (*ab).usedmap.is_null() { return -ENOMEM; }
    if xchk_xattr_want_freemap(sc) {
        (*ab).freemap = kvmalloc(bmp_sz, XCHK_GFP_FLAGS) as *mut c_ulong;
        if (*ab).freemap.is_null() { return -ENOMEM; }
    }
    if xchk_could_repair(sc) {
        (*ab).name = kvmalloc(XATTR_NAME_MAX + 1, XCHK_GFP_FLAGS) as *mut u8;
        if (*ab).name.is_null() { return -ENOMEM; }
    }
    xchk_setup_xattr_buf_resize(sc, ab, value_size)
}

unsafe fn xchk_setup_xattr_buf_resize(_sc: *mut xfs_scrub, ab: *mut xchk_xattr_buf, value_size: usize) -> i32 {
    if (*ab).value_sz >= value_size { return 0; }
    if !(*ab).value.is_null() {
        kvfree((*ab).value as *mut core::ffi::c_void);
        (*ab).value = core::ptr::null_mut();
        (*ab).value_sz = 0;
    }
    let new_val = kvmalloc(value_size, XCHK_GFP_FLAGS) as *mut u8;
    if new_val.is_null() { return -ENOMEM; }
    (*ab).value = new_val;
    (*ab).value_sz = value_size;
    0
}

unsafe fn xchk_setup_xattr(sc: *mut xfs_scrub) -> i32 {
    let mut error;
    if xchk_could_repair(sc) {
        error = xrep_setup_xattr(sc);
        if error != 0 { return error; }
    }
    if (*sc).flags & XCHK_TRY_HARDER != 0 {
        error = xchk_setup_xattr_buf(sc, XATTR_SIZE_MAX);
        if error != 0 { return error; }
    }
    xchk_setup_inode_contents(sc, 0)
}

unsafe fn xchk_xattr_actor(sc: *mut xfs_scrub, ip: *mut xfs_inode,
    attr_flags: u32, name: *const u8, namelen: u32, value: *const core::ffi::c_void,
    valuelen: u32, _priv: *mut core::ffi::c_void) -> i32 {
    let mut args = xfs_da_args {
        attr_filter: attr_flags & XFS_ATTR_NSP_ONDISK_MASK, geo: (*(*sc).mp).m_attr_geo,
        whichfork: XFS_ATTR_FORK, dp: ip, name, namelen, trans: (*sc).tp,
        valuelen, owner: I_INO(ip), ..core::mem::zeroed()
    };
    let ab = (*sc).buf as *mut xchk_xattr_buf;
    let mut error = 0;
    if xchk_should_terminate(sc, &mut error) { return error; }
    if attr_flags & !XFS_ATTR_ONDISK_MASK != 0 { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, args.blkno); return -ECANCELED; }
    if attr_flags & XFS_ATTR_INCOMPLETE != 0 { xchk_ino_set_preen(sc, I_INO(ip)); return 0; }
    if !xfs_attr_namecheck(attr_flags, name, namelen) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, args.blkno); return -ECANCELED; }
    if attr_flags & XFS_ATTR_PARENT != 0 && !xfs_parent_valuecheck((*sc).mp, value, valuelen) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, args.blkno); return -ECANCELED; }
    error = xchk_setup_xattr_buf(sc, valuelen as usize);
    if error == -ENOMEM { error = -EDEADLOCK; }
    if error != 0 { return error; }
    if attr_flags & XFS_ATTR_PARENT != 0 { core::ptr::copy_nonoverlapping(value as *const u8, (*ab).value, valuelen as usize); }
    args.value = (*ab).value as *mut core::ffi::c_void;
    xfs_attr_sethash(&mut args);
    error = xfs_attr_get_ilocked(&mut args);
    if error == -ENODATA { error = -EFSCORRUPTED; }
    if !xchk_fblock_process_error(sc, XFS_ATTR_FORK, args.blkno, &mut error) { return error; }
    if args.valuelen != valuelen { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, args.blkno); }
    0
}

unsafe fn xchk_xattr_set_map(sc: *mut xfs_scrub, map: *mut c_ulong, start: u32, mut len: u32) -> bool {
    let mapsize = (*(*sc).mp).m_attr_geo.blksize;
    let mut ret = true;
    if start >= mapsize { return false; }
    if start + len > mapsize { len = mapsize - start; ret = false; }
    if find_next_bit(map, mapsize, start) < start + len { ret = false; }
    bitmap_set(map, start, len);
    ret
}

unsafe fn xchk_xattr_entry(ds: *mut xchk_da_btree, level: i32, buf_end: *mut u8,
    leaf: *mut xfs_attr_leafblock, leafhdr: *mut xfs_attr3_icleaf_hdr,
    ent: *mut xfs_attr_leaf_entry, idx: i32, usedbytes: *mut u32, last_hashval: *mut u32) {
    let mp = (*(*ds).state).mp;
    let ab = (*(*ds).sc).buf as *mut xchk_xattr_buf;
    if (*ent).pad2 != 0 { xchk_da_set_corrupt(ds, level); }
    if be32_to_cpu((*ent).hashval) < *last_hashval { xchk_da_set_corrupt(ds, level); }
    *last_hashval = be32_to_cpu((*ent).hashval);
    let nameidx = be16_to_cpu((*ent).nameidx);
    if nameidx < (*leafhdr).firstused || nameidx >= (*mp).m_attr_geo.blksize { xchk_da_set_corrupt(ds, level); return; }
    let namesize;
    let name_end;
    if (*ent).flags & XFS_ATTR_LOCAL != 0 {
        let lentry = xfs_attr3_leaf_name_local(leaf, idx);
        namesize = xfs_attr_leaf_entsize_local((*lentry).namelen, be16_to_cpu((*lentry).valuelen));
        name_end = lentry as *mut u8 .add(namesize as usize);
        if (*lentry).namelen == 0 { xchk_da_set_corrupt(ds, level); }
    } else {
        let rentry = xfs_attr3_leaf_name_remote(leaf, idx);
        namesize = xfs_attr_leaf_entsize_remote((*rentry).namelen);
        name_end = rentry as *mut u8 .add(namesize as usize);
        if (*rentry).namelen == 0 { xchk_da_set_corrupt(ds, level); }
        if (*rentry).valueblk == 0 && (*ent).flags & XFS_ATTR_INCOMPLETE == 0 { xchk_da_set_corrupt(ds, level); }
    }
    if name_end > buf_end { xchk_da_set_corrupt(ds, level); }
    if !xchk_xattr_set_map((*ds).sc, (*ab).usedmap, nameidx, namesize) { xchk_da_set_corrupt(ds, level); }
    if (*(*ds).sc).sm.as_ref().unwrap().sm_flags & XFS_SCRUB_OFLAG_CORRUPT == 0 { *usedbytes += namesize; }
}

unsafe fn xchk_xattr_block(ds: *mut xchk_da_btree, level: i32) -> i32 {
    let blk = &mut (*(*ds).state).path.blk[level as usize];
    let mp = (*(*ds).state).mp;
    let ab = (*(*ds).sc).buf as *mut xchk_xattr_buf;
    let leaf = (*blk.bp).b_addr as *mut xfs_attr_leafblock;
    let last_checked = (*ds).private as *mut xfs_dablk_t;
    if *last_checked == blk.blkno { return 0; }
    *last_checked = blk.blkno;
    bitmap_zero((*ab).usedmap, (*mp).m_attr_geo.blksize);
    bitmap_zero((*ab).freemap, (*mp).m_attr_geo.blksize);
    let mut leafhdr: xfs_attr3_icleaf_hdr = core::mem::zeroed();
    xfs_attr3_leaf_hdr_from_disk((*mp).m_attr_geo, &mut leafhdr, leaf);
    let hdrsize = xfs_attr3_leaf_hdr_size(leaf);
    if leafhdr.count == 0 { if blk.blkno == 0 { xchk_da_set_preen(ds, level); } else { xchk_da_set_corrupt(ds, level); } }
    if leafhdr.usedbytes > (*mp).m_attr_geo.blksize || leafhdr.firstused > (*mp).m_attr_geo.blksize || leafhdr.firstused < hdrsize { xchk_da_set_corrupt(ds, level); }
    if !xchk_xattr_set_map((*ds).sc, (*ab).usedmap, 0, hdrsize) { xchk_da_set_corrupt(ds, level); }
    if leafhdr.holes != 0 { xchk_da_set_preen(ds, level); }
    if (*(*ds).sc).sm.as_ref().unwrap().sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    let entries = xfs_attr3_leaf_entryp(leaf);
    if (entries.add(leafhdr.count as usize) as *mut u8) > (leaf as *mut u8).add(leafhdr.firstused as usize) { xchk_da_set_corrupt(ds, level); return 0; }
    let mut last_hashval = 0u32; let mut usedbytes = 0u32;
    for i in 0..leafhdr.count as i32 {
        let ent = entries.add(i as usize);
        let off = ent as usize - leaf as usize;
        if !xchk_xattr_set_map((*ds).sc, (*ab).usedmap, off as u32, core::mem::size_of::<xfs_attr_leaf_entry>() as u32) { xchk_da_set_corrupt(ds, level); return 0; }
        xchk_xattr_entry(ds, level, (*blk).bp.b_addr as *mut u8 .add((*mp).m_attr_geo.blksize as usize), leaf, &mut leafhdr, ent, i, &mut usedbytes, &mut last_hashval);
        if (*(*ds).sc).sm.as_ref().unwrap().sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    }
    for i in 0..XFS_ATTR_LEAF_MAPSIZE { if !xchk_xattr_set_map((*ds).sc, (*ab).freemap, leafhdr.freemap[i].base, leafhdr.freemap[i].size) { xchk_da_set_corrupt(ds, level); } if leafhdr.freemap[i].size == 0 && leafhdr.freemap[i].base > 0 { xchk_da_set_preen(ds, level); } }
    if bitmap_intersects((*ab).freemap, (*ab).usedmap, (*mp).m_attr_geo.blksize) || leafhdr.usedbytes != usedbytes { xchk_da_set_corrupt(ds, level); }
    0
}

unsafe fn xchk_xattr_rec(ds: *mut xchk_da_btree, level: i32) -> i32 {
    let blk = &mut (*(*ds).state).path.blk[level as usize];
    assert!(blk.magic == XFS_ATTR_LEAF_MAGIC);
    let ent = xfs_attr3_leaf_entryp(blk.bp.b_addr).add(blk.index as usize);
    let error = xchk_xattr_block(ds, level); if error != 0 { return error; }
    if (*(*ds).sc).sm.as_ref().unwrap().sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    xchk_da_btree_hash(ds, level, &mut (*ent).hashval)
}

// The remaining leaf/block and shortform checks preserve the original XFS
// operations and rely on the corresponding external C-layout declarations.
unsafe fn xchk_xattr_check_sf(sc: *mut xfs_scrub) -> i32 {
    let ab = (*sc).buf as *mut xchk_xattr_buf;
    let ifp = &mut (*(*sc).ip).i_af;
    let sf = ifp.if_data as *mut xfs_attr_sf_hdr;
    let mut sfe = xfs_attr_sf_firstentry(sf);
    let end = ifp.if_data.add(ifp.if_bytes);
    let mut error = 0;
    bitmap_zero((*ab).usedmap, ifp.if_bytes);
    xchk_xattr_set_map(sc, (*ab).usedmap, 0, core::mem::size_of::<xfs_attr_sf_hdr>() as u32);
    if sfe as *mut u8 > end { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); return 0; }
    for _ in 0..(*sf).count {
        let name = (*sfe).nameval;
        let value = name.add((*sfe).namelen as usize);
        if xchk_should_terminate(sc, &mut error) { return error; }
        let next = xfs_attr_sf_nextentry(sfe);
        if next as *mut u8 > end { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); break; }
        if (*sfe).flags & !XFS_ATTR_NSP_ONDISK_MASK != 0 { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); break; }
        if !xchk_xattr_set_map(sc, (*ab).usedmap, sfe as usize - sf as usize, core::mem::size_of::<xfs_attr_sf_entry>() as u32) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); break; }
        if !xchk_xattr_set_map(sc, (*ab).usedmap, name as usize - sf as usize, (*sfe).namelen as u32) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); break; }
        if !xchk_xattr_set_map(sc, (*ab).usedmap, value as usize - sf as usize, (*sfe).valuelen as u32) { xchk_fblock_set_corrupt(sc, XFS_ATTR_FORK, 0); break; }
        sfe = next;
    }
    0
}

unsafe fn xchk_xattr(sc: *mut xfs_scrub) -> i32 {
    let mut last_checked: xfs_dablk_t = !0;
    let mut error = 0;
    if !xfs_inode_hasattr((*sc).ip) { return -ENOENT; }
    error = xchk_setup_xattr_buf(sc, 0);
    if error == -ENOMEM { return -EDEADLOCK; }
    if error != 0 { return error; }
    if (*sc).ip.as_ref().unwrap().i_af.if_format == XFS_DINODE_FMT_LOCAL {
        error = xchk_xattr_check_sf(sc);
    } else {
        error = xchk_da_btree(sc, XFS_ATTR_FORK, xchk_xattr_rec, &mut last_checked);
    }
    if error != 0 { return error; }
    if (*sc).sm.as_ref().unwrap().sm_flags & XFS_SCRUB_OFLAG_CORRUPT != 0 { return 0; }
    error = xchk_xattr_walk(sc, (*sc).ip, xchk_xattr_actor, core::ptr::null_mut(), core::ptr::null_mut());
    if !xchk_fblock_process_error(sc, XFS_ATTR_FORK, 0, &mut error) { return error; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
