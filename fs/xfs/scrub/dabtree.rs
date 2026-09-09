// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Directory/Attribute Btree.  C headers and external XFS symbols are supplied
 * by the surrounding translation unit. */

pub unsafe fn xchk_da_process_error(ds: *mut xchk_da_btree, level: i32, error: *mut i32) -> bool {
    let sc = (*ds).sc;
    if *error == 0 { return true; }
    match *error {
        -EDEADLOCK | -ECHRNG => {
            trace_xchk_deadlock_retry((*sc).ip, (*sc).sm, *error);
        }
        -EFSBADCRC | -EFSCORRUPTED | -EIO | -ENODATA => {
            (*(*sc).sm).sm_flags |= XFS_SCRUB_OFLAG_CORRUPT;
            *error = 0;
            trace_xchk_file_op_error(sc, (*ds).dargs.whichfork,
                xfs_dir2_da_to_db((*ds).dargs.geo,
                    (*ds).state.path.blk[level as usize].blkno), *error,
                __return_address);
        }
        _ => {
            trace_xchk_file_op_error(sc, (*ds).dargs.whichfork,
                xfs_dir2_da_to_db((*ds).dargs.geo,
                    (*ds).state.path.blk[level as usize].blkno), *error,
                __return_address);
        }
    }
    false
}

pub unsafe fn xchk_da_set_corrupt(ds: *mut xchk_da_btree, level: i32) {
    let sc = (*ds).sc;
    (*(*sc).sm).sm_flags |= XFS_SCRUB_OFLAG_CORRUPT;
    trace_xchk_fblock_error(sc, (*ds).dargs.whichfork,
        xfs_dir2_da_to_db((*ds).dargs.geo,
            (*ds).state.path.blk[level as usize].blkno), __return_address);
}

pub unsafe fn xchk_da_set_preen(ds: *mut xchk_da_btree, level: i32) {
    let sc = (*ds).sc;
    (*(*sc).sm).sm_flags |= XFS_SCRUB_OFLAG_PREEN;
    trace_xchk_fblock_preen(sc, (*ds).dargs.whichfork,
        xfs_dir2_da_to_db((*ds).dargs.geo,
            (*ds).state.path.blk[level as usize].blkno), __return_address);
}

unsafe fn xchk_da_btree_node_entry(ds: *mut xchk_da_btree, level: i32) -> *mut xfs_da_node_entry {
    let blk = &mut (*ds).state.path.blk[level as usize];
    ASSERT((*blk).magic == XFS_DA_NODE_MAGIC);
    let mut hdr: xfs_da3_icnode_hdr = core::mem::zeroed();
    xfs_da3_node_hdr_from_disk((*ds).sc.mp, &mut hdr, (*blk).bp.b_addr);
    hdr.btree.add((*blk).index as usize)
}

pub unsafe fn xchk_da_btree_hash(ds: *mut xchk_da_btree, level: i32, hashp: *mut __be32) -> i32 {
    let hash = be32_to_cpu(*hashp);
    if hash < (*ds).hashes[level as usize] { xchk_da_set_corrupt(ds, level); }
    (*ds).hashes[level as usize] = hash;
    if level == 0 { return 0; }
    let entry = xchk_da_btree_node_entry(ds, level - 1);
    if be32_to_cpu((*entry).hashval) < hash { xchk_da_set_corrupt(ds, level); }
    0
}

unsafe fn xchk_da_btree_ptr_ok(ds: *mut xchk_da_btree, level: i32, blkno: xfs_dablk_t) -> bool {
    if blkno < (*ds).lowest || ((*ds).highest != 0 && blkno >= (*ds).highest) {
        xchk_da_set_corrupt(ds, level); return false;
    }
    true
}

