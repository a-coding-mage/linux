/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from fs/f2fs/node.h. External kernel types and helpers are supplied elsewhere. */

pub const FREE_NID_PAGES: usize = 8;
pub const SHRINK_NID_BATCH_SIZE: usize = 8;
pub const DEF_RA_NID_PAGES: usize = 0;
pub const MAX_RA_NODE: usize = 128;
pub const DEF_RAM_THRESHOLD: usize = 1;
pub const DEF_DIRTY_NAT_RATIO_THRESHOLD: usize = 10;
pub const DEF_NAT_CACHE_THRESHOLD: usize = 100000;
pub const DEF_RF_NODE_BLOCKS: usize = 0;
pub const NAT_VEC_SIZE: usize = 32;
pub const LOCKED_PAGE: usize = 1;
pub const FILE_NOT_ALIGNED: usize = 1;

#[inline]
pub const fn start_nid(nid: nid_t) -> nid_t { (nid / NAT_ENTRY_PER_BLOCK) * NAT_ENTRY_PER_BLOCK }
#[inline]
pub const fn nat_block_offset(start_nid: nid_t) -> nid_t { start_nid / NAT_ENTRY_PER_BLOCK }
#[inline]
pub const fn max_free_nids() -> usize { NAT_ENTRY_PER_BLOCK * FREE_NID_PAGES }

pub const IS_CHECKPOINTED: usize = 0;
pub const HAS_FSYNCED_INODE: usize = 1;
pub const HAS_LAST_FSYNC: usize = 2;
pub const IS_DIRTY: usize = 3;
pub const IS_PREALLOC: usize = 4;

#[repr(C)]
pub struct node_info { pub nid: nid_t, pub ino: nid_t, pub blk_addr: block_t, pub version: u8, pub flag: u8 }
#[repr(C)]
pub struct nat_entry { pub list: list_head, pub ni: node_info }

#[inline] pub unsafe fn nat_get_nid(nat: *const nat_entry) -> nid_t { (*nat).ni.nid }
#[inline] pub unsafe fn nat_set_nid(nat: *mut nat_entry, n: nid_t) { (*nat).ni.nid = n; }
#[inline] pub unsafe fn nat_get_blkaddr(nat: *const nat_entry) -> block_t { (*nat).ni.blk_addr }
#[inline] pub unsafe fn nat_set_blkaddr(nat: *mut nat_entry, b: block_t) { (*nat).ni.blk_addr = b; }
#[inline] pub unsafe fn nat_get_ino(nat: *const nat_entry) -> nid_t { (*nat).ni.ino }
#[inline] pub unsafe fn nat_set_ino(nat: *mut nat_entry, i: nid_t) { (*nat).ni.ino = i; }
#[inline] pub unsafe fn nat_get_version(nat: *const nat_entry) -> u8 { (*nat).ni.version }
#[inline] pub unsafe fn nat_set_version(nat: *mut nat_entry, v: u8) { (*nat).ni.version = v; }
#[inline] pub fn inc_node_version(version: &mut u8) { *version = version.wrapping_add(1); }

#[inline] pub unsafe fn copy_node_info(dst: *mut node_info, src: *const node_info) {
    (*dst).nid = (*src).nid; (*dst).ino = (*src).ino; (*dst).blk_addr = (*src).blk_addr; (*dst).version = (*src).version;
}
#[inline] pub unsafe fn set_nat_flag(ne: *mut nat_entry, typ: u32, set: bool) {
    if set { (*ne).ni.flag |= BIT(typ) as u8; } else { (*ne).ni.flag &= !(BIT(typ) as u8); }
}
#[inline] pub unsafe fn get_nat_flag(ne: *const nat_entry, typ: u32) -> bool { ((*ne).ni.flag & BIT(typ) as u8) != 0 }
#[inline] pub unsafe fn nat_reset_flag(ne: *mut nat_entry) { set_nat_flag(ne, IS_CHECKPOINTED as u32, true); set_nat_flag(ne, HAS_FSYNCED_INODE as u32, false); set_nat_flag(ne, HAS_LAST_FSYNC as u32, true); }

#[inline] pub unsafe fn node_info_from_raw_nat(ni: *mut node_info, raw_ne: *const f2fs_nat_entry) { (*ni).ino = le32_to_cpu((*raw_ne).ino); (*ni).blk_addr = le32_to_cpu((*raw_ne).block_addr); (*ni).version = (*raw_ne).version; }
#[inline] pub unsafe fn raw_nat_from_node_info(raw_ne: *mut f2fs_nat_entry, ni: *const node_info) { (*raw_ne).ino = cpu_to_le32((*ni).ino); (*raw_ne).block_addr = cpu_to_le32((*ni).blk_addr); (*raw_ne).version = (*ni).version; }

