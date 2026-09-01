// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C macro: #define barrier_var(var) /**/
macro_rules! barrier_var {
    ($var:expr) => {};
}

// C macro: #define UNROLL
const UNROLL: () = ();

// C macro: #define INLINE __noinline
// Depends on the BPF/build environment's Rust equivalent of __noinline.
const INLINE: () = ();

// C include dependency: "profiler.inc.h"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
