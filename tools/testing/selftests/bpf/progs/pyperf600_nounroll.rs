// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

pub const STACK_MAX_LEN: i32 = 600;

// C source defines NO_UNROLL before including "pyperf.h".
pub const NO_UNROLL: bool = true;

// Dependency intent from C: #include "pyperf.h"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
