/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 MIPS Tech, LLC
 * Author: Matt Redfearn <matt.redfearn@mips.com>
 */

/*
 * The ISA revision level. This is 0 for MIPS I to V and N for
 * MIPS{32,64}rN.
 */

/*
 * If the compiler has defined __mips_isa_rev, believe it. A compiler-defined
 * ISA revision is a build-time condition not expressible from this file alone.
 * The fallback matches the C header when no such definition is supplied.
 */
pub const MIPS_ISA_REV: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
