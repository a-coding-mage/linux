// SPDX-License-Identifier: GPL-2.0
/* Literal low-level translation of xfs_bmap_btree.c. */

static mut xfs_bmbt_cur_cache: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn xfs_bmbt_init_block(ip: *mut xfs_inode, buf: *mut xfs_btree_block,
    bp: *mut xfs_buf, level: u16, numrecs: u16) {
    if !bp.is_null() { xfs_btree_init_buf((*ip).i_mount, bp, &xfs_bmbt_ops, level, numrecs, I_INO(ip)); }
    else { xfs_btree_init_block((*ip).i_mount, buf, &xfs_bmbt_ops, level, numrecs, I_INO(ip)); }
}

pub unsafe fn xfs_bmdr_to_bmbt(ip: *mut xfs_inode, dblock: *mut xfs_bmdr_block_t,
    dblocklen: i32, rblock: *mut xfs_btree_block, rblocklen: i32) {
    let mp = (*ip).i_mount; xfs_bmbt_init_block(ip, rblock, core::ptr::null_mut(), 0, 0);
    (*rblock).bb_level = (*dblock).bb_level; ASSERT(be16_to_cpu((*rblock).bb_level) > 0);
    (*rblock).bb_numrecs = (*dblock).bb_numrecs;
    let mut dmxr = xfs_bmdr_maxrecs(dblocklen, 0);
    let fkp = xfs_bmdr_key_addr(dblock, 1); let tkp = xfs_bmbt_key_addr(mp, rblock, 1);
    let fpp = xfs_bmdr_ptr_addr(dblock, 1, dmxr); let tpp = xfs_bmap_broot_ptr_addr(mp, rblock, 1, rblocklen);
    dmxr = be16_to_cpu((*dblock).bb_numrecs) as i32;
    memcpy(tkp, fkp, (core::mem::size_of::<xfs_bmbt_key_t>() * dmxr as usize));
    memcpy(tpp, fpp, (core::mem::size_of::<u64>() * dmxr as usize));
}

pub unsafe fn xfs_bmbt_disk_get_all(rec: *const xfs_bmbt_rec, irec: *mut xfs_bmbt_irec) {
    let l0 = get_unaligned_be64(&(*rec).l0); let l1 = get_unaligned_be64(&(*rec).l1);
    (*irec).br_startoff = (l0 & xfs_mask64lo(64 - BMBT_EXNTFLAG_BITLEN)) >> 9;
    (*irec).br_startblock = ((l0 & xfs_mask64lo(9)) << 43) | (l1 >> 21);
    (*irec).br_blockcount = l1 & xfs_mask64lo(21);
    (*irec).br_state = if l0 >> (64 - BMBT_EXNTFLAG_BITLEN) != 0 { XFS_EXT_UNWRITTEN } else { XFS_EXT_NORM };
}
pub unsafe fn xfs_bmbt_disk_get_blockcount(r: *const xfs_bmbt_rec) -> xfs_filblks_t { (be64_to_cpu((*r).l1) & xfs_mask64lo(21)) as xfs_filblks_t }
pub unsafe fn xfs_bmbt_disk_get_startoff(r: *const xfs_bmbt_rec) -> xfs_fileoff_t { ((be64_to_cpu((*r).l0) as xfs_fileoff_t) & xfs_mask64lo(64 - BMBT_EXNTFLAG_BITLEN)) >> 9 }
pub unsafe fn xfs_bmbt_disk_set_all(r: *mut xfs_bmbt_rec, s: *mut xfs_bmbt_irec) {
    let extent_flag = ((*s).br_state != XFS_EXT_NORM) as u64;
    ASSERT((*s).br_state == XFS_EXT_NORM || (*s).br_state == XFS_EXT_UNWRITTEN);
    ASSERT((*s).br_startoff & xfs_mask64hi(64-BMBT_STARTOFF_BITLEN) == 0);
    ASSERT((*s).br_blockcount & xfs_mask64hi(64-BMBT_BLOCKCOUNT_BITLEN) == 0);
    ASSERT((*s).br_startblock & xfs_mask64hi(64-BMBT_STARTBLOCK_BITLEN) == 0);
    put_unaligned_be64((extent_flag << 63) | ((*s).br_startoff as u64 << 9) | ((*s).br_startblock as u64 >> 43), &mut (*r).l0);
    put_unaligned_be64(((*s).br_startblock as u64 << 21) | ((*s).br_blockcount as u64 & xfs_mask64lo(21)), &mut (*r).l1);
}

