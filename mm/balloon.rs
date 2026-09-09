// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common interface for implementing a memory balloon, including support
 * for migration of pages inflated in a memory balloon.
 *
 * Copyright (C) 2012, Red Hat, Inc.  Rafael Aquini <aquini@redhat.com>
 */

// Kernel declarations supplied by the corresponding Linux headers.
use core::ffi::c_void;

extern "C" {
    static mut balloon_pages_lock: c_void;
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct page {
    pub lru: list_head,
}

#[repr(C)]
pub struct balloon_dev_info {
    pub pages: list_head,
    pub isolated_pages: usize,
    pub adjust_managed_page_count: Option<unsafe extern "C" fn(*mut page, i32)>,
    #[cfg(CONFIG_BALLOON_MIGRATION)]
    pub migratepage: unsafe extern "C" fn(*mut balloon_dev_info, *mut page, *mut page, migrate_mode_t) -> i32,
}

pub type gfp_t = u32;
pub type isolate_mode_t = u32;
pub type migrate_mode_t = u32;

extern "C" {
    fn __SetPageOffline(page: *mut page);
    fn SetPageMovableOps(page: *mut page);
    fn set_page_private(page: *mut page, private: usize);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn adjust_managed_page_count(page: *mut page, count: i32);
    fn __count_vm_event(event: i32);
    fn inc_node_page_state(page: *mut page, item: i32);
    fn dec_node_page_state(page: *mut page, item: i32);
    fn alloc_page(flags: gfp_t) -> *mut page;
    fn page_private(page: *mut page) -> usize;
    fn get_page(page: *mut page);
    fn put_page(page: *mut page);
    fn page_zone(page: *mut page) -> *mut c_void;
    fn set_movable_ops(ops: *const movable_operations, pgtype: i32) -> i32;
    fn BUG() -> !;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

const __GFP_NOMEMALLOC: gfp_t = 1 << 20;
const __GFP_NORETRY: gfp_t = 1 << 21;
const __GFP_NOWARN: gfp_t = 1 << 22;
const GFP_HIGHUSER_MOVABLE: gfp_t = 1 << 23;
const GFP_HIGHUSER: gfp_t = 1 << 24;
const BALLOON_INFLATE: i32 = 0;
const BALLOON_DEFLATE: i32 = 1;
const BALLOON_MIGRATE: i32 = 2;
const NR_BALLOON_PAGES: i32 = 0;
const EAGAIN: i32 = 11;
const ENOENT: i32 = 2;
const PGTY_offline: i32 = 0;

static mut BALLOON_PAGES_LOCK: c_void = c_void::default();

unsafe fn balloon_page_insert(balloon: *mut balloon_dev_info, page: *mut page) {
    __SetPageOffline(page);
    #[cfg(CONFIG_BALLOON_MIGRATION)]
    {
        SetPageMovableOps(page);
        set_page_private(page, balloon as usize);
    }
    list_add(&mut (*page).lru, &mut (*balloon).pages);
}

unsafe fn balloon_page_finalize(page: *mut page) {
    #[cfg(CONFIG_BALLOON_MIGRATION)]
    set_page_private(page, 0);
}

unsafe fn balloon_page_enqueue_one(b_dev_info: *mut balloon_dev_info, page: *mut page) {
    balloon_page_insert(b_dev_info, page);
    if let Some(adjust) = (*b_dev_info).adjust_managed_page_count {
        adjust(page, -1);
    }
    __count_vm_event(BALLOON_INFLATE);
    inc_node_page_state(page, NR_BALLOON_PAGES);
}

pub unsafe extern "C" fn balloon_page_list_enqueue(
    b_dev_info: *mut balloon_dev_info,
    pages: *mut list_head,
) -> usize {
    let mut n_pages = 0usize;
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    // list_for_each_entry_safe(page, tmp, pages, lru)
    let mut entry = (*pages).next;
    while entry != pages {
        let next = (*entry).next;
        let page = entry as *mut page;
        list_del(&mut (*page).lru);
        balloon_page_enqueue_one(b_dev_info, page);
        n_pages += 1;
        entry = next;
    }
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
    n_pages
}

pub unsafe extern "C" fn balloon_page_list_dequeue(
    b_dev_info: *mut balloon_dev_info,
    pages: *mut list_head,
    n_req_pages: usize,
) -> usize {
    let mut n_pages = 0usize;
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    let mut entry = (*b_dev_info).pages.next;
    while entry != &mut (*b_dev_info).pages as *mut list_head && n_pages != n_req_pages {
        let next = (*entry).next;
        let page = entry as *mut page;
        list_del(&mut (*page).lru);
        if let Some(adjust) = (*b_dev_info).adjust_managed_page_count { adjust(page, 1); }
        balloon_page_finalize(page);
        __count_vm_event(BALLOON_DEFLATE);
        list_add(&mut (*page).lru, pages);
        dec_node_page_state(page, NR_BALLOON_PAGES);
        n_pages += 1;
        entry = next;
    }
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
    n_pages
}

pub unsafe extern "C" fn balloon_page_alloc() -> *mut page {
    let mut gfp_flags = __GFP_NOMEMALLOC | __GFP_NORETRY | __GFP_NOWARN;
    #[cfg(CONFIG_BALLOON_MIGRATION)]
    { gfp_flags |= GFP_HIGHUSER_MOVABLE; }
    #[cfg(not(CONFIG_BALLOON_MIGRATION))]
    { gfp_flags |= GFP_HIGHUSER; }
    alloc_page(gfp_flags)
}

pub unsafe extern "C" fn balloon_page_enqueue(b_dev_info: *mut balloon_dev_info, page: *mut page) {
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    balloon_page_enqueue_one(b_dev_info, page);
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
}

pub unsafe extern "C" fn balloon_page_dequeue(b_dev_info: *mut balloon_dev_info) -> *mut page {
    let mut pages = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
    pages.next = &mut pages;
    pages.prev = &mut pages;
    let n_pages = balloon_page_list_dequeue(b_dev_info, &mut pages, 1);
    if n_pages != 1 {
        spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
        if (*b_dev_info).pages.next == &mut (*b_dev_info).pages as *mut list_head && (*b_dev_info).isolated_pages == 0 { BUG(); }
        spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
        return core::ptr::null_mut();
    }
    pages.next as *mut page
}

#[cfg(CONFIG_BALLOON_MIGRATION)]
#[repr(C)]
pub struct movable_operations {
    pub migrate_page: unsafe extern "C" fn(*mut page, *mut page, migrate_mode_t) -> i32,
    pub isolate_page: unsafe extern "C" fn(*mut page, isolate_mode_t) -> bool,
    pub putback_page: unsafe extern "C" fn(*mut page),
}

#[cfg(CONFIG_BALLOON_MIGRATION)]
unsafe fn balloon_page_device(page: *mut page) -> *mut balloon_dev_info { page_private(page) as *mut balloon_dev_info }

#[cfg(CONFIG_BALLOON_MIGRATION)]
unsafe extern "C" fn balloon_page_isolate(page: *mut page, _mode: isolate_mode_t) -> bool {
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    let b = balloon_page_device(page);
    if b.is_null() { spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0); return false; }
    list_del(&mut (*page).lru);
    (*b).isolated_pages += 1;
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
    true
}

#[cfg(CONFIG_BALLOON_MIGRATION)]
unsafe extern "C" fn balloon_page_putback(page: *mut page) {
    let b = balloon_page_device(page);
    if WARN_ON_ONCE(b.is_null()) { return; }
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    list_add(&mut (*page).lru, &mut (*b).pages);
    (*b).isolated_pages -= 1;
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
}

#[cfg(CONFIG_BALLOON_MIGRATION)]
unsafe extern "C" fn balloon_page_migrate(newpage: *mut page, page: *mut page, mode: migrate_mode_t) -> i32 {
    let b = balloon_page_device(page);
    if WARN_ON_ONCE(b.is_null()) { return -EAGAIN; }
    let rc = ((*b).migratepage)(b, newpage, page, mode);
    if rc < 0 && rc != -ENOENT { return rc; }
    spin_lock_irqsave(&mut BALLOON_PAGES_LOCK, &mut 0usize);
    if rc == 0 {
        get_page(newpage);
        balloon_page_insert(b, newpage);
        __count_vm_event(BALLOON_MIGRATE);
        if (*b).adjust_managed_page_count.is_some() && page_zone(page) != page_zone(newpage) {
            adjust_managed_page_count(page, 1);
            adjust_managed_page_count(newpage, -1);
        }
    } else {
        __count_vm_event(BALLOON_DEFLATE);
        if let Some(adjust) = (*b).adjust_managed_page_count { adjust(page, 1); }
    }
    (*b).isolated_pages -= 1;
    balloon_page_finalize(page);
    spin_unlock_irqrestore(&mut BALLOON_PAGES_LOCK, 0);
    put_page(page);
    0
}

#[cfg(CONFIG_BALLOON_MIGRATION)]
static BALLOON_MOPS: movable_operations = movable_operations {
    migrate_page: balloon_page_migrate,
    isolate_page: balloon_page_isolate,
    putback_page: balloon_page_putback,
};

#[cfg(CONFIG_BALLOON_MIGRATION)]
pub unsafe extern "C" fn balloon_init() -> i32 { set_movable_ops(&BALLOON_MOPS, PGTY_offline) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
