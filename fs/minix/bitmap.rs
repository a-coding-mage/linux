// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/minix/bitmap.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

/*
 * Modified for 680x0 by Hamish Macdonald
 * Fixed for 680x0 by Andreas Schwab
 */

/* bitmap.c contains the code that handles the inode and block bitmaps */

// Dependencies supplied by the surrounding kernel translation.

static mut bitmap_lock: spinlock_t = DEFINE_SPINLOCK_INIT();

/*
 * bitmap consists of blocks filled with 16bit words
 * bit set == busy, bit clear == free
 * endianness is a mess, but for counting zero bits it really doesn't matter...
 */
unsafe fn count_free(map: *mut *mut buffer_head, blocksize: c_uint, mut numbits: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut blocks = DIV_ROUND_UP(numbits, blocksize * 8);

    while blocks != 0 {
        blocks -= 1;
        let mut words = blocksize / 2;
        let mut p = (*(*map)).b_data as *mut u16;
        map = map.add(1);
        while words != 0 {
            words -= 1;
            sum = sum.wrapping_add(16 - hweight16(*p));
            p = p.add(1);
        }
    }
    sum
}

pub unsafe fn minix_free_block(inode: *mut inode, block: c_ulong) {
    let sb = (*inode).i_sb;
    let sbi = minix_sb(sb);
    let mut bh: *mut buffer_head;
    let k = (*sb).s_blocksize_bits + 3;
    let bit: c_ulong;
    let mut zone: c_ulong;

    if block < (*sbi).s_firstdatazone || block >= (*sbi).s_nzones {
        printk(cstr!("Trying to free block not in datazone\n"));
        return;
    }
    zone = block - (*sbi).s_firstdatazone + 1;
    bit = zone & ((1 << k) - 1);
    zone >>= k;
    if zone >= (*sbi).s_zmap_blocks {
        printk(cstr!("minix_free_block: nonexistent bitmap buffer\n"));
        return;
    }
    bh = *(*sbi).s_zmap.add(zone as usize);
    spin_lock(&raw mut bitmap_lock);
    if !minix_test_and_clear_bit(bit, (*bh).b_data) {
        printk(cstr!("minix_free_block (%s:%lu): bit already cleared\n"), (*sb).s_id, block);
    }
    spin_unlock(&raw mut bitmap_lock);
    mark_buffer_dirty(bh);
}

pub unsafe fn minix_new_block(inode: *mut inode) -> c_int {
    let sbi = minix_sb((*inode).i_sb);
    let bits_per_zone = 8 * (*(*inode).i_sb).s_blocksize;
    let mut i = 0;
    while i < (*sbi).s_zmap_blocks {
        let bh = *(*sbi).s_zmap.add(i as usize);
        spin_lock(&raw mut bitmap_lock);
        let mut j = minix_find_first_zero_bit((*bh).b_data, bits_per_zone);
        if j < bits_per_zone {
            minix_set_bit(j, (*bh).b_data);
            spin_unlock(&raw mut bitmap_lock);
            mark_buffer_dirty(bh);
            j += i * bits_per_zone + (*sbi).s_firstdatazone - 1;
            if j < (*sbi).s_firstdatazone || j >= (*sbi).s_nzones { break; }
            return j as c_int;
        }
        spin_unlock(&raw mut bitmap_lock);
        i += 1;
    }
    0
}

pub unsafe fn minix_count_free_blocks(sb: *mut super_block) -> c_ulong {
    let sbi = minix_sb(sb);
    let bits = (*sbi).s_nzones - (*sbi).s_firstdatazone + 1;
    count_free((*sbi).s_zmap, (*sb).s_blocksize, bits) << (*sbi).s_log_zone_size
}

pub unsafe fn minix_V1_raw_inode(sb: *mut super_block, mut ino: ino_t, bh: *mut *mut buffer_head) -> *mut minix_inode {
    let mut block: c_int;
    let sbi = minix_sb(sb);
    if ino == 0 || ino > (*sbi).s_ninodes {
        printk(cstr!("Bad inode number on dev %s: %ld is out of range\n"), (*sb).s_id, ino as c_long);
        return core::ptr::null_mut();
    }
    ino -= 1;
    block = 2 + (*sbi).s_imap_blocks + (*sbi).s_zmap_blocks + ino / MINIX_INODES_PER_BLOCK;
    *bh = sb_bread(sb, block);
    if (*bh).is_null() { printk(cstr!("Unable to read inode block\n")); return core::ptr::null_mut(); }
    ((*(*bh)).b_data as *mut minix_inode).add((ino % MINIX_INODES_PER_BLOCK) as usize)
}

