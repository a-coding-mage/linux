/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// External dependency supplied by the surrounding environment:
// struct rb_root_cached;
// typedef unsigned int u32;
// typedef signed long long s64;

#[repr(C)]
pub struct range_tree {
	/* root of interval tree */
	pub it_root: rb_root_cached,
	/* root of rbtree of interval sizes */
	pub range_size_root: rb_root_cached,
}

extern "C" {
	pub fn range_tree_init(rt: *mut range_tree);
	pub fn range_tree_destroy(rt: *mut range_tree);

	pub fn range_tree_clear(rt: *mut range_tree, start: u32, len: u32) -> i32;
	pub fn range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> i32;
	pub fn is_range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> i32;
	pub fn range_tree_find(rt: *mut range_tree, len: u32) -> s64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
