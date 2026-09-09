// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/OMFS implementation are
// intentionally referenced here but not reimplemented.

use core::ffi::c_void;

extern "C" {
    fn OMFS_SB(sb: *mut super_block) -> *mut omfs_sb_info;
    fn bitmap_weight(addr: *const c_void, nbits: i32) -> i32;
    fn find_next_bit(addr: *const c_void, nbits: i32, bit: i32) -> i32;
    fn find_next_zero_bit(addr: *const c_void, nbits: i32, bit: i32) -> i32;
    fn set_bit(bit: i32, addr: *mut c_ulong);
    fn clear_bit(bit: i32, addr: *mut c_ulong);
    fn test_and_set_bit(bit: u32, addr: *mut c_ulong) -> i32;
    fn sb_bread(sb: *mut super_block, block: u64) -> *mut buffer_head;
    fn clus_to_blk(sbi: *mut omfs_sb_info, cluster: u64) -> u64;
    fn mark_buffer_dirty(bh: *mut buffer_head);
    fn brelse(bh: *mut buffer_head);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn do_div(n: *mut u64, base: i32) -> u32;
}

type c_ulong = usize;

const ENOMEM: i32 = 12;
const ENOSPC: i32 = 28;

#[repr(C)]
pub struct super_block {
    pub s_blocksize: i32,
}

#[repr(C)]
pub struct buffer_head {
    pub b_data: *mut u8,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omfs_sb_info {
    pub s_imap_size: u32,
    pub s_imap: *mut *mut c_ulong,
    pub s_bitmap_ino: u64,
    pub s_bitmap_lock: mutex,
    pub s_mirrors: i32,
    pub s_clustersize: i32,
}

unsafe fn count_run(
    mut addr: *mut *mut c_ulong,
    nbits: i32,
    mut addrlen: i32,
    mut bit: i32,
    max: i32,
) -> i32 {
    let mut count = 0;
    while addrlen > 0 {
        let x = find_next_bit(*addr, nbits, bit);
        count += x - bit;

        if x < nbits || count > max {
            return core::cmp::min(count, max);
        }

        bit = 0;
        addr = addr.add(1);
        addrlen -= 1;
    }
    core::cmp::min(count, max)
}

unsafe fn set_run(
    sb: *mut super_block,
    mut map: i32,
    nbits: i32,
    mut bit: i32,
    count: i32,
    set: i32,
) -> i32 {
    let mut err = -ENOMEM;
    let sbi = OMFS_SB(sb);
    let mut bh = sb_bread(sb, clus_to_blk(sbi, (*sbi).s_bitmap_ino) + map as u64);
    if bh.is_null() {
        return err;
    }

    let mut i = 0;
    while i < count {
        if bit >= nbits {
            bit = 0;
            map += 1;

            mark_buffer_dirty(bh);
            brelse(bh);
            bh = sb_bread(sb, clus_to_blk(sbi, (*sbi).s_bitmap_ino) + map as u64);
            if bh.is_null() {
                return err;
            }
        }

        if set != 0 {
            set_bit(bit, (*sbi).s_imap.add(map as usize).read());
            set_bit(bit, (*bh).b_data as *mut c_ulong);
        } else {
            clear_bit(bit, (*sbi).s_imap.add(map as usize).read());
            clear_bit(bit, (*bh).b_data as *mut c_ulong);
        }
        i += 1;
        bit += 1;
    }
    mark_buffer_dirty(bh);
    brelse(bh);
    err = 0;
    err
}

pub unsafe fn omfs_count_free(sb: *mut super_block) -> usize {
    let sbi = OMFS_SB(sb);
    let nbits = (*sb).s_blocksize * 8;
    let mut sum: usize = 0;
    let mut i = 0;
    while i < (*sbi).s_imap_size {
        let map = (*sbi).s_imap.add(i as usize).read();
        sum += (nbits - bitmap_weight(map as *const c_void, nbits)) as usize;
        i += 1;
    }
    sum
}

pub unsafe fn omfs_allocate_block(sb: *mut super_block, block: u64) -> i32 {
    let sbi = OMFS_SB(sb);
    let bits_per_entry = 8 * (*sb).s_blocksize;
    let mut tmp = block;
    let bit = do_div(&mut tmp, bits_per_entry) as u32;
    let map = tmp as u32;
    let mut ret = 0;

    mutex_lock(&mut (*sbi).s_bitmap_lock);
    if map >= (*sbi).s_imap_size
        || test_and_set_bit(bit, (*sbi).s_imap.add(map as usize).read()) != 0
    {
        mutex_unlock(&mut (*sbi).s_bitmap_lock);
        return ret;
    }

    if (*sbi).s_bitmap_ino > 0 {
        let bh = sb_bread(
            sb,
            clus_to_blk(sbi, (*sbi).s_bitmap_ino) + map as u64,
        );
        if bh.is_null() {
            mutex_unlock(&mut (*sbi).s_bitmap_lock);
            return ret;
        }
        set_bit(bit as i32, (*bh).b_data as *mut c_ulong);
        mark_buffer_dirty(bh);
        brelse(bh);
    }
    ret = 1;
    mutex_unlock(&mut (*sbi).s_bitmap_lock);
    ret
}

pub unsafe fn omfs_allocate_range(
    sb: *mut super_block,
    min_request: i32,
    max_request: i32,
    return_block: *mut u64,
    return_size: *mut i32,
) -> i32 {
    let sbi = OMFS_SB(sb);
    let bits_per_entry = 8 * (*sb).s_blocksize;
    let mut ret = 0;
    let mut i = 0;
    let mut run;
    let mut bit;

    mutex_lock(&mut (*sbi).s_bitmap_lock);
    while i < (*sbi).s_imap_size as i32 {
        bit = 0;
        while bit < bits_per_entry {
            let map = (*sbi).s_imap.add(i as usize).read();
            bit = find_next_zero_bit(map, bits_per_entry, bit);
            if bit == bits_per_entry {
                break;
            }
            run = count_run(
                (*sbi).s_imap.add(i as usize),
                bits_per_entry,
                (*sbi).s_imap_size as i32 - i,
                bit,
                max_request,
            );
            if run >= min_request {
                *return_block = i as u64 * bits_per_entry as u64 + bit as u64;
                *return_size = run;
                ret = set_run(sb, i, bits_per_entry, bit, run, 1);
                mutex_unlock(&mut (*sbi).s_bitmap_lock);
                return ret;
            }
            bit += run;
        }
        i += 1;
    }
    ret = -ENOSPC;
    mutex_unlock(&mut (*sbi).s_bitmap_lock);
    ret
}

pub unsafe fn omfs_clear_range(sb: *mut super_block, block: u64, count: i32) -> i32 {
    let sbi = OMFS_SB(sb);
    let bits_per_entry = 8 * (*sb).s_blocksize;
    let mut tmp = block;
    let bit = do_div(&mut tmp, bits_per_entry) as i32;
    let map = tmp as u32;

    if map >= (*sbi).s_imap_size {
        return 0;
    }
    mutex_lock(&mut (*sbi).s_bitmap_lock);
    let ret = set_run(sb, map as i32, bits_per_entry, bit, count, 0);
    mutex_unlock(&mut (*sbi).s_bitmap_lock);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
