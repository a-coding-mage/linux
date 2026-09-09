// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

// Linux kernel and Squashfs declarations are supplied by the surrounding repository.

/* Read separately compressed datablock directly into page cache */
pub unsafe fn squashfs_readpage_block(
    folio: *mut folio,
    block: u64,
    bsize: i32,
    expected: i32,
) -> i32 {
    let target_page: *mut page = &mut (*folio).page;
    let inode: *mut inode = (*(*folio).mapping).host;
    let msblk: *mut squashfs_sb_info = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    let file_end: loff_t = (i_size_read(inode).wrapping_sub(1)) >> PAGE_SHIFT;
    let mask: i32 = (1i32 << ((*msblk).block_log - PAGE_SHIFT)) - 1;
    let start_index: loff_t = (*folio).index & !(mask as loff_t);
    let mut end_index: loff_t = start_index | mask as loff_t;
    let mut index: loff_t;
    let mut i: i32;
    let mut pages: i32;
    let mut bytes: i32;
    let mut res: i32 = -ENOMEM;
    let page: *mut *mut page;
    let last_page: *mut page;
    let actor: *mut squashfs_page_actor;
    let pageaddr: *mut core::ffi::c_void;

    if end_index > file_end {
        end_index = file_end;
    }

    pages = (end_index - start_index + 1) as i32;

    page = kmalloc_array(pages as usize, core::mem::size_of::<*mut core::ffi::c_void>(), GFP_KERNEL)
        as *mut *mut page;
    if page.is_null() {
        return res;
    }

    /* Try to grab all the pages covered by the Squashfs block */
    i = 0;
    index = start_index;
    while index <= end_index {
        *page.add(i as usize) = if index == (*folio).index {
            target_page
        } else {
            grab_cache_page_nowait((*folio).mapping, index)
        };

        if (*page.add(i as usize)).is_null() {
            index += 1;
            continue;
        }

        if PageUptodate(*page.add(i as usize)) != 0 {
            unlock_page(*page.add(i as usize));
            put_page(*page.add(i as usize));
            index += 1;
            continue;
        }

        i += 1;
        index += 1;
    }

    pages = i;

    /*
     * Create a "page actor" which will kmap and kunmap the
     * page cache pages appropriately within the decompressor
     */
    actor = squashfs_page_actor_init_special(
        msblk,
        page,
        pages,
        expected,
        start_index << PAGE_SHIFT,
    );
    if actor.is_null() {
        kfree(page as *mut core::ffi::c_void);
        return res;
    }

    /* Decompress directly into the page cache buffers */
    res = squashfs_read_data((*inode).i_sb, block, bsize, core::ptr::null_mut(), actor);

    last_page = squashfs_page_actor_free(actor);

    if res < 0 {
        for n in 0..pages as usize {
            if (*page.add(n)).is_null() || *page.add(n) == target_page {
                continue;
            }
            flush_dcache_page(*page.add(n));
            unlock_page(*page.add(n));
            put_page(*page.add(n));
        }
        kfree(page as *mut core::ffi::c_void);
        return res;
    }

    if res != expected || IS_ERR(last_page) != 0 {
        res = -EIO;
        for n in 0..pages as usize {
            if (*page.add(n)).is_null() || *page.add(n) == target_page {
                continue;
            }
            flush_dcache_page(*page.add(n));
            unlock_page(*page.add(n));
            put_page(*page.add(n));
        }
        kfree(page as *mut core::ffi::c_void);
        return res;
    }

    /* Last page (if present) may have trailing bytes not filled */
    bytes = res % PAGE_SIZE;
    if end_index == file_end && !last_page.is_null() && bytes != 0 {
        pageaddr = kmap_local_page(last_page);
        core::ptr::write_bytes(
            (pageaddr as *mut u8).add(bytes as usize),
            0,
            (PAGE_SIZE - bytes) as usize,
        );
        kunmap_local(pageaddr);
    }

    /* Mark pages as uptodate, unlock and release */
    for n in 0..pages as usize {
        flush_dcache_page(*page.add(n));
        SetPageUptodate(*page.add(n));
        unlock_page(*page.add(n));
        if *page.add(n) != target_page {
            put_page(*page.add(n));
        }
    }

    kfree(page as *mut core::ffi::c_void);
    return 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
