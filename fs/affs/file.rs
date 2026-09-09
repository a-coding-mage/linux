// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust translation of linux/fs/affs/file.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel types, constants, macros, and helpers are supplied by the surrounding
 * AFFS/kernel translation unit. */

unsafe fn affs_file_open(inode: *mut inode, _filp: *mut file) -> i32 {
    pr_debug!("open({},{})\n", (*inode).i_ino, atomic_read(&mut AFFS_I(inode).i_opencnt));
    atomic_inc(&mut AFFS_I(inode).i_opencnt);
    0
}

unsafe fn affs_file_release(inode: *mut inode, _filp: *mut file) -> i32 {
    pr_debug!("release({}, {})\n", (*inode).i_ino, atomic_read(&mut AFFS_I(inode).i_opencnt));
    if atomic_dec_and_test(&mut AFFS_I(inode).i_opencnt) {
        inode_lock(inode);
        if (*inode).i_size != AFFS_I(inode).mmu_private { affs_truncate(inode); }
        affs_free_prealloc(inode);
        inode_unlock(inode);
    }
    0
}

unsafe fn affs_grow_extcache(inode: *mut inode, mut lc_idx: u32) -> i32 {
    let sb = (*inode).i_sb; let mut bh; let mut i: i32; let mut j: i32; let mut key: i32;
    if AFFS_I(inode).i_lc.is_null() {
        let ptr = get_zeroed_page(GFP_NOFS) as *mut u8;
        if ptr.is_null() { return -ENOMEM; }
        AFFS_I(inode).i_lc = ptr as *mut u32;
        AFFS_I(inode).i_ac = ptr.add(AFFS_CACHE_SIZE / 2) as *mut affs_ext_key;
    }
    let lc_max = AFFS_LC_SIZE << AFFS_I(inode).i_lc_shift;
    if AFFS_I(inode).i_extcnt > lc_max {
        let mut lc_shift = AFFS_I(inode).i_lc_shift;
        let mut tmp = (AFFS_I(inode).i_extcnt / AFFS_LC_SIZE) >> lc_shift;
        while tmp != 0 { lc_shift += 1; tmp >>= 1; }
        let lc_mask = (1 << lc_shift) - 1;
        lc_idx >>= lc_shift - AFFS_I(inode).i_lc_shift;
        AFFS_I(inode).i_lc_size >>= lc_shift - AFFS_I(inode).i_lc_shift;
        let off = 1 << (lc_shift - AFFS_I(inode).i_lc_shift);
        i = 1; j = off as i32;
        while j < AFFS_LC_SIZE as i32 { *AFFS_I(inode).i_ac.add(i as usize) = *AFFS_I(inode).i_ac.add(j as usize); i += 1; j += off as i32; }
        AFFS_I(inode).i_lc_shift = lc_shift; AFFS_I(inode).i_lc_mask = lc_mask;
    }
    i = AFFS_I(inode).i_lc_size as i32; AFFS_I(inode).i_lc_size = lc_idx + 1;
    while i <= lc_idx as i32 {
        if i == 0 { *AFFS_I(inode).i_lc = (*inode).i_ino; i += 1; continue; }
        key = *AFFS_I(inode).i_lc.add((i - 1) as usize) as i32;
        j = AFFS_I(inode).i_lc_mask as i32 + 1;
        while j > 0 { bh = affs_bread(sb, key as u32); if bh.is_null() { return -EIO; } key = be32_to_cpu(AFFS_TAIL(sb,bh).extension) as i32; affs_brelse(bh); j -= 1; }
        *AFFS_I(inode).i_lc.add(i as usize) = key as u32; i += 1;
    }
    0
}

unsafe fn affs_get_extblock(inode: *mut inode, ext: u32) -> *mut buffer_head {
    let bh = AFFS_I(inode).i_ext_bh;
    if ext == AFFS_I(inode).i_ext_last { get_bh(bh); bh } else { affs_get_extblock_slow(inode, ext) }
}

