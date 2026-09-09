/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-hp300.h -- HP9000/300-specific boot information definitions
*/

// C header guard: _UAPI_ASM_M68K_BOOTINFO_HP300_H

/*
 *  HP9000/300-specific tags
 */

pub const BI_HP300_MODEL: u32 = 0x8000; // model (__be32)
pub const BI_HP300_UART_SCODE: u32 = 0x8001; // UART select code (__be32)
pub const BI_HP300_UART_ADDR: u32 = 0x8002; // phys. addr of UART (__be32)

/*
 *  HP9000/300 and /400 models (BI_HP300_MODEL)
 *
 * This information was taken from NetBSD
 */

pub const HP_320: u32 = 0; // 16MHz 68020+HP MMU+16K external cache
pub const HP_330: u32 = 1; // 16MHz 68020+68851 MMU
pub const HP_340: u32 = 2; // 16MHz 68030
pub const HP_345: u32 = 3; // 50MHz 68030+32K external cache
pub const HP_350: u32 = 4; // 25MHz 68020+HP MMU+32K external cache
pub const HP_360: u32 = 5; // 25MHz 68030
pub const HP_370: u32 = 6; // 33MHz 68030+64K external cache
pub const HP_375: u32 = 7; // 50MHz 68030+32K external cache
pub const HP_380: u32 = 8; // 25MHz 68040
pub const HP_385: u32 = 9; // 33MHz 68040

pub const HP_400: u32 = 10; // 50MHz 68030+32K external cache
pub const HP_425T: u32 = 11; // 25MHz 68040 - model 425t
pub const HP_425S: u32 = 12; // 25MHz 68040 - model 425s
pub const HP_425E: u32 = 13; // 25MHz 68040 - model 425e
pub const HP_433T: u32 = 14; // 33MHz 68040 - model 433t
pub const HP_433S: u32 = 15; // 33MHz 68040 - model 433s

/*
 *  Latest HP9000/300 bootinfo version
 */

pub const HP300_BOOTI_VERSION: _ = MK_BI_VERSION!(2, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