pub unsafe fn minix_V2_raw_inode(sb: *mut super_block, mut ino: ino_t, bh: *mut *mut buffer_head) -> *mut minix2_inode {
    let sbi = minix_sb(sb);
    let per_block = (*sb).s_blocksize / core::mem::size_of::<minix2_inode>();
    *bh = core::ptr::null_mut();
    if ino == 0 || ino > (*sbi).s_ninodes { printk(cstr!("Bad inode number on dev %s: %ld is out of range\n"), (*sb).s_id, ino as c_long); return core::ptr::null_mut(); }
    ino -= 1;
    let block = 2 + (*sbi).s_imap_blocks + (*sbi).s_zmap_blocks + ino / per_block;
    *bh = sb_bread(sb, block as c_int);
    if (*bh).is_null() { printk(cstr!("Unable to read inode block\n")); return core::ptr::null_mut(); }
    ((*(*bh)).b_data as *mut minix2_inode).add((ino % per_block) as usize)
}

/* Clear the link count and mode of a deleted inode on disk. */
unsafe fn minix_clear_inode(inode: *mut inode) {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    if INODE_VERSION(inode) == MINIX_V1 {
        let raw = minix_V1_raw_inode((*inode).i_sb, (*inode).i_ino, &mut bh);
        if !raw.is_null() { (*raw).i_nlinks = 0; (*raw).i_mode = 0; }
    } else {
        let raw = minix_V2_raw_inode((*inode).i_sb, (*inode).i_ino, &mut bh);
        if !raw.is_null() { (*raw).i_nlinks = 0; (*raw).i_mode = 0; }
    }
    if !bh.is_null() { mark_buffer_dirty(bh); brelse(bh); }
}

pub unsafe fn minix_free_inode(inode: *mut inode) {
    let sb = (*inode).i_sb; let sbi = minix_sb(sb); let k = (*sb).s_blocksize_bits + 3;
    let mut ino = (*inode).i_ino; if ino < 1 || ino > (*sbi).s_ninodes { printk(cstr!("minix_free_inode: inode 0 or nonexistent inode\n")); return; }
    let bit = ino & ((1 << k) - 1); ino >>= k;
    if ino >= (*sbi).s_imap_blocks { printk(cstr!("minix_free_inode: nonexistent imap in superblock\n")); return; }
    minix_clear_inode(inode); /* clear on-disk copy */
    let bh = *(*sbi).s_imap.add(ino as usize); spin_lock(&raw mut bitmap_lock);
    if !minix_test_and_clear_bit(bit, (*bh).b_data) { printk(cstr!("minix_free_inode: bit %lu already cleared\n"), bit); }
    spin_unlock(&raw mut bitmap_lock); mark_buffer_dirty(bh);
}

pub unsafe fn minix_new_inode(dir: *const inode, mode: umode_t) -> *mut inode {
    let sb = (*dir).i_sb;
    let sbi = minix_sb(sb);
    let inode = new_inode(sb);
    let bits_per_zone = 8 * (*sb).s_blocksize;
    let mut j = bits_per_zone;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    spin_lock(&raw mut bitmap_lock);
    let mut i = 0;
    while i < (*sbi).s_imap_blocks {
        bh = *(*sbi).s_imap.add(i as usize);
        j = minix_find_first_zero_bit((*bh).b_data, bits_per_zone);
        if j < bits_per_zone { break; }
        i += 1;
    }
    if bh.is_null() || j >= bits_per_zone {
        spin_unlock(&raw mut bitmap_lock); iput(inode); return ERR_PTR(-ENOSPC);
    }
    if minix_test_and_set_bit(j, (*bh).b_data) {
        spin_unlock(&raw mut bitmap_lock);
        printk(cstr!("minix_new_inode: bit already set\n"));
        iput(inode); return ERR_PTR(-ENOSPC);
    }
    spin_unlock(&raw mut bitmap_lock);
    mark_buffer_dirty(bh);
    j += i * bits_per_zone;
    if j == 0 || j > (*sbi).s_ninodes { iput(inode); return ERR_PTR(-EFSCORRUPTED); }
    inode_init_owner(&raw mut nop_mnt_idmap, inode, dir, mode);
    (*inode).i_ino = j;
    simple_inode_init_ts(inode);
    (*inode).i_blocks = 0;
    core::ptr::write_bytes(&mut (*minix_i(inode)).u as *mut _, 0, 1);
    insert_inode_hash(inode);
    mark_inode_dirty(inode);
    inode
}

pub unsafe fn minix_count_free_inodes(sb: *mut super_block) -> c_ulong {
    let sbi = minix_sb(sb); let bits = (*sbi).s_ninodes + 1;
    count_free((*sbi).s_imap, (*sb).s_blocksize, bits) as c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
