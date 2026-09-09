/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2022-2025 HiSilicon Limited.
 */

// Translated from the C UAPI header. The `_IOWR` and `NSEC_PER_MSEC`
// symbols are supplied by the surrounding Linux bindings.

pub const DMA_MAP_BENCHMARK: usize = _IOWR!('d', 1, map_benchmark);
pub const DMA_MAP_MAX_THREADS: u32 = 1024;
pub const DMA_MAP_MAX_SECONDS: u32 = 300;
pub const DMA_MAP_MAX_TRANS_DELAY: u64 = 10 * NSEC_PER_MSEC;

pub const DMA_MAP_BIDIRECTIONAL: u32 = 0;
pub const DMA_MAP_TO_DEVICE: u32 = 1;
pub const DMA_MAP_FROM_DEVICE: u32 = 2;

pub const DMA_MAP_BENCH_SINGLE_MODE: u32 = 0;
pub const DMA_MAP_BENCH_SG_MODE: u32 = 1;
pub const DMA_MAP_BENCH_MODE_MAX: u32 = 2;

#[repr(C)]
pub struct map_benchmark {
    pub avg_map_100ns: u64, // average map latency in 100ns
    pub map_stddev: u64, // standard deviation of map latency
    pub avg_unmap_100ns: u64, // as above
    pub unmap_stddev: u64,
    pub threads: u32, // how many threads will do map/unmap in parallel
    pub seconds: u32, // how long the test will last
    pub node: i32, // which numa node this benchmark will run on
    pub dma_bits: u32, // DMA addressing capability
    pub dma_dir: u32, // DMA data direction
    pub dma_trans_ns: u32, // time for DMA transmission in ns
    pub granule: u32, // - SINGLE_MODE: number of pages mapped/unmapped per operation
    // - SG_MODE: number of scatterlist entries (each maps one page)
    pub map_mode: u8, // the mode of dma map
    pub expansion: [u8; 75], // For future use
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
