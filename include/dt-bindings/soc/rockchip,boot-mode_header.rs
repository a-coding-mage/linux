/* SPDX-License-Identifier: GPL-2.0 */

/* high 24 bits is tag, low 8 bits is type */
pub const REBOOT_FLAG: u32 = 0x5242C300;
/* normal boot */
pub const BOOT_NORMAL: u32 = REBOOT_FLAG + 0;
/* enter bootloader rockusb mode */
pub const BOOT_BL_DOWNLOAD: u32 = REBOOT_FLAG + 1;
/* enter recovery */
pub const BOOT_RECOVERY: u32 = REBOOT_FLAG + 3;
/* enter fastboot mode */
pub const BOOT_FASTBOOT: u32 = REBOOT_FLAG + 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