unsafe fn xchk_da_btree_read_verify(bp: *mut xfs_buf) {
    let info = (*bp).b_addr as *mut xfs_da_blkinfo;
    match be16_to_cpu((*info).magic) {
        XFS_DIR2_LEAF1_MAGIC | XFS_DIR3_LEAF1_MAGIC => { (*bp).b_ops = &xfs_dir3_leaf1_buf_ops; ((*bp).b_ops).verify_read(bp); }
        _ => { (*bp).b_ops = &xfs_da3_node_buf_ops; ((*bp).b_ops).verify_read(bp); }
    }
}
unsafe fn xchk_da_btree_write_verify(bp: *mut xfs_buf) {
    let info = (*bp).b_addr as *mut xfs_da_blkinfo;
    match be16_to_cpu((*info).magic) {
        XFS_DIR2_LEAF1_MAGIC | XFS_DIR3_LEAF1_MAGIC => { (*bp).b_ops = &xfs_dir3_leaf1_buf_ops; ((*bp).b_ops).verify_write(bp); }
        _ => { (*bp).b_ops = &xfs_da3_node_buf_ops; ((*bp).b_ops).verify_write(bp); }
    }
}
unsafe fn xchk_da_btree_verify(bp: *mut xfs_buf) -> *mut core::ffi::c_void {
    let info = (*bp).b_addr as *mut xfs_da_blkinfo;
    match be16_to_cpu((*info).magic) {
        XFS_DIR2_LEAF1_MAGIC | XFS_DIR3_LEAF1_MAGIC => { (*bp).b_ops = &xfs_dir3_leaf1_buf_ops; ((*bp).b_ops).verify_struct(bp) }
        _ => { (*bp).b_ops = &xfs_da3_node_buf_ops; ((*bp).b_ops).verify_struct(bp) }
    }
}
static xchk_da_btree_buf_ops: xfs_buf_ops = xfs_buf_ops { name: "xchk_da_btree", verify_read: xchk_da_btree_read_verify, verify_write: xchk_da_btree_write_verify, verify_struct: xchk_da_btree_verify };

unsafe fn xchk_da_btree_block_check_sibling(ds: *mut xchk_da_btree, level: i32, direction: i32, sibling: xfs_dablk_t) -> i32 {
    let path = &mut (*ds).state.path;
    let altpath = &mut (*ds).state.altpath;
    core::ptr::copy_nonoverlapping(path, altpath, 1);
    let mut retval = 0;
    let mut error;
    if sibling == 0 {
        error = xfs_da3_path_shift((*ds).state, altpath, direction, false, &mut retval);
        if error == 0 && retval == 0 { xchk_da_set_corrupt(ds, level); }
        error = 0;
    } else {
        error = xfs_da3_path_shift((*ds).state, altpath, direction, false, &mut retval);
        if !xchk_da_process_error(ds, level, &mut error) { return error; }
        if retval != 0 { xchk_da_set_corrupt(ds, level); return error; }
        if !altpath.blk[level as usize].bp.is_null() { xchk_buffer_recheck((*ds).sc, altpath.blk[level as usize].bp); }
        if altpath.blk[level as usize].blkno != sibling { xchk_da_set_corrupt(ds, level); }
    }
    for plevel in 0..altpath.active as usize {
        let abp = altpath.blk[plevel].bp;
        if abp.is_null() || (plevel < path.active as usize && abp == path.blk[plevel].bp) { continue; }
        xfs_trans_brelse((*ds).dargs.trans, abp); altpath.blk[plevel].bp = core::ptr::null_mut();
    }
    error
}

unsafe fn xchk_da_btree_block_check_siblings(ds: *mut xchk_da_btree, level: i32, hdr: *mut xfs_da_blkinfo) -> i32 {
    let forw = be32_to_cpu((*hdr).forw); let back = be32_to_cpu((*hdr).back);
    if level == 0 { if forw != 0 || back != 0 { xchk_da_set_corrupt(ds, level); } return 0; }
    let mut error = xchk_da_btree_block_check_sibling(ds, level, 0, back);
    if error == 0 { error = xchk_da_btree_block_check_sibling(ds, level, 1, forw); }
    core::ptr::write_bytes(&mut (*ds).state.altpath, 0, 1); error
}

