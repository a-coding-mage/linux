// SPDX-License-Identifier: LGPL-2.0+
/*
 *  Rate conversion Plug-In
 *  Copyright (c) 1999 by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies: <linux/time.h>, <sound/core.h>, <sound/pcm.h>, "pcm_plugin.h"

const SHIFT: u32 = 11;
const BITS: u32 = 1 << SHIFT;
const R_MASK: u32 = BITS - 1;

const ENXIO: i32 = 6;
const SNDRV_PCM_FORMAT_S16: i32 = 2;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

pub type snd_pcm_sframes_t = isize;
pub type snd_pcm_uframes_t = usize;

#[repr(C)]
pub struct snd_pcm_channel_area {
    pub addr: *mut core::ffi::c_void,
    pub first: u32,
    pub step: u32,
}

#[repr(C)]
pub struct snd_pcm_plugin_channel {
    pub area: snd_pcm_channel_area,
    pub frames: snd_pcm_uframes_t,
    pub enabled: i32,
    pub wanted: i32,
}

#[repr(C)]
pub struct snd_pcm_plugin_format {
    pub format: i32,
    pub rate: u32,
    pub channels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_pcm_plugin_action {
    INIT,
    PREPARE,
}

pub type transfer_f = unsafe extern "C" fn(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t;

pub type frames_f = unsafe extern "C" fn(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t;

pub type action_f = unsafe extern "C" fn(
    plugin: *mut snd_pcm_plugin,
    action: snd_pcm_plugin_action,
    udata: u64,
) -> i32;

#[repr(C)]
pub struct snd_pcm_plugin {
    pub src_format: snd_pcm_plugin_format,
    pub dst_format: snd_pcm_plugin_format,
    pub extra_data: *mut core::ffi::c_void,
    pub transfer: Option<transfer_f>,
    pub src_frames: Option<frames_f>,
    pub dst_frames: Option<frames_f>,
    pub action: Option<action_f>,
}

/*
 *  Basic rate conversion plugin
 */

#[repr(C)]
struct rate_channel {
    last_S1: i16,
    last_S2: i16,
}

type rate_f = unsafe extern "C" fn(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    src_frames: i32,
    dst_frames: i32,
);

#[repr(C)]
struct rate_priv {
    pitch: u32,
    pos: u32,
    func: Option<rate_f>,
    old_src_frames: snd_pcm_sframes_t,
    old_dst_frames: snd_pcm_sframes_t,
    channels: [rate_channel; 0],
}

unsafe extern "C" {
    fn snd_pcm_area_silence(
        dst_area: *mut snd_pcm_channel_area,
        dst_offset: u32,
        samples: u32,
        format: i32,
    );
    fn snd_pcm_plugin_build(
        plug: *mut snd_pcm_substream,
        name: *const u8,
        src_format: *mut snd_pcm_plugin_format,
        dst_format: *mut snd_pcm_plugin_format,
        extra: usize,
        ret: *mut *mut snd_pcm_plugin,
    ) -> i32;
}

unsafe fn snd_BUG_ON(cond: bool) -> bool {
    cond
}

fn DIV_ROUND_CLOSEST(x: snd_pcm_uframes_t, divisor: u32) -> snd_pcm_sframes_t {
    ((x + (divisor as usize / 2)) / divisor as usize) as snd_pcm_sframes_t
}

unsafe fn struct_size_rate_priv_channels(channels: u32) -> usize {
    core::mem::size_of::<rate_priv>()
        + channels as usize * core::mem::size_of::<rate_channel>()
}

unsafe extern "C" fn rate_init(plugin: *mut snd_pcm_plugin) {
    let mut channel: u32;
    let data = (*plugin).extra_data as *mut rate_priv;
    (*data).pos = 0;
    channel = 0;
    while channel < (*plugin).src_format.channels {
        let channels = (*data).channels.as_mut_ptr();
        (*channels.add(channel as usize)).last_S1 = 0;
        (*channels.add(channel as usize)).last_S2 = 0;
        channel += 1;
    }
}

