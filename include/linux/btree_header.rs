/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the Linux kernel translation environment:
 * linux/kernel.h, linux/mempool.h, linux/btree-128.h, and linux/btree-type.h.
 */

/**
 * DOC: B+Tree basics
 *
 * A B+Tree is a data structure for looking up arbitrary (currently allowing
 * unsigned long, u32, u64 and 2 * u64) keys into pointers. The data structure
 * is described at https://en.wikipedia.org/wiki/B-tree, we currently do not
 * use binary search to find the key on lookups.
 *
 * Each B+Tree consists of a head, that contains bookkeeping information and
 * a variable number (starting with zero) nodes. Each node contains the keys
 * and pointers to sub-nodes, or, for leaf nodes, the keys and values for the
 * tree entries.
 *
 * Each node in this implementation has the following layout:
 * [key1, key2, ..., keyN] [val1, val2, ..., valN]
 *
 * Each key here is an array of unsigned longs, geo->no_longs in total. The
 * number of keys and values (N) is geo->no_pairs.
 */

#[repr(C)]
pub struct btree_head {
    pub node: *mut ::core::ffi::c_ulong,
    pub mempool: *mut mempool_t,
    pub height: ::core::ffi::c_int,
}

/* btree geometry */
#[repr(C)]
pub struct btree_geo {
    _private: [u8; 0],
}

extern "C" {
    pub fn btree_alloc(gfp_mask: gfp_t, pool_data: *mut ::core::ffi::c_void)
        -> *mut ::core::ffi::c_void;
    pub fn btree_free(element: *mut ::core::ffi::c_void,
                      pool_data: *mut ::core::ffi::c_void);
    pub fn btree_init_mempool(head: *mut btree_head, mempool: *mut mempool_t);
    pub fn btree_init(head: *mut btree_head) -> ::core::ffi::c_int;
    pub fn btree_destroy(head: *mut btree_head);
    pub fn btree_lookup(head: *mut btree_head, geo: *mut btree_geo,
                        key: *mut ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn btree_insert(head: *mut btree_head, geo: *mut btree_geo,
                        key: *mut ::core::ffi::c_ulong,
                        val: *mut ::core::ffi::c_void, gfp: gfp_t)
        -> ::core::ffi::c_int;
    pub fn btree_update(head: *mut btree_head, geo: *mut btree_geo,
                        key: *mut ::core::ffi::c_ulong,
                        val: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn btree_remove(head: *mut btree_head, geo: *mut btree_geo,
                        key: *mut ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn btree_merge(target: *mut btree_head, victim: *mut btree_head,
                       geo: *mut btree_geo, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn btree_last(head: *mut btree_head, geo: *mut btree_geo,
                      key: *mut ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn btree_get_prev(head: *mut btree_head, geo: *mut btree_geo,
                          key: *mut ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn btree_visitor(
        head: *mut btree_head, geo: *mut btree_geo, opaque: ::core::ffi::c_ulong,
        func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void,
                                           ::core::ffi::c_ulong,
                                           *mut ::core::ffi::c_ulong,
                                           usize, *mut ::core::ffi::c_void)>,
        func2: *mut ::core::ffi::c_void) -> usize;
    pub fn btree_grim_visitor(
        head: *mut btree_head, geo: *mut btree_geo, opaque: ::core::ffi::c_ulong,
        func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void,
                                           ::core::ffi::c_ulong,
                                           *mut ::core::ffi::c_ulong,
                                           usize, *mut ::core::ffi::c_void)>,
        func2: *mut ::core::ffi::c_void) -> usize;

    pub static mut btree_geo32: btree_geo;
    pub static mut btree_geo64: btree_geo;
}

/* linux/btree-128.h and linux/btree-type.h provide the generated typed APIs. */

#[macro_export]
macro_rules! btree_for_each_safel {
    ($head:expr, $key:expr, $val:expr) => {
        for $val in unsafe { btree_lastl($head, &mut $key) } {
            let _ = $val;
            unsafe { btree_get_prevl($head, &mut $key) }
        }
    };
}

#[macro_export]
macro_rules! btree_for_each_safe32 {
    ($head:expr, $key:expr, $val:expr) => {
        for $val in unsafe { btree_last32($head, &mut $key) } {
            let _ = $val;
            unsafe { btree_get_prev32($head, &mut $key) }
        }
    };
}

#[macro_export]
macro_rules! btree_for_each_safe64 {
    ($head:expr, $key:expr, $val:expr) => {
        for $val in unsafe { btree_last64($head, &mut $key) } {
            let _ = $val;
            unsafe { btree_get_prev64($head, &mut $key) }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
