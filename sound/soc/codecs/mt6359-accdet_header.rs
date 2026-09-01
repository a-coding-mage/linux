/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 MediaTek Inc.
 * Author: Argus Lin <argus.lin@mediatek.com>
 */

/* C dependencies removed from executable Rust:
 * #include <linux/ctype.h>
 * #include <linux/string.h>
 */

pub const ACCDET_DEVNAME: &str = "accdet";

pub const HEADSET_MODE_1: u32 = 1;
pub const HEADSET_MODE_2: u32 = 2;
pub const HEADSET_MODE_6: u32 = 6;

pub const MT6359_ACCDET_NUM_BUTTONS: u32 = 4;
pub const MT6359_ACCDET_JACK_MASK: u32 = SND_JACK_HEADPHONE
    | SND_JACK_HEADSET
    | SND_JACK_BTN_0
    | SND_JACK_BTN_1
    | SND_JACK_BTN_2
    | SND_JACK_BTN_3;
pub const MT6359_ACCDET_BTN_MASK: u32 =
    SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum eint_moisture_status {
    M_PLUG_IN = 0,
    M_WATER_IN = 1,
    M_HP_PLUG_IN = 2,
    M_PLUG_OUT = 3,
    M_NO_ACT = 4,
    M_UNKNOWN = 5,
}

pub const accdet_state000: u32 = 0;
pub const accdet_state001: u32 = 1;
pub const accdet_state010: u32 = 2;
pub const accdet_state011: u32 = 3;
pub const accdet_auxadc: u32 = 4;
pub const eint_state000: u32 = 5;
pub const eint_state001: u32 = 6;
pub const eint_state010: u32 = 7;
pub const eint_state011: u32 = 8;
pub const eint_inverter_state000: u32 = 9;

#[repr(C)]
pub struct three_key_threshold {
    pub mid: u32,
    pub up: u32,
    pub down: u32,
}

#[repr(C)]
pub struct four_key_threshold {
    pub mid: u32,
    pub voice: u32,
    pub up: u32,
    pub down: u32,
}

#[repr(C)]
pub struct pwm_deb_settings {
    pub pwm_width: u32,
    pub pwm_thresh: u32,
    pub fall_delay: u32,
    pub rise_delay: u32,
    pub debounce0: u32,
    pub debounce1: u32,
    pub debounce3: u32,
    pub debounce4: u32,
    pub eint_pwm_width: u32,
    pub eint_pwm_thresh: u32,
    pub eint_debounce0: u32,
    pub eint_debounce1: u32,
    pub eint_debounce2: u32,
    pub eint_debounce3: u32,
    pub eint_inverter_debounce: u32,
}

#[repr(C)]
pub struct dts_data {
    pub mic_vol: u32,
    pub mic_mode: u32,
    pub plugout_deb: u32,
    pub eint_pol: u32,
    pub pwm_deb: *mut pwm_deb_settings,
    pub three_key: three_key_threshold,
    pub four_key: four_key_threshold,
    pub moisture_detect_enable: u32,
    pub eint_detect_mode: u32,
    pub eint_use_ext_res: u32,
    pub eint_comp_vth: u32,
    pub moisture_detect_mode: u32,
    pub moisture_comp_vth: u32,
    pub moisture_comp_vref2: u32,
    pub moisture_use_ext_res: u32,
}

#[repr(C)]
pub struct mt6359_accdet {
    pub jack: *mut snd_soc_jack,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub data: *mut dts_data,
    pub caps: u32,
    pub accdet_irq: i32,
    pub accdet_eint0: i32,
    pub accdet_eint1: i32,
    pub res_lock: mutex, /* lock protection */
    pub jack_plugged: bool,
    pub jack_type: u32,
    pub btn_type: u32,
    pub accdet_status: u32,
    pub pre_accdet_status: u32,
    pub cali_voltage: u32,
    pub jd_sts: u32,
    pub accdet_work: work_struct,
    pub accdet_workqueue: *mut workqueue_struct,
    pub jd_work: work_struct,
    pub jd_workqueue: *mut workqueue_struct,
}

/* C condition preserved:
 * #if IS_ENABLED(CONFIG_SND_SOC_MT6359_ACCDET)
 */
#[cfg(CONFIG_SND_SOC_MT6359_ACCDET)]
extern "C" {
    pub fn mt6359_accdet_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> i32;
}

/* #else */
#[cfg(not(CONFIG_SND_SOC_MT6359_ACCDET))]
#[inline]
pub unsafe fn mt6359_accdet_enable_jack_detect(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
) -> i32 {
    let _ = component;
    let _ = jack;
    -EOPNOTSUPP
}
/* #endif */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
