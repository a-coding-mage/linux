// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/* VESA text modes */

/* Dependencies are supplied by the surrounding bootloader translation. */

static mut vginfo: vesa_general_info = unsafe { core::mem::zeroed() };
static mut vminfo: vesa_mode_info = unsafe { core::mem::zeroed() };

static mut video_vesa: __videocard = __videocard {
    card_name: "VESA",
    probe: vesa_probe,
    set_mode: vesa_set_mode,
    xmode_first: VIDEO_FIRST_VESA,
    xmode_n: 0x200,
};

#[cfg(not(_WAKEUP))]
unsafe fn vesa_store_mode_params_graphics();

#[cfg(_WAKEUP)]
#[inline]
unsafe fn vesa_store_mode_params_graphics() {}

unsafe fn vesa_probe() -> i32 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    let mut mode: u16;
    let mut mode_ptr: addr_t;
    let mut mi: *mut mode_info;
    let mut nmodes: i32 = 0;

    video_vesa.modes = GET_HEAP::<mode_info>(0);

    initregs(&mut ireg);
    ireg.ax = 0x4f00;
    ireg.di = &raw mut vginfo as usize;
    intcall(0x10, &mut ireg, &mut oreg);

    if oreg.ax != 0x004f || vginfo.signature != VESA_MAGIC || vginfo.version < 0x0102 {
        return 0;
    }

    set_fs(vginfo.video_mode_ptr.seg);
    mode_ptr = vginfo.video_mode_ptr.off;

    loop {
        mode = rdfs16(mode_ptr);
        if mode == 0xffff {
            break;
        }
        mode_ptr = mode_ptr.wrapping_add(2);

        if !heap_free(core::mem::size_of::<mode_info>()) {
            break;
        }
        if (mode & !0x1ff) != 0 {
            continue;
        }

        memset(&raw mut vminfo as *mut _, 0, core::mem::size_of::<vesa_mode_info>());

        ireg.ax = 0x4f01;
        ireg.cx = mode;
        ireg.di = &raw mut vminfo as usize;
        intcall(0x10, &mut ireg, &mut oreg);

        if oreg.ax != 0x004f {
            continue;
        }

        if (vminfo.mode_attr & 0x15) == 0x05 {
            mi = GET_HEAP::<mode_info>(1);
            (*mi).mode = mode.wrapping_add(VIDEO_FIRST_VESA);
            (*mi).depth = 0;
            (*mi).x = vminfo.h_res;
            (*mi).y = vminfo.v_res;
            nmodes += 1;
        } else if (vminfo.mode_attr & 0x99) == 0x99
            && (vminfo.memory_layout == 4 || vminfo.memory_layout == 6)
            && vminfo.memory_planes == 1
        {
            // CONFIG_BOOT_VESA_SUPPORT: graphics modes are registered only when enabled.
            #[cfg(CONFIG_BOOT_VESA_SUPPORT)]
            {
                mi = GET_HEAP::<mode_info>(1);
                (*mi).mode = mode.wrapping_add(VIDEO_FIRST_VESA);
                (*mi).depth = vminfo.bpp;
                (*mi).x = vminfo.h_res;
                (*mi).y = vminfo.v_res;
                nmodes += 1;
            }
        }
    }

    nmodes
}

unsafe fn vesa_set_mode(mode: *mut mode_info) -> i32 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    let is_graphic: i32;
    let mut vesa_mode: u16 = (*mode).mode.wrapping_sub(VIDEO_FIRST_VESA);

    memset(&raw mut vminfo as *mut _, 0, core::mem::size_of::<vesa_mode_info>());

    initregs(&mut ireg);
    ireg.ax = 0x4f01;
    ireg.cx = vesa_mode;
    ireg.di = &raw mut vminfo as usize;
    intcall(0x10, &mut ireg, &mut oreg);
    if oreg.ax != 0x004f {
        return -1;
    }

    if (vminfo.mode_attr & 0x15) == 0x05 {
        is_graphic = 0;
    } else {
        #[cfg(CONFIG_BOOT_VESA_SUPPORT)]
        if (vminfo.mode_attr & 0x99) == 0x99 {
            is_graphic = 1;
            vesa_mode |= 0x4000;
        } else {
            return -1;
        }
        #[cfg(not(CONFIG_BOOT_VESA_SUPPORT))]
        {
            return -1;
        }
    }

    initregs(&mut ireg);
    ireg.ax = 0x4f02;
    ireg.bx = vesa_mode;
    intcall(0x10, &mut ireg, &mut oreg);
    if oreg.ax != 0x004f {
        return -1;
    }

    graphic_mode = is_graphic;
    if is_graphic == 0 {
        force_x = (*mode).x;
        force_y = (*mode).y;
        do_restore = 1;
    } else {
        vesa_store_mode_params_graphics();
    }
    0
}

