// SPDX-License-Identifier: GPL-2.0+
//
// media_header.rs - Media Controller specific ALSA driver code
//
// Copyright (c) 2019 Shuah Khan <shuah@kernel.org>
//
// This file adds Media Controller support to the ALSA driver
// to use the Media Controller API to share the tuner with DVB
// and V4L2 drivers that control the media device.
//
// The media device is created based on the existing quirks framework.
// Using this approach, the media controller API usage can be added for
// a specific device.

// External dependencies: linux/media.h, media/media-device.h,
// media/media-entity.h, media/media-dev-allocator.h, sound/asound.h

#[repr(C)]
pub struct media_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_entity {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_intf_devnode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_pad {
    _private: [u8; 0],
}

#[repr(C)]
pub struct media_pipeline {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_audio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_usb_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_interface {
    _private: [u8; 0],
}

// From sound/asound.h
pub const SNDRV_PCM_STREAM_LAST: usize = 1;

// One source pad each for SNDRV_PCM_STREAM_CAPTURE and
// SNDRV_PCM_STREAM_PLAYBACK. One for sink pad to link
// to AUDIO Source
pub const MEDIA_MIXER_PAD_MAX: usize = SNDRV_PCM_STREAM_LAST + 2;

#[repr(C)]
pub struct media_ctl {
    pub media_dev: *mut media_device,
    pub media_entity: media_entity,
    pub intf_devnode: *mut media_intf_devnode,
    pub intf_link: *mut media_link,
    pub media_pad: media_pad,
    pub media_pipe: media_pipeline,
}

#[repr(C)]
pub struct media_mixer_ctl {
    pub media_dev: *mut media_device,
    pub media_entity: media_entity,
    pub intf_devnode: *mut media_intf_devnode,
    pub intf_link: *mut media_link,
    pub media_pad: [media_pad; MEDIA_MIXER_PAD_MAX],
    pub media_pipe: media_pipeline,
}

#[cfg(any())]
// CONFIG_SND_USB_AUDIO_USE_MEDIA_CONTROLLER enabled
extern "C" {
    pub fn snd_media_device_create(chip: *mut snd_usb_audio, iface: *mut usb_interface) -> i32;
    pub fn snd_media_device_delete(chip: *mut snd_usb_audio);
    pub fn snd_media_stream_init(
        subs: *mut snd_usb_substream,
        pcm: *mut snd_pcm,
        stream: i32,
    ) -> i32;
    pub fn snd_media_stream_delete(subs: *mut snd_usb_substream);
    pub fn snd_media_start_pipeline(subs: *mut snd_usb_substream) -> i32;
    pub fn snd_media_stop_pipeline(subs: *mut snd_usb_substream);
}

#[cfg(not(any()))]
// CONFIG_SND_USB_AUDIO_USE_MEDIA_CONTROLLER disabled
pub mod stubs {
    use super::*;

    #[inline]
    pub fn snd_media_device_create(_chip: *mut snd_usb_audio, _iface: *mut usb_interface) -> i32 {
        0
    }

    #[inline]
    pub fn snd_media_device_delete(_chip: *mut snd_usb_audio) {}

    #[inline]
    pub fn snd_media_stream_init(
        _subs: *mut snd_usb_substream,
        _pcm: *mut snd_pcm,
        _stream: i32,
    ) -> i32 {
        0
    }

    #[inline]
    pub fn snd_media_stream_delete(_subs: *mut snd_usb_substream) {}

    #[inline]
    pub fn snd_media_start_pipeline(_subs: *mut snd_usb_substream) -> i32 {
        0
    }

    #[inline]
    pub fn snd_media_stop_pipeline(_subs: *mut snd_usb_substream) {}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
