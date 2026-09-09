/*
 *  linux/drivers/video/tgafb.h -- DEC 21030 TGA frame buffer device
 *
 *   Copyright (C) 1999,2000 Martin Lucina, Tom Zerucha
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License. See the file COPYING in the main directory of this archive for
 *  more details.
 */

/* TGA hardware description (minimal) */
pub const TGA_TYPE_8PLANE: u32 = 0;
pub const TGA_TYPE_24PLANE: u32 = 1;
pub const TGA_TYPE_24PLUSZ: u32 = 3;

/* Offsets within Memory Space */
pub const TGA_ROM_OFFSET: u32 = 0x0000000;
pub const TGA_REGS_OFFSET: u32 = 0x0100000;
pub const TGA_8PLANE_FB_OFFSET: u32 = 0x0200000;
pub const TGA_24PLANE_FB_OFFSET: u32 = 0x0804000;
pub const TGA_24PLUSZ_FB_OFFSET: u32 = 0x1004000;

pub const TGA_FOREGROUND_REG: u32 = 0x0020;
pub const TGA_BACKGROUND_REG: u32 = 0x0024;
pub const TGA_PLANEMASK_REG: u32 = 0x0028;
pub const TGA_PIXELMASK_ONESHOT_REG: u32 = 0x002c;
pub const TGA_MODE_REG: u32 = 0x0030;
pub const TGA_RASTEROP_REG: u32 = 0x0034;
pub const TGA_PIXELSHIFT_REG: u32 = 0x0038;
pub const TGA_DEEP_REG: u32 = 0x0050;
pub const TGA_START_REG: u32 = 0x0054;
pub const TGA_PIXELMASK_REG: u32 = 0x005c;
pub const TGA_CURSOR_BASE_REG: u32 = 0x0060;
pub const TGA_HORIZ_REG: u32 = 0x0064;
pub const TGA_VERT_REG: u32 = 0x0068;
pub const TGA_BASE_ADDR_REG: u32 = 0x006c;
pub const TGA_VALID_REG: u32 = 0x0070;
pub const TGA_CURSOR_XY_REG: u32 = 0x0074;
pub const TGA_INTR_STAT_REG: u32 = 0x007c;
pub const TGA_DATA_REG: u32 = 0x0080;
pub const TGA_RAMDAC_SETUP_REG: u32 = 0x00c0;
pub const TGA_BLOCK_COLOR0_REG: u32 = 0x0140;
pub const TGA_BLOCK_COLOR1_REG: u32 = 0x0144;
pub const TGA_BLOCK_COLOR2_REG: u32 = 0x0148;
pub const TGA_BLOCK_COLOR3_REG: u32 = 0x014c;
pub const TGA_BLOCK_COLOR4_REG: u32 = 0x0150;
pub const TGA_BLOCK_COLOR5_REG: u32 = 0x0154;
pub const TGA_BLOCK_COLOR6_REG: u32 = 0x0158;
pub const TGA_BLOCK_COLOR7_REG: u32 = 0x015c;
pub const TGA_COPY64_SRC: u32 = 0x0160;
pub const TGA_COPY64_DST: u32 = 0x0164;
pub const TGA_CLOCK_REG: u32 = 0x01e8;
pub const TGA_RAMDAC_REG: u32 = 0x01f0;
pub const TGA_CMD_STAT_REG: u32 = 0x01f8;