#[repr(C)] pub struct nat_entry_set { pub set_list: list_head, pub entry_list: list_head, pub set: nid_t, pub entry_cnt: u32 }
#[repr(C)] pub struct free_nid { pub list: list_head, pub nid: nid_t, pub state: i32 }

#[inline] pub unsafe fn next_free_nid(sbi: *mut f2fs_sb_info, nid: *mut nid_t) {
    let nm_i = NM_I(sbi); spin_lock(&mut (*nm_i).nid_list_lock);
    if (*nm_i).nid_cnt[FREE_NID] <= 0 { spin_unlock(&mut (*nm_i).nid_list_lock); return; }
    let fnid = list_first_entry(&mut (*nm_i).free_nid_list, free_nid, list);
    *nid = (*fnid).nid; spin_unlock(&mut (*nm_i).nid_list_lock);
}

#[inline] pub unsafe fn get_nat_bitmap(sbi: *mut f2fs_sb_info, addr: *mut core::ffi::c_void) {
    let nm_i = NM_I(sbi);
    /* CONFIG_F2FS_CHECK_FS conditionally compares nat_bitmap and nat_bitmap_mir. */
    memcpy(addr, (*nm_i).nat_bitmap as *const _, (*nm_i).bitmap_size);
}
#[inline] pub unsafe fn current_nat_addr(sbi: *mut f2fs_sb_info, start: nid_t) -> pgoff_t {
    let nm_i = NM_I(sbi); let block_off = nat_block_offset(start);
    let mut block_addr = ((*nm_i).nat_blkaddr + (block_off << 1) - (block_off & (BLKS_PER_SEG(sbi) - 1))) as pgoff_t;
    if f2fs_test_bit(block_off, (*nm_i).nat_bitmap) { block_addr += BLKS_PER_SEG(sbi); } block_addr
}
#[inline] pub unsafe fn next_nat_addr(sbi: *mut f2fs_sb_info, mut block_addr: pgoff_t) -> pgoff_t { let nm_i = NM_I(sbi); block_addr -= (*nm_i).nat_blkaddr; block_addr ^= BIT((*sbi).log_blocks_per_seg); block_addr + (*nm_i).nat_blkaddr }
#[inline] pub unsafe fn set_to_next_nat(nm_i: *mut f2fs_nm_info, start_nid: nid_t) { f2fs_change_bit(nat_block_offset(start_nid), (*nm_i).nat_bitmap); /* CONFIG_F2FS_CHECK_FS also changes nat_bitmap_mir. */ }

#[inline] pub unsafe fn ino_of_node(node_folio: *const folio) -> nid_t { le32_to_cpu((*F2FS_NODE(node_folio)).footer.ino) }
#[inline] pub unsafe fn nid_of_node(node_folio: *const folio) -> nid_t { le32_to_cpu((*F2FS_NODE(node_folio)).footer.nid) }
#[inline] pub unsafe fn ofs_of_node(node_folio: *const folio) -> u32 { le32_to_cpu((*F2FS_NODE(node_folio)).footer.flag) >> OFFSET_BIT_SHIFT }
#[inline] pub unsafe fn cpver_of_node(node_folio: *const folio) -> u64 { le64_to_cpu((*F2FS_NODE(node_folio)).footer.cp_ver) }
#[inline] pub unsafe fn next_blkaddr_of_node(node_folio: *const folio) -> block_t { le32_to_cpu((*F2FS_NODE(node_folio)).footer.next_blkaddr) }

