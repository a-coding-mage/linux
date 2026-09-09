/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 98, 99, 2000, 01, 05 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2001 MIPS Technologies, Inc.
 */

// Declaration supplied by the translated dependency context.
unsafe extern "C" {
    pub fn ack_bad_irq(irq: ::core::ffi::c_uint);
}

// C macro: #define ack_bad_irq ack_bad_irq (an identity alias).

// Dependency intent: equivalent to including <asm-generic/hardirq.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
