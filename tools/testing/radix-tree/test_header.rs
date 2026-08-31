/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of testing/radix-tree/test.h. */
/* C includes translated as external dependency expectations:
 * linux/gfp.h, linux/types.h, linux/radix-tree.h, linux/rcupdate.h
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct item {
    pub rcu_head: rcu_head,
    pub index: c_ulong,
    pub order: c_uint,
}

unsafe extern "C" {
    pub fn item_create(index: c_ulong, order: c_uint) -> *mut item;
    pub fn item_insert(root: *mut radix_tree_root, index: c_ulong) -> c_int;
    pub fn item_sanity(item: *mut item, index: c_ulong);
    pub fn item_free(item: *mut item, index: c_ulong);
    pub fn item_delete(root: *mut radix_tree_root, index: c_ulong) -> c_int;
    pub fn item_delete_rcu(xa: *mut xarray, index: c_ulong) -> c_int;
    pub fn item_lookup(root: *mut radix_tree_root, index: c_ulong) -> *mut item;

    pub fn item_check_present(root: *mut radix_tree_root, index: c_ulong);
    pub fn item_check_absent(root: *mut radix_tree_root, index: c_ulong);
    pub fn item_gang_check_present(
        root: *mut radix_tree_root,
        start: c_ulong,
        nr: c_ulong,
        chunk: c_int,
        hop: c_int,
    );
    pub fn item_full_scan(
        root: *mut radix_tree_root,
        start: c_ulong,
        nr: c_ulong,
        chunk: c_int,
    );
    pub fn item_kill_tree(root: *mut radix_tree_root);

    pub fn tag_tagged_items(
        xa: *mut xarray,
        start: c_ulong,
        end: c_ulong,
        batch: c_uint,
        iftag: xa_mark_t,
        thentag: xa_mark_t,
    ) -> c_int;

    pub fn xarray_tests();
    pub fn tag_check();
    pub fn multiorder_checks();
    pub fn iteration_test(order: c_uint, duration: c_uint);
    pub fn iteration_test2(duration: c_uint);
    pub fn benchmark();
    pub fn idr_checks();
    pub fn ida_tests();

    pub fn item_tag_set(root: *mut radix_tree_root, index: c_ulong, tag: c_int) -> *mut item;
    pub fn item_tag_clear(root: *mut radix_tree_root, index: c_ulong, tag: c_int) -> *mut item;
    pub fn item_tag_get(root: *mut radix_tree_root, index: c_ulong, tag: c_int) -> c_int;
    pub fn tree_verify_min_height(root: *mut radix_tree_root, maxindex: c_int);
    pub fn verify_tag_consistency(root: *mut radix_tree_root, tag: c_uint);

    pub static mut nr_allocated: c_int;

    /* Normally private parts of lib/radix-tree.c */
    pub fn entry_to_node(ptr: *mut c_void) -> *mut radix_tree_node;
    pub fn radix_tree_dump(root: *mut radix_tree_root);
    pub fn root_tag_get(root: *mut radix_tree_root, tag: c_uint) -> c_int;
    pub fn node_maxindex(node: *mut radix_tree_node) -> c_ulong;
    pub fn shift_maxindex(shift: c_uint) -> c_ulong;
    pub fn radix_tree_cpu_dead(cpu: c_uint) -> c_int;
    pub static mut radix_tree_preloads: radix_tree_preload;
}
