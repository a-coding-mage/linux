// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/radix-tree/test.c.  C include dependencies:
// <stdlib.h>, <assert.h>, <stdio.h>, <linux/types.h>, <linux/kernel.h>,
// <linux/bitops.h>, and "test.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::mem::offset_of;
use core::ptr;

pub type xa_mark_t = c_uint;

pub const BITS_PER_LONG: c_uint = c_ulong::BITS;

unsafe extern "C" {
    static RADIX_TREE_TAG_LONGS: c_int;
    static RADIX_TREE_MAX_TAGS: c_int;
    static RADIX_TREE_MAP_SIZE: c_int;
    static RADIX_TREE_MAP_SHIFT: c_uint;
    static ULONG_MAX: c_ulong;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const i8, ...) -> c_int;

    fn radix_tree_tag_set(
        root: *mut radix_tree_root,
        index: c_ulong,
        tag: c_int,
    ) -> *mut item;
    fn radix_tree_tag_clear(
        root: *mut radix_tree_root,
        index: c_ulong,
        tag: c_int,
    ) -> *mut item;
    fn radix_tree_tag_get(root: *mut radix_tree_root, index: c_ulong, tag: c_int) -> c_int;
    fn radix_tree_insert(root: *mut radix_tree_root, index: c_ulong, item: *mut item) -> c_int;
    fn radix_tree_is_internal_node(node: *const c_void) -> c_int;
    fn radix_tree_delete(root: *mut radix_tree_root, index: c_ulong) -> *mut item;
    fn xa_erase(xa: *mut xarray, index: c_ulong) -> *mut item;
    fn call_rcu(head: *mut rcu_head, func: unsafe extern "C" fn(*mut rcu_head));
    fn radix_tree_lookup(root: *mut radix_tree_root, index: c_ulong) -> *mut item;
    fn radix_tree_gang_lookup(
        root: *mut radix_tree_root,
        results: *mut *mut c_void,
        first_index: c_ulong,
        max_items: c_uint,
    ) -> c_uint;

    fn xas_lock_irq(xas: *mut xa_state);
    fn xas_unlock_irq(xas: *mut xa_state);
    fn xas_set_mark(xas: *mut xa_state, mark: xa_mark_t);
    fn xas_pause(xas: *mut xa_state);
    fn rcu_barrier();
    fn xas_find_marked(xas: *mut xa_state, max: c_ulong, mark: xa_mark_t) -> *mut item;
    fn xas_find(xas: *mut xa_state, max: c_ulong) -> *mut c_void;
    fn xas_store(xas: *mut xa_state, entry: *mut c_void);

    fn entry_to_node(slot: *mut radix_tree_node) -> *mut radix_tree_node;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn root_tag_get(root: *mut radix_tree_root, tag: c_uint) -> c_int;
    fn xa_is_value(entry: *const c_void) -> c_int;
    fn xa_empty(xa: *mut xarray) -> c_int;
    fn node_maxindex(node: *mut radix_tree_node) -> c_ulong;
    fn shift_maxindex(shift: c_uint) -> c_ulong;
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct radix_tree_root {
    pub xa_head: *mut radix_tree_node,
}

#[repr(C)]
pub struct radix_tree_node {
    pub shift: c_uint,
    pub slots: [*mut radix_tree_node; 64],
    pub tags: [[c_ulong; 1]; 3],
}

#[repr(C)]
pub struct xa_state {
    pub xa: *mut xarray,
    pub xa_index: c_ulong,
}

#[repr(C)]
pub struct item {
    pub index: c_ulong,
    pub order: c_uint,
    pub rcu_head: rcu_head,
}

