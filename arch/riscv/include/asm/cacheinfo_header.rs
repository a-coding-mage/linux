/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 SiFive
 */

/* Dependency: linux/cacheinfo.h */

#[repr(C)]
pub struct riscv_cacheinfo_ops {
    pub get_priv_group: Option<unsafe extern "C" fn(
        this_leaf: *mut cacheinfo,
    ) -> *const attribute_group>,
}

extern "C" {
    pub fn riscv_set_cacheinfo_ops(ops: *mut riscv_cacheinfo_ops);
    pub fn get_cache_size(level: u32, type_: cache_type) -> usize;
    pub fn get_cache_geometry(level: u32, type_: cache_type) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
