// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2021 Facebook */

pub const STROBE_MAX_INTS: i32 = 2;
pub const STROBE_MAX_STRS: i32 = 25;
pub const STROBE_MAX_MAPS: i32 = 100;
pub const STROBE_MAX_MAP_ENTRIES: i32 = 20;

// USE_BPF_LOOP was defined before including "strobemeta.h" in the C source.
pub const USE_BPF_LOOP: bool = true;

// Depends on declarations and definitions from "strobemeta.h".