unsafe extern "C" fn resample_expand(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    src_frames: i32,
    dst_frames: i32,
) {
    let mut pos: u32 = 0;
    let mut val: i32;
    let mut S1: i16;
    let mut S2: i16;
    let mut src: *mut i16;
    let mut dst: *mut i16;
    let mut channel: u32;
    let src_step: i32;
    let dst_step: i32;
    let mut src_frames1: i32;
    let mut dst_frames1: i32;
    let data = (*plugin).extra_data as *mut rate_priv;
    let mut rchannels = (*data).channels.as_mut_ptr();

    channel = 0;
    while channel < (*plugin).src_format.channels {
        pos = (*data).pos;
        S1 = (*rchannels).last_S1;
        S2 = (*rchannels).last_S2;
        if (*src_channels.add(channel as usize)).enabled == 0 {
            if (*dst_channels.add(channel as usize)).wanted != 0 {
                snd_pcm_area_silence(
                    &mut (*dst_channels.add(channel as usize)).area,
                    0,
                    dst_frames as u32,
                    (*plugin).dst_format.format,
                );
            }
            (*dst_channels.add(channel as usize)).enabled = 0;
            channel += 1;
            continue;
        }
        (*dst_channels.add(channel as usize)).enabled = 1;
        src = ((*src_channels.add(channel as usize)).area.addr as *mut i16)
            .add(((*src_channels.add(channel as usize)).area.first / 8 / 2) as usize);
        dst = ((*dst_channels.add(channel as usize)).area.addr as *mut i16)
            .add(((*dst_channels.add(channel as usize)).area.first / 8 / 2) as usize);
        src_step = ((*src_channels.add(channel as usize)).area.step / 8 / 2) as i32;
        dst_step = ((*dst_channels.add(channel as usize)).area.step / 8 / 2) as i32;
        src_frames1 = src_frames;
        dst_frames1 = dst_frames;
        while {
            let old = dst_frames1;
            dst_frames1 -= 1;
            old > 0
        } {
            if (pos & !R_MASK) != 0 {
                pos &= R_MASK;
                S1 = S2;
                if {
                    let old = src_frames1;
                    src_frames1 -= 1;
                    old > 0
                } {
                    S2 = *src;
                    src = src.offset(src_step as isize);
                }
            }
            val = S1 as i32 + (((S2 as i32 - S1 as i32) * pos as i32) / BITS as i32);
            if val < -32768 {
                val = -32768;
            } else if val > 32767 {
                val = 32767;
            }
            *dst = val as i16;
            dst = dst.offset(dst_step as isize);
            pos = pos.wrapping_add((*data).pitch);
        }
        (*rchannels).last_S1 = S1;
        (*rchannels).last_S2 = S2;
        rchannels = rchannels.add(1);
        channel += 1;
    }
    (*data).pos = pos;
}

unsafe extern "C" fn resample_shrink(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    src_frames: i32,
    dst_frames: i32,
) {
    let mut pos: u32 = 0;
    let mut val: i32;
    let mut S1: i16;
    let mut S2: i16;
    let mut src: *mut i16;
    let mut dst: *mut i16;
    let mut channel: u32;
    let src_step: i32;
    let dst_step: i32;
    let mut src_frames1: i32;
    let mut dst_frames1: i32;
    let data = (*plugin).extra_data as *mut rate_priv;
    let mut rchannels = (*data).channels.as_mut_ptr();

    channel = 0;
    while channel < (*plugin).src_format.channels {
        pos = (*data).pos;
        S1 = (*rchannels).last_S1;
        S2 = (*rchannels).last_S2;
        if (*src_channels.add(channel as usize)).enabled == 0 {
            if (*dst_channels.add(channel as usize)).wanted != 0 {
                snd_pcm_area_silence(
                    &mut (*dst_channels.add(channel as usize)).area,
                    0,
                    dst_frames as u32,
                    (*plugin).dst_format.format,
                );
            }
            (*dst_channels.add(channel as usize)).enabled = 0;
            channel += 1;
            continue;
        }
        (*dst_channels.add(channel as usize)).enabled = 1;
        src = ((*src_channels.add(channel as usize)).area.addr as *mut i16)
            .add(((*src_channels.add(channel as usize)).area.first / 8 / 2) as usize);
        dst = ((*dst_channels.add(channel as usize)).area.addr as *mut i16)
            .add(((*dst_channels.add(channel as usize)).area.first / 8 / 2) as usize);
        src_step = ((*src_channels.add(channel as usize)).area.step / 8 / 2) as i32;
        dst_step = ((*dst_channels.add(channel as usize)).area.step / 8 / 2) as i32;
        src_frames1 = src_frames;
        dst_frames1 = dst_frames;
        while dst_frames1 > 0 {
            S1 = S2;
            if {
                let old = src_frames1;
                src_frames1 -= 1;
                old > 0
            } {
                S2 = *src;
                src = src.offset(src_step as isize);
            }
            if (pos & !R_MASK) != 0 {
                pos &= R_MASK;
                val = S1 as i32 + (((S2 as i32 - S1 as i32) * pos as i32) / BITS as i32);
                if val < -32768 {
                    val = -32768;
                } else if val > 32767 {
                    val = 32767;
                }
                *dst = val as i16;
                dst = dst.offset(dst_step as isize);
                dst_frames1 -= 1;
            }
            pos = pos.wrapping_add((*data).pitch);
        }
        (*rchannels).last_S1 = S1;
        (*rchannels).last_S2 = S2;
        rchannels = rchannels.add(1);
        channel += 1;
    }
    (*data).pos = pos;
}

