// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2019 Facebook

pub const STROBE_MAX_INTS: i32 = 2;
pub const STROBE_MAX_STRS: i32 = 25;
pub const STROBE_MAX_MAPS: i32 = 30;
pub const STROBE_MAX_MAP_ENTRIES: i32 = 20;

// C preprocessor flag used before including "strobemeta.h".
pub const NO_UNROLL: bool = true;

// The original C source includes "strobemeta.h" here, which supplies the
// implementation parameterized by the constants above.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
