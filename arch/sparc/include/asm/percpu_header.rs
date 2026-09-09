/* SPDX-License-Identifier: GPL-2.0 */

// Build-time selection preserved from the original header:
// if both `__sparc__` and `__arch64__` are defined, use asm/percpu_64.h;
// otherwise, use asm/percpu_32.h.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[path = "percpu_64.rs"]
mod percpu;

#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
#[path = "percpu_32.rs"]
mod percpu;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
