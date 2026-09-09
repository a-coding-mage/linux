/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux UAPI header <linux/kd.h>. */

/* 0x4B is 'K', to avoid collision with termios and vt */

pub const GIO_FONT: u32 = 0x4B60; /* gets font in expanded form */
pub const PIO_FONT: u32 = 0x4B61; /* use font in expanded form */

pub const GIO_FONTX: u32 = 0x4B6B; /* get font using struct consolefontdesc */
pub const PIO_FONTX: u32 = 0x4B6C; /* set font using struct consolefontdesc */
#[repr(C)]
pub struct consolefontdesc {
    pub charcount: u16, /* characters in font (256 or 512) */
    pub charheight: u16, /* scan lines per character (1-32) */
    pub chardata: *mut core::ffi::c_char, /* font data in expanded form */
}

pub const PIO_FONTRESET: u32 = 0x4B6D; /* reset to default font */
pub const GIO_CMAP: u32 = 0x4B70; /* gets colour palette on VGA+ */
pub const PIO_CMAP: u32 = 0x4B71; /* sets colour palette on VGA+ */
pub const KIOCSOUND: u32 = 0x4B2F; /* start sound generation (0 for off) */
pub const KDMKTONE: u32 = 0x4B30; /* generate tone */
pub const KDGETLED: u32 = 0x4B31; /* return current led state */
pub const KDSETLED: u32 = 0x4B32; /* set led state [lights, not flags] */
pub const LED_SCR: u32 = 0x01; /* scroll lock led */
pub const LED_NUM: u32 = 0x02; /* num lock led */
pub const LED_CAP: u32 = 0x04; /* caps lock led */
pub const KDGKBTYPE: u32 = 0x4B33; /* get keyboard type */
pub const KB_84: u32 = 0x01;
pub const KB_101: u32 = 0x02; /* this is what we always answer */
pub const KB_OTHER: u32 = 0x03;
pub const KDADDIO: u32 = 0x4B34; /* add i/o port as valid */
pub const KDDELIO: u32 = 0x4B35; /* del i/o port as valid */
pub const KDENABIO: u32 = 0x4B36; /* enable i/o to video board */
pub const KDDISABIO: u32 = 0x4B37; /* disable i/o to video board */
pub const KDSETMODE: u32 = 0x4B3A; /* set text/graphics mode */
pub const KD_TEXT: u32 = 0x00;
pub const KD_GRAPHICS: u32 = 0x01;
pub const KD_TEXT0: u32 = 0x02; /* obsolete */
pub const KD_TEXT1: u32 = 0x03; /* obsolete */
pub const KDGETMODE: u32 = 0x4B3B; /* get current mode */
pub const KDMAPDISP: u32 = 0x4B3C; /* map display into address space */
pub const KDUNMAPDISP: u32 = 0x4B3D; /* unmap display from address space */

pub type scrnmap_t = core::ffi::c_char;
pub const E_TABSZ: usize = 256;
pub const GIO_SCRNMAP: u32 = 0x4B40; /* get screen mapping from kernel */
pub const PIO_SCRNMAP: u32 = 0x4B41; /* put screen mapping table in kernel */
pub const GIO_UNISCRNMAP: u32 = 0x4B69; /* get full Unicode screen mapping */
pub const PIO_UNISCRNMAP: u32 = 0x4B6A; /* set full Unicode screen mapping */
pub const GIO_UNIMAP: u32 = 0x4B66; /* get unicode-to-font mapping from kernel */
#[repr(C)]
pub struct unipair { pub unicode: u16, pub fontpos: u16 }
#[repr(C)]
pub struct unimapdesc { pub entry_ct: u16, pub entries: *mut unipair }
pub const PIO_UNIMAP: u32 = 0x4B67; /* put unicode-to-font mapping in kernel */
pub const PIO_UNIMAPCLR: u32 = 0x4B68; /* clear table, possibly advise hash algorithm */
#[repr(C)]
pub struct unimapinit { pub advised_hashsize: u16, pub advised_hashstep: u16, pub advised_hashlevel: u16 }
pub const UNI_DIRECT_BASE: u32 = 0xF000;
pub const UNI_DIRECT_MASK: u32 = 0x01FF;
pub const K_RAW: u32 = 0x00;
pub const K_XLATE: u32 = 0x01;
pub const K_MEDIUMRAW: u32 = 0x02;
pub const K_UNICODE: u32 = 0x03;
pub const K_OFF: u32 = 0x04;
pub const KDGKBMODE: u32 = 0x4B44;
pub const KDSKBMODE: u32 = 0x4B45;
pub const K_METABIT: u32 = 0x03;
pub const K_ESCPREFIX: u32 = 0x04;
pub const KDGKBMETA: u32 = 0x4B62;
pub const KDSKBMETA: u32 = 0x4B63;
pub const K_SCROLLLOCK: u32 = 0x01;
pub const K_NUMLOCK: u32 = 0x02;
pub const K_CAPSLOCK: u32 = 0x04;
pub const KDGKBLED: u32 = 0x4B64;
pub const KDSKBLED: u32 = 0x4B65;

