// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependencies supplied by the surrounding XFS translation.

/* Call a function for every entry in a shortform xattr structure. */
unsafe fn xchk_xattr_walk_sf(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    attr_fn: xchk_xattr_fn,
    priv_: *mut core::ffi::c_void,
) -> i32 {
    let hdr = (*ip).i_af.if_data as *mut xfs_attr_sf_hdr;
    let mut sfe: *mut xfs_attr_sf_entry;
    let mut i: u32;
    let mut error: i32;

    sfe = xfs_attr_sf_firstentry(hdr);
    i = 0;
    while i < (*hdr).count {
        error = attr_fn(
            sc,
            ip,
            (*sfe).flags,
            (*sfe).nameval,
            (*sfe).namelen,
            (*sfe).nameval.add((*sfe).namelen as usize),
            (*sfe).valuelen,
            priv_,
        );
        if error != 0 {
            return error;
        }

        sfe = xfs_attr_sf_nextentry(sfe);
        i += 1;
    }

    0
}

/* Call a function for every entry in this xattr leaf block. */
unsafe fn xchk_xattr_walk_leaf_entries(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    attr_fn: xchk_xattr_fn,
    bp: *mut xfs_buf,
    priv_: *mut core::ffi::c_void,
) -> i32 {
    let mut ichdr: xfs_attr3_icleaf_hdr = core::mem::zeroed();
    let mp = (*sc).mp;
    let leaf = (*bp).b_addr as *mut xfs_attr_leafblock;
    let mut entry: *mut xfs_attr_leaf_entry;
    let mut i: u32;
    let mut error: i32;

    xfs_attr3_leaf_hdr_from_disk((*mp).m_attr_geo, &mut ichdr, leaf);
    entry = xfs_attr3_leaf_entryp(leaf);

    i = 0;
    while i < ichdr.count {
        let value: *mut core::ffi::c_void;
        let name: *mut u8;
        let namelen: u32;
        let valuelen: u32;

        if (*entry).flags & XFS_ATTR_LOCAL != 0 {
            let name_loc = xfs_attr3_leaf_name_local(leaf, i);
            name = (*name_loc).nameval;
            namelen = (*name_loc).namelen;
            value = (*name_loc).nameval.add((*name_loc).namelen as usize) as *mut core::ffi::c_void;
            valuelen = be16_to_cpu((*name_loc).valuelen) as u32;
        } else {
            let name_rmt = xfs_attr3_leaf_name_remote(leaf, i);
            name = (*name_rmt).name;
            namelen = (*name_rmt).namelen;
            value = core::ptr::null_mut();
            valuelen = be32_to_cpu((*name_rmt).valuelen);
        }

        error = attr_fn(sc, ip, (*entry).flags, name, namelen, value, valuelen, priv_);
        if error != 0 {
            return error;
        }

        entry = entry.add(1);
        i += 1;
    }

    0
}

/*
 * Call a function for every entry in a leaf-format xattr structure.  Avoid
 * memory allocations for the loop detector since there's only one block.
 */
unsafe fn xchk_xattr_walk_leaf(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    attr_fn: xchk_xattr_fn,
    priv_: *mut core::ffi::c_void,
) -> i32 {
    let mut leaf_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error = xfs_attr3_leaf_read((*sc).tp, ip, I_INO(ip), 0, &mut leaf_bp);
    if error != 0 {
        return error;
    }

    error = xchk_xattr_walk_leaf_entries(sc, ip, attr_fn, leaf_bp, priv_);
    xfs_trans_brelse((*sc).tp, leaf_bp);
    error
}

