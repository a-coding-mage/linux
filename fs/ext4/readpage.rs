// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/ext4/readpage.c
 *
 * Copyright (C) 2002, Linus Torvalds.
 * Copyright (C) 2015, Google, Inc.
 *
 * This was originally taken from fs/mpage.c
 */

// Linux kernel headers and "ext4.h" / trace event dependencies are supplied externally.

const NUM_VERITY_WORKS: usize = 128;

static mut ext4_verity_work_cache: *mut kmem_cache = core::ptr::null_mut();
static mut ext4_verity_work_pool: *mut mempool_t = core::ptr::null_mut();

#[repr(C)]
struct ext4_verity_work {
    bio: *mut bio,
    vi: *mut fsverity_info,
    work: work_struct,
}

unsafe fn __read_end_io(bio: *mut bio) {
    let mut fi: folio_iter;
    bio_for_each_folio_all!(fi, bio, {
        folio_end_read(fi.folio, (*bio).bi_status == 0);
    });
    if !(*bio).bi_private.is_null() {
        mempool_free((*bio).bi_private, ext4_verity_work_pool);
    }
    bio_put(bio);
}

unsafe fn verity_work(work: *mut work_struct) {
    let ctx = container_of!(work, ext4_verity_work, work);
    let bio = (*ctx).bio;
    let vi = (*ctx).vi;

    /* Free the ext4_verity_work right away, since it's no longer needed. */
    mempool_free(ctx as *mut core::ffi::c_void, ext4_verity_work_pool);
    (*bio).bi_private = core::ptr::null_mut();

    fsverity_verify_bio(vi, bio);
    __read_end_io(bio);
}

unsafe fn mpage_end_io(bio: *mut bio) {
    if IS_ENABLED!(CONFIG_FS_VERITY) && !(*bio).bi_private.is_null() && (*bio).bi_status == 0 {
        let ctx = (*bio).bi_private as *mut ext4_verity_work;
        INIT_WORK!(&mut (*ctx).work, verity_work);
        fsverity_enqueue_verify_work(&mut (*ctx).work);
        return;
    }
    __read_end_io(bio);
}

unsafe fn ext4_set_verity_work(bio: *mut bio, vi: *mut fsverity_info) {
    if !vi.is_null() {
        let ctx = mempool_alloc(ext4_verity_work_pool, GFP_NOFS) as *mut ext4_verity_work;
        (*ctx).bio = bio;
        (*ctx).vi = vi;
        (*bio).bi_private = ctx as *mut core::ffi::c_void;
    }
}

#[inline]
unsafe fn ext4_readpage_limit(inode: *mut inode) -> loff_t {
    if IS_ENABLED!(CONFIG_FS_VERITY) && IS_VERITY(inode) {
        return (*(*inode).i_sb).s_maxbytes;
    }
    i_size_read(inode)
}

