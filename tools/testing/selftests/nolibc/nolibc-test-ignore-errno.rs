/* SPDX-License-Identifier: GPL-2.0 */

pub const NOLIBC_IGNORE_ERRNO: bool = true;

/* Include all of nolibc and make sure everything compiles */
/* C source included <stdlib.h> after defining NOLIBC_IGNORE_ERRNO. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
