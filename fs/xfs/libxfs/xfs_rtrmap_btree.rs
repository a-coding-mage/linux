// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of xfs_rtrmap_btree.c.  Types and helpers are
 * supplied by the surrounding XFS Rust bindings. */

static mut XFS_RTRMAPBT_CUR_CACHE: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xfs_rtrmapbt_dup_cursor(cur: *mut xfs_btree_cur) -> *mut xfs_btree_cur {
    xfs_rtrmapbt_init_cursor((*cur).bc_tp, to_rtg((*cur).bc_group))
}
unsafe fn xfs_rtrmapbt_get_minrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    if level == (*cur).bc_nlevels - 1 { let ifp = xfs_btree_ifork_ptr(cur); return xfs_rtrmapbt_maxrecs((*cur).bc_mp, (*ifp).if_broot_bytes, level == 0) / 2; }
    (*(*cur).bc_mp).m_rtrmap_mnr[(level != 0) as usize]
}
unsafe fn xfs_rtrmapbt_get_maxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 {
    if level == (*cur).bc_nlevels - 1 { let ifp = xfs_btree_ifork_ptr(cur); return xfs_rtrmapbt_maxrecs((*cur).bc_mp, (*ifp).if_broot_bytes, level == 0); }
    (*(*cur).bc_mp).m_rtrmap_mxr[(level != 0) as usize]
}

pub unsafe fn xfs_rtrmapbt_droot_maxrecs(mut blocklen: u32, leaf: bool) -> u32 {
    blocklen -= core::mem::size_of::<xfs_rtrmap_root>() as u32;
    if leaf { blocklen / core::mem::size_of::<xfs_rmap_rec>() as u32 } else { blocklen / (2 * core::mem::size_of::<xfs_rmap_key>() as u32 + core::mem::size_of::<xfs_rtrmap_ptr_t>() as u32) }
}
unsafe fn xfs_rtrmapbt_get_dmaxrecs(cur: *mut xfs_btree_cur, level: i32) -> i32 { if level != (*cur).bc_nlevels - 1 { (*(*cur).bc_mp).m_rtrmap_mxr[(level != 0) as usize] } else { xfs_rtrmapbt_droot_maxrecs((*cur).bc_ino.forksize, level == 0) as i32 } }
#[inline] unsafe fn ondisk_rec_offset_to_key(rec: *const xfs_btree_rec) -> __be64 { (*rec).rmap.rm_offset & !cpu_to_be64(XFS_RMAP_OFF_UNWRITTEN) }
unsafe fn xfs_rtrmapbt_init_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) { (*key).rmap.rm_startblock=(*rec).rmap.rm_startblock; (*key).rmap.rm_owner=(*rec).rmap.rm_owner; (*key).rmap.rm_offset=ondisk_rec_offset_to_key(rec); }
unsafe fn xfs_rtrmapbt_init_high_key_from_rec(key: *mut xfs_btree_key, rec: *const xfs_btree_rec) { let adj=be32_to_cpu((*rec).rmap.rm_blockcount)-1; (*key).rmap.rm_startblock=(*rec).rmap.rm_startblock; be32_add_cpu(&mut (*key).rmap.rm_startblock,adj); (*key).rmap.rm_owner=(*rec).rmap.rm_owner; (*key).rmap.rm_offset=ondisk_rec_offset_to_key(rec); if XFS_RMAP_NON_INODE_OWNER(be64_to_cpu((*rec).rmap.rm_owner)) || XFS_RMAP_IS_BMBT_BLOCK(be64_to_cpu((*rec).rmap.rm_offset)){return;} let mut off=be64_to_cpu((*key).rmap.rm_offset); off=(XFS_RMAP_OFF(off)+adj as u64)|(off&!XFS_RMAP_OFF_MASK); (*key).rmap.rm_offset=cpu_to_be64(off); }
unsafe fn xfs_rtrmapbt_init_rec_from_cur(cur:*mut xfs_btree_cur, rec:*mut xfs_btree_rec){(*rec).rmap.rm_startblock=cpu_to_be32((*cur).bc_rec.r.rm_startblock);(*rec).rmap.rm_blockcount=cpu_to_be32((*cur).bc_rec.r.rm_blockcount);(*rec).rmap.rm_owner=cpu_to_be64((*cur).bc_rec.r.rm_owner);(*rec).rmap.rm_offset=cpu_to_be64(xfs_rmap_irec_offset_pack(&(*cur).bc_rec.r));}
unsafe fn xfs_rtrmapbt_init_ptr_from_cur(_cur:*mut xfs_btree_cur,ptr:*mut xfs_btree_ptr){(*ptr).l=0;}
#[inline] unsafe fn offset_keymask(offset:u64)->u64{offset & !XFS_RMAP_OFF_UNWRITTEN}
unsafe fn xfs_rtrmapbt_cmp_key_with_cur(cur:*mut xfs_btree_cur,key:*const xfs_btree_key)->i32{let r=&(*cur).bc_rec.r;let k=&(*key).rmap; cmp_int(be32_to_cpu(k.rm_startblock),r.rm_startblock).or_else(||cmp_int(be64_to_cpu(k.rm_owner),r.rm_owner)).or_else(||cmp_int(offset_keymask(be64_to_cpu(k.rm_offset)),offset_keymask(xfs_rmap_irec_offset_pack(r)))).unwrap_or(0)}
unsafe fn xfs_rtrmapbt_cmp_two_keys(_cur:*mut xfs_btree_cur,k1:*const xfs_btree_key,k2:*const xfs_btree_key,mask:*const xfs_btree_key)->i32{let a=&(*k1).rmap;let b=&(*k2).rmap; if !mask.is_null(){ASSERT((*mask).rmap.rm_startblock!=0);} let mut d=cmp_int(be32_to_cpu(a.rm_startblock),be32_to_cpu(b.rm_startblock));if d!=0{return d;}if mask.is_null()||(*mask).rmap.rm_owner!=0{d=cmp_int(be64_to_cpu(a.rm_owner),be64_to_cpu(b.rm_owner));if d!=0{return d;}}if mask.is_null()||(*mask).rmap.rm_offset!=0{d=cmp_int(offset_keymask(be64_to_cpu(a.rm_offset)),offset_keymask(be64_to_cpu(b.rm_offset)));}d}

