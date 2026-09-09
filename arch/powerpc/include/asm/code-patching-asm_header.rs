// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2018, Michael Ellerman, IBM Corporation.
 */

// Define a "site" that can be patched.
//
// This preserves the original assembler macro's placement, alignment,
// exported symbol, and relative 32-bit relocation.
#[macro_export]
macro_rules! patch_site {
    ($label:ident $name:ident) => {
        core::arch::global_asm!(concat!(
            ".pushsection \".rodata\"\n",
            ".balign 4\n",
            ".global ", stringify!($name), "\n",
            stringify!($name), ":\n",
            ".4byte ", stringify!($label), " - .\n",
            ".popsection\n",
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
