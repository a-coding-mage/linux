// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/* Select video mode */

// External types, globals, constants, and functions are supplied by the boot
// environment and the other translated source files.

static mut video_segment: u16 = 0;

unsafe fn store_cursor_position() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ah = 0x03;
    intcall(0x10, &ireg, &mut oreg);

    boot_params.screen_info.orig_x = oreg.dl;
    boot_params.screen_info.orig_y = oreg.dh;

    if oreg.ch & 0x20 != 0 {
        boot_params.screen_info.flags |= VIDEO_FLAGS_NOCURSOR;
    }

    if (oreg.ch & 0x1f) > (oreg.cl & 0x1f) {
        boot_params.screen_info.flags |= VIDEO_FLAGS_NOCURSOR;
    }
}

unsafe fn store_video_mode() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ah = 0x0f;
    intcall(0x10, &ireg, &mut oreg);

    boot_params.screen_info.orig_video_mode = oreg.al & 0x7f;
    boot_params.screen_info.orig_video_page = oreg.bh;
}

unsafe fn store_mode_params() {
    let mut font_size: u16;
    let mut x: i32;
    let mut y: i32;

    if graphic_mode {
        return;
    }

    store_cursor_position();
    store_video_mode();

    if boot_params.screen_info.orig_video_mode == 0x07 {
        video_segment = 0xb000;
    } else {
        video_segment = 0xb800;
    }

    set_fs(0);
    font_size = rdfs16(0x485);
    boot_params.screen_info.orig_video_points = font_size;

    x = rdfs16(0x44a) as i32;
    y = if adapter == ADAPTER_CGA { 25 } else { rdfs8(0x484) as i32 + 1 };

    if force_x != 0 { x = force_x; }
    if force_y != 0 { y = force_y; }

    boot_params.screen_info.orig_video_cols = x;
    boot_params.screen_info.orig_video_lines = y;
}

unsafe fn get_entry() -> u32 {
    let mut entry_buf = [0i8; 4];
    let mut len = 0usize;
    let mut key: i32;
    let mut v: u32;

    loop {
        key = getchar();

        if key == b'\b' as i32 {
            if len > 0 {
                puts("\b \b");
                len -= 1;
            }
        } else if (key >= b'0' as i32 && key <= b'9' as i32) ||
                  (key >= b'A' as i32 && key <= b'Z' as i32) ||
                  (key >= b'a' as i32 && key <= b'z' as i32) {
            if len < entry_buf.len() {
                entry_buf[len] = key as i8;
                len += 1;
                putchar(key);
            }
        }
        if key == b'\r' as i32 { break; }
    }
    putchar(b'\n' as i32);

    if len == 0 { return VIDEO_CURRENT_MODE; }

    v = 0;
    for i in 0..len {
        v <<= 4;
        key = entry_buf[i] as i32 | 0x20;
        v += if key > b'9' as i32 { (key - b'a' as i32 + 10) as u32 } else { (key - b'0' as i32) as u32 };
    }
    v
}

unsafe fn display_menu() {
    let mut card: *mut card_info;
    let mut mi: *mut mode_info;
    let mut ch = b'0' as i8;
    let mut nmodes = 0;
    let mut modes_per_line = 1;
    let mut col;

    card = video_cards;
    while card < video_cards_end { nmodes += (*card).nmodes; card = card.add(1); }
    if nmodes >= 20 { modes_per_line = 3; }
    for _ in 0..modes_per_line { puts("Mode: Resolution:  Type: "); }
    putchar(b'\n' as i32);

    col = 0;
    card = video_cards;
    while card < video_cards_end {
        mi = (*card).modes;
        for _ in 0..(*card).nmodes {
            let visible = (*mi).x != 0 && (*mi).y != 0;
            let mode_id = if (*mi).mode != 0 { (*mi).mode } else { ((*mi).y << 8) + (*mi).x };
            if visible {
                let mut resbuf = [0i8; 32];
                if (*mi).depth != 0 { sprintf(resbuf.as_mut_ptr(), "%dx%d", (*mi).y, (*mi).depth); }
                else { sprintf(resbuf.as_mut_ptr(), "%d", (*mi).y); }
                printf("%c %03X %4dx%-7s %-6s", ch, mode_id, (*mi).x, resbuf.as_ptr(), (*card).card_name);
                col += 1;
                if col >= modes_per_line { putchar(b'\n' as i32); col = 0; }
                if ch == b'9' as i8 { ch = b'a' as i8; }
                else if ch == b'z' as i8 || ch == b' ' as i8 { ch = b' ' as i8; }
                else { ch += 1; }
            }
            mi = mi.add(1);
        }
        card = card.add(1);
    }
    if col != 0 { putchar(b'\n' as i32); }
}

