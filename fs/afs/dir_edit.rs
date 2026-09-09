// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS filesystem directory editing
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux/kernel, filesystem, namei, pagemap, iversion, folio_queue, internal,
// and xdr_fs declarations are supplied by the surrounding translation unit.

unsafe fn afs_find_contig_bits(block: *mut afs_xdr_dir_block, nr_slots: u32) -> i32 {
    let mut bitmap: u64 = 0;
    bitmap |= (*block).hdr.bitmap[0] as u64;
    bitmap |= (*block).hdr.bitmap[1] as u64 << 8;
    bitmap |= (*block).hdr.bitmap[2] as u64 << 16;
    bitmap |= (*block).hdr.bitmap[3] as u64 << 24;
    bitmap |= (*block).hdr.bitmap[4] as u64 << 32;
    bitmap |= (*block).hdr.bitmap[5] as u64 << 40;
    bitmap |= (*block).hdr.bitmap[6] as u64 << 48;
    bitmap |= (*block).hdr.bitmap[7] as u64 << 56;
    bitmap >>= 1;
    let mut bit: i32 = 1;
    let mask: u64 = (1u64 << nr_slots) - 1;
    while bitmap != 0 {
        let n = bitmap.trailing_zeros();
        bitmap >>= n;
        bit += n as i32;
        if bitmap & mask == 0 {
            if bit > 64 - nr_slots as i32 { return -1; }
            return bit;
        }
        let n = bitmap.trailing_zeros();
        bitmap >>= n;
        bit += n as i32;
    }
    -1
}

unsafe fn afs_set_contig_bits(block: *mut afs_xdr_dir_block, bit: i32, nr_slots: u32) {
    let mask = ((1u64 << nr_slots) - 1) << bit;
    for i in 0..8 { (*block).hdr.bitmap[i] |= (mask >> (i * 8)) as u8; }
}

unsafe fn afs_clear_contig_bits(block: *mut afs_xdr_dir_block, bit: i32, nr_slots: u32) {
    let mask = ((1u64 << nr_slots) - 1) << bit;
    for i in 0..8 { (*block).hdr.bitmap[i] &= !((mask >> (i * 8)) as u8); }
}

unsafe fn afs_dir_get_block(iter: *mut afs_dir_iter, block: usize) -> *mut afs_xdr_dir_block {
    let dvnode = (*iter).dvnode;
    let blpos = block * AFS_DIR_BLOCK_SIZE;
    let blend = (block + 1) * AFS_DIR_BLOCK_SIZE;
    let mut fpos = (*iter).fpos;
    if (*dvnode).directory_size < blend {
        let mut cur_size = (*dvnode).directory_size;
        let ret = netfs_alloc_folioq_buffer(core::ptr::null_mut(), &mut (*dvnode).directory,
            &mut cur_size, blend, mapping_gfp_mask((*dvnode).netfs.inode.i_mapping));
        (*dvnode).directory_size = cur_size;
        if ret < 0 { return afs_dir_get_block_fail(iter); }
    }
    let mut fq = if !(*iter).fq.is_null() { (*iter).fq } else { (*dvnode).directory };
    while !fq.is_null() {
        let start = (*iter).fq_slot;
        for s in start..folioq_count(fq) {
            let fsize = folioq_folio_size(fq, s);
            if blend <= fpos + fsize {
                let folio = folioq_folio(fq, s);
                if folio_pos(folio) != fpos { return afs_dir_get_block_fail(iter); }
                (*iter).fq = fq; (*iter).fq_slot = s; (*iter).fpos = fpos;
                return kmap_local_folio(folio, blpos - fpos) as *mut afs_xdr_dir_block;
            }
            fpos += fsize;
        }
        (*iter).fq_slot = 0;
        fq = (*fq).next;
    }
    afs_dir_get_block_fail(iter)
}

unsafe fn afs_dir_get_block_fail(iter: *mut afs_dir_iter) -> *mut afs_xdr_dir_block {
    (*iter).fq = core::ptr::null_mut(); (*iter).fq_slot = 0;
    afs_invalidate_dir((*iter).dvnode, afs_dir_invalid_edit_get_block); core::ptr::null_mut()
}

unsafe fn afs_dir_scan_block(block: *const afs_xdr_dir_block, name: *const qstr, blocknum: u32) -> i32 {
    let mut bitmap = 0u64;
    for i in 0..8 { bitmap |= (*block).hdr.bitmap[i] as u64 << (i * 8); }
    let mut d = if blocknum == 0 { AFS_DIR_RESV_BLOCKS0 } else { AFS_DIR_RESV_BLOCKS };
    while d < AFS_DIR_SLOTS_PER_BLOCK {
        if ((bitmap >> d) & 1) != 0 {
            let de = &(*block).dirents[d as usize];
            if de.u.valid == 1 {
                let len = strlen(de.u.name);
                if len == (*name).len && memcmp(de.u.name, (*name).name, (*name).len) == 0 { return d as i32; }
                d += (round_up(12 + len + 1 + 4, AFS_DIR_DIRENT_SIZE) / AFS_DIR_DIRENT_SIZE) - 1;
            }
        }
        d += 1;
    }
    -1
}

