/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/drivers/video/sstfb.h. */

/* Debug macros are build-time C variadic macros; their conditional intent is preserved here. */
#[cfg(feature = "SST_DEBUG")]
pub const SST_DEBUG_REG: i32 = 1;
#[cfg(not(feature = "SST_DEBUG"))]
pub const SST_DEBUG_REG: i32 = 0;
#[cfg(feature = "SST_DEBUG")]
pub const SST_DEBUG_FUNC: i32 = 1;
#[cfg(not(feature = "SST_DEBUG"))]
pub const SST_DEBUG_FUNC: i32 = 0;
#[cfg(feature = "SST_DEBUG")]
pub const SST_DEBUG_VAR: i32 = 1;
#[cfg(not(feature = "SST_DEBUG"))]
pub const SST_DEBUG_VAR: i32 = 0;

#[inline]
pub const fn POW2(x: u32) -> usize { 1usize << x }

pub const PCI_INIT_ENABLE: u32 = 0x40;
pub const PCI_EN_INIT_WR: u32 = BIT(0);
pub const PCI_EN_FIFO_WR: u32 = BIT(1);
pub const PCI_REMAP_DAC: u32 = BIT(2);
pub const PCI_VCLK_ENABLE: u32 = 0xc0;
pub const PCI_VCLK_DISABLE: u32 = 0xe0;

pub const STATUS: u32 = 0x0000;
pub const STATUS_FBI_BUSY: u32 = BIT(7);
pub const FBZMODE: u32 = 0x0110;
pub const EN_CLIPPING: u32 = BIT(0);
pub const EN_RGB_WRITE: u32 = BIT(9);
pub const EN_ALPHA_WRITE: u32 = BIT(10);
pub const ENGINE_INVERT_Y: u32 = BIT(17);
pub const LFBMODE: u32 = 0x0114;
pub const LFB_565: u32 = 0;
pub const LFB_888: u32 = 4;
pub const LFB_8888: u32 = 5;
pub const WR_BUFF_FRONT: u32 = 0;
pub const WR_BUFF_BACK: u32 = 1 << 4;
pub const RD_BUFF_FRONT: u32 = 0;
pub const RD_BUFF_BACK: u32 = 1 << 6;
pub const EN_PXL_PIPELINE: u32 = BIT(8);
pub const LFB_WORD_SWIZZLE_WR: u32 = BIT(11);
pub const LFB_BYTE_SWIZZLE_WR: u32 = BIT(12);
pub const LFB_INVERT_Y: u32 = BIT(13);
pub const LFB_WORD_SWIZZLE_RD: u32 = BIT(15);
pub const LFB_BYTE_SWIZZLE_RD: u32 = BIT(16);
pub const CLIP_LEFT_RIGHT: u32 = 0x0118;
pub const CLIP_LOWY_HIGHY: u32 = 0x011c;
pub const NOPCMD: u32 = 0x0120;
pub const FASTFILLCMD: u32 = 0x0124;
pub const SWAPBUFFCMD: u32 = 0x0128;
pub const FBIINIT4: u32 = 0x0200;
pub const FAST_PCI_READS: u32 = 0;
pub const SLOW_PCI_READS: u32 = BIT(0);
pub const LFB_READ_AHEAD: u32 = BIT(1);
pub const BACKPORCH: u32 = 0x0208;
pub const VIDEODIMENSIONS: u32 = 0x020c;
pub const FBIINIT0: u32 = 0x0210;
pub const DIS_VGA_PASSTHROUGH: u32 = BIT(0);
pub const FBI_RESET: u32 = BIT(1);
pub const FIFO_RESET: u32 = BIT(2);
pub const FBIINIT1: u32 = 0x0214;
pub const VIDEO_MASK: u32 = 0x8080010f;
pub const FAST_PCI_WRITES: u32 = 0;
pub const SLOW_PCI_WRITES: u32 = BIT(1);
pub const EN_LFB_READ: u32 = BIT(3);
pub const TILES_IN_X_SHIFT: u32 = 4;
pub const VIDEO_RESET: u32 = BIT(8);
pub const EN_BLANKING: u32 = BIT(12);
pub const EN_DATA_OE: u32 = BIT(13);
pub const EN_BLANK_OE: u32 = BIT(14);
pub const EN_HVSYNC_OE: u32 = BIT(15);
pub const EN_DCLK_OE: u32 = BIT(16);
pub const SEL_INPUT_VCLK_2X: u32 = 0;
pub const SEL_INPUT_VCLK_SLAVE: u32 = BIT(17);
pub const SEL_SOURCE_VCLK_SLAVE: u32 = 0;
pub const SEL_SOURCE_VCLK_2X_DIV2: u32 = 0x01 << 20;
pub const SEL_SOURCE_VCLK_2X_SEL: u32 = 0x02 << 20;
pub const EN_24BPP: u32 = BIT(22);
pub const TILES_IN_X_MSB_SHIFT: u32 = 24;
pub const VCLK_2X_SEL_DEL_SHIFT: u32 = 27;
pub const VCLK_DEL_SHIFT: u32 = 29;
pub const FBIINIT2: u32 = 0x0218;
pub const EN_FAST_RAS_READ: u32 = BIT(5);
pub const EN_DRAM_OE: u32 = BIT(6);
pub const EN_FAST_RD_AHEAD_WR: u32 = BIT(7);
pub const VIDEO_OFFSET_SHIFT: u32 = 11;
pub const SWAP_DACVSYNC: u32 = 0;
pub const SWAP_DACDATA0: u32 = 1 << 9;
pub const SWAP_FIFO_STALL: u32 = 2 << 9;
pub const EN_RD_AHEAD_FIFO: u32 = BIT(21);
pub const EN_DRAM_REFRESH: u32 = BIT(22);
pub const DRAM_REFRESH_16: u32 = 0x30 << 23;
pub const DAC_READ: u32 = FBIINIT2;
pub const FBIINIT3: u32 = 0x021c;
pub const DISABLE_TEXTURE: u32 = BIT(6);
pub const Y_SWAP_ORIGIN_SHIFT: u32 = 22;
pub const HSYNC: u32 = 0x0220;
pub const VSYNC: u32 = 0x0224;
pub const DAC_DATA: u32 = 0x022c;
pub const DAC_READ_CMD: u32 = BIT(11);
pub const FBIINIT5: u32 = 0x0244;
pub const FBIINIT5_MASK: u32 = 0xfa40ffff;
pub const HDOUBLESCAN: u32 = BIT(20);
pub const VDOUBLESCAN: u32 = BIT(21);
pub const HSYNC_HIGH: u32 = BIT(23);
pub const VSYNC_HIGH: u32 = BIT(24);
pub const INTERLACE: u32 = BIT(26);
pub const FBIINIT6: u32 = 0x0248;
pub const TILES_IN_X_LSB_SHIFT: u32 = 30;
pub const FBIINIT7: u32 = 0x024c;

