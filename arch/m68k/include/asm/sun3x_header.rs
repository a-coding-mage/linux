/* SPDX-License-Identifier: GPL-2.0 */

/* hardware addresses */
pub const SUN3X_IOMMU: u32 = 0x60000000;
pub const SUN3X_ENAREG: u32 = 0x61000000;
pub const SUN3X_INTREG: u32 = 0x61001400;
pub const SUN3X_DIAGREG: u32 = 0x61001800;
pub const SUN3X_ZS1: u32 = 0x62000000;
pub const SUN3X_ZS2: u32 = 0x62002000;
pub const SUN3X_LANCE: u32 = 0x65002000;
pub const SUN3X_EEPROM: u32 = 0x64000000;
pub const SUN3X_IDPROM: u32 = 0x640007d8;
pub const SUN3X_VIDEO_BASE: u32 = 0x50000000;
pub const SUN3X_VIDEO_P4ID: u32 = 0x50300000;
pub const SUN3X_ESP_BASE: u32 = 0x66000000;
pub const SUN3X_ESP_DMA: u32 = 0x66001000;
pub const SUN3X_FDC: u32 = 0x6e000000;
pub const SUN3X_FDC_FCR: u32 = 0x6e000400;
pub const SUN3X_FDC_FVR: u32 = 0x6e000800;

/* some NVRAM addresses */
pub const SUN3X_EEPROM_CONS: u32 = SUN3X_EEPROM + 0x1f;
pub const SUN3X_EEPROM_PORTA: u32 = SUN3X_EEPROM + 0x58;
pub const SUN3X_EEPROM_PORTB: u32 = SUN3X_EEPROM + 0x60;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
