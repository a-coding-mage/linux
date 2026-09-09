/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * This header provides constants for Renesas RZ/G2L family IRQC bindings.
 *
 * Copyright (C) 2022 Renesas Electronics Corp.
 *
 */

/* NMI maps to SPI0 */
pub const RZG2L_NMI: u32 = 0;

/* IRQ0-7 map to SPI1-8 */
pub const RZG2L_IRQ0: u32 = 1;
pub const RZG2L_IRQ1: u32 = 2;
pub const RZG2L_IRQ2: u32 = 3;
pub const RZG2L_IRQ3: u32 = 4;
pub const RZG2L_IRQ4: u32 = 5;
pub const RZG2L_IRQ5: u32 = 6;
pub const RZG2L_IRQ6: u32 = 7;
pub const RZG2L_IRQ7: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
