/* SPDX-License-Identifier: GPL-2.0 */
/**
 * This header provides index for the reset controller
 * based on hi6220 SoC.
 */

pub const PERIPH_RSTDIS0_MMC0: u32 = 0x000;
pub const PERIPH_RSTDIS0_MMC1: u32 = 0x001;
pub const PERIPH_RSTDIS0_MMC2: u32 = 0x002;
pub const PERIPH_RSTDIS0_NANDC: u32 = 0x003;
pub const PERIPH_RSTDIS0_USBOTG_BUS: u32 = 0x004;
pub const PERIPH_RSTDIS0_POR_PICOPHY: u32 = 0x005;
pub const PERIPH_RSTDIS0_USBOTG: u32 = 0x006;
pub const PERIPH_RSTDIS0_USBOTG_32K: u32 = 0x007;
pub const PERIPH_RSTDIS1_HIFI: u32 = 0x100;
pub const PERIPH_RSTDIS1_DIGACODEC: u32 = 0x105;
pub const PERIPH_RSTEN2_IPF: u32 = 0x200;
pub const PERIPH_RSTEN2_SOCP: u32 = 0x201;
pub const PERIPH_RSTEN2_DMAC: u32 = 0x202;
pub const PERIPH_RSTEN2_SECENG: u32 = 0x203;
pub const PERIPH_RSTEN2_ABB: u32 = 0x204;
pub const PERIPH_RSTEN2_HPM0: u32 = 0x205;
pub const PERIPH_RSTEN2_HPM1: u32 = 0x206;
pub const PERIPH_RSTEN2_HPM2: u32 = 0x207;
pub const PERIPH_RSTEN2_HPM3: u32 = 0x208;
pub const PERIPH_RSTEN3_CSSYS: u32 = 0x300;
pub const PERIPH_RSTEN3_I2C0: u32 = 0x301;
pub const PERIPH_RSTEN3_I2C1: u32 = 0x302;
pub const PERIPH_RSTEN3_I2C2: u32 = 0x303;
pub const PERIPH_RSTEN3_I2C3: u32 = 0x304;
pub const PERIPH_RSTEN3_UART1: u32 = 0x305;
pub const PERIPH_RSTEN3_UART2: u32 = 0x306;
pub const PERIPH_RSTEN3_UART3: u32 = 0x307;
pub const PERIPH_RSTEN3_UART4: u32 = 0x308;
pub const PERIPH_RSTEN3_SSP: u32 = 0x309;
pub const PERIPH_RSTEN3_PWM: u32 = 0x30a;
pub const PERIPH_RSTEN3_BLPWM: u32 = 0x30b;
pub const PERIPH_RSTEN3_TSENSOR: u32 = 0x30c;
pub const PERIPH_RSTEN3_DAPB: u32 = 0x312;
pub const PERIPH_RSTEN3_HKADC: u32 = 0x313;
pub const PERIPH_RSTEN3_CODEC_SSI: u32 = 0x314;
pub const PERIPH_RSTEN3_PMUSSI1: u32 = 0x316;
pub const PERIPH_RSTEN8_RS0: u32 = 0x400;
pub const PERIPH_RSTEN8_RS2: u32 = 0x401;
pub const PERIPH_RSTEN8_RS3: u32 = 0x402;
pub const PERIPH_RSTEN8_MS0: u32 = 0x403;
pub const PERIPH_RSTEN8_MS2: u32 = 0x405;
pub const PERIPH_RSTEN8_XG2RAM0: u32 = 0x406;
pub const PERIPH_RSTEN8_X2SRAM_TZMA: u32 = 0x407;
pub const PERIPH_RSTEN8_SRAM: u32 = 0x408;
pub const PERIPH_RSTEN8_HARQ: u32 = 0x40a;
pub const PERIPH_RSTEN8_DDRC: u32 = 0x40c;
pub const PERIPH_RSTEN8_DDRC_APB: u32 = 0x40d;
pub const PERIPH_RSTEN8_DDRPACK_APB: u32 = 0x40e;
pub const PERIPH_RSTEN8_DDRT: u32 = 0x411;
pub const PERIPH_RSDIST9_CARM_DAP: u32 = 0x500;
pub const PERIPH_RSDIST9_CARM_ATB: u32 = 0x501;
pub const PERIPH_RSDIST9_CARM_LBUS: u32 = 0x502;
pub const PERIPH_RSDIST9_CARM_POR: u32 = 0x503;
pub const PERIPH_RSDIST9_CARM_CORE: u32 = 0x504;
pub const PERIPH_RSDIST9_CARM_DBG: u32 = 0x505;
pub const PERIPH_RSDIST9_CARM_L2: u32 = 0x506;
pub const PERIPH_RSDIST9_CARM_SOCDBG: u32 = 0x507;
pub const PERIPH_RSDIST9_CARM_ETM: u32 = 0x508;

pub const MEDIA_G3D: u32 = 0;
pub const MEDIA_CODEC_VPU: u32 = 2;
pub const MEDIA_CODEC_JPEG: u32 = 3;
pub const MEDIA_ISP: u32 = 4;
pub const MEDIA_ADE: u32 = 5;
pub const MEDIA_MMU: u32 = 6;
pub const MEDIA_XG2RAM1: u32 = 7;

pub const AO_G3D: u32 = 1;
pub const AO_CODECISP: u32 = 2;
pub const AO_MCPU: u32 = 4;
pub const AO_BBPHARQMEM: u32 = 5;
pub const AO_HIFI: u32 = 8;
pub const AO_ACPUSCUL2C: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