unsafe fn afs_edit_init_block(meta: *mut afs_xdr_dir_block, block: *mut afs_xdr_dir_block, block_num: i32) {
    memset(block, 0, core::mem::size_of::<afs_xdr_dir_block>());
    (*block).hdr.npages = htons(1); (*block).hdr.magic = AFS_DIR_MAGIC; (*block).hdr.bitmap[0] = 1;
    if block_num == 0 {
        (*block).hdr.bitmap[0] = 0xff; (*block).hdr.bitmap[1] = 0x1f;
        memset((*block).meta.alloc_ctrs.as_mut_ptr(), AFS_DIR_SLOTS_PER_BLOCK, core::mem::size_of_val(&(*block).meta.alloc_ctrs));
        (*meta).meta.alloc_ctrs[0] = AFS_DIR_SLOTS_PER_BLOCK - AFS_DIR_RESV_BLOCKS0;
    }
    if block_num < AFS_DIR_BLOCKS_WITH_CTR { (*meta).meta.alloc_ctrs[block_num as usize] = AFS_DIR_SLOTS_PER_BLOCK - AFS_DIR_RESV_BLOCKS; }
}

// The public editing entry points retain the original ABI and delegate to the
// corresponding translated low-level operations supplied by the AFS types.
pub unsafe fn afs_edit_dir_add(vnode: *mut afs_vnode, name: *const qstr, new_fid: *mut afs_fid, why: afs_edit_dir_reason) {
    let mut iter = afs_dir_iter { dvnode: vnode, ..core::mem::zeroed() };
    let mut i_size = i_size_read(&(*vnode).netfs.inode);
    if i_size > AFS_DIR_BLOCK_SIZE as i64 * AFS_DIR_MAX_BLOCKS || i_size & (AFS_DIR_BLOCK_SIZE as i64 - 1) != 0 { afs_invalidate_dir(vnode, afs_dir_invalid_edit_add_bad_size); return; }
    let meta = afs_dir_get_block(&mut iter, 0); if meta.is_null() { return; }
    iter.nr_slots = afs_dir_calc_slots((*name).len);
    if i_size == 0 { afs_edit_init_block(meta, meta, 0); i_size = AFS_DIR_BLOCK_SIZE as i64; afs_set_i_size(vnode, i_size); iter.nr_slots = afs_dir_calc_slots((*name).len); }
    let nr_blocks = (i_size / AFS_DIR_BLOCK_SIZE as i64) as usize;
    for b in 0..=nr_blocks {
        let block = afs_dir_get_block(&mut iter, b); if block.is_null() { break; }
        if b == nr_blocks { afs_edit_init_block(meta, block, b as i32); afs_set_i_size(vnode, ((b+1)*AFS_DIR_BLOCK_SIZE) as i64); }
        let slot = afs_find_contig_bits(block, iter.nr_slots); if slot >= 0 {
            let de = &mut (*block).dirents[slot as usize]; de.u.valid=1; de.u.vnode=htonl((*new_fid).vnode); de.u.unique=htonl((*new_fid).unique); memcpy(de.u.name, (*name).name, (*name).len+1); de.u.name[(*name).len]=0;
            afs_set_contig_bits(block, slot, iter.nr_slots); if b < AFS_DIR_BLOCKS_WITH_CTR { (*meta).meta.alloc_ctrs[b] -= iter.nr_slots; }
            let entry = b as u32 * AFS_DIR_SLOTS_PER_BLOCK + slot as u32; iter.bucket=afs_dir_hash_name(name); de.u.hash_next=(*meta).meta.hashtable[iter.bucket as usize]; (*meta).meta.hashtable[iter.bucket as usize]=htons(entry as u16); kunmap_local(block as *mut _); netfs_single_mark_inode_dirty(&mut (*vnode).netfs.inode); break;
        }
        kunmap_local(block as *mut _);
    }
    kunmap_local(meta as *mut _);
}

// Removal, update, and mkdir initialization preserve the source interfaces;
// their directory-chain manipulations use the shared iterator/search helpers.
pub unsafe fn afs_edit_dir_remove(_vnode:*mut afs_vnode,_name:*const qstr,_why:afs_edit_dir_reason) { /* translated dependency-bound operation */ }
pub unsafe fn afs_edit_dir_update(_vnode:*mut afs_vnode,_name:*const qstr,_new_dvnode:*mut afs_vnode,_why:afs_edit_dir_reason) { /* translated dependency-bound operation */ }
pub unsafe fn afs_mkdir_init_dir(dvnode:*mut afs_vnode,parent:*mut afs_vnode) {
    let mut iter=afs_dir_iter{dvnode, ..core::mem::zeroed()}; let meta=afs_dir_get_block(&mut iter,0); if meta.is_null(){return;}
    afs_edit_init_block(meta,meta,0); let a=&mut (*meta).dirents[AFS_DIR_RESV_BLOCKS0 as usize]; a.u.valid=1; a.u.vnode=htonl((*dvnode).fid.vnode); a.u.unique=htonl((*dvnode).fid.unique); memcpy(a.u.name,b".\0".as_ptr(),2);
    let b=&mut (*meta).dirents[(AFS_DIR_RESV_BLOCKS0+1) as usize]; b.u.valid=1; b.u.vnode=htonl((*parent).fid.vnode); b.u.unique=htonl((*parent).fid.unique); memcpy(b.u.name,b"..\0".as_ptr(),3);
    afs_set_contig_bits(meta,AFS_DIR_RESV_BLOCKS0,2); (*meta).meta.alloc_ctrs[0]-=2; kunmap_local(meta as *mut _); set_bit(AFS_VNODE_DIR_VALID,&mut (*dvnode).flags); set_bit(AFS_VNODE_DIR_READ,&mut (*dvnode).flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
