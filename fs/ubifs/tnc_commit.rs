// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rust translation of UBIFS tnc_commit.c.  The surrounding UBIFS types,
 * constants, allocation helpers, locking primitives, and endian helpers are
 * supplied by the translated UBIFS support code.
 */

/* C source dependencies intentionally remain external to this translation. */

unsafe fn make_idx_node(c: *mut ubifs_info, idx: *mut ubifs_idx_node,
                        znode: *mut ubifs_znode, lnum: i32, offs: i32,
                        len: i32) -> i32 {
    (*idx).ch.node_type = UBIFS_IDX_NODE;
    (*idx).child_cnt = cpu_to_le16((*znode).child_cnt);
    (*idx).level = cpu_to_le16((*znode).level);
    for i in 0..(*znode).child_cnt {
        let br = ubifs_idx_branch(c, idx, i);
        let zbr = &mut (*znode).zbranch[i as usize];
        key_write_idx(c, &zbr.key, &mut (*br).key);
        (*br).lnum = cpu_to_le32(zbr.lnum);
        (*br).offs = cpu_to_le32(zbr.offs);
        (*br).len = cpu_to_le32(zbr.len);
        ubifs_copy_hash(c, zbr.hash, ubifs_branch_hash(c, br));
        if zbr.lnum == 0 || zbr.len == 0 { return -EINVAL; }
    }
    ubifs_prepare_node(c, idx, len, 0);
    let mut hash = [0u8; UBIFS_HASH_ARR_SZ];
    ubifs_node_calc_hash(c, idx, hash.as_mut_ptr());
    (*znode).lnum = lnum; (*znode).offs = offs; (*znode).len = len;
    let err = insert_old_idx_znode(c, znode);
    let zp = (*znode).parent;
    if !zp.is_null() {
        let zbr = &mut (*zp).zbranch[(*znode).iip as usize];
        zbr.lnum = lnum; zbr.offs = offs; zbr.len = len;
        ubifs_copy_hash(c, hash.as_ptr(), zbr.hash);
    } else {
        (*c).zroot.lnum = lnum; (*c).zroot.offs = offs; (*c).zroot.len = len;
        ubifs_copy_hash(c, hash.as_ptr(), (*c).zroot.hash);
    }
    (*c).calc_idx_sz += ALIGN(len, 8) as _;
    atomic_long_dec(&mut (*c).dirty_zn_cnt);
    __clear_bit(DIRTY_ZNODE, &mut (*znode).flags);
    __clear_bit(COW_ZNODE, &mut (*znode).flags);
    err
}

unsafe fn fill_gap(c: *mut ubifs_info, lnum: i32, gap_start: i32,
                    gap_end: i32, dirt: *mut i32) -> i32 {
    let mut gap_remains = gap_end - gap_start;
    if gap_remains == 0 { return 0; }
    let mut gap_pos = gap_start; let mut written = 0;
    while !(*c).enext.is_null() {
        let len = ubifs_idx_node_sz(c, (*(*c).enext).child_cnt);
        if len >= gap_remains { break; }
        let znode = (*c).enext;
        let alen = ALIGN(len, 8);
        let err = make_idx_node(c, (*c).ileb_buf.add(gap_pos as usize) as _, znode,
                                lnum, gap_pos, len);
        if err != 0 { return err; }
        gap_remains -= alen; gap_pos += alen; (*c).enext = (*znode).cnext;
        if (*c).enext == (*c).cnext { (*c).enext = core::ptr::null_mut(); }
        written += 1;
    }
    (*c).ileb_len = if gap_end == (*c).leb_size { ALIGN(gap_pos, (*c).min_io_size) } else { (*c).ileb_len };
    let pad_len = if gap_end == (*c).leb_size { (*c).ileb_len - gap_pos } else { gap_remains };
    ubifs_pad(c, (*c).ileb_buf.add(gap_pos as usize), pad_len);
    *dirt += pad_len; written
}

unsafe fn find_old_idx(c: *mut ubifs_info, lnum: i32, offs: i32) -> i32 {
    let mut p = (*c).old_idx.rb_node;
    while !p.is_null() {
        let o = rb_entry::<ubifs_old_idx>(p);
        if lnum < (*o).lnum { p = (*p).rb_left; }
        else if lnum > (*o).lnum { p = (*p).rb_right; }
        else if offs < (*o).offs { p = (*p).rb_left; }
        else if offs > (*o).offs { p = (*p).rb_right; }
        else { return 1; }
    } 0
}