/* Find the leftmost leaf in the xattr dabtree. */
unsafe fn xchk_xattr_find_leftmost_leaf(
    sc: *mut xfs_scrub,
    ip: *mut xfs_inode,
    seen_dablks: *mut xdab_bitmap,
    leaf_bpp: *mut *mut xfs_buf,
) -> i32 {
    let mut nodehdr: xfs_da3_icnode_hdr = core::mem::zeroed();
    let mp = (*sc).mp;
    let tp = (*sc).tp;
    let mut node: *mut xfs_da_intnode;
    let mut btree: *mut xfs_da_node_entry;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    let mut fa: xfs_failaddr_t;
    let mut blkno: xfs_dablk_t = 0;
    let mut expected_level: u32 = 0;

    loop {
        let mut len: xfs_extlen_t = 1;
        let magic: u16;
        if xdab_bitmap_test(seen_dablks, blkno, &mut len) {
            return -EFSCORRUPTED;
        }
        let mut error = xfs_da3_node_read(tp, ip, blkno, &mut bp, XFS_ATTR_FORK);
        if error != 0 { return error; }
        node = (*bp).b_addr as *mut xfs_da_intnode;
        magic = be16_to_cpu((*node).hdr.info.magic);
        if magic == XFS_ATTR_LEAF_MAGIC || magic == XFS_ATTR3_LEAF_MAGIC { break; }
        error = -EFSCORRUPTED;
        if magic != XFS_DA_NODE_MAGIC && magic != XFS_DA3_NODE_MAGIC { goto out_buf; }
        fa = xfs_da3_node_header_check(bp, I_INO(ip));
        if !fa.is_null() { goto out_buf; }
        xfs_da3_node_hdr_from_disk(mp, &mut nodehdr, node);
        if nodehdr.count == 0 || nodehdr.level >= XFS_DA_NODE_MAXDEPTH { goto out_buf; }
        if blkno == 0 { expected_level = nodehdr.level - 1; }
        else if expected_level != nodehdr.level { goto out_buf; }
        else { expected_level -= 1; }
        error = xdab_bitmap_set(seen_dablks, blkno, 1);
        if error != 0 { goto out_buf; }
        btree = nodehdr.btree;
        blkno = be32_to_cpu((*btree).before);
        xfs_trans_brelse(tp, bp);
    }
    let mut error = -EFSCORRUPTED;
    fa = xfs_attr3_leaf_header_check(bp, I_INO(ip));
    if !fa.is_null() { goto out_buf; }
    if expected_level != 0 { goto out_buf; }
    error = xdab_bitmap_set(seen_dablks, blkno, 1);
    if error != 0 { goto out_buf; }
    *leaf_bpp = bp;
    return 0;

out_buf:
    xfs_trans_brelse(tp, bp);
    error
}

/* Call a function for every entry in a node-format xattr structure. */
unsafe fn xchk_xattr_walk_node(
    sc: *mut xfs_scrub, ip: *mut xfs_inode, attr_fn: xchk_xattr_fn,
    leaf_fn: xchk_xattrleaf_fn, priv_: *mut core::ffi::c_void,
) -> i32 {
    let mut leafhdr: xfs_attr3_icleaf_hdr = core::mem::zeroed();
    let mut seen_dablks: xdab_bitmap = core::mem::zeroed();
    let mp = (*sc).mp;
    let mut leaf: *mut xfs_attr_leafblock;
    let mut leaf_bp: *mut xfs_buf = core::ptr::null_mut();
    let mut error: i32;
    xdab_bitmap_init(&mut seen_dablks);
    error = xchk_xattr_find_leftmost_leaf(sc, ip, &mut seen_dablks, &mut leaf_bp);
    if error != 0 { goto out_bitmap; }
    loop {
        error = xchk_xattr_walk_leaf_entries(sc, ip, attr_fn, leaf_bp, priv_);
        if error != 0 { goto out_leaf; }
        leaf = (*leaf_bp).b_addr as *mut xfs_attr_leafblock;
        xfs_attr3_leaf_hdr_from_disk((*mp).m_attr_geo, &mut leafhdr, leaf);
        if leafhdr.forw == 0 { goto out_leaf; }
        xfs_trans_brelse((*sc).tp, leaf_bp);
        if !leaf_fn.is_none() { error = leaf_fn.unwrap()(sc, priv_); if error != 0 { goto out_bitmap; } }
        let mut len: xfs_extlen_t = 1;
        if xdab_bitmap_test(&mut seen_dablks, leafhdr.forw, &mut len) { error = -EFSCORRUPTED; goto out_bitmap; }
        error = xfs_attr3_leaf_read((*sc).tp, ip, I_INO(ip), leafhdr.forw, &mut leaf_bp);
        if error != 0 { goto out_bitmap; }
        error = xdab_bitmap_set(&mut seen_dablks, leafhdr.forw, 1);
        if error != 0 { goto out_leaf; }
    }
out_leaf:
    xfs_trans_brelse((*sc).tp, leaf_bp);
out_bitmap:
    xdab_bitmap_destroy(&mut seen_dablks);
    error
}

/*
 * Call a function for every extended attribute in a file.
 *
 * Callers must hold the ILOCK.  No validation or cursor restarts allowed.
 * Returns -EFSCORRUPTED on any problem, including loops in the dabtree.
 */
pub unsafe fn xchk_xattr_walk(
    sc: *mut xfs_scrub, ip: *mut xfs_inode, attr_fn: xchk_xattr_fn,
    leaf_fn: xchk_xattrleaf_fn, priv_: *mut core::ffi::c_void,
) -> i32 {
    xfs_assert_ilocked(ip, XFS_ILOCK_SHARED | XFS_ILOCK_EXCL);
    if !xfs_inode_hasattr(ip) { return 0; }
    if (*ip).i_af.if_format == XFS_DINODE_FMT_LOCAL {
        return xchk_xattr_walk_sf(sc, ip, attr_fn, priv_);
    }
    let error = xfs_iread_extents((*sc).tp, ip, XFS_ATTR_FORK);
    if error != 0 { return error; }
    if xfs_attr_is_leaf(ip) { return xchk_xattr_walk_leaf(sc, ip, attr_fn, priv_); }
    xchk_xattr_walk_node(sc, ip, attr_fn, leaf_fn, priv_)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
