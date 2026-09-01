// SPDX-License-Identifier: GPL-2.0-only

// C source condition:
// If HAVE_GENHDR is defined, include "autoconf.h"; otherwise define
// CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS for i386, x86_64, s390x, or aarch64.
// The generated-header include is an external build dependency in Rust.

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "s390x",
    target_arch = "aarch64"
))]
pub const CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS: i32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
