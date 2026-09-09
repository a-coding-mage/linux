/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by linux/linkage.h and linux/init.h are intentionally
// not reproduced here.

/* Define the prototype for start_kernel here, rather than cluttering
   up something else. */

// C attributes: asmlinkage, __init, and __noreturn.
unsafe extern "C" {
    pub fn start_kernel() -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