unsafe extern "C" fn rate_src_frames(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let data: *mut rate_priv;
    let mut res: snd_pcm_sframes_t;

    if snd_BUG_ON(plugin.is_null()) {
        return -ENXIO as snd_pcm_sframes_t;
    }
    if frames == 0 {
        return 0;
    }
    data = (*plugin).extra_data as *mut rate_priv;
    if (*plugin).src_format.rate < (*plugin).dst_format.rate {
        res = (((frames * (*data).pitch as usize) + (BITS as usize / 2)) >> SHIFT)
            as snd_pcm_sframes_t;
    } else {
        res = DIV_ROUND_CLOSEST(frames << SHIFT, (*data).pitch);
    }
    if (*data).old_src_frames > 0 {
        let mut frames1 = frames as snd_pcm_sframes_t;
        let mut res1 = (*data).old_dst_frames;
        while (*data).old_src_frames < frames1 {
            frames1 >>= 1;
            res1 <<= 1;
        }
        while (*data).old_src_frames > frames1 {
            frames1 <<= 1;
            res1 >>= 1;
        }
        if (*data).old_src_frames == frames1 {
            return res1;
        }
    }
    (*data).old_src_frames = frames as snd_pcm_sframes_t;
    (*data).old_dst_frames = res;
    res
}

unsafe extern "C" fn rate_dst_frames(
    plugin: *mut snd_pcm_plugin,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let data: *mut rate_priv;
    let mut res: snd_pcm_sframes_t;

    if snd_BUG_ON(plugin.is_null()) {
        return -ENXIO as snd_pcm_sframes_t;
    }
    if frames == 0 {
        return 0;
    }
    data = (*plugin).extra_data as *mut rate_priv;
    if (*plugin).src_format.rate < (*plugin).dst_format.rate {
        res = DIV_ROUND_CLOSEST(frames << SHIFT, (*data).pitch);
    } else {
        res = (((frames * (*data).pitch as usize) + (BITS as usize / 2)) >> SHIFT)
            as snd_pcm_sframes_t;
    }
    if (*data).old_dst_frames > 0 {
        let mut frames1 = frames as snd_pcm_sframes_t;
        let mut res1 = (*data).old_src_frames;
        while (*data).old_dst_frames < frames1 {
            frames1 >>= 1;
            res1 <<= 1;
        }
        while (*data).old_dst_frames > frames1 {
            frames1 <<= 1;
            res1 >>= 1;
        }
        if (*data).old_dst_frames == frames1 {
            return res1;
        }
    }
    (*data).old_dst_frames = frames as snd_pcm_sframes_t;
    (*data).old_src_frames = res;
    res
}

