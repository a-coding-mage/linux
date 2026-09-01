/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Digital Audio (Plugin interface) abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C header dependencies intentionally omitted:
// snd_pcm_uframes_t, snd_pcm_sframes_t, snd_pcm_format_t, snd_pcm_access_t,
// snd_pcm_substream, snd_pcm_hw_params, snd_mask, ROUTE_PLUGIN_RESOLUTION,
// size_t, and pr_debug are supplied by surrounding translated code.

// Original C condition: #ifdef CONFIG_SND_PCM_OSS_PLUGINS
#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
pub unsafe fn snd_pcm_plug_stream(plug: *mut snd_pcm_plugin) -> ::core::ffi::c_int {
    unsafe { (*plug).stream }
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_pcm_plugin_action {
    INIT = 0,
    PREPARE = 1,
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
#[repr(C)]
pub struct snd_pcm_channel_area {
    pub addr: *mut ::core::ffi::c_void, /* base address of channel samples */
    pub first: ::core::ffi::c_uint,     /* offset to first sample in bits */
    pub step: ::core::ffi::c_uint,      /* samples distance in bits */
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
#[repr(C)]
pub struct snd_pcm_plugin_channel {
    pub aptr: *mut ::core::ffi::c_void, /* pointer to the allocated area */
    pub area: snd_pcm_channel_area,
    pub frames: snd_pcm_uframes_t,      /* allocated frames */
    // C bitfields:
    // unsigned int enabled:1;          /* channel need to be processed */
    // unsigned int wanted:1;           /* channel is wanted */
    pub enabled_wanted: ::core::ffi::c_uint,
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
impl snd_pcm_plugin_channel {
    pub const ENABLED_MASK: ::core::ffi::c_uint = 1 << 0;
    pub const WANTED_MASK: ::core::ffi::c_uint = 1 << 1;
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
#[repr(C)]
pub struct snd_pcm_plugin_format {
    pub format: snd_pcm_format_t,
    pub rate: ::core::ffi::c_uint,
    pub channels: ::core::ffi::c_uint,
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
#[repr(C)]
pub struct snd_pcm_plugin {
    pub name: *const ::core::ffi::c_char, /* plug-in name */
    pub stream: ::core::ffi::c_int,
    pub src_format: snd_pcm_plugin_format, /* source format */
    pub dst_format: snd_pcm_plugin_format, /* destination format */
    pub src_width: ::core::ffi::c_int,      /* sample width in bits */
    pub dst_width: ::core::ffi::c_int,      /* sample width in bits */
    pub access: snd_pcm_access_t,
    pub src_frames: Option<
        unsafe extern "C" fn(
            plugin: *mut snd_pcm_plugin,
            dst_frames: snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
    pub dst_frames: Option<
        unsafe extern "C" fn(
            plugin: *mut snd_pcm_plugin,
            src_frames: snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
    pub client_channels: Option<
        unsafe extern "C" fn(
            plugin: *mut snd_pcm_plugin,
            frames: snd_pcm_uframes_t,
            channels: *mut *mut snd_pcm_plugin_channel,
        ) -> snd_pcm_sframes_t,
    >,
    pub transfer: Option<
        unsafe extern "C" fn(
            plugin: *mut snd_pcm_plugin,
            src_channels: *const snd_pcm_plugin_channel,
            dst_channels: *mut snd_pcm_plugin_channel,
            frames: snd_pcm_uframes_t,
        ) -> snd_pcm_sframes_t,
    >,
    pub action: Option<
        unsafe extern "C" fn(
            plugin: *mut snd_pcm_plugin,
            action: snd_pcm_plugin_action,
            data: ::core::ffi::c_ulong,
        ) -> ::core::ffi::c_int,
    >,
    pub prev: *mut snd_pcm_plugin,
    pub next: *mut snd_pcm_plugin,
    pub plug: *mut snd_pcm_substream,
    pub private_data: *mut ::core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(plugin: *mut snd_pcm_plugin)>,
    pub buf: *mut ::core::ffi::c_char,
    pub buf_frames: snd_pcm_uframes_t,
    pub buf_channels: *mut snd_pcm_plugin_channel,
    pub extra_data: [::core::ffi::c_char; 0],
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
unsafe extern "C" {
    pub fn snd_pcm_plugin_build(
        handle: *mut snd_pcm_substream,
        name: *const ::core::ffi::c_char,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        extra: size_t,
        ret: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_free(plugin: *mut snd_pcm_plugin) -> ::core::ffi::c_int;
    pub fn snd_pcm_plug_alloc(
        plug: *mut snd_pcm_substream,
        frames: snd_pcm_uframes_t,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plug_client_size(
        handle: *mut snd_pcm_substream,
        drv_size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_plug_slave_size(
        handle: *mut snd_pcm_substream,
        clt_size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
}

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
pub const FULL: _ = ROUTE_PLUGIN_RESOLUTION;
#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
pub const HALF: _ = ROUTE_PLUGIN_RESOLUTION / 2;

#[cfg(CONFIG_SND_PCM_OSS_PLUGINS)]
unsafe extern "C" {
    pub fn snd_pcm_plugin_build_io(
        handle: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_build_linear(
        handle: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_build_mulaw(
        handle: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_build_rate(
        handle: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_build_route(
        handle: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plugin_build_copy(
        handle: *mut snd_pcm_substream,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        r_plugin: *mut *mut snd_pcm_plugin,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plug_format_plugins(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_params: *mut snd_pcm_hw_params,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_plug_slave_format(
        format: snd_pcm_format_t,
        format_mask: *const snd_mask,
    ) -> snd_pcm_format_t;
    pub fn snd_pcm_plugin_append(plugin: *mut snd_pcm_plugin) -> ::core::ffi::c_int;
    pub fn snd_pcm_plug_write_transfer(
        handle: *mut snd_pcm_substream,
        src_channels: *mut snd_pcm_plugin_channel,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_plug_read_transfer(
        handle: *mut snd_pcm_substream,
        dst_channels_final: *mut snd_pcm_plugin_channel,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_plug_client_channels_buf(
        handle: *mut snd_pcm_substream,
        buf: *mut ::core::ffi::c_char,
        count: snd_pcm_uframes_t,
        channels: *mut *mut snd_pcm_plugin_channel,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_plugin_client_channels(
        plugin: *mut snd_pcm_plugin,
        frames: snd_pcm_uframes_t,
        channels: *mut *mut snd_pcm_plugin_channel,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_area_silence(
        dst_channel: *const snd_pcm_channel_area,
        dst_offset: size_t,
        samples: size_t,
        format: snd_pcm_format_t,
    ) -> ::core::ffi::c_int;
    pub fn snd_pcm_area_copy(
        src_channel: *const snd_pcm_channel_area,
        src_offset: size_t,
        dst_channel: *const snd_pcm_channel_area,
        dst_offset: size_t,
        samples: size_t,
        format: snd_pcm_format_t,
    ) -> ::core::ffi::c_int;
}

// Original C condition: #else of #ifdef CONFIG_SND_PCM_OSS_PLUGINS
#[cfg(not(CONFIG_SND_PCM_OSS_PLUGINS))]
pub unsafe fn snd_pcm_plug_client_size(
    _handle: *mut snd_pcm_substream,
    drv_size: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    drv_size as snd_pcm_sframes_t
}

#[cfg(not(CONFIG_SND_PCM_OSS_PLUGINS))]
pub unsafe fn snd_pcm_plug_slave_size(
    _handle: *mut snd_pcm_substream,
    clt_size: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    clt_size as snd_pcm_sframes_t
}

#[cfg(not(CONFIG_SND_PCM_OSS_PLUGINS))]
pub unsafe fn snd_pcm_plug_slave_format(
    format: ::core::ffi::c_int,
    _format_mask: *const snd_mask,
) -> ::core::ffi::c_int {
    format
}

unsafe extern "C" {
    pub fn snd_pcm_oss_write3(
        substream: *mut snd_pcm_substream,
        ptr: *const ::core::ffi::c_char,
        size: snd_pcm_uframes_t,
        in_kernel: ::core::ffi::c_int,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_oss_read3(
        substream: *mut snd_pcm_substream,
        ptr: *mut ::core::ffi::c_char,
        size: snd_pcm_uframes_t,
        in_kernel: ::core::ffi::c_int,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_oss_writev3(
        substream: *mut snd_pcm_substream,
        bufs: *mut *mut ::core::ffi::c_void,
        frames: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    pub fn snd_pcm_oss_readv3(
        substream: *mut snd_pcm_substream,
        bufs: *mut *mut ::core::ffi::c_void,
        frames: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
}

// Original C condition:
// #ifdef PLUGIN_DEBUG
// #define pdprintf(fmt, args...) pr_debug("plugin: " fmt, ##args)
// #else
// #define pdprintf(fmt, args...)
// #endif
#[cfg(PLUGIN_DEBUG)]
macro_rules! pdprintf {
    ($fmt:literal $(, $args:expr)* $(,)?) => {
        pr_debug!(concat!("plugin: ", $fmt) $(, $args)*)
    };
}

#[cfg(not(PLUGIN_DEBUG))]
macro_rules! pdprintf {
    ($fmt:literal $(, $args:expr)* $(,)?) => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
