// SPDX-License-Identifier: GPL-2.0-only
/*
 * partition.c
 *
 * PURPOSE
 *      Partition handling routines for the OSTA-UDF(tm) filesystem.
 *
 * COPYRIGHT
 *  (C) 1998-2001 Ben Fennema
 */

// Declarations supplied by udfdecl.h, udf_sb.h, udf_i.h and Linux headers are
// intentionally left as external dependencies of this translation unit.

pub unsafe fn udf_get_pblock(sb: *mut super_block, block: u32,
                             partition: u16, offset: u32) -> u32 {
    let sbi = UDF_SB(sb);
    if partition >= (*sbi).s_partitions {
        udf_debug(c"block=%u, partition=%u, offset=%u: invalid partition\n", block, partition, offset);
        return 0xffff_ffff;
    }
    let map = &mut (*sbi).s_partmaps[partition as usize];
    if let Some(func) = (*map).s_partition_func {
        func(sb, block, partition, offset)
    } else {
        (*map).s_partition_root + block + offset
    }
}

pub unsafe fn udf_get_pblock_virt15(sb: *mut super_block, mut block: u32,
                                    partition: u16, offset: u32) -> u32 {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut newblock: u32;
    let mut index: u32;
    let loc: u32;
    let sbi = UDF_SB(sb);
    let map = &mut (*sbi).s_partmaps[partition as usize];
    let vdata = &mut (*map).s_type_specific.s_virtual;
    let iinfo = UDF_I((*sbi).s_vat_inode);

    if block >= (*vdata).s_num_entries {
        udf_debug(c"Trying to access block beyond end of VAT (%u max %u)\n", block, (*vdata).s_num_entries);
        return 0xffff_ffff;
    }
    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB {
        loc = le32_to_cpu(*(((*iinfo).i_data.add((*vdata).s_start_offset as usize) as *const u32).add(block as usize)));
    } else {
        index = ((*sb).s_blocksize - (*vdata).s_start_offset) / core::mem::size_of::<u32>() as u32;
        if block >= index {
            block -= index;
            newblock = 1 + block / ((*sb).s_blocksize / core::mem::size_of::<u32>() as u32);
            index = block % ((*sb).s_blocksize / core::mem::size_of::<u32>() as u32);
        } else {
            newblock = 0;
            index = (*vdata).s_start_offset / core::mem::size_of::<u32>() as u32 + block;
        }
        let mut err = 0;
        bh = udf_bread((*sbi).s_vat_inode, newblock, 0, &mut err);
        if bh.is_null() {
            udf_debug(c"get_pblock(UDF_VIRTUAL_MAP:%p,%u,%u)\n", sb, block, partition);
            return 0xffff_ffff;
        }
        loc = le32_to_cpu(*(((*bh).b_data as *const u32).add(index as usize)));
        brelse(bh);
    }
    if (*iinfo).i_location.partitionReferenceNum == partition {
        udf_debug(c"recursive call to udf_get_pblock!\n");
        return 0xffff_ffff;
    }
    udf_get_pblock(sb, loc, (*iinfo).i_location.partitionReferenceNum, offset)
}

#[inline]
pub unsafe fn udf_get_pblock_virt20(sb: *mut super_block, block: u32,
                                    partition: u16, offset: u32) -> u32 {
    udf_get_pblock_virt15(sb, block, partition, offset)
}

pub unsafe fn udf_get_pblock_spar15(sb: *mut super_block, block: u32,
                                    partition: u16, offset: u32) -> u32 {
    let sbi = UDF_SB(sb);
    let map = &mut (*sbi).s_partmaps[partition as usize];
    let sdata = &mut (*map).s_type_specific.s_sparing;
    let packet = (block + offset) & !((*sdata).s_packet_len - 1);
    let mut st: *mut sparingTable = core::ptr::null_mut();
    for i in 0..4 { if !(*sdata).s_spar_map[i].is_null() { st = (*sdata).s_spar_map[i].b_data as *mut sparingTable; break; } }
    if !st.is_null() {
        for i in 0..le16_to_cpu((*st).reallocationTableLen) as usize {
            let entry = &(*st).mapEntry[i];
            let origLoc = le32_to_cpu((*entry).origLocation);
            if origLoc >= 0xffff_fff0 { break; }
            else if origLoc == packet { return le32_to_cpu((*entry).mappedLocation) + ((block + offset) & ((*sdata).s_packet_len - 1)); }
            else if origLoc > packet { break; }
        }
    }
    (*map).s_partition_root + block + offset
}

