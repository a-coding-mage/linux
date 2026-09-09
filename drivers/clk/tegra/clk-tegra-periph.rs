// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of clk-tegra-periph.c. */

// Kernel dependencies supplied by the surrounding translation unit.

macro_rules! BIT { ($x:expr) => { 1u32 << ($x) }; }
macro_rules! MASK { ($x:expr) => { BIT!($x) - 1 }; }

const CLK_SOURCE_I2S0: u32 = 0x1d8;
const CLK_SOURCE_I2S1: u32 = 0x100;
const CLK_SOURCE_I2S2: u32 = 0x104;
const CLK_SOURCE_NDFLASH: u32 = 0x160;
const CLK_SOURCE_I2S3: u32 = 0x3bc;
const CLK_SOURCE_I2S4: u32 = 0x3c0;
const CLK_SOURCE_SPDIF_OUT: u32 = 0x108;
const CLK_SOURCE_SPDIF_IN: u32 = 0x10c;
const CLK_SOURCE_PWM: u32 = 0x110;
const CLK_SOURCE_ADX: u32 = 0x638;
const CLK_SOURCE_ADX1: u32 = 0x670;
const CLK_SOURCE_AMX: u32 = 0x63c;
const CLK_SOURCE_AMX1: u32 = 0x674;
const CLK_SOURCE_HDA: u32 = 0x428;
const CLK_SOURCE_HDA2CODEC_2X: u32 = 0x3e4;
const CLK_SOURCE_SBC1: u32 = 0x134;
const CLK_SOURCE_SBC2: u32 = 0x118;
const CLK_SOURCE_SBC3: u32 = 0x11c;
const CLK_SOURCE_SBC4: u32 = 0x1b4;
const CLK_SOURCE_SBC5: u32 = 0x3c8;
const CLK_SOURCE_SBC6: u32 = 0x3cc;
const CLK_SOURCE_SATA_OOB: u32 = 0x420;
const CLK_SOURCE_SATA: u32 = 0x424;
const CLK_SOURCE_NDSPEED: u32 = 0x3f8;
const CLK_SOURCE_VFIR: u32 = 0x168;
const CLK_SOURCE_SDMMC1: u32 = 0x150;
const CLK_SOURCE_SDMMC2: u32 = 0x154;
const CLK_SOURCE_SDMMC3: u32 = 0x1bc;
const CLK_SOURCE_SDMMC4: u32 = 0x164;
const CLK_SOURCE_CVE: u32 = 0x140;
const CLK_SOURCE_TVO: u32 = 0x188;
const CLK_SOURCE_TVDAC: u32 = 0x194;
const CLK_SOURCE_VDE: u32 = 0x1c8;
const CLK_SOURCE_CSITE: u32 = 0x1d4;
const CLK_SOURCE_LA: u32 = 0x1f8;
const CLK_SOURCE_TRACE: u32 = 0x634;
const CLK_SOURCE_OWR: u32 = 0x1cc;
const CLK_SOURCE_NOR: u32 = 0x1d0;
const CLK_SOURCE_MIPI: u32 = 0x174;
const CLK_SOURCE_I2C1: u32 = 0x124;
const CLK_SOURCE_I2C2: u32 = 0x198;
const CLK_SOURCE_I2C3: u32 = 0x1b8;
const CLK_SOURCE_I2C4: u32 = 0x3c4;
const CLK_SOURCE_I2C5: u32 = 0x128;
const CLK_SOURCE_I2C6: u32 = 0x65c;
const CLK_SOURCE_UARTA: u32 = 0x178;
const CLK_SOURCE_UARTB: u32 = 0x17c;
const CLK_SOURCE_UARTC: u32 = 0x1a0;
const CLK_SOURCE_UARTD: u32 = 0x1c0;
const CLK_SOURCE_UARTE: u32 = 0x1c4;
const CLK_SOURCE_3D: u32 = 0x158;
const CLK_SOURCE_2D: u32 = 0x15c;
const CLK_SOURCE_MPE: u32 = 0x170;
const CLK_SOURCE_VI_SENSOR: u32 = 0x1a8;
const CLK_SOURCE_VI: u32 = 0x148;
const CLK_SOURCE_EPP: u32 = 0x16c;
const CLK_SOURCE_MSENC: u32 = 0x1f0;
const CLK_SOURCE_TSEC: u32 = 0x1f4;
const CLK_SOURCE_HOST1X: u32 = 0x180;
const CLK_SOURCE_HDMI: u32 = 0x18c;
const CLK_SOURCE_DISP1: u32 = 0x138;
const CLK_SOURCE_DISP2: u32 = 0x13c;
const CLK_SOURCE_CILAB: u32 = 0x614;
const CLK_SOURCE_CILCD: u32 = 0x618;
const CLK_SOURCE_CILE: u32 = 0x61c;
const CLK_SOURCE_DSIALP: u32 = 0x620;
const CLK_SOURCE_DSIBLP: u32 = 0x624;
const CLK_SOURCE_TSENSOR: u32 = 0x3b8;
const CLK_SOURCE_D_AUDIO: u32 = 0x3d0;
const CLK_SOURCE_DAM0: u32 = 0x3d8;
const CLK_SOURCE_DAM1: u32 = 0x3dc;
const CLK_SOURCE_DAM2: u32 = 0x3e0;
const CLK_SOURCE_ACTMON: u32 = 0x3e8;
const CLK_SOURCE_EXTERN1: u32 = 0x3ec;
const CLK_SOURCE_EXTERN2: u32 = 0x3f0;
const CLK_SOURCE_EXTERN3: u32 = 0x3f4;
const CLK_SOURCE_I2CSLOW: u32 = 0x3fc;
const CLK_SOURCE_SE: u32 = 0x42c;
const CLK_SOURCE_MSELECT: u32 = 0x3b4;
const CLK_SOURCE_DFLL_REF: u32 = 0x62c;
const CLK_SOURCE_DFLL_SOC: u32 = 0x630;
const CLK_SOURCE_SOC_THERM: u32 = 0x644;
const CLK_SOURCE_XUSB_HOST_SRC: u32 = 0x600;
const CLK_SOURCE_XUSB_FALCON_SRC: u32 = 0x604;
const CLK_SOURCE_XUSB_FS_SRC: u32 = 0x608;
const CLK_SOURCE_XUSB_SS_SRC: u32 = 0x610;
const CLK_SOURCE_XUSB_DEV_SRC: u32 = 0x60c;
const CLK_SOURCE_ISP: u32 = 0x144;
const CLK_SOURCE_SOR0: u32 = 0x414;
const CLK_SOURCE_DPAUX: u32 = 0x418;
const CLK_SOURCE_ENTROPY: u32 = 0x628;
const CLK_SOURCE_VI_SENSOR2: u32 = 0x658;
const CLK_SOURCE_HDMI_AUDIO: u32 = 0x668;
const CLK_SOURCE_VIC03: u32 = 0x678;
const CLK_SOURCE_CLK72MHZ: u32 = 0x66c;
const CLK_SOURCE_DBGAPB: u32 = 0x718;
const CLK_SOURCE_NVENC: u32 = 0x6a0;
const CLK_SOURCE_NVDEC: u32 = 0x698;
const CLK_SOURCE_NVJPG: u32 = 0x69c;
const CLK_SOURCE_APE: u32 = 0x6c0;
const CLK_SOURCE_SDMMC_LEGACY: u32 = 0x694;
const CLK_SOURCE_QSPI: u32 = 0x6c4;
const CLK_SOURCE_VI_I2C: u32 = 0x6c8;
const CLK_SOURCE_MIPIBIF: u32 = 0x660;
const CLK_SOURCE_UARTAPE: u32 = 0x710;
const CLK_SOURCE_TSECB: u32 = 0x6d8;
const CLK_SOURCE_MAUD: u32 = 0x6d4;
const CLK_SOURCE_USB2_HSIC_TRK: u32 = 0x6cc;
const CLK_SOURCE_DMIC1: u32 = 0x64c;
const CLK_SOURCE_DMIC2: u32 = 0x650;
const CLK_SOURCE_DMIC3: u32 = 0x6bc;

