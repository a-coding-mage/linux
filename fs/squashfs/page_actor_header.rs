/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

use core::ffi::c_void;

/* Types and functions supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct squashfs_sb_info {
    _private: [u8; 0],
}

pub type pgoff_t = usize;
pub type loff_t = i64;

extern "C" {
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct squashfs_page_actor {
    pub buffer_or_page: squashfs_page_actor_buffer_or_page,
    pub pageaddr: *mut c_void,
    pub tmp_buffer: *mut c_void,
    pub squashfs_first_page:
        Option<unsafe extern "C" fn(*mut squashfs_page_actor) -> *mut c_void>,
    pub squashfs_next_page:
        Option<unsafe extern "C" fn(*mut squashfs_page_actor) -> *mut c_void>,
    pub squashfs_finish_page: Option<unsafe extern "C" fn(*mut squashfs_page_actor)>,
    pub last_page: *mut page,
    pub pages: i32,
    pub length: i32,
    pub next_page: i32,
    pub alloc_buffer: i32,
    pub returned_pages: i32,
    pub next_index: pgoff_t,
}

#[repr(C)]
pub union squashfs_page_actor_buffer_or_page {
    pub buffer: *mut *mut c_void,
    pub page: *mut *mut page,
}

extern "C" {
    pub fn squashfs_page_actor_init(
        buffer: *mut *mut c_void,
        pages: i32,
        length: i32,
    ) -> *mut squashfs_page_actor;
    pub fn squashfs_page_actor_init_special(
        msblk: *mut squashfs_sb_info,
        page: *mut *mut page,
        pages: i32,
        length: i32,
        start_index: loff_t,
    ) -> *mut squashfs_page_actor;
}

#[inline]
pub unsafe fn squashfs_page_actor_free(actor: *mut squashfs_page_actor) -> *mut page {
    let last_page = if (*actor).next_page == (*actor).pages {
        (*actor).last_page
    } else {
        /* ERR_PTR(-EIO), preserving the kernel error-pointer convention. */
        (-5isize) as *mut page
    };

    kfree((*actor).tmp_buffer);
    kfree(actor as *mut c_void);

    last_page
}

#[inline]
pub unsafe fn squashfs_first_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    ((*actor).squashfs_first_page.expect("null callback"))(actor)
}

#[inline]
pub unsafe fn squashfs_next_page(actor: *mut squashfs_page_actor) -> *mut c_void {
    ((*actor).squashfs_next_page.expect("null callback"))(actor)
}

#[inline]
pub unsafe fn squashfs_finish_page(actor: *mut squashfs_page_actor) {
    ((*actor).squashfs_finish_page.expect("null callback"))(actor);
}

#[inline]
pub unsafe fn squashfs_actor_nobuff(actor: *mut squashfs_page_actor) {
    (*actor).alloc_buffer = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
