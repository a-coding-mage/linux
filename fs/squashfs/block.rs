// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * block.c
 */

/* This file implements the low-level routines to read and decompress
 * datablocks and metadata blocks. */

// Dependencies supplied by the surrounding kernel/Squashfs translation.

unsafe fn copy_bio_to_actor(
    bio: *mut bio,
    actor: *mut squashfs_page_actor,
    mut offset: i32,
    req_length: i32,
) -> i32 {
    let mut actor_addr: *mut core::ffi::c_void;
    let mut iter_all: bvec_iter_all = core::mem::zeroed();
    let mut bvec: *mut bio_vec = bvec_init_iter_all(&mut iter_all);
    let mut copied_bytes = 0;
    let mut actor_offset = 0;

    squashfs_actor_nobuff(actor);
    actor_addr = squashfs_first_page(actor);

    if WARN_ON_ONCE(!bio_next_segment(bio, &mut iter_all)) {
        return 0;
    }

    while copied_bytes < req_length {
        let mut bytes_to_copy = core::cmp::min((*bvec).bv_len - offset, PAGE_SIZE - actor_offset);
        bytes_to_copy = core::cmp::min(bytes_to_copy, req_length - copied_bytes);
        if !IS_ERR(actor_addr) {
            memcpy(
                (actor_addr as *mut u8).add(actor_offset as usize) as *mut core::ffi::c_void,
                (bvec_virt(bvec) as *mut u8).add(offset as usize) as *const core::ffi::c_void,
                bytes_to_copy as usize,
            );
        }

        actor_offset += bytes_to_copy;
        copied_bytes += bytes_to_copy;
        offset += bytes_to_copy;

        if actor_offset >= PAGE_SIZE {
            actor_addr = squashfs_next_page(actor);
            if actor_addr.is_null() {
                break;
            }
            actor_offset = 0;
        }
        if offset >= (*bvec).bv_len {
            if !bio_next_segment(bio, &mut iter_all) {
                break;
            }
            offset = 0;
        }
    }
    squashfs_finish_page(actor);
    copied_bytes
}

unsafe fn squashfs_bio_read_cached(
    fullbio: *mut bio,
    cache_mapping: *mut address_space,
    index: u64,
    length: i32,
    read_start: u64,
    read_end: u64,
    page_count: i32,
) -> i32 {
    let mut head_to_cache: *mut folio = core::ptr::null_mut();
    let mut tail_to_cache: *mut folio = core::ptr::null_mut();
    let bdev = (*fullbio).bi_bdev;
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut fi: folio_iter = core::mem::zeroed();
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut idx = 0;
    let mut err = 0;

    // CONFIG_SQUASHFS_COMP_CACHE_FULL: conditionally allocate zeroed cache_folios.
    let mut cache_folios: *mut *mut folio = core::ptr::null_mut();

    while bio_for_each_folio_all(&mut fi, fullbio) {
        let folio = fi.folio;
        if (*folio).mapping == cache_mapping {
            idx += 1;
            continue;
        }
        if idx == 0 && index != read_start {
            head_to_cache = folio;
        } else if idx == page_count - 1 && index + length as u64 != read_end {
            tail_to_cache = folio;
        }

        if bio.is_null() || idx != end_idx {
            let new_bio = bio_alloc_clone(bdev, fullbio, GFP_NOIO, &fs_bio_set);
            if !bio.is_null() {
                bio_trim(bio, start_idx * PAGE_SECTORS, (end_idx - start_idx) * PAGE_SECTORS);
                bio_chain(bio, new_bio);
                submit_bio(bio);
            }
            bio = new_bio;
            start_idx = idx;
        }
        idx += 1;
        end_idx = idx;
    }

    if !bio.is_null() {
        bio_trim(bio, start_idx * PAGE_SECTORS, (end_idx - start_idx) * PAGE_SECTORS);
        err = submit_bio_wait(bio);
        bio_put(bio);
    }
    if err != 0 {
        return err;
    }

    if !head_to_cache.is_null() {
        let ret = filemap_add_folio(cache_mapping, head_to_cache, read_start >> PAGE_SHIFT, GFP_NOIO);
        if ret == 0 { folio_mark_uptodate(head_to_cache); folio_unlock(head_to_cache); }
    }
    if !tail_to_cache.is_null() {
        let ret = filemap_add_folio(cache_mapping, tail_to_cache, (read_end >> PAGE_SHIFT) - 1, GFP_NOIO);
        if ret == 0 { folio_mark_uptodate(tail_to_cache); folio_unlock(tail_to_cache); }
    }

    // CONFIG_SQUASHFS_COMP_CACHE_FULL: cache every page in the BIO when enabled.
    let _ = cache_folios;
    0
}

unsafe fn squashfs_get_cache_page(mapping: *mut address_space, index: pgoff_t) -> *mut page {
    if mapping.is_null() { return core::ptr::null_mut(); }
    let page = find_get_page(mapping, index);
    if page.is_null() { return core::ptr::null_mut(); }
    if !PageUptodate(page) {
        put_page(page);
        return core::ptr::null_mut();
    }
    page
}

