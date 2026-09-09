/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	coldfire.h -- Motorola ColdFire CPU specific defines
 *
 *	(C) Copyright 1999-2006, Greg Ungerer (gerg@snapgear.com)
 *	(C) Copyright 2000, Lineo (www.lineo.com)
 */

/****************************************************************************/

/*
 * Define master clock frequency. This is done at config time now.
 * No point enumerating dozens of possible clock options here. And
 * in any case new boards come along from time to time that have yet
 * another different clocking frequency.
 *
 * The C CONFIG_CLOCK_FREQ build-time symbol is represented by the
 * corresponding Rust configuration feature and external constant.
 */
#[cfg(feature = "CONFIG_CLOCK_FREQ")]
pub const MCF_CLK: usize = CONFIG_CLOCK_FREQ;

#[cfg(not(feature = "CONFIG_CLOCK_FREQ"))]
compile_error!("Don't know what your ColdFire CPU clock frequency is??");

/*
 * Define the processor internal peripherals base address.
 *
 * The majority of ColdFire parts use an MBAR register to set the
 * base address. Some have an IPSBAR register instead, and it has
 * slightly different rules on its size and alignment. Some parts
 * have fixed addresses and the internal peripherals cannot be
 * relocated in the CPU address space.
 *
 * The value of MBAR or IPSBAR is config time selectable. No MBAR or
 * IPSBAR is defined if this part has a fixed peripheral address map.
 *
 * The C CONFIG_MBAR and CONFIG_IPSBAR build-time symbols are
 * represented by the corresponding Rust configuration features and
 * external constants.
 */
#[cfg(feature = "CONFIG_MBAR")]
pub const MCF_MBAR: usize = CONFIG_MBAR;

#[cfg(feature = "CONFIG_IPSBAR")]
pub const MCF_IPSBAR: usize = CONFIG_IPSBAR;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