pub const BLTSRCBASEADDR: u32 = 0x02c0;
pub const BLTDSTBASEADDR: u32 = 0x02c4;
pub const BLTXYSTRIDES: u32 = 0x02c8;
pub const BLTSRCCHROMARANGE: u32 = 0x02cc;
pub const BLTDSTCHROMARANGE: u32 = 0x02d0;
pub const BLTCLIPX: u32 = 0x02d4;
pub const BLTCLIPY: u32 = 0x02d8;
pub const BLTSRCXY: u32 = 0x02e0;
pub const BLTDSTXY: u32 = 0x02e4;
pub const BLTSIZE: u32 = 0x02e8;
pub const BLTROP: u32 = 0x02ec;
pub const BLTROP_COPY: u32 = 0x0cccc;
pub const BLTROP_INVERT: u32 = 0x05555;
pub const BLTROP_XOR: u32 = 0x06666;
pub const BLTCOLOR: u32 = 0x02f0;
pub const BLTCOMMAND: u32 = 0x02f8;
pub const BLT_SCR2SCR_BITBLT: u32 = 0;
pub const BLT_CPU2SCR_BITBLT: u32 = 1;
pub const BLT_RECFILL_BITBLT: u32 = 2;
pub const BLT_16BPP_FMT: u32 = 2;
pub const BLTDATA: u32 = 0x02fc;
pub const LAUNCH_BITBLT: u32 = BIT(31);

