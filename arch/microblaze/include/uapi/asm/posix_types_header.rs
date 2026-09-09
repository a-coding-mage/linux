/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub type __kernel_mode_t = u16;

// Equivalent of the C self-referential macro: #define __kernel_mode_t __kernel_mode_t
// Dependency intent preserved from: #include <asm-generic/posix_types.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
