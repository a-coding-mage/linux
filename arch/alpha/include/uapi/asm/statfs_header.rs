/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header guard `_ALPHA_STATFS_H` prevents multiple inclusion.

// Dependency: <linux/types.h>

/* Alpha is the only 64-bit platform with 32-bit statfs. And doesn't
   even seem to implement statfs64 */
pub type __statfs_word = u32;

// Dependency: <asm-generic/statfs.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
