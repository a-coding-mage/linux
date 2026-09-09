// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * Common all-VGA modes
 */

// Dependencies supplied by the boot and video modules are intentionally
// referenced here rather than reimplemented.

static mut VGA_MODES: [mode_info; 7] = [
    mode_info { mode: VIDEO_80x25, x: 80, y: 25, depth: 0 },
    mode_info { mode: VIDEO_8POINT, x: 80, y: 50, depth: 0 },
    mode_info { mode: VIDEO_80x43, x: 80, y: 43, depth: 0 },
    mode_info { mode: VIDEO_80x28, x: 80, y: 28, depth: 0 },
    mode_info { mode: VIDEO_80x30, x: 80, y: 30, depth: 0 },
    mode_info { mode: VIDEO_80x34, x: 80, y: 34, depth: 0 },
    mode_info { mode: VIDEO_80x60, x: 80, y: 60, depth: 0 },
];

static mut EGA_MODES: [mode_info; 2] = [
    mode_info { mode: VIDEO_80x25, x: 80, y: 25, depth: 0 },
    mode_info { mode: VIDEO_8POINT, x: 80, y: 43, depth: 0 },
];

static mut CGA_MODES: [mode_info; 1] = [
    mode_info { mode: VIDEO_80x25, x: 80, y: 25, depth: 0 },
];

unsafe fn vga_set_basic_mode() -> u8 {
    let mut ireg = biosregs::default();
    let mut oreg = biosregs::default();
    let mut mode: u8;

    initregs(&mut ireg);

    /* Query current mode */
    ireg.ax = 0x0f00;
    intcall(0x10, &mut ireg, &mut oreg);
    mode = oreg.al;

    if mode != 3 && mode != 7 { mode = 3; }

    /* Set the mode */
    ireg.ax = mode; /* AH=0: set mode */
    intcall(0x10, &mut ireg, core::ptr::null_mut());
    do_restore = 1;
    mode
}

unsafe fn vga_set_8font() {
    /* Set 8x8 font - 80x43 on EGA, 80x50 on VGA */
    let mut ireg = biosregs::default();
    initregs(&mut ireg);

    /* Set 8x8 font */
    ireg.ax = 0x1112;
    /* ireg.bl = 0; */
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Use alternate print screen */
    ireg.ax = 0x1200;
    ireg.bl = 0x20;
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Turn off cursor emulation */
    ireg.ax = 0x1201;
    ireg.bl = 0x34;
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Cursor is scan lines 6-7 */
    ireg.ax = 0x0100;
    ireg.cx = 0x0607;
    intcall(0x10, &mut ireg, core::ptr::null_mut());
}

unsafe fn vga_set_14font() {
    /* Set 9x14 font - 80x28 on VGA */
    let mut ireg = biosregs::default();
    initregs(&mut ireg);

    /* Set 9x14 font */
    ireg.ax = 0x1111;
    /* ireg.bl = 0; */
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Turn off cursor emulation */
    ireg.ax = 0x1201;
    ireg.bl = 0x34;
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Cursor is scan lines 11-12 */
    ireg.ax = 0x0100;
    ireg.cx = 0x0b0c;
    intcall(0x10, &mut ireg, core::ptr::null_mut());
}

unsafe fn vga_set_80x43() {
    /* Set 80x43 mode on VGA (not EGA) */
    let mut ireg = biosregs::default();
    initregs(&mut ireg);

    /* Set 350 scans */
    ireg.ax = 0x1201;
    ireg.bl = 0x30;
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    /* Reset video mode */
    ireg.ax = 0x0003;
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    vga_set_8font();
}

/* I/O address of the VGA CRTC */
pub unsafe fn vga_crtc() -> u16 {
    if inb(0x3cc) & 1 != 0 { 0x3d4 } else { 0x3b4 }
}

