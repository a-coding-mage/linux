/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <ppc-asm.h>
// #include <asm/unistd.h>

// Fallback used by the C header when __NR_switch_endian is not provided.
pub const __NR_switch_endian: i32 = 363;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
