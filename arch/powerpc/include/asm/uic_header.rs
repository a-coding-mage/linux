/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * IBM PPC4xx UIC external definitions and structure.
 *
 * Maintainer: David Gibson <dwg@au1.ibm.com>
 * Copyright 2007 IBM Corporation.
 */

/* C declarations were guarded by __KERNEL__. Preserve that build-time intent. */
#[cfg(feature = "kernel")]
extern "C" {
    pub fn uic_init_tree();
    pub fn uic_get_irq() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
