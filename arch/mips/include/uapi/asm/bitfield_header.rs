/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 by Ralf Baechle <ralf@linux-mips.org>
 */

/*
 * Bitfields depend on byte order.  The original C header selects the
 * expansion according to the MIPS endianness configuration.
 */

#[cfg(target_endian = "big")]
macro_rules! __BITFIELD_FIELD {
    ($($field:tt)* ; $($more:tt)*) => {
        $($field)*;
        $($more)*
    };
}

#[cfg(target_endian = "little")]
macro_rules! __BITFIELD_FIELD {
    ($($field:tt)* ; $($more:tt)*) => {
        $($more)*
        $($field)*;
    };
}

/* The C source errors when neither __MIPSEB__ nor __MIPSEL__ is defined. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
