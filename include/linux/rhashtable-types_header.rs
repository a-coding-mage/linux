/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Resizable, Scalable, Concurrent Hash Table
 *
 * Simple structures that might be needed in include
 * files.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct rhash_head {
    pub next: *mut rhash_head,
}

#[repr(C)]
pub struct rhlist_head {
    pub rhead: rhash_head,
    pub next: *mut rhlist_head,
}

pub struct bucket_table;

/**
 * struct rhashtable_compare_arg - Key for the function rhashtable_compare
 * @ht: Hash table
 * @key: Key to compare against
 */
#[repr(C)]
pub struct rhashtable_compare_arg {
    pub ht: *mut rhashtable,
    pub key: *const core::ffi::c_void,
}

pub type rht_hashfn_t = unsafe extern "C" fn(
    data: *const core::ffi::c_void,
    len: u32,
    seed: u32,
) -> u32;
pub type rht_obj_hashfn_t = unsafe extern "C" fn(
    data: *const core::ffi::c_void,
    len: u32,
    seed: u32,
) -> u32;
pub type rht_obj_cmpfn_t = unsafe extern "C" fn(
    arg: *mut rhashtable_compare_arg,
    obj: *const core::ffi::c_void,
) -> i32;

/**
 * struct rhashtable_params - Hash table construction parameters
 * @nelem_hint: Hint on number of elements, should be 75% of desired size
 * @key_len: Length of key
 * @key_offset: Offset of key in struct to be hashed
 * @head_offset: Offset of rhash_head in struct to be hashed
 * @max_size: Maximum size while expanding
 * @min_size: Minimum size while shrinking
 * @insecure_elasticity: Set to true to disable chain length checks
 * @automatic_shrinking: Enable automatic shrinking of tables
 * @hashfn: Hash function (default: jhash2 if !(key_len % 4), or jhash)
 * @obj_hashfn: Function to hash object
 * @obj_cmpfn: Function to compare key with object
 */
#[repr(C)]
pub struct rhashtable_params {
    pub nelem_hint: u16,
    pub key_len: u16,
    pub key_offset: u16,
    pub head_offset: u16,
    pub max_size: u32,
    pub min_size: u16,
    pub insecure_elasticity: bool,
    pub automatic_shrinking: bool,
    pub hashfn: Option<rht_hashfn_t>,
    pub obj_hashfn: Option<rht_obj_hashfn_t>,
    pub obj_cmpfn: Option<rht_obj_cmpfn_t>,
}

/**
 * struct rhashtable - Hash table handle
 * @tbl: Bucket table
 * @key_len: Key length for hashfn
 * @max_elems: Maximum number of elements in table
 * @p: Configuration parameters
 * @rhlist: True if this is an rhltable
 * @run_work: Deferred worker to expand/shrink asynchronously
 * @run_irq_work: Bounces the @run_work kick through hard IRQ context.
 * @mutex: Mutex to protect current/future table swapping
 * @lock: Spin lock to protect walker list
 * @nelems: Number of elements in table
 */
#[repr(C)]
pub struct rhashtable {
    pub tbl: *mut bucket_table,
    pub key_len: u32,
    pub max_elems: u32,
    pub p: rhashtable_params,
    pub rhlist: bool,
    pub run_work: work_struct,
    pub run_irq_work: irq_work,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub nelems: atomic_t,
    #[cfg(feature = "CONFIG_MEM_ALLOC_PROFILING")]
    pub alloc_tag: *mut alloc_tag,
}

/**
 * struct rhltable - Hash table with duplicate objects in a list
 * @ht: Underlying rhtable
 */
#[repr(C)]
pub struct rhltable {
    pub ht: rhashtable,
}

/**
 * struct rhashtable_walker - Hash table walker
 * @list: List entry on list of walkers
 * @tbl: The table that we were walking over
 */
#[repr(C)]
pub struct rhashtable_walker {
    pub list: list_head,
    pub tbl: *mut bucket_table,
}

/**
 * struct rhashtable_iter - Hash table iterator
 * @ht: Table to iterate through
 * @p: Current pointer
 * @list: Current hash list pointer
 * @walker: Associated rhashtable walker
 * @slot: Current slot
 * @skip: Number of entries to skip in slot
 */
#[repr(C)]
pub struct rhashtable_iter {
    pub ht: *mut rhashtable,
    pub p: *mut rhash_head,
    pub list: *mut rhlist_head,
    pub walker: rhashtable_walker,
    pub slot: u32,
    pub skip: u32,
    pub end_of_table: bool,
}

extern "C" {
    pub fn __rhashtable_init_noprof(
        ht: *mut rhashtable,
        params: *const rhashtable_params,
        key: *mut lock_class_key,
    ) -> i32;
}

#[macro_export]
macro_rules! rhashtable_init_noprof {
    ($ht:expr, $params:expr) => {{
        static mut __key: lock_class_key = unsafe { core::mem::zeroed() };
        unsafe { __rhashtable_init_noprof($ht, $params, core::ptr::addr_of_mut!(__key)) }
    }};
}

#[macro_export]
macro_rules! rhashtable_init {
    ($($args:tt)*) => {
        alloc_hooks!(rhashtable_init_noprof!($($args)*))
    };
}

extern "C" {
    pub fn __rhltable_init_noprof(
        hlt: *mut rhltable,
        params: *const rhashtable_params,
        key: *mut lock_class_key,
    ) -> i32;
}

#[macro_export]
macro_rules! rhltable_init_noprof {
    ($hlt:expr, $params:expr) => {{
        static mut __key: lock_class_key = unsafe { core::mem::zeroed() };
        unsafe { __rhltable_init_noprof($hlt, $params, core::ptr::addr_of_mut!(__key)) }
    }};
}

#[macro_export]
macro_rules! rhltable_init {
    ($($args:tt)*) => {
        alloc_hooks!(rhltable_init_noprof!($($args)*))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
