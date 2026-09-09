/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// These are set up by the setup-routine at boot-time:
#[repr(C, packed)]
pub struct screen_info {
    pub orig_x: u8,             /* 0x00 */
    pub orig_y: u8,             /* 0x01 */
    pub ext_mem_k: u16,         /* 0x02 */
    pub orig_video_page: u16,   /* 0x04 */
    pub orig_video_mode: u8,    /* 0x06 */
    pub orig_video_cols: u8,    /* 0x07 */
    pub flags: u8,              /* 0x08 */
    pub unused2: u8,            /* 0x09 */
    pub orig_video_ega_bx: u16, /* 0x0a */
    pub unused3: u16,           /* 0x0c */
    pub orig_video_lines: u8,   /* 0x0e */
    pub orig_video_isVGA: u8,   /* 0x0f */
    pub orig_video_points: u16, /* 0x10 */

    /* VESA graphic mode -- linear frame buffer */
    pub lfb_width: u16,         /* 0x12 */
    pub lfb_height: u16,        /* 0x14 */
    pub lfb_depth: u16,         /* 0x16 */
    pub lfb_base: u32,          /* 0x18 */
    pub lfb_size: u32,          /* 0x1c */
    pub cl_magic: u16,          /* 0x20 */
    pub cl_offset: u16,         /* 0x20 */
    pub lfb_linelength: u16,    /* 0x24 */
    pub red_size: u8,           /* 0x26 */
    pub red_pos: u8,            /* 0x27 */
    pub green_size: u8,         /* 0x28 */
    pub green_pos: u8,          /* 0x29 */
    pub blue_size: u8,          /* 0x2a */
    pub blue_pos: u8,           /* 0x2b */
    pub rsvd_size: u8,          /* 0x2c */
    pub rsvd_pos: u8,           /* 0x2d */
    pub vesapm_seg: u16,        /* 0x2e */
    pub vesapm_off: u16,        /* 0x30 */
    pub pages: u16,             /* 0x32 */
    pub vesa_attributes: u16,   /* 0x34 */
    pub capabilities: u32,      /* 0x36 */
    pub ext_lfb_base: u32,      /* 0x3a */
    pub _reserved: [u8; 2],     /* 0x3e */
}

pub const VIDEO_TYPE_MDA: u32 = 0x10;       /* Monochrome Text Display */
pub const VIDEO_TYPE_CGA: u32 = 0x11;       /* CGA Display */
pub const VIDEO_TYPE_EGAM: u32 = 0x20;      /* EGA/VGA in Monochrome Mode */
pub const VIDEO_TYPE_EGAC: u32 = 0x21;      /* EGA in Color Mode */
pub const VIDEO_TYPE_VGAC: u32 = 0x22;      /* VGA+ in Color Mode */
pub const VIDEO_TYPE_VLFB: u32 = 0x23;      /* VESA VGA in graphic mode */
pub const VIDEO_TYPE_PICA_S3: u32 = 0x30;   /* ACER PICA-61 local S3 video */
pub const VIDEO_TYPE_MIPS_G364: u32 = 0x31; /* MIPS Magnum 4000 G364 video */
pub const VIDEO_TYPE_SGI: u32 = 0x33;       /* Various SGI graphics hardware */
pub const VIDEO_TYPE_TGAC: u32 = 0x40;      /* DEC TGA */
pub const VIDEO_TYPE_SUN: u32 = 0x50;       /* Sun frame buffer. */
pub const VIDEO_TYPE_SUNPCI: u32 = 0x51;    /* Sun PCI based frame buffer. */
pub const VIDEO_TYPE_PMAC: u32 = 0x60;      /* PowerMacintosh frame buffer. */
pub const VIDEO_TYPE_EFI: u32 = 0x70;       /* EFI graphic mode */

pub const VIDEO_FLAGS_NOCURSOR: u32 = 1 << 0; /* The video mode has no cursor set */

pub const VIDEO_CAPABILITY_SKIP_QUIRKS: u32 = 1 << 0;
pub const VIDEO_CAPABILITY_64BIT_BASE: u32 = 1 << 1; /* Frame buffer base is 64-bit */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
