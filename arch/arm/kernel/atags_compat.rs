// SPDX-License-Identifier: GPL-2.0-only
// Translation of linux/arch/arm/kernel/atags_compat.c.
// Includes and build-time configuration supplied by the surrounding kernel are
// intentionally represented by external names/configuration below.

use core::ffi::{c_char, c_int, c_ulong, c_void};

// External kernel types, constants, macros, and functions supplied elsewhere.
extern "C" {
    fn tag_next(tag: *mut tag) -> *mut tag;
    fn tag_size(tag_type: c_ulong) -> c_ulong;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pr_warn(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
}

// These declarations are provided by asm/setup.h and related kernel headers.
#[allow(non_camel_case_types)]
type tag = crate::tag;
extern "C" {
    static PAGE_SIZE: c_ulong;
    static PHYS_OFFSET: c_ulong;
    static ATAG_MEM: c_ulong;
    static ATAG_CORE: c_ulong;
    static ATAG_RAMDISK: c_ulong;
    static ATAG_INITRD: c_ulong;
    static ATAG_SERIAL: c_ulong;
    static ATAG_REVISION: c_ulong;
    static ATAG_MEMCLK: c_ulong;
    static ATAG_VIDEOTEXT: c_ulong;
    static ATAG_ACORN: c_ulong;
    static ATAG_CMDLINE: c_ulong;
    static ATAG_NONE: c_ulong;
    static tag_mem32: c_ulong;
    static tag_core: c_ulong;
    static tag_ramdisk: c_ulong;
    static tag_initrd: c_ulong;
    static tag_serialnr: c_ulong;
    static tag_revision: c_ulong;
    static tag_memclk: c_ulong;
    static tag_videotext: c_ulong;
    static tag_acorn: c_ulong;
}

#[repr(C)]
pub struct param_struct {
    pub u1: param_u1,
    pub u2: param_u2,
    pub commandline: [c_char; COMMAND_LINE_SIZE],
}

pub const FLAG_READONLY: c_ulong = 1;
pub const FLAG_RDLOAD: c_ulong = 4;
pub const FLAG_RDPROMPT: c_ulong = 8;
pub const COMMAND_LINE_SIZE: usize = 1024;

#[repr(C)]
pub union param_u1 {
    pub s: param_s,
    pub unused: [c_char; 256],
}

#[repr(C)]
pub struct param_s {
    pub page_size: c_ulong,
    pub nr_pages: c_ulong,
    pub ramdisk_size: c_ulong,
    pub flags: c_ulong,
    pub rootdev: c_ulong,
    pub video_num_cols: c_ulong,
    pub video_num_rows: c_ulong,
    pub video_x: c_ulong,
    pub video_y: c_ulong,
    pub memc_control_reg: c_ulong,
    pub sounddefault: u8,
    pub adfsdrives: u8,
    pub bytes_per_char_h: u8,
    pub bytes_per_char_v: u8,
    pub pages_in_bank: [c_ulong; 4],
    pub pages_in_vram: c_ulong,
    pub initrd_start: c_ulong,
    pub initrd_size: c_ulong,
    pub rd_start: c_ulong,
    pub system_rev: c_ulong,
    pub system_serial_low: c_ulong,
    pub system_serial_high: c_ulong,
    pub mem_fclk_21285: c_ulong,
}

#[repr(C)]
pub union param_u2 {
    pub paths: [[c_char; 128]; 8],
    pub s: param_u2_s,
}

#[repr(C)]
pub struct param_u2_s {
    pub magic: c_ulong,
    pub n: [c_char; 1024 - core::mem::size_of::<c_ulong>()],
}

unsafe fn memtag(mut tagp: *mut tag, start: c_ulong, size: c_ulong) -> *mut tag {
    tagp = tag_next(tagp);
    (*tagp).hdr.tag = ATAG_MEM;
    (*tagp).hdr.size = tag_size(tag_mem32);
    (*tagp).u.mem.size = size;
    (*tagp).u.mem.start = start;
    tagp
}

unsafe fn build_tag_list(params: *mut param_struct, taglist: *mut c_void) {
    let mut tagp = taglist as *mut tag;
    if (*params).u1.s.page_size != PAGE_SIZE {
        pr_warn(b"Warning: bad configuration page, trying to continue\0".as_ptr() as *const c_char);
        return;
    }
    printk(b"Converting old-style param struct to taglist\n\0".as_ptr() as *const c_char);

    #[cfg(CONFIG_ARCH_NETWINDER)]
    if (*params).u1.s.nr_pages != 0x02000 && (*params).u1.s.nr_pages != 0x04000 &&
        (*params).u1.s.nr_pages != 0x08000 && (*params).u1.s.nr_pages != 0x10000 {
        pr_warn(b"Warning: bad NeTTrom parameters detected, using defaults\n\0".as_ptr() as *const c_char);
        (*params).u1.s.nr_pages = 0x1000;
        (*params).u1.s.ramdisk_size = 0;
        (*params).u1.s.flags = FLAG_READONLY;
        (*params).u1.s.initrd_start = 0;
        (*params).u1.s.initrd_size = 0;
        (*params).u1.s.rd_start = 0;
    }

    (*tagp).hdr.tag = ATAG_CORE;
    (*tagp).hdr.size = tag_size(tag_core);
    (*tagp).u.core.flags = (*params).u1.s.flags & FLAG_READONLY;
    (*tagp).u.core.pagesize = (*params).u1.s.page_size;
    (*tagp).u.core.rootdev = (*params).u1.s.rootdev;

    tagp = tag_next(tagp);
    (*tagp).hdr.tag = ATAG_RAMDISK;
    (*tagp).hdr.size = tag_size(tag_ramdisk);
    (*tagp).u.ramdisk.flags = if (*params).u1.s.flags & FLAG_RDLOAD != 0 { 1 } else { 0 } |
        if (*params).u1.s.flags & FLAG_RDPROMPT != 0 { 2 } else { 0 };
    (*tagp).u.ramdisk.size = (*params).u1.s.ramdisk_size;
    (*tagp).u.ramdisk.start = (*params).u1.s.rd_start;

    tagp = tag_next(tagp); (*tagp).hdr.tag = ATAG_INITRD; (*tagp).hdr.size = tag_size(tag_initrd);
    (*tagp).u.initrd.start = (*params).u1.s.initrd_start; (*tagp).u.initrd.size = (*params).u1.s.initrd_size;
    tagp = tag_next(tagp); (*tagp).hdr.tag = ATAG_SERIAL; (*tagp).hdr.size = tag_size(tag_serialnr);
    (*tagp).u.serialnr.low = (*params).u1.s.system_serial_low; (*tagp).u.serialnr.high = (*params).u1.s.system_serial_high;
    tagp = tag_next(tagp); (*tagp).hdr.tag = ATAG_REVISION; (*tagp).hdr.size = tag_size(tag_revision);
    (*tagp).u.revision.rev = (*params).u1.s.system_rev;
    #[cfg(CONFIG_ARCH_ACORN)]
    if machine_is_riscpc() {
        for i in 0..4 {
            tagp = memtag(tagp, PHYS_OFFSET + ((i as c_ulong) << 26),
                (*params).u1.s.pages_in_bank[i].wrapping_mul(PAGE_SIZE));
        }
    } else {
        tagp = memtag(tagp, PHYS_OFFSET, (*params).u1.s.nr_pages.wrapping_mul(PAGE_SIZE));
    }
    #[cfg(not(CONFIG_ARCH_ACORN))]
    { tagp = memtag(tagp, PHYS_OFFSET, (*params).u1.s.nr_pages.wrapping_mul(PAGE_SIZE)); }

    #[cfg(CONFIG_FOOTBRIDGE)]
    if (*params).u1.s.mem_fclk_21285 != 0 {
        tagp = tag_next(tagp);
        (*tagp).hdr.tag = ATAG_MEMCLK;
        (*tagp).hdr.size = tag_size(tag_memclk);
        (*tagp).u.memclk.fmemclk = (*params).u1.s.mem_fclk_21285;
    }

    #[cfg(CONFIG_ARCH_EBSA285)]
    if machine_is_ebsa285() {
        tagp = tag_next(tagp);
        (*tagp).hdr.tag = ATAG_VIDEOTEXT;
        (*tagp).hdr.size = tag_size(tag_videotext);
        (*tagp).u.videotext.x = (*params).u1.s.video_x;
        (*tagp).u.videotext.y = (*params).u1.s.video_y;
        (*tagp).u.videotext.video_page = 0;
        (*tagp).u.videotext.video_mode = 0;
        (*tagp).u.videotext.video_cols = (*params).u1.s.video_num_cols;
        (*tagp).u.videotext.video_ega_bx = 0;
        (*tagp).u.videotext.video_lines = (*params).u1.s.video_num_rows;
        (*tagp).u.videotext.video_isvga = 1;
        (*tagp).u.videotext.video_points = 8;
    }

    #[cfg(CONFIG_ARCH_ACORN)]
    {
        tagp = tag_next(tagp);
        (*tagp).hdr.tag = ATAG_ACORN;
        (*tagp).hdr.size = tag_size(tag_acorn);
        (*tagp).u.acorn.memc_control_reg = (*params).u1.s.memc_control_reg;
        (*tagp).u.acorn.vram_pages = (*params).u1.s.pages_in_vram;
        (*tagp).u.acorn.sounddefault = (*params).u1.s.sounddefault;
        (*tagp).u.acorn.adfsdrives = (*params).u1.s.adfsdrives;
    }

    tagp = tag_next(tagp); (*tagp).hdr.tag = ATAG_CMDLINE;
    (*tagp).hdr.size = (strlen((*params).commandline.as_ptr()) + 3 + core::mem::size_of::<crate::tag_header>()) as c_ulong >> 2;
    strcpy((*tagp).u.cmdline.cmdline.as_mut_ptr(), (*params).commandline.as_ptr());
    tagp = tag_next(tagp); (*tagp).hdr.tag = ATAG_NONE; (*tagp).hdr.size = 0;
    memmove(params as *mut c_void, taglist, (tagp as usize - taglist as usize) + core::mem::size_of::<crate::tag_header>());
}

pub unsafe fn convert_to_tag_list(tags: *mut tag) {
    let params = tags as *mut param_struct;
    build_tag_list(params, &mut (*params).u2 as *mut param_u2 as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