unsafe fn xchk_da_btree_block(ds: *mut xchk_da_btree, level: i32, blkno: xfs_dablk_t) -> i32 {
    let blk = &mut (*ds).state.path.blk[level as usize];
    (*ds).state.path.active = level + 1;
    if !blk.bp.is_null() { xfs_trans_brelse((*ds).dargs.trans, blk.bp); blk.bp = core::ptr::null_mut(); }
    blk.blkno = blkno;
    if !xchk_da_btree_ptr_ok(ds, level, blkno) { blk.blkno = 0; return 0; }
    let mut error = xfs_da_read_buf((*ds).dargs.trans, (*ds).dargs.dp, blk.blkno, XFS_DABUF_MAP_HOLE_OK, &mut blk.bp, (*ds).dargs.whichfork, &xchk_da_btree_buf_ops);
    if !xchk_da_process_error(ds, level, &mut error) { blk.blkno = 0; return error; }
    if !blk.bp.is_null() { xchk_buffer_recheck((*ds).sc, blk.bp); }
    if (*ds).dargs.whichfork == XFS_DATA_FORK && level == 0 && blk.bp.is_null() { blk.blkno = 0; return error; }
    if blk.bp.is_null() { xchk_da_set_corrupt(ds, level); blk.blkno = 0; return error; }
    let hdr3 = blk.bp.b_addr as *mut xfs_da3_blkinfo;
    blk.magic = be16_to_cpu((*hdr3).hdr.magic); let pmaxrecs = &mut (*ds).maxrecs[level as usize];
    if xfs_has_crc((*ds).sc.mp) && (*hdr3).hdr.pad != 0 { xchk_da_set_corrupt(ds, level); }
    if xfs_has_crc((*ds).dargs.dp.i_mount) && be64_to_cpu((*hdr3).owner) != I_INO((*ds).dargs.dp) { xchk_da_set_corrupt(ds, level); }
    error = xchk_da_btree_block_check_siblings(ds, level, &mut (*hdr3).hdr); if error != 0 { return error; }
    match blk.magic {
        XFS_ATTR_LEAF_MAGIC | XFS_ATTR3_LEAF_MAGIC => { xfs_trans_buf_set_type((*ds).dargs.trans, blk.bp, XFS_BLFT_ATTR_LEAF_BUF); blk.magic = XFS_ATTR_LEAF_MAGIC; blk.hashval = xfs_attr_leaf_lasthash(blk.bp, pmaxrecs); if (*ds).tree_level != 0 { xchk_da_set_corrupt(ds, level); } }
        XFS_DIR2_LEAFN_MAGIC | XFS_DIR3_LEAFN_MAGIC => { xfs_trans_buf_set_type((*ds).dargs.trans, blk.bp, XFS_BLFT_DIR_LEAFN_BUF); blk.magic = XFS_DIR2_LEAFN_MAGIC; blk.hashval = xfs_dir2_leaf_lasthash((*ds).dargs.dp, blk.bp, pmaxrecs); if (*ds).tree_level != 0 { xchk_da_set_corrupt(ds, level); } }
        XFS_DIR2_LEAF1_MAGIC | XFS_DIR3_LEAF1_MAGIC => { xfs_trans_buf_set_type((*ds).dargs.trans, blk.bp, XFS_BLFT_DIR_LEAF1_BUF); blk.magic = XFS_DIR2_LEAF1_MAGIC; blk.hashval = xfs_dir2_leaf_lasthash((*ds).dargs.dp, blk.bp, pmaxrecs); if (*ds).tree_level != 0 { xchk_da_set_corrupt(ds, level); } }
        XFS_DA_NODE_MAGIC | XFS_DA3_NODE_MAGIC => {
            xfs_trans_buf_set_type((*ds).dargs.trans, blk.bp, XFS_BLFT_DA_NODE_BUF); blk.magic = XFS_DA_NODE_MAGIC;
            let mut nodehdr: xfs_da3_icnode_hdr = core::mem::zeroed(); xfs_da3_node_hdr_from_disk((*ds).dargs.dp.i_mount, &mut nodehdr, blk.bp.b_addr); *pmaxrecs = nodehdr.count; blk.hashval = be32_to_cpu((*nodehdr.btree.add((*pmaxrecs - 1) as usize)).hashval);
            if level == 0 { if nodehdr.level >= XFS_DA_NODE_MAXDEPTH { xchk_da_set_corrupt(ds, level); } (*ds).tree_level = nodehdr.level; } else if (*ds).tree_level != nodehdr.level { xchk_da_set_corrupt(ds, level); }
            if xfs_has_crc((*ds).dargs.dp.i_mount) && (*(blk.bp.b_addr as *mut xfs_da3_node_hdr)).__pad32 != 0 { xchk_da_set_preen(ds, level); }
        }
        _ => { xchk_da_set_corrupt(ds, level); }
    }
    if xfs_da3_header_check(blk.bp, (*ds).dargs.owner) != 0 { xchk_da_set_corrupt(ds, level); }
    if level > 0 { let key = xchk_da_btree_node_entry(ds, level - 1); if be32_to_cpu((*key).hashval) != blk.hashval { xchk_da_set_corrupt(ds, level); } }
    error
}

