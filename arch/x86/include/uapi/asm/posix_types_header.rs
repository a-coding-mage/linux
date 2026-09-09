/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// This header is intentionally empty for kernel builds (`__KERNEL__`).
//
// For userspace builds, select the architecture-specific POSIX types header:
// - `__i386__`:    <asm/posix_types_32.h>
// - `__ILP32__`:   <asm/posix_types_x32.h>
// - otherwise:     <asm/posix_types_64.h>
//
// The referenced architecture-specific declarations are supplied by other
// translated headers and are not redefined here.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
