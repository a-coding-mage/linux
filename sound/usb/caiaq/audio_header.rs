// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub type snd_usb_caiaqdev;

    pub fn snd_usb_caiaq_audio_init(cdev: *mut snd_usb_caiaqdev) -> i32;
    pub fn snd_usb_caiaq_audio_disconnect(cdev: *mut snd_usb_caiaqdev);
    pub fn snd_usb_caiaq_audio_free(cdev: *mut snd_usb_caiaqdev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