unsafe extern "C" fn rate_transfer(
    plugin: *mut snd_pcm_plugin,
    src_channels: *const snd_pcm_plugin_channel,
    dst_channels: *mut snd_pcm_plugin_channel,
    frames: snd_pcm_uframes_t,
) -> snd_pcm_sframes_t {
    let mut dst_frames: snd_pcm_uframes_t;
    let data: *mut rate_priv;

    if snd_BUG_ON(plugin.is_null() || src_channels.is_null() || dst_channels.is_null()) {
        return -ENXIO as snd_pcm_sframes_t;
    }
    if frames == 0 {
        return 0;
    }
    // CONFIG_SND_DEBUG: validate channel area bit offsets and steps are byte-aligned.
    #[cfg(CONFIG_SND_DEBUG)]
    {
        let mut channel: u32 = 0;
        while channel < (*plugin).src_format.channels {
            if snd_BUG_ON(
                (*src_channels.add(channel as usize)).area.first % 8 != 0
                    || (*src_channels.add(channel as usize)).area.step % 8 != 0,
            ) {
                return -ENXIO as snd_pcm_sframes_t;
            }
            if snd_BUG_ON(
                (*dst_channels.add(channel as usize)).area.first % 8 != 0
                    || (*dst_channels.add(channel as usize)).area.step % 8 != 0,
            ) {
                return -ENXIO as snd_pcm_sframes_t;
            }
            channel += 1;
        }
    }

    dst_frames = rate_dst_frames(plugin, frames) as snd_pcm_uframes_t;
    if dst_frames > (*dst_channels).frames {
        dst_frames = (*dst_channels).frames;
    }
    data = (*plugin).extra_data as *mut rate_priv;
    if let Some(func) = (*data).func {
        func(
            plugin,
            src_channels,
            dst_channels,
            frames as i32,
            dst_frames as i32,
        );
    }
    dst_frames as snd_pcm_sframes_t
}

unsafe extern "C" fn rate_action(
    plugin: *mut snd_pcm_plugin,
    action: snd_pcm_plugin_action,
    _udata: u64,
) -> i32 {
    if snd_BUG_ON(plugin.is_null()) {
        return -ENXIO;
    }
    match action {
        snd_pcm_plugin_action::INIT | snd_pcm_plugin_action::PREPARE => {
            rate_init(plugin);
        }
    }
    0 /* silently ignore other actions */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_pcm_plugin_build_rate(
    plug: *mut snd_pcm_substream,
    src_format: *mut snd_pcm_plugin_format,
    dst_format: *mut snd_pcm_plugin_format,
    r_plugin: *mut *mut snd_pcm_plugin,
) -> i32 {
    let mut err: i32;
    let data: *mut rate_priv;
    let mut plugin: *mut snd_pcm_plugin = core::ptr::null_mut();

    if snd_BUG_ON(r_plugin.is_null()) {
        return -ENXIO;
    }
    *r_plugin = core::ptr::null_mut();

    if snd_BUG_ON((*src_format).channels != (*dst_format).channels) {
        return -ENXIO;
    }
    if snd_BUG_ON((*src_format).channels <= 0) {
        return -ENXIO;
    }
    if snd_BUG_ON((*src_format).format != SNDRV_PCM_FORMAT_S16) {
        return -ENXIO;
    }
    if snd_BUG_ON((*dst_format).format != SNDRV_PCM_FORMAT_S16) {
        return -ENXIO;
    }
    if snd_BUG_ON((*src_format).rate == (*dst_format).rate) {
        return -ENXIO;
    }

    err = snd_pcm_plugin_build(
        plug,
        b"rate conversion\0".as_ptr(),
        src_format,
        dst_format,
        struct_size_rate_priv_channels((*src_format).channels),
        &mut plugin,
    );
    if err < 0 {
        return err;
    }
    data = (*plugin).extra_data as *mut rate_priv;
    if (*src_format).rate < (*dst_format).rate {
        (*data).pitch = (((*src_format).rate << SHIFT) + ((*dst_format).rate >> 1))
            / (*dst_format).rate;
        (*data).func = Some(resample_expand);
    } else {
        (*data).pitch = (((*dst_format).rate << SHIFT) + ((*src_format).rate >> 1))
            / (*src_format).rate;
        (*data).func = Some(resample_shrink);
    }
    (*data).pos = 0;
    rate_init(plugin);
    (*data).old_dst_frames = 0;
    (*data).old_src_frames = (*data).old_dst_frames;
    (*plugin).transfer = Some(rate_transfer);
    (*plugin).src_frames = Some(rate_src_frames);
    (*plugin).dst_frames = Some(rate_dst_frames);
    (*plugin).action = Some(rate_action);
    *r_plugin = plugin;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
