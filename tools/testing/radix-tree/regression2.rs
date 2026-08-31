// SPDX-License-Identifier: GPL-2.0
/*
 * Regression2
 * Description:
 * Toshiyuki Okajima describes the following radix-tree bug:
 *
 * In the following case, we can get a hangup on
 *   radix_radix_tree_gang_lookup_tag_slot.
 *
 * 0.  The radix tree contains RADIX_TREE_MAP_SIZE items. And the tag of
 *     a certain item has PAGECACHE_TAG_DIRTY.
 * 1.  radix_tree_range_tag_if_tagged(, start, end, , PAGECACHE_TAG_DIRTY,
 *     PAGECACHE_TAG_TOWRITE) is called to add PAGECACHE_TAG_TOWRITE tag
 *     for the tag which has PAGECACHE_TAG_DIRTY. However, there is no tag with
 *     PAGECACHE_TAG_DIRTY within the range from start to end. As the result,
 *     There is no tag with PAGECACHE_TAG_TOWRITE but the root tag has
 *     PAGECACHE_TAG_TOWRITE.
 * 2.  An item is added into the radix tree and then the level of it is
 *     extended into 2 from 1. At that time, the new radix tree node succeeds
 *     the tag status of the root tag. Therefore the tag of the new radix tree
 *     node has PAGECACHE_TAG_TOWRITE but there is not slot with
 *     PAGECACHE_TAG_TOWRITE tag in the child node of the new radix tree node.
 * 3.  The tag of a certain item is cleared with PAGECACHE_TAG_DIRTY.
 * 4.  All items within the index range from 0 to RADIX_TREE_MAP_SIZE - 1 are
 *     released. (Only the item which index is RADIX_TREE_MAP_SIZE exist in the
 *     radix tree.) As the result, the slot of the radix tree node is NULL but
 *     the tag which corresponds to the slot has PAGECACHE_TAG_TOWRITE.
 * 5.  radix_tree_gang_lookup_tag_slot(PAGECACHE_TAG_TOWRITE) calls
 *     __lookup_tag. __lookup_tag returns with 0. And __lookup_tag doesn't
 *     change the index that is the input and output parameter. Because the 1st
 *     slot of the radix tree node is NULL, but the tag which corresponds to
 *     the slot has PAGECACHE_TAG_TOWRITE.
 *     Therefore radix_tree_gang_lookup_tag_slot tries to get some items by
 *     calling __lookup_tag, but it cannot get any items forever.
 *
 * The fix is to change that radix_tree_tag_if_tagged doesn't tag the root tag
 * if it doesn't set any tags within the specified range.
 *
 * Running:
 * This test should run to completion immediately. The above bug would cause it
 * to hang indefinitely.
 *
 * Upstream commit:
 * Not yet
 */

use core::ffi::{c_int, c_ulong, c_void};

pub const PAGECACHE_TAG_DIRTY: c_int = XA_MARK_0;
pub const PAGECACHE_TAG_WRITEBACK: c_int = XA_MARK_1;
pub const PAGECACHE_TAG_TOWRITE: c_int = XA_MARK_2;

// C dependency macro: static RADIX_TREE(mt_tree, GFP_KERNEL);
// The concrete initializer is supplied by the radix-tree support code.
static mut mt_tree: radix_tree_root = RADIX_TREE_INIT(GFP_KERNEL);

#[no_mangle]
pub static mut page_count: c_ulong = 0;

#[repr(C)]
pub struct page {
    pub index: c_ulong,
}

unsafe fn page_alloc() -> *mut page {
    let mut p: *mut page;
    p = malloc(core::mem::size_of::<page>()) as *mut page;
    (*p).index = page_count;
    page_count = page_count.wrapping_add(1);

    p
}

#[no_mangle]
pub unsafe extern "C" fn regression2_test() {
    let mut i: c_int;
    let mut p: *mut page;
    let max_slots: c_int = RADIX_TREE_MAP_SIZE;
    let mut start: c_ulong;
    let mut end: c_ulong;
    let mut pages: [*mut page; 1] = [core::ptr::null_mut(); 1];

    printv(
        1,
        c"running regression test 2 (should take milliseconds)\n".as_ptr(),
    );
    /* 0. */
    i = 0;
    while i <= max_slots - 1 {
        p = page_alloc();
        radix_tree_insert(&raw mut mt_tree, i as c_ulong, p as *mut c_void);
        i += 1;
    }
    radix_tree_tag_set(
        &raw mut mt_tree,
        (max_slots - 1) as c_ulong,
        PAGECACHE_TAG_DIRTY,
    );

    /* 1. */
    start = 0;
    end = (max_slots - 2) as c_ulong;
    tag_tagged_items(
        &raw mut mt_tree,
        start,
        end,
        1,
        PAGECACHE_TAG_DIRTY,
        PAGECACHE_TAG_TOWRITE,
    );

    /* 2. */
    p = page_alloc();
    radix_tree_insert(&raw mut mt_tree, max_slots as c_ulong, p as *mut c_void);

    /* 3. */
    radix_tree_tag_clear(
        &raw mut mt_tree,
        (max_slots - 1) as c_ulong,
        PAGECACHE_TAG_DIRTY,
    );

    /* 4. */
    i = max_slots - 1;
    while i >= 0 {
        free(radix_tree_delete(&raw mut mt_tree, i as c_ulong));
        i -= 1;
    }

    /* 5. */
    // NOTE: start should not be 0 because radix_tree_gang_lookup_tag_slot
    //       can return.
    start = 1;
    end = (max_slots - 2) as c_ulong;
    radix_tree_gang_lookup_tag_slot(
        &raw mut mt_tree,
        pages.as_mut_ptr() as *mut *mut *mut c_void,
        start,
        end,
        PAGECACHE_TAG_TOWRITE,
    );

    /* We remove all the remained nodes */
    free(radix_tree_delete(&raw mut mt_tree, max_slots as c_ulong));

    BUG_ON(!radix_tree_empty(&raw mut mt_tree));

    printv(1, c"regression test 2, done\n".as_ptr());
}

unsafe extern "C" {
    static XA_MARK_0: c_int;
    static XA_MARK_1: c_int;
    static XA_MARK_2: c_int;
    static GFP_KERNEL: c_int;
    static RADIX_TREE_MAP_SIZE: c_int;

    type radix_tree_root;

    fn RADIX_TREE_INIT(gfp_mask: c_int) -> radix_tree_root;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printv(level: c_int, fmt: *const i8, ...);
    fn radix_tree_insert(root: *mut radix_tree_root, index: c_ulong, item: *mut c_void) -> c_int;
    fn radix_tree_tag_set(root: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn tag_tagged_items(
        root: *mut radix_tree_root,
        start: c_ulong,
        end: c_ulong,
        nr_to_tag: c_int,
        fromtag: c_int,
        totag: c_int,
    ) -> c_int;
    fn radix_tree_tag_clear(root: *mut radix_tree_root, index: c_ulong, tag: c_int);
    fn radix_tree_delete(root: *mut radix_tree_root, index: c_ulong) -> *mut c_void;
    fn radix_tree_gang_lookup_tag_slot(
        root: *mut radix_tree_root,
        results: *mut *mut *mut c_void,
        first_index: c_ulong,
        max_items: c_ulong,
        tag: c_int,
    ) -> c_int;
    fn radix_tree_empty(root: *mut radix_tree_root) -> bool;
    fn BUG_ON(condition: bool);
}
