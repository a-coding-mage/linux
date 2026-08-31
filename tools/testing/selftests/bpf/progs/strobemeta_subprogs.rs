// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2019 Facebook

pub const STROBE_MAX_INTS: u32 = 2;
pub const STROBE_MAX_STRS: u32 = 25;
pub const STROBE_MAX_MAPS: u32 = 13;
pub const STROBE_MAX_MAP_ENTRIES: u32 = 20;

// C preprocessor feature flags used before including "strobemeta.h".
// Keep their intent for the Rust-side dependency that provides strobemeta items.
// #define NO_UNROLL
// #define SUBPROGS

// Dependency intent from C: #include "strobemeta.h"
