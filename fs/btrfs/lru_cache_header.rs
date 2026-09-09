/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/maple_tree.h, and linux/list.h.

/*
 * A cache entry. This is meant to be embedded in a structure of a user of
 * this module. Similar to how struct list_head and struct rb_node are used.
 *
 * Note: it should be embedded as the first element in a struct (offset 0), and
 * this module assumes it was allocated with kmalloc(), so it calls kfree() when
 * it needs to free an entry.
 */
#[repr(C)]
pub struct btrfs_lru_cache_entry {
    pub lru_list: list_head,
    pub key: u64,
    /*
     * Optional generation associated to a key. Use 0 if not needed/used.
     * Entries with the same key and different generations are stored in a
     * linked list, so use this only for cases where there's a small number
     * of different generations.
     */
    pub gen: u64,
    /*
     * The maple tree uses unsigned long type for the keys, which is 32 bits
     * on 32 bits systems, and 64 bits on 64 bits systems. So if we want to
     * use something like inode numbers as keys, which are always a u64, we
     * have to deal with this in a special way - we store the key in the
     * entry itself, as a u64, and the values inserted into the maple tree
     * are linked lists of entries - so in case we are on a 64 bits system,
     * that list always has a single entry, while on 32 bits systems it
     * may have more than one, with each entry having the same value for
     * their lower 32 bits of the u64 key.
     */
    pub list: list_head,
}

#[repr(C)]
pub struct btrfs_lru_cache {
    pub lru_list: list_head,
    pub entries: maple_tree,
    /* Number of entries stored in the cache. */
    pub size: ::core::ffi::c_uint,
    /* Maximum number of entries the cache can have. */
    pub max_size: ::core::ffi::c_uint,
}

#[macro_export]
macro_rules! btrfs_lru_cache_for_each_entry_safe {
    ($cache:expr, $entry:expr, $tmp:expr) => {
        list_for_each_entry_safe_reverse!($entry, $tmp, &($cache).lru_list, lru_list)
    };
}

pub unsafe fn btrfs_lru_cache_lru_entry(
    cache: *mut btrfs_lru_cache,
) -> *mut btrfs_lru_cache_entry {
    list_first_entry_or_null!(
        unsafe { &(*cache).lru_list },
        btrfs_lru_cache_entry,
        lru_list
    )
}

unsafe extern "C" {
    pub fn btrfs_lru_cache_init(cache: *mut btrfs_lru_cache, max_size: ::core::ffi::c_uint);
    pub fn btrfs_lru_cache_lookup(
        cache: *mut btrfs_lru_cache,
        key: u64,
        gen: u64,
    ) -> *mut btrfs_lru_cache_entry;
    pub fn btrfs_lru_cache_store(
        cache: *mut btrfs_lru_cache,
        new_entry: *mut btrfs_lru_cache_entry,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn btrfs_lru_cache_remove(
        cache: *mut btrfs_lru_cache,
        entry: *mut btrfs_lru_cache_entry,
    );
    pub fn btrfs_lru_cache_clear(cache: *mut btrfs_lru_cache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