unsafe fn xfs_rtrmapbt_keys_inorder(_c:*mut xfs_btree_cur,a:*const xfs_btree_key,b:*const xfs_btree_key)->i32{let x=be32_to_cpu((*a).rmap.rm_startblock);let y=be32_to_cpu((*b).rmap.rm_startblock);if x!=y{return (x<y) as i32;}let x=be64_to_cpu((*a).rmap.rm_owner);let y=be64_to_cpu((*b).rmap.rm_owner);if x!=y{return (x<y) as i32;} (offset_keymask(be64_to_cpu((*a).rmap.rm_offset))<=offset_keymask(be64_to_cpu((*b).rmap.rm_offset))) as i32}
unsafe fn xfs_rtrmapbt_recs_inorder(_c:*mut xfs_btree_cur,a:*const xfs_btree_rec,b:*const xfs_btree_rec)->i32{let x=be32_to_cpu((*a).rmap.rm_startblock);let y=be32_to_cpu((*b).rmap.rm_startblock);if x!=y{return (x<y) as i32;}let x=be64_to_cpu((*a).rmap.rm_owner);let y=be64_to_cpu((*b).rmap.rm_owner);if x!=y{return (x<y) as i32;} (offset_keymask(be64_to_cpu((*a).rmap.rm_offset))<=offset_keymask(be64_to_cpu((*b).rmap.rm_offset))) as i32}
unsafe fn xfs_rtrmapbt_keys_contiguous(_c:*mut xfs_btree_cur,a:*const xfs_btree_key,b:*const xfs_btree_key,mask:*const xfs_btree_key)->enum_xbtree_key_contig{if !mask.is_null(){ASSERT((*mask).rmap.rm_startblock!=0);ASSERT((*mask).rmap.rm_owner==0&&(*mask).rmap.rm_offset==0);}xbtree_key_contig(be32_to_cpu((*a).rmap.rm_startblock),be32_to_cpu((*b).rmap.rm_startblock))}

