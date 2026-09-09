/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from uapi/linux/fb.h. */

pub const FB_MAX: u32 = 32;

pub const FBIOGET_VSCREENINFO: u32 = 0x4600;
pub const FBIOPUT_VSCREENINFO: u32 = 0x4601;
pub const FBIOGET_FSCREENINFO: u32 = 0x4602;
pub const FBIOGETCMAP: u32 = 0x4604;
pub const FBIOPUTCMAP: u32 = 0x4605;
pub const FBIOPAN_DISPLAY: u32 = 0x4606;
/* FBIO_CURSOR uses the platform _IOWR('F', 0x08, struct fb_cursor) encoding. */
pub const FBIOGET_CON2FBMAP: u32 = 0x460f;
pub const FBIOPUT_CON2FBMAP: u32 = 0x4610;
pub const FBIOBLANK: u32 = 0x4611;
/* FBIOGET_VBLANK uses the platform _IOR('F', 0x12, struct fb_vblank) encoding. */
pub const FBIO_ALLOC: u32 = 0x4613;
pub const FBIO_FREE: u32 = 0x4614;
pub const FBIOGET_GLYPH: u32 = 0x4615;
pub const FBIOGET_HWCINFO: u32 = 0x4616;
pub const FBIOPUT_MODEINFO: u32 = 0x4617;
pub const FBIOGET_DISPINFO: u32 = 0x4618;
/* FBIO_WAITFORVSYNC uses the platform _IOW('F', 0x20, __u32) encoding. */

pub const FB_TYPE_PACKED_PIXELS: u32 = 0;
pub const FB_TYPE_PLANES: u32 = 1;
pub const FB_TYPE_INTERLEAVED_PLANES: u32 = 2;
pub const FB_TYPE_TEXT: u32 = 3;
pub const FB_TYPE_VGA_PLANES: u32 = 4;
pub const FB_TYPE_FOURCC: u32 = 5;

pub const FB_AUX_TEXT_MDA: u32 = 0;
pub const FB_AUX_TEXT_CGA: u32 = 1;
pub const FB_AUX_TEXT_S3_MMIO: u32 = 2;
pub const FB_AUX_TEXT_MGA_STEP16: u32 = 3;
pub const FB_AUX_TEXT_MGA_STEP8: u32 = 4;
pub const FB_AUX_TEXT_SVGA_GROUP: u32 = 8;
pub const FB_AUX_TEXT_SVGA_MASK: u32 = 7;
pub const FB_AUX_TEXT_SVGA_STEP2: u32 = 8;
pub const FB_AUX_TEXT_SVGA_STEP4: u32 = 9;
pub const FB_AUX_TEXT_SVGA_STEP8: u32 = 10;
pub const FB_AUX_TEXT_SVGA_STEP16: u32 = 11;
pub const FB_AUX_TEXT_SVGA_LAST: u32 = 15;
pub const FB_AUX_VGA_PLANES_VGA4: u32 = 0;
pub const FB_AUX_VGA_PLANES_CFB4: u32 = 1;
pub const FB_AUX_VGA_PLANES_CFB8: u32 = 2;

pub const FB_VISUAL_MONO01: u32 = 0;
pub const FB_VISUAL_MONO10: u32 = 1;
pub const FB_VISUAL_TRUECOLOR: u32 = 2;
pub const FB_VISUAL_PSEUDOCOLOR: u32 = 3;
pub const FB_VISUAL_DIRECTCOLOR: u32 = 4;
pub const FB_VISUAL_STATIC_PSEUDOCOLOR: u32 = 5;
pub const FB_VISUAL_FOURCC: u32 = 6;

