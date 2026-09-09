/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000,2012 MIPS Technologies, Inc. All rights reserved.
 *     Carsten Langgaard <carstenl@mips.com>
 *     Steven J. Hill <sjhill@mips.com>
 */

/*
 * Interrupts 0..15 are used for Malta ISA compatible interrupts
 */
pub const MALTA_INT_BASE: i32 = 0;

/* CPU interrupt offsets */
pub const MIPSCPU_INT_SW0: i32 = 0;
pub const MIPSCPU_INT_SW1: i32 = 1;
pub const MIPSCPU_INT_MB0: i32 = 2;
pub const MIPSCPU_INT_I8259A: i32 = MIPSCPU_INT_MB0;
pub const MIPSCPU_INT_GIC: i32 = MIPSCPU_INT_MB0; /* GIC chained interrupt */
pub const MIPSCPU_INT_MB1: i32 = 3;
pub const MIPSCPU_INT_SMI: i32 = MIPSCPU_INT_MB1;
pub const MIPSCPU_INT_MB2: i32 = 4;
pub const MIPSCPU_INT_MB3: i32 = 5;
pub const MIPSCPU_INT_COREHI: i32 = MIPSCPU_INT_MB3;
pub const MIPSCPU_INT_MB4: i32 = 6;
pub const MIPSCPU_INT_CORELO: i32 = MIPSCPU_INT_MB4;

/*
 * Interrupts 96..127 are used for Soc-it Classic interrupts
 */
pub const MSC01C_INT_BASE: i32 = 96;

/* SOC-it Classic interrupt offsets */
pub const MSC01C_INT_TMR: i32 = 0;
pub const MSC01C_INT_PCI: i32 = 1;

/*
 * Interrupts 96..127 are used for Soc-it EIC interrupts
 */
pub const MSC01E_INT_BASE: i32 = 96;

/* SOC-it EIC interrupt offsets */
pub const MSC01E_INT_SW0: i32 = 1;
pub const MSC01E_INT_SW1: i32 = 2;
pub const MSC01E_INT_MB0: i32 = 3;
pub const MSC01E_INT_I8259A: i32 = MSC01E_INT_MB0;
pub const MSC01E_INT_MB1: i32 = 4;
pub const MSC01E_INT_SMI: i32 = MSC01E_INT_MB1;
pub const MSC01E_INT_MB2: i32 = 5;
pub const MSC01E_INT_MB3: i32 = 6;
pub const MSC01E_INT_COREHI: i32 = MSC01E_INT_MB3;
pub const MSC01E_INT_MB4: i32 = 7;
pub const MSC01E_INT_CORELO: i32 = MSC01E_INT_MB4;
pub const MSC01E_INT_TMR: i32 = 8;
pub const MSC01E_INT_PCI: i32 = 9;
pub const MSC01E_INT_PERFCTR: i32 = 10;
pub const MSC01E_INT_CPUCTR: i32 = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