pub unsafe fn xfs_rtrmapbt_block_maxrecs(blocklen:u32,leaf:bool)->u32{if leaf{blocklen/core::mem::size_of::<xfs_rmap_rec>() as u32}else{blocklen/(2*core::mem::size_of::<xfs_rmap_key>() as u32+core::mem::size_of::<xfs_rtrmap_ptr_t>() as u32)}}
pub unsafe fn xfs_rtrmapbt_maxrecs(_mp:*mut xfs_mount,mut blocklen:u32,leaf:bool)->u32{blocklen-=XFS_RTRMAP_BLOCK_LEN;xfs_rtrmapbt_block_maxrecs(blocklen,leaf)}
pub unsafe fn xfs_rtrmapbt_maxlevels_ondisk()->u32{let bl=XFS_MIN_CRC_BLOCKSIZE-XFS_BTREE_LBLOCK_CRC_LEN;let min=[xfs_rtrmapbt_block_maxrecs(bl,true)/2,xfs_rtrmapbt_block_maxrecs(bl,false)/2];xfs_btree_space_to_height(min,(-1i32 as u32 as u64)*XFS_MAX_CRC_AG_BLOCKS as u64) }
pub unsafe fn xfs_rtrmapbt_calc_size(mp:*mut xfs_mount,len:u64)->u64{xfs_btree_calc_size((*mp).m_rtrmap_mnr,len)}
unsafe fn xfs_rtrmapbt_max_size(mp:*mut xfs_mount,blocks:xfs_rtblock_t)->u64{if (*mp).m_rtrmap_mxr[0]==0{0}else{xfs_rtrmapbt_calc_size(mp,blocks)}}
pub unsafe fn xfs_rtrmapbt_calc_reserves(mp:*mut xfs_mount)->xfs_filblks_t{let b=(*mp).m_groups[XG_TYPE_RTG].blocks;if !xfs_has_rtrmapbt(mp){0}else{max_t(b/100,xfs_rtrmapbt_max_size(mp,b))}}

pub unsafe fn xfs_rtrmapbt_compute_maxlevels(mp:*mut xfs_mount){if !xfs_has_rtrmapbt(mp){(*mp).m_rtrmap_maxlevels=0;return;}let d=xfs_btree_space_to_height((*mp).m_rtrmap_mnr,(*mp).m_sb.sb_dblocks);if xfs_has_rtreflink(mp){(*mp).m_rtrmap_maxlevels=d+1;}else{let r=xfs_btree_compute_maxlevels((*mp).m_rtrmap_mnr,(*mp).m_groups[XG_TYPE_RTG].blocks);(*mp).m_rtrmap_maxlevels=min(d,r)+1;}}

// Verifier, cursor, staging, in-memory-tree, disk conversion, and cache routines
// retain their C ABI and are declared against the surrounding XFS bindings.
extern "C" { pub fn xfs_rtrmapbt_init_cursor(tp:*mut xfs_trans,rtg:*mut xfs_rtgroup)->*mut xfs_btree_cur; pub fn xfs_iformat_rtrmap(ip:*mut xfs_inode,dip:*mut xfs_dinode)->i32; pub fn xfs_rtrmapbt_to_disk(mp:*mut xfs_mount,rblock:*mut xfs_btree_block,rblocklen:u32,dblock:*mut xfs_rtrmap_root,dblocklen:u32); pub fn xfs_iflush_rtrmap(ip:*mut xfs_inode,dip:*mut xfs_dinode); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