pub const FB_ACCEL_NONE: u32 = 0;
pub const FB_ACCEL_ATARIBLITT: u32 = 1;
pub const FB_ACCEL_AMIGABLITT: u32 = 2;
pub const FB_ACCEL_S3_TRIO64: u32 = 3;
pub const FB_ACCEL_NCR_77C32BLT: u32 = 4;
pub const FB_ACCEL_S3_VIRGE: u32 = 5;
pub const FB_ACCEL_ATI_MACH64GX: u32 = 6;
pub const FB_ACCEL_DEC_TGA: u32 = 7;
pub const FB_ACCEL_ATI_MACH64CT: u32 = 8;
pub const FB_ACCEL_ATI_MACH64VT: u32 = 9;
pub const FB_ACCEL_ATI_MACH64GT: u32 = 10;
pub const FB_ACCEL_SUN_CREATOR: u32 = 11;
pub const FB_ACCEL_SUN_CGSIX: u32 = 12;
pub const FB_ACCEL_SUN_LEO: u32 = 13;
pub const FB_ACCEL_IMS_TWINTURBO: u32 = 14;
pub const FB_ACCEL_3DLABS_PERMEDIA2: u32 = 15;
pub const FB_ACCEL_MATROX_MGA2064W: u32 = 16;
pub const FB_ACCEL_MATROX_MGA1064SG: u32 = 17;
pub const FB_ACCEL_MATROX_MGA2164W: u32 = 18;
pub const FB_ACCEL_MATROX_MGA2164W_AGP: u32 = 19;
pub const FB_ACCEL_MATROX_MGAG100: u32 = 20;
pub const FB_ACCEL_MATROX_MGAG200: u32 = 21;
pub const FB_ACCEL_SUN_CG14: u32 = 22;
pub const FB_ACCEL_SUN_BWTWO: u32 = 23;
pub const FB_ACCEL_SUN_CGTHREE: u32 = 24;
pub const FB_ACCEL_SUN_TCX: u32 = 25;
pub const FB_ACCEL_MATROX_MGAG400: u32 = 26;
pub const FB_ACCEL_NV3: u32 = 27;
pub const FB_ACCEL_NV4: u32 = 28;
pub const FB_ACCEL_NV5: u32 = 29;
pub const FB_ACCEL_CT_6555X: u32 = 30;
pub const FB_ACCEL_3DFX_BANSHEE: u32 = 31;
pub const FB_ACCEL_ATI_RAGE128: u32 = 32;
pub const FB_ACCEL_IGS_CYBER2000: u32 = 33;
pub const FB_ACCEL_IGS_CYBER2010: u32 = 34;
pub const FB_ACCEL_IGS_CYBER5000: u32 = 35;
pub const FB_ACCEL_SIS_GLAMOUR: u32 = 36;
pub const FB_ACCEL_3DLABS_PERMEDIA3: u32 = 37;
pub const FB_ACCEL_ATI_RADEON: u32 = 38;
pub const FB_ACCEL_I810: u32 = 39;
pub const FB_ACCEL_SIS_GLAMOUR_2: u32 = 40;
pub const FB_ACCEL_SIS_XABRE: u32 = 41;
pub const FB_ACCEL_I830: u32 = 42;
pub const FB_ACCEL_NV_10: u32 = 43;
pub const FB_ACCEL_NV_20: u32 = 44;
pub const FB_ACCEL_NV_30: u32 = 45;
pub const FB_ACCEL_NV_40: u32 = 46;
pub const FB_ACCEL_XGI_VOLARI_V: u32 = 47;
pub const FB_ACCEL_XGI_VOLARI_Z: u32 = 48;
pub const FB_ACCEL_OMAP1610: u32 = 49;
pub const FB_ACCEL_TRIDENT_TGUI: u32 = 50;
pub const FB_ACCEL_TRIDENT_3DIMAGE: u32 = 51;
pub const FB_ACCEL_TRIDENT_BLADE3D: u32 = 52;
pub const FB_ACCEL_TRIDENT_BLADEXP: u32 = 53;
pub const FB_ACCEL_CIRRUS_ALPINE: u32 = 53;
pub const FB_ACCEL_NEOMAGIC_NM2070: u32 = 90;
pub const FB_ACCEL_NEOMAGIC_NM2090: u32 = 91;
pub const FB_ACCEL_NEOMAGIC_NM2093: u32 = 92;
pub const FB_ACCEL_NEOMAGIC_NM2097: u32 = 93;
pub const FB_ACCEL_NEOMAGIC_NM2160: u32 = 94;
pub const FB_ACCEL_NEOMAGIC_NM2200: u32 = 95;
pub const FB_ACCEL_NEOMAGIC_NM2230: u32 = 96;
pub const FB_ACCEL_NEOMAGIC_NM2360: u32 = 97;
pub const FB_ACCEL_NEOMAGIC_NM2380: u32 = 98;
pub const FB_ACCEL_PXA3XX: u32 = 99;
pub const FB_ACCEL_SAVAGE4: u32 = 0x80;
pub const FB_ACCEL_SAVAGE3D: u32 = 0x81;
pub const FB_ACCEL_SAVAGE3D_MV: u32 = 0x82;
pub const FB_ACCEL_SAVAGE2000: u32 = 0x83;
pub const FB_ACCEL_SAVAGE_MX_MV: u32 = 0x84;
pub const FB_ACCEL_SAVAGE_MX: u32 = 0x85;
pub const FB_ACCEL_SAVAGE_IX_MV: u32 = 0x86;
pub const FB_ACCEL_SAVAGE_IX: u32 = 0x87;
pub const FB_ACCEL_PROSAVAGE_PM: u32 = 0x88;
pub const FB_ACCEL_PROSAVAGE_KM: u32 = 0x89;
pub const FB_ACCEL_S3TWISTER_P: u32 = 0x8a;
pub const FB_ACCEL_S3TWISTER_K: u32 = 0x8b;
pub const FB_ACCEL_SUPERSAVAGE: u32 = 0x8c;
pub const FB_ACCEL_PROSAVAGE_DDR: u32 = 0x8d;
pub const FB_ACCEL_PROSAVAGE_DDRK: u32 = 0x8e;
pub const FB_ACCEL_PUV3_UNIGFX: u32 = 0xa0;
pub const FB_CAP_FOURCC: u32 = 1;

