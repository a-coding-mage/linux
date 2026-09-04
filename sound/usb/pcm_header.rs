// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_usb_set_pcm_ops(pcm: *mut snd_pcm, stream: i32);
    pub fn snd_usb_pcm_suspend(as_: *mut snd_usb_stream) -> i32;
    pub fn snd_usb_pcm_resume(as_: *mut snd_usb_stream) -> i32;

    pub fn snd_usb_pcm_has_fixed_rate(as_: *mut snd_usb_substream) -> bool;

    pub fn snd_usb_init_pitch(chip: *mut snd_usb_audio, fmt: *const audioformat) -> i32;
    pub fn snd_usb_preallocate_buffer(subs: *mut snd_usb_substream);

    pub fn snd_usb_audioformat_set_sync_ep(
        chip: *mut snd_usb_audio,
        fmt: *mut audioformat,
    ) -> i32;

    pub fn snd_usb_find_format(
        fmt_list_head: *mut list_head,
        format: snd_pcm_format_t,
        rate: u32,
        channels: u32,
        strict_match: bool,
        subs: *mut snd_usb_substream,
    ) -> *const audioformat;

    pub fn snd_usb_find_substream_format(
        subs: *mut snd_usb_substream,
        params: *const snd_pcm_hw_params,
    ) -> *const audioformat;

    pub fn snd_usb_hw_params(
        subs: *mut snd_usb_substream,
        hw_params: *mut snd_pcm_hw_params,
    ) -> i32;
    pub fn snd_usb_hw_free(subs: *mut snd_usb_substream) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
