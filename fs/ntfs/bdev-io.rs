// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS block device I/O.
 *
 * Copyright (c) 2026 LG Electronics Co., Ltd.
 */

// Linux block-device and NTFS dependencies are supplied by the surrounding
// translation unit.

/*
 * ntfs_bdev_read - Read data directly from block device using bio
 * @bdev: block device to read from
 * @data: destination buffer
 * @start: starting byte offset on the block device
 * @size: number of bytes to read
 *
 * Reads @size bytes starting from byte offset @start directly from the block
 * device using one or more BIOs. This function bypasses the page cache
 * completely and performs synchronous I/O with REQ_META | REQ_SYNC flags set.
 *
 * The @start offset must be sector-aligned (512 bytes). If it is not aligned,
 * the function will return -EINVAL.
 *
 * If the destination buffer @data is not a vmalloc address, it falls back
 * to the more efficient bdev_rw_virt() helper.
 *
 * Return: 0 on success, negative error code on failure.
 */
pub unsafe fn ntfs_bdev_read(
    bdev: *mut block_device,
    data: *mut std::ffi::c_char,
    start: loff_t,
    size: usize,
) -> i32 {
    let mut done: u32 = 0;
    let mut added: u32;
    let error: i32;
    let mut bio: *mut bio;
    let op: blk_opf_t;
    let sector: sector_t = (start >> SECTOR_SHIFT) as sector_t;

    if (start & (SECTOR_SIZE - 1)) != 0 {
        return -EINVAL;
    }

    op = REQ_OP_READ | REQ_META | REQ_SYNC;
    if !is_vmalloc_addr(data as *const std::ffi::c_void) {
        return bdev_rw_virt(bdev, sector, data, size, op);
    }

    bio = bio_alloc(
        bdev,
        bio_max_segs(DIV_ROUND_UP(size, PAGE_SIZE)),
        op,
        GFP_KERNEL,
    );
    (*bio).bi_iter.bi_sector = sector;

    loop {
        added = bio_add_vmalloc_chunk(bio, data.add(done as usize), size - done as usize);
        if added == 0 {
            let prev: *mut bio = bio;

            bio = bio_alloc(
                (*prev).bi_bdev,
                bio_max_segs(DIV_ROUND_UP(size - done as usize, PAGE_SIZE)),
                (*prev).bi_opf,
                GFP_KERNEL,
            );
            (*bio).bi_iter.bi_sector = bio_end_sector(prev);
            bio_chain(prev, bio);
            submit_bio(prev);
        }
        done += added;
        if done as usize >= size {
            break;
        }
    }

    error = submit_bio_wait(bio);
    bio_put(bio);

    if (op & REQ_OP_MASK) == REQ_OP_READ {
        invalidate_kernel_vmap_range(data as *mut std::ffi::c_void, size);
    }
    error
}

/*
 * ntfs_bdev_write - Update block device contents via page cache
 * @sb: super block of the mounted NTFS filesystem
 * @buf: source buffer containing data to write
 * @start: starting byte offset on the block device
 * @size: number of bytes to write
 *
 * Writes @size bytes from @buf to the block device (sb->s_bdev) starting
 * at byte offset @start. The write is performed entirely through the page
 * cache of the block device's address space.
 */
pub unsafe fn ntfs_bdev_write(
    sb: *mut super_block,
    buf: *mut std::ffi::c_void,
    start: loff_t,
    size: usize,
) -> i32 {
    let mut idx: pgoff_t;
    let idx_end: pgoff_t;
    let mut offset: loff_t;
    let end: loff_t = start + size as loff_t;
    let mut from: u32;
    let mut to: u32;
    let mut buf_off: u32 = 0;
    let mut folio: *mut folio;

    idx = (start >> PAGE_SHIFT) as pgoff_t;
    idx_end = (end >> PAGE_SHIFT) as pgoff_t;
    from = (start & !PAGE_MASK) as u32;

    let mut idx_end = idx_end;
    if idx == idx_end {
        idx_end += 1;
    }

    while idx < idx_end {
        let len: u32;

        folio = read_mapping_folio((*(*sb).s_bdev).bd_mapping, idx, std::ptr::null_mut());
        if IS_ERR(folio) {
            ntfs_error(sb, "Unable to read %ld page", idx);
            return PTR_ERR(folio);
        }

        offset = (idx << PAGE_SHIFT) as loff_t;
        to = min_t::<u32>((end - offset) as u32, PAGE_SIZE as u32);
        len = to - from;

        memcpy_to_folio(folio, from, (buf as *mut u8).add(buf_off as usize), len);
        buf_off += len;
        folio_mark_uptodate(folio);
        folio_mark_dirty(folio);
        folio_put(folio);

        idx += 1;
        from = 0;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
