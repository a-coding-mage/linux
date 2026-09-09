/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Digital Audio (PCM) - OSS compatibility abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#[repr(C)]
pub struct snd_pcm_oss_setup {
    pub task_name: *mut ::core::ffi::c_char,
    /* C bit-fields: disable, direct, block, nonblock, partialfrag,
     * nosilence, and buggyptr, each one bit. */
    pub disable: u32,
    pub direct: u32,
    pub block: u32,
    pub nonblock: u32,
    pub partialfrag: u32,
    pub nosilence: u32,
    pub buggyptr: u32,
    pub periods: ::core::ffi::c_uint,
    pub period_size: ::core::ffi::c_uint,
    pub next: *mut snd_pcm_oss_setup,
}

#[repr(C)]
pub struct snd_pcm_oss_runtime {
    /* C bit-fields: params, prepare, trigger, and sync_trigger, each one bit. */
    pub params: u32,
    pub prepare: u32,
    pub trigger: u32,
    pub sync_trigger: u32,
    pub rate: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_uint,
    pub fragshift: ::core::ffi::c_uint,
    pub maxfrags: ::core::ffi::c_uint,
    pub subdivision: ::core::ffi::c_uint,
    pub period_bytes: usize,
    pub period_frames: usize,
    pub period_ptr: usize,
    pub periods: ::core::ffi::c_uint,
    pub buffer_bytes: usize,
    pub bytes: usize,
    pub mmap_bytes: usize,
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_used: usize,
    pub params_lock: mutex,
    pub rw_ref: atomic_t,
    /* CONFIG_SND_PCM_OSS_PLUGINS conditional declarations preserved below. */
    #[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
    pub plugin_first: *mut snd_pcm_plugin,
    #[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
    pub plugin_last: *mut snd_pcm_plugin,
    pub prev_hw_ptr_period: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_pcm_oss_file {
    pub streams: [*mut snd_pcm_substream; 2],
}

#[repr(C)]
pub struct snd_pcm_oss_substream {
    /* C bit-field: oss, one bit. */
    pub oss: u32,
    pub setup: snd_pcm_oss_setup,
}

#[repr(C)]
pub struct snd_pcm_oss_stream {
    pub setup_list: *mut snd_pcm_oss_setup,
    pub setup_mutex: mutex,
    /* CONFIG_SND_VERBOSE_PROCFS conditional declaration preserved below. */
    #[cfg(CONFIG_SND_VERBOSE_PROCFS)]
    pub proc_entry: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_pcm_oss {
    pub reg: ::core::ffi::c_int,
    pub reg_mask: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
