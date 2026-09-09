// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hfsplus/bitmap.c
 *
 * Copyright (C) 2001
 * Brad Boyer (flar@allandria.com)
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 *
 * Handling of allocation file
 */

// Dependencies supplied by the kernel and hfsplus headers are intentionally
// left as external symbols for the surrounding translation unit.

const PAGE_CACHE_BITS: u32 = PAGE_SIZE * 8;

pub unsafe fn hfsplus_block_allocate(
    sb: *mut super_block,
    size: u32,
    mut offset: u32,
    max: *mut u32,
) -> i32 {
    let sbi = HFSPLUS_SB(sb);
    let mut page: *mut page;
    let mapping: *mut address_space;
    let mut pptr: *mut __be32;
    let mut curr: *mut __be32;
    let mut end: *mut __be32;
    let mut mask: u32;
    let mut start: u32;
    let mut len: u32;
    let mut n: u32;
    let mut val: __be32;
    let mut i: i32;

    len = *max;
    if len == 0 {
        return size as i32;
    }

    hfs_dbg!("size {}, offset {}, len {}\n", size, offset, len);
    mutex_lock(&mut (*sbi).alloc_mutex);
    mapping = (*(*sbi).alloc_file).i_mapping;
    page = read_mapping_page(mapping, (offset / PAGE_CACHE_BITS) as _, core::ptr::null_mut());
    if IS_ERR(page) {
        start = size;
        goto_out!(out);
    }
    pptr = kmap_local_page(page);
    curr = pptr.add(((offset & (PAGE_CACHE_BITS - 1)) / 32) as usize);
    i = (offset % 32) as i32;
    offset &= !(PAGE_CACHE_BITS - 1);
    if ((size ^ offset) / PAGE_CACHE_BITS) != 0 {
        end = pptr.add((PAGE_CACHE_BITS / 32) as usize);
    } else {
        end = pptr.add((((size + 31) & (PAGE_CACHE_BITS - 1)) / 32) as usize);
    }

    val = *curr;
    if !val.is_zero() {
        n = be32_to_cpu(val);
        mask = (1u32 << 31) >> i;
        while i < 32 {
            if (n & mask) == 0 {
                goto_found!(found);
            }
            mask >>= 1;
            i += 1;
        }
    }
    curr = curr.add(1);

    'scan: loop {
        while curr < end {
            val = *curr;
            if !val.is_zero() {
                n = be32_to_cpu(val);
                mask = 1u32 << 31;
                for i0 in 0..32 {
                    i = i0;
                    if (n & mask) == 0 {
                        goto_found!(found);
                    }
                    mask >>= 1;
                }
            }
            curr = curr.add(1);
        }
        kunmap_local(pptr as *mut _);
        offset += PAGE_CACHE_BITS;
        if offset >= size {
            hfs_dbg!("bitmap full\n");
            start = size;
            break 'scan;
        }
        page = read_mapping_page(mapping, (offset / PAGE_CACHE_BITS) as _, core::ptr::null_mut());
        if IS_ERR(page) {
            start = size;
            goto_out!(out);
        }
        curr = kmap_local_page(page);
        pptr = curr;
        if ((size ^ offset) / PAGE_CACHE_BITS) != 0 {
            end = pptr.add((PAGE_CACHE_BITS / 32) as usize);
        } else {
            end = pptr.add((((size + 31) & (PAGE_CACHE_BITS - 1)) / 32) as usize);
        }
    }
    goto_out!(out);

    found: {
        start = offset + curr.offset_from(pptr) as u32 * 32 + i as u32;
        if start >= size {
            hfs_dbg!("bitmap full\n");
            goto_out!(out);
        }
        len = core::cmp::min(size - start, len);
        loop {
            n |= mask;
            i += 1;
            if i >= 32 { break; }
            mask >>= 1;
            len -= 1;
            if len == 0 || (n & mask) != 0 { break; }
        }
        if len != 0 && i < 32 {
            *curr = cpu_to_be32(n);
            curr = curr.add(1);
            loop {
                while curr < end {
                    n = be32_to_cpu(*curr);
                    if len < 32 { break; }
                    if n != 0 { len = 32; break; }
                    *curr = cpu_to_be32(0xffffffff);
                    curr = curr.add(1);
                    len -= 32;
                }
                if curr < end { break; }
                set_page_dirty(page);
                kunmap_local(pptr as *mut _);
                offset += PAGE_CACHE_BITS;
                page = read_mapping_page(mapping, (offset / PAGE_CACHE_BITS) as _, core::ptr::null_mut());
                if IS_ERR(page) { start = size; goto_out!(out); }
                pptr = kmap_local_page(page);
                curr = pptr;
                end = pptr.add((PAGE_CACHE_BITS / 32) as usize);
            }
            mask = 1u32 << 31;
            for _ in 0..len {
                if (n & mask) != 0 { break; }
                n |= mask;
                mask >>= 1;
            }
        }
        *curr = cpu_to_be32(n);
        set_page_dirty(page);
        kunmap_local(pptr as *mut _);
        *max = offset + curr.offset_from(pptr) as u32 * 32 + i as u32 - start;
        (*sbi).free_blocks -= *max;
        hfsplus_mark_mdb_dirty(sb);
        hfs_dbg!("start {}, max {}\n", start, *max);
    }

    out: {
        mutex_unlock(&mut (*sbi).alloc_mutex);
        return start as i32;
    }
}

