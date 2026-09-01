// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * da7219-aad.c - Dialog DA7219 ALSA SoC AAD Driver
 *
 * Copyright (c) 2015 Dialog Semiconductor Ltd.
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/*
 * C includes translated as external dependencies:
 * linux/module.h, linux/platform_device.h, linux/clk.h, linux/i2c.h,
 * linux/property.h, linux/pm_wakeirq.h, linux/slab.h, linux/delay.h,
 * linux/workqueue.h, sound/soc.h, sound/jack.h, sound/da7219.h,
 * da7219.h, da7219-aad.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_uint;
type __le16 = u16;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct da7219_priv {
    pub aad: *mut da7219_aad_priv,
    pub pdata: *mut da7219_pdata,
    pub micbias_on_event: bool_,
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub ctrl_lock: mutex,
    pub pll_lock: mutex,
}

#[repr(C)]
pub struct da7219_aad_priv {
    pub component: *mut snd_soc_component,
    pub jack: *mut snd_soc_jack,
    pub jack_inserted: bool_,
    pub micbias_resume_enable: bool_,
    pub micbias_pulse_lvl: u8,
    pub micbias_pulse_time: c_uint,
    pub btn_cfg: u8,
    pub irq: c_int,
    pub aad_wq: *mut workqueue_struct,
    pub jack_det_work: delayed_work,
    pub btn_det_work: work_struct,
    pub hptest_work: work_struct,
    pub gnd_switch_delay: c_int,
}

#[repr(C)]
pub struct da7219_pdata {
    pub aad_pdata: *mut da7219_aad_pdata,
}

#[repr(C)]
pub struct da7219_aad_pdata {
    pub irq: c_int,
    pub micbias_pulse_lvl: da7219_aad_micbias_pulse_lvl,
    pub micbias_pulse_time: u32,
    pub btn_cfg: da7219_aad_btn_cfg,
    pub mic_det_thr: da7219_aad_mic_det_thr,
    pub jack_ins_deb: da7219_aad_jack_ins_deb,
    pub jack_ins_det_pty: da7219_aad_jack_ins_det_pty,
    pub jack_det_rate: da7219_aad_jack_det_rate,
    pub jack_rem_deb: da7219_aad_jack_rem_deb,
    pub a_d_btn_thr: u8,
    pub d_b_btn_thr: u8,
    pub b_c_btn_thr: u8,
    pub c_mic_btn_thr: u8,
    pub btn_avg: da7219_aad_btn_avg,
    pub adc_1bit_rpt: da7219_aad_adc_1bit_rpt,
}

type da7219_aad_micbias_pulse_lvl = c_uint;
type da7219_aad_btn_cfg = c_uint;
type da7219_aad_mic_det_thr = c_uint;
type da7219_aad_jack_ins_deb = c_uint;
type da7219_aad_jack_ins_det_pty = c_uint;
type da7219_aad_jack_det_rate = c_uint;
type da7219_aad_jack_rem_deb = c_uint;
type da7219_aad_btn_avg = c_uint;
type da7219_aad_adc_1bit_rpt = c_uint;

extern "C" {
    static DA7219_AAD_REPORT_ALL_MASK: c_int;
    static DA7219_ACCDET_CONFIG_1: c_uint;
    static DA7219_ACCDET_EN_MASK: c_uint;
    static DA7219_HP_L_CTRL: c_uint;
    static DA7219_HP_L_AMP_OE_MASK: c_uint;
    static DA7219_HP_R_CTRL: c_uint;
    static DA7219_HP_R_AMP_OE_MASK: c_uint;
    static DA7219_ACCDET_STATUS_A: c_uint;
    static DA7219_MICBIAS_UP_STS_MASK: c_uint;
    static DA7219_AAD_MICBIAS_CHK_RETRIES: c_int;
    static DA7219_AAD_MICBIAS_CHK_DELAY: c_uint;
    static DA7219_MICBIAS_CTRL: c_uint;
    static DA7219_MICBIAS1_LEVEL_MASK: c_uint;
    static DA7219_BUTTON_CONFIG_MASK: c_uint;
    static DA7219_PLL_SRM_STS: c_uint;
    static DA7219_PLL_SRM_STS_MCLK: c_uint;
    static DA7219_AAD_HPTEST_RAMP_FREQ: c_uint;
    static DA7219_AAD_HPTEST_RAMP_FREQ_INT_OSC: c_uint;
    static DA7219_PLL_CTRL: c_uint;
    static DA7219_PLL_MODE_MASK: c_uint;
    static DA7219_PLL_MODE_BYPASS: c_uint;
    static DA7219_SYSCLK_PLL: c_int;
    static DA7219_PLL_FREQ_OUT_98304: c_uint;
    static DA7219_GAIN_RAMP_CTRL: c_uint;
    static DA7219_GAIN_RAMP_RATE_X8: c_uint;
    static DA7219_TONE_GEN_CFG1: c_uint;
    static DA7219_ACCDET_CONFIG_8: c_uint;
    static DA7219_HPTEST_EN_MASK: c_uint;
    static DA7219_HPTEST_RES_SEL_MASK: c_uint;
    static DA7219_HPTEST_RES_SEL_1KOHMS: c_uint;
    static DA7219_DAC_L_GAIN: c_uint;
    static DA7219_DAC_DIGITAL_GAIN_0DB: c_uint;
    static DA7219_DAC_R_GAIN: c_uint;
    static DA7219_HP_L_GAIN: c_uint;
    static DA7219_HP_AMP_GAIN_0DB: c_uint;
    static DA7219_HP_R_GAIN: c_uint;
    static DA7219_DAC_FILTERS1: c_uint;
    static DA7219_HPF_MODE_MASK: c_uint;
    static DA7219_DAC_FILTERS4: c_uint;
    static DA7219_DAC_EQ_EN_MASK: c_uint;
    static DA7219_DAC_FILTERS5: c_uint;
    static DA7219_DAC_SOFTMUTE_EN_MASK: c_uint;
    static DA7219_CP_CTRL: c_uint;
    static DA7219_CP_EN_MASK: c_uint;
    static DA7219_DIG_ROUTING_DAC: c_uint;
    static DA7219_DAC_L_SRC_MASK: c_uint;
    static DA7219_DAC_R_SRC_MASK: c_uint;
    static DA7219_DAC_L_SRC_TONEGEN: c_uint;
    static DA7219_DAC_R_SRC_TONEGEN: c_uint;
    static DA7219_DAC_L_CTRL: c_uint;
    static DA7219_DAC_L_EN_MASK: c_uint;
    static DA7219_DAC_L_MUTE_EN_MASK: c_uint;
    static DA7219_DAC_R_CTRL: c_uint;
    static DA7219_DAC_R_EN_MASK: c_uint;
    static DA7219_DAC_R_MUTE_EN_MASK: c_uint;
    static DA7219_MIXOUT_L_SELECT: c_uint;
    static DA7219_MIXOUT_L_MIX_SELECT_MASK: c_uint;
    static DA7219_MIXOUT_R_SELECT: c_uint;
    static DA7219_MIXOUT_R_MIX_SELECT_MASK: c_uint;
    static DA7219_DROUTING_ST_OUTFILT_1L: c_uint;
    static DA7219_OUTFILT_ST_1L_SRC_MASK: c_uint;
    static DA7219_DMIX_ST_SRC_OUTFILT1L: c_uint;
    static DA7219_DROUTING_ST_OUTFILT_1R: c_uint;
    static DA7219_OUTFILT_ST_1R_SRC_MASK: c_uint;
    static DA7219_DMIX_ST_SRC_OUTFILT1R: c_uint;
    static DA7219_MIXOUT_L_CTRL: c_uint;
    static DA7219_MIXOUT_L_AMP_EN_MASK: c_uint;
    static DA7219_MIXOUT_R_CTRL: c_uint;
    static DA7219_MIXOUT_R_AMP_EN_MASK: c_uint;
    static DA7219_HP_L_AMP_EN_MASK: c_uint;
    static DA7219_HP_R_AMP_EN_MASK: c_uint;
    static DA7219_SETTLING_DELAY: c_uint;
    static DA7219_HP_L_AMP_MUTE_EN_MASK: c_uint;
    static DA7219_HP_L_AMP_MIN_GAIN_EN_MASK: c_uint;
    static DA7219_HP_R_AMP_MUTE_EN_MASK: c_uint;
    static DA7219_HP_R_AMP_MIN_GAIN_EN_MASK: c_uint;
    static DA7219_AAD_HPTEST_INT_OSC_PATH_DELAY: c_uint;
    static DA7219_TONE_GEN_ON_PER: c_uint;
    static DA7219_BEEP_ON_PER_MASK: c_uint;
    static DA7219_TONE_GEN_FREQ1_L: c_uint;
    static DA7219_TONE_GEN_CFG2: c_uint;
    static DA7219_SWG_SEL_MASK: c_uint;
    static DA7219_TONE_GEN_GAIN_MASK: c_uint;
    static DA7219_SWG_SEL_SRAMP: c_uint;
    static DA7219_TONE_GEN_GAIN_MINUS_15DB: c_uint;
    static DA7219_START_STOPN_MASK: c_uint;
    static DA7219_AAD_HPTEST_PERIOD: c_uint;
    static DA7219_HPTEST_COMP_MASK: c_uint;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_LINEOUT: c_int;
    static DA7219_TONE_GEN_FREQ1_U: c_uint;
    static DA7219_SYSCLK_MCLK: c_int;
    static SND_JACK_HEADSET: c_int;
    static DA7219_AAD_IRQ_REG_MAX: usize;
    static DA7219_ACCDET_IRQ_EVENT_A: c_uint;
    static DA7219_AAD_IRQ_REG_A: usize;
    static DA7219_AAD_IRQ_REG_B: usize;
    static IRQ_NONE: irqreturn_t;
    static DA7219_JACK_INSERTION_STS_MASK: c_uint;
    static DA7219_E_JACK_INSERTED_MASK: c_uint;
    static DA7219_JACK_TYPE_STS_MASK: c_uint;
    static DA7219_E_JACK_DETECT_COMPLETE_MASK: c_uint;
    static DA7219_AAD_MAX_BUTTONS: c_int;
    static DA7219_E_BUTTON_A_PRESSED_MASK: c_uint;
    static SND_JACK_BTN_0: c_int;
    static DA7219_E_BUTTON_A_RELEASED_MASK: c_uint;
    static DA7219_E_JACK_REMOVED_MASK: c_uint;
    static DA7219_BUTTON_CONFIG_SHIFT: c_uint;
    static DA7219_ACCDET_IRQ_MASK_A: c_uint;
    static DA7219_BYTE_MASK: c_int;
    static IRQ_HANDLED: irqreturn_t;
    static SND_JACK_MECHANICAL: c_int;
    static DA7219_AAD_MICBIAS_PULSE_LVL_2_8V: da7219_aad_micbias_pulse_lvl;
    static DA7219_AAD_MICBIAS_PULSE_LVL_2_9V: da7219_aad_micbias_pulse_lvl;
    static DA7219_AAD_MICBIAS_PULSE_LVL_OFF: da7219_aad_micbias_pulse_lvl;
    static DA7219_AAD_BTN_CFG_2MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_5MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_10MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_50MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_100MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_200MS: da7219_aad_btn_cfg;
    static DA7219_AAD_BTN_CFG_500MS: da7219_aad_btn_cfg;
    static DA7219_AAD_MIC_DET_THR_200_OHMS: da7219_aad_mic_det_thr;
    static DA7219_AAD_MIC_DET_THR_500_OHMS: da7219_aad_mic_det_thr;
    static DA7219_AAD_MIC_DET_THR_750_OHMS: da7219_aad_mic_det_thr;
    static DA7219_AAD_MIC_DET_THR_1000_OHMS: da7219_aad_mic_det_thr;
    static DA7219_AAD_JACK_INS_DEB_5MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_10MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_20MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_50MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_100MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_200MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_500MS: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DEB_1S: da7219_aad_jack_ins_deb;
    static DA7219_AAD_JACK_INS_DET_PTY_LOW: da7219_aad_jack_ins_det_pty;
    static DA7219_AAD_JACK_INS_DET_PTY_HIGH: da7219_aad_jack_ins_det_pty;
    static DA7219_AAD_JACK_DET_RATE_32_64MS: da7219_aad_jack_det_rate;
    static DA7219_AAD_JACK_DET_RATE_64_128MS: da7219_aad_jack_det_rate;
    static DA7219_AAD_JACK_DET_RATE_128_256MS: da7219_aad_jack_det_rate;
    static DA7219_AAD_JACK_DET_RATE_256_512MS: da7219_aad_jack_det_rate;
    static DA7219_AAD_JACK_REM_DEB_1MS: da7219_aad_jack_rem_deb;
    static DA7219_AAD_JACK_REM_DEB_5MS: da7219_aad_jack_rem_deb;
    static DA7219_AAD_JACK_REM_DEB_10MS: da7219_aad_jack_rem_deb;
    static DA7219_AAD_JACK_REM_DEB_20MS: da7219_aad_jack_rem_deb;
    static DA7219_AAD_BTN_AVG_1: da7219_aad_btn_avg;
    static DA7219_AAD_BTN_AVG_2: da7219_aad_btn_avg;
    static DA7219_AAD_BTN_AVG_4: da7219_aad_btn_avg;
    static DA7219_AAD_BTN_AVG_8: da7219_aad_btn_avg;
    static DA7219_AAD_ADC_1BIT_RPT_1: da7219_aad_adc_1bit_rpt;
    static DA7219_AAD_ADC_1BIT_RPT_2: da7219_aad_adc_1bit_rpt;
    static DA7219_AAD_ADC_1BIT_RPT_4: da7219_aad_adc_1bit_rpt;
    static DA7219_AAD_ADC_1BIT_RPT_8: da7219_aad_adc_1bit_rpt;
    static GFP_KERNEL: c_uint;
    static DA7219_MICBIAS1_LEVEL_SHIFT: c_uint;
    static DA7219_MIC_DET_THRESH_SHIFT: c_uint;
    static DA7219_MIC_DET_THRESH_MASK: c_uint;
    static DA7219_ACCDET_CONFIG_2: c_uint;
    static DA7219_JACKDET_DEBOUNCE_SHIFT: c_uint;
    static DA7219_JACKDET_DEBOUNCE_MASK: c_uint;
    static DA7219_JACK_DETECT_RATE_SHIFT: c_uint;
    static DA7219_JACK_DETECT_RATE_MASK: c_uint;
    static DA7219_JACKDET_REM_DEB_SHIFT: c_uint;
    static DA7219_JACKDET_REM_DEB_MASK: c_uint;
    static DA7219_ACCDET_CONFIG_3: c_uint;
    static DA7219_ACCDET_CONFIG_4: c_uint;
    static DA7219_ACCDET_CONFIG_5: c_uint;
    static DA7219_ACCDET_CONFIG_6: c_uint;
    static DA7219_BUTTON_AVERAGE_SHIFT: c_uint;
    static DA7219_BUTTON_AVERAGE_MASK: c_uint;
    static DA7219_ADC_1_BIT_REPEAT_SHIFT: c_uint;
    static DA7219_ADC_1_BIT_REPEAT_MASK: c_uint;
    static DA7219_ACCDET_CONFIG_7: c_uint;
    static DA7219_MICBIAS1_EN_MASK: c_uint;
    static ENOMEM: c_int;
    static IRQF_TRIGGER_LOW: c_uint;
    static IRQF_ONESHOT: c_uint;
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, report: c_int, mask: c_int);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn da7219_set_pll(component: *mut snd_soc_component, source: c_int, fout: c_uint) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_len: usize) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_count: usize) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_count: usize) -> c_int;
    fn msleep(msecs: c_uint);
    fn msecs_to_jiffies(msecs: c_int) -> c_uint;
    fn queue_delayed_work(wq: *mut workqueue_struct, dwork: *mut delayed_work, delay: c_uint) -> bool_;
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool_;
    fn cancel_work_sync(work: *mut work_struct) -> bool_;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn device_get_named_child_node(dev: *mut device, childname: *const c_char) -> *mut fwnode_handle;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn fwnode_property_read_u32(fwnode: *mut fwnode_handle, propname: *const c_char, val: *mut u32) -> c_int;
    fn fwnode_property_read_string(fwnode: *mut fwnode_handle, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn INIT_DELAYED_WORK(dwork: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn request_threaded_irq(irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
}

unsafe fn cpu_to_le16(v: c_uint) -> __le16 {
    (v as u16).to_le()
}

unsafe fn da7219_aad_from_btn_work(work: *mut work_struct) -> *mut da7219_aad_priv {
    (work as *mut u8).sub(core::mem::offset_of!(da7219_aad_priv, btn_det_work)) as *mut da7219_aad_priv
}

unsafe fn da7219_aad_from_hptest_work(work: *mut work_struct) -> *mut da7219_aad_priv {
    (work as *mut u8).sub(core::mem::offset_of!(da7219_aad_priv, hptest_work)) as *mut da7219_aad_priv
}

unsafe fn da7219_aad_from_jack_det_work(work: *mut work_struct) -> *mut da7219_aad_priv {
    (work as *mut u8).sub(core::mem::offset_of!(da7219_aad_priv, jack_det_work) + core::mem::offset_of!(delayed_work, work)) as *mut da7219_aad_priv
}

/*
 * Detection control
 */