pub unsafe fn xfs_bmbt_to_bmdr(mp: *mut xfs_mount, rblock: *mut xfs_btree_block, rblocklen: i32, dblock: *mut xfs_bmdr_block_t, dblocklen: i32) {
    if xfs_has_crc(mp) { ASSERT((*rblock).bb_magic == cpu_to_be32(XFS_BMAP_CRC_MAGIC)); ASSERT(uuid_equal(&(*rblock).bb_u.l.bb_uuid, &(*mp).m_sb.sb_meta_uuid)); ASSERT((*rblock).bb_u.l.bb_blkno == cpu_to_be64(XFS_BUF_DADDR_NULL)); }
    else { ASSERT((*rblock).bb_magic == cpu_to_be32(XFS_BMAP_MAGIC)); }
    ASSERT((*rblock).bb_u.l.bb_leftsib == cpu_to_be64(NULLFSBLOCK)); ASSERT((*rblock).bb_u.l.bb_rightsib == cpu_to_be64(NULLFSBLOCK)); ASSERT((*rblock).bb_level != 0);
    (*dblock).bb_level = (*rblock).bb_level; (*dblock).bb_numrecs = (*rblock).bb_numrecs;
    let mut dmxr = xfs_bmdr_maxrecs(dblocklen, 0); let fkp = xfs_bmbt_key_addr(mp,rblock,1); let tkp=xfs_bmdr_key_addr(dblock,1); let fpp=xfs_bmap_broot_ptr_addr(mp,rblock,1,rblocklen); let tpp=xfs_bmdr_ptr_addr(dblock,1,dmxr); dmxr=be16_to_cpu((*dblock).bb_numrecs) as i32;
    memcpy(tkp,fkp,core::mem::size_of::<xfs_bmbt_key_t>()*dmxr as usize); memcpy(tpp,fpp,core::mem::size_of::<u64>()*dmxr as usize);
}

pub unsafe fn xfs_bmbt_get_maxrecs(cur:*mut xfs_btree_cur, level:i32)->i32 { if level == (*cur).bc_nlevels as i32-1 { let ifp=xfs_btree_ifork_ptr(cur); xfs_bmbt_maxrecs((*cur).bc_mp,(*ifp).if_broot_bytes,level==0) } else { (*(*cur).bc_mp).m_bmap_dmxr[(level!=0) as usize] } }
pub unsafe fn xfs_bmbt_get_minrecs(cur:*mut xfs_btree_cur, level:i32)->i32 { if level == (*cur).bc_nlevels as i32-1 { let ifp=xfs_btree_ifork_ptr(cur); xfs_bmbt_maxrecs((*cur).bc_mp,(*ifp).if_broot_bytes,level==0)/2 } else { (*(*cur).bc_mp).m_bmap_dmnr[(level!=0) as usize] } }
pub unsafe fn xfs_bmbt_maxrecs(mp:*mut xfs_mount, blocklen:u32, leaf:bool)->u32 { xfs_bmbt_block_maxrecs(blocklen-xfs_bmbt_block_len(mp),leaf) }
unsafe fn xfs_bmbt_block_maxrecs(blocklen:u32,leaf:bool)->u32 { if leaf { blocklen/core::mem::size_of::<xfs_bmbt_rec_t>() as u32 } else { blocklen/(core::mem::size_of::<xfs_bmbt_key_t>() as u32+core::mem::size_of::<xfs_bmbt_ptr_t>() as u32) } }

