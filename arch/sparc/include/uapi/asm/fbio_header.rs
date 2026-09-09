/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from fbio.h. Kernel __user annotations and ioctl encoding are
 * represented by raw pointers and dependency-provided ioctl macros/types. */

pub const FBTYPE_NOTYPE: i32 = -1;
pub const FBTYPE_SUN1BW: i32 = 0;
pub const FBTYPE_SUN1COLOR: i32 = 1;
pub const FBTYPE_SUN2BW: i32 = 2;
pub const FBTYPE_SUN2COLOR: i32 = 3;
pub const FBTYPE_SUN2GP: i32 = 4;
pub const FBTYPE_SUN5COLOR: i32 = 5;
pub const FBTYPE_SUN3COLOR: i32 = 6;
pub const FBTYPE_MEMCOLOR: i32 = 7;
pub const FBTYPE_SUN4COLOR: i32 = 8;
pub const FBTYPE_NOTSUN1: i32 = 9;
pub const FBTYPE_NOTSUN2: i32 = 10;
pub const FBTYPE_NOTSUN3: i32 = 11;
pub const FBTYPE_SUNFAST_COLOR: i32 = 12;
pub const FBTYPE_SUNROP_COLOR: i32 = 13;
pub const FBTYPE_SUNFB_VIDEO: i32 = 14;
pub const FBTYPE_SUNGIFB: i32 = 15;
pub const FBTYPE_SUNGPLAS: i32 = 16;
pub const FBTYPE_SUNGP3: i32 = 17;
pub const FBTYPE_SUNGT: i32 = 18;
pub const FBTYPE_SUNLEO: i32 = 19;
pub const FBTYPE_MDICOLOR: i32 = 20;
pub const FBTYPE_TCXCOLOR: i32 = 21;
pub const FBTYPE_LASTPLUSONE: i32 = 21;
pub const FBTYPE_CREATOR: i32 = 22;
pub const FBTYPE_PCI_IGA1682: i32 = 23;
pub const FBTYPE_P9100COLOR: i32 = 24;
pub const FBTYPE_PCI_GENERIC: i32 = 1000;
pub const FBTYPE_PCI_MACH64: i32 = 1001;

#[repr(C)]
pub struct fbtype { pub fb_type: i32, pub fb_height: i32, pub fb_width: i32, pub fb_depth: i32, pub fb_cmsize: i32, pub fb_size: i32 }

#[repr(C)]
pub struct fbcmap {
    pub index: i32, pub count: i32,
    pub red: *mut u8, pub green: *mut u8, pub blue: *mut u8,
}

pub const FB_ATTR_NDEVSPECIFIC: usize = 8;
pub const FB_ATTR_NEMUTYPES: usize = 4;

#[repr(C)]
pub struct fbsattr { pub flags: i32, pub emu_type: i32, pub dev_specific: [i32; FB_ATTR_NDEVSPECIFIC] }
#[repr(C)]
pub struct fbgattr { pub real_type: i32, pub owner: i32, pub fbtype: fbtype, pub sattr: fbsattr, pub emu_types: [i32; FB_ATTR_NEMUTYPES] }

#[repr(C)]
pub struct fbcursor {
    pub set: i16, pub enable: i16, pub pos: fbcurpos, pub hot: fbcurpos,
    pub cmap: fbcmap, pub size: fbcurpos, pub image: *mut i8, pub mask: *mut i8,
}

pub const FB_WID_SHARED_8: u32 = 0;
pub const FB_WID_SHARED_24: u32 = 1;
pub const FB_WID_DBL_8: u32 = 2;
pub const FB_WID_DBL_24: u32 = 3;
#[repr(C)] pub struct fb_wid_alloc { pub wa_type: u32, pub wa_index: i32, pub wa_count: u32 }
#[repr(C)] pub struct fb_wid_item { pub wi_type: u32, pub wi_index: i32, pub wi_attrs: u32, pub wi_values: [u32; 32] }
#[repr(C)] pub struct fb_wid_list { pub wl_flags: u32, pub wl_count: u32, pub wl_list: *mut fb_wid_item }

pub const FFB_IOCTL: u32 = ('F' as u32) << 8;
pub const FFB_SYS_INFO: u32 = FFB_IOCTL | 80;
pub const FFB_CLUTREAD: u32 = FFB_IOCTL | 81;
pub const FFB_CLUTPOST: u32 = FFB_IOCTL | 82;
pub const FFB_SETDIAGMODE: u32 = FFB_IOCTL | 83;
pub const FFB_GETMONITORID: u32 = FFB_IOCTL | 84;
pub const FFB_GETVIDEOMODE: u32 = FFB_IOCTL | 85;
pub const FFB_SETVIDEOMODE: u32 = FFB_IOCTL | 86;
pub const FFB_SETSERVER: u32 = FFB_IOCTL | 87;
pub const FFB_SETOVCTL: u32 = FFB_IOCTL | 88;
pub const FFB_GETOVCTL: u32 = FFB_IOCTL | 89;
pub const FFB_GETSAXNUM: u32 = FFB_IOCTL | 90;
pub const FFB_FBDEBUG: u32 = FFB_IOCTL | 91;

