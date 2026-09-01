/* SPDX-License-Identifier: GPL-2.0 */

pub const WM8785_R0: u32 = 0;
pub const WM8785_R1: u32 = 1;
pub const WM8785_R2: u32 = 2;
pub const WM8785_R7: u32 = 7;

/* R0 */
pub const WM8785_MCR_MASK: u32 = 0x007;
pub const WM8785_MCR_SLAVE: u32 = 0x000;
pub const WM8785_MCR_MASTER_128: u32 = 0x001;
pub const WM8785_MCR_MASTER_192: u32 = 0x002;
pub const WM8785_MCR_MASTER_256: u32 = 0x003;
pub const WM8785_MCR_MASTER_384: u32 = 0x004;
pub const WM8785_MCR_MASTER_512: u32 = 0x005;
pub const WM8785_MCR_MASTER_768: u32 = 0x006;
pub const WM8785_OSR_MASK: u32 = 0x018;
pub const WM8785_OSR_SINGLE: u32 = 0x000;
pub const WM8785_OSR_DOUBLE: u32 = 0x008;
pub const WM8785_OSR_QUAD: u32 = 0x010;
pub const WM8785_FORMAT_MASK: u32 = 0x060;
pub const WM8785_FORMAT_RJUST: u32 = 0x000;
pub const WM8785_FORMAT_LJUST: u32 = 0x020;
pub const WM8785_FORMAT_I2S: u32 = 0x040;
pub const WM8785_FORMAT_DSP: u32 = 0x060;

/* R1 */
pub const WM8785_WL_MASK: u32 = 0x003;
pub const WM8785_WL_16: u32 = 0x000;
pub const WM8785_WL_20: u32 = 0x001;
pub const WM8785_WL_24: u32 = 0x002;
pub const WM8785_WL_32: u32 = 0x003;
pub const WM8785_LRP: u32 = 0x004;
pub const WM8785_BCLKINV: u32 = 0x008;
pub const WM8785_LRSWAP: u32 = 0x010;
pub const WM8785_DEVNO_MASK: u32 = 0x0e0;

/* R2 */
pub const WM8785_HPFR: u32 = 0x001;
pub const WM8785_HPFL: u32 = 0x002;
pub const WM8785_SDODIS: u32 = 0x004;
pub const WM8785_PWRDNR: u32 = 0x008;
pub const WM8785_PWRDNL: u32 = 0x010;
pub const WM8785_TDM_MASK: u32 = 0x1c0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
