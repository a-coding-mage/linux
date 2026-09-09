/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/glue-cache.h
 *
 *  Copyright (C) 1999-2002 Russell King
 */

// Dependency: <asm/glue.h>

/*
 * Cache Model
 * ===========
 *
 * The original preprocessor logic selects `_CACHE` when exactly one cache
 * model is configured, and selects `MULTI_CACHE` when several models apply.
 * These build-time configuration conditions are retained below as comments;
 * the selected glue symbols are supplied by the surrounding build.
 */

// #undef _CACHE
// #undef MULTI_CACHE
// CONFIG_CPU_CACHE_V4       => _CACHE = v4, or MULTI_CACHE
// CONFIG_CPU_ARM920T/ARM922T/ARM925T/ARM1020/ARM1026 => MULTI_CACHE
// CONFIG_CPU_FA526          => _CACHE = fa, or MULTI_CACHE
// CONFIG_CPU_ARM926T        => _CACHE = arm926, or MULTI_CACHE
// CONFIG_CPU_ARM940T        => _CACHE = arm940, or MULTI_CACHE
// CONFIG_CPU_ARM946E        => _CACHE = arm946, or MULTI_CACHE
// CONFIG_CPU_CACHE_V4WB     => _CACHE = v4wb, or MULTI_CACHE
// CONFIG_CPU_XSCALE         => _CACHE = xscale, or MULTI_CACHE
// CONFIG_CPU_XSC3           => _CACHE = xsc3, or MULTI_CACHE
// CONFIG_CPU_MOHAWK         => _CACHE = mohawk, or MULTI_CACHE
// CONFIG_CPU_FEROCEON       => MULTI_CACHE
// CONFIG_CPU_V6/CONFIG_CPU_V6K => _CACHE = v6, or MULTI_CACHE
// CONFIG_CPU_V7             => _CACHE = v7, or MULTI_CACHE
// CONFIG_CACHE_B15_RAC      => MULTI_CACHE
// CONFIG_CPU_CACHE_NOP      => MULTI_CACHE
// CONFIG_CPU_V7M            => MULTI_CACHE

// #error Unknown cache maintenance model

/*
 * For a single cache model, the C header aliases each generic operation to
 * the corresponding model-specific glue symbol.  Rust cannot concatenate
 * identifiers at item-definition time, so preserve the aliases as external
 * function declarations under the selected model configuration.
 */

#[cfg(not(feature = "multi_cache"))]
extern "C" {
    pub fn __cpuc_flush_icache_all();
    pub fn __cpuc_flush_kern_all();

    /* This function only has a dedicated assembly callback on the v7 cache. */
    #[cfg(feature = "cpu_cache_v7")]
    pub fn __cpuc_flush_kern_louis();
    #[cfg(not(feature = "cpu_cache_v7"))]
    pub fn __cpuc_flush_kern_louis();

    pub fn __cpuc_flush_user_all();
    pub fn __cpuc_flush_user_range(start: usize, end: usize);
    pub fn __cpuc_coherent_kern_range(start: usize, end: usize);
    pub fn __cpuc_coherent_user_range(start: usize, end: usize);
    pub fn __cpuc_flush_dcache_area(start: *mut core::ffi::c_void, size: usize);
    pub fn dmac_flush_range(start: usize, end: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
