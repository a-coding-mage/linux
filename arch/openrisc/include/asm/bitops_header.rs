/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

// C header guard: __ASM_OPENRISC_BITOPS_H

/*
 * Where we haven't written assembly versions yet, we fall back to the
 * generic implementations.  Otherwise, we pull in our (hopefully)
 * optimized versions.
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/irqflags.h>
// <linux/compiler.h>
// <asm/barrier.h>
// <asm/bitops/__ffs.h>
// <asm-generic/bitops/ffz.h>
// <asm/bitops/fls.h>
// <asm/bitops/__fls.h>
// <asm-generic/bitops/fls64.h>

// C preprocessor condition: _LINUX_BITOPS_H must be defined when this header
// is included; otherwise the original header emits a preprocessing error.

// <asm-generic/bitops/sched.h>
// <asm/bitops/ffs.h>
// <asm-generic/bitops/hweight.h>
// <asm-generic/bitops/lock.h>
// <asm/bitops/atomic.h>
// <asm-generic/bitops/non-atomic.h>
// <asm-generic/bitops/le.h>
// <asm-generic/bitops/ext2-atomic.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
