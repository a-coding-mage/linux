/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: __ABI_CSKY_VDSO_H

/* movi r1, 127; addi r1, (139 - 127) */
pub const SET_SYSCALL_ID: u32 = 0x20b167f1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
