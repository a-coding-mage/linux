/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * linux/include/asm/setup.h
 *
 * Structure passed to kernel to tell it about the hardware it's running on.
 * See Documentation/arch/arm/setup.rst for more info.
 */

// Dependency intent: `__u8`, `__u16`, and `__u32` are supplied by linux/types.h.

pub const COMMAND_LINE_SIZE: usize = 1024;

/* The list ends with an ATAG_NONE node. */
pub const ATAG_NONE: u32 = 0x0000_0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_header {
    pub size: u32,
    pub tag: u32,
}

/* The list must start with an ATAG_CORE node */
pub const ATAG_CORE: u32 = 0x5441_0001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_core {
    pub flags: u32,    /* bit 0 = read-only */
    pub pagesize: u32,
    pub rootdev: u32,
}

/* it is allowed to have multiple ATAG_MEM nodes */
pub const ATAG_MEM: u32 = 0x5441_0002;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_mem32 {
    pub size: u32,
    pub start: u32,    /* physical start address */
}

/* VGA text type displays */
pub const ATAG_VIDEOTEXT: u32 = 0x5441_0003;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_videotext {
    pub x: u8,
    pub y: u8,
    pub video_page: u16,
    pub video_mode: u8,
    pub video_cols: u8,
    pub video_ega_bx: u16,
    pub video_lines: u8,
    pub video_isvga: u8,
    pub video_points: u16,
}

/* describes how the ramdisk will be used in kernel */
pub const ATAG_RAMDISK: u32 = 0x5441_0004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_ramdisk {
    pub flags: u32,    /* bit 0 = load, bit 1 = prompt */
    pub size: u32,     /* decompressed ramdisk size in _kilo_ bytes */
    pub start: u32,    /* starting block of floppy-based RAM disk image */
}

/* describes where the compressed ramdisk image lives (virtual address) */
/* this one accidentally used virtual addresses - as such, it's deprecated. */
pub const ATAG_INITRD: u32 = 0x5441_0005;

/* describes where the compressed ramdisk image lives (physical address) */
pub const ATAG_INITRD2: u32 = 0x5442_0005;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_initrd {
    pub start: u32,    /* physical start address */
    pub size: u32,     /* size of compressed ramdisk image in bytes */
}

/* board serial number. "64 bits should be enough for everybody" */
pub const ATAG_SERIAL: u32 = 0x5441_0006;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_serialnr {
    pub low: u32,
    pub high: u32,
}

/* board revision */
pub const ATAG_REVISION: u32 = 0x5441_0007;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_revision {
    pub rev: u32,
}

/* initial values for vesafb-type framebuffers. see struct screen_info */
pub const ATAG_VIDEOLFB: u32 = 0x5441_0008;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_videolfb {
    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_linelength: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
}

/* command line: \0 terminated string */
pub const ATAG_CMDLINE: u32 = 0x5441_0009;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_cmdline {
    pub cmdline: [i8; 1],    /* this is the minimum size */
}

/* acorn RiscPC specific information */
pub const ATAG_ACORN: u32 = 0x4100_0101;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_acorn {
    pub memc_control_reg: u32,
    pub vram_pages: u32,
    pub sounddefault: u8,
    pub adfsdrives: u8,
}

/* footbridge memory clock, see arch/arm/mach-footbridge/arch.c */
pub const ATAG_MEMCLK: u32 = 0x4100_0402;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_memclk {
    pub fmemclk: u32,
}

#[repr(C)]
pub union tag_u {
    pub core: tag_core,
    pub mem: tag_mem32,
    pub videotext: tag_videotext,
    pub ramdisk: tag_ramdisk,
    pub initrd: tag_initrd,
    pub serialnr: tag_serialnr,
    pub revision: tag_revision,
    pub videolfb: tag_videolfb,
    pub cmdline: tag_cmdline,
    /* Acorn specific */
    pub acorn: tag_acorn,
    /* DC21285 specific */
    pub memclk: tag_memclk,
}

#[repr(C)]
pub struct tag {
    pub hdr: tag_header,
    pub u: tag_u,
}

#[repr(C)]
pub struct tagtable {
    pub tag: u32,
    pub parse: Option<unsafe extern "C" fn(*const tag) -> i32>,
}

#[macro_export]
macro_rules! tag_member_present {
    ($tag:expr, $member:ident) => {
        (unsafe {
            ((core::ptr::addr_of!((*($tag as *const $crate::tag)).$member).add(1) as usize)
                <= (($tag).as_ref().unwrap().hdr.size as usize).wrapping_mul(4))
        })
    };
}

#[inline]
pub unsafe fn tag_next(t: *mut tag) -> *mut tag {
    (t as *mut u32).add((*t).hdr.size as usize) as *mut tag
}

#[inline]
pub const fn tag_size<T>() -> usize {
    (core::mem::size_of::<tag_header>() + core::mem::size_of::<T>()) >> 2
}

/* Build-time iteration macro equivalent to C's for_each_tag. */
#[macro_export]
macro_rules! for_each_tag {
    ($t:ident, $base:expr, $body:block) => {{
        let mut $t: *mut $crate::tag = $base;
        while unsafe { !$t.is_null() && (*$t).hdr.size != 0 } {
            $body
            $t = unsafe { $crate::tag_next($t) };
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
