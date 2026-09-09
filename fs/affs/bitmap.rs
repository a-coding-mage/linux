// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/affs/bitmap.c
 *
 *  (c) 1996 Hans-Joachim Widmaier
 *
 *  bitmap.c contains the code that handles all bitmap related stuff -
 *  block allocation, deallocation, calculation of free space.
 */

// Dependency declarations are supplied by the surrounding kernel translation.

pub unsafe fn affs_count_free_blocks(sb: *mut super_block) -> u32 {
    let mut bm: *mut affs_bm_info;
    let mut free: u32;
    let mut i: i32;

    pr_debug!("%s()\n", "affs_count_free_blocks");

    if sb_rdonly(sb) { return 0; }
    mutex_lock(&mut (*AFFS_SB(sb)).s_bmlock);
    bm = (*AFFS_SB(sb)).s_bitmap;
    free = 0;
    i = (*AFFS_SB(sb)).s_bmap_count;
    while i > 0 {
        free = free.wrapping_add((*bm).bm_free);
        bm = bm.add(1);
        i -= 1;
    }
    mutex_unlock(&mut (*AFFS_SB(sb)).s_bmlock);
    free
}

pub unsafe fn affs_free_block(sb: *mut super_block, block: u32) {
    let sbi = AFFS_SB(sb);
    let mut bm: *mut affs_bm_info;
    let mut bh: *mut buffer_head;
    let (blk, bmap, bit, mask, tmp): (u32, u32, u32, u32, u32);
    let data: *mut u32;

    pr_debug!("%s(%u)\n", "affs_free_block", block);
    if block > (*sbi).s_partition_size { goto!(err_range); }
    let blk = block.wrapping_sub((*sbi).s_reserved);
    let bmap = blk / (*sbi).s_bmap_bits;
    let bit = blk % (*sbi).s_bmap_bits;
    let bm = (*sbi).s_bitmap.add(bmap as usize);
    mutex_lock(&mut (*sbi).s_bmlock);
    bh = (*sbi).s_bmap_bh;
    if (*sbi).s_last_bmap != bmap {
        affs_brelse(bh);
        bh = affs_bread(sb, (*bm).bm_key);
        if bh.is_null() { goto!(err_bh_read); }
        (*sbi).s_bmap_bh = bh;
        (*sbi).s_last_bmap = bmap;
    }
    let mask = 1u32 << (bit & 31);
    let data = ((*bh).b_data as *mut u8).add((bit / 32 * 4 + 4) as usize) as *mut u32;
    let tmp = u32::from_be((*data));
    if tmp & mask != 0 { goto!(err_free); }
    *data = (tmp | mask).to_be();
    let tmp = u32::from_be(*((*bh).b_data as *mut u32));
    *((*bh).b_data as *mut u32) = tmp.wrapping_sub(mask).to_be();
    mark_buffer_dirty(bh); affs_mark_sb_dirty(sb); (*bm).bm_free += 1;
    mutex_unlock(&mut (*sbi).s_bmlock); return;
err_free:
    affs_warning(sb, "affs_free_block", "Trying to free block %u which is already free", block);
    mutex_unlock(&mut (*sbi).s_bmlock); return;
err_bh_read:
    affs_error(sb, "affs_free_block", "Cannot read bitmap block %u", (*bm).bm_key);
    (*sbi).s_bmap_bh = core::ptr::null_mut(); (*sbi).s_last_bmap = !0;
    mutex_unlock(&mut (*sbi).s_bmlock); return;
err_range:
    affs_error(sb, "affs_free_block", "Block %u outside partition", block);
}

