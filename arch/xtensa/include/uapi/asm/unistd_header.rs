/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependency: #include <asm/unistd_32.h>

pub const __ARCH_WANT_SYS_OLDUMOUNT: bool = true;

/*
 * sysxtensa syscall handler
 *
 * int sysxtensa (SYS_XTENSA_ATOMIC_SET,     ptr, val,    unused);
 * int sysxtensa (SYS_XTENSA_ATOMIC_ADD,     ptr, val,    unused);
 * int sysxtensa (SYS_XTENSA_ATOMIC_EXG_ADD, ptr, val,    unused);
 * int sysxtensa (SYS_XTENSA_ATOMIC_CMP_SWP, ptr, oldval, newval);
 *        a2            a6                   a3    a4      a5
 */

pub const SYS_XTENSA_RESERVED: i32 = 0;       /* don't use this */
pub const SYS_XTENSA_ATOMIC_SET: i32 = 1;     /* set variable */
pub const SYS_XTENSA_ATOMIC_EXG_ADD: i32 = 2; /* exchange memory and add */
pub const SYS_XTENSA_ATOMIC_ADD: i32 = 3;     /* add to memory */
pub const SYS_XTENSA_ATOMIC_CMP_SWP: i32 = 4; /* compare and swap */
pub const SYS_XTENSA_COUNT: i32 = 5;          /* count */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