#[inline] pub unsafe fn fill_node_footer(folio: *const folio, nid: nid_t, ino: nid_t, ofs: u32, reset: bool) { let rn = F2FS_NODE(folio); let old_flag = if reset { memset(rn as *mut _, 0, core::mem::size_of::<f2fs_node>()); 0 } else { le32_to_cpu((*rn).footer.flag) }; (*rn).footer.nid = cpu_to_le32(nid); (*rn).footer.ino = cpu_to_le32(ino); (*rn).footer.flag = cpu_to_le32((ofs << OFFSET_BIT_SHIFT) | (old_flag & OFFSET_BIT_MASK)); }
#[inline] pub unsafe fn copy_node_footer(dst: *const folio, src: *const folio) { memcpy(&mut (*F2FS_NODE(dst)).footer as *mut _ as *mut _, &(*F2FS_NODE(src)).footer as *const _ as *const _, core::mem::size_of::<node_footer>()); }
#[inline] pub unsafe fn fill_node_footer_blkaddr(folio: *mut folio, blkaddr: block_t) { let ckpt = F2FS_CKPT(F2FS_F_SB(folio)); let rn = F2FS_NODE(folio); let mut cp_ver = cur_cp_version(ckpt); if __is_set_ckpt_flags(ckpt, CP_CRC_RECOVERY_FLAG) { cp_ver |= cur_cp_crc(ckpt) << 32; } (*rn).footer.cp_ver = cpu_to_le64(cp_ver); (*rn).footer.next_blkaddr = cpu_to_le32(blkaddr); }
#[inline] pub unsafe fn is_recoverable_dnode(folio: *const folio) -> bool { let ckpt = F2FS_CKPT(F2FS_F_SB(folio)); let mut cp_ver = cur_cp_version(ckpt); if __is_set_ckpt_flags(ckpt, CP_NOCRC_RECOVERY_FLAG) { return (cp_ver << 32) == (cpver_of_node(folio) << 32); } if __is_set_ckpt_flags(ckpt, CP_CRC_RECOVERY_FLAG) { cp_ver |= cur_cp_crc(ckpt) << 32; } cp_ver == cpver_of_node(folio) }

#[inline] pub unsafe fn IS_DNODE(node_folio: *const folio) -> bool { let mut ofs = ofs_of_node(node_folio); if f2fs_has_xattr_block(ofs) { return true; } if ofs == 3 || ofs == 4 + NIDS_PER_BLOCK || ofs == 5 + 2 * NIDS_PER_BLOCK { return false; } if ofs >= 6 + 2 * NIDS_PER_BLOCK { ofs -= 6 + 2 * NIDS_PER_BLOCK; if (ofs as i64) % (NIDS_PER_BLOCK + 1) == 0 { return false; } } true }
#[inline] pub unsafe fn set_nid(folio: *mut folio, off: i32, nid: nid_t, i: bool) -> i32 { let rn = F2FS_NODE(folio); f2fs_folio_wait_writeback(folio, NODE, true, true); if i { (*rn).i.i_nid[(off - NODE_DIR1_BLOCK) as usize] = cpu_to_le32(nid); } else { (*rn).in_.nid[off as usize] = cpu_to_le32(nid); } folio_mark_dirty(folio) }
#[inline] pub unsafe fn get_nid(folio: *const folio, off: i32, i: bool) -> nid_t { let rn = F2FS_NODE(folio); if i { return le32_to_cpu((*rn).i.i_nid[(off - NODE_DIR1_BLOCK) as usize]); } le32_to_cpu((*rn).in_.nid[off as usize]) }
#[inline] pub unsafe fn is_node(folio: *const folio, typ: u32) -> i32 { le32_to_cpu((*F2FS_NODE(folio)).footer.flag) & BIT(typ) }
#[inline] pub unsafe fn is_cold_node(folio: *const folio) -> i32 { is_node(folio, COLD_BIT_SHIFT) }
#[inline] pub unsafe fn is_fsync_dnode(folio: *const folio) -> i32 { is_node(folio, FSYNC_BIT_SHIFT) }
#[inline] pub unsafe fn is_dent_dnode(folio: *const folio) -> i32 { is_node(folio, DENT_BIT_SHIFT) }
#[inline] pub unsafe fn __set_mark(folio: *const folio, mark: bool, typ: u32) { let rn = F2FS_NODE(folio); let mut flag = le32_to_cpu((*rn).footer.flag); if mark { flag |= BIT(typ); } else { flag &= !BIT(typ); } (*rn).footer.flag = cpu_to_le32(flag); }
#[inline] pub unsafe fn set_cold_node(folio: *const folio, is_dir: bool) { __set_mark(folio, !is_dir, COLD_BIT_SHIFT); }
#[inline] pub unsafe fn set_mark(folio: *mut folio, mark: bool, typ: i32) { __set_mark(folio, mark, typ as u32); /* CONFIG_F2FS_CHECK_FS calls f2fs_inode_chksum_set here. */ }
#[inline] pub unsafe fn set_dentry_mark(folio: *mut folio, mark: bool) { set_mark(folio, mark, DENT_BIT_SHIFT as i32); }
#[inline] pub unsafe fn set_fsync_mark(folio: *mut folio, mark: bool) { set_mark(folio, mark, FSYNC_BIT_SHIFT as i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
