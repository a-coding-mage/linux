/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <ppc-asm.h>
// #include <asm/unistd.h>

// Fallback used by the C header when __NR_switch_endian is not provided.
pub const __NR_switch_endian: i32 = 363;
