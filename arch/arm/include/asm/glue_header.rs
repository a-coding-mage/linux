/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/glue.h
 *
 *  Copyright (C) 1997-1999 Russell King
 *  Copyright (C) 2000-2002 Deep Blue Solutions Ltd.
 *
 *  This file provides the glue to stick the processor-specific bits
 *  into the kernel in an efficient manner.  The idea is to use branches
 *  when we're only targeting one class of TLB, or indirect calls
 *  when we're targeting multiple classes of TLBs.
 */

/* C header scope: these definitions are active only when __KERNEL__ is set. */
/*
 * C preprocessor token pasting:
 *   ____glue(name, fn) expands to the identifier formed by concatenating
 *   name and fn; __glue(name, fn) delegates to ____glue.
 * Rust has no stable direct equivalent for constructing an identifier from
 * two macro arguments, so the token-pasting intent is preserved here.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
