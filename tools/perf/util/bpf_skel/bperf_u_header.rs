// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Facebook

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum bperf_filter_type {
    BPERF_FILTER_GLOBAL = 1,
    BPERF_FILTER_CPU = 2,
    BPERF_FILTER_PID = 3,
    BPERF_FILTER_TGID = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bperf_filter_value {
    pub accum_key: __u32,
    pub exited: __u8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
