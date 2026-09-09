/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * This header provides constants for the ARM GIC.
 */

// Dependency: <dt-bindings/interrupt-controller/irq.h>

/* interrupt specifier cell 0 */

pub const GIC_SPI: i32 = 0;
pub const GIC_PPI: i32 = 1;
pub const GIC_ESPI: i32 = 2;
pub const GIC_EPPI: i32 = 3;

/*
 * Interrupt specifier cell 2.
 * The flags in irq.h are valid, plus those below.
 */
macro_rules! GIC_CPU_MASK_RAW {
    ($x:expr) => {
        ($x) << 8
    };
}

macro_rules! GIC_CPU_MASK_SIMPLE {
    ($num:expr) => {
        GIC_CPU_MASK_RAW!((1 << ($num)) - 1)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
