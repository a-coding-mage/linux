/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * S3C24XX - Memory map definitions
 */

// C header guard: __ASM_PLAT_MAP_S3C_H
// Dependency: map.h

/*
 * GPIO ports
 *
 * the calculation for the VA of this must ensure that
 * it is the same distance apart from the UART in the
 * phsyical address space, as the initial mapping for the IO
 * is done as a 1:1 mapping. This puts it (currently) at
 * 0xFA800000, which is not in the way of any current mapping
 * by the base system.
 */

// S3C_ADDR_CPU is supplied by the translated map dependency.
pub const S3C64XX_VA_GPIO: usize = s3c_addr_cpu!(0x00000000);

pub const S3C64XX_VA_MODEM: usize = s3c_addr_cpu!(0x00100000);
pub const S3C64XX_VA_USB_HSPHY: usize = s3c_addr_cpu!(0x00200000);

pub const S3C_VA_USB_HSPHY: usize = S3C64XX_VA_USB_HSPHY;

// Dependency: map-s5p.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