#[no_mangle]
pub unsafe extern "C" fn da7219_aad_jack_det(component: *mut snd_soc_component, jack: *mut snd_soc_jack) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;

    (*(*da7219).aad).jack = jack;
    (*(*da7219).aad).jack_inserted = false;

    /* Send an initial empty report */
    snd_soc_jack_report(jack, 0, DA7219_AAD_REPORT_ALL_MASK);

    /* Enable/Disable jack detection */
    snd_soc_component_update_bits(
        component,
        DA7219_ACCDET_CONFIG_1,
        DA7219_ACCDET_EN_MASK,
        if !jack.is_null() { DA7219_ACCDET_EN_MASK } else { 0 },
    );
}

/*
 * Button/HPTest work
 */

unsafe extern "C" fn da7219_aad_btn_det_work(work: *mut work_struct) {
    let da7219_aad = da7219_aad_from_btn_work(work);
    let component = (*da7219_aad).component;
    let dapm = snd_soc_component_to_dapm(component);
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let mut statusa: u8;
    let micbias_ctrl: u8;
    let mut micbias_up = false;
    let mut retries: c_int = 0;

    /* Drive headphones/lineout */
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_OE_MASK, DA7219_HP_L_AMP_OE_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_OE_MASK, DA7219_HP_R_AMP_OE_MASK);

    /* Make sure mic bias is up */
    snd_soc_dapm_force_enable_pin(dapm, b"Mic Bias\0".as_ptr() as *const c_char);
    snd_soc_dapm_sync(dapm);

    loop {
        statusa = snd_soc_component_read(component, DA7219_ACCDET_STATUS_A) as u8;
        if (statusa as c_uint & DA7219_MICBIAS_UP_STS_MASK) != 0 {
            micbias_up = true;
        } else if {
            let old = retries;
            retries += 1;
            old < DA7219_AAD_MICBIAS_CHK_RETRIES
        } {
            msleep(DA7219_AAD_MICBIAS_CHK_DELAY);
        }
        if !(!micbias_up && retries < DA7219_AAD_MICBIAS_CHK_RETRIES) {
            break;
        }
    }

    if retries >= DA7219_AAD_MICBIAS_CHK_RETRIES {
        dev_warn((*component).dev, b"Mic bias status check timed out\0".as_ptr() as *const c_char);
    }

    (*da7219).micbias_on_event = true;

    /*
     * Mic bias pulse required to enable mic, must be done before enabling
     * button detection to prevent erroneous button readings.
     */
    if (*da7219_aad).micbias_pulse_lvl != 0 && (*da7219_aad).micbias_pulse_time != 0 {
        /* Pulse higher level voltage */
        micbias_ctrl = snd_soc_component_read(component, DA7219_MICBIAS_CTRL) as u8;
        snd_soc_component_update_bits(
            component,
            DA7219_MICBIAS_CTRL,
            DA7219_MICBIAS1_LEVEL_MASK,
            (*da7219_aad).micbias_pulse_lvl as c_uint,
        );
        msleep((*da7219_aad).micbias_pulse_time);
        snd_soc_component_write(component, DA7219_MICBIAS_CTRL, micbias_ctrl as c_uint);
    }

    snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, DA7219_BUTTON_CONFIG_MASK, (*da7219_aad).btn_cfg as c_uint);
}