#[repr(C)]
pub struct fb_fix_screeninfo {
    pub id: [i8; 16], pub smem_start: ::core::ffi::c_ulong, pub smem_len: __u32,
    pub type_: __u32, pub type_aux: __u32, pub visual: __u32,
    pub xpanstep: __u16, pub ypanstep: __u16, pub ywrapstep: __u16,
    pub line_length: __u32, pub mmio_start: ::core::ffi::c_ulong, pub mmio_len: __u32,
    pub accel: __u32, pub capabilities: __u16, pub reserved: [__u16; 2],
}

#[repr(C)]
pub struct fb_bitfield { pub offset: __u32, pub length: __u32, pub msb_right: __u32 }

pub const FB_NONSTD_HAM: u32 = 1;
pub const FB_NONSTD_REV_PIX_IN_B: u32 = 2;
pub const FB_ACTIVATE_NOW: u32 = 0;
pub const FB_ACTIVATE_NXTOPEN: u32 = 1;
pub const FB_ACTIVATE_TEST: u32 = 2;
pub const FB_ACTIVATE_MASK: u32 = 15;
pub const FB_ACTIVATE_VBL: u32 = 16;
pub const FB_CHANGE_CMAP_VBL: u32 = 32;
pub const FB_ACTIVATE_ALL: u32 = 64;
pub const FB_ACTIVATE_FORCE: u32 = 128;
pub const FB_ACTIVATE_INV_MODE: u32 = 256;
pub const FB_ACTIVATE_KD_TEXT: u32 = 512;
pub const FB_ACCELF_TEXT: u32 = 1;
pub const FB_SYNC_HOR_HIGH_ACT: u32 = 1;
pub const FB_SYNC_VERT_HIGH_ACT: u32 = 2;
pub const FB_SYNC_EXT: u32 = 4;
pub const FB_SYNC_COMP_HIGH_ACT: u32 = 8;
pub const FB_SYNC_BROADCAST: u32 = 16;
pub const FB_SYNC_ON_GREEN: u32 = 32;
pub const FB_VMODE_NONINTERLACED: u32 = 0;
pub const FB_VMODE_INTERLACED: u32 = 1;
pub const FB_VMODE_DOUBLE: u32 = 2;
pub const FB_VMODE_ODD_FLD_FIRST: u32 = 4;
pub const FB_VMODE_MASK: u32 = 255;
pub const FB_VMODE_YWRAP: u32 = 256;
pub const FB_VMODE_SMOOTH_XPAN: u32 = 512;
pub const FB_VMODE_CONUPDATE: u32 = 512;
pub const FB_ROTATE_UR: u32 = 0;
pub const FB_ROTATE_CW: u32 = 1;
pub const FB_ROTATE_UD: u32 = 2;
pub const FB_ROTATE_CCW: u32 = 3;

#[inline] pub const fn PICOS2KHZ(a: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { 1_000_000_000u64 as ::core::ffi::c_ulong / a }
#[inline] pub const fn KHZ2PICOS(a: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { 1_000_000_000u64 as ::core::ffi::c_ulong / a }