unsafe fn ext4_mpage_readpages(
    inode: *mut inode,
    vi: *mut fsverity_info,
    rac: *mut readahead_control,
    mut folio: *mut folio,
) -> i32 {
    let mut bio: *mut bio = core::ptr::null_mut();
    let mut last_block_in_bio: sector_t = 0;
    let blkbits = (*inode).i_blkbits;
    let blocksize = 1u64 << blkbits;
    let mut block_in_file: sector_t;
    let mut last_block: sector_t;
    let mut last_block_in_file: sector_t;
    let mut first_block: sector_t = 0;
    let mut pos: loff_t;
    let mut page_block: u32;
    let bdev = (*(*inode).i_sb).s_bdev;
    let mut length: i32;
    let mut relative_block: u32 = 0;
    let mut map = ext4_map_blocks { m_pblk: 0, m_lblk: 0, m_len: 0, m_flags: 0 };
    let mut nr_pages = if !rac.is_null() { readahead_count(rac) } else { folio_nr_pages(folio) };
    while nr_pages != 0 {
        let mut fully_mapped = true;
        let mut first_hole: u32;
        let blocks_per_folio: u32;
        let mut folio_pages: u32;

        if !rac.is_null() { folio = readahead_folio(rac); }
        folio_pages = folio_nr_pages(folio);
        prefetchw!(&mut (*folio).flags);
        if !folio_buffers(folio).is_null() { goto!(confused); }
        blocks_per_folio = (folio_size(folio) >> blkbits) as u32;
        first_hole = blocks_per_folio;
        pos = folio_pos(folio);
        block_in_file = (pos >> blkbits) as sector_t;
        last_block = EXT4_PG_TO_LBLK!(inode, (*folio).index + nr_pages);
        last_block_in_file = ((ext4_readpage_limit(inode) + blocksize as i64 - 1) >> blkbits) as sector_t;
        if last_block > last_block_in_file { last_block = last_block_in_file; }
        page_block = 0;

        if (map.m_flags & EXT4_MAP_MAPPED) != 0 && block_in_file > map.m_lblk && block_in_file < map.m_lblk + map.m_len {
            let map_offset = block_in_file - map.m_lblk;
            let last = map.m_len - map_offset;
            first_block = map.m_pblk + map_offset;
            relative_block = 0;
            loop {
                if relative_block == last { map.m_flags &= !EXT4_MAP_MAPPED; break; }
                if page_block == blocks_per_folio { break; }
                page_block += 1; block_in_file += 1; relative_block += 1;
            }
        }

        while page_block < blocks_per_folio {
            if block_in_file < last_block {
                map.m_lblk = block_in_file; map.m_len = last_block - block_in_file;
                if ext4_map_blocks(core::ptr::null_mut(), inode, &mut map, 0) < 0 {
                    folio_zero_segment(folio, 0, folio_size(folio)); folio_unlock(folio); goto!(next_page);
                }
            }
            if (map.m_flags & EXT4_MAP_MAPPED) == 0 {
                fully_mapped = false; if first_hole == blocks_per_folio { first_hole = page_block; }
                page_block += 1; block_in_file += 1; continue;
            }
            if first_hole != blocks_per_folio { goto!(confused); }
            if page_block == 0 { first_block = map.m_pblk; } else if first_block + page_block as u64 != map.m_pblk { goto!(confused); }
            relative_block = 0;
            loop {
                if relative_block == map.m_len { map.m_flags &= !EXT4_MAP_MAPPED; break; }
                if page_block == blocks_per_folio { break; }
                page_block += 1; block_in_file += 1; relative_block += 1;
            }
        }
        if first_hole != blocks_per_folio {
            folio_zero_segment(folio, (first_hole << blkbits) as usize, folio_size(folio));
            if first_hole == 0 { if !vi.is_null() && !fsverity_verify_folio(vi, folio) { goto!(set_error_page); } folio_end_read(folio, true); continue; }
        } else if fully_mapped { folio_set_mappedtodisk(folio); }
        if !bio.is_null() && (last_block_in_bio != first_block - 1 || !fscrypt_mergeable_bio(bio, inode, pos)) { goto!(submit_and_realloc); }
        if bio.is_null() {
            bio = bio_alloc(bdev, bio_max_segs(nr_pages), REQ_OP_READ, GFP_KERNEL);
            fscrypt_set_bio_crypt_ctx(bio, inode, pos, GFP_KERNEL); ext4_set_verity_work(bio, vi);
            (*bio).bi_iter.bi_sector = first_block << (blkbits - 9); (*bio).bi_end_io = Some(mpage_end_io);
            if !rac.is_null() { (*bio).bi_opf |= REQ_RAHEAD; }
        }
        length = (first_hole << blkbits) as i32;
        if !bio_add_folio(bio, folio, length, 0) { goto!(submit_and_realloc); }
        if ((map.m_flags & EXT4_MAP_BOUNDARY) != 0 && relative_block == map.m_len) || first_hole != blocks_per_folio { blk_crypto_submit_bio(bio); bio = core::ptr::null_mut(); } else { last_block_in_bio = first_block + blocks_per_folio as u64 - 1; }
        continue;
        submit_and_realloc: blk_crypto_submit_bio(bio); bio = core::ptr::null_mut(); continue;
        confused: if !bio.is_null() { blk_crypto_submit_bio(bio); bio = core::ptr::null_mut(); } if !folio_test_uptodate(folio) { block_read_full_folio(folio, ext4_get_block); } else { folio_unlock(folio); }
        next_page:;
        nr_pages -= folio_pages;
    }
    if !bio.is_null() { blk_crypto_submit_bio(bio); }
    0
}

pub unsafe fn ext4_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let inode = (*(*folio).mapping).host; let mut vi: *mut fsverity_info = core::ptr::null_mut();
    trace_ext4_read_folio(inode, folio);
    if ext4_has_inline_data(inode) { let ret = ext4_readpage_inline(inode, folio); if ret != -EAGAIN { return ret; } }
    if (*folio).index < DIV_ROUND_UP!((*inode).i_size, PAGE_SIZE) { vi = fsverity_get_info(inode); }
    if !vi.is_null() { fsverity_readahead(vi, (*folio).index, folio_nr_pages(folio)); }
    ext4_mpage_readpages(inode, vi, core::ptr::null_mut(), folio)
}

pub unsafe fn ext4_readahead(rac: *mut readahead_control) {
    let inode = (*(*rac).mapping).host; let mut vi: *mut fsverity_info = core::ptr::null_mut();
    if ext4_has_inline_data(inode) { return; }
    if readahead_index(rac) < DIV_ROUND_UP!((*inode).i_size, PAGE_SIZE) { vi = fsverity_get_info(inode); }
    if !vi.is_null() { fsverity_readahead(vi, readahead_index(rac), readahead_count(rac)); }
    ext4_mpage_readpages(inode, vi, rac, core::ptr::null_mut());
}

pub unsafe fn ext4_init_verity_caches() -> i32 {
    if !IS_ENABLED!(CONFIG_FS_VERITY) { return 0; }
    ext4_verity_work_cache = KMEM_CACHE!(ext4_verity_work, SLAB_RECLAIM_ACCOUNT);
    if ext4_verity_work_cache.is_null() { return -ENOMEM; }
    ext4_verity_work_pool = mempool_create_slab_pool(NUM_VERITY_WORKS, ext4_verity_work_cache);
    if ext4_verity_work_pool.is_null() { kmem_cache_destroy(ext4_verity_work_cache); return -ENOMEM; }
    0
}

pub unsafe fn ext4_exit_verity_caches() {
    if !IS_ENABLED!(CONFIG_FS_VERITY) { return; }
    mempool_destroy(ext4_verity_work_pool); kmem_cache_destroy(ext4_verity_work_cache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
