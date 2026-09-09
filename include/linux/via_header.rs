/* SPDX-License-Identifier: GPL-2.0 */
/* Miscellaneous definitions for VIA chipsets
   Currently used only by drivers/parport/parport_pc.c */

/* Values for SuperIO function select configuration register */
pub const VIA_FUNCTION_PARPORT_SPP: u32 = 0x00;
pub const VIA_FUNCTION_PARPORT_ECP: u32 = 0x01;
pub const VIA_FUNCTION_PARPORT_EPP: u32 = 0x02;
pub const VIA_FUNCTION_PARPORT_DISABLE: u32 = 0x03;
pub const VIA_FUNCTION_PROBE: u32 = 0xFF; /* Special magic value to be used in code, not to be written into chip */

/* Bits for parallel port mode configuration register */
pub const VIA_PARPORT_ECPEPP: u32 = 0x20;
pub const VIA_PARPORT_BIDIR: u32 = 0x80;

/* VIA configuration registers */
pub const VIA_CONFIG_INDEX: u32 = 0x3F0;
pub const VIA_CONFIG_DATA: u32 = 0x3F1;

/* Mask for parallel port IRQ bits (in ISA PnP IRQ routing register 1) */
pub const VIA_IRQCONTROL_PARALLEL: u32 = 0xF0;
/* Mask for parallel port DMA bits (in ISA PnP DMA routing register) */
pub const VIA_DMACONTROL_PARALLEL: u32 = 0x0C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
