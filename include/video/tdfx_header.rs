/* SPDX-License-Identifier: GPL-2.0 */

/* Register offsets supplied by the TDFX hardware interface. */
pub const STATUS: u32 = 0x00;
pub const PCIINIT0: u32 = 0x04;
pub const SIPMONITOR: u32 = 0x08;
pub const LFBMEMORYCONFIG: u32 = 0x0c;
pub const MISCINIT0: u32 = 0x10;
pub const MISCINIT1: u32 = 0x14;
pub const DRAMINIT0: u32 = 0x18;
pub const DRAMINIT1: u32 = 0x1c;
pub const AGPINIT: u32 = 0x20;
pub const TMUGBEINIT: u32 = 0x24;
pub const VGAINIT0: u32 = 0x28;
pub const VGAINIT1: u32 = 0x2c;
pub const DRAMCOMMAND: u32 = 0x30;
pub const DRAMDATA: u32 = 0x34;
pub const PLLCTRL0: u32 = 0x40;
pub const PLLCTRL1: u32 = 0x44;
pub const PLLCTRL2: u32 = 0x48;
pub const DACMODE: u32 = 0x4c;
pub const DACADDR: u32 = 0x50;
pub const DACDATA: u32 = 0x54;
pub const RGBMAXDELTA: u32 = 0x58;
pub const VIDPROCCFG: u32 = 0x5c;
pub const HWCURPATADDR: u32 = 0x60;
pub const HWCURLOC: u32 = 0x64;
pub const HWCURC0: u32 = 0x68;
pub const HWCURC1: u32 = 0x6c;
pub const VIDINFORMAT: u32 = 0x70;
pub const VIDINSTATUS: u32 = 0x74;
pub const VIDSERPARPORT: u32 = 0x78;
pub const VIDINXDELTA: u32 = 0x7c;
pub const VIDININITERR: u32 = 0x80;
pub const VIDINYDELTA: u32 = 0x84;
pub const VIDPIXBUFTHOLD: u32 = 0x88;
pub const VIDCHRMIN: u32 = 0x8c;
pub const VIDCHRMAX: u32 = 0x90;
pub const VIDCURLIN: u32 = 0x94;
pub const VIDSCREENSIZE: u32 = 0x98;
pub const VIDOVRSTARTCRD: u32 = 0x9c;
pub const VIDOVRENDCRD: u32 = 0xa0;
pub const VIDOVRDUDX: u32 = 0xa4;
pub const VIDOVRDUDXOFF: u32 = 0xa8;
pub const VIDOVRDVDY: u32 = 0xac;
pub const VIDOVRDVDYOFF: u32 = 0xe0;
pub const VIDDESKSTART: u32 = 0xe4;
pub const VIDDESKSTRIDE: u32 = 0xe8;
pub const VIDINADDR0: u32 = 0xec;
pub const VIDINADDR1: u32 = 0xf0;
pub const VIDINADDR2: u32 = 0xf4;
pub const VIDINSTRIDE: u32 = 0xf8;
pub const VIDCUROVRSTART: u32 = 0xfc;

pub const INTCTRL: u32 = 0x00100000 + 0x04;
pub const CLIP0MIN: u32 = 0x00100000 + 0x08;
pub const CLIP0MAX: u32 = 0x00100000 + 0x0c;
pub const DSTBASE: u32 = 0x00100000 + 0x10;
pub const DSTFORMAT: u32 = 0x00100000 + 0x14;
pub const SRCBASE: u32 = 0x00100000 + 0x34;
pub const COMMANDEXTRA_2D: u32 = 0x00100000 + 0x38;
pub const CLIP1MIN: u32 = 0x00100000 + 0x4c;
pub const CLIP1MAX: u32 = 0x00100000 + 0x50;
pub const SRCFORMAT: u32 = 0x00100000 + 0x54;
pub const SRCSIZE: u32 = 0x00100000 + 0x58;
pub const SRCXY: u32 = 0x00100000 + 0x5c;
pub const COLORBACK: u32 = 0x00100000 + 0x60;
pub const COLORFORE: u32 = 0x00100000 + 0x64;
pub const DSTSIZE: u32 = 0x00100000 + 0x68;
pub const DSTXY: u32 = 0x00100000 + 0x6c;
pub const COMMAND_2D: u32 = 0x00100000 + 0x70;
pub const LAUNCH_2D: u32 = 0x00100000 + 0x80;
pub const COMMAND_3D: u32 = 0x00200000 + 0x120;

