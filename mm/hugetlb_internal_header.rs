/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Internal HugeTLB definitions.
 * (C) Nadia Yvette Chambers, April 2004
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/hugetlb.h>
// <linux/hugetlb_cgroup.h>

pub unsafe fn hstate_is_gigantic_no_runtime(h: *mut hstate) -> bool {
    unsafe { hstate_is_gigantic(h) && !gigantic_page_runtime_supported() }
}

/*
 * common helper functions for hstate_next_node_to_{alloc|free}.
 * We may have allocated or freed a huge page based on a different
 * nodes_allowed previously, so h->next_node_to_{alloc|free} might
 * be outside of *nodes_allowed.  Ensure that we use an allowed
 * node for alloc or free.
 */
pub unsafe fn next_node_allowed(mut nid: i32, nodes_allowed: *mut nodemask_t) -> i32 {
    unsafe {
        nid = next_node_in(nid, *nodes_allowed);
        VM_BUG_ON(nid >= MAX_NUMNODES);
    }
    nid
}

pub unsafe fn get_valid_node_allowed(mut nid: i32, nodes_allowed: *mut nodemask_t) -> i32 {
    unsafe {
        if !node_isset(nid, *nodes_allowed) {
            nid = next_node_allowed(nid, nodes_allowed);
        }
    }
    nid
}

/*
 * returns the previously saved node ["this node"] from which to
 * allocate a persistent huge page for the pool and advance the
 * next node from which to allocate, handling wrap at end of node
 * mask.
 */
pub unsafe fn hstate_next_node_to_alloc(
    next_node: *mut i32,
    nodes_allowed: *mut nodemask_t,
) -> i32 {
    unsafe {
        VM_BUG_ON(nodes_allowed.is_null());

        let nid = get_valid_node_allowed(*next_node, nodes_allowed);
        *next_node = next_node_allowed(nid, nodes_allowed);

        nid
    }
}

/*
 * helper for remove_pool_hugetlb_folio() - return the previously saved
 * node ["this node"] from which to free a huge page.  Advance the
 * next node id whether or not we find a free huge page to free so
 * that the next attempt to free addresses the next node.
 */
pub unsafe fn hstate_next_node_to_free(
    h: *mut hstate,
    nodes_allowed: *mut nodemask_t,
) -> i32 {
    unsafe {
        VM_BUG_ON(nodes_allowed.is_null());

        let nid = get_valid_node_allowed((*h).next_nid_to_free, nodes_allowed);
        (*h).next_nid_to_free = next_node_allowed(nid, nodes_allowed);

        nid
    }
}

#[macro_export]
macro_rules! for_each_node_mask_to_alloc {
    ($next_node:expr, $nr_nodes:expr, $node:expr, $mask:expr, $body:block) => {{
        $nr_nodes = unsafe { nodes_weight(*$mask) };
        while $nr_nodes > 0 {
            $node = unsafe { hstate_next_node_to_alloc($next_node, $mask) };
            $body
            $nr_nodes -= 1;
        }
    }};
}

#[macro_export]
macro_rules! for_each_node_mask_to_free {
    ($hs:expr, $nr_nodes:expr, $node:expr, $mask:expr, $body:block) => {{
        $nr_nodes = unsafe { nodes_weight(*$mask) };
        while $nr_nodes > 0 {
            $node = unsafe { hstate_next_node_to_free($hs, $mask) };
            $body
            $nr_nodes -= 1;
        }
    }};
}

unsafe extern "C" {
    pub fn remove_hugetlb_folio(h: *mut hstate, folio: *mut folio, adjust_surplus: bool);
    pub fn add_hugetlb_folio(h: *mut hstate, folio: *mut folio, adjust_surplus: bool);
    pub fn init_new_hugetlb_folio(folio: *mut folio);
    pub fn prep_and_add_allocated_folios(h: *mut hstate, folio_list: *mut list_head);
    pub fn demote_pool_huge_page(
        src: *mut hstate,
        nodes_allowed: *mut nodemask_t,
        nr_to_demote: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    pub fn __nr_hugepages_store_common(
        obey_mempolicy: bool,
        h: *mut hstate,
        nid: i32,
        count: ::core::ffi::c_ulong,
        len: usize,
    ) -> isize;
    pub fn hugetlb_sysfs_init(); // C __init annotation
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" {
    pub fn hugetlb_sysctl_init();
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
pub unsafe fn hugetlb_sysctl_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
