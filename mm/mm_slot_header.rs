// SPDX-License-Identifier: GPL-2.0

// Translated from the Linux kernel header.  The following names are supplied
// by the corresponding dependency headers in the surrounding translation.

/*
 * struct mm_slot - hash lookup from mm to mm_slot
 * @hash: link to the mm_slots hash list
 * @mm_node: link into the mm_slots list
 * @mm: the mm that this information is valid for
 */
#[repr(C)]
pub struct mm_slot {
    pub hash: hlist_node,
    pub mm_node: list_head,
    pub mm: *mut mm_struct,
}

#[macro_export]
macro_rules! mm_slot_entry {
    ($ptr:expr, $type:ty, $member:ident) => {
        container_of!($ptr, $type, $member)
    };
}

pub unsafe fn mm_slot_alloc(cache: *mut kmem_cache) -> *mut core::ffi::c_void {
    if cache.is_null() {
        // initialization failed
        return core::ptr::null_mut();
    }
    kmem_cache_zalloc(cache, GFP_KERNEL)
}

pub unsafe fn mm_slot_free(cache: *mut kmem_cache, objp: *mut core::ffi::c_void) {
    kmem_cache_free(cache, objp);
}

/*
 * Note: mm_slot_lookup and mm_slot_insert cannot be converted to functions
 * because the hash helpers (hash_for_each_possible and hash_add) rely on the
 * actual array argument `hashtable` for sizeof() instead of pointers.
 */

#[macro_export]
macro_rules! mm_slot_lookup {
    ($hashtable:expr, $mm:expr) => {{
        let mut tmp_slot: *mut mm_slot;
        let mut mm_slot: *mut mm_slot = core::ptr::null_mut();

        hash_for_each_possible!($hashtable, tmp_slot, hash, $mm as usize);
        if $mm == (*tmp_slot).mm {
            mm_slot = tmp_slot;
            break;
        }

        mm_slot
    }};
}

#[macro_export]
macro_rules! mm_slot_insert {
    ($hashtable:expr, $mm:expr, $mm_slot:expr) => {{
        (*$mm_slot).mm = $mm;
        hash_add!($hashtable, &mut (*$mm_slot).hash, $mm as usize);
    }};
}

pub unsafe fn mm_slot_remove(slot: *mut mm_slot) {
    hash_del(&mut (*slot).hash);
    list_del(&mut (*slot).mm_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
