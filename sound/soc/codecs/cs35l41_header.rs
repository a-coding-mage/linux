/* SPDX-License-Identifier: GPL-2.0
 *
 * cs35l41.h -- CS35L41 ALSA SoC audio driver
 *
 * Copyright 2017-2021 Cirrus Logic, Inc.
 *
 * Author: David Rhodes <david.rhodes@cirrus.com>
 */

/* C header dependencies:
 * linux/gpio/consumer.h
 * linux/regulator/consumer.h
 * linux/firmware.h
 * sound/core.h
 * sound/cs35l41.h
 * wm_adsp.h
 */

pub const CS35L41_RX_FORMATS: u64 =
    (SNDRV_PCM_FMTBIT_S16_LE as u64) | (SNDRV_PCM_FMTBIT_S24_LE as u64);
pub const CS35L41_TX_FORMATS: u64 =
    (SNDRV_PCM_FMTBIT_S16_LE as u64) | (SNDRV_PCM_FMTBIT_S24_LE as u64);

unsafe extern "C" {
    pub static cs35l41_pm_ops: dev_pm_ops;
}

#[repr(C)]
pub struct cs35l41_private {
    pub dsp: wm_adsp, /* needs to be first member */
    pub codec: *mut snd_soc_codec,
    pub hw_cfg: cs35l41_hw_cfg,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; CS35L41_NUM_SUPPLIES],
    pub irq: ::core::ffi::c_int,
    /* GPIO for /RST */
    pub reset_gpio: *mut gpio_desc,
}

unsafe extern "C" {
    pub fn cs35l41_probe(
        cs35l41: *mut cs35l41_private,
        hw_cfg: *const cs35l41_hw_cfg,
    ) -> ::core::ffi::c_int;
    pub fn cs35l41_remove(cs35l41: *mut cs35l41_private);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
