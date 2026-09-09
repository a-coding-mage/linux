// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Intel Corporation; author Matt Fleming
 */

// Dependencies supplied by the surrounding kernel translation.

static mut earlycon_console: *const console = core::ptr::null();
static mut font: *const font_desc = core::ptr::null();
static mut cur_line_y: u16 = 0;
static mut max_line_y: u16 = 0;
static mut efi_x_array: [u32; 1024] = [0; 1024];
static mut efi_x: u32 = 0;
static mut efi_y: u32 = 0;
static mut fb_base: u64 = 0;
static mut fb_wb: bool = false;
static mut efi_fb: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    static mut sysfb_primary_display: sysfb_display;
    fn console_is_registered(con: *const console) -> bool;
    fn memremap(offset: u64, size: u64, flags: u32) -> *mut core::ffi::c_void;
    fn memunmap(addr: *mut core::ffi::c_void);
    fn early_memremap_prot(start: u64, len: usize, prot: usize) -> *mut core::ffi::c_void;
    fn early_memunmap(addr: *mut core::ffi::c_void, len: usize);
    fn pgprot_writecombine(prot: usize) -> usize;
    fn pgprot_val(prot: usize) -> usize;
    fn memset(dst: *mut core::ffi::c_void, value: i32, len: usize) -> *mut core::ffi::c_void;
    fn memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    fn strnchrnul(s: *const i8, count: usize, c: i32) -> *const i8;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn get_default_font(xres: u16, yres: u16, a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> *const font_desc;
    fn setup_earlycon(name: *const i8);
}

#[repr(C)]
struct console { write: Option<unsafe extern "C" fn(*mut console, *const i8, u32)> }
#[repr(C)] struct font_desc { width: u32, height: u32, data: *const u8 }
#[repr(C)] struct screen_info { lfb_size: u64, lfb_linelength: u16, lfb_height: u16, lfb_width: u16, orig_video_isVGA: u8, capabilities: u16, lfb_base: u32, ext_lfb_base: u32, lfb_depth: u8 }
#[repr(C)] struct sysfb_display { screen: screen_info }
#[repr(C)] struct earlycon_device { con: *mut console }

const MEMREMAP_WB: u32 = 1;
const MEMREMAP_WC: u32 = 2;
const PAGE_KERNEL: usize = 0;
const VIDEO_TYPE_EFI: u8 = 0x04;
const VIDEO_CAPABILITY_64BIT_BASE: u16 = 0x02;

unsafe fn efi_earlycon_remap_fb() -> i32 {
    let si = &sysfb_primary_display.screen;
    if earlycon_console.is_null() || !console_is_registered(earlycon_console) { return 0; }
    efi_fb = memremap(fb_base, si.lfb_size, if fb_wb { MEMREMAP_WB } else { MEMREMAP_WC });
    if !efi_fb.is_null() { 0 } else { -12 }
}

unsafe fn efi_earlycon_unmap_fb() -> i32 {
    if !efi_fb.is_null() && !console_is_registered(earlycon_console) { memunmap(efi_fb); }
    0
}

unsafe fn efi_earlycon_map(start: usize, len: usize) -> *mut core::ffi::c_void {
    if !efi_fb.is_null() { return (efi_fb as *mut u8).add(start) as *mut core::ffi::c_void; }
    let fb_prot = if fb_wb { PAGE_KERNEL } else { pgprot_writecombine(PAGE_KERNEL) };
    early_memremap_prot(fb_base + start as u64, len, pgprot_val(fb_prot))
}

unsafe fn efi_earlycon_unmap(addr: *mut core::ffi::c_void, len: usize) {
    if efi_fb.is_null() { early_memunmap(addr, len); }
}

unsafe fn efi_earlycon_clear_scanline(y: u32, si: &screen_info) {
    let len = si.lfb_linelength as usize;
    let dst = efi_earlycon_map(y as usize * len, len);
    if !dst.is_null() { memset(dst, 0, len); efi_earlycon_unmap(dst, len); }
}

