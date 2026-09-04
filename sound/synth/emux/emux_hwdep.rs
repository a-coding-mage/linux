// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Interface for hwdep device
 *
 *  Copyright (C) 2004 Takashi Iwai <tiwai@suse.de>
 */

// Dependencies from sound/core.h, sound/hwdep.h, linux/uaccess.h, linux/nospec.h, emux_voice.h

use std::ffi::c_void;

const TMP_CLIENT_ID: u32 = 0x1001;

// load patch
unsafe fn snd_emux_hwdep_load_patch(
    emu: *mut snd_emux,
    arg: *mut c_void,
) -> i32 {
    let mut patch: soundfont_patch_info = std::mem::zeroed();

    if copy_from_user(
        &mut patch as *mut _ as *mut c_void,
        arg,
        std::mem::size_of::<soundfont_patch_info>(),
    ) != 0
    {
        return -14; // -EFAULT
    }

    if patch.key == GUS_PATCH {
        return snd_soundfont_load_guspatch(
            (*emu).card,
            (*emu).sflist,
            arg,
            patch.len + std::mem::size_of::<soundfont_patch_info>(),
        );
    }

    if patch.type_field >= SNDRV_SFNT_LOAD_INFO && patch.type_field <= SNDRV_SFNT_PROBE_DATA {
        let err = snd_soundfont_load(
            (*emu).card,
            (*emu).sflist,
            arg,
            patch.len + std::mem::size_of::<soundfont_patch_info>(),
            TMP_CLIENT_ID,
        );
        if err < 0 {
            return err;
        }
    } else {
        if !(*emu).ops.load_fx.is_null() {
            return (*emu).ops.load_fx(
                emu,
                patch.type_field,
                patch.optarg,
                arg,
                patch.len + std::mem::size_of::<soundfont_patch_info>(),
            );
        } else {
            return -22; // -EINVAL
        }
    }
    0
}

// set misc mode
unsafe fn snd_emux_hwdep_misc_mode(
    emu: *mut snd_emux,
    arg: *mut c_void,
) -> i32 {
    let mut info: snd_emux_misc_mode = std::mem::zeroed();

    if copy_from_user(
        &mut info as *mut _ as *mut c_void,
        arg,
        std::mem::size_of::<snd_emux_misc_mode>(),
    ) != 0
    {
        return -14; // -EFAULT
    }
    if info.mode < 0 || info.mode >= EMUX_MD_END as i32 {
        return -22; // -EINVAL
    }
    info.mode = array_index_nospec(info.mode as usize, EMUX_MD_END as usize) as i32;

    if info.port < 0 {
        for i in 0..(*emu).num_ports {
            (*(*emu).portptrs[i as usize]).ctrls[info.mode as usize] = info.value;
        }
    } else {
        if (info.port as usize) < (*emu).num_ports {
            info.port = array_index_nospec(info.port as usize, (*emu).num_ports) as i32;
            (*(*emu).portptrs[info.port as usize]).ctrls[info.mode as usize] = info.value;
        }
    }
    0
}

// ioctl
unsafe fn snd_emux_hwdep_ioctl(
    hw: *mut snd_hwdep,
    file: *mut std::ffi::c_void,
    cmd: u32,
    arg: usize,
) -> i32 {
    let emu = (*hw).private_data as *mut snd_emux;

    match cmd {
        SNDRV_EMUX_IOCTL_VERSION => {
            put_user(
                SNDRV_EMUX_VERSION as u32,
                arg as *mut u32,
            )
        }
        SNDRV_EMUX_IOCTL_LOAD_PATCH => {
            snd_emux_hwdep_load_patch(emu, arg as *mut c_void)
        }
        SNDRV_EMUX_IOCTL_RESET_SAMPLES => {
            snd_soundfont_remove_samples((*emu).sflist);
            0
        }
        SNDRV_EMUX_IOCTL_REMOVE_LAST_SAMPLES => {
            snd_soundfont_remove_unlocked((*emu).sflist);
            0
        }
        SNDRV_EMUX_IOCTL_MEM_AVAIL => {
            if !(*emu).memhdr.is_null() {
                let size = snd_util_mem_avail((*emu).memhdr);
                put_user(size as u32, arg as *mut u32)
            } else {
                0
            }
        }
        SNDRV_EMUX_IOCTL_MISC_MODE => {
            snd_emux_hwdep_misc_mode(emu, arg as *mut c_void)
        }
        _ => 0,
    }
}

