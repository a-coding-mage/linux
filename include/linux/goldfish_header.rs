/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux I/O layer:
// #include <linux/io.h>

/* Helpers for Goldfish virtual platform */

// Equivalent to:
// #ifndef gf_ioread32
// #define gf_ioread32 ioread32
// #endif
pub use ioread32 as gf_ioread32;

// Equivalent to:
// #ifndef gf_iowrite32
// #define gf_iowrite32 iowrite32
// #endif
pub use iowrite32 as gf_iowrite32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
