/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999, 2000, 2003 Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependency supplied by the surrounding translation unit:
// <asm/asm-offsets.h>

/* C stringification helpers. */
macro_rules! __str2 {
    ($x:tt) => { stringify!($x) };
}

macro_rules! __str {
    ($x:tt) => { __str2!($x) };
}

/* CONFIG_32BIT conditional from the original header. */
#[cfg(CONFIG_32BIT)]
macro_rules! save_static_function {
    ($symbol:ident) => {
        core::arch::global_asm!(concat!(
            ".text\n\t",
            ".globl\t__", stringify!($symbol), "\n\t",
            ".align\t2\n\t",
            ".type\t__", stringify!($symbol), ", @function\n\t",
            ".ent\t__", stringify!($symbol), ", 0\n__",
            stringify!($symbol), ":\n\t",
            ".frame\t$29, 0, $31\n\t",
            "sw\t$16,", __str!(PT_R16), "($29)\t\t\t# save_static_function\n\t",
            "sw\t$17,", __str!(PT_R17), "($29)\n\t",
            "sw\t$18,", __str!(PT_R18), "($29)\n\t",
            "sw\t$19,", __str!(PT_R19), "($29)\n\t",
            "sw\t$20,", __str!(PT_R20), "($29)\n\t",
            "sw\t$21,", __str!(PT_R21), "($29)\n\t",
            "sw\t$22,", __str!(PT_R22), "($29)\n\t",
            "sw\t$23,", __str!(PT_R23), "($29)\n\t",
            "sw\t$30,", __str!(PT_R30), "($29)\n\t",
            "j\t", stringify!($symbol), "\n\t",
            ".end\t__", stringify!($symbol), "\n\t",
            ".size\t__", stringify!($symbol), ",. - __", stringify!($symbol)
        ));
    };
}

/* CONFIG_64BIT conditional from the original header. */
#[cfg(CONFIG_64BIT)]
macro_rules! save_static_function {
    ($symbol:ident) => {
        core::arch::global_asm!(concat!(
            ".text\n\t",
            ".globl\t__", stringify!($symbol), "\n\t",
            ".align\t2\n\t",
            ".type\t__", stringify!($symbol), ", @function\n\t",
            ".ent\t__", stringify!($symbol), ", 0\n__",
            stringify!($symbol), ":\n\t",
            ".frame\t$29, 0, $31\n\t",
            "sd\t$16,", __str!(PT_R16), "($29)\t\t\t# save_static_function\n\t",
            "sd\t$17,", __str!(PT_R17), "($29)\n\t",
            "sd\t$18,", __str!(PT_R18), "($29)\n\t",
            "sd\t$19,", __str!(PT_R19), "($29)\n\t",
            "sd\t$20,", __str!(PT_R20), "($29)\n\t",
            "sd\t$21,", __str!(PT_R21), "($29)\n\t",
            "sd\t$22,", __str!(PT_R22), "($29)\n\t",
            "sd\t$23,", __str!(PT_R23), "($29)\n\t",
            "sd\t$30,", __str!(PT_R30), "($29)\n\t",
            "j\t", stringify!($symbol), "\n\t",
            ".end\t__", stringify!($symbol), "\n\t",
            ".size\t__", stringify!($symbol), ",. - __", stringify!($symbol)
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
