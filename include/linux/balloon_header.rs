/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common interface for implementing a memory balloon, including support
 * for migration of pages inflated in a memory balloon.
 *
 * Balloon page migration makes use of the general "movable_ops page migration"
 * feature.
 *
 * page->private is used to reference the responsible balloon device.
 * That these pages have movable_ops, and which movable_ops apply,
 * is derived from the page type (PageOffline()) combined with the
 * PG_movable_ops flag (PageMovableOps()).
 *
 * Once the page type and the PG_movable_ops are set, migration code
 * can initiate page isolation by invoking the
 * movable_operations()->isolate_page() callback
 *
 * As long as page->private is set, the page is either on the balloon list
 * or isolated for migration. If page->private is not set, the page is
 * either still getting inflated, or was deflated to be freed by the balloon
 * driver soon. Isolation is impossible in both cases.
 *
 * As the page isolation scanning step a compaction thread does is a lockless
 * procedure (from a page standpoint), it might bring some racy situations while
 * performing balloon page migration. In order to sort out these racy scenarios
 * and safely perform balloon's page migration we must, always, ensure following
 * these simple rules:
 *
 *   i. Inflation/deflation must set/clear page->private under the
 *      balloon_pages_lock
 *
 *  ii. isolation or dequeueing procedure must remove the page from balloon
 *      device page list under balloon_pages_lock
 *
 * Copyright (C) 2012, Red Hat, Inc.  Rafael Aquini <aquini@redhat.com>
 */

// C header dependencies: linux/pagemap.h, linux/page-flags.h,
// linux/migrate.h, linux/gfp.h, linux/err.h, and linux/list.h.

#[repr(C)]
pub struct balloon_dev_info {
    pub isolated_pages: ::core::ffi::c_ulong,
    pub pages: crate::list_head,
    pub migratepage: Option<unsafe extern "C" fn(
        *mut balloon_dev_info,
        *mut crate::page,
        *mut crate::page,
        crate::migrate_mode,
    )>,
    pub adjust_managed_page_count: bool,
}

extern "C" {
    pub fn balloon_page_alloc() -> *mut crate::page;
    pub fn balloon_page_enqueue(
        b_dev_info: *mut balloon_dev_info,
        page: *mut crate::page,
    );
    pub fn balloon_page_dequeue(b_dev_info: *mut balloon_dev_info) -> *mut crate::page;
    pub fn balloon_page_list_enqueue(
        b_dev_info: *mut balloon_dev_info,
        pages: *mut crate::list_head,
    ) -> usize;
    pub fn balloon_page_list_dequeue(
        b_dev_info: *mut balloon_dev_info,
        pages: *mut crate::list_head,
        n_req_pages: usize,
    ) -> usize;
}

#[inline]
pub unsafe fn balloon_devinfo_init(balloon: *mut balloon_dev_info) {
    (*balloon).isolated_pages = 0;
    crate::INIT_LIST_HEAD(&mut (*balloon).pages);
    (*balloon).migratepage = None;
    (*balloon).adjust_managed_page_count = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