pub const MDI_IOCTL: u32 = ('M' as u32) << 8;
pub const MDI_RESET: u32 = MDI_IOCTL | 1;
pub const MDI_GET_CFGINFO: u32 = MDI_IOCTL | 2;
pub const MDI_SET_PIXELMODE: u32 = MDI_IOCTL | 3;
pub const MDI_32_PIX: i32 = 32;
pub const MDI_16_PIX: i32 = 16;
pub const MDI_8_PIX: i32 = 8;
#[repr(C)] pub struct mdi_cfginfo { pub mdi_ncluts: i32, pub mdi_type: i32, pub mdi_height: i32, pub mdi_width: i32, pub mdi_size: i32, pub mdi_mode: i32, pub mdi_pixfreq: i32 }
pub const MDI_CLEAR_XLUT: u32 = MDI_IOCTL | 9;

#[repr(C)] pub struct fb_clut_alloc { pub clutid: u32, pub flag: u32, pub index: u32 }
pub const FB_CLUT_WAIT: u32 = 0x00000001;
#[repr(C)] pub struct fb_clut { pub flag: u32, pub clutid: u32, pub offset: u32, pub count: u32, pub red: *mut i8, pub green: *mut i8, pub blue: *mut i8 }
#[repr(C)] pub struct fb_clut32 { pub flag: u32, pub clutid: u32, pub offset: u32, pub count: u32, pub red: u32, pub green: u32, pub blue: u32 }

pub const MDI_DIRECT_MAP: u32 = 0x10000000;
pub const MDI_CTLREG_MAP: u32 = 0x20000000;
pub const MDI_CURSOR_MAP: u32 = 0x30000000;
pub const MDI_SHDW_VRT_MAP: u32 = 0x40000000;
pub const MDI_CHUNKY_XBGR_MAP: u32 = 0x50000000;
pub const MDI_CHUNKY_BGR_MAP: u32 = 0x60000000;
pub const MDI_PLANAR_X16_MAP: u32 = 0x70000000;
pub const MDI_PLANAR_C16_MAP: u32 = 0x80000000;
pub const MDI_PLANAR_X32_MAP: u32 = 0x90000000;
pub const MDI_PLANAR_B32_MAP: u32 = 0xa0000000;

/* FBIO*/LEO_* ioctl constants use the source header's _IO*, _IOR, _IOW and
 * _IOWR macros; those architecture-dependent encodings remain dependency
 * supplied rather than being guessed here. */
/*
 * FBIOGTYPE = _IOR('F', 0, struct fbtype)
 * FBIOPUTCMAP = _IOW('F', 3, struct fbcmap)
 * FBIOGETCMAP = _IOW('F', 4, struct fbcmap)
 * FBIOSATTR = _IOW('F', 5, struct fbgattr), FBIOGATTR = _IOR('F', 6, struct fbgattr)
 * FBIOSVIDEO = _IOW('F', 7, int), FBIOGVIDEO = _IOR('F', 8, int)
 * FBIOSCURSOR = _IOW('F', 24, struct fbcursor), FBIOGCURSOR = _IOWR('F', 25, struct fbcursor)
 * FBIOSCURPOS = _IOW('F', 26, struct fbcurpos), FBIOGCURPOS = _IOW('F', 27, struct fbcurpos)
 * FBIOGCURMAX = _IOR('F', 28, struct fbcurpos)
 * FBIO_WID_ALLOC = _IOWR('F', 30, struct fb_wid_alloc), FBIO_WID_FREE = _IOW('F', 31, struct fb_wid_alloc)
 * FBIO_WID_PUT = _IOW('F', 32, struct fb_wid_list), FBIO_WID_GET = _IOWR('F', 33, struct fb_wid_list)
 * LEO_CLUTALLOC = _IOWR('L', 53, struct fb_clut_alloc), LEO_CLUTFREE = _IOW('L', 54, struct fb_clut_alloc)
 * LEO_CLUTREAD = _IOW('L', 55, struct fb_clut), LEO_CLUTPOST = _IOW('L', 56, struct fb_clut)
 * LEO_SETGAMMA = _IOW('L', 68, int), LEO_GETGAMMA = _IOR('L', 69, int)
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
