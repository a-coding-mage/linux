/*
 *  linux/fs/hfs/bitmap.c
 *
 * Copyright (C) 1996-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * Based on GPLed code Copyright (C) 1995  Michael Dreher
 *
 * This file contains the code to modify the volume bitmap:
 * search/set/clear bits.
 */

/* Dependency declarations and macros are supplied by hfs_fs.h. */

unsafe fn hfs_find_set_zero_bits(
    bitmap: *mut u32,
    size: u32,
    offset: u32,
    max: *mut u32,
) -> u32 {
    let mut curr: *mut u32;
    let end: *mut u32;
    let mut mask: u32;
    let mut start: u32;
    let mut len: u32;
    let mut n: u32;
    let mut val: u32;
    let mut i: i32;

    len = *max;
    if len == 0 {
        return size;
    }

    curr = bitmap.add((offset / 32) as usize);
    end = bitmap.add(((size + 31) / 32) as usize);

    /* scan the first partial u32 for zero bits */
    val = *curr;
    if !val != 0 {
        n = be32_to_cpu(val);
        i = (offset % 32) as i32;
        mask = (1u32 << 31) >> i;
        while i < 32 {
            if (n & mask) == 0 {
                break;
            }
            mask >>= 1;
            i += 1;
        }
        if i < 32 {
            start = curr.offset_from(bitmap) as u32 * 32 + i as u32;
            return hfs_find_set_zero_bits_found(bitmap, curr, size, max, start, n, mask, i);
        }
    }

    /* scan complete u32s for the first zero bit */
    curr = curr.add(1);
    while curr < end {
        val = *curr;
        if !val != 0 {
            n = be32_to_cpu(val);
            mask = 1u32 << 31;
            i = 0;
            while i < 32 {
                if (n & mask) == 0 {
                    start = curr.offset_from(bitmap) as u32 * 32 + i as u32;
                    return hfs_find_set_zero_bits_found(bitmap, curr, size, max, start, n, mask, i);
                }
                mask >>= 1;
                i += 1;
            }
        }
        curr = curr.add(1);
    }
    size
}

unsafe fn hfs_find_set_zero_bits_found(
    bitmap: *mut u32,
    mut curr: *mut u32,
    size: u32,
    max: *mut u32,
    start: u32,
    mut n: u32,
    mut mask: u32,
    mut i: i32,
) -> u32 {
    if start >= size {
        return start;
    }
    let mut len = core::cmp::min(size - start, *max);
    loop {
        n |= mask;
        i += 1;
        if i >= 32 {
            break;
        }
        mask >>= 1;
        len -= 1;
        if len == 0 || (n & mask) != 0 {
            break;
        }
    }
    if i < 32 && len == 0 {
        *curr = cpu_to_be32(n);
        *max = curr.offset_from(bitmap) as u32 * 32 + i as u32 - start;
        return start;
    }
    if i >= 32 {
        len -= 1;
    }
    if len == 0 {
        *curr = cpu_to_be32(n);
        *max = curr.offset_from(bitmap) as u32 * 32 + i as u32 - start;
        return start;
    }
    *curr = cpu_to_be32(n);
    curr = curr.add(1);
    loop {
        n = be32_to_cpu(*curr);
        if len < 32 {
            break;
        }
        if n != 0 {
            len = 32;
            break;
        }
        *curr = cpu_to_be32(0xffff_ffff);
        curr = curr.add(1);
        len -= 32;
    }
    mask = 1u32 << 31;
    i = 0;
    while (i as u32) < len {
        if (n & mask) != 0 {
            break;
        }
        n |= mask;
        mask >>= 1;
        i += 1;
    }
    *curr = cpu_to_be32(n);
    *max = curr.offset_from(bitmap) as u32 * 32 + i as u32 - start;
    start
}

pub unsafe fn hfs_vbm_search_free(sb: *mut super_block, goal: u32, num_bits: *mut u32) -> u32 {
    let bitmap: *mut u32;
    let mut pos: u32;

    if *num_bits == 0 {
        return 0;
    }

    mutex_lock(&mut (*HFS_SB(sb)).bitmap_lock);
    bitmap = (*HFS_SB(sb)).bitmap as *mut u32;

    pos = hfs_find_set_zero_bits(bitmap, (*HFS_SB(sb)).fs_ablocks, goal, num_bits);
    if pos >= (*HFS_SB(sb)).fs_ablocks {
        if goal != 0 {
            pos = hfs_find_set_zero_bits(bitmap, goal, 0, num_bits);
        }
        if pos >= (*HFS_SB(sb)).fs_ablocks {
            *num_bits = 0;
            pos = 0;
            mutex_unlock(&mut (*HFS_SB(sb)).bitmap_lock);
            return pos;
        }
    }

    hfs_dbg!("pos %u, num_bits %u\n", pos, *num_bits);
    (*HFS_SB(sb)).free_ablocks -= *num_bits;
    hfs_bitmap_dirty(sb);
    mutex_unlock(&mut (*HFS_SB(sb)).bitmap_lock);
    pos
}

pub unsafe fn hfs_clear_vbm_bits(sb: *mut super_block, start: u16, mut count: u16) -> i32 {
    let mut curr: *mut u32;
    let mut mask: u32;
    let mut i: i32;
    let len: u16;

    if count == 0 {
        return 0;
    }

    hfs_dbg!("start %u, count %u\n", start, count);
    if (start as u32 + count as u32) > (*HFS_SB(sb)).fs_ablocks {
        return -2;
    }

    mutex_lock(&mut (*HFS_SB(sb)).bitmap_lock);
    curr = ((*HFS_SB(sb)).bitmap as *mut u32).add((start / 32) as usize);
    len = count;

    i = (start % 32) as i32;
    if i != 0 {
        let j = 32 - i;
        mask = 0xffff_ffffu32 << j;
        if j > count as i32 {
            mask |= 0xffff_ffffu32 >> (i + count as i32);
            *curr &= cpu_to_be32(mask);
            (*HFS_SB(sb)).free_ablocks += len as u32;
            mutex_unlock(&mut (*HFS_SB(sb)).bitmap_lock);
            hfs_bitmap_dirty(sb);
            return 0;
        }
        *curr &= cpu_to_be32(mask);
        curr = curr.add(1);
        count -= j as u16;
    }

    while count >= 32 {
        *curr = 0;
        curr = curr.add(1);
        count -= 32;
    }
    if count != 0 {
        mask = 0xffff_ffffu32 >> count;
        *curr &= cpu_to_be32(mask);
    }
    (*HFS_SB(sb)).free_ablocks += len as u32;
    mutex_unlock(&mut (*HFS_SB(sb)).bitmap_lock);
    hfs_bitmap_dirty(sb);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
