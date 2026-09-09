/*
 * zsmalloc memory allocator
 *
 * Copyright (C) 2011  Nitin Gupta
 * Copyright (C) 2012, 2013 Minchan Kim
 *
 * This code is released using a dual license strategy: BSD/GPL
 * You can choose the license that better fits your requirements.
 *
 * Released under the terms of 3-clause BSD License
 * Released under the terms of GNU General Public License Version 2.0
 */

#[repr(C)]
pub struct zs_pool_stats {
	/* How many pages were migrated (freed) */
	pub pages_compacted: atomic_long_t,
}

pub struct zs_pool;
pub struct scatterlist;

extern "C" {
	pub fn zs_create_pool(name: *const c_char) -> *mut zs_pool;
	pub fn zs_destroy_pool(pool: *mut zs_pool);

	pub fn zs_malloc(
		pool: *mut zs_pool,
		size: size_t,
		flags: gfp_t,
		nid: c_int,
	) -> c_ulong;
	pub fn zs_free(pool: *mut zs_pool, obj: c_ulong);

	pub fn zs_huge_class_size(pool: *mut zs_pool) -> size_t;

	pub fn zs_get_total_pages(pool: *mut zs_pool) -> c_ulong;
	pub fn zs_compact(pool: *mut zs_pool) -> c_ulong;

	pub fn zs_lookup_class_index(pool: *mut zs_pool, size: c_uint) -> c_uint;

	pub fn zs_pool_stats(pool: *mut zs_pool, stats: *mut zs_pool_stats);

	pub fn zs_obj_read_begin(
		pool: *mut zs_pool,
		handle: c_ulong,
		mem_len: size_t,
		local_copy: *mut c_void,
	) -> *mut c_void;
	pub fn zs_obj_read_end(
		pool: *mut zs_pool,
		handle: c_ulong,
		mem_len: size_t,
		handle_mem: *mut c_void,
	);
	pub fn zs_obj_read_sg_begin(
		pool: *mut zs_pool,
		handle: c_ulong,
		sg: *mut scatterlist,
		mem_len: size_t,
	);
	pub fn zs_obj_read_sg_end(pool: *mut zs_pool, handle: c_ulong);
	pub fn zs_obj_write(
		pool: *mut zs_pool,
		handle: c_ulong,
		handle_mem: *mut c_void,
		mem_len: size_t,
	);

	pub static zsmalloc_mops: movable_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