// register hwdep device
pub unsafe fn snd_emux_init_hwdep(emu: *mut snd_emux) -> i32 {
    let mut hw: *mut snd_hwdep = std::ptr::null_mut();

    let err = snd_hwdep_new(
        (*emu).card,
        SNDRV_EMUX_HWDEP_NAME.as_ptr(),
        (*emu).hwdep_idx,
        &mut hw,
    );
    if err < 0 {
        return err;
    }
    (*emu).hwdep = hw;
    strscpy((*hw).name.as_mut_ptr(), SNDRV_EMUX_HWDEP_NAME.as_ptr(), std::mem::size_of_val(&(*hw).name));
    (*hw).iface = SNDRV_HWDEP_IFACE_EMUX_WAVETABLE;
    (*hw).ops.ioctl = Some(snd_emux_hwdep_ioctl);
    // The ioctl parameter types are compatible between 32- and
    // 64-bit architectures, so use the same function.
    (*hw).ops.ioctl_compat = Some(snd_emux_hwdep_ioctl);
    (*hw).exclusive = 1;
    (*hw).private_data = emu as *mut c_void;
    let err = snd_card_register((*emu).card);
    if err < 0 {
        return err;
    }

    0
}

// unregister
pub unsafe fn snd_emux_delete_hwdep(emu: *mut snd_emux) {
    if !(*emu).hwdep.is_null() {
        snd_device_free((*emu).card, (*emu).hwdep);
        (*emu).hwdep = std::ptr::null_mut();
    }
}

// External declarations (from included headers and other modules)
extern "C" {
    type snd_emux;
    type soundfont_patch_info;
    type snd_emux_misc_mode;
    type snd_hwdep;

    static SNDRV_EMUX_HWDEP_NAME: [u8; 0];
    static SNDRV_EMUX_VERSION: u32;

    static GUS_PATCH: u32;
    static SNDRV_SFNT_LOAD_INFO: u32;
    static SNDRV_SFNT_PROBE_DATA: u32;
    static EMUX_MD_END: usize;

    static SNDRV_EMUX_IOCTL_VERSION: u32;
    static SNDRV_EMUX_IOCTL_LOAD_PATCH: u32;
    static SNDRV_EMUX_IOCTL_RESET_SAMPLES: u32;
    static SNDRV_EMUX_IOCTL_REMOVE_LAST_SAMPLES: u32;
    static SNDRV_EMUX_IOCTL_MEM_AVAIL: u32;
    static SNDRV_EMUX_IOCTL_MISC_MODE: u32;
    static SNDRV_HWDEP_IFACE_EMUX_WAVETABLE: u32;

    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn put_user(val: u32, ptr: *mut u32) -> i32;
    fn array_index_nospec(index: usize, size: usize) -> usize;

    fn snd_soundfont_load_guspatch(
        card: *mut c_void,
        sflist: *mut c_void,
        data: *mut c_void,
        len: usize,
    ) -> i32;
    fn snd_soundfont_load(
        card: *mut c_void,
        sflist: *mut c_void,
        data: *mut c_void,
        len: usize,
        client: u32,
    ) -> i32;
    fn snd_soundfont_remove_samples(sflist: *mut c_void);
    fn snd_soundfont_remove_unlocked(sflist: *mut c_void);
    fn snd_util_mem_avail(memhdr: *mut c_void) -> usize;
    fn snd_hwdep_new(
        card: *mut c_void,
        id: *const u8,
        device: i32,
        rhw: *mut *mut snd_hwdep,
    ) -> i32;
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> usize;
    fn snd_card_register(card: *mut c_void) -> i32;
    fn snd_device_free(card: *mut c_void, device: *mut c_void) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
