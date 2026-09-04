// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn snd_usb_audio_create_proc(chip: *mut snd_usb_audio);
    pub fn snd_usb_proc_pcm_format_add(stream: *mut snd_usb_stream);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
