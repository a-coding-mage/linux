/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2009 Samsung Electronics Co. Ltd
 * Author: Jaswinder Singh <jassi.brar@samsung.com>
 */

/* The machine init code calls s3c*_ac97_setup_gpio with
 * one of these defines in order to select appropriate bank
 * of GPIO for AC97 pins
 */
pub const S3C64XX_AC97_GPD: i32 = 0;
pub const S3C64XX_AC97_GPE: i32 = 1;

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct samsung_i2s_type {
    /* If the Primary DAI has 5.1 Channels */
    pub quirks: u32,
    pub idma_addr: dma_addr_t,
}

/* If the Primary DAI has 5.1 Channels */
pub const QUIRK_PRI_6CHAN: u32 = 1 << 0;
/* If the I2S block has a Stereo Overlay Channel */
pub const QUIRK_SEC_DAI: u32 = 1 << 1;
/*
 * If the I2S block has no internal prescalar or MUX (I2SMOD[10] bit)
 * The Machine driver must provide suitably set clock to the I2S block.
 */
pub const QUIRK_NO_MUXPSR: u32 = 1 << 2;
pub const QUIRK_NEED_RSTCLR: u32 = 1 << 3;
pub const QUIRK_SUPPORTS_TDM: u32 = 1 << 4;
pub const QUIRK_SUPPORTS_IDMA: u32 = 1 << 5;

/* Quirks of the I2S controller */

/**
 * struct s3c_audio_pdata - common platform data for audio device drivers
 * @cfg_gpio: Callback function to setup mux'ed pins in I2S/PCM/AC97 mode
 */
#[repr(C)]
pub struct s3c_audio_pdata {
    pub cfg_gpio: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub dma_filter: dma_filter_fn,
    pub dma_playback: *mut core::ffi::c_void,
    pub dma_capture: *mut core::ffi::c_void,
    pub dma_play_sec: *mut core::ffi::c_void,
    pub dma_capture_mic: *mut core::ffi::c_void,
    pub r#type: samsung_i2s_type,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
