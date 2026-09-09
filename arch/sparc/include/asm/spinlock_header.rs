/* SPDX-License-Identifier: GPL-2.0 */

// The C header selects the architecture-specific spinlock implementation:
// `asm/spinlock_64.h` when `__sparc__` and `__arch64__` are defined, otherwise
// `asm/spinlock_32.h`. Those external dependencies are intentionally not
// implemented here.

#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
// Dependency corresponding to <asm/spinlock_64.h>.
mod spinlock_64;

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
// Dependency corresponding to <asm/spinlock_32.h>.
mod spinlock_32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
