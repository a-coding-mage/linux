// SPDX-License-Identifier: GPL-2.0-only
/*
 * pcm_local.h - a local header file for snd-pcm module.
 *
 * Copyright (c) Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// C header guard removed: __SOUND_CORE_PCM_LOCAL_H.

unsafe extern "C" {
    pub static snd_pcm_known_rates: snd_pcm_hw_constraint_list;

    pub fn snd_interval_mul(
        a: *const snd_interval,
        b: *const snd_interval,
        c: *mut snd_interval,
    );
    pub fn snd_interval_div(
        a: *const snd_interval,
        b: *const snd_interval,
        c: *mut snd_interval,
    );
    pub fn snd_interval_muldivk(
        a: *const snd_interval,
        b: *const snd_interval,
        k: ::core::ffi::c_uint,
        c: *mut snd_interval,
    );
    pub fn snd_interval_mulkdiv(
        a: *const snd_interval,
        k: ::core::ffi::c_uint,
        b: *const snd_interval,
        c: *mut snd_interval,
    );

    pub fn snd_pcm_hw_constraint_mask(
        runtime: *mut snd_pcm_runtime,
        var: snd_pcm_hw_param_t,
        mask: u32,
    ) -> ::core::ffi::c_int;

    pub fn pcm_lib_apply_appl_ptr(
        substream: *mut snd_pcm_substream,
        appl_ptr: snd_pcm_uframes_t,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_update_state(
        substream: *mut snd_pcm_substream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_update_hw_ptr(substream: *mut snd_pcm_substream) -> ::core::ffi::c_int;

    pub fn snd_pcm_playback_silence(
        substream: *mut snd_pcm_substream,
        new_hw_ptr: snd_pcm_uframes_t,
    );
}

#[inline]
pub unsafe fn snd_pcm_avail(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        unsafe { snd_pcm_playback_avail((*substream).runtime) }
    } else {
        unsafe { snd_pcm_capture_avail((*substream).runtime) }
    }
}

#[inline]
pub unsafe fn snd_pcm_hw_avail(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        unsafe { snd_pcm_playback_hw_avail((*substream).runtime) }
    } else {
        unsafe { snd_pcm_capture_hw_avail((*substream).runtime) }
    }
}

// Original C conditional:
// #ifdef CONFIG_SND_PCM_TIMER
//   external timer functions are declared.
// #else
//   inline no-op functions are provided.
// #endif
// The active Rust mapping cannot be selected from this isolated header alone.
#[cfg(CONFIG_SND_PCM_TIMER)]
unsafe extern "C" {
    pub fn snd_pcm_timer_resolution_change(substream: *mut snd_pcm_substream);
    pub fn snd_pcm_timer_init(substream: *mut snd_pcm_substream);
    pub fn snd_pcm_timer_done(substream: *mut snd_pcm_substream);
}

#[cfg(not(CONFIG_SND_PCM_TIMER))]
#[inline]
pub unsafe fn snd_pcm_timer_resolution_change(_substream: *mut snd_pcm_substream) {}

#[cfg(not(CONFIG_SND_PCM_TIMER))]
#[inline]
pub unsafe fn snd_pcm_timer_init(_substream: *mut snd_pcm_substream) {}

#[cfg(not(CONFIG_SND_PCM_TIMER))]
#[inline]
pub unsafe fn snd_pcm_timer_done(_substream: *mut snd_pcm_substream) {}

unsafe extern "C" {
    pub fn __snd_pcm_xrun(substream: *mut snd_pcm_substream);
    pub fn snd_pcm_group_init(group: *mut snd_pcm_group);
    pub fn snd_pcm_sync_stop(substream: *mut snd_pcm_substream, sync_irq: bool);
}

#[inline]
pub unsafe fn PCM_RUNTIME_CHECK(sub: *mut snd_pcm_substream) -> ::core::ffi::c_int {
    unsafe { snd_BUG_ON(sub.is_null() || (*sub).runtime.is_null()) }
}

/*
 * loop over all PCM substreams
 *
 * C macro:
 * #define for_each_pcm_substream(pcm, str, subs) \
 *     for ((str) = 0; (str) < 2; (str)++) \
 *         for ((subs) = (pcm)->streams[str].substream; (subs); \
 *              (subs) = (subs)->next)
 */

#[inline]
pub unsafe fn snd_pcm_dma_buffer_sync(
    substream: *mut snd_pcm_substream,
    mode: snd_dma_sync_mode,
) {
    if unsafe { (*(*substream).runtime).info & SNDRV_PCM_INFO_EXPLICIT_SYNC } != 0 {
        unsafe {
            snd_dma_buffer_sync(snd_pcm_get_dma_buf(substream), mode);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
