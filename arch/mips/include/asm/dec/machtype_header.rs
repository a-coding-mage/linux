/*
 * Various machine type macros
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1998, 2000 Harald Koerfgen
 */

// Dependency supplied by asm/bootinfo.h is expected to provide `mips_machtype`
// and the MACH_* machine-type constants.

macro_rules! TURBOCHANNEL {
    () => {
        (mips_machtype == MACH_DS5000_200
            || mips_machtype == MACH_DS5000_1XX
            || mips_machtype == MACH_DS5000_XX
            || mips_machtype == MACH_DS5000_2X0
            || mips_machtype == MACH_DS5900)
    };
}

macro_rules! IOASIC {
    () => {
        (mips_machtype == MACH_DS5000_1XX
            || mips_machtype == MACH_DS5000_XX
            || mips_machtype == MACH_DS5000_2X0
            || mips_machtype == MACH_DS5900)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
