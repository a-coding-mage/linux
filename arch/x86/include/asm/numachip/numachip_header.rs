/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Numascale NumaConnect-specific header file
 *
 * Copyright (C) 2012 Numascale AS. All rights reserved.
 *
 * Send feedback to <support@numascale.com>
 *
 */

// C declaration: extern u8 numachip_system;
// C declaration: extern int __init pci_numachip_init(void);
// The C `__init` annotation is a build/link-time placement attribute.
unsafe extern "C" {
    pub static mut numachip_system: u8;
    pub fn pci_numachip_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