const SCAN: u32 = 0x5cae;

unsafe fn mode_menu() -> u32 {
    let mut key;
    puts("Press <ENTER> to see video modes available, <SPACE> to continue, or wait 30 sec\n");
    kbd_flush();
    loop {
        key = getchar_timeout();
        if key == b' ' as i32 || key == 0 { return VIDEO_CURRENT_MODE; }
        if key == b'\r' as i32 { break; }
        putchar(b'\a' as i32);
    }
    loop {
        display_menu();
        puts("Enter a video mode or \"scan\" to scan for additional modes: ");
        let sel = get_entry();
        if sel != SCAN { return sel; }
        probe_cards(1);
    }
}

static mut saved: saved_screen = saved_screen { x: 0, y: 0, curx: 0, cury: 0, data: core::ptr::null_mut() };

unsafe fn save_screen() {
    saved.x = boot_params.screen_info.orig_video_cols;
    saved.y = boot_params.screen_info.orig_video_lines;
    saved.curx = boot_params.screen_info.orig_x;
    saved.cury = boot_params.screen_info.orig_y;
    if !heap_free((saved.x * saved.y * core::mem::size_of::<u16>() as i32 + 512) as usize) { return; }
    saved.data = GET_HEAP::<u16>((saved.x * saved.y) as usize);
    set_fs(video_segment);
    copy_from_fs(saved.data, 0, (saved.x * saved.y * 2) as usize);
}

unsafe fn restore_screen() {
    let xs = boot_params.screen_info.orig_video_cols;
    let ys = boot_params.screen_info.orig_video_lines;
    let mut dst: addr_t = 0;
    let mut src = saved.data;
    let mut ireg: biosregs = core::mem::zeroed();
    if graphic_mode || src.is_null() { return; }
    set_fs(video_segment);
    for y in 0..ys {
        let npad;
        if y < saved.y {
            let copy = if xs < saved.x { xs } else { saved.x };
            copy_to_fs(dst, src, (copy * 2) as usize);
            dst += (copy * 2) as addr_t;
            src = src.add(saved.x as usize);
            npad = if xs < saved.x { 0 } else { xs - saved.x };
        } else { npad = xs; }
        for _ in 0..npad { core::ptr::write_volatile((video_segment as usize + dst as usize) as *mut u16, 0x0720); dst += 2; }
    }
    if saved.curx >= xs { saved.curx = xs - 1; }
    if saved.cury >= ys { saved.cury = ys - 1; }
    initregs(&mut ireg); ireg.ah = 0x02; ireg.dh = saved.cury as u8; ireg.dl = saved.curx as u8;
    intcall(0x10, &ireg, core::ptr::null_mut());
    store_cursor_position();
}

pub unsafe fn set_video() {
    let mut mode = boot_params.hdr.vid_mode;
    RESET_HEAP();
    store_mode_params();
    save_screen();
    probe_cards(0);
    loop {
        if mode == ASK_VGA { mode = mode_menu(); }
        if !set_mode(mode) { break; }
        printf("Undefined video mode number: %x\n", mode);
        mode = ASK_VGA;
    }
    boot_params.hdr.vid_mode = mode;
    vesa_store_edid();
    store_mode_params();
    if do_restore { restore_screen(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