pub unsafe fn udf_relocate_blocks(sb: *mut super_block, old_block: i64, new_block: *mut i64) -> i32 {
    let sbi = UDF_SB(sb); let mut ret = 0; let mut st: *mut sparingTable = core::ptr::null_mut();
    mutex_lock(&mut (*sbi).s_alloc_mutex);
    for i in 0..(*sbi).s_partitions as usize {
        let map = &mut (*sbi).s_partmaps[i];
        if old_block > (*map).s_partition_root as i64 && old_block < ((*map).s_partition_root + (*map).s_partition_len) as i64 {
            let sdata = &mut (*map).s_type_specific.s_sparing;
            let packet = ((old_block as u32 - (*map).s_partition_root) & !((*sdata).s_packet_len - 1));
            let mut j = 0usize; while j < 4 { if !(*sdata).s_spar_map[j].is_null() { st = (*sdata).s_spar_map[j].b_data as *mut sparingTable; break; } j += 1; }
            if st.is_null() { ret = 1; break; }
            let len = le16_to_cpu((*st).reallocationTableLen) as usize;
            let mut k = 0usize;
            while k < len {
                let entry = &mut (*st).mapEntry[k]; let orig = le32_to_cpu((*entry).origLocation);
                if orig == 0xffff_ffff {
                    while j < 4 { let bh = (*sdata).s_spar_map[j]; if !bh.is_null() { let st2 = (*bh).b_data as *mut sparingTable; (*st2).mapEntry[k].origLocation = cpu_to_le32(packet); udf_update_tag((*st2) as *mut i8, core::mem::size_of::<sparingTable>() + len * core::mem::size_of::<sparingEntry>()); mark_buffer_dirty(bh); } j += 1; }
                    *new_block = le32_to_cpu((*entry).mappedLocation) as i64 + ((old_block as u32 - (*map).s_partition_root) & ((*sdata).s_packet_len - 1)) as i64; ret = 0; break;
                } else if orig == packet { *new_block = le32_to_cpu((*entry).mappedLocation) as i64 + ((old_block as u32 - (*map).s_partition_root) & ((*sdata).s_packet_len - 1)) as i64; ret = 0; break; }
                else if orig > packet { break; } k += 1;
            }
            if ret == 0 { break; }
            let mut l = k;
            while l < len {
                let entry = &(*st).mapEntry[l];
                if le32_to_cpu((*entry).origLocation) == 0xffff_ffff {
                    while j < 4 {
                        let bh = (*sdata).s_spar_map[j];
                        if !bh.is_null() {
                            let st2 = (*bh).b_data as *mut sparingTable;
                            let map_entry = (*st2).mapEntry[l];
                            (*st2).mapEntry[k + 1..=l].copy_from_slice(&(*st2).mapEntry[k..l]);
                            let mut moved = map_entry;
                            moved.origLocation = cpu_to_le32(packet);
                            (*st2).mapEntry[k] = moved;
                            udf_update_tag(st2 as *mut i8, core::mem::size_of::<sparingTable>() + len * core::mem::size_of::<sparingEntry>());
                            mark_buffer_dirty(bh);
                        }
                        j += 1;
                    }
                    *new_block = le32_to_cpu((*st).mapEntry[k].mappedLocation) as i64 +
                        ((old_block as u32 - (*map).s_partition_root) & ((*sdata).s_packet_len - 1)) as i64;
                    ret = 0;
                    break;
                }
                l += 1;
            }
            if ret == 0 { break; }
            ret = 1; break;
        }
    }
    mutex_unlock(&mut (*sbi).s_alloc_mutex); ret
}

unsafe fn udf_try_read_meta(inode: *mut inode, block: u32, partition: u16, offset: u32) -> u32 {
    let sb = (*inode).i_sb; let mut epos = core::mem::zeroed(); let mut eloc = core::mem::zeroed(); let mut elen = 0; let mut ext_offset = 0; let mut etype = 0;
    let err = inode_bmap(inode, block, &mut epos, &mut eloc, &mut elen, &mut ext_offset, &mut etype);
    let phyblock = if err <= 0 || etype != (EXT_RECORDED_ALLOCATED >> 30) { 0xffff_ffff } else { let map = &UDF_SB(sb).s_partmaps[partition as usize]; udf_get_pblock(sb, eloc.logicalBlockNum, (*map).s_type_specific.s_metadata.s_phys_partition_ref, ext_offset + offset) };
    brelse(epos.bh); phyblock
}

pub unsafe fn udf_get_pblock_meta25(sb: *mut super_block, block: u32, partition: u16, offset: u32) -> u32 {
    let sbi = UDF_SB(sb); let map = &mut (*sbi).s_partmaps[partition as usize]; let mdata = &mut (*map).s_type_specific.s_metadata;
    udf_debug(c"READING from METADATA\n");
    let mut inode = if !(*mdata).s_metadata_fe.is_null() { (*mdata).s_metadata_fe } else { (*mdata).s_mirror_fe };
    if inode.is_null() { return 0xffff_ffff; }
    let mut retblk = udf_try_read_meta(inode, block, partition, offset);
    if retblk == 0xffff_ffff && !(*mdata).s_metadata_fe.is_null() {
        udf_warn(sb, c"error reading from METADATA, trying to read from MIRROR\n");
        if (*mdata).s_flags & MF_MIRROR_FE_LOADED == 0 { (*mdata).s_mirror_fe = udf_find_metadata_inode_efe(sb, (*mdata).s_mirror_file_loc, (*mdata).s_phys_partition_ref); if IS_ERR((*mdata).s_mirror_fe) { (*mdata).s_mirror_fe = core::ptr::null_mut(); } (*mdata).s_flags |= MF_MIRROR_FE_LOADED; }
        inode = (*mdata).s_mirror_fe; if inode.is_null() { return 0xffff_ffff; } retblk = udf_try_read_meta(inode, block, partition, offset);
    }
    retblk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