#[cfg(not(_WAKEUP))]
unsafe fn vesa_dac_set_8bits() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    let mut dac_size: u8 = 6;

    if vginfo.capabilities & 1 != 0 {
        initregs(&mut ireg);
        ireg.ax = 0x4f08;
        ireg.bh = 0x08;
        intcall(0x10, &mut ireg, &mut oreg);
        if oreg.ax == 0x004f {
            dac_size = oreg.bh;
        }
    }

    boot_params.screen_info.red_size = dac_size;
    boot_params.screen_info.green_size = dac_size;
    boot_params.screen_info.blue_size = dac_size;
    boot_params.screen_info.rsvd_size = dac_size;
    boot_params.screen_info.red_pos = 0;
    boot_params.screen_info.green_pos = 0;
    boot_params.screen_info.blue_pos = 0;
    boot_params.screen_info.rsvd_pos = 0;
}

#[cfg(not(_WAKEUP))]
unsafe fn vesa_store_pm_info() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();
    initregs(&mut ireg);
    ireg.ax = 0x4f0a;
    intcall(0x10, &mut ireg, &mut oreg);
    if oreg.ax == 0x004f {
        boot_params.screen_info.vesapm_seg = oreg.es;
        boot_params.screen_info.vesapm_off = oreg.di;
    }
}

#[cfg(not(_WAKEUP))]
unsafe fn vesa_store_mode_params_graphics() {
    boot_params.screen_info.orig_video_isVGA = VIDEO_TYPE_VLFB;
    boot_params.screen_info.vesa_attributes = vminfo.mode_attr;
    boot_params.screen_info.lfb_linelength = vminfo.logical_scan;
    boot_params.screen_info.lfb_width = vminfo.h_res;
    boot_params.screen_info.lfb_height = vminfo.v_res;
    boot_params.screen_info.lfb_depth = vminfo.bpp;
    boot_params.screen_info.pages = vminfo.image_planes;
    boot_params.screen_info.lfb_base = vminfo.lfb_ptr;
    memcpy(
        &mut boot_params.screen_info.red_size as *mut _ as *mut u8,
        &vminfo.rmask as *const _ as *const u8,
        8,
    );
    boot_params.screen_info.lfb_size = vginfo.total_memory;
    if vminfo.bpp <= 8 {
        vesa_dac_set_8bits();
    }
    vesa_store_pm_info();
}

/* Save EDID information for the kernel; this is invoked, separately, after mode-setting. */
unsafe fn vesa_store_edid() {
    #[cfg(CONFIG_FIRMWARE_EDID)]
    {
        let mut ireg: biosregs = core::mem::zeroed();
        let mut oreg: biosregs = core::mem::zeroed();
        memset(&mut boot_params.edid_info as *mut _, 0x13, core::mem::size_of_val(&boot_params.edid_info));
        if vginfo.version < 0x0200 {
            return;
        }
        initregs(&mut ireg);
        ireg.ax = 0x4f15;
        ireg.es = 0;
        intcall(0x10, &mut ireg, &mut oreg);
        if oreg.ax != 0x004f {
            return;
        }
        ireg.ax = 0x4f15;
        ireg.bx = 0x0001;
        ireg.es = ds();
        ireg.di = &mut boot_params.edid_info as *mut _ as usize;
        intcall(0x10, &mut ireg, &mut oreg);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