unsafe extern "C" fn da7219_aad_hptest_work(work: *mut work_struct) {
    let da7219_aad = da7219_aad_from_hptest_work(work);
    let component = (*da7219_aad).component;
    let dapm = snd_soc_component_to_dapm(component);
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;

    let tonegen_freq_hptest: __le16;
    let pll_srm_sts: u8;
    let pll_ctrl: u8;
    let gain_ramp_ctrl: u8;
    let accdet_cfg8: u8;
    let mut report: c_int = 0;
    let ret: c_int;

    /* Lock DAPM, Kcontrols affected by this test and the PLL */
    snd_soc_dapm_mutex_lock(dapm);
    mutex_lock(&mut (*da7219).ctrl_lock);
    mutex_lock(&mut (*da7219).pll_lock);

    /* Ensure MCLK is available for HP test procedure */
    if !(*da7219).mclk.is_null() {
        ret = clk_prepare_enable((*da7219).mclk);
        if ret != 0 {
            dev_err((*component).dev, b"Failed to enable mclk - %d\n\0".as_ptr() as *const c_char, ret);
            mutex_unlock(&mut (*da7219).pll_lock);
            mutex_unlock(&mut (*da7219).ctrl_lock);
            snd_soc_dapm_mutex_unlock(dapm);
            return;
        }
    }

    /*
     * If MCLK not present, then we're using the internal oscillator and
     * require different frequency settings to achieve the same result.
     *
     * If MCLK is present, but PLL is not enabled then we enable it here to
     * ensure a consistent detection procedure.
     */
    pll_srm_sts = snd_soc_component_read(component, DA7219_PLL_SRM_STS) as u8;
    if (pll_srm_sts as c_uint & DA7219_PLL_SRM_STS_MCLK) != 0 {
        tonegen_freq_hptest = cpu_to_le16(DA7219_AAD_HPTEST_RAMP_FREQ);

        pll_ctrl = snd_soc_component_read(component, DA7219_PLL_CTRL) as u8;
        if (pll_ctrl as c_uint & DA7219_PLL_MODE_MASK) == DA7219_PLL_MODE_BYPASS {
            da7219_set_pll(component, DA7219_SYSCLK_PLL, DA7219_PLL_FREQ_OUT_98304);
        }
    } else {
        tonegen_freq_hptest = cpu_to_le16(DA7219_AAD_HPTEST_RAMP_FREQ_INT_OSC);
        pll_ctrl = 0;
    }

    /* Ensure gain ramping at fastest rate */
    gain_ramp_ctrl = snd_soc_component_read(component, DA7219_GAIN_RAMP_CTRL) as u8;
    snd_soc_component_write(component, DA7219_GAIN_RAMP_CTRL, DA7219_GAIN_RAMP_RATE_X8);

    /* Bypass cache so it saves current settings */
    regcache_cache_bypass((*da7219).regmap, true);

    /* Make sure Tone Generator is disabled */
    snd_soc_component_write(component, DA7219_TONE_GEN_CFG1, 0);

    /* Enable HPTest block, 1KOhms check */
    snd_soc_component_update_bits(
        component,
        DA7219_ACCDET_CONFIG_8,
        DA7219_HPTEST_EN_MASK | DA7219_HPTEST_RES_SEL_MASK,
        DA7219_HPTEST_EN_MASK | DA7219_HPTEST_RES_SEL_1KOHMS,
    );

    /* Set gains to 0db */
    snd_soc_component_write(component, DA7219_DAC_L_GAIN, DA7219_DAC_DIGITAL_GAIN_0DB);
    snd_soc_component_write(component, DA7219_DAC_R_GAIN, DA7219_DAC_DIGITAL_GAIN_0DB);
    snd_soc_component_write(component, DA7219_HP_L_GAIN, DA7219_HP_AMP_GAIN_0DB);
    snd_soc_component_write(component, DA7219_HP_R_GAIN, DA7219_HP_AMP_GAIN_0DB);

    /* Disable DAC filters, EQs and soft mute */
    snd_soc_component_update_bits(component, DA7219_DAC_FILTERS1, DA7219_HPF_MODE_MASK, 0);
    snd_soc_component_update_bits(component, DA7219_DAC_FILTERS4, DA7219_DAC_EQ_EN_MASK, 0);
    snd_soc_component_update_bits(component, DA7219_DAC_FILTERS5, DA7219_DAC_SOFTMUTE_EN_MASK, 0);

    /* Enable HP left & right paths */
    snd_soc_component_update_bits(component, DA7219_CP_CTRL, DA7219_CP_EN_MASK, DA7219_CP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_DIG_ROUTING_DAC, DA7219_DAC_L_SRC_MASK | DA7219_DAC_R_SRC_MASK, DA7219_DAC_L_SRC_TONEGEN | DA7219_DAC_R_SRC_TONEGEN);
    snd_soc_component_update_bits(component, DA7219_DAC_L_CTRL, DA7219_DAC_L_EN_MASK | DA7219_DAC_L_MUTE_EN_MASK, DA7219_DAC_L_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_DAC_R_CTRL, DA7219_DAC_R_EN_MASK | DA7219_DAC_R_MUTE_EN_MASK, DA7219_DAC_R_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_MIXOUT_L_SELECT, DA7219_MIXOUT_L_MIX_SELECT_MASK, DA7219_MIXOUT_L_MIX_SELECT_MASK);
    snd_soc_component_update_bits(component, DA7219_MIXOUT_R_SELECT, DA7219_MIXOUT_R_MIX_SELECT_MASK, DA7219_MIXOUT_R_MIX_SELECT_MASK);
    snd_soc_component_update_bits(component, DA7219_DROUTING_ST_OUTFILT_1L, DA7219_OUTFILT_ST_1L_SRC_MASK, DA7219_DMIX_ST_SRC_OUTFILT1L);
    snd_soc_component_update_bits(component, DA7219_DROUTING_ST_OUTFILT_1R, DA7219_OUTFILT_ST_1R_SRC_MASK, DA7219_DMIX_ST_SRC_OUTFILT1R);
    snd_soc_component_update_bits(component, DA7219_MIXOUT_L_CTRL, DA7219_MIXOUT_L_AMP_EN_MASK, DA7219_MIXOUT_L_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_MIXOUT_R_CTRL, DA7219_MIXOUT_R_AMP_EN_MASK, DA7219_MIXOUT_R_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_OE_MASK | DA7219_HP_L_AMP_EN_MASK, DA7219_HP_L_AMP_OE_MASK | DA7219_HP_L_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_OE_MASK | DA7219_HP_R_AMP_EN_MASK, DA7219_HP_R_AMP_OE_MASK | DA7219_HP_R_AMP_EN_MASK);
    msleep(DA7219_SETTLING_DELAY);
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_MUTE_EN_MASK | DA7219_HP_L_AMP_MIN_GAIN_EN_MASK, 0);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_MUTE_EN_MASK | DA7219_HP_R_AMP_MIN_GAIN_EN_MASK, 0);

    /*
     * If we're running from the internal oscillator then give audio paths
     * time to settle before running test.
     */
    if (pll_srm_sts as c_uint & DA7219_PLL_SRM_STS_MCLK) == 0 {
        msleep(DA7219_AAD_HPTEST_INT_OSC_PATH_DELAY);
    }

    /* Configure & start Tone Generator */
    snd_soc_component_write(component, DA7219_TONE_GEN_ON_PER, DA7219_BEEP_ON_PER_MASK);
    regmap_raw_write((*da7219).regmap, DA7219_TONE_GEN_FREQ1_L, &tonegen_freq_hptest as *const _ as *const c_void, size_of::<__le16>());
    snd_soc_component_update_bits(component, DA7219_TONE_GEN_CFG2, DA7219_SWG_SEL_MASK | DA7219_TONE_GEN_GAIN_MASK, DA7219_SWG_SEL_SRAMP | DA7219_TONE_GEN_GAIN_MINUS_15DB);
    snd_soc_component_write(component, DA7219_TONE_GEN_CFG1, DA7219_START_STOPN_MASK);

    msleep(DA7219_AAD_HPTEST_PERIOD);

    /* Grab comparator reading */
    accdet_cfg8 = snd_soc_component_read(component, DA7219_ACCDET_CONFIG_8) as u8;
    if (accdet_cfg8 as c_uint & DA7219_HPTEST_COMP_MASK) != 0 {
        report |= SND_JACK_HEADPHONE;
    } else {
        report |= SND_JACK_LINEOUT;
    }

    /* Stop tone generator */
    snd_soc_component_write(component, DA7219_TONE_GEN_CFG1, 0);

    msleep(DA7219_AAD_HPTEST_PERIOD);

    /* Restore original settings from cache */
    regcache_mark_dirty((*da7219).regmap);
    regcache_sync_region((*da7219).regmap, DA7219_HP_L_CTRL, DA7219_HP_R_CTRL);
    msleep(DA7219_SETTLING_DELAY);
    regcache_sync_region((*da7219).regmap, DA7219_MIXOUT_L_CTRL, DA7219_MIXOUT_R_CTRL);
    regcache_sync_region((*da7219).regmap, DA7219_DROUTING_ST_OUTFILT_1L, DA7219_DROUTING_ST_OUTFILT_1R);
    regcache_sync_region((*da7219).regmap, DA7219_MIXOUT_L_SELECT, DA7219_MIXOUT_R_SELECT);
    regcache_sync_region((*da7219).regmap, DA7219_DAC_L_CTRL, DA7219_DAC_R_CTRL);
    regcache_sync_region((*da7219).regmap, DA7219_DIG_ROUTING_DAC, DA7219_DIG_ROUTING_DAC);
    regcache_sync_region((*da7219).regmap, DA7219_CP_CTRL, DA7219_CP_CTRL);
    regcache_sync_region((*da7219).regmap, DA7219_DAC_FILTERS5, DA7219_DAC_FILTERS5);
    regcache_sync_region((*da7219).regmap, DA7219_DAC_FILTERS4, DA7219_DAC_FILTERS1);
    regcache_sync_region((*da7219).regmap, DA7219_HP_L_GAIN, DA7219_HP_R_GAIN);
    regcache_sync_region((*da7219).regmap, DA7219_DAC_L_GAIN, DA7219_DAC_R_GAIN);
    regcache_sync_region((*da7219).regmap, DA7219_TONE_GEN_ON_PER, DA7219_TONE_GEN_ON_PER);
    regcache_sync_region((*da7219).regmap, DA7219_TONE_GEN_FREQ1_L, DA7219_TONE_GEN_FREQ1_U);
    regcache_sync_region((*da7219).regmap, DA7219_TONE_GEN_CFG1, DA7219_TONE_GEN_CFG2);

    regcache_cache_bypass((*da7219).regmap, false);

    /* Disable HPTest block */
    snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_8, DA7219_HPTEST_EN_MASK, 0);

    /*
     * If we're running from the internal oscillator then give audio paths
     * time to settle before allowing headphones to be driven as required.
     */
    if (pll_srm_sts as c_uint & DA7219_PLL_SRM_STS_MCLK) == 0 {
        msleep(DA7219_AAD_HPTEST_INT_OSC_PATH_DELAY);
    }

    /* Restore gain ramping rate */
    snd_soc_component_write(component, DA7219_GAIN_RAMP_CTRL, gain_ramp_ctrl as c_uint);

    /* Drive Headphones/lineout */
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_OE_MASK, DA7219_HP_L_AMP_OE_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_OE_MASK, DA7219_HP_R_AMP_OE_MASK);

    /* Restore PLL to previous configuration, if re-configured */
    if (pll_srm_sts as c_uint & DA7219_PLL_SRM_STS_MCLK) != 0
        && ((pll_ctrl as c_uint & DA7219_PLL_MODE_MASK) == DA7219_PLL_MODE_BYPASS)
    {
        da7219_set_pll(component, DA7219_SYSCLK_MCLK, 0);
    }

    /* Remove MCLK, if previously enabled */
    if !(*da7219).mclk.is_null() {
        clk_disable_unprepare((*da7219).mclk);
    }

    mutex_unlock(&mut (*da7219).pll_lock);
    mutex_unlock(&mut (*da7219).ctrl_lock);
    snd_soc_dapm_mutex_unlock(dapm);

    /*
     * Only send report if jack hasn't been removed during process,
     * otherwise it's invalid and we drop it.
     */
    if (*da7219_aad).jack_inserted {
        snd_soc_jack_report((*da7219_aad).jack, report, SND_JACK_HEADSET | SND_JACK_LINEOUT);
    }
}