unsafe fn vga_set_480_scanlines() {
    let crtc = vga_crtc();
    let mut csel: u8;

    out_idx(0x0c, crtc, 0x11); /* Vertical sync end, unlock CR0-7 */
    out_idx(0x0b, crtc, 0x06); /* Vertical total */
    out_idx(0x3e, crtc, 0x07); /* Vertical overflow */
    out_idx(0xea, crtc, 0x10); /* Vertical sync start */
    out_idx(0xdf, crtc, 0x12); /* Vertical display end */
    out_idx(0xe7, crtc, 0x15); /* Vertical blank start */
    out_idx(0x04, crtc, 0x16); /* Vertical blank end */
    csel = inb(0x3cc);
    csel &= 0x0d;
    csel |= 0xe2;
    outb(csel, 0x3c2);
}

unsafe fn vga_set_vertical_end(lines: i32) {
    let crtc = vga_crtc();
    let end = lines - 1;
    let ovfw = 0x3c | ((end >> (8 - 1)) & 0x02) | ((end >> (9 - 6)) & 0x40);

    out_idx(ovfw as u8, crtc, 0x07); /* Vertical overflow */
    out_idx(end as u8, crtc, 0x12); /* Vertical display end */
}

unsafe fn vga_set_80x30() {
    vga_set_480_scanlines();
    vga_set_vertical_end(30 * 16);
}

unsafe fn vga_set_80x34() {
    vga_set_480_scanlines();
    vga_set_14font();
    vga_set_vertical_end(34 * 14);
}

unsafe fn vga_set_80x60() {
    vga_set_480_scanlines();
    vga_set_8font();
    vga_set_vertical_end(60 * 8);
}

unsafe fn vga_set_mode(mode: *mut mode_info) -> i32 {
    /* Set the basic mode */
    vga_set_basic_mode();

    /* Override a possibly broken BIOS */
    force_x = (*mode).x;
    force_y = (*mode).y;

    match (*mode).mode {
        VIDEO_80x25 => {},
        VIDEO_8POINT => vga_set_8font(),
        VIDEO_80x43 => vga_set_80x43(),
        VIDEO_80x28 => vga_set_14font(),
        VIDEO_80x30 => vga_set_80x30(),
        VIDEO_80x34 => vga_set_80x34(),
        VIDEO_80x60 => vga_set_80x60(),
        _ => {},
    }
    0
}

/*
 * Note: this probe includes basic information required by all
 * systems.  It should be executed first, by making sure
 * video-vga.c is listed first in the Makefile.
 */
unsafe fn vga_probe() -> i32 {
    let card_name: [*const u8; 3] = [b"CGA/MDA/HGC\0".as_ptr(), b"EGA\0".as_ptr(), b"VGA\0".as_ptr()];
    let mode_lists: [*mut mode_info; 3] = [CGA_MODES.as_mut_ptr(), EGA_MODES.as_mut_ptr(), VGA_MODES.as_mut_ptr()];
    let mode_count: [i32; 3] = [1, 2, 7];
    let mut ireg = biosregs::default();
    let mut oreg = biosregs::default();

    initregs(&mut ireg);
    ireg.ax = 0x1200;
    ireg.bl = 0x10; /* Check EGA/VGA */
    intcall(0x10, &mut ireg, &mut oreg);

    /* _WAKEUP builds omit this boot parameter update. */
    #[cfg(not(_WAKEUP))]
    { boot_params.screen_info.orig_video_ega_bx = oreg.bx; }

    /* If we have MDA/CGA/HGC then BL will be unchanged at 0x10 */
    if oreg.bl != 0x10 {
        /* EGA/VGA */
        ireg.ax = 0x1a00;
        intcall(0x10, &mut ireg, &mut oreg);

        if oreg.al == 0x1a {
            adapter = ADAPTER_VGA;
            #[cfg(not(_WAKEUP))]
            { boot_params.screen_info.orig_video_isVGA = 1; }
        } else {
            adapter = ADAPTER_EGA;
        }
    } else {
        adapter = ADAPTER_CGA;
    }

    video_vga.modes = mode_lists[adapter as usize];
    video_vga.card_name = card_name[adapter as usize];
    mode_count[adapter as usize]
}

static mut video_vga: __videocard = __videocard {
    card_name: b"VGA\0".as_ptr(),
    probe: Some(vga_probe),
    set_mode: Some(vga_set_mode),
    ..__videocard::zeroed()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
