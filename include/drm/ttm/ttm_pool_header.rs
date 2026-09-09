/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

use core::ffi::{c_int, c_long, c_ulong, c_uint};

// Dependencies supplied by the surrounding kernel translation.
// #include <linux/mmzone.h>
// #include <linux/llist.h>
// #include <linux/spinlock.h>
// #include <linux/list_lru.h>
// #include <drm/ttm/ttm_caching.h>

pub struct device;
pub struct seq_file;
pub struct ttm_backup_flags;
pub struct ttm_operation_ctx;
pub struct ttm_tt;
pub struct list_head;
pub struct list_lru;

/**
 * struct ttm_pool_type - Pool for a certain memory type
 *
 * @pool: the pool we belong to, might be NULL for the global ones
 * @order: the allocation order our pages have
 * @caching: the caching type our pages have
 * @shrinker_list: our place on the global shrinker list
 * @pages: the lru_list of pages in the pool
 */
#[repr(C)]
pub struct ttm_pool_type {
    pub pool: *mut ttm_pool,
    pub order: c_uint,
    pub caching: ttm_caching,

    pub shrinker_list: list_head,
    pub pages: list_lru,
}

/**
 * struct ttm_pool - Pool for all caching and orders
 *
 * @dev: the device we allocate pages for
 * @nid: which numa node to use
 * @alloc_flags: TTM_ALLOCATION_POOL_* flags
 * @caching: pools for each caching/order
 */
#[repr(C)]
pub struct ttm_pool {
    pub dev: *mut device,
    pub nid: c_int,
    pub alloc_flags: c_uint,
    pub caching: [ttm_pool_caching; TTM_NUM_CACHING_TYPES],
}

#[repr(C)]
pub struct ttm_pool_caching {
    pub orders: [ttm_pool_type; NR_PAGE_ORDERS],
}

extern "C" {
    pub fn ttm_pool_alloc(
        pool: *mut ttm_pool,
        tt: *mut ttm_tt,
        ctx: *mut ttm_operation_ctx,
    ) -> c_int;
    pub fn ttm_pool_free(pool: *mut ttm_pool, tt: *mut ttm_tt);

    pub fn ttm_pool_init(
        pool: *mut ttm_pool,
        dev: *mut device,
        nid: c_int,
        alloc_flags: c_uint,
    );
    pub fn ttm_pool_fini(pool: *mut ttm_pool);

    pub fn ttm_pool_debugfs(pool: *mut ttm_pool, m: *mut seq_file) -> c_int;

    pub fn ttm_pool_drop_backed_up(tt: *mut ttm_tt);

    pub fn ttm_pool_backup(
        pool: *mut ttm_pool,
        ttm: *mut ttm_tt,
        flags: *const ttm_backup_flags,
    ) -> c_long;
    pub fn ttm_pool_restore_and_alloc(
        pool: *mut ttm_pool,
        tt: *mut ttm_tt,
        ctx: *const ttm_operation_ctx,
    ) -> c_int;

    pub fn ttm_pool_mgr_init(num_pages: c_ulong) -> c_int;
    pub fn ttm_pool_mgr_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