unsafe extern "C" fn da7219_aad_jack_det_work(work: *mut work_struct) {
    let da7219_aad = da7219_aad_from_jack_det_work(work);
    let component = (*da7219_aad).component;

    /* Enable ground switch */
    snd_soc_component_update_bits(component, 0xFB, 0x01, 0x01);
}

/*
 * IRQ
 */

unsafe extern "C" fn da7219_aad_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let da7219_aad = data as *mut da7219_aad_priv;
    let component = (*da7219_aad).component;
    let dapm = snd_soc_component_to_dapm(component);
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let mut events: [u8; 2] = [0; 2];
    let statusa: u8;
    let mut i: c_int;
    let ret: c_int;
    let mut report: c_int = 0;
    let mut mask: c_int = 0;

    /* Read current IRQ events */
    ret = regmap_bulk_read((*da7219).regmap, DA7219_ACCDET_IRQ_EVENT_A, events.as_mut_ptr() as *mut c_void, DA7219_AAD_IRQ_REG_MAX);
    if ret != 0 {
        dev_warn_ratelimited((*component).dev, b"Failed to read IRQ events: %d\n\0".as_ptr() as *const c_char, ret);
        return IRQ_NONE;
    }

    if events[DA7219_AAD_IRQ_REG_A] == 0 && events[DA7219_AAD_IRQ_REG_B] == 0 {
        return IRQ_NONE;
    }

    /* Read status register for jack insertion & type status */
    statusa = snd_soc_component_read(component, DA7219_ACCDET_STATUS_A) as u8;

    if (events[DA7219_AAD_IRQ_REG_A] as c_uint & DA7219_E_JACK_INSERTED_MASK) != 0 {
        let srm_st: u8;
        let mut delay: c_int = 0;

        srm_st = (snd_soc_component_read(component, DA7219_PLL_SRM_STS) & DA7219_PLL_SRM_STS_MCLK) as u8;
        delay = (*da7219_aad).gnd_switch_delay * if srm_st == 0x0 { 2 } else { 1 } - 2;
        queue_delayed_work((*da7219_aad).aad_wq, &mut (*da7219_aad).jack_det_work, msecs_to_jiffies(delay));
    }

    /* Clear events */
    regmap_bulk_write((*da7219).regmap, DA7219_ACCDET_IRQ_EVENT_A, events.as_ptr() as *const c_void, DA7219_AAD_IRQ_REG_MAX);

    dev_dbg(
        (*component).dev,
        b"IRQ events = 0x%x|0x%x, status = 0x%x\n\0".as_ptr() as *const c_char,
        events[DA7219_AAD_IRQ_REG_A] as c_int,
        events[DA7219_AAD_IRQ_REG_B] as c_int,
        statusa as c_int,
    );

    if (statusa as c_uint & DA7219_JACK_INSERTION_STS_MASK) != 0 {
        /* Jack Insertion */
        if (events[DA7219_AAD_IRQ_REG_A] as c_uint & DA7219_E_JACK_INSERTED_MASK) != 0 {
            report |= SND_JACK_MECHANICAL;
            mask |= SND_JACK_MECHANICAL;
            (*da7219_aad).jack_inserted = true;
        }

        /* Jack type detection */
        if (events[DA7219_AAD_IRQ_REG_A] as c_uint & DA7219_E_JACK_DETECT_COMPLETE_MASK) != 0 {
            /*
             * If 4-pole, then enable button detection, else perform
             * HP impedance test to determine output type to report.
             *
             * We schedule work here as the tasks themselves can
             * take time to complete, and in particular for hptest
             * we want to be able to check if the jack was removed
             * during the procedure as this will invalidate the
             * result. By doing this as work, the IRQ thread can
             * handle a removal, and we can check at the end of
             * hptest if we have a valid result or not.
             */

            cancel_delayed_work_sync(&mut (*da7219_aad).jack_det_work);
            /* Disable ground switch */
            snd_soc_component_update_bits(component, 0xFB, 0x01, 0x00);

            if (statusa as c_uint & DA7219_JACK_TYPE_STS_MASK) != 0 {
                report |= SND_JACK_HEADSET;
                mask |= SND_JACK_HEADSET | SND_JACK_LINEOUT;
                queue_work((*da7219_aad).aad_wq, &mut (*da7219_aad).btn_det_work);
            } else {
                queue_work((*da7219_aad).aad_wq, &mut (*da7219_aad).hptest_work);
            }
        }

        /* Button support for 4-pole jack */
        if (statusa as c_uint & DA7219_JACK_TYPE_STS_MASK) != 0 {
            i = 0;
            while i < DA7219_AAD_MAX_BUTTONS {
                /* Button Press */
                if (events[DA7219_AAD_IRQ_REG_B] as c_uint & (DA7219_E_BUTTON_A_PRESSED_MASK << i)) != 0 {
                    report |= SND_JACK_BTN_0 >> i;
                    mask |= SND_JACK_BTN_0 >> i;
                }
                i += 1;
            }
            snd_soc_jack_report((*da7219_aad).jack, report, mask);

            i = 0;
            while i < DA7219_AAD_MAX_BUTTONS {
                /* Button Release */
                if (events[DA7219_AAD_IRQ_REG_B] as c_uint & (DA7219_E_BUTTON_A_RELEASED_MASK >> i)) != 0 {
                    report &= !(SND_JACK_BTN_0 >> i);
                    mask |= SND_JACK_BTN_0 >> i;
                }
                i += 1;
            }
        }
    } else {
        /* Jack removal */
        if (events[DA7219_AAD_IRQ_REG_A] as c_uint & DA7219_E_JACK_REMOVED_MASK) != 0 {
            report = 0;
            mask |= DA7219_AAD_REPORT_ALL_MASK;
            (*da7219_aad).jack_inserted = false;

            /* Cancel any pending work */
            cancel_delayed_work_sync(&mut (*da7219_aad).jack_det_work);
            cancel_work_sync(&mut (*da7219_aad).btn_det_work);
            cancel_work_sync(&mut (*da7219_aad).hptest_work);

            /* Un-drive headphones/lineout */
            snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_OE_MASK, 0);
            snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_OE_MASK, 0);

            /* Ensure button detection disabled */
            snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, DA7219_BUTTON_CONFIG_MASK, 0);

            (*da7219).micbias_on_event = false;

            /* Disable mic bias */
            snd_soc_dapm_disable_pin(dapm, b"Mic Bias\0".as_ptr() as *const c_char);
            snd_soc_dapm_sync(dapm);

            /* Disable ground switch */
            snd_soc_component_update_bits(component, 0xFB, 0x01, 0x00);
        }
    }

    snd_soc_jack_report((*da7219_aad).jack, report, mask);

    IRQ_HANDLED
}