pub unsafe fn xchk_da_btree(sc: *mut xfs_scrub, whichfork: i32, scrub_fn: xchk_da_btree_rec_fn, private: *mut core::ffi::c_void) -> i32 {
    if !xfs_ifork_has_extents(xfs_ifork_ptr((*sc).ip, whichfork)) { return 0; }
    let ds = kzalloc_obj::<xchk_da_btree>(XCHK_GFP_FLAGS); if ds.is_null() { return -ENOMEM; }
    (*ds).dargs.dp = (*sc).ip; (*ds).dargs.whichfork = whichfork; (*ds).dargs.trans = (*sc).tp; (*ds).dargs.op_flags = XFS_DA_OP_OKNOENT; (*ds).dargs.owner = I_INO((*sc).ip); (*ds).state = xfs_da_state_alloc(&mut (*ds).dargs); (*ds).sc = sc; (*ds).private = private;
    if whichfork == XFS_ATTR_FORK { (*ds).dargs.geo = (*sc).mp.m_attr_geo; (*ds).lowest = 0; (*ds).highest = 0; } else { (*ds).dargs.geo = (*sc).mp.m_dir_geo; (*ds).lowest = (*ds).dargs.geo.leafblk; (*ds).highest = (*ds).dargs.geo.freeblk; }
    let mut level = 0; let mut error = xchk_da_btree_block(ds, 0, (*ds).lowest); let blks = (*ds).state.path.blk.as_mut_ptr();
    if error == 0 && !(*blks).bp.is_null() { (*blks).index = 0; while level >= 0 && level < XFS_DA_NODE_MAXDEPTH as i32 {
        let b = &mut *blks.add(level as usize);
        if b.magic != XFS_DA_NODE_MAGIC { if b.index >= (*ds).maxrecs[level as usize] { if level > 0 { (*blks.add((level-1) as usize)).index += 1; } (*ds).tree_level += 1; level -= 1; continue; } error = scrub_fn(ds, level); if error != 0 || xchk_should_terminate(sc, &mut error) || ((*(*sc).sm).sm_flags & XFS_SCRUB_OFLAG_CORRUPT) != 0 { break; } b.index += 1; continue; }
        if b.index >= (*ds).maxrecs[level as usize] { if level > 0 { (*blks.add((level-1) as usize)).index += 1; } (*ds).tree_level += 1; level -= 1; continue; }
        let key = xchk_da_btree_node_entry(ds, level); error = xchk_da_btree_hash(ds, level, &mut (*key).hashval); if error != 0 { break; }
        let next = be32_to_cpu((*key).before); level += 1; if level >= XFS_DA_NODE_MAXDEPTH as i32 { xchk_da_set_corrupt(ds, level-1); break; } (*ds).tree_level -= 1; error = xchk_da_btree_block(ds, level, next); if error != 0 || (*blks.add(level as usize)).bp.is_null() { break; } (*blks.add(level as usize)).index = 0;
    }}
    for i in 0..XFS_DA_NODE_MAXDEPTH as usize { let bp = (*blks.add(i)).bp; if !bp.is_null() { xfs_trans_brelse((*sc).tp, bp); (*blks.add(i)).bp = core::ptr::null_mut(); } }
    xfs_da_state_free((*ds).state); kfree(ds); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
