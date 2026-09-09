/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/hash.h, linux/list_bl.h, linux/list.h,
// linux/atomic.h, and linux/fs.h.

pub struct mb_cache;

/* Cache entry flags */
pub const MBE_REFERENCED_B: usize = 0;
pub const MBE_REUSABLE_B: usize = 1;

#[repr(C)]
pub struct mb_cache_entry {
	/* List of entries in cache - protected by cache->c_list_lock */
	pub e_list: list_head,
	/*
	 * Hash table list - protected by hash chain bitlock. The entry is
	 * guaranteed to be hashed while e_refcnt > 0.
	 */
	pub e_hash_list: hlist_bl_node,
	/*
	 * Entry refcount. Once it reaches zero, entry is unhashed and freed.
	 * While refcount > 0, the entry is guaranteed to stay in the hash and
	 * e.g. mb_cache_entry_try_delete() will fail.
	 */
	pub e_refcnt: atomic_t,
	/* Key in hash - stable during lifetime of the entry */
	pub e_key: u32,
	pub e_flags: ::core::ffi::c_ulong,
	/* User provided value - stable during lifetime of the entry */
	pub e_value: u64,
}

unsafe extern "C" {
	pub fn mb_cache_create(bucket_bits: ::core::ffi::c_int) -> *mut mb_cache;
	pub fn mb_cache_destroy(cache: *mut mb_cache);

	pub fn mb_cache_entry_create(
		cache: *mut mb_cache,
		mask: gfp_t,
		key: u32,
		value: u64,
		reusable: bool,
	) -> ::core::ffi::c_int;
	pub fn __mb_cache_entry_free(cache: *mut mb_cache, entry: *mut mb_cache_entry);
	pub fn mb_cache_entry_wait_unused(entry: *mut mb_cache_entry);

	pub fn atomic_dec_return(v: *mut atomic_t) -> ::core::ffi::c_uint;
	pub fn wake_up_var(var: *mut atomic_t);
}

#[inline]
pub unsafe fn mb_cache_entry_put(cache: *mut mb_cache, entry: *mut mb_cache_entry) {
	let cnt: ::core::ffi::c_uint = atomic_dec_return(&mut (*entry).e_refcnt);

	if cnt > 0 {
		if cnt <= 2 {
			wake_up_var(&mut (*entry).e_refcnt);
		}
		return;
	}
	__mb_cache_entry_free(cache, entry);
}

unsafe extern "C" {
	pub fn mb_cache_entry_delete_or_get(
		cache: *mut mb_cache,
		key: u32,
		value: u64,
	) -> *mut mb_cache_entry;
	pub fn mb_cache_entry_get(
		cache: *mut mb_cache,
		key: u32,
		value: u64,
	) -> *mut mb_cache_entry;
	pub fn mb_cache_entry_find_first(cache: *mut mb_cache, key: u32) -> *mut mb_cache_entry;
	pub fn mb_cache_entry_find_next(
		cache: *mut mb_cache,
		entry: *mut mb_cache_entry,
	) -> *mut mb_cache_entry;
	pub fn mb_cache_entry_touch(cache: *mut mb_cache, entry: *mut mb_cache_entry);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
