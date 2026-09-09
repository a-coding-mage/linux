// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * Standard video BIOS modes
 *
 * We have two options for this; silent and scanned.
 */

// Dependencies supplied by the surrounding boot/video implementation.

/* Set a conventional BIOS mode */
unsafe fn bios_set_mode(mi: *mut mode_info) -> i32 {
    set_bios_mode((*mi).mode.wrapping_sub(VIDEO_FIRST_BIOS))
}

unsafe fn set_bios_mode(mode: u8) -> i32 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    let new_mode: u8;

    initregs(&mut ireg);
    (*(&mut ireg)).al = mode; /* AH=0x00 Set Video Mode */
    intcall(0x10, &mut ireg, core::ptr::null_mut());

    ireg.ah = 0x0f; /* Get Current Video Mode */
    intcall(0x10, &mut ireg, &mut oreg);

    do_restore = 1; /* Assume video contents were lost */

    /* Not all BIOSes are clean with the top bit */
    new_mode = oreg.al & 0x7f;

    if new_mode == mode {
        return 0; /* Mode change OK */
    }

    // The C condition is excluded when _WAKEUP is defined.
    if new_mode != boot_params.screen_info.orig_video_mode {
        /* Mode setting failed, but we didn't end up where we
           started.  That's bad.  Try to revert to the original
           video mode. */
        ireg.ax = boot_params.screen_info.orig_video_mode as _;
        intcall(0x10, &mut ireg, core::ptr::null_mut());
    }

    -1
}

unsafe fn bios_probe() -> i32 {
    let mut mode: u8;
    // When _WAKEUP is defined, saved_mode is initialized to 0x03.
    let saved_mode: u8 = boot_params.screen_info.orig_video_mode;
    let crtc: u16;
    let mut mi: *mut mode_info;
    let mut nmodes: i32 = 0;

    if adapter != ADAPTER_EGA && adapter != ADAPTER_VGA {
        return 0;
    }

    set_fs(0);
    crtc = vga_crtc();

    video_bios.modes = GET_HEAP::<mode_info>(0);

    mode = 0x14;
    while mode <= 0x7f {
        if !heap_free(core::mem::size_of::<mode_info>()) {
            break;
        }

        if mode_defined(VIDEO_FIRST_BIOS.wrapping_add(mode)) {
            mode = mode.wrapping_add(1);
            continue;
        }

        if set_bios_mode(mode) != 0 {
            mode = mode.wrapping_add(1);
            continue;
        }

        /* Try to verify that it's a text mode. */

        /* Attribute Controller: make graphics controller disabled */
        if in_idx(0x3c0, 0x10) & 0x01 != 0 {
            mode = mode.wrapping_add(1);
            continue;
        }

        /* Graphics Controller: verify Alpha addressing enabled */
        if in_idx(0x3ce, 0x06) & 0x01 != 0 {
            mode = mode.wrapping_add(1);
            continue;
        }

        /* CRTC cursor location low should be zero(?) */
        if in_idx(crtc, 0x0f) != 0 {
            mode = mode.wrapping_add(1);
            continue;
        }

        mi = GET_HEAP::<mode_info>(1);
        (*mi).mode = VIDEO_FIRST_BIOS.wrapping_add(mode);
        (*mi).depth = 0; /* text */
        (*mi).x = rdfs16(0x44a);
        (*mi).y = rdfs8(0x484).wrapping_add(1);
        nmodes += 1;

        mode = mode.wrapping_add(1);
    }

    set_bios_mode(saved_mode);

    nmodes
}

static mut video_bios: __videocard = __videocard {
    card_name: "BIOS",
    probe: Some(bios_probe),
    set_mode: Some(bios_set_mode),
    unsafe_: 1,
    xmode_first: VIDEO_FIRST_BIOS,
    xmode_n: 0x80,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