#[inline]
unsafe fn XA_STATE(xa: *mut xarray, index: c_ulong) -> xa_state {
    xa_state { xa, xa_index: index }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_tag_set(
    root: *mut radix_tree_root,
    index: c_ulong,
    tag: c_int,
) -> *mut item {
    unsafe { radix_tree_tag_set(root, index, tag) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_tag_clear(
    root: *mut radix_tree_root,
    index: c_ulong,
    tag: c_int,
) -> *mut item {
    unsafe { radix_tree_tag_clear(root, index, tag) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_tag_get(
    root: *mut radix_tree_root,
    index: c_ulong,
    tag: c_int,
) -> c_int {
    unsafe { radix_tree_tag_get(root, index, tag) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_create(index: c_ulong, order: c_uint) -> *mut item {
    let ret = unsafe { malloc(core::mem::size_of::<item>()) as *mut item };

    unsafe {
        (*ret).index = index;
        (*ret).order = order;
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_insert(root: *mut radix_tree_root, index: c_ulong) -> c_int {
    let item = unsafe { item_create(index, 0) };
    let err = unsafe { radix_tree_insert(root, (*item).index, item) };
    if err != 0 {
        unsafe { free(item as *mut c_void) };
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_sanity(item: *mut item, index: c_ulong) {
    let mask: c_ulong;
    assert!(unsafe { radix_tree_is_internal_node(item as *const c_void) } == 0);
    assert!(unsafe { (*item).order } < BITS_PER_LONG);
    mask = (1 as c_ulong).wrapping_shl(unsafe { (*item).order }) - 1;
    assert!((unsafe { (*item).index } | mask) == (index | mask));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_free(item: *mut item, index: c_ulong) {
    unsafe { item_sanity(item, index) };
    unsafe { free(item as *mut c_void) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_delete(root: *mut radix_tree_root, index: c_ulong) -> c_int {
    let item = unsafe { radix_tree_delete(root, index) };

    if item.is_null() {
        return 0;
    }

    unsafe { item_free(item, index) };
    1
}

unsafe extern "C" fn item_free_rcu(head: *mut rcu_head) {
    let item = (head as *mut u8).wrapping_sub(offset_of!(item, rcu_head)) as *mut item;

    unsafe { free(item as *mut c_void) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_delete_rcu(xa: *mut xarray, index: c_ulong) -> c_int {
    let item = unsafe { xa_erase(xa, index) };

    if !item.is_null() {
        unsafe { item_sanity(item, index) };
        unsafe { call_rcu(&mut (*item).rcu_head, item_free_rcu) };
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_check_present(root: *mut radix_tree_root, index: c_ulong) {
    let item: *mut item;

    item = unsafe { radix_tree_lookup(root, index) };
    assert!(!item.is_null());
    unsafe { item_sanity(item, index) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_lookup(root: *mut radix_tree_root, index: c_ulong) -> *mut item {
    unsafe { radix_tree_lookup(root, index) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_check_absent(root: *mut radix_tree_root, index: c_ulong) {
    let item: *mut item;

    item = unsafe { radix_tree_lookup(root, index) };
    assert!(item.is_null());
}

/*
 * Scan only the passed (start, start+nr] for present items
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_gang_check_present(
    root: *mut radix_tree_root,
    start: c_ulong,
    nr: c_ulong,
    chunk: c_int,
    hop: c_int,
) {
    let mut items = vec![ptr::null_mut::<item>(); chunk as usize];
    let mut into: c_ulong;

    into = 0;
    while into < nr {
        let mut nr_to_find: c_int = chunk;
        let mut i: c_int;

        if (nr_to_find as c_ulong) > nr - into {
            nr_to_find = (nr - into) as c_int;
        }

        let nfound = unsafe {
            radix_tree_gang_lookup(
                root,
                items.as_mut_ptr() as *mut *mut c_void,
                start + into,
                nr_to_find as c_uint,
            )
        } as c_int;
        assert!(nfound == nr_to_find);
        i = 0;
        while i < nfound {
            assert!(unsafe { (*items[i as usize]).index } == start + into + i as c_ulong);
            i += 1;
        }
        into = into.wrapping_add(hop as c_ulong);
    }
}

/*
 * Scan the entire tree, only expecting present items (start, start+nr]
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_full_scan(
    root: *mut radix_tree_root,
    start: c_ulong,
    nr: c_ulong,
    chunk: c_int,
) {
    let mut items = vec![ptr::null_mut::<item>(); chunk as usize];
    let mut into: c_ulong = 0;
    let mut this_index: c_ulong = start;
    let mut nfound: c_int;
    let mut i: c_int;

    //	printf("%s(0x%08lx, 0x%08lx, %d)\n", __FUNCTION__, start, nr, chunk);

    loop {
        nfound = unsafe {
            radix_tree_gang_lookup(
                root,
                items.as_mut_ptr() as *mut *mut c_void,
                into,
                chunk as c_uint,
            )
        } as c_int;
        if nfound == 0 {
            break;
        }
        //		printf("At 0x%08lx, nfound=%d\n", into, nfound);
        i = 0;
        while i < nfound {
            assert!(unsafe { (*items[i as usize]).index } == this_index);
            this_index = this_index.wrapping_add(1);
            i += 1;
        }
        //		printf("Found 0x%08lx->0x%08lx\n",
        //			items[0]->index, items[nfound-1]->index);
        into = this_index;
    }
    if chunk != 0 {
        assert!(this_index == start + nr);
    }
    nfound = unsafe {
        radix_tree_gang_lookup(
            root,
            items.as_mut_ptr() as *mut *mut c_void,
            this_index,
            chunk as c_uint,
        )
    } as c_int;
    assert!(nfound == 0);
}

/* Use the same pattern as tag_pages_for_writeback() in mm/page-writeback.c */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tag_tagged_items(
    xa: *mut xarray,
    start: c_ulong,
    end: c_ulong,
    mut batch: c_uint,
    iftag: xa_mark_t,
    thentag: xa_mark_t,
) -> c_int {
    let mut xas = unsafe { XA_STATE(xa, start) };
    let mut tagged: c_uint = 0;
    let mut item: *mut item;

    if batch == 0 {
        batch = 1;
    }

    unsafe { xas_lock_irq(&mut xas) };
    loop {
        item = unsafe { xas_find_marked(&mut xas, end, iftag) };
        if item.is_null() {
            break;
        }
        unsafe { xas_set_mark(&mut xas, thentag) };
        tagged = tagged.wrapping_add(1);
        if tagged % batch != 0 {
            continue;
        }

        unsafe { xas_pause(&mut xas) };
        unsafe { xas_unlock_irq(&mut xas) };
        unsafe { rcu_barrier() };
        unsafe { xas_lock_irq(&mut xas) };
    }
    unsafe { xas_unlock_irq(&mut xas) };

    tagged as c_int
}

unsafe fn verify_node(slot: *mut radix_tree_node, tag: c_uint, tagged: c_int) -> c_int {
    let mut anyset: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;

    let slot = unsafe { entry_to_node(slot) };

    /* Verify consistency at this level */
    i = 0;
    while i < unsafe { RADIX_TREE_TAG_LONGS } {
        if unsafe { (*slot).tags[tag as usize][i as usize] } != 0 {
            anyset = 1;
            break;
        }
        i += 1;
    }
    if tagged != anyset {
        unsafe {
            printf(
                c"tag: %u, shift %u, tagged: %d, anyset: %d\n".as_ptr(),
                tag,
                (*slot).shift,
                tagged,
                anyset,
            )
        };
        j = 0;
        while j < unsafe { RADIX_TREE_MAX_TAGS } {
            unsafe { printf(c"tag %d: ".as_ptr(), j) };
            i = 0;
            while i < unsafe { RADIX_TREE_TAG_LONGS } {
                unsafe {
                    printf(
                        c"%016lx ".as_ptr(),
                        (*slot).tags[j as usize][i as usize],
                    )
                };
                i += 1;
            }
            unsafe { printf(c"\n".as_ptr()) };
            j += 1;
        }
        return 1;
    }
    assert!(tagged == anyset);

    /* Go for next level */
    if unsafe { (*slot).shift } > 0 {
        i = 0;
        while i < unsafe { RADIX_TREE_MAP_SIZE } {
            if !unsafe { (*slot).slots[i as usize] }.is_null() {
                if unsafe {
                    verify_node(
                        (*slot).slots[i as usize],
                        tag,
                        (test_bit(i, (*slot).tags[tag as usize].as_ptr()) != 0) as c_int,
                    )
                } != 0
                {
                    unsafe { printf(c"Failure at off %d\n".as_ptr(), i) };
                    j = 0;
                    while j < unsafe { RADIX_TREE_MAX_TAGS } {
                        unsafe { printf(c"tag %d: ".as_ptr(), j) };
                        i = 0;
                        while i < unsafe { RADIX_TREE_TAG_LONGS } {
                            unsafe {
                                printf(
                                    c"%016lx ".as_ptr(),
                                    (*slot).tags[j as usize][i as usize],
                                )
                            };
                            i += 1;
                        }
                        unsafe { printf(c"\n".as_ptr()) };
                        j += 1;
                    }
                    return 1;
                }
            }
            i += 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn verify_tag_consistency(root: *mut radix_tree_root, tag: c_uint) {
    let node = unsafe { (*root).xa_head };
    if unsafe { radix_tree_is_internal_node(node as *const c_void) } == 0 {
        return;
    }
    unsafe { verify_node(node, tag, (root_tag_get(root, tag) != 0) as c_int) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn item_kill_tree(xa: *mut xarray) {
    let mut xas = unsafe { XA_STATE(xa, 0) };
    let mut entry: *mut c_void;

    loop {
        entry = unsafe { xas_find(&mut xas, ULONG_MAX) };
        if entry.is_null() {
            break;
        }
        if unsafe { xa_is_value(entry) } == 0 {
            unsafe { item_free(entry as *mut item, xas.xa_index) };
        }
        unsafe { xas_store(&mut xas, ptr::null_mut()) };
    }

    assert!(unsafe { xa_empty(xa) } != 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tree_verify_min_height(root: *mut radix_tree_root, maxindex: c_int) {
    let mut shift: c_uint;
    let mut node = unsafe { (*root).xa_head };
    if unsafe { radix_tree_is_internal_node(node as *const c_void) } == 0 {
        assert!(maxindex == 0);
        return;
    }

    node = unsafe { entry_to_node(node) };
    assert!((maxindex as c_ulong) <= unsafe { node_maxindex(node) });

    shift = unsafe { (*node).shift };
    if shift > 0 {
        assert!((maxindex as c_ulong) > unsafe { shift_maxindex(shift - RADIX_TREE_MAP_SHIFT) });
    } else {
        assert!(maxindex > 0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
