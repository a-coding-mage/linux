// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

pub const STACK_MAX_LEN: i32 = 50;

// C source defines GLOBAL_FUNC before including "pyperf.h".
// The declarations and definitions from that header are external to this
// isolated translation unit and are intentionally not reproduced here.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