pub unsafe fn xfs_bmbt_calc_size(mp:*mut xfs_mount,len:u64)->u64 { xfs_btree_calc_size((*mp).m_bmap_dmnr,len) }
pub unsafe fn xfs_bmbt_maxlevels_ondisk()->u32 { let blocklen=core::cmp::min(XFS_MIN_BLOCKSIZE-XFS_BTREE_SBLOCK_LEN,XFS_MIN_CRC_BLOCKSIZE-XFS_BTREE_SBLOCK_CRC_LEN); let minrecs=[xfs_bmbt_block_maxrecs(blocklen,true)/2,xfs_bmbt_block_maxrecs(blocklen,false)/2]; xfs_btree_compute_maxlevels(minrecs,XFS_MAX_EXTCNT_DATA_FORK_LARGE)+1 }
pub unsafe fn xfs_bmdr_maxrecs(mut blocklen:i32,leaf:i32)->i32 { blocklen-=core::mem::size_of::<xfs_bmdr_block_t>() as i32; if leaf!=0 { blocklen/core::mem::size_of::<xfs_bmdr_rec_t>() as i32 } else { blocklen/(core::mem::size_of::<xfs_bmdr_key_t>() as i32+core::mem::size_of::<xfs_bmdr_ptr_t>() as i32) } }

unsafe fn xfs_bmbt_init_key_from_rec(key:*mut xfs_btree_key, rec:*const xfs_btree_rec) { (*key).bmbt.br_startoff=cpu_to_be64(xfs_bmbt_disk_get_startoff(&(*rec).bmbt)); }
unsafe fn xfs_bmbt_init_high_key_from_rec(key:*mut xfs_btree_key, rec:*const xfs_btree_rec) { (*key).bmbt.br_startoff=cpu_to_be64(xfs_bmbt_disk_get_startoff(&(*rec).bmbt)+xfs_bmbt_disk_get_blockcount(&(*rec).bmbt)-1); }
unsafe fn xfs_bmbt_init_rec_from_cur(cur:*mut xfs_btree_cur, rec:*mut xfs_btree_rec) { xfs_bmbt_disk_set_all(&mut (*rec).bmbt,&mut (*cur).bc_rec.b); }
unsafe fn xfs_bmbt_cmp_key_with_cur(cur:*mut xfs_btree_cur,key:*const xfs_btree_key)->i32 { cmp_int(be64_to_cpu((*key).bmbt.br_startoff),(*cur).bc_rec.b.br_startoff) }
unsafe fn xfs_bmbt_cmp_two_keys(_cur:*mut xfs_btree_cur,k1:*const xfs_btree_key,k2:*const xfs_btree_key,mask:*const xfs_btree_key)->i32 { ASSERT(mask.is_null() || (*mask).bmbt.br_startoff!=0); cmp_int(be64_to_cpu((*k1).bmbt.br_startoff),be64_to_cpu((*k2).bmbt.br_startoff)) }
unsafe fn xfs_bmbt_keys_inorder(_cur:*mut xfs_btree_cur,k1:*const xfs_btree_key,k2:*const xfs_btree_key)->i32 { (be64_to_cpu((*k1).bmbt.br_startoff)<be64_to_cpu((*k2).bmbt.br_startoff)) as i32 }
unsafe fn xfs_bmbt_recs_inorder(_cur:*mut xfs_btree_cur,r1:*const xfs_btree_rec,r2:*const xfs_btree_rec)->i32 { (xfs_bmbt_disk_get_startoff(&(*r1).bmbt)+xfs_bmbt_disk_get_blockcount(&(*r1).bmbt)<=xfs_bmbt_disk_get_startoff(&(*r2).bmbt)) as i32 }

pub unsafe fn xfs_bmbt_destroy_cur_cache() { kmem_cache_destroy(xfs_bmbt_cur_cache); xfs_bmbt_cur_cache=core::ptr::null_mut(); }
pub unsafe fn xfs_bmbt_init_cur_cache()->i32 { xfs_bmbt_cur_cache=kmem_cache_create("xfs_bmbt_cur",xfs_btree_cur_sizeof(xfs_bmbt_maxlevels_ondisk()),0,0,core::ptr::null_mut()); if xfs_bmbt_cur_cache.is_null() {-ENOMEM} else {0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