unsafe fn efi_earlycon_scroll_up(si: &screen_info) {
    let mut maxlen: u16 = 0;
    for i in 0..max_line_y as usize { if efi_x_array[i] > maxlen as u32 { maxlen = efi_x_array[i] as u16; } }
    maxlen = maxlen.wrapping_mul(4);
    let len = si.lfb_linelength as usize;
    for i in 0..(si.lfb_height as u32 - (*font).height) {
        let dst = efi_earlycon_map(i as usize * len, len);
        if dst.is_null() { return; }
        let src = efi_earlycon_map((i + (*font).height) as usize * len, len);
        if src.is_null() { efi_earlycon_unmap(dst, len); return; }
        memmove(dst, src, maxlen as usize);
        efi_earlycon_unmap(src, len); efi_earlycon_unmap(dst, len);
    }
}

unsafe fn efi_earlycon_write_char(dst: *mut u32, c: u8, h: u32, _si: &screen_info) {
    let bytes = ((*font).width + 7) / 8;
    let src = (*font).data.add(c as usize * (*font).height as usize * bytes as usize + h as usize * bytes as usize);
    for m in 0..(*font).width { let n = m % 8; let x = *src.add((m / 8) as usize); *dst.add(m as usize) = if ((x >> (7 - n)) & 1) != 0 { 0x00ffffff } else { 0 }; }
}

unsafe extern "C" fn efi_earlycon_write(con: *mut console, str_: *const i8, mut num: u32) {
    let si = &sysfb_primary_display.screen; let mut cur_efi_x = efi_x; let len = si.lfb_linelength as usize; let mut str_ = str_;
    while num != 0 {
        let linemax = (si.lfb_width as u32 - efi_x) / (*font).width;
        let mut count = strnchrnul(str_, num as usize, b'\n' as i32).offset_from(str_) as u32; if count > linemax { count = linemax; }
        for h in 0..(*font).height { let dst = efi_earlycon_map((efi_y + h) as usize * len, len); if dst.is_null() { return; } let mut x = efi_x; for j in 0..count { efi_earlycon_write_char((dst as *mut u8).add((x * 4) as usize) as *mut u32, *str_.add(j as usize) as u8, h, si); x += (*font).width; } efi_earlycon_unmap(dst, len); }
        num -= count; efi_x += count * (*font).width; str_ = str_.add(count as usize);
        if num > 0 && *str_ == b'\n' as i8 { cur_efi_x = efi_x; efi_x = 0; efi_y += (*font).height; str_ = str_.add(1); num -= 1; }
        if efi_x + (*font).width > si.lfb_width as u32 { cur_efi_x = efi_x; efi_x = 0; efi_y += (*font).height; }
        if efi_y + (*font).height > si.lfb_height as u32 { efi_x_array[cur_line_y as usize] = cur_efi_x; cur_line_y = (cur_line_y + 1) % max_line_y; efi_y -= (*font).height; efi_earlycon_scroll_up(si); for i in 0..(*font).height { efi_earlycon_clear_scanline(efi_y + i, si); } }
    }
}

static mut fb_probed: bool = false;
unsafe extern "C" fn efi_earlycon_reprobe() { if fb_probed { setup_earlycon(b"efifb\0".as_ptr() as *const i8); } }

unsafe extern "C" fn efi_earlycon_setup(device: *mut earlycon_device, opt: *const i8) -> i32 {
    let si = &sysfb_primary_display.screen; fb_wb = !opt.is_null() && strcmp(opt, b"ram\0".as_ptr() as *const i8) == 0;
    if si.orig_video_isVGA != VIDEO_TYPE_EFI { fb_probed = true; return -19; }
    fb_base = si.lfb_base as u64; if si.capabilities & VIDEO_CAPABILITY_64BIT_BASE != 0 { fb_base |= (si.ext_lfb_base as u64) << 32; }
    let xres = si.lfb_width; let yres = si.lfb_height; if si.lfb_depth != 32 { return -19; }
    font = get_default_font(xres, yres, core::ptr::null(), core::ptr::null()); if font.is_null() { return -19; }
    for x in &mut efi_x_array { *x = xres as u32 - xres as u32 % (*font).width; } efi_y = yres as u32 - yres as u32 % (*font).height; max_line_y = (efi_y / (*font).height + 1) as u16; cur_line_y = 0; efi_y -= (*font).height;
    for _ in 0..((yres as u32 - efi_y) / (*font).height) { efi_earlycon_scroll_up(si); }
    (*(*device).con).write = Some(efi_earlycon_write); earlycon_console = (*device).con; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