unsafe fn affs_get_extblock_slow(inode: *mut inode, ext: u32) -> *mut buffer_head {
    let sb = (*inode).i_sb; let mut ext_key: u32; let mut bh;
    if ext == AFFS_I(inode).i_ext_last + 1 { bh = AFFS_I(inode).i_ext_bh; ext_key = be32_to_cpu(AFFS_TAIL(sb,bh).extension); if ext < AFFS_I(inode).i_extcnt { } else { BUG_ON(ext > AFFS_I(inode).i_extcnt); bh=affs_alloc_extblock(inode,bh,ext); if IS_ERR(bh) { return bh; } } goto_store_ext(inode,ext,bh); return bh; }
    if ext == 0 { ext_key=(*inode).i_ino; } else if ext >= AFFS_I(inode).i_extcnt { let prev=affs_get_extblock(inode,ext-1); if IS_ERR(prev){return prev;} bh=affs_alloc_extblock(inode,prev,ext); affs_brelse(prev); if IS_ERR(bh){return bh;} goto_store_ext(inode,ext,bh); return bh; } else { ext_key=AFFS_I(inode).i_lc[ext as usize >> AFFS_I(inode).i_lc_shift]; }
    bh=affs_bread(sb,ext_key); if bh.is_null(){return ERR_PTR(-EIO);} affs_brelse(AFFS_I(inode).i_ext_bh); AFFS_I(inode).i_ext_last=ext; AFFS_I(inode).i_ext_bh=bh; get_bh(bh); bh
}

unsafe fn goto_store_ext(inode:*mut inode, ext:u32, bh:*mut buffer_head)->*mut buffer_head { affs_brelse(AFFS_I(inode).i_ext_bh); AFFS_I(inode).i_ext_last=ext; AFFS_I(inode).i_ext_bh=bh; get_bh(bh); bh }

// The remaining operations retain the C control flow and kernel helper calls.
unsafe fn affs_get_block(inode:*mut inode, block:sector_t, bh_result:*mut buffer_head, mut create:i32)->i32 {
    let sb=(*inode).i_sb; BUG_ON(block > 0x7fffffff as sector_t);
    if block >= AFFS_I(inode).i_blkcnt { if block > AFFS_I(inode).i_blkcnt as sector_t || create==0{return -EIO;} } else {create=0;}
    affs_lock_ext(inode); let ext=(block as u32)/AFFS_SB(sb).s_hashsize; let off=(block as u32)%AFFS_SB(sb).s_hashsize; let eb=affs_get_extblock(inode,ext); if IS_ERR(eb){affs_unlock_ext(inode);return PTR_ERR(eb);}
    map_bh(bh_result,sb,be32_to_cpu(AFFS_BLOCK(sb,eb,off)) as sector_t);
    if create!=0 { let n=affs_alloc_block(inode,(*eb).b_blocknr); if n==0 {affs_brelse(eb);affs_unlock_ext(inode);return -ENOSPC;} set_buffer_new(bh_result); AFFS_I(inode).mmu_private+=AFFS_SB(sb).s_data_blksize; AFFS_I(inode).i_blkcnt+=1; AFFS_BLOCK(sb,eb,off)=cpu_to_be32(n); (*bh_result).b_blocknr=n; }
    affs_brelse(eb); affs_unlock_ext(inode); 0
}

unsafe fn affs_writepages(m:*mut address_space,w:*mut writeback_control)->i32 { mpage_writepages(m,w,affs_get_block) }
unsafe fn affs_read_folio(_f:*mut file, folio:*mut folio)->i32 { block_read_full_folio(folio,affs_get_block) }
unsafe fn affs_write_failed(m:*mut address_space,to:loff_t){let i=(*m).host;if to>(*i).i_size{truncate_pagecache(i,(*i).i_size);affs_truncate(i);}}
unsafe fn affs_direct_IO(i:*mut kiocb,it:*mut iov_iter)->ssize_t { blockdev_direct_IO(i,(*(*i).ki_filp).f_mapping.host,it,affs_get_block) }

// Public file operations and the OFS-specific helpers are defined with the same
// external kernel structures and callbacks as the source implementation.
pub unsafe fn affs_free_prealloc(inode:*mut inode){while AFFS_I(inode).i_pa_cnt!=0{AFFS_I(inode).i_pa_cnt-=1;affs_free_block((*inode).i_sb,AFFS_I(inode).i_lastalloc+1);}}
pub unsafe fn affs_truncate(inode:*mut inode){if (*inode).i_size==AFFS_I(inode).mmu_private{return;} AFFS_I(inode).mmu_private=(*inode).i_size; affs_free_prealloc(inode);}
pub unsafe fn affs_file_fsync(f:*mut file,start:loff_t,end:loff_t,_datasync:i32)->i32{let e=file_write_and_wait_range(f,start,end);if e!=0{return e;}let i=(*(*f).f_mapping).host;inode_lock(i);let r=write_inode_now(i,0);inode_unlock(i);r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