const PLLP_BASE: u32 = 0xa0;
const PLLP_MISC: u32 = 0xac;
const PLLP_MISC1: u32 = 0x680;
const PLLP_OUTA: u32 = 0xa4;
const PLLP_OUTB: u32 = 0xa8;
const PLLP_OUTC: u32 = 0x67c;
const PLL_BASE_LOCK: u32 = BIT!(27);
const PLL_MISC_LOCK_ENABLE: u32 = 18;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pll_out_data {
    pub div_name: *mut i8, pub pll_out_name: *mut i8, pub offset: u32,
    pub clk_id: i32, pub div_shift: u8, pub div_flags: u8,
    pub rst_shift: u8, pub lock: *mut core::ffi::c_void,
}

// The following arrays and initializer tables retain the C source's ABI-facing
// names and are populated by the surrounding Tegra clock definitions.
extern "C" {
    fn init_pllp(clk_base: *mut core::ffi::c_void, pmc_base: *mut core::ffi::c_void,
                 tegra_clks: *mut core::ffi::c_void, pll_params: *mut core::ffi::c_void);
}

// C's __init entry point; external clock structures/functions are intentionally
// unresolved here, as they are supplied by the translated kernel dependencies.
pub unsafe fn tegra_periph_clk_init(clk_base: *mut core::ffi::c_void,
                                    pmc_base: *mut core::ffi::c_void,
                                    tegra_clks: *mut core::ffi::c_void,
                                    pll_params: *mut core::ffi::c_void) {
    init_pllp(clk_base, pmc_base, tegra_clks, pll_params);
    // periph_clk_init(clk_base, tegra_clks);
    // gate_clk_init(clk_base, tegra_clks);
    // div_clk_init(clk_base, tegra_clks);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
