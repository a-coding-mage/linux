/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * env.c: ARCS environment variable routines.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 */

// Dependencies supplied by the corresponding Linux/MIPS headers:
// linux/init.h, linux/kernel.h, linux/string.h, asm/fw/arc/types.h,
// and asm/sgialib.h.

pub unsafe fn ArcGetEnvironmentVariable(name: *mut CHAR) -> PCHAR {
    ARC_CALL1!(get_evar, name) as PCHAR
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
