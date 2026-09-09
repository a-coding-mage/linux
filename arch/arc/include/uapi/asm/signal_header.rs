/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * Amit Bhor, Sameer Dhavale: Codito Technologies 2004
 */

/*
 * This is much needed for ARC sigreturn optimization.
 * This allows uClibc to piggback the addr of a sigreturn stub in sigaction,
 * which allows sigreturn based re-entry into kernel after handling signal.
 * W/o this kernel needs to "synthesize" the sigreturn trampoline on user
 * mode stack which in turn forces the following:
 * -TLB Flush (after making the stack page executable)
 * -Cache line Flush (to make I/D Cache lines coherent)
 */
pub const SA_RESTORER: u32 = 0x0400_0000;

// Dependency supplied by asm-generic/signal.h is intentionally not expanded here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