pub const TGA_HORIZ_ODD: u32 = 0x80000000;
pub const TGA_HORIZ_POLARITY: u32 = 0x40000000;
pub const TGA_HORIZ_ACT_MSB: u32 = 0x30000000;
pub const TGA_HORIZ_BP: u32 = 0x0fe00000;
pub const TGA_HORIZ_SYNC: u32 = 0x001fc000;
pub const TGA_HORIZ_FP: u32 = 0x00007c00;
pub const TGA_HORIZ_ACT_LSB: u32 = 0x000001ff;
pub const TGA_VERT_SE: u32 = 0x80000000;
pub const TGA_VERT_POLARITY: u32 = 0x40000000;
pub const TGA_VERT_RESERVED: u32 = 0x30000000;
pub const TGA_VERT_BP: u32 = 0x0fc00000;
pub const TGA_VERT_SYNC: u32 = 0x003f0000;
pub const TGA_VERT_FP: u32 = 0x0000f800;
pub const TGA_VERT_ACTIVE: u32 = 0x000007ff;
pub const TGA_VALID_VIDEO: u32 = 0x01;
pub const TGA_VALID_BLANK: u32 = 0x02;
pub const TGA_VALID_CURSOR: u32 = 0x04;
pub const TGA_MODE_SBM_8BPP: u32 = 0x000;
pub const TGA_MODE_SBM_24BPP: u32 = 0x300;
pub const TGA_MODE_SIMPLE: u32 = 0x00;
pub const TGA_MODE_SIMPLEZ: u32 = 0x10;
pub const TGA_MODE_OPAQUE_STIPPLE: u32 = 0x01;
pub const TGA_MODE_OPAQUE_FILL: u32 = 0x21;
pub const TGA_MODE_TRANSPARENT_STIPPLE: u32 = 0x03;
pub const TGA_MODE_TRANSPARENT_FILL: u32 = 0x23;
pub const TGA_MODE_BLOCK_STIPPLE: u32 = 0x0d;
pub const TGA_MODE_BLOCK_FILL: u32 = 0x2d;
pub const TGA_MODE_COPY: u32 = 0x07;
pub const TGA_MODE_DMA_READ_COPY_ND: u32 = 0x17;
pub const TGA_MODE_DMA_READ_COPY_D: u32 = 0x37;
pub const TGA_MODE_DMA_WRITE_COPY: u32 = 0x1f;

pub const TGA_PLL_BASE_FREQ: u32 = 14318; /* .18 */
pub const TGA_PLL_MAX_FREQ: u32 = 230000;

pub const BT485_READ_BIT: u8 = 0x01;
pub const BT485_WRITE_BIT: u8 = 0x00;
pub const BT485_ADDR_PAL_WRITE: u8 = 0x00;
pub const BT485_DATA_PAL: u8 = 0x02;
pub const BT485_PIXEL_MASK: u8 = 0x04;
pub const BT485_ADDR_PAL_READ: u8 = 0x06;
pub const BT485_ADDR_CUR_WRITE: u8 = 0x08;
pub const BT485_DATA_CUR: u8 = 0x0a;
pub const BT485_CMD_0: u8 = 0x0c;
pub const BT485_ADDR_CUR_READ: u8 = 0x0e;
pub const BT485_CMD_1: u8 = 0x10;
pub const BT485_CMD_2: u8 = 0x12;
pub const BT485_STATUS: u8 = 0x14;
pub const BT485_CMD_3: u8 = 0x14;
pub const BT485_CUR_RAM: u8 = 0x16;
pub const BT485_CUR_LOW_X: u8 = 0x18;
pub const BT485_CUR_HIGH_X: u8 = 0x1a;
pub const BT485_CUR_LOW_Y: u8 = 0x1c;
pub const BT485_CUR_HIGH_Y: u8 = 0x1e;

pub const BT463_ADDR_LO: u32 = 0x0;
pub const BT463_ADDR_HI: u32 = 0x1;
pub const BT463_REG_ACC: u32 = 0x2;
pub const BT463_PALETTE: u32 = 0x3;
pub const BT463_CUR_CLR_0: u32 = 0x0100;
pub const BT463_CUR_CLR_1: u32 = 0x0101;
pub const BT463_CMD_REG_0: u32 = 0x0201;
pub const BT463_CMD_REG_1: u32 = 0x0202;
pub const BT463_CMD_REG_2: u32 = 0x0203;
pub const BT463_READ_MASK_0: u32 = 0x0205;
pub const BT463_READ_MASK_1: u32 = 0x0206;
pub const BT463_READ_MASK_2: u32 = 0x0207;
pub const BT463_READ_MASK_3: u32 = 0x0208;
pub const BT463_BLINK_MASK_0: u32 = 0x0209;
pub const BT463_BLINK_MASK_1: u32 = 0x020a;
pub const BT463_BLINK_MASK_2: u32 = 0x020b;
pub const BT463_BLINK_MASK_3: u32 = 0x020c;
pub const BT463_WINDOW_TYPE_BASE: u32 = 0x0300;