pub const DACREG_WMA: u32 = 0x0;
pub const DACREG_LUT: u32 = 0x01;
pub const DACREG_RMR: u32 = 0x02;
pub const DACREG_RMA: u32 = 0x03;
pub const DACREG_ADDR_I: u32 = DACREG_WMA;
pub const DACREG_DATA_I: u32 = DACREG_RMR;
pub const DACREG_RMR_I: u32 = 0x00;
pub const DACREG_CR0_I: u32 = 0x01;
pub const DACREG_CR0_EN_INDEXED: u32 = BIT(0);
pub const DACREG_CR0_8BIT: u32 = BIT(1);
pub const DACREG_CR0_PWDOWN: u32 = BIT(3);
pub const DACREG_CR0_16BPP: u32 = 0x30;
pub const DACREG_CR0_24BPP: u32 = 0x50;
pub const DACREG_CR1_I: u32 = 0x05;
pub const DACREG_CC_I: u32 = 0x06;
pub const DACREG_CC_CLKA: u32 = BIT(7);
pub const DACREG_CC_CLKA_C: u32 = 2 << 4;
pub const DACREG_CC_CLKB: u32 = BIT(3);
pub const DACREG_CC_CLKB_D: u32 = 3;
pub const DACREG_AC0_I: u32 = 0x48;
pub const DACREG_AC1_I: u32 = 0x49;
pub const DACREG_BD0_I: u32 = 0x6c;
pub const DACREG_BD1_I: u32 = 0x6d;
pub const DACREG_MIR_TI: u32 = 0x97;
pub const DACREG_DIR_TI: u32 = 0x09;
pub const DACREG_MIR_ATT: u32 = 0x84;
pub const DACREG_DIR_ATT: u32 = 0x09;
pub const DACREG_ICS_PLLWMA: u32 = 0x04;
pub const DACREG_ICS_PLLDATA: u32 = 0x05;
pub const DACREG_ICS_CMD: u32 = 0x06;
pub const DACREG_ICS_CMD_16BPP: u32 = 0x50;
pub const DACREG_ICS_CMD_24BPP: u32 = 0x70;
pub const DACREG_ICS_CMD_PWDOWN: u32 = BIT(0);
pub const DACREG_ICS_PLLRMA: u32 = 0x07;
pub const DACREG_ICS_PLL_CLK0_1_INI: u32 = 0x55;
pub const DACREG_ICS_PLL_CLK0_7_INI: u32 = 0x71;
pub const DACREG_ICS_PLL_CLK1_B_INI: u32 = 0x79;
pub const DACREG_ICS_PLL_CTRL: u32 = 0x0e;
pub const DACREG_ICS_CLK0: u32 = BIT(5);
pub const DACREG_ICS_CLK0_0: u32 = 0;
pub const DACREG_ICS_CLK1_A: u32 = 0;

pub const FBIINIT0_DEFAULT: u32 = DIS_VGA_PASSTHROUGH;
pub const FBIINIT1_DEFAULT: u32 = FAST_PCI_WRITES | VIDEO_RESET | (10 << TILES_IN_X_SHIFT) | SEL_SOURCE_VCLK_2X_SEL | EN_LFB_READ;
pub const FBIINIT2_DEFAULT: u32 = SWAP_DACVSYNC | EN_DRAM_OE | DRAM_REFRESH_16 | EN_DRAM_REFRESH | EN_FAST_RAS_READ | EN_RD_AHEAD_FIFO | EN_FAST_RD_AHEAD_WR;
pub const FBIINIT3_DEFAULT: u32 = DISABLE_TEXTURE;
pub const FBIINIT4_DEFAULT: u32 = FAST_PCI_READS | LFB_READ_AHEAD;
pub const FBIINIT6_DEFAULT: u32 = 0x0;

pub const SSTFB_SET_VGAPASS: u32 = _IOW('F' as u32, 0xdd, core::mem::size_of::<u32>() as u32);
pub const SSTFB_GET_VGAPASS: u32 = _IOR('F' as u32, 0xdd, core::mem::size_of::<u32>() as u32);

pub const VID_CLOCK: i32 = 0;
pub const GFX_CLOCK: i32 = 1;
pub const DAC_FREF: u32 = 14318;
pub const VCO_MAX: u32 = 260000;

#[repr(C)]
pub struct pll_timing { pub m: u32, pub n: u32, pub p: u32 }

#[repr(C)]
pub struct dac_switch {
    pub name: *const core::ffi::c_char,
    pub detect: Option<unsafe extern "C" fn(*mut fb_info) -> i32>,
    pub set_pll: Option<unsafe extern "C" fn(*mut fb_info, *const pll_timing, i32) -> i32>,
    pub set_vidmod: Option<unsafe extern "C" fn(*mut fb_info, i32)>,
}

#[repr(C)]
pub struct sst_spec { pub name: *mut core::ffi::c_char, pub default_gfx_clock: i32, pub max_gfxclk: i32 }

#[repr(C)]
pub struct sstfb_par {
    pub palette: [u32; 16], pub yDim: u32, pub hSyncOn: u32, pub hSyncOff: u32,
    pub hBackPorch: u32, pub vSyncOn: u32, pub vSyncOff: u32, pub vBackPorch: u32,
    pub pll: pll_timing, pub tiles_in_X: u32, pub mmio_vbase: *mut u8,
    pub dac_sw: dac_switch, pub dev: *mut pci_dev, pub type_: i32, pub revision: u8, pub vgapass: u8,
}

/* External kernel types and ioctl/bit macros are supplied by the surrounding translation unit. */
extern "C" {
    fn BIT(n: u32) -> u32;
    fn _IOW(ty: u32, nr: u32, size: u32) -> u32;
    fn _IOR(ty: u32, nr: u32, size: u32) -> u32;
}
#[allow(non_camel_case_types)] pub enum fb_info {}
#[allow(non_camel_case_types)] pub enum pci_dev {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
