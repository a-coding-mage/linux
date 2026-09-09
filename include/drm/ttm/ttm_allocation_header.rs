/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/* Copyright (c) 2025 Valve Corporation */

/*
 * TTM_ALLOCATION_POOL_BENEFICIAL_ORDER(n)
 * Max order which caller can benefit from.
 */
#[macro_export]
macro_rules! TTM_ALLOCATION_POOL_BENEFICIAL_ORDER {
    ($n:expr) => {
        ($n) & 0xff
    };
}

/* Use coherent DMA allocations. */
#[macro_export]
macro_rules! TTM_ALLOCATION_POOL_USE_DMA_ALLOC {
    () => {
        BIT(8)
    };
}

/* Use GFP_DMA32 allocations. */
#[macro_export]
macro_rules! TTM_ALLOCATION_POOL_USE_DMA32 {
    () => {
        BIT(9)
    };
}

/* Do not convert ENOSPC from resource managers to ENOMEM. */
#[macro_export]
macro_rules! TTM_ALLOCATION_PROPAGATE_ENOSPC {
    () => {
        BIT(10)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