/*
 * DT/ACPI to pdata conversion
 */

unsafe fn da7219_aad_fw_micbias_pulse_lvl(dev: *mut device, val: u32) -> da7219_aad_micbias_pulse_lvl {
    match val {
        2800 => DA7219_AAD_MICBIAS_PULSE_LVL_2_8V,
        2900 => DA7219_AAD_MICBIAS_PULSE_LVL_2_9V,
        _ => {
            dev_warn(dev, b"Invalid micbias pulse level\0".as_ptr() as *const c_char);
            DA7219_AAD_MICBIAS_PULSE_LVL_OFF
        }
    }
}

unsafe fn da7219_aad_fw_btn_cfg(dev: *mut device, val: u32) -> da7219_aad_btn_cfg {
    match val {
        2 => DA7219_AAD_BTN_CFG_2MS,
        5 => DA7219_AAD_BTN_CFG_5MS,
        10 => DA7219_AAD_BTN_CFG_10MS,
        50 => DA7219_AAD_BTN_CFG_50MS,
        100 => DA7219_AAD_BTN_CFG_100MS,
        200 => DA7219_AAD_BTN_CFG_200MS,
        500 => DA7219_AAD_BTN_CFG_500MS,
        _ => {
            dev_warn(dev, b"Invalid button config\0".as_ptr() as *const c_char);
            DA7219_AAD_BTN_CFG_10MS
        }
    }
}