/* Allocate a block in the given allocation zone. */
pub unsafe fn affs_alloc_block(inode: *mut inode, mut goal: u32) -> u32 {
    let sb = (*inode).i_sb; let sbi = AFFS_SB(sb);
    pr_debug!("balloc(inode=%llu,goal=%u): ", (*inode).i_ino, goal);
    if (*AFFS_I(inode)).i_pa_cnt != 0 {
        pr_debug!("%d\n", (*AFFS_I(inode)).i_lastalloc + 1);
        (*AFFS_I(inode)).i_pa_cnt -= 1; (*AFFS_I(inode)).i_lastalloc += 1;
        return (*AFFS_I(inode)).i_lastalloc;
    }
    if goal == 0 || goal > (*sbi).s_partition_size {
        if goal != 0 { affs_warning(sb, "affs_balloc", "invalid goal %d", goal); }
        goal = (*sbi).s_reserved;
    }
    let mut blk = goal - (*sbi).s_reserved;
    let mut bmap = blk / (*sbi).s_bmap_bits;
    let mut bm = (*sbi).s_bitmap.add(bmap as usize);
    mutex_lock(&mut (*sbi).s_bmlock);
    if (*bm).bm_free == 0 {
        let mut i = (*sbi).s_bmap_count;
        loop {
            i -= 1; if i < 0 { mutex_unlock(&mut (*sbi).s_bmlock); pr_debug!("failed\n"); return 0; }
            bmap += 1; bm = bm.add(1);
            if bmap >= (*sbi).s_bmap_count { bmap = 0; bm = (*sbi).s_bitmap; }
            if (*bm).bm_free != 0 { break; }
        }
        blk = bmap * (*sbi).s_bmap_bits;
    }
    let mut bh = (*sbi).s_bmap_bh;
    if (*sbi).s_last_bmap != bmap {
        affs_brelse(bh); bh = affs_bread(sb, (*bm).bm_key);
        if bh.is_null() { affs_error(sb,"affs_read_block","Cannot read bitmap block %u",(*bm).bm_key); (*sbi).s_bmap_bh=core::ptr::null_mut(); (*sbi).s_last_bmap=!0; mutex_unlock(&mut (*sbi).s_bmlock); pr_debug!("failed\n"); return 0; }
        (*sbi).s_bmap_bh=bh; (*sbi).s_last_bmap=bmap;
    }
    let bit = blk % (*sbi).s_bmap_bits;
    let mut data = ((*bh).b_data as *mut u8).add((bit / 32 * 4 + 4) as usize) as *mut u32;
    let enddata = (*bh).b_data.add((*sb).s_blocksize as usize) as *mut u32;
    let mut mask = !0u32 << (bit & 31); blk &= !31;
    let mut tmp = u32::from_be(*data);
    if tmp & mask == 0 {
        loop { blk += 32; data = data.add(1); if data >= enddata { if (*bm).bm_free == 0 { mutex_unlock(&mut (*sbi).s_bmlock); return 0; } continue; } if *data != 0 { break; } }
        tmp=u32::from_be(*data); mask=!0;
    }
    let bit = (tmp & mask).trailing_zeros(); blk += bit + (*sbi).s_reserved;
    let mut mask2 = 1u32 << (bit & 31); mask = mask2; (*AFFS_I(inode)).i_lastalloc=blk;
    while { mask2 <<= 1; mask2 != 0 } { if tmp & mask2 == 0 { break; } (*AFFS_I(inode)).i_pa_cnt += 1; mask |= mask2; }
    (*bm).bm_free -= (*AFFS_I(inode)).i_pa_cnt + 1; *data=(tmp & !mask).to_be();
    tmp=u32::from_be(*((*bh).b_data as *mut u32)); *((*bh).b_data as *mut u32)=tmp.wrapping_add(mask).to_be();
    mark_buffer_dirty(bh); affs_mark_sb_dirty(sb); mutex_unlock(&mut (*sbi).s_bmlock); pr_debug!("%d\n",blk); blk
}

pub unsafe fn affs_init_bitmap(sb: *mut super_block, flags: *mut i32) -> i32 {
    let sbi=AFFS_SB(sb); if *flags & SB_RDONLY != 0 { return 0; }
    if (*AFFS_ROOT_TAIL(sb,(*sbi).s_root_bh)).bm_flag == 0 { pr_notice!("Bitmap invalid - mounting %s read only\n",(*sb).s_id); *flags|=SB_RDONLY; return 0; }
    (*sbi).s_last_bmap=!0; (*sbi).s_bmap_bh=core::ptr::null_mut(); (*sbi).s_bmap_bits=(*sb).s_blocksize*8-32;
    (*sbi).s_bmap_count=((*sbi).s_partition_size-(*sbi).s_reserved+(*sbi).s_bmap_bits-1)/(*sbi).s_bmap_bits;
    let size=(*sbi).s_bmap_count as usize*core::mem::size_of::<affs_bm_info>(); (*sbi).s_bitmap=kzalloc(size,GFP_KERNEL); if (*sbi).s_bitmap.is_null(){pr_err!("Bitmap allocation failed\n");return -ENOMEM;}
    let mut bmap_blk=(*sbi).s_root_bh.b_data as *mut u32; let mut blk=(*sb).s_blocksize/4-49; let mut end=blk+25; let mut bh=core::ptr::null_mut(); let mut bmap_bh=core::ptr::null_mut(); let mut bm=(*sbi).s_bitmap; let mut i=(*sbi).s_bmap_count;
    while i>0 { affs_brelse(bh); (*bm).bm_key=u32::from_be(*bmap_blk.add(blk as usize)); bh=affs_bread(sb,(*bm).bm_key); if bh.is_null(){pr_err!("Cannot read bitmap\n");affs_brelse(bmap_bh);return -EIO;} if affs_checksum_block(sb,bh)!=0{*flags|=SB_RDONLY;break;} (*bm).bm_free=memweight((*bh).b_data.add(4),(*sb).s_blocksize-4); bm=bm.add(1); i-=1; blk+=1; if blk>=end && i!=0 {affs_brelse(bmap_bh);bmap_bh=affs_bread(sb,u32::from_be(*bmap_blk.add(blk as usize)));if bmap_bh.is_null(){affs_brelse(bh);return -EIO;}bmap_blk=(*bmap_bh).b_data as *mut u32;blk=0;end=(*sb).s_blocksize/4-1;} }
    affs_brelse(bh); affs_brelse(bmap_bh); 0
}

pub unsafe fn affs_free_bitmap(sb: *mut super_block) { let sbi=AFFS_SB(sb); if (*sbi).s_bitmap.is_null(){return;} affs_brelse((*sbi).s_bmap_bh); (*sbi).s_bmap_bh=core::ptr::null_mut(); (*sbi).s_last_bmap=!0; kfree((*sbi).s_bitmap as *mut core::ffi::c_void); (*sbi).s_bitmap=core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