pub unsafe fn hfsplus_block_free(sb: *mut super_block, mut offset: u32, mut count: u32) -> i32 {
    let sbi = HFSPLUS_SB(sb);
    if count == 0 { return 0; }
    hfs_dbg!("offset {}, count {}\n", offset, count);
    if offset + count > (*sbi).total_blocks { return -ENOENT; }
    mutex_lock(&mut (*sbi).alloc_mutex);
    let mapping = (*(*sbi).alloc_file).i_mapping;
    let mut pnr = offset / PAGE_CACHE_BITS;
    let mut page = read_mapping_page(mapping, pnr as _, core::ptr::null_mut());
    if IS_ERR(page) { goto_kaboom!(kaboom); }
    let mut pptr = kmap_local_page(page);
    let mut curr = pptr.add(((offset & (PAGE_CACHE_BITS - 1)) / 32) as usize);
    let mut end = pptr.add((PAGE_CACHE_BITS / 32) as usize);
    let len = count;
    let i = offset % 32;
    if i != 0 {
        let j = 32 - i;
        let mut mask = 0xffffffffu32 << j;
        if j > count {
            mask |= 0xffffffffu32 >> (i + count);
            *curr &= cpu_to_be32(mask);
            curr = curr.add(1);
            goto_out_free!(out);
        }
        *curr &= cpu_to_be32(mask);
        curr = curr.add(1);
        count -= j;
    }
    'free: loop {
        while curr < end {
            if count < 32 { break 'free; }
            *curr = 0;
            curr = curr.add(1);
            count -= 32;
        }
        if count == 0 { break; }
        set_page_dirty(page); kunmap_local(pptr as *mut _);
        pnr += 1; page = read_mapping_page(mapping, pnr as _, core::ptr::null_mut());
        if IS_ERR(page) { goto_kaboom!(kaboom); }
        pptr = kmap_local_page(page); curr = pptr; end = pptr.add((PAGE_CACHE_BITS / 32) as usize);
    }
    if count != 0 { *curr &= cpu_to_be32(0xffffffffu32 >> count); }
    set_page_dirty(page); kunmap_local(pptr as *mut _);
    (*sbi).free_blocks += len; hfsplus_mark_mdb_dirty(sb); mutex_unlock(&mut (*sbi).alloc_mutex); return 0;
    out: { set_page_dirty(page); kunmap_local(pptr as *mut _); (*sbi).free_blocks += len; hfsplus_mark_mdb_dirty(sb); mutex_unlock(&mut (*sbi).alloc_mutex); return 0; }
    kaboom: { pr_crit!("unable to mark blocks free: error %ld\n", PTR_ERR(page)); mutex_unlock(&mut (*sbi).alloc_mutex); return -EIO; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
