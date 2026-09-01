// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Depends on constants from <asm-generic/errno.h>.
// C header locally undefines EDEADLOCK before and after that include.

pub const EDEADLOCK: i32 = 58; // File locking deadlock error

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
