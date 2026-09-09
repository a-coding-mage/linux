// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007-2008 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * arch/i386/boot/video-mode.c
 *
 * Set the video mode.  This is separated out into a different
 * file in order to be shared with the ACPI wakeup code.
 */

// Dependencies supplied by the surrounding boot/video/vesa modules and uapi.

/*
 * Common variables
 */
pub static mut adapter: ::core::ffi::c_int = 0;       /* 0=CGA/MDA/HGC, 1=EGA, 2=VGA+ */
pub static mut force_x: ::core::ffi::c_int = 0;
pub static mut force_y: ::core::ffi::c_int = 0;       /* Don't query the BIOS for cols/rows */
pub static mut do_restore: ::core::ffi::c_int = 0;    /* Screen contents changed during mode flip */
pub static mut graphic_mode: ::core::ffi::c_int = 0;  /* Graphic mode with linear frame buffer */

/* Probe the video drivers and have them generate their mode lists. */
pub unsafe fn probe_cards(unsafe_: ::core::ffi::c_int) {
    static mut probed: [u8; 2] = [0; 2];
    let mut card: *mut card_info;

    if probed[unsafe_ as usize] != 0 {
        return;
    }

    probed[unsafe_ as usize] = 1;

    card = video_cards;
    while card < video_cards_end {
        if (*card).unsafe_ == unsafe_ {
            if let Some(probe) = (*card).probe {
                (*card).nmodes = probe();
            } else {
                (*card).nmodes = 0;
            }
        }
        card = card.add(1);
    }
}

/* Test if a mode is defined */
pub unsafe fn mode_defined(mode: u16) -> ::core::ffi::c_int {
    let mut card: *mut card_info;
    let mut mi: *mut mode_info;
    let mut i: ::core::ffi::c_int;

    card = video_cards;
    while card < video_cards_end {
        mi = (*card).modes;
        i = 0;
        while i < (*card).nmodes {
            if (*mi).mode == mode {
                return 1;
            }
            i += 1;
            mi = mi.add(1);
        }
        card = card.add(1);
    }

    0
}

/* Set mode (without recalc) */
unsafe fn raw_set_mode(mode: u16, real_mode: *mut u16) -> ::core::ffi::c_int {
    let mut mode = mode & !VIDEO_RECALC;
    let mut nmode: ::core::ffi::c_int = 0;
    let mut card: *mut card_info;
    let mut mi: *mut mode_info;
    let mut i: ::core::ffi::c_int;

    /* Scan for mode based on fixed ID, position, or resolution */
    card = video_cards;
    while card < video_cards_end {
        mi = (*card).modes;
        i = 0;
        while i < (*card).nmodes {
            let visible = (*mi).x != 0 || (*mi).y != 0;

            if ((mode as ::core::ffi::c_int == nmode) && visible)
                || mode == (*mi).mode
                || mode == ((*mi).y as u16).wrapping_shl(8).wrapping_add((*mi).x as u16)
            {
                *real_mode = (*mi).mode;
                return ((*card).set_mode.unwrap())(mi);
            }

            if visible {
                nmode += 1;
            }
            i += 1;
            mi = mi.add(1);
        }
        card = card.add(1);
    }

    /* Nothing found?  Is it an "exceptional" (unprobed) mode? */
    card = video_cards;
    while card < video_cards_end {
        if mode >= (*card).xmode_first
            && mode < (*card).xmode_first.wrapping_add((*card).xmode_n)
        {
            let mut mix: mode_info = ::core::mem::zeroed();
            *real_mode = mode;
            mix.mode = mode;
            mix.x = 0;
            mix.y = 0;
            return ((*card).set_mode.unwrap())(&mut mix);
        }
        card = card.add(1);
    }

    /* Otherwise, failure... */
    -1
}

/*
 * Recalculate the vertical video cutoff (hack!)
 */
unsafe fn vga_recalc_vertical() {
    let font_size: u32;
    let mut rows: u32;
    let crtc: u16;
    let mut pt: u8;
    let mut ov: u8;

    set_fs(0);
    font_size = rdfs8(0x485); /* BIOS: font size (pixels) */
    rows = if force_y != 0 { force_y as u32 } else { rdfs8(0x484) + 1 }; /* Text rows */

    rows = rows.wrapping_mul(font_size); /* Visible scan lines */
    rows = rows.wrapping_sub(1);          /* ... minus one */

    crtc = vga_crtc();

    pt = in_idx(crtc, 0x11);
    pt &= !0x80;          /* Unlock CR0-7 */
    out_idx(pt, crtc, 0x11);

    out_idx(rows as u8, crtc, 0x12); /* Lower height register */

    ov = in_idx(crtc, 0x07); /* Overflow register */
    ov &= 0xbd;
    ov |= ((rows >> (8 - 1)) & 0x02) as u8;
    ov |= ((rows >> (9 - 6)) & 0x40) as u8;
    out_idx(ov, crtc, 0x07);
}

/* Set mode (with recalc if specified) */
pub unsafe fn set_mode(mut mode: u16) -> ::core::ffi::c_int {
    let rv: ::core::ffi::c_int;
    let mut real_mode: u16 = 0;

    /* Very special mode numbers... */
    if mode == VIDEO_CURRENT_MODE {
        return 0; /* Nothing to do... */
    } else if mode == NORMAL_VGA {
        mode = VIDEO_80X25;
    } else if mode == EXTENDED_VGA {
        mode = VIDEO_8POINT;
    }

    rv = raw_set_mode(mode, &mut real_mode);
    if rv != 0 {
        return rv;
    }

    if mode & VIDEO_RECALC != 0 {
        vga_recalc_vertical();
    }

    /* Save the canonical mode number for the kernel, not
       an alias, size specification or menu position */
    // The _WAKEUP build omits this assignment.
    #[cfg(not(_WAKEUP))]
    {
        (*boot_params).hdr.vid_mode = real_mode;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