pub const TDFX_ROP_COPY: u32 = 0xcc;
pub const TDFX_ROP_INVERT: u32 = 0x55;
pub const TDFX_ROP_XOR: u32 = 0x66;
pub const AUTOINC_DSTX: u32 = 1u32 << 10;
pub const AUTOINC_DSTY: u32 = 1u32 << 11;
pub const COMMAND_2D_FILLRECT: u32 = 0x05;
pub const COMMAND_2D_S2S_BITBLT: u32 = 0x01;
pub const COMMAND_2D_H2S_BITBLT: u32 = 0x03;
pub const COMMAND_3D_NOP: u32 = 0x00;
pub const STATUS_RETRACE: u32 = 1u32 << 6;
pub const STATUS_BUSY: u32 = 1u32 << 9;
pub const MISCINIT1_CLUT_INV: u32 = 1u32 << 0;
pub const MISCINIT1_2DBLOCK_DIS: u32 = 1u32 << 15;
pub const DRAMINIT0_SGRAM_NUM: u32 = 1u32 << 26;
pub const DRAMINIT0_SGRAM_TYPE: u32 = 1u32 << 27;
pub const DRAMINIT0_SGRAM_TYPE_MASK: u32 = (1u32 << 27) | (1u32 << 28) | (1u32 << 29);
pub const DRAMINIT0_SGRAM_TYPE_SHIFT: u32 = 27;
pub const DRAMINIT1_MEM_SDRAM: u32 = 1u32 << 30;
pub const VGAINIT0_VGA_DISABLE: u32 = 1u32 << 0;
pub const VGAINIT0_EXT_TIMING: u32 = 1u32 << 1;
pub const VGAINIT0_8BIT_DAC: u32 = 1u32 << 2;
pub const VGAINIT0_EXT_ENABLE: u32 = 1u32 << 6;
pub const VGAINIT0_WAKEUP_3C3: u32 = 1u32 << 8;
pub const VGAINIT0_LEGACY_DISABLE: u32 = 1u32 << 9;
pub const VGAINIT0_ALT_READBACK: u32 = 1u32 << 10;
pub const VGAINIT0_FAST_BLINK: u32 = 1u32 << 11;
pub const VGAINIT0_EXTSHIFTOUT: u32 = 1u32 << 12;
pub const VGAINIT0_DECODE_3C6: u32 = 1u32 << 13;
pub const VGAINIT0_SGRAM_HBLANK_DISABLE: u32 = 1u32 << 22;
pub const VGAINIT1_MASK: u32 = 0x1fffff;
pub const VIDCFG_VIDPROC_ENABLE: u32 = 1u32 << 0;
pub const VIDCFG_CURS_X11: u32 = 1u32 << 1;
pub const VIDCFG_INTERLACE: u32 = 1u32 << 3;
pub const VIDCFG_HALF_MODE: u32 = 1u32 << 4;
pub const VIDCFG_DESK_ENABLE: u32 = 1u32 << 7;
pub const VIDCFG_CLUT_BYPASS: u32 = 1u32 << 10;
pub const VIDCFG_2X: u32 = 1u32 << 26;
pub const VIDCFG_HWCURSOR_ENABLE: u32 = 1u32 << 27;
pub const VIDCFG_PIXFMT_SHIFT: u32 = 18;
pub const DACMODE_2X: u32 = 1u32 << 0;

pub const DDC_ENAB: u32 = 0x00040000;
pub const DDC_SCL_OUT: u32 = 0x00080000;
pub const DDC_SDA_OUT: u32 = 0x00100000;
pub const DDC_SCL_IN: u32 = 0x00200000;
pub const DDC_SDA_IN: u32 = 0x00400000;
pub const I2C_ENAB: u32 = 0x00800000;
pub const I2C_SCL_OUT: u32 = 0x01000000;
pub const I2C_SDA_OUT: u32 = 0x02000000;
pub const I2C_SCL_IN: u32 = 0x04000000;
pub const I2C_SDA_IN: u32 = 0x08000000;

pub const MISC_W: u32 = 0x3c2;
pub const MISC_R: u32 = 0x3cc;
pub const SEQ_I: u32 = 0x3c4;
pub const SEQ_D: u32 = 0x3c5;
pub const CRT_I: u32 = 0x3d4;
pub const CRT_D: u32 = 0x3d5;
pub const ATT_IW: u32 = 0x3c0;
pub const IS1_R: u32 = 0x3da;
pub const GRA_I: u32 = 0x3ce;
pub const GRA_D: u32 = 0x3cf;

#[repr(C)]
pub struct banshee_reg {
    pub att: [u8; 21],
    pub crt: [u8; 25],
    pub gra: [u8; 9],
    pub misc: [u8; 1],
    pub seq: [u8; 5],
    pub ext: [u8; 2],
    pub vidcfg: usize,
    pub vidpll: usize,
    pub mempll: usize,
    pub gfxpll: usize,
    pub dacmode: usize,
    pub vgainit0: usize,
    pub vgainit1: usize,
    pub screensize: usize,
    pub stride: usize,
    pub cursloc: usize,
    pub curspataddr: usize,
    pub cursc0: usize,
    pub cursc1: usize,
    pub startaddr: usize,
    pub clip0min: usize,
    pub clip0max: usize,
    pub clip1min: usize,
    pub clip1max: usize,
    pub miscinit0: usize,
}

#[repr(C)]
pub struct tdfxfb_i2c_chan {
    pub par: *mut tdfx_par,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

#[repr(C)]
#[repr(C)]
pub struct tdfx_par {
    pub max_pixclock: u32,
    pub palette: [u32; 16],
    pub regbase_virt: *mut core::ffi::c_void,
    pub iobase: usize,
    pub wc_cookie: i32,
    #[cfg(CONFIG_FB_3DFX_I2C)]
    pub chan: [tdfxfb_i2c_chan; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
