/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by linux/unistd.h:
// __NR_sigreturn

// #include <asm-generic/seccomp.h>
// Generic seccomp declarations are supplied by the corresponding dependency.

pub const __NR_seccomp_sigreturn: usize = __NR_sigreturn as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
