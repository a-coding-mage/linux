// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

use core::ffi::c_void;

// Linux kernel and SquashFS declarations are supplied by the surrounding
// translation unit.

/*
 * This file contains implementations of page_actor for decompressing into
 * an intermediate buffer, and for decompressing directly into the
 * page cache.
 *
 * Calling code should avoid sleeping between calls to squashfs_first_page()
 * and squashfs_finish_page().
 */

/* Implementation of page_actor for decompressing into intermediate buffer */
unsafe fn cache_first_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    (*actor).next_page = 1;
    (*actor).buffer.read()
}

unsafe fn cache_next_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    if (*actor).next_page == (*actor).pages {
        return core::ptr::null_mut();
    }

    let page = (*actor).buffer.add((*actor).next_page).read();
    (*actor).next_page += 1;
    page
}

unsafe fn cache_finish_page(_actor: *mut squashfs_page_actor) {
    /* empty */
}

unsafe fn squashfs_page_actor_init(
    buffer: *mut *mut c_void,
    pages: i32,
    length: i32,
) -> *mut squashfs_page_actor {
    let actor = kmalloc_obj_squashfs_page_actor();

    if actor.is_null() {
        return core::ptr::null_mut();
    }

    (*actor).length = if length != 0 { length } else { pages * PAGE_SIZE };
    (*actor).buffer = buffer;
    (*actor).pages = pages;
    (*actor).next_page = 0;
    (*actor).tmp_buffer = core::ptr::null_mut();
    (*actor).squashfs_first_page = Some(cache_first_page);
    (*actor).squashfs_next_page = Some(cache_next_page);
    (*actor).squashfs_finish_page = Some(cache_finish_page);
    actor
}

/* Implementation of page_actor for decompressing directly into page cache. */
unsafe fn page_next_index(actor: *mut squashfs_page_actor) -> loff_t {
    page_folio((*actor).page.add((*actor).next_page).read()).index
}

unsafe fn handle_next_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    let max_pages = ((*actor).length + PAGE_SIZE - 1) >> PAGE_SHIFT;

    if (*actor).returned_pages == max_pages {
        return core::ptr::null_mut();
    }

    if (*actor).next_page == (*actor).pages
        || (*actor).next_index != page_next_index(actor)
    {
        (*actor).next_index += 1;
        (*actor).returned_pages += 1;
        (*actor).last_page = core::ptr::null_mut();
        return if (*actor).alloc_buffer {
            (*actor).tmp_buffer
        } else {
            ERR_PTR(-ENOMEM)
        };
    }

    (*actor).next_index += 1;
    (*actor).returned_pages += 1;
    (*actor).last_page = (*actor).page.add((*actor).next_page).read();
    (*actor).pageaddr = kmap_local_page((*actor).page.add((*actor).next_page).read());
    (*actor).next_page += 1;
    (*actor).pageaddr
}

unsafe fn direct_first_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    handle_next_page(actor)
}

unsafe fn direct_next_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    if !(*actor).pageaddr.is_null() {
        kunmap_local((*actor).pageaddr);
        (*actor).pageaddr = core::ptr::null_mut();
    }

    handle_next_page(actor)
}

unsafe fn direct_finish_page(actor: *mut squashfs_page_actor) {
    if !(*actor).pageaddr.is_null() {
        kunmap_local((*actor).pageaddr);
    }
}

unsafe fn squashfs_page_actor_init_special(
    msblk: *mut squashfs_sb_info,
    page: *mut *mut page,
    pages: i32,
    length: i32,
    start_index: loff_t,
) -> *mut squashfs_page_actor {
    let actor = kmalloc_obj_squashfs_page_actor();

    if actor.is_null() {
        return core::ptr::null_mut();
    }

    if (*(*msblk).decompressor).alloc_buffer {
        (*actor).tmp_buffer = kmalloc(PAGE_SIZE, GFP_KERNEL);

        if (*actor).tmp_buffer.is_null() {
            kfree(actor);
            return core::ptr::null_mut();
        }
    } else {
        (*actor).tmp_buffer = core::ptr::null_mut();
    }

    (*actor).length = if length != 0 { length } else { pages * PAGE_SIZE };
    (*actor).page = page;
    (*actor).pages = pages;
    (*actor).next_page = 0;
    (*actor).returned_pages = 0;
    (*actor).next_index = start_index >> PAGE_SHIFT;
    (*actor).pageaddr = core::ptr::null_mut();
    (*actor).last_page = core::ptr::null_mut();
    (*actor).alloc_buffer = (*(*msblk).decompressor).alloc_buffer;
    (*actor).squashfs_first_page = Some(direct_first_page);
    (*actor).squashfs_next_page = Some(direct_next_page);
    (*actor).squashfs_finish_page = Some(direct_finish_page);
    actor
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
