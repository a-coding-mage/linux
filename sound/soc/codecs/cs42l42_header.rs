// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l42.h -- CS42L42 ALSA SoC audio driver header
 *
 * Copyright 2016-2022 Cirrus Logic, Inc.
 *
 * Author: James Schulman <james.schulman@cirrus.com>
 * Author: Brian Austin <brian.austin@cirrus.com>
 * Author: Michael White <michael.white@cirrus.com>
 */

// C header dependencies:
// <dt-bindings/sound/cs42l42.h>
// <linux/device.h>
// <linux/gpio/consumer.h>
// <linux/mutex.h>
// <linux/regmap.h>
// <linux/regulator/consumer.h>
// <linux/soundwire/sdw.h>
// <sound/jack.h>
// <sound/cs42l42.h>
// <sound/soc-component.h>
// <sound/soc-dai.h>

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct cs42l42_private {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub supplies: [regulator_bulk_data; CS42L42_NUM_SUPPLIES],
    pub reset_gpio: *mut gpio_desc,
    pub pdn_done: completion,
    pub jack: *mut snd_soc_jack,
    pub sdw_peripheral: *mut sdw_slave,
    pub irq_lock: mutex,
    pub devid: c_int,
    pub irq: c_int,
    pub pll_config: c_int,
    pub sclk: u32,
    pub sample_rate: u32,
    pub bclk_ratio: u32,
    pub plug_state: u8,
    pub hs_type: u8,
    pub ts_inv: u8,
    pub ts_dbnc_rise: u8,
    pub ts_dbnc_fall: u8,
    pub btn_det_init_dbnce: u8,
    pub btn_det_event_dbnce: u8,
    pub bias_thresholds: [u8; CS42L42_NUM_BIASES],
    pub hs_bias_ramp_rate: u8,
    pub hs_bias_ramp_time: u8,
    pub hs_bias_sense_en: u8,
    pub stream_use: u8,
    pub hp_adc_up_pending: bool,
    pub suspended: bool,
    pub sdw_waiting_first_unattach: bool,
    pub init_done: bool,
}

extern "C" {
    pub static cs42l42_page_range: regmap_range_cfg;
    pub static cs42l42_regmap: regmap_config;
    pub static cs42l42_soc_component: snd_soc_component_driver;
    pub static mut cs42l42_dai: snd_soc_dai_driver;

    pub fn cs42l42_readable_register(dev: *mut device, reg: c_uint) -> bool;
    pub fn cs42l42_volatile_register(dev: *mut device, reg: c_uint) -> bool;

    pub fn cs42l42_pll_config(
        component: *mut snd_soc_component,
        clk: c_uint,
        sample_rate: c_uint,
    ) -> c_int;
    pub fn cs42l42_src_config(component: *mut snd_soc_component, sample_rate: c_uint);
    pub fn cs42l42_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int;
    pub fn cs42l42_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
    pub fn cs42l42_suspend(dev: *mut device) -> c_int;
    pub fn cs42l42_resume(dev: *mut device) -> c_int;
    pub fn cs42l42_resume_restore(dev: *mut device);
    pub fn cs42l42_common_probe(
        cs42l42: *mut cs42l42_private,
        component_drv: *const snd_soc_component_driver,
        dai: *mut snd_soc_dai_driver,
    ) -> c_int;
    pub fn cs42l42_init(cs42l42: *mut cs42l42_private) -> c_int;
    pub fn cs42l42_common_remove(cs42l42: *mut cs42l42_private);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