#[repr(C)]
pub struct kbentry { pub kb_table: u8, pub kb_index: u8, pub kb_value: u16 }
pub const K_NORMTAB: u32 = 0x00;
pub const K_SHIFTTAB: u32 = 0x01;
pub const K_ALTTAB: u32 = 0x02;
pub const K_ALTSHIFTTAB: u32 = 0x03;
pub const KDGKBENT: u32 = 0x4B46;
pub const KDSKBENT: u32 = 0x4B47;
#[repr(C)]
pub struct kbsentry { pub kb_func: u8, pub kb_string: [u8; 512] }
pub const KDGKBSENT: u32 = 0x4B48;
pub const KDSKBSENT: u32 = 0x4B49;
#[repr(C)]
pub struct kbdiacr { pub diacr: u8, pub base: u8, pub result: u8 }
#[repr(C)]
pub struct kbdiacrs { pub kb_cnt: u32, pub kbdiacr: [kbdiacr; 256] }
pub const KDGKBDIACR: u32 = 0x4B4A;
pub const KDSKBDIACR: u32 = 0x4B4B;
#[repr(C)]
pub struct kbdiacruc { pub diacr: u32, pub base: u32, pub result: u32 }
#[repr(C)]
pub struct kbdiacrsuc { pub kb_cnt: u32, pub kbdiacruc: [kbdiacruc; 256] }
pub const KDGKBDIACRUC: u32 = 0x4BFA;
pub const KDSKBDIACRUC: u32 = 0x4BFB;
#[repr(C)]
pub struct kbkeycode { pub scancode: u32, pub keycode: u32 }
pub const KDGETKEYCODE: u32 = 0x4B4C;
pub const KDSETKEYCODE: u32 = 0x4B4D;
pub const KDSIGACCEPT: u32 = 0x4B4E;
#[repr(C)]
pub struct kbd_repeat { pub delay: i32, pub period: i32 }
pub const KDKBDREP: u32 = 0x4B52;
pub const KDFONTOP: u32 = 0x4B72;
#[repr(C)]
pub struct console_font_op { pub op: u32, pub flags: u32, pub width: u32, pub height: u32, pub charcount: u32, pub data: *mut u8 }
#[repr(C)]
pub struct console_font { pub width: u32, pub height: u32, pub charcount: u32, pub data: *mut u8 }
pub const KD_FONT_OP_SET: u32 = 0;
pub const KD_FONT_OP_GET: u32 = 1;
pub const KD_FONT_OP_SET_DEFAULT: u32 = 2;
pub const KD_FONT_OP_COPY: u32 = 3;
pub const KD_FONT_OP_SET_TALL: u32 = 4;
pub const KD_FONT_OP_GET_TALL: u32 = 5;
pub const KD_FONT_FLAG_DONT_RECALC: u32 = 1;

/* note: 0x4B00-0x4B4E all have had a value at some time; don't reuse for the time being */
/* note: 0x4B60-0x4B6D, 0x4B70-0x4B72 used above */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