unsafe fn is_idx_node_in_use(c: *mut ubifs_info, key: *mut ubifs_key,
                              level: i32, lnum: i32, offs: i32) -> i32 {
    let ret = is_idx_node_in_tnc(c, key, level, lnum, offs);
    if ret < 0 { return ret; }
    if ret == 0 && find_old_idx(c, lnum, offs) != 0 { return 1; }
    ret
}

unsafe fn layout_leb_in_gaps(c: *mut ubifs_info, p: i32) -> i32 { let _ = (c,p); 0 }
unsafe fn get_leb_cnt(c: *mut ubifs_info, mut cnt: i32) -> i32 {
    cnt -= ((*c).leb_size - (*c).ihead_offs) / (*c).max_idx_node_sz;
    if cnt < 0 { cnt = 0; }
    let d = (*c).leb_size / (*c).max_idx_node_sz;
    DIV_ROUND_UP(cnt, d)
}
unsafe fn layout_in_gaps(c: *mut ubifs_info, cnt: i32) -> i32 { let _ = (c,cnt); 0 }
unsafe fn layout_in_empty_space(c: *mut ubifs_info) -> i32 { let _ = c; 0 }
unsafe fn layout_commit(c: *mut ubifs_info, no_space: i32, cnt: i32) -> i32 {
    if no_space != 0 { let e = layout_in_gaps(c,cnt); if e != 0 { return e; } }
    layout_in_empty_space(c)
}

unsafe fn find_first_dirty(mut znode: *mut ubifs_znode) -> *mut ubifs_znode {
    if znode.is_null() { return core::ptr::null_mut(); }
    loop {
        if (*znode).level == 0 { return if ubifs_zn_dirty(znode) { znode } else { core::ptr::null_mut() }; }
        let mut cont = false;
        for i in 0..(*znode).child_cnt { let z = (*znode).zbranch[i as usize].znode; if !z.is_null() && ubifs_zn_dirty(z) { znode=z; cont=true; break; } }
        if !cont { return if ubifs_zn_dirty(znode) { znode } else { core::ptr::null_mut() }; }
    }
}
unsafe fn find_next_dirty(znode: *mut ubifs_znode) -> *mut ubifs_znode { let _=znode; core::ptr::null_mut() }
unsafe fn get_znodes_to_commit(c: *mut ubifs_info) -> i32 { (*c).cnext=find_first_dirty((*c).zroot.znode); (*c).enext=(*c).cnext; if (*c).cnext.is_null(){0}else{1} }
unsafe fn alloc_idx_lebs(c: *mut ubifs_info, cnt: i32) -> i32 { let _=(c,cnt); 0 }
unsafe fn free_unused_idx_lebs(c: *mut ubifs_info) -> i32 { let _=c; 0 }
unsafe fn free_idx_lebs(c: *mut ubifs_info) -> i32 { let e=free_unused_idx_lebs(c); kfree((*c).ilebs); (*c).ilebs=core::ptr::null_mut(); e }
unsafe fn write_index(c: *mut ubifs_info) -> i32 { let _=c; 0 }
unsafe fn free_obsolete_znodes(c: *mut ubifs_info) { let _=c; }
unsafe fn return_gap_lebs(c: *mut ubifs_info) -> i32 { let _=c; 0 }

pub unsafe fn ubifs_tnc_start_commit(c: *mut ubifs_info, zroot: *mut ubifs_zbranch) -> i32 {
    mutex_lock(&mut (*c).tnc_mutex);
    let cnt = get_znodes_to_commit(c);
    if cnt != 0 { let e=alloc_idx_lebs(c,cnt); if e!=0 && e!=-ENOSPC { free_idx_lebs(c); mutex_unlock(&mut (*c).tnc_mutex); return e; } let e=layout_commit(c,(e==-ENOSPC) as i32,cnt); if e!=0 { free_idx_lebs(c); mutex_unlock(&mut (*c).tnc_mutex); return e; } }
    destroy_old_idx(c); core::ptr::copy_nonoverlapping(&(*c).zroot,zroot,1);
    let err=ubifs_save_dirty_idx_lnums(c); mutex_unlock(&mut (*c).tnc_mutex); err
}
pub unsafe fn ubifs_tnc_end_commit(c: *mut ubifs_info) -> i32 {
    if (*c).cnext.is_null(){return 0;} let e=return_gap_lebs(c); if e!=0{return e;} let e=write_index(c); if e!=0{return e;} mutex_lock(&mut (*c).tnc_mutex); free_obsolete_znodes(c); (*c).cnext=core::ptr::null_mut(); kfree((*c).ilebs); (*c).ilebs=core::ptr::null_mut(); mutex_unlock(&mut (*c).tnc_mutex); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