unsafe fn da7219_aad_fw_mic_det_thr(dev: *mut device, val: u32) -> da7219_aad_mic_det_thr {
    match val {
        200 => DA7219_AAD_MIC_DET_THR_200_OHMS,
        500 => DA7219_AAD_MIC_DET_THR_500_OHMS,
        750 => DA7219_AAD_MIC_DET_THR_750_OHMS,
        1000 => DA7219_AAD_MIC_DET_THR_1000_OHMS,
        _ => {
            dev_warn(dev, b"Invalid mic detect threshold\0".as_ptr() as *const c_char);
            DA7219_AAD_MIC_DET_THR_500_OHMS
        }
    }
}

unsafe fn da7219_aad_fw_jack_ins_deb(dev: *mut device, val: u32) -> da7219_aad_jack_ins_deb {
    match val {
        5 => DA7219_AAD_JACK_INS_DEB_5MS,
        10 => DA7219_AAD_JACK_INS_DEB_10MS,
        20 => DA7219_AAD_JACK_INS_DEB_20MS,
        50 => DA7219_AAD_JACK_INS_DEB_50MS,
        100 => DA7219_AAD_JACK_INS_DEB_100MS,
        200 => DA7219_AAD_JACK_INS_DEB_200MS,
        500 => DA7219_AAD_JACK_INS_DEB_500MS,
        1000 => DA7219_AAD_JACK_INS_DEB_1S,
        _ => {
            dev_warn(dev, b"Invalid jack insert debounce\0".as_ptr() as *const c_char);
            DA7219_AAD_JACK_INS_DEB_20MS
        }
    }
}

unsafe fn da7219_aad_fw_jack_ins_det_pty(dev: *mut device, str_: *const c_char) -> da7219_aad_jack_ins_det_pty {
    if strcmp(str_, b"low\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_INS_DET_PTY_LOW
    } else if strcmp(str_, b"high\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_INS_DET_PTY_HIGH
    } else {
        dev_warn(dev, b"Invalid jack insertion detection polarity\0".as_ptr() as *const c_char);
        DA7219_AAD_JACK_INS_DET_PTY_LOW
    }
}

unsafe fn da7219_aad_fw_jack_det_rate(dev: *mut device, str_: *const c_char) -> da7219_aad_jack_det_rate {
    if strcmp(str_, b"32_64\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_DET_RATE_32_64MS
    } else if strcmp(str_, b"64_128\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_DET_RATE_64_128MS
    } else if strcmp(str_, b"128_256\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_DET_RATE_128_256MS
    } else if strcmp(str_, b"256_512\0".as_ptr() as *const c_char) == 0 {
        DA7219_AAD_JACK_DET_RATE_256_512MS
    } else {
        dev_warn(dev, b"Invalid jack detect rate\0".as_ptr() as *const c_char);
        DA7219_AAD_JACK_DET_RATE_256_512MS
    }
}

unsafe fn da7219_aad_fw_jack_rem_deb(dev: *mut device, val: u32) -> da7219_aad_jack_rem_deb {
    match val {
        1 => DA7219_AAD_JACK_REM_DEB_1MS,
        5 => DA7219_AAD_JACK_REM_DEB_5MS,
        10 => DA7219_AAD_JACK_REM_DEB_10MS,
        20 => DA7219_AAD_JACK_REM_DEB_20MS,
        _ => {
            dev_warn(dev, b"Invalid jack removal debounce\0".as_ptr() as *const c_char);
            DA7219_AAD_JACK_REM_DEB_1MS
        }
    }
}

unsafe fn da7219_aad_fw_btn_avg(dev: *mut device, val: u32) -> da7219_aad_btn_avg {
    match val {
        1 => DA7219_AAD_BTN_AVG_1,
        2 => DA7219_AAD_BTN_AVG_2,
        4 => DA7219_AAD_BTN_AVG_4,
        8 => DA7219_AAD_BTN_AVG_8,
        _ => {
            dev_warn(dev, b"Invalid button average value\0".as_ptr() as *const c_char);
            DA7219_AAD_BTN_AVG_2
        }
    }
}

unsafe fn da7219_aad_fw_adc_1bit_rpt(dev: *mut device, val: u32) -> da7219_aad_adc_1bit_rpt {
    match val {
        1 => DA7219_AAD_ADC_1BIT_RPT_1,
        2 => DA7219_AAD_ADC_1BIT_RPT_2,
        4 => DA7219_AAD_ADC_1BIT_RPT_4,
        8 => DA7219_AAD_ADC_1BIT_RPT_8,
        _ => {
            dev_warn(dev, b"Invalid ADC 1-bit repeat value\0".as_ptr() as *const c_char);
            DA7219_AAD_ADC_1BIT_RPT_1
        }
    }
}