#[repr(C)]
pub struct fb_var_screeninfo {
    pub xres: __u32, pub yres: __u32, pub xres_virtual: __u32, pub yres_virtual: __u32,
    pub xoffset: __u32, pub yoffset: __u32, pub bits_per_pixel: __u32, pub grayscale: __u32,
    pub red: fb_bitfield, pub green: fb_bitfield, pub blue: fb_bitfield, pub transp: fb_bitfield,
    pub nonstd: __u32, pub activate: __u32, pub height: __u32, pub width: __u32,
    pub accel_flags: __u32, pub pixclock: __u32, pub left_margin: __u32, pub right_margin: __u32,
    pub upper_margin: __u32, pub lower_margin: __u32, pub hsync_len: __u32, pub vsync_len: __u32,
    pub sync: __u32, pub vmode: __u32, pub rotate: __u32, pub colorspace: __u32,
    pub reserved: [__u32; 4],
}

#[repr(C)]
pub struct fb_cmap { pub start: __u32, pub len: __u32, pub red: *mut __u16, pub green: *mut __u16, pub blue: *mut __u16, pub transp: *mut __u16 }
#[repr(C)] pub struct fb_con2fbmap { pub console: __u32, pub framebuffer: __u32 }

pub const FB_BLANK_UNBLANK: u32 = VESA_NO_BLANKING;
pub const FB_BLANK_NORMAL: u32 = VESA_NO_BLANKING + 1;
pub const FB_BLANK_VSYNC_SUSPEND: u32 = VESA_VSYNC_SUSPEND + 1;
pub const FB_BLANK_HSYNC_SUSPEND: u32 = VESA_HSYNC_SUSPEND + 1;
pub const FB_BLANK_POWERDOWN: u32 = VESA_POWERDOWN + 1;
pub const FB_VBLANK_VBLANKING: u32 = 0x001;
pub const FB_VBLANK_HBLANKING: u32 = 0x002;
pub const FB_VBLANK_HAVE_VBLANK: u32 = 0x004;
pub const FB_VBLANK_HAVE_HBLANK: u32 = 0x008;
pub const FB_VBLANK_HAVE_COUNT: u32 = 0x010;
pub const FB_VBLANK_HAVE_VCOUNT: u32 = 0x020;
pub const FB_VBLANK_HAVE_HCOUNT: u32 = 0x040;
pub const FB_VBLANK_VSYNCING: u32 = 0x080;
pub const FB_VBLANK_HAVE_VSYNC: u32 = 0x100;

#[repr(C)] pub struct fb_vblank { pub flags: __u32, pub count: __u32, pub vcount: __u32, pub hcount: __u32, pub reserved: [__u32; 4] }
pub const ROP_COPY: u32 = 0;
pub const ROP_XOR: u32 = 1;
#[repr(C)] pub struct fb_copyarea { pub dx: __u32, pub dy: __u32, pub width: __u32, pub height: __u32, pub sx: __u32, pub sy: __u32 }
#[repr(C)] pub struct fb_fillrect { pub dx: __u32, pub dy: __u32, pub width: __u32, pub height: __u32, pub color: __u32, pub rop: __u32 }
#[repr(C)] pub struct fb_image { pub dx: __u32, pub dy: __u32, pub width: __u32, pub height: __u32, pub fg_color: __u32, pub bg_color: __u32, pub depth: __u8, pub data: *const i8, pub cmap: fb_cmap }
pub const FB_CUR_SETIMAGE: u16 = 0x01;
pub const FB_CUR_SETPOS: u16 = 0x02;
pub const FB_CUR_SETHOT: u16 = 0x04;
pub const FB_CUR_SETCMAP: u16 = 0x08;
pub const FB_CUR_SETSHAPE: u16 = 0x10;
pub const FB_CUR_SETSIZE: u16 = 0x20;
pub const FB_CUR_SETALL: u16 = 0xFF;
#[repr(C)] pub struct fbcurpos { pub x: __u16, pub y: __u16 }
#[repr(C)] pub struct fb_cursor { pub set: __u16, pub enable: __u16, pub rop: __u16, pub mask: *const i8, pub hot: fbcurpos, pub image: fb_image }
pub const FB_BACKLIGHT_LEVELS: u32 = 128;
pub const FB_BACKLIGHT_MAX: u32 = 0xFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
