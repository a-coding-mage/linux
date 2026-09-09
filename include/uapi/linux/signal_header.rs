/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the C header:
// #include <asm/signal.h>
// #include <asm/siginfo.h>

pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 2;

/* bit-flags */
pub const SS_AUTODISARM: u32 = 1u32 << 31; /* disable sas during sighandling */
/* mask for all SS_xxx flags */
pub const SS_FLAG_BITS: u32 = SS_AUTODISARM;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
