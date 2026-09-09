/*
 * platform/hardware.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 Tensilica Inc.
 */

/*
 * This file contains the hardware configuration of the XT2000 board.
 */

// The C header guard `_XTENSA_XT2000_HARDWARE_H` has no Rust equivalent.
// Dependency supplied by <asm/core.h> remains external.

/*
 * On-board components.
 */

pub const SONIC83934_INTNUM: _ = XCHAL_EXTINT3_NUM;
pub const SONIC83934_ADDR: _ = IOADDR!(0x0d030000);

/*
 * V3-PCI
 */

/* The XT2000 uses the V3 as a cascaded interrupt controller for the PCI bus */

pub const IRQ_PCI_A: _ = XCHAL_NUM_INTERRUPTS + 0;
pub const IRQ_PCI_B: _ = XCHAL_NUM_INTERRUPTS + 1;
pub const IRQ_PCI_C: _ = XCHAL_NUM_INTERRUPTS + 2;

/*
 * Various other components.
 */

pub const XT2000_LED_ADDR: _ = IOADDR!(0x0d040000);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