unsafe fn da7219_aad_fw_to_pdata(dev: *mut device) -> *mut da7219_aad_pdata {
    let i2c = to_i2c_client(dev);
    let aad_np: *mut fwnode_handle;
    let aad_pdata: *mut da7219_aad_pdata;
    let mut fw_str: *const c_char = ptr::null();
    let mut fw_val32: u32 = 0;

    aad_np = device_get_named_child_node(dev, b"da7219_aad\0".as_ptr() as *const c_char);
    if aad_np.is_null() {
        return ptr::null_mut();
    }

    aad_pdata = devm_kzalloc(dev, size_of::<da7219_aad_pdata>(), GFP_KERNEL) as *mut da7219_aad_pdata;
    if aad_pdata.is_null() {
        fwnode_handle_put(aad_np);
        return ptr::null_mut();
    }

    (*aad_pdata).irq = (*i2c).irq;

    if fwnode_property_read_u32(aad_np, b"dlg,micbias-pulse-lvl\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).micbias_pulse_lvl = da7219_aad_fw_micbias_pulse_lvl(dev, fw_val32);
    } else {
        (*aad_pdata).micbias_pulse_lvl = DA7219_AAD_MICBIAS_PULSE_LVL_OFF;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,micbias-pulse-time\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).micbias_pulse_time = fw_val32;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,btn-cfg\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).btn_cfg = da7219_aad_fw_btn_cfg(dev, fw_val32);
    } else {
        (*aad_pdata).btn_cfg = DA7219_AAD_BTN_CFG_10MS;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,mic-det-thr\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).mic_det_thr = da7219_aad_fw_mic_det_thr(dev, fw_val32);
    } else {
        (*aad_pdata).mic_det_thr = DA7219_AAD_MIC_DET_THR_200_OHMS;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,jack-ins-deb\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).jack_ins_deb = da7219_aad_fw_jack_ins_deb(dev, fw_val32);
    } else {
        (*aad_pdata).jack_ins_deb = DA7219_AAD_JACK_INS_DEB_20MS;
    }

    if fwnode_property_read_string(aad_np, b"dlg,jack-ins-det-pty\0".as_ptr() as *const c_char, &mut fw_str) == 0 {
        (*aad_pdata).jack_ins_det_pty = da7219_aad_fw_jack_ins_det_pty(dev, fw_str);
    } else {
        (*aad_pdata).jack_ins_det_pty = DA7219_AAD_JACK_INS_DET_PTY_LOW;
    }

    if fwnode_property_read_string(aad_np, b"dlg,jack-det-rate\0".as_ptr() as *const c_char, &mut fw_str) == 0 {
        (*aad_pdata).jack_det_rate = da7219_aad_fw_jack_det_rate(dev, fw_str);
    } else {
        (*aad_pdata).jack_det_rate = DA7219_AAD_JACK_DET_RATE_256_512MS;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,jack-rem-deb\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).jack_rem_deb = da7219_aad_fw_jack_rem_deb(dev, fw_val32);
    } else {
        (*aad_pdata).jack_rem_deb = DA7219_AAD_JACK_REM_DEB_1MS;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,a-d-btn-thr\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).a_d_btn_thr = fw_val32 as u8;
    } else {
        (*aad_pdata).a_d_btn_thr = 0xA;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,d-b-btn-thr\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).d_b_btn_thr = fw_val32 as u8;
    } else {
        (*aad_pdata).d_b_btn_thr = 0x16;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,b-c-btn-thr\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).b_c_btn_thr = fw_val32 as u8;
    } else {
        (*aad_pdata).b_c_btn_thr = 0x21;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,c-mic-btn-thr\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).c_mic_btn_thr = fw_val32 as u8;
    } else {
        (*aad_pdata).c_mic_btn_thr = 0x3E;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,btn-avg\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).btn_avg = da7219_aad_fw_btn_avg(dev, fw_val32);
    } else {
        (*aad_pdata).btn_avg = DA7219_AAD_BTN_AVG_2;
    }

    if fwnode_property_read_u32(aad_np, b"dlg,adc-1bit-rpt\0".as_ptr() as *const c_char, &mut fw_val32) >= 0 {
        (*aad_pdata).adc_1bit_rpt = da7219_aad_fw_adc_1bit_rpt(dev, fw_val32);
    } else {
        (*aad_pdata).adc_1bit_rpt = DA7219_AAD_ADC_1BIT_RPT_1;
    }

    fwnode_handle_put(aad_np);

    aad_pdata
}

unsafe fn da7219_aad_handle_pdata(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let pdata = (*da7219).pdata;

    if !pdata.is_null() && !(*pdata).aad_pdata.is_null() {
        let aad_pdata = (*pdata).aad_pdata;
        let mut cfg: u8;
        let mut mask: u8;

        (*da7219_aad).irq = (*aad_pdata).irq;

        match (*aad_pdata).micbias_pulse_lvl {
            x if x == DA7219_AAD_MICBIAS_PULSE_LVL_2_8V || x == DA7219_AAD_MICBIAS_PULSE_LVL_2_9V => {
                (*da7219_aad).micbias_pulse_lvl = ((*aad_pdata).micbias_pulse_lvl << DA7219_MICBIAS1_LEVEL_SHIFT) as u8;
            }
            _ => {}
        }

        (*da7219_aad).micbias_pulse_time = (*aad_pdata).micbias_pulse_time;

        match (*aad_pdata).btn_cfg {
            x if x == DA7219_AAD_BTN_CFG_2MS
                || x == DA7219_AAD_BTN_CFG_5MS
                || x == DA7219_AAD_BTN_CFG_10MS
                || x == DA7219_AAD_BTN_CFG_50MS
                || x == DA7219_AAD_BTN_CFG_100MS
                || x == DA7219_AAD_BTN_CFG_200MS
                || x == DA7219_AAD_BTN_CFG_500MS =>
            {
                (*da7219_aad).btn_cfg = ((*aad_pdata).btn_cfg << DA7219_BUTTON_CONFIG_SHIFT) as u8;
            }
            _ => {}
        }

        cfg = 0;
        mask = 0;
        match (*aad_pdata).mic_det_thr {
            x if x == DA7219_AAD_MIC_DET_THR_200_OHMS || x == DA7219_AAD_MIC_DET_THR_500_OHMS || x == DA7219_AAD_MIC_DET_THR_750_OHMS || x == DA7219_AAD_MIC_DET_THR_1000_OHMS => {
                cfg |= ((*aad_pdata).mic_det_thr << DA7219_MIC_DET_THRESH_SHIFT) as u8;
                mask |= DA7219_MIC_DET_THRESH_MASK as u8;
            }
            _ => {}
        }
        snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, mask as c_uint, cfg as c_uint);

        cfg = 0;
        mask = 0;
        match (*aad_pdata).jack_ins_deb {
            x if x == DA7219_AAD_JACK_INS_DEB_5MS
                || x == DA7219_AAD_JACK_INS_DEB_10MS
                || x == DA7219_AAD_JACK_INS_DEB_20MS
                || x == DA7219_AAD_JACK_INS_DEB_50MS
                || x == DA7219_AAD_JACK_INS_DEB_100MS
                || x == DA7219_AAD_JACK_INS_DEB_200MS
                || x == DA7219_AAD_JACK_INS_DEB_500MS
                || x == DA7219_AAD_JACK_INS_DEB_1S =>
            {
                cfg |= ((*aad_pdata).jack_ins_deb << DA7219_JACKDET_DEBOUNCE_SHIFT) as u8;
                mask |= DA7219_JACKDET_DEBOUNCE_MASK as u8;
            }
            _ => {}
        }
        match (*aad_pdata).jack_det_rate {
            x if x == DA7219_AAD_JACK_DET_RATE_32_64MS || x == DA7219_AAD_JACK_DET_RATE_64_128MS || x == DA7219_AAD_JACK_DET_RATE_128_256MS || x == DA7219_AAD_JACK_DET_RATE_256_512MS => {
                cfg |= ((*aad_pdata).jack_det_rate << DA7219_JACK_DETECT_RATE_SHIFT) as u8;
                mask |= DA7219_JACK_DETECT_RATE_MASK as u8;
            }
            _ => {}
        }
        match (*aad_pdata).jack_rem_deb {
            x if x == DA7219_AAD_JACK_REM_DEB_1MS || x == DA7219_AAD_JACK_REM_DEB_5MS || x == DA7219_AAD_JACK_REM_DEB_10MS || x == DA7219_AAD_JACK_REM_DEB_20MS => {
                cfg |= ((*aad_pdata).jack_rem_deb << DA7219_JACKDET_REM_DEB_SHIFT) as u8;
                mask |= DA7219_JACKDET_REM_DEB_MASK as u8;
            }
            _ => {}
        }
        snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_2, mask as c_uint, cfg as c_uint);

        snd_soc_component_write(component, DA7219_ACCDET_CONFIG_3, (*aad_pdata).a_d_btn_thr as c_uint);
        snd_soc_component_write(component, DA7219_ACCDET_CONFIG_4, (*aad_pdata).d_b_btn_thr as c_uint);
        snd_soc_component_write(component, DA7219_ACCDET_CONFIG_5, (*aad_pdata).b_c_btn_thr as c_uint);
        snd_soc_component_write(component, DA7219_ACCDET_CONFIG_6, (*aad_pdata).c_mic_btn_thr as c_uint);

        cfg = 0;
        mask = 0;
        match (*aad_pdata).btn_avg {
            x if x == DA7219_AAD_BTN_AVG_1 || x == DA7219_AAD_BTN_AVG_2 || x == DA7219_AAD_BTN_AVG_4 || x == DA7219_AAD_BTN_AVG_8 => {
                cfg |= ((*aad_pdata).btn_avg << DA7219_BUTTON_AVERAGE_SHIFT) as u8;
                mask |= DA7219_BUTTON_AVERAGE_MASK as u8;
            }
            _ => {}
        }
        match (*aad_pdata).adc_1bit_rpt {
            x if x == DA7219_AAD_ADC_1BIT_RPT_1 || x == DA7219_AAD_ADC_1BIT_RPT_2 || x == DA7219_AAD_ADC_1BIT_RPT_4 || x == DA7219_AAD_ADC_1BIT_RPT_8 => {
                cfg |= ((*aad_pdata).adc_1bit_rpt << DA7219_ADC_1_BIT_REPEAT_SHIFT) as u8;
                mask |= DA7219_ADC_1_BIT_REPEAT_MASK as u8;
            }
            _ => {}
        }
        snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_7, mask as c_uint, cfg as c_uint);

        match (*aad_pdata).jack_ins_det_pty {
            x if x == DA7219_AAD_JACK_INS_DET_PTY_LOW => {
                snd_soc_component_write(component, 0xF0, 0x8B);
                snd_soc_component_write(component, 0x75, 0x80);
                snd_soc_component_write(component, 0xF0, 0x00);
            }
            x if x == DA7219_AAD_JACK_INS_DET_PTY_HIGH => {
                snd_soc_component_write(component, 0xF0, 0x8B);
                snd_soc_component_write(component, 0x75, 0x00);
                snd_soc_component_write(component, 0xF0, 0x00);
            }
            _ => {}
        }
    }
}

