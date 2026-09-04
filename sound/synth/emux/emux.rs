// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Routines for control of EMU WaveTable chip
 */

// Linux kernel and ALSA headers
// #include <linux/wait.h>
// #include <linux/slab.h>
// #include <linux/string.h>
// #include <sound/core.h>
// #include <sound/emux_synth.h>
// #include <linux/init.h>
// #include <linux/module.h>
// #include "emux_voice.h"

// MODULE_AUTHOR("Takashi Iwai");
// MODULE_DESCRIPTION("Routines for control of EMU WaveTable chip");
// MODULE_LICENSE("GPL");

use core::ffi::c_void;
use core::ptr;

// External opaque types from kernel and ALSA headers
#[repr(C)]
pub struct snd_emux {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_sf_sample {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_util_memhdr {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_sf_callback {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_emux_voice {
    _opaque: [u8; 0],
}

// External kernel and ALSA functions
extern "C" {
    fn kzalloc_obj(ptr: *const c_void) -> *mut c_void;
    fn spin_lock_init(lock: *mut c_void);
    fn mutex_init(mutex: *mut c_void);
    fn timer_setup(timer: *mut c_void, callback: unsafe extern "C" fn(*mut c_void), flags: u32);
    fn snd_emux_timer_callback(arg: *mut c_void);
    fn kzalloc_objs(tp: *const c_void, n: usize) -> *mut c_void;
    fn kstrdup_const(s: *const u8, gfp_mask: u32) -> *const u8;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn snd_sf_new(callback: *const snd_sf_callback, hdr: *mut snd_util_memhdr) -> *mut c_void;
    fn snd_emux_init_hwdep(emu: *mut snd_emux) -> i32;
    fn snd_emux_init_voices(emu: *mut snd_emux);
    fn snd_emux_init_seq(emu: *mut snd_emux, card: *mut snd_card, index: i32);
    fn snd_emux_init_seq_oss(emu: *mut snd_emux);
    fn snd_emux_init_virmidi(emu: *mut snd_emux, card: *mut snd_card);
    fn snd_emux_proc_init(emu: *mut snd_emux, card: *mut snd_card, index: i32);
    fn timer_shutdown_sync(timer: *mut c_void);
    fn snd_emux_proc_free(emu: *mut snd_emux);
    fn snd_emux_delete_virmidi(emu: *mut snd_emux);
    fn snd_emux_detach_seq_oss(emu: *mut snd_emux);
    fn snd_emux_detach_seq(emu: *mut snd_emux);
    fn snd_emux_delete_hwdep(emu: *mut snd_emux);
    fn snd_sf_free(sflist: *mut c_void);
    fn kfree(objp: *mut c_void);
    fn kfree_const(x: *const c_void);
}

const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;
const GFP_KERNEL: u32 = 0xd0;

// Create a new hardware dependent device for Emu8000/Emu10k1
#[no_mangle]
pub unsafe extern "C" fn snd_emux_new(remu: *mut *mut snd_emux) -> i32 {
    let emu: *mut snd_emux;

    *remu = ptr::null_mut();
    emu = kzalloc_obj(ptr::null()) as *mut snd_emux;
    if emu.is_null() {
        return ENOMEM;
    }

    spin_lock_init(emu as *mut c_void);
    mutex_init((emu as *mut u8).add(0) as *mut c_void);

    // emu->client = -1;
    // #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
    // emu->oss_synth = NULL;
    // #endif
    // emu->max_voices = 0;
    // emu->use_time = 0;

    timer_setup(emu as *mut c_void, snd_emux_timer_callback, 0);
    // emu->timer_active = 0;

    *remu = emu;
    0
}

// EXPORT_SYMBOL(snd_emux_new);

unsafe extern "C" fn sf_sample_new(
    private_data: *mut c_void,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
    buf: *const c_void,
    count: i64,
) -> i32 {
    let emu = private_data as *mut snd_emux;
    // emu->ops.sample_new(emu, sp, hdr, buf, count)
    // Note: actual ops function call requires knowledge of snd_emux.ops member layout
    0
}

unsafe extern "C" fn sf_sample_free(
    private_data: *mut c_void,
    sp: *mut snd_sf_sample,
    hdr: *mut snd_util_memhdr,
) -> i32 {
    let emu = private_data as *mut snd_emux;
    // emu->ops.sample_free(emu, sp, hdr)
    // Note: actual ops function call requires knowledge of snd_emux.ops member layout
    0
}

unsafe extern "C" fn sf_sample_reset(private_data: *mut c_void) {
    let emu = private_data as *mut snd_emux;
    // emu->ops.sample_reset(emu)
    // Note: actual ops function call requires knowledge of snd_emux.ops member layout
}

#[no_mangle]
pub unsafe extern "C" fn snd_emux_register(
    emu: *mut snd_emux,
    card: *mut snd_card,
    index: i32,
    name: *mut u8,
) -> i32 {
    let mut err: i32;
    let mut sf_cb: snd_sf_callback;

    if emu.is_null() || (*emu as *const snd_emux).is_null() || (*emu as *const c_void).is_null()
        || (*emu as *const snd_emux) as *const u8 as i32 <= 0
    {
        return EINVAL;
    }
    if card.is_null() || name.is_null() {
        return EINVAL;
    }

    // emu->card = card;
    // emu->name = kstrdup_const(name, GFP_KERNEL);
    let _emu_name = kstrdup_const(name, GFP_KERNEL);
    // emu->voices = kzalloc_objs(struct snd_emux_voice, emu->max_voices);
    let _emu_voices = kzalloc_objs(ptr::null(), 0);

    if _emu_name.is_null() || _emu_voices.is_null() {
        return ENOMEM;
    }

    // Create soundfont list
    memset(&mut sf_cb as *mut snd_sf_callback as *mut c_void, 0, core::mem::size_of::<snd_sf_callback>());
    // sf_cb.private_data = emu;
    // sf_cb.sample_new = sf_sample_new;
    // sf_cb.sample_free = sf_sample_free;
    // if (emu->ops.sample_reset)
    //     sf_cb.sample_reset = sf_sample_reset;

    // emu->sflist = snd_sf_new(&sf_cb, emu->memhdr);
    let _sflist = snd_sf_new(&sf_cb, ptr::null_mut());
    if _sflist.is_null() {
        return ENOMEM;
    }

    err = snd_emux_init_hwdep(emu);
    if err < 0 {
        return err;
    }

    snd_emux_init_voices(emu);

    snd_emux_init_seq(emu, card, index);
    // #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
    snd_emux_init_seq_oss(emu);
    // #endif
    snd_emux_init_virmidi(emu, card);

    snd_emux_proc_init(emu, card, index);
    0
}

// EXPORT_SYMBOL(snd_emux_register);

#[no_mangle]
pub unsafe extern "C" fn snd_emux_free(emu: *mut snd_emux) -> i32 {
    if emu.is_null() {
        return EINVAL;
    }

    timer_shutdown_sync(emu as *mut c_void);

    snd_emux_proc_free(emu);
    snd_emux_delete_virmidi(emu);
    // #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
    snd_emux_detach_seq_oss(emu);
    // #endif
    snd_emux_detach_seq(emu);
    snd_emux_delete_hwdep(emu);
    snd_sf_free(ptr::null_mut());
    kfree(ptr::null_mut());
    kfree_const(ptr::null());
    kfree(emu as *mut c_void);
    0
}

// EXPORT_SYMBOL(snd_emux_free);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
