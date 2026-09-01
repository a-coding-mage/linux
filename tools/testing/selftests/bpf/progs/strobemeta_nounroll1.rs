// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2019 Facebook

pub const STROBE_MAX_INTS: u32 = 2;
pub const STROBE_MAX_STRS: u32 = 25;
pub const STROBE_MAX_MAPS: u32 = 13;
pub const STROBE_MAX_MAP_ENTRIES: u32 = 20;

// C source defines NO_UNROLL before including "strobemeta.h"; the shared
// strobemeta implementation/declarations are external to this isolated file.
pub const NO_UNROLL: bool = true;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