unsafe fn da7219_aad_handle_gnd_switch_time(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let jack_det: u8;

    jack_det = (snd_soc_component_read(component, DA7219_ACCDET_CONFIG_2) & DA7219_JACK_DETECT_RATE_MASK) as u8;
    match jack_det {
        0x00 => (*da7219_aad).gnd_switch_delay = 32,
        0x10 => (*da7219_aad).gnd_switch_delay = 64,
        0x20 => (*da7219_aad).gnd_switch_delay = 128,
        0x30 => (*da7219_aad).gnd_switch_delay = 256,
        _ => (*da7219_aad).gnd_switch_delay = 32,
    }
}

/*
 * Suspend/Resume
 *
 * Original C guarded these functions with CONFIG_PM.
 */

#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn da7219_aad_suspend(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let dapm = snd_soc_component_to_dapm(component);
    let micbias_ctrl: u8;

    disable_irq((*da7219_aad).irq);

    if !(*da7219_aad).jack.is_null() {
        /* Disable jack detection during suspend */
        snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, DA7219_ACCDET_EN_MASK, 0);
        cancel_delayed_work_sync(&mut (*da7219_aad).jack_det_work);
        /* Disable ground switch */
        snd_soc_component_update_bits(component, 0xFB, 0x01, 0x00);

        /*
         * If we have a 4-pole jack inserted, then micbias will be
         * enabled. We can disable micbias here, and keep a note to
         * re-enable it on resume. If jack removal occurred during
         * suspend then this will be dealt with through the IRQ handler.
         */
        if (*da7219_aad).jack_inserted {
            micbias_ctrl = snd_soc_component_read(component, DA7219_MICBIAS_CTRL) as u8;
            if (micbias_ctrl as c_uint & DA7219_MICBIAS1_EN_MASK) != 0 {
                snd_soc_dapm_disable_pin(dapm, b"Mic Bias\0".as_ptr() as *const c_char);
                snd_soc_dapm_sync(dapm);
                (*da7219_aad).micbias_resume_enable = true;
            }
        }
    }
}

#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn da7219_aad_resume(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let dapm = snd_soc_component_to_dapm(component);

    if !(*da7219_aad).jack.is_null() {
        /* Re-enable micbias if previously enabled for 4-pole jack */
        if (*da7219_aad).jack_inserted && (*da7219_aad).micbias_resume_enable {
            snd_soc_dapm_force_enable_pin(dapm, b"Mic Bias\0".as_ptr() as *const c_char);
            snd_soc_dapm_sync(dapm);
            (*da7219_aad).micbias_resume_enable = false;
        }

        /* Re-enable jack detection */
        snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, DA7219_ACCDET_EN_MASK, DA7219_ACCDET_EN_MASK);
    }

    enable_irq((*da7219_aad).irq);
}

/*
 * Init/Exit
 */

#[no_mangle]
pub unsafe extern "C" fn da7219_aad_init(component: *mut snd_soc_component) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let mut mask: [u8; 2] = [0; 2];
    let ret: c_int;

    (*da7219_aad).component = component;

    /* Handle any DT/ACPI/platform data */
    da7219_aad_handle_pdata(component);

    /* Disable button detection */
    snd_soc_component_update_bits(component, DA7219_ACCDET_CONFIG_1, DA7219_BUTTON_CONFIG_MASK, 0);

    da7219_aad_handle_gnd_switch_time(component);

    (*da7219_aad).aad_wq = create_singlethread_workqueue(b"da7219-aad\0".as_ptr() as *const c_char);
    if (*da7219_aad).aad_wq.is_null() {
        dev_err((*component).dev, b"Failed to create aad workqueue\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    INIT_DELAYED_WORK(&mut (*da7219_aad).jack_det_work, da7219_aad_jack_det_work);
    INIT_WORK(&mut (*da7219_aad).btn_det_work, da7219_aad_btn_det_work);
    INIT_WORK(&mut (*da7219_aad).hptest_work, da7219_aad_hptest_work);

    ret = request_threaded_irq(
        (*da7219_aad).irq,
        None,
        da7219_aad_irq_thread,
        IRQF_TRIGGER_LOW | IRQF_ONESHOT,
        b"da7219-aad\0".as_ptr() as *const c_char,
        da7219_aad as *mut c_void,
    );
    if ret != 0 {
        dev_err((*component).dev, b"Failed to request IRQ: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* Unmask AAD IRQs */
    mask.fill(0);
    regmap_bulk_write((*da7219).regmap, DA7219_ACCDET_IRQ_MASK_A, mask.as_ptr() as *const c_void, DA7219_AAD_IRQ_REG_MAX);

    0
}

#[no_mangle]
pub unsafe extern "C" fn da7219_aad_exit(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component) as *mut da7219_priv;
    let da7219_aad = (*da7219).aad;
    let mut mask: [u8; 2] = [0; 2];

    /* Mask off AAD IRQs */
    mask.fill(DA7219_BYTE_MASK as u8);
    regmap_bulk_write((*da7219).regmap, DA7219_ACCDET_IRQ_MASK_A, mask.as_ptr() as *const c_void, DA7219_AAD_IRQ_REG_MAX);

    free_irq((*da7219_aad).irq, da7219_aad as *mut c_void);

    cancel_delayed_work_sync(&mut (*da7219_aad).jack_det_work);
    cancel_work_sync(&mut (*da7219_aad).btn_det_work);
    cancel_work_sync(&mut (*da7219_aad).hptest_work);
    destroy_workqueue((*da7219_aad).aad_wq);
}

/*
 * AAD related I2C probe handling
 */

#[no_mangle]
pub unsafe extern "C" fn da7219_aad_probe(i2c: *mut i2c_client) -> c_int {
    let da7219 = i2c_get_clientdata(i2c) as *mut da7219_priv;
    let dev = &mut (*i2c).dev as *mut device;
    let da7219_aad: *mut da7219_aad_priv;

    da7219_aad = devm_kzalloc(dev, size_of::<da7219_aad_priv>(), GFP_KERNEL) as *mut da7219_aad_priv;
    if da7219_aad.is_null() {
        return -ENOMEM;
    }

    (*da7219).aad = da7219_aad;

    /* Retrieve any DT/ACPI/platform data */
    if !(*da7219).pdata.is_null() && (*(*da7219).pdata).aad_pdata.is_null() {
        (*(*da7219).pdata).aad_pdata = da7219_aad_fw_to_pdata(dev);
    }

    0
}

/* MODULE_DESCRIPTION("ASoC DA7219 AAD Driver"); */
/* MODULE_AUTHOR("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>"); */
/* MODULE_AUTHOR("David Rau <David.Rau.opensource@dm.renesas.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