unsafe fn squashfs_bio_read(
    sb: *mut super_block, index: u64, length: i32,
    biop: *mut *mut bio, block_offset: *mut i32,
) -> i32 {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let cache_mapping = (*msblk).cache_mapping;
    let read_start = round_down(index, (*msblk).devblksize);
    let block = read_start >> (*msblk).devblksize_log2;
    let read_end = round_up(index + length as u64, (*msblk).devblksize);
    let block_end = read_end >> (*msblk).devblksize_log2;
    let mut offset = (read_start - round_down(index, PAGE_SIZE as u64)) as i32;
    let mut total_len = ((block_end - block) << (*msblk).devblksize_log2) as i32;
    let page_count = (total_len + offset + PAGE_SIZE - 1) / PAGE_SIZE;
    let bio = bio_kmalloc(page_count, GFP_NOIO);
    if bio.is_null() { return -ENOMEM; }
    bio_init_inline(bio, (*sb).s_bdev, page_count, REQ_OP_READ);
    (*bio).bi_iter.bi_sector = block * ((*msblk).devblksize >> SECTOR_SHIFT);

    for i in 0..page_count {
        let len = core::cmp::min(PAGE_SIZE - offset, total_len) as u32;
        let page = {
            let p = squashfs_get_cache_page(cache_mapping, (read_start >> PAGE_SHIFT) as pgoff_t + i as pgoff_t);
            if p.is_null() { alloc_page(GFP_NOIO) } else { p }
        };
        if page.is_null() { bio_free_pages(bio); bio_uninit(bio); kfree(bio); return -ENOMEM; }
        __bio_add_page(bio, page, len, offset as u32);
        offset = 0;
        total_len -= len as i32;
    }

    let error = if !cache_mapping.is_null() {
        squashfs_bio_read_cached(bio, cache_mapping, index, length, read_start, read_end, page_count)
    } else { submit_bio_wait(bio) };
    if error != 0 { bio_free_pages(bio); bio_uninit(bio); kfree(bio); return error; }
    *biop = bio;
    *block_offset = (index & ((1u64 << (*msblk).devblksize_log2) - 1)) as i32;
    0
}

pub unsafe fn squashfs_read_data(
    sb: *mut super_block, mut index: u64, mut length: i32,
    next_index: *mut u64, output: *mut squashfs_page_actor,
) -> i32 {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut bio: *mut bio = core::ptr::null_mut();
    let compressed: bool;
    let mut res: i32;
    let mut offset = 0;

    if length != 0 {
        compressed = SQUASHFS_COMPRESSED_BLOCK(length);
        length = SQUASHFS_COMPRESSED_SIZE_BLOCK(length);
        TRACE!("Block @ 0x%llx, %scompressed size %d, src size %d\n", index, if compressed { "" } else { "un" }, length, (*output).length);
    } else {
        let mut iter_all: bvec_iter_all = core::mem::zeroed();
        let mut bvec = bvec_init_iter_all(&mut iter_all);
        if index + 2 > (*msblk).bytes_used { res = -EIO; return squashfs_read_data_error(msblk, index, res); }
        res = squashfs_bio_read(sb, index, 2, &mut bio, &mut offset);
        if res != 0 { return squashfs_read_data_error(msblk, index, res); }
        if WARN_ON_ONCE(!bio_next_segment(bio, &mut iter_all)) { res = -EIO; bio_free_pages(bio); bio_uninit(bio); kfree(bio); return squashfs_read_data_error(msblk, index, res); }
        let data = bvec_virt(bvec) as *const u8;
        length = *data.add(offset as usize) as i32;
        if offset < (*bvec).bv_len - 1 { length |= (*data.add(offset as usize + 1) as i32) << 8; }
        else { if WARN_ON_ONCE(!bio_next_segment(bio, &mut iter_all)) { res = -EIO; bio_free_pages(bio); bio_uninit(bio); kfree(bio); return squashfs_read_data_error(msblk, index, res); } bvec = bvec_init_iter_all(&mut iter_all); length |= (*(bvec_virt(bvec) as *const u8) as i32) << 8; }
        bio_free_pages(bio); bio_uninit(bio); kfree(bio); bio = core::ptr::null_mut();
        compressed = SQUASHFS_COMPRESSED(length); length = SQUASHFS_COMPRESSED_SIZE(length); index += 2;
        TRACE!("Block @ 0x%llx, %scompressed size %d\n", index - 2, if compressed { "" } else { "un" }, length);
    }
    if length <= 0 || length > (*output).length || index + length as u64 > (*msblk).bytes_used { res = -EIO; return squashfs_read_data_error(msblk, index, res); }
    if !next_index.is_null() { *next_index = index + length as u64; }
    res = squashfs_bio_read(sb, index, length, &mut bio, &mut offset);
    if res == 0 { res = if compressed { if (*msblk).stream.is_null() { -EIO } else { (*msblk).thread_ops.decompress(msblk, bio, offset, length, output) } } else { copy_bio_to_actor(bio, output, offset, length) }; }
    if !bio.is_null() { bio_free_pages(bio); bio_uninit(bio); kfree(bio); }
    if res < 0 { ERROR!("Failed to read block 0x%llx: %d\n", index, res); if (*msblk).panic_on_errors { panic!("squashfs read failed"); } }
    res
}

unsafe fn squashfs_read_data_error(msblk: *mut squashfs_sb_info, index: u64, res: i32) -> i32 {
    ERROR!("Failed to read block 0x%llx: %d\n", index, res);
    if (*msblk).panic_on_errors { panic!("squashfs read failed"); }
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