pub const BT459_ADDR_LO: u32 = 0x0;
pub const BT459_ADDR_HI: u32 = 0x1;
pub const BT459_REG_ACC: u32 = 0x2;
pub const BT459_PALETTE: u32 = 0x3;
pub const BT459_CUR_CLR_1: u32 = 0x0181;
pub const BT459_CUR_CLR_2: u32 = 0x0182;
pub const BT459_CUR_CLR_3: u32 = 0x0183;
pub const BT459_CMD_REG_0: u32 = 0x0201;
pub const BT459_CMD_REG_1: u32 = 0x0202;
pub const BT459_CMD_REG_2: u32 = 0x0203;
pub const BT459_READ_MASK: u32 = 0x0204;
pub const BT459_BLINK_MASK: u32 = 0x0206;
pub const BT459_CUR_CMD_REG: u32 = 0x0300;

/* The framebuffer driver private data. */
#[repr(C)]
pub struct tga_par {
    pub dev: *mut device,
    pub tga_mem_base: *mut core::ffi::c_void,
    pub tga_fb_base: *mut core::ffi::c_void,
    pub tga_regs_base: *mut core::ffi::c_void,
    pub tga_type: u8,
    pub tga_chip_rev: u8,
    pub vesa_blanked: u8,
    pub xres: u32,
    pub yres: u32,
    pub htimings: u32,
    pub vtimings: u32,
    pub pll_freq: u32,
    pub bits_per_pixel: u32,
    pub sync_on_green: u32,
    pub palette: [u32; 16],
}

pub struct device;

extern "C" {
    pub fn writel(v: u32, addr: *mut core::ffi::c_void);
    pub fn readl(addr: *mut core::ffi::c_void) -> u32;
}

#[inline]
pub unsafe fn TGA_WRITE_REG(par: *mut tga_par, v: u32, r: u32) {
    writel(v, (*par).tga_regs_base.add(r as usize));
}

#[inline]
pub unsafe fn TGA_READ_REG(par: *mut tga_par, r: u32) -> u32 {
    readl((*par).tga_regs_base.add(r as usize))
}

#[inline]
pub unsafe fn BT485_WRITE(par: *mut tga_par, v: u8, r: u8) {
    TGA_WRITE_REG(par, r as u32, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (v as u32) | ((r as u32) << 8), TGA_RAMDAC_REG);
}

#[inline]
pub unsafe fn BT463_LOAD_ADDR(par: *mut tga_par, a: u16) {
    TGA_WRITE_REG(par, BT463_ADDR_LO << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (BT463_ADDR_LO << 10) | ((a as u32) & 0xff), TGA_RAMDAC_REG);
    TGA_WRITE_REG(par, BT463_ADDR_HI << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (BT463_ADDR_HI << 10) | ((a as u32) >> 8), TGA_RAMDAC_REG);
}

#[inline]
pub unsafe fn BT463_WRITE(par: *mut tga_par, m: u32, a: u16, v: u8) {
    BT463_LOAD_ADDR(par, a);
    TGA_WRITE_REG(par, m << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (m << 10) | v as u32, TGA_RAMDAC_REG);
}

#[inline]
pub unsafe fn BT459_LOAD_ADDR(par: *mut tga_par, a: u16) {
    TGA_WRITE_REG(par, BT459_ADDR_LO << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (a as u32) & 0xff, TGA_RAMDAC_REG);
    TGA_WRITE_REG(par, BT459_ADDR_HI << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, (a as u32) >> 8, TGA_RAMDAC_REG);
}

#[inline]
pub unsafe fn BT459_WRITE(par: *mut tga_par, m: u32, a: u16, v: u8) {
    BT459_LOAD_ADDR(par, a);
    TGA_WRITE_REG(par, m << 2, TGA_RAMDAC_SETUP_REG);
    TGA_WRITE_REG(par, v as u32, TGA_RAMDAC_REG);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
