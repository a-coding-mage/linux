// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2015-2021, The Linux Foundation. All rights reserved.

// Translated from C implementation source. Kernel, ALSA SoC, and local header
// dependencies are declared as external/opaque items expected from other files.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = u8;
pub type u32 = u32;
pub type uint32_t = u32;
pub type bool_t = bool;
pub type irqreturn_t = c_int;

pub const HS_DETECT_PLUG_TIME_MS: c_int = 3 * 1000;
pub const MBHC_BUTTON_PRESS_THRESHOLD_MIN: c_ulong = 250;
pub const GND_MIC_SWAP_THRESHOLD: c_uint = 4;
pub const GND_MIC_USBC_SWAP_THRESHOLD: c_uint = 2;
pub const WCD_FAKE_REMOVAL_MIN_PERIOD_MS: c_int = 100;
pub const HPHL_CROSS_CONN_THRESHOLD: c_int = 100;
pub const HS_VREF_MIN_VAL: u32 = 1400;
pub const FAKE_REM_RETRY_ATTEMPTS: c_int = 3;
pub const WCD_MBHC_ADC_HS_THRESHOLD_MV: c_int = 1700;
pub const WCD_MBHC_ADC_HPH_THRESHOLD_MV: c_int = 75;
pub const WCD_MBHC_ADC_MICBIAS_MV: c_int = 1800;
pub const WCD_MBHC_FAKE_INS_RETRY: u8 = 4;

pub const WCD_MBHC_JACK_MASK: c_int = SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_MECHANICAL;
pub const WCD_MBHC_JACK_BUTTON_MASK: c_int =
    SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4 | SND_JACK_BTN_5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcd_mbhc_adc_mux_ctl {
    MUX_CTL_AUTO = 0,
    MUX_CTL_IN2P,
    MUX_CTL_IN3P,
    MUX_CTL_IN4P,
    MUX_CTL_HPH_L,
    MUX_CTL_HPH_R,
    MUX_CTL_NONE,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_jack {
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
pub struct mutex {
    _private: [u8; 0],
}

pub type snd_jack_types = c_int;
pub type wcd_mbhc_hph_type = c_int;
pub type wcd_mbhc_cs_mb_en_flag = c_int;
pub type wcd_mbhc_plug_type = c_int;

#[repr(C)]
pub struct wcd_mbhc_field {
    pub reg: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct wcd_mbhc_intr {
    pub mbhc_sw_intr: c_int,
    pub mbhc_btn_press_intr: c_int,
    pub mbhc_btn_release_intr: c_int,
    pub mbhc_hs_ins_intr: c_int,
    pub mbhc_hs_rem_intr: c_int,
    pub hph_left_ocp: c_int,
    pub hph_right_ocp: c_int,
}

#[repr(C)]
pub struct wcd_mbhc_config {
    pub btn_low: *mut c_int,
    pub btn_high: [c_int; WCD_MBHC_DEF_BUTTONS as usize],
    pub num_btn: c_int,
    pub v_hs_max: u32,
    pub linein_th: u32,
    pub hphl_swh: bool,
    pub gnd_swh: bool,
    pub gnd_det_en: bool,
    pub typec_analog_mux: bool,
    pub hs_thr: c_int,
    pub hph_thr: c_int,
    pub micb_mv: c_int,
    pub swap_gnd_mic: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct wcd_mbhc_cb {
    pub set_btn_thr: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut c_int, *mut c_int, c_int, bool)>,
    pub mbhc_micbias_control: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int)>,
    pub mbhc_micb_ctrl_thr_mic: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, bool)>,
    pub set_micbias_value: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub compute_impedance: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut uint32_t, *mut uint32_t)>,
    pub hph_pull_down_ctrl: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
    pub mbhc_micb_ramp_control: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
    pub mbhc_bias: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
    pub micbias_enable_status: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int) -> bool>,
    pub hph_pull_up_control_v2: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int)>,
    pub hph_pull_up_control: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int)>,
    pub mbhc_gnd_det_ctrl: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
    pub clk_setup: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
    pub get_micbias_val: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut c_int)>,
    pub bcs_enable: Option<unsafe extern "C" fn(*mut snd_soc_component, bool)>,
}

#[repr(C)]
pub struct wcd_mbhc {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub jack: *mut snd_soc_jack,
    pub cfg: *mut wcd_mbhc_config,
    pub mbhc_cb: *const wcd_mbhc_cb,
    pub intr_ids: *const wcd_mbhc_intr,
    pub fields: *const wcd_mbhc_field,
    /* Delayed work to report long button press */
    pub mbhc_btn_dwork: delayed_work,
    /* Work to handle plug report */
    pub mbhc_plug_detect_work: work_struct,
    /* Work to correct accessory type */
    pub correct_plug_swch: work_struct,
    pub lock: mutex,
    pub buttons_pressed: c_int,
    pub hph_status: u32, /* track headhpone status */
    pub current_plug: u8,
    pub swap_thr: c_uint,
    pub is_btn_press: bool,
    pub in_swch_irq_handler: bool,
    pub hs_detect_work_stop: bool,
    pub is_hs_recording: bool,
    pub extn_cable_hph_rem: bool,
    pub force_linein: bool,
    pub impedance_detect: bool,
    pub event_state: c_ulong,
    pub jiffies_atreport: c_ulong,
    /* impedance of hphl and hphr */
    pub zl: uint32_t,
    pub zr: uint32_t,
    /* Holds type of Headset - Mono/Stereo */
    pub hph_type: wcd_mbhc_hph_type,
    /* Holds mbhc detection method - ADC/Legacy */
    pub mbhc_detection_logic: c_int,
}

extern "C" {
    static mut jiffies: c_ulong;
    fn snd_soc_component_write_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_int) -> c_int;
    fn snd_soc_component_read_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> c_int;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn jiffies_to_msecs(j: c_ulong) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msleep(msecs: c_uint);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn mutex_is_locked(lock: *mut mutex) -> c_int;
    fn disable_irq_nosync(irq: c_int);
    fn enable_irq(irq: c_int);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn request_threaded_irq(
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn ERR_PTR(error: c_long) -> *mut wcd_mbhc;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut c_int) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut c_int, sz: usize) -> c_int;
}

pub type c_long = isize;

extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn WARN_ON(condition: c_int) -> c_int;
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> c_int;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool;
    fn set_bit(nr: c_int, addr: *mut c_ulong);
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
}

extern "C" {
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_LINEOUT: c_int;
    static SND_JACK_MECHANICAL: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static SND_JACK_BTN_5: c_int;
}

pub const SND_JACK_HEADPHONE: c_int = SND_JACK_HEADSET;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const EINVAL: c_int = 22;
pub const EACCES: c_int = 13;
pub const ENOMEM: c_int = 12;
pub const GFP_KERNEL: c_uint = 0;
pub const IRQF_ONESHOT: c_ulong = 0;
pub const IRQF_TRIGGER_RISING: c_ulong = 0;

extern "C" {
    static WCD_MBHC_HS_VREF: c_int;
    static WCD_MBHC_MICB_CTRL: c_int;
    static WCD_MBHC_BTN_ISRC_CTL: c_int;
    static WCD_MBHC_FSM_EN: c_int;
    static WCD_MBHC_EVENT_PA_HPHL: c_int;
    static WCD_MBHC_EVENT_PA_HPHR: c_int;
    static MIC_BIAS_2: c_int;
    static MICB_DISABLE: c_int;
    static MICB_ENABLE: c_int;
    static WCD_MBHC_HPH_NONE: c_int;
    static MBHC_PLUG_TYPE_NONE: u8;
    static MBHC_PLUG_TYPE_HEADPHONE: u8;
    static MBHC_PLUG_TYPE_HEADSET: u8;
    static MBHC_PLUG_TYPE_HIGH_HPH: u8;
    static MBHC_PLUG_TYPE_GND_MIC_SWAP: u8;
    static MBHC_PLUG_TYPE_INVALID: u8;
    static WCD_MBHC_MUX_CTL: c_int;
    static WCD_MBHC_HPH_PA_EN: c_int;
    static WCD_MBHC_ELECT_SCHMT_ISRC: c_int;
    static WCD_MBHC_ELECT_DETECTION_TYPE: c_int;
    static WCD_MBHC_MECH_DETECTION_TYPE: c_int;
    static WCD_DETECTION_ADC: c_int;
    static WCD_MBHC_ELECT_ISRC_EN: c_int;
    static WCD_MBHC_DETECTION_DONE: c_int;
    static WCD_MBHC_L_DET_EN: c_int;
    static WCD_MBHC_BTN_RESULT: c_int;
    static WCD_MBHC_OCP_FSM_EN: c_int;
    static HS_PULLUP_I_OFF: c_int;
    static HS_PULLUP_I_DEFAULT: c_int;
    static I_OFF: c_int;
    static I_DEFAULT: c_int;
    static WCD_MBHC_HS_L_DET_PULL_UP_CTRL: c_int;
    static WCD_MBHC_HPHL_PLUG_TYPE: c_int;
    static WCD_MBHC_GND_PLUG_TYPE: c_int;
    static WCD_MBHC_SW_HPH_LP_100K_TO_GND: c_int;
    static WCD_MBHC_HS_L_DET_PULL_UP_COMP_CTRL: c_int;
    static WCD_MBHC_INSREM_DBNC: c_int;
    static WCD_MBHC_BTN_DBNC: c_int;
    static WCD_MBHC_MICB2_VOUT: c_int;
    static WCD_MBHC_ADC_MODE: c_int;
    static WCD_MBHC_ADC_EN: c_int;
    static WCD_MBHC_ADC_RESULT: c_int;
    static WCD_MBHC_ADC_TIMEOUT: c_int;
    static WCD_MBHC_ADC_COMPLETE: c_int;
    static WCD_MBHC_IN2P_CLAMP_STATE: c_int;
    static WCD_MBHC_DEF_BUTTONS: c_int;
    static WCD_EVENT_POST_DAPM_MICBIAS_2_ON: c_ulong;
    static WCD_EVENT_POST_MICBIAS_2_ON: c_ulong;
    static WCD_EVENT_PRE_MICBIAS_2_OFF: c_ulong;
    static WCD_EVENT_POST_DAPM_MICBIAS_2_OFF: c_ulong;
    static WCD_EVENT_POST_MICBIAS_2_OFF: c_ulong;
    static WCD_EVENT_POST_HPHL_PA_OFF: c_ulong;
    static WCD_EVENT_POST_HPHR_PA_OFF: c_ulong;
    static WCD_EVENT_PRE_HPHL_PA_ON: c_ulong;
    static WCD_EVENT_PRE_HPHR_PA_ON: c_ulong;
}

unsafe fn container_of_work(_work: *mut work_struct) -> *mut wcd_mbhc {
    // TODO: external kernel container_of layout mapping.
    core::ptr::null_mut()
}

unsafe fn container_of_delayed_work(_dwork: *mut delayed_work) -> *mut wcd_mbhc {
    // TODO: external kernel container_of layout mapping.
    core::ptr::null_mut()
}

unsafe fn to_delayed_work(work: *mut work_struct) -> *mut delayed_work {
    work as *mut delayed_work
}

unsafe fn wcd_mbhc_write_field(mbhc: *const wcd_mbhc, field: c_int, val: c_int) -> c_int {
    if (*(*mbhc).fields.add(field as usize)).reg == 0 {
        return 0;
    }
    snd_soc_component_write_field(
        (*mbhc).component,
        (*(*mbhc).fields.add(field as usize)).reg,
        (*(*mbhc).fields.add(field as usize)).mask,
        val,
    )
}

unsafe fn wcd_mbhc_read_field(mbhc: *const wcd_mbhc, field: c_int) -> c_int {
    if (*(*mbhc).fields.add(field as usize)).reg == 0 {
        return 0;
    }
    snd_soc_component_read_field(
        (*mbhc).component,
        (*(*mbhc).fields.add(field as usize)).reg,
        (*(*mbhc).fields.add(field as usize)).mask,
    )
}

unsafe fn wcd_program_hs_vref(mbhc: *mut wcd_mbhc) {
    let reg_val: u32 = ((*(*mbhc).cfg).v_hs_max - HS_VREF_MIN_VAL) / 100;
    wcd_mbhc_write_field(mbhc, WCD_MBHC_HS_VREF, reg_val as c_int);
}

unsafe fn wcd_program_btn_threshold(mbhc: *const wcd_mbhc, micbias: bool) {
    let component = (*mbhc).component;
    ((*(*mbhc).mbhc_cb).set_btn_thr.unwrap())(
        component,
        (*(*mbhc).cfg).btn_low,
        (*(*mbhc).cfg).btn_high.as_mut_ptr(),
        (*(*mbhc).cfg).num_btn,
        micbias,
    );
}

unsafe fn wcd_mbhc_curr_micbias_control(mbhc: *const wcd_mbhc, cs_mb_en: wcd_mbhc_cs_mb_en_flag) {
    /*
     * Some codecs handle micbias/pullup enablement in codec
     * drivers itself and micbias is not needed for regular
     * plug type detection. So if micbias_control callback function
     * is defined, just return.
     */
    if (*(*mbhc).mbhc_cb).mbhc_micbias_control.is_some() {
        return;
    }
    match cs_mb_en {
        WCD_MBHC_EN_CS => {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_MICB_CTRL, 0);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 3);
            /* Program Button threshold registers as per CS */
            wcd_program_btn_threshold(mbhc, false);
        }
        WCD_MBHC_EN_MB => {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
            /* Disable PULL_UP_EN & enable MICBIAS */
            wcd_mbhc_write_field(mbhc, WCD_MBHC_MICB_CTRL, 2);
            /* Program Button threshold registers as per MICBIAS */
            wcd_program_btn_threshold(mbhc, true);
        }
        WCD_MBHC_EN_PULLUP => {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 3);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_MICB_CTRL, 1);
            /* Program Button threshold registers as per MICBIAS */
            wcd_program_btn_threshold(mbhc, true);
        }
        WCD_MBHC_EN_NONE => {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_MICB_CTRL, 0);
        }
        _ => dev_err((*mbhc).dev, c"%s: Invalid parameter".as_ptr(), c"wcd_mbhc_curr_micbias_control".as_ptr()),
    }
}

pub const WCD_MBHC_EN_CS: c_int = 0;
pub const WCD_MBHC_EN_MB: c_int = 1;
pub const WCD_MBHC_EN_PULLUP: c_int = 2;
pub const WCD_MBHC_EN_NONE: c_int = 3;

pub unsafe extern "C" fn wcd_mbhc_event_notify(mbhc: *mut wcd_mbhc, event: c_ulong) -> c_int {
    let component: *mut snd_soc_component;
    let mut micbias2 = false;
    if mbhc.is_null() {
        return 0;
    }
    component = (*mbhc).component;
    if let Some(cb) = (*(*mbhc).mbhc_cb).micbias_enable_status {
        micbias2 = cb(component, MIC_BIAS_2);
    }
    if event == WCD_EVENT_POST_DAPM_MICBIAS_2_ON {
        (*mbhc).is_hs_recording = true;
    } else if event == WCD_EVENT_POST_MICBIAS_2_ON {
        /* Disable current source if micbias2 enabled */
        if (*(*mbhc).mbhc_cb).mbhc_micbias_control.is_some() {
            if wcd_mbhc_read_field(mbhc, WCD_MBHC_FSM_EN) != 0 {
                wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
            }
        } else {
            (*mbhc).is_hs_recording = true;
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_MB);
        }
    } else if event == WCD_EVENT_PRE_MICBIAS_2_OFF {
        /*
         * Before MICBIAS_2 is turned off, if FSM is enabled,
         * make sure current source is enabled so as to detect
         * button press/release events
         */
        if (*(*mbhc).mbhc_cb).mbhc_micbias_control.is_some() {
            if wcd_mbhc_read_field(mbhc, WCD_MBHC_FSM_EN) != 0 {
                wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 3);
            }
        }
    } else if event == WCD_EVENT_POST_DAPM_MICBIAS_2_OFF {
        (*mbhc).is_hs_recording = false;
    } else if event == WCD_EVENT_POST_MICBIAS_2_OFF {
        if (*(*mbhc).mbhc_cb).mbhc_micbias_control.is_none() {
            (*mbhc).is_hs_recording = false;
        }
        /* Enable PULL UP if PA's are enabled */
        if test_bit(WCD_MBHC_EVENT_PA_HPHL, &(*mbhc).event_state)
            || test_bit(WCD_MBHC_EVENT_PA_HPHR, &(*mbhc).event_state)
        {
            /* enable pullup and cs, disable mb */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_PULLUP);
        } else {
            /* enable current source and disable mb, pullup*/
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_CS);
        }
    } else if event == WCD_EVENT_POST_HPHL_PA_OFF {
        clear_bit(WCD_MBHC_EVENT_PA_HPHL, &mut (*mbhc).event_state);
        /* check if micbias is enabled */
        if micbias2 {
            /* Disable cs, pullup & enable micbias */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_MB);
        } else {
            /* Disable micbias, pullup & enable cs */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_CS);
        }
    } else if event == WCD_EVENT_POST_HPHR_PA_OFF {
        clear_bit(WCD_MBHC_EVENT_PA_HPHR, &mut (*mbhc).event_state);
        /* check if micbias is enabled */
        if micbias2 {
            /* Disable cs, pullup & enable micbias */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_MB);
        } else {
            /* Disable micbias, pullup & enable cs */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_CS);
        }
    } else if event == WCD_EVENT_PRE_HPHL_PA_ON {
        set_bit(WCD_MBHC_EVENT_PA_HPHL, &mut (*mbhc).event_state);
        /* check if micbias is enabled */
        if micbias2 {
            /* Disable cs, pullup & enable micbias */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_MB);
        } else {
            /* Disable micbias, enable pullup & cs */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_PULLUP);
        }
    } else if event == WCD_EVENT_PRE_HPHR_PA_ON {
        set_bit(WCD_MBHC_EVENT_PA_HPHR, &mut (*mbhc).event_state);
        /* check if micbias is enabled */
        if micbias2 {
            /* Disable cs, pullup & enable micbias */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_MB);
        } else {
            /* Disable micbias, enable pullup & cs */
            wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_PULLUP);
        }
    }
    0
}

unsafe fn wcd_cancel_btn_work(mbhc: *mut wcd_mbhc) -> c_int {
    cancel_delayed_work_sync(&mut (*mbhc).mbhc_btn_dwork)
}

unsafe fn wcd_micbias_disable(mbhc: *mut wcd_mbhc) {
    let component = (*mbhc).component;
    if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micbias_control {
        cb(component, MIC_BIAS_2, MICB_DISABLE);
    }
    if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micb_ctrl_thr_mic {
        cb(component, MIC_BIAS_2, false);
    }
    if let Some(cb) = (*(*mbhc).mbhc_cb).set_micbias_value {
        cb(component);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_MICB_CTRL, 0);
    }
}

unsafe fn wcd_mbhc_report_plug_removal(mbhc: *mut wcd_mbhc, jack_type: snd_jack_types) {
    (*mbhc).hph_status &= !(jack_type as u32);
    /*
     * cancel possibly scheduled btn work and
     * report release if we reported button press
     */
    if wcd_cancel_btn_work(mbhc) == 0 && (*mbhc).buttons_pressed != 0 {
        snd_soc_jack_report((*mbhc).jack, 0, (*mbhc).buttons_pressed);
        (*mbhc).buttons_pressed &= !WCD_MBHC_JACK_BUTTON_MASK;
    }
    wcd_micbias_disable(mbhc);
    (*mbhc).hph_type = WCD_MBHC_HPH_NONE;
    (*mbhc).zr = 0;
    (*mbhc).zl = (*mbhc).zr;
    snd_soc_jack_report((*mbhc).jack, (*mbhc).hph_status as c_int, WCD_MBHC_JACK_MASK);
    (*mbhc).current_plug = MBHC_PLUG_TYPE_NONE;
    (*mbhc).force_linein = false;
}

unsafe fn wcd_mbhc_compute_impedance(mbhc: *mut wcd_mbhc) {
    if !(*mbhc).impedance_detect {
        return;
    }
    if (*(*mbhc).cfg).linein_th != 0 {
        let fsm_en: u8 = wcd_mbhc_read_field(mbhc, WCD_MBHC_FSM_EN) as u8;
        /* Set MUX_CTL to AUTO for Z-det */
        wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_MUX_CTL, wcd_mbhc_adc_mux_ctl::MUX_CTL_AUTO as c_int);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
        ((*(*mbhc).mbhc_cb).compute_impedance.unwrap())((*mbhc).component, &mut (*mbhc).zl, &mut (*mbhc).zr);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, fsm_en as c_int);
    }
}

unsafe fn wcd_mbhc_report_plug_insertion(mbhc: *mut wcd_mbhc, mut jack_type: snd_jack_types) {
    let is_pa_on: bool;
    /*
     * Report removal of current jack type.
     * Headphone to headset shouldn't report headphone
     * removal.
     */
    if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADSET && jack_type == SND_JACK_HEADPHONE {
        (*mbhc).hph_status &= !(SND_JACK_HEADSET as u32);
    }
    /* Report insertion */
    if jack_type == SND_JACK_HEADPHONE {
        (*mbhc).current_plug = MBHC_PLUG_TYPE_HEADPHONE;
    } else if jack_type == SND_JACK_HEADSET {
        (*mbhc).current_plug = MBHC_PLUG_TYPE_HEADSET;
        (*mbhc).jiffies_atreport = jiffies;
    } else if jack_type == SND_JACK_LINEOUT {
        (*mbhc).current_plug = MBHC_PLUG_TYPE_HIGH_HPH;
    }
    is_pa_on = wcd_mbhc_read_field(mbhc, WCD_MBHC_HPH_PA_EN) != 0;
    if !is_pa_on {
        wcd_mbhc_compute_impedance(mbhc);
        if (*mbhc).zl > (*(*mbhc).cfg).linein_th
            && (*mbhc).zr > (*(*mbhc).cfg).linein_th
            && jack_type == SND_JACK_HEADPHONE
        {
            jack_type = SND_JACK_LINEOUT;
            (*mbhc).force_linein = true;
            (*mbhc).current_plug = MBHC_PLUG_TYPE_HIGH_HPH;
            if (*mbhc).hph_status != 0 {
                (*mbhc).hph_status &= !((SND_JACK_HEADSET | SND_JACK_LINEOUT) as u32);
                snd_soc_jack_report((*mbhc).jack, (*mbhc).hph_status as c_int, WCD_MBHC_JACK_MASK);
            }
        }
    }
    /* Do not calculate impedance again for lineout
     * as during playback pa is on and impedance values
     * will not be correct resulting in lineout detected
     * as headphone.
     */
    if is_pa_on && (*mbhc).force_linein {
        jack_type = SND_JACK_LINEOUT;
        (*mbhc).current_plug = MBHC_PLUG_TYPE_HIGH_HPH;
        if (*mbhc).hph_status != 0 {
            (*mbhc).hph_status &= !((SND_JACK_HEADSET | SND_JACK_LINEOUT) as u32);
            snd_soc_jack_report((*mbhc).jack, (*mbhc).hph_status as c_int, WCD_MBHC_JACK_MASK);
        }
    }
    (*mbhc).hph_status |= jack_type as u32;
    if jack_type == SND_JACK_HEADPHONE {
        if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micb_ramp_control {
            cb((*mbhc).component, false);
        }
    }
    snd_soc_jack_report(
        (*mbhc).jack,
        ((*mbhc).hph_status | SND_JACK_MECHANICAL as u32) as c_int,
        WCD_MBHC_JACK_MASK,
    );
}

unsafe fn wcd_mbhc_report_plug(mbhc: *mut wcd_mbhc, insertion: c_int, jack_type: snd_jack_types) {
    WARN_ON((mutex_is_locked(&mut (*mbhc).lock) == 0) as c_int);
    if insertion == 0 {
        /* Report removal */
        wcd_mbhc_report_plug_removal(mbhc, jack_type);
    } else {
        wcd_mbhc_report_plug_insertion(mbhc, jack_type);
    }
}

unsafe fn wcd_cancel_hs_detect_plug(mbhc: *mut wcd_mbhc, work: *mut work_struct) {
    (*mbhc).hs_detect_work_stop = true;
    mutex_unlock(&mut (*mbhc).lock);
    cancel_work_sync(work);
    mutex_lock(&mut (*mbhc).lock);
}

unsafe fn wcd_mbhc_cancel_pending_work(mbhc: *mut wcd_mbhc) {
    /* cancel pending button press */
    wcd_cancel_btn_work(mbhc);
    /* cancel correct work function */
    wcd_cancel_hs_detect_plug(mbhc, &mut (*mbhc).correct_plug_swch);
}

unsafe fn wcd_mbhc_elec_hs_report_unplug(mbhc: *mut wcd_mbhc) {
    wcd_mbhc_cancel_pending_work(mbhc);
    /* Report extension cable */
    wcd_mbhc_report_plug(mbhc, 1, SND_JACK_LINEOUT);
    /*
     * Disable HPHL trigger and MIC Schmitt triggers.
     * Setup for insertion detection.
     */
    disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_rem_intr);
    wcd_mbhc_curr_micbias_control(mbhc, WCD_MBHC_EN_NONE);
    /* Disable HW FSM */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC, 3);
    /* Set the detection type appropriately */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_DETECTION_TYPE, 1);
    enable_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
}

unsafe fn wcd_mbhc_find_plug_and_report(mbhc: *mut wcd_mbhc, plug_type: wcd_mbhc_plug_type) {
    if (*mbhc).current_plug as c_int == plug_type {
        return;
    }
    mutex_lock(&mut (*mbhc).lock);
    if plug_type == MBHC_PLUG_TYPE_HEADPHONE as c_int {
        wcd_mbhc_report_plug(mbhc, 1, SND_JACK_HEADPHONE);
    } else if plug_type == MBHC_PLUG_TYPE_HEADSET as c_int {
        wcd_mbhc_report_plug(mbhc, 1, SND_JACK_HEADSET);
    } else if plug_type == MBHC_PLUG_TYPE_HIGH_HPH as c_int {
        wcd_mbhc_report_plug(mbhc, 1, SND_JACK_LINEOUT);
    } else if plug_type == MBHC_PLUG_TYPE_GND_MIC_SWAP as c_int {
        if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADPHONE {
            wcd_mbhc_report_plug(mbhc, 0, SND_JACK_HEADPHONE);
        }
        if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADSET {
            wcd_mbhc_report_plug(mbhc, 0, SND_JACK_HEADSET);
        }
    } else {
        WARN(1, c"Unexpected current plug_type %d, plug_type %d\n".as_ptr(), (*mbhc).current_plug as c_int, plug_type);
    }
    mutex_unlock(&mut (*mbhc).lock);
}

unsafe fn wcd_schedule_hs_detect_plug(mbhc: *mut wcd_mbhc, work: *mut work_struct) {
    WARN_ON((mutex_is_locked(&mut (*mbhc).lock) == 0) as c_int);
    (*mbhc).hs_detect_work_stop = false;
    schedule_work(work);
}

unsafe fn wcd_mbhc_adc_detect_plug_type(mbhc: *mut wcd_mbhc) {
    let component = (*mbhc).component;
    WARN_ON((mutex_is_locked(&mut (*mbhc).lock) == 0) as c_int);
    if let Some(cb) = (*(*mbhc).mbhc_cb).hph_pull_down_ctrl {
        cb(component, false);
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 0);
    if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micbias_control {
        cb(component, MIC_BIAS_2, MICB_ENABLE);
        wcd_schedule_hs_detect_plug(mbhc, &mut (*mbhc).correct_plug_swch);
    }
}

unsafe extern "C" fn mbhc_plug_detect_fn(work: *mut work_struct) {
    let mbhc = container_of_work(work);
    let component = (*mbhc).component;
    let mut jack_type: snd_jack_types;
    let detection_type: bool;
    mutex_lock(&mut (*mbhc).lock);
    (*mbhc).in_swch_irq_handler = true;
    wcd_mbhc_cancel_pending_work(mbhc);
    detection_type = wcd_mbhc_read_field(mbhc, WCD_MBHC_MECH_DETECTION_TYPE) != 0;
    /* Set the detection type appropriately */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_MECH_DETECTION_TYPE, (!detection_type) as c_int);
    /* Enable micbias ramp */
    if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micb_ramp_control {
        cb(component, true);
    }
    if detection_type {
        if (*mbhc).current_plug != MBHC_PLUG_TYPE_NONE {
            goto_exit(mbhc);
            return;
        }
        /* Make sure MASTER_BIAS_CTL is enabled */
        ((*(*mbhc).mbhc_cb).mbhc_bias.unwrap())(component, true);
        (*mbhc).is_btn_press = false;
        wcd_mbhc_adc_detect_plug_type(mbhc);
    } else {
        /* Disable HW FSM */
        wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
        (*mbhc).extn_cable_hph_rem = false;
        if (*mbhc).current_plug == MBHC_PLUG_TYPE_NONE {
            goto_exit(mbhc);
            return;
        }
        (*mbhc).is_btn_press = false;
        if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADPHONE {
            jack_type = SND_JACK_HEADPHONE;
        } else if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADSET {
            jack_type = SND_JACK_HEADSET;
        } else if (*mbhc).current_plug == MBHC_PLUG_TYPE_HIGH_HPH {
            if (*mbhc).mbhc_detection_logic == WCD_DETECTION_ADC {
                wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_ISRC_EN, 0);
            }
            jack_type = SND_JACK_LINEOUT;
        } else if (*mbhc).current_plug == MBHC_PLUG_TYPE_GND_MIC_SWAP {
            dev_err((*mbhc).dev, c"Ground and Mic Swapped on plug\n".as_ptr());
            goto_exit(mbhc);
            return;
        } else {
            dev_err((*mbhc).dev, c"Invalid current plug: %d\n".as_ptr(), (*mbhc).current_plug as c_int);
            goto_exit(mbhc);
            return;
        }
        disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_rem_intr);
        disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_DETECTION_TYPE, 1);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC, 0);
        wcd_mbhc_report_plug(mbhc, 0, jack_type);
    }
    goto_exit(mbhc);
}

unsafe fn goto_exit(mbhc: *mut wcd_mbhc) {
    (*mbhc).in_swch_irq_handler = false;
    mutex_unlock(&mut (*mbhc).lock);
}

unsafe extern "C" fn wcd_mbhc_mech_plug_detect_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mbhc = data as *mut wcd_mbhc;
    if !(*(*mbhc).cfg).typec_analog_mux {
        schedule_work(&mut (*mbhc).mbhc_plug_detect_work);
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn wcd_mbhc_typec_report_unplug(mbhc: *mut wcd_mbhc) -> c_int {
    if mbhc.is_null() || !(*(*mbhc).cfg).typec_analog_mux {
        return -EINVAL;
    }
    if let Some(cb) = (*(*mbhc).mbhc_cb).clk_setup {
        cb((*mbhc).component, false);
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_L_DET_EN, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_MECH_DETECTION_TYPE, 0);
    schedule_work(&mut (*mbhc).mbhc_plug_detect_work);
    0
}

pub unsafe extern "C" fn wcd_mbhc_typec_report_plug(mbhc: *mut wcd_mbhc) -> c_int {
    if mbhc.is_null() || !(*(*mbhc).cfg).typec_analog_mux {
        return -EINVAL;
    }
    if let Some(cb) = (*(*mbhc).mbhc_cb).clk_setup {
        cb((*mbhc).component, true);
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_L_DET_EN, 1);
    schedule_work(&mut (*mbhc).mbhc_plug_detect_work);
    0
}

unsafe fn wcd_mbhc_get_button_mask(mbhc: *mut wcd_mbhc) -> c_int {
    let mut mask = 0;
    let btn = wcd_mbhc_read_field(mbhc, WCD_MBHC_BTN_RESULT);
    if btn == 0 {
        mask = SND_JACK_BTN_0;
    } else if btn == 1 {
        mask = SND_JACK_BTN_1;
    } else if btn == 2 {
        mask = SND_JACK_BTN_2;
    } else if btn == 3 {
        mask = SND_JACK_BTN_3;
    } else if btn == 4 {
        mask = SND_JACK_BTN_4;
    } else if btn == 5 {
        mask = SND_JACK_BTN_5;
    }
    mask
}

unsafe extern "C" fn wcd_btn_long_press_fn(work: *mut work_struct) {
    let dwork = to_delayed_work(work);
    let mbhc = container_of_delayed_work(dwork);
    if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADSET {
        snd_soc_jack_report((*mbhc).jack, (*mbhc).buttons_pressed, (*mbhc).buttons_pressed);
    }
}

unsafe extern "C" fn wcd_mbhc_btn_press_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mbhc = data as *mut wcd_mbhc;
    let mask: c_int;
    let msec_val: c_ulong;
    mutex_lock(&mut (*mbhc).lock);
    wcd_cancel_btn_work(mbhc);
    (*mbhc).is_btn_press = true;
    msec_val = jiffies_to_msecs(jiffies - (*mbhc).jiffies_atreport);
    /* Too short, ignore button press */
    if msec_val >= MBHC_BUTTON_PRESS_THRESHOLD_MIN
        /* If switch interrupt already kicked in, ignore button press */
        && !(*mbhc).in_swch_irq_handler
        /* Plug isn't headset, ignore button press */
        && (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADSET
    {
        mask = wcd_mbhc_get_button_mask(mbhc);
        (*mbhc).buttons_pressed |= mask;
        if schedule_delayed_work(&mut (*mbhc).mbhc_btn_dwork, msecs_to_jiffies(400)) == 0 {
            WARN(1, c"Button pressed twice without release event\n".as_ptr());
        }
    }
    mutex_unlock(&mut (*mbhc).lock);
    IRQ_HANDLED
}

unsafe extern "C" fn wcd_mbhc_btn_release_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mbhc = data as *mut wcd_mbhc;
    let ret: c_int;
    mutex_lock(&mut (*mbhc).lock);
    if (*mbhc).is_btn_press {
        (*mbhc).is_btn_press = false;
    } else {
        mutex_unlock(&mut (*mbhc).lock);
        return IRQ_HANDLED;
    }
    if ((*mbhc).buttons_pressed & WCD_MBHC_JACK_BUTTON_MASK) != 0 {
        ret = wcd_cancel_btn_work(mbhc);
        if ret == 0 {
            /* Reporting long button release event */
            snd_soc_jack_report((*mbhc).jack, 0, (*mbhc).buttons_pressed);
        } else if !(*mbhc).in_swch_irq_handler {
            /* Reporting btn press n Release */
            snd_soc_jack_report((*mbhc).jack, (*mbhc).buttons_pressed, (*mbhc).buttons_pressed);
            snd_soc_jack_report((*mbhc).jack, 0, (*mbhc).buttons_pressed);
        }
        (*mbhc).buttons_pressed &= !WCD_MBHC_JACK_BUTTON_MASK;
    }
    mutex_unlock(&mut (*mbhc).lock);
    IRQ_HANDLED
}

unsafe fn wcd_mbhc_hph_ocp_irq(mbhc: *mut wcd_mbhc, hphr: bool) -> irqreturn_t {
    /* TODO Find a better way to report this to Userspace */
    dev_err((*mbhc).dev, c"MBHC Over Current on %s detected\n".as_ptr(), if hphr { c"HPHR".as_ptr() } else { c"HPHL".as_ptr() });
    wcd_mbhc_write_field(mbhc, WCD_MBHC_OCP_FSM_EN, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_OCP_FSM_EN, 1);
    IRQ_HANDLED
}

unsafe extern "C" fn wcd_mbhc_hphl_ocp_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    wcd_mbhc_hph_ocp_irq(data as *mut wcd_mbhc, false)
}

unsafe extern "C" fn wcd_mbhc_hphr_ocp_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    wcd_mbhc_hph_ocp_irq(data as *mut wcd_mbhc, true)
}

unsafe fn wcd_mbhc_initialise(mbhc: *mut wcd_mbhc) -> c_int {
    let component = (*mbhc).component;
    let mut ret: c_int;
    ret = pm_runtime_get_sync((*component).dev);
    if ret < 0 && ret != -EACCES {
        dev_err_ratelimited((*component).dev, c"pm_runtime_get_sync failed in %s, ret %d\n".as_ptr(), c"wcd_mbhc_initialise".as_ptr(), ret);
        pm_runtime_put_noidle((*component).dev);
        return ret;
    }
    mutex_lock(&mut (*mbhc).lock);
    if (*(*mbhc).cfg).typec_analog_mux {
        (*mbhc).swap_thr = GND_MIC_USBC_SWAP_THRESHOLD;
    } else {
        (*mbhc).swap_thr = GND_MIC_SWAP_THRESHOLD;
    }
    /* setup HS detection */
    if let Some(cb) = (*(*mbhc).mbhc_cb).hph_pull_up_control_v2 {
        cb(component, if (*(*mbhc).cfg).typec_analog_mux { HS_PULLUP_I_OFF } else { HS_PULLUP_I_DEFAULT });
    } else if let Some(cb) = (*(*mbhc).mbhc_cb).hph_pull_up_control {
        cb(component, if (*(*mbhc).cfg).typec_analog_mux { I_OFF } else { I_DEFAULT });
    } else {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_HS_L_DET_PULL_UP_CTRL, if (*(*mbhc).cfg).typec_analog_mux { 0 } else { 3 });
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_HPHL_PLUG_TYPE, (*(*mbhc).cfg).hphl_swh as c_int);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_GND_PLUG_TYPE, (*(*mbhc).cfg).gnd_swh as c_int);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_SW_HPH_LP_100K_TO_GND, 1);
    if (*(*mbhc).cfg).gnd_det_en {
        if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_gnd_det_ctrl {
            cb(component, true);
        }
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_HS_L_DET_PULL_UP_COMP_CTRL, 1);
    /* Plug detect is triggered manually if analog goes through USBCC */
    if (*(*mbhc).cfg).typec_analog_mux {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_L_DET_EN, 0);
    } else {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_L_DET_EN, 1);
    }
    if (*(*mbhc).cfg).typec_analog_mux {
        /* Insertion debounce set to 48ms */
        wcd_mbhc_write_field(mbhc, WCD_MBHC_INSREM_DBNC, 4);
    } else {
        /* Insertion debounce set to 96ms */
        wcd_mbhc_write_field(mbhc, WCD_MBHC_INSREM_DBNC, 6);
    }
    /* Button Debounce set to 16ms */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_DBNC, 2);
    /* enable bias */
    ((*(*mbhc).mbhc_cb).mbhc_bias.unwrap())(component, true);
    /* enable MBHC clock */
    if let Some(cb) = (*(*mbhc).mbhc_cb).clk_setup {
        cb(component, if (*(*mbhc).cfg).typec_analog_mux { false } else { true });
    }
    /* program HS_VREF value */
    wcd_program_hs_vref(mbhc);
    wcd_program_btn_threshold(mbhc, false);
    mutex_unlock(&mut (*mbhc).lock);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe fn wcd_mbhc_get_micbias(mbhc: *mut wcd_mbhc) -> c_int {
    let mut micbias = 0;
    if let Some(cb) = (*(*mbhc).mbhc_cb).get_micbias_val {
        cb((*mbhc).component, &mut micbias);
    } else {
        let vout_ctl: u8;
        /* Read MBHC Micbias (Mic Bias2) voltage */
        vout_ctl = wcd_mbhc_read_field(mbhc, WCD_MBHC_MICB2_VOUT) as u8;
        /* Formula for getting micbias from vout
         * micbias = 1.0V + VOUT_CTL * 50mV
         */
        micbias = 1000 + (vout_ctl as c_int * 50);
    }
    micbias
}

fn wcd_get_voltage_from_adc(val: u8, micbias: c_int) -> c_int {
    /* Formula for calculating voltage from ADC
     * Voltage = ADC_RESULT*12.5mV*V_MICBIAS/1.8
     */
    ((val as c_int * 125 * micbias) / (WCD_MBHC_ADC_MICBIAS_MV * 10))
}

unsafe fn wcd_measure_adc_continuous(mbhc: *mut wcd_mbhc) -> c_int {
    let mut adc_result: u8 = 0;
    let output_mv: c_int;
    let mut retry = 3;
    let adc_en: u8;
    /* Pre-requisites for ADC continuous measurement */
    /* Read legacy electircal detection and disable */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC, 0x00);
    /* Set ADC to continuous measurement */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, 1);
    /* Read ADC Enable bit to restore after adc measurement */
    adc_en = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_EN) as u8;
    /* Disable ADC_ENABLE bit */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 0);
    /* Disable MBHC FSM */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
    /* Set the MUX selection to IN2P */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_MUX_CTL, wcd_mbhc_adc_mux_ctl::MUX_CTL_IN2P as c_int);
    /* Enable MBHC FSM */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
    /* Enable ADC_ENABLE bit */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 1);
    while retry != 0 {
        retry -= 1;
        /* wait for 3 msec before reading ADC result */
        usleep_range(3000, 3100);
        adc_result = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_RESULT) as u8;
    }
    /* Restore ADC Enable */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, adc_en as c_int);
    /* Get voltage from ADC result */
    output_mv = wcd_get_voltage_from_adc(adc_result, wcd_mbhc_get_micbias(mbhc));
    output_mv
}

unsafe fn wcd_measure_adc_once(mbhc: *mut wcd_mbhc, mux_ctl: c_int) -> c_int {
    let dev = (*mbhc).dev;
    let mut adc_timeout: u8 = 0;
    let mut adc_complete: u8 = 0;
    let mut adc_result: u8;
    let mut retry = 6;
    let ret: c_int;
    let mut output_mv = 0;
    let adc_en: u8;
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, 0);
    /* Read ADC Enable bit to restore after adc measurement */
    adc_en = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_EN) as u8;
    /* Trigger ADC one time measurement */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
    /* Set the appropriate MUX selection */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_MUX_CTL, mux_ctl);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 1);
    while retry != 0 {
        retry -= 1;
        /* wait for 600usec to get adc results */
        usleep_range(600, 610);
        /* check for ADC Timeout */
        adc_timeout = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_TIMEOUT) as u8;
        if adc_timeout != 0 {
            continue;
        }
        /* Read ADC complete bit */
        adc_complete = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_COMPLETE) as u8;
        if adc_complete == 0 {
            continue;
        }
        /* Read ADC result */
        adc_result = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_RESULT) as u8;
        /* Get voltage from ADC result */
        output_mv = wcd_get_voltage_from_adc(adc_result, wcd_mbhc_get_micbias(mbhc));
        break;
    }
    /* Restore ADC Enable */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, adc_en as c_int);
    if retry <= 0 {
        dev_err(dev, c"%s: adc complete: %d, adc timeout: %d\n".as_ptr(), c"wcd_measure_adc_once".as_ptr(), adc_complete as c_int, adc_timeout as c_int);
        ret = -EINVAL;
    } else {
        ret = output_mv;
    }
    ret
}

/* To determine if cross connection occurred */
unsafe fn wcd_check_cross_conn(mbhc: *mut wcd_mbhc) -> c_int {
    let adc_mode: u8;
    let elect_ctl: u8;
    let adc_en: u8;
    let fsm_en: u8;
    let hphl_adc_res: c_int;
    let hphr_adc_res: c_int;
    let mut is_cross_conn = false;
    /* If PA is enabled, dont check for cross-connection */
    if wcd_mbhc_read_field(mbhc, WCD_MBHC_HPH_PA_EN) != 0 {
        return -EINVAL;
    }
    /* Read legacy electircal detection and disable */
    elect_ctl = wcd_mbhc_read_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC) as u8;
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC, 0);
    /* Read and set ADC to single measurement */
    adc_mode = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_MODE) as u8;
    /* Read ADC Enable bit to restore after adc measurement */
    adc_en = wcd_mbhc_read_field(mbhc, WCD_MBHC_ADC_EN) as u8;
    /* Read FSM status */
    fsm_en = wcd_mbhc_read_field(mbhc, WCD_MBHC_FSM_EN) as u8;
    /* Get adc result for HPH L */
    hphl_adc_res = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_HPH_L as c_int);
    if hphl_adc_res < 0 {
        return hphl_adc_res;
    }
    /* Get adc result for HPH R in mV */
    hphr_adc_res = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_HPH_R as c_int);
    if hphr_adc_res < 0 {
        return hphr_adc_res;
    }
    if hphl_adc_res > HPHL_CROSS_CONN_THRESHOLD || hphr_adc_res > HPHL_CROSS_CONN_THRESHOLD {
        is_cross_conn = true;
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 0);
    /* Set the MUX selection to Auto */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_MUX_CTL, wcd_mbhc_adc_mux_ctl::MUX_CTL_AUTO as c_int);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, 1);
    /* Restore ADC Enable */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, adc_en as c_int);
    /* Restore ADC mode */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, adc_mode as c_int);
    /* Restore FSM state */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_FSM_EN, fsm_en as c_int);
    /* Restore electrical detection */
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_SCHMT_ISRC, elect_ctl as c_int);
    is_cross_conn as c_int
}

unsafe fn wcd_mbhc_adc_get_hs_thres(mbhc: *mut wcd_mbhc) -> c_int {
    let hs_threshold: c_int;
    let micbias_mv = wcd_mbhc_get_micbias(mbhc);
    if (*(*mbhc).cfg).hs_thr != 0 {
        if (*(*mbhc).cfg).micb_mv == micbias_mv {
            hs_threshold = (*(*mbhc).cfg).hs_thr;
        } else {
            hs_threshold = ((*(*mbhc).cfg).hs_thr * micbias_mv) / (*(*mbhc).cfg).micb_mv;
        }
    } else {
        hs_threshold = (WCD_MBHC_ADC_HS_THRESHOLD_MV * micbias_mv) / WCD_MBHC_ADC_MICBIAS_MV;
    }
    hs_threshold
}

unsafe fn wcd_mbhc_adc_get_hph_thres(mbhc: *mut wcd_mbhc) -> c_int {
    let hph_threshold: c_int;
    let micbias_mv = wcd_mbhc_get_micbias(mbhc);
    if (*(*mbhc).cfg).hph_thr != 0 {
        if (*(*mbhc).cfg).micb_mv == micbias_mv {
            hph_threshold = (*(*mbhc).cfg).hph_thr;
        } else {
            hph_threshold = ((*(*mbhc).cfg).hph_thr * micbias_mv) / (*(*mbhc).cfg).micb_mv;
        }
    } else {
        hph_threshold = (WCD_MBHC_ADC_HPH_THRESHOLD_MV * micbias_mv) / WCD_MBHC_ADC_MICBIAS_MV;
    }
    hph_threshold
}

unsafe fn wcd_mbhc_adc_update_fsm_source(mbhc: *mut wcd_mbhc, plug_type: wcd_mbhc_plug_type) {
    let mut micbias2 = false;
    if plug_type == MBHC_PLUG_TYPE_HEADPHONE as c_int {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 3);
    } else if plug_type == MBHC_PLUG_TYPE_HEADSET as c_int {
        if let Some(cb) = (*(*mbhc).mbhc_cb).micbias_enable_status {
            micbias2 = cb((*mbhc).component, MIC_BIAS_2);
        }
        if !(*mbhc).is_hs_recording && !micbias2 {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 3);
        }
    } else {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
    }
}

unsafe fn wcd_mbhc_bcs_enable(mbhc: *mut wcd_mbhc, plug_type: c_int, enable: bool) {
    if plug_type == MBHC_PLUG_TYPE_HEADSET as c_int || plug_type == MBHC_PLUG_TYPE_HEADPHONE as c_int {
        if let Some(cb) = (*(*mbhc).mbhc_cb).bcs_enable {
            cb((*mbhc).component, enable);
        }
    }
}

unsafe fn wcd_mbhc_get_plug_from_adc(mbhc: *mut wcd_mbhc, adc_result: c_int) -> c_int {
    let plug_type: wcd_mbhc_plug_type;
    let hs_thr: c_int = wcd_mbhc_adc_get_hs_thres(mbhc);
    let hph_thr: c_int = wcd_mbhc_adc_get_hph_thres(mbhc);
    if adc_result < hph_thr {
        plug_type = MBHC_PLUG_TYPE_HEADPHONE as c_int;
    } else if adc_result > hs_thr {
        plug_type = MBHC_PLUG_TYPE_HIGH_HPH as c_int;
    } else {
        plug_type = MBHC_PLUG_TYPE_HEADSET as c_int;
    }
    plug_type
}

unsafe fn wcd_mbhc_get_spl_hs_thres(mbhc: *mut wcd_mbhc) -> c_int {
    let hs_threshold: c_int;
    let micbias_mv = wcd_mbhc_get_micbias(mbhc);
    if (*(*mbhc).cfg).hs_thr != 0 && (*(*mbhc).cfg).micb_mv != WCD_MBHC_ADC_MICBIAS_MV {
        if (*(*mbhc).cfg).micb_mv == micbias_mv {
            hs_threshold = (*(*mbhc).cfg).hs_thr;
        } else {
            hs_threshold = ((*(*mbhc).cfg).hs_thr * micbias_mv) / (*(*mbhc).cfg).micb_mv;
        }
    } else {
        hs_threshold = (WCD_MBHC_ADC_HS_THRESHOLD_MV * micbias_mv) / WCD_MBHC_ADC_MICBIAS_MV;
    }
    hs_threshold
}

unsafe fn wcd_mbhc_check_for_spl_headset(mbhc: *mut wcd_mbhc) -> bool {
    let mut is_spl_hs = false;
    let output_mv: c_int;
    let hs_threshold: c_int;
    let hph_threshold: c_int;
    if (*(*mbhc).mbhc_cb).mbhc_micb_ctrl_thr_mic.is_none() {
        return false;
    }
    /* Bump up MIC_BIAS2 to 2.7V */
    ((*(*mbhc).mbhc_cb).mbhc_micb_ctrl_thr_mic.unwrap())((*mbhc).component, MIC_BIAS_2, true);
    usleep_range(10000, 10100);
    output_mv = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_IN2P as c_int);
    hs_threshold = wcd_mbhc_get_spl_hs_thres(mbhc);
    hph_threshold = wcd_mbhc_adc_get_hph_thres(mbhc);
    if !(output_mv > hs_threshold || output_mv < hph_threshold) {
        is_spl_hs = true;
    }
    /* Back MIC_BIAS2 to 1.8v if the type is not special headset */
    if !is_spl_hs {
        ((*(*mbhc).mbhc_cb).mbhc_micb_ctrl_thr_mic.unwrap())((*mbhc).component, MIC_BIAS_2, false);
        /* Add 10ms delay for micbias to settle */
        usleep_range(10000, 10100);
    }
    is_spl_hs
}

unsafe extern "C" fn wcd_correct_swch_plug(work: *mut work_struct) {
    let mbhc = container_of_work(work);
    let component = (*mbhc).component;
    let mut plug_type: wcd_mbhc_plug_type = MBHC_PLUG_TYPE_INVALID as c_int;
    let timeout: c_ulong;
    let mut pt_gnd_mic_swap_cnt = 0;
    let mut output_mv: c_int;
    let mut cross_conn: c_int;
    let mut hs_threshold: c_int;
    let mut try_count = 0;
    let micbias_mv: c_int;
    let mut is_spl_hs = false;
    let is_pa_on: bool;
    let ret: c_int;
    ret = pm_runtime_get_sync((*component).dev);
    if ret < 0 && ret != -EACCES {
        dev_err_ratelimited((*component).dev, c"pm_runtime_get_sync failed in %s, ret %d\n".as_ptr(), c"wcd_correct_swch_plug".as_ptr(), ret);
        pm_runtime_put_noidle((*component).dev);
        return;
    }
    micbias_mv = wcd_mbhc_get_micbias(mbhc);
    hs_threshold = wcd_mbhc_adc_get_hs_thres(mbhc);
    /* Mask ADC COMPLETE interrupt */
    disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
    /* Check for cross connection */
    loop {
        cross_conn = wcd_check_cross_conn(mbhc);
        try_count += 1;
        if try_count >= (*mbhc).swap_thr as c_int {
            break;
        }
    }
    if cross_conn > 0 {
        plug_type = MBHC_PLUG_TYPE_GND_MIC_SWAP as c_int;
        dev_err((*mbhc).dev, c"cross connection found, Plug type %d\n".as_ptr(), plug_type);
    } else {
        /* Find plug type */
        output_mv = wcd_measure_adc_continuous(mbhc);
        plug_type = wcd_mbhc_get_plug_from_adc(mbhc, output_mv);
        /*
         * Report plug type if it is either headset or headphone
         * else start the 3 sec loop
         */
        if plug_type == MBHC_PLUG_TYPE_HEADPHONE as c_int {
            wcd_mbhc_find_plug_and_report(mbhc, plug_type);
        } else if plug_type == MBHC_PLUG_TYPE_HEADSET as c_int {
            wcd_mbhc_find_plug_and_report(mbhc, plug_type);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, 0);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 0);
            wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 1);
        }
    }
    /* Disable BCS slow insertion detection */
    wcd_mbhc_bcs_enable(mbhc, plug_type, false);
    timeout = jiffies + msecs_to_jiffies(HS_DETECT_PLUG_TIME_MS as c_uint);
    while !time_after(jiffies, timeout) {
        if (*mbhc).hs_detect_work_stop {
            wcd_micbias_disable(mbhc);
            wcd_correct_swch_exit(mbhc, component, plug_type);
            return;
        }
        msleep(180);
        /*
         * Use ADC single mode to minimize the chance of missing out
         * btn press/release for HEADSET type during correct work.
         */
        output_mv = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_IN2P as c_int);
        plug_type = wcd_mbhc_get_plug_from_adc(mbhc, output_mv);
        let pa_on_now = wcd_mbhc_read_field(mbhc, WCD_MBHC_HPH_PA_EN) != 0;
        if output_mv > hs_threshold && !is_spl_hs {
            is_spl_hs = wcd_mbhc_check_for_spl_headset(mbhc);
            output_mv = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_IN2P as c_int);
            if is_spl_hs {
                hs_threshold *= wcd_mbhc_get_micbias(mbhc);
                hs_threshold /= micbias_mv;
            }
        }
        if output_mv <= hs_threshold && !pa_on_now {
            /* Check for cross connection*/
            cross_conn = wcd_check_cross_conn(mbhc);
            if cross_conn > 0 {
                /* cross-connection */
                pt_gnd_mic_swap_cnt += 1;
                if pt_gnd_mic_swap_cnt < (*mbhc).swap_thr as c_int {
                    continue;
                } else {
                    plug_type = MBHC_PLUG_TYPE_GND_MIC_SWAP as c_int;
                }
            } else if cross_conn == 0 {
                /* no cross connection */
                pt_gnd_mic_swap_cnt = 0;
                plug_type = wcd_mbhc_get_plug_from_adc(mbhc, output_mv);
                continue;
            } else {
                /* Error if (cross_conn < 0) */
                continue;
            }
            if pt_gnd_mic_swap_cnt == (*mbhc).swap_thr as c_int {
                /* US_EU gpio present, flip switch */
                if let Some(cb) = (*(*mbhc).cfg).swap_gnd_mic {
                    if cb(component) != 0 {
                        continue;
                    }
                }
            }
        }
        /* cable is extension cable */
        if output_mv > hs_threshold || (*mbhc).force_linein {
            plug_type = MBHC_PLUG_TYPE_HIGH_HPH as c_int;
        }
    }
    wcd_mbhc_bcs_enable(mbhc, plug_type, true);
    if plug_type == MBHC_PLUG_TYPE_HIGH_HPH as c_int {
        if is_spl_hs {
            plug_type = MBHC_PLUG_TYPE_HEADSET as c_int;
        } else {
            wcd_mbhc_write_field(mbhc, WCD_MBHC_ELECT_ISRC_EN, 1);
        }
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 0);
    wcd_mbhc_find_plug_and_report(mbhc, plug_type);
    /*
     * Set DETECTION_DONE bit for HEADSET
     * so that btn press/release interrupt can be generated.
     * For other plug type, clear the bit.
     */
    if plug_type == MBHC_PLUG_TYPE_HEADSET as c_int {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 1);
    } else {
        wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 0);
    }
    if (*(*mbhc).mbhc_cb).mbhc_micbias_control.is_some() {
        wcd_mbhc_adc_update_fsm_source(mbhc, plug_type);
    }
    wcd_correct_swch_exit(mbhc, component, plug_type);
}

unsafe fn wcd_correct_swch_exit(mbhc: *mut wcd_mbhc, component: *mut snd_soc_component, plug_type: c_int) {
    if let Some(cb) = (*(*mbhc).mbhc_cb).mbhc_micbias_control {
        cb(component, MIC_BIAS_2, MICB_DISABLE);
    }
    /*
     * If plug type is corrected from special headset to headphone,
     * clear the micbias enable flag, set micbias back to 1.8V and
     * disable micbias.
     */
    if plug_type == MBHC_PLUG_TYPE_HEADPHONE as c_int {
        wcd_micbias_disable(mbhc);
        /*
         * Enable ADC COMPLETE interrupt for HEADPHONE.
         * Btn release may happen after the correct work, ADC COMPLETE
         * interrupt needs to be captured to correct plug type.
         */
        enable_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
    }
    if let Some(cb) = (*(*mbhc).mbhc_cb).hph_pull_down_ctrl {
        cb(component, true);
    }
    pm_runtime_put_autosuspend((*component).dev);
}

unsafe extern "C" fn wcd_mbhc_adc_hs_rem_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mbhc = data as *mut wcd_mbhc;
    let timeout: c_ulong;
    let adc_threshold: c_int;
    let mut output_mv: c_int;
    let mut retry = 0;
    mutex_lock(&mut (*mbhc).lock);
    timeout = jiffies + msecs_to_jiffies(WCD_FAKE_REMOVAL_MIN_PERIOD_MS as c_uint);
    adc_threshold = wcd_mbhc_adc_get_hs_thres(mbhc);
    loop {
        retry += 1;
        /*
         * read output_mv every 10ms to look for
         * any change in IN2_P
         */
        usleep_range(10000, 10100);
        output_mv = wcd_measure_adc_once(mbhc, wcd_mbhc_adc_mux_ctl::MUX_CTL_IN2P as c_int);
        /* Check for fake removal */
        if output_mv <= adc_threshold && retry > FAKE_REM_RETRY_ATTEMPTS {
            mutex_unlock(&mut (*mbhc).lock);
            return IRQ_HANDLED;
        }
        if time_after(jiffies, timeout) {
            break;
        }
    }
    /*
     * ADC COMPLETE and ELEC_REM interrupts are both enabled for
     * HEADPHONE, need to reject the ADC COMPLETE interrupt which
     * follows ELEC_REM one when HEADPHONE is removed.
     */
    if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADPHONE {
        (*mbhc).extn_cable_hph_rem = true;
    }
    wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_MODE, 0);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_ADC_EN, 0);
    wcd_mbhc_elec_hs_report_unplug(mbhc);
    wcd_mbhc_write_field(mbhc, WCD_MBHC_BTN_ISRC_CTL, 0);
    mutex_unlock(&mut (*mbhc).lock);
    IRQ_HANDLED
}

unsafe extern "C" fn wcd_mbhc_adc_hs_ins_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let mbhc = data as *mut wcd_mbhc;
    let mut clamp_state: u8;
    let mut clamp_retry: u8 = WCD_MBHC_FAKE_INS_RETRY;
    /*
     * ADC COMPLETE and ELEC_REM interrupts are both enabled for HEADPHONE,
     * need to reject the ADC COMPLETE interrupt which follows ELEC_REM one
     * when HEADPHONE is removed.
     */
    if (*mbhc).extn_cable_hph_rem == true {
        (*mbhc).extn_cable_hph_rem = false;
        return IRQ_HANDLED;
    }
    loop {
        clamp_state = wcd_mbhc_read_field(mbhc, WCD_MBHC_IN2P_CLAMP_STATE) as u8;
        if clamp_state != 0 {
            return IRQ_HANDLED;
        }
        /*
         * check clamp for 120ms but at 30ms chunks to leave
         * room for other interrupts to be processed
         */
        usleep_range(30000, 30100);
        clamp_retry = clamp_retry.wrapping_sub(1);
        if clamp_retry == 0 {
            break;
        }
    }
    /*
     * If current plug is headphone then there is no chance to
     * get ADC complete interrupt, so connected cable should be
     * headset not headphone.
     */
    if (*mbhc).current_plug == MBHC_PLUG_TYPE_HEADPHONE {
        disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
        wcd_mbhc_write_field(mbhc, WCD_MBHC_DETECTION_DONE, 1);
        wcd_mbhc_find_plug_and_report(mbhc, MBHC_PLUG_TYPE_HEADSET as c_int);
        return IRQ_HANDLED;
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn wcd_mbhc_get_impedance(mbhc: *mut wcd_mbhc, zl: *mut uint32_t, zr: *mut uint32_t) -> c_int {
    *zl = (*mbhc).zl;
    *zr = (*mbhc).zr;
    if *zl != 0 && *zr != 0 {
        0
    } else {
        -EINVAL
    }
}

pub unsafe extern "C" fn wcd_mbhc_set_hph_type(mbhc: *mut wcd_mbhc, hph_type: c_int) {
    (*mbhc).hph_type = hph_type;
}

pub unsafe extern "C" fn wcd_mbhc_get_hph_type(mbhc: *mut wcd_mbhc) -> c_int {
    (*mbhc).hph_type
}

pub unsafe extern "C" fn wcd_mbhc_start(mbhc: *mut wcd_mbhc, cfg: *mut wcd_mbhc_config, jack: *mut snd_soc_jack) -> c_int {
    if mbhc.is_null() || cfg.is_null() || jack.is_null() {
        return -EINVAL;
    }
    (*mbhc).cfg = cfg;
    (*mbhc).jack = jack;
    wcd_mbhc_initialise(mbhc)
}

pub unsafe extern "C" fn wcd_mbhc_stop(mbhc: *mut wcd_mbhc) {
    (*mbhc).current_plug = MBHC_PLUG_TYPE_NONE;
    (*mbhc).hph_status = 0;
    disable_irq_nosync((*(*mbhc).intr_ids).hph_left_ocp);
    disable_irq_nosync((*(*mbhc).intr_ids).hph_right_ocp);
}

pub unsafe extern "C" fn wcd_dt_parse_mbhc_data(dev: *mut device, cfg: *mut wcd_mbhc_config) -> c_int {
    let np = (*dev).of_node;
    let mut ret: c_int;
    let mut i: c_int;
    let mut microvolt: c_int = 0;
    if of_property_read_bool(np, c"qcom,hphl-jack-type-normally-closed".as_ptr()) {
        (*cfg).hphl_swh = false;
    } else {
        (*cfg).hphl_swh = true;
    }
    if of_property_read_bool(np, c"qcom,ground-jack-type-normally-closed".as_ptr()) {
        (*cfg).gnd_swh = false;
    } else {
        (*cfg).gnd_swh = true;
    }
    ret = of_property_read_u32(np, c"qcom,mbhc-headset-vthreshold-microvolt".as_ptr(), &mut microvolt);
    if ret != 0 {
        dev_dbg(dev, c"missing qcom,mbhc-hs-mic-max-vthreshold--microvolt in dt node\n".as_ptr());
    } else {
        (*cfg).hs_thr = microvolt / 1000;
    }
    ret = of_property_read_u32(np, c"qcom,mbhc-headphone-vthreshold-microvolt".as_ptr(), &mut microvolt);
    if ret != 0 {
        dev_dbg(dev, c"missing qcom,mbhc-hs-mic-min-vthreshold-microvolt\tentry\n".as_ptr());
    } else {
        (*cfg).hph_thr = microvolt / 1000;
    }
    ret = of_property_read_u32_array(
        np,
        c"qcom,mbhc-buttons-vthreshold-microvolt".as_ptr(),
        (*cfg).btn_high.as_mut_ptr(),
        WCD_MBHC_DEF_BUTTONS as usize,
    );
    if ret != 0 {
        dev_err(dev, c"missing qcom,mbhc-buttons-vthreshold-microvolt entry\n".as_ptr());
    }
    i = 0;
    while i < WCD_MBHC_DEF_BUTTONS {
        if ret != 0 {
            /* default voltage */
            (*cfg).btn_high[i as usize] = 500000;
        } else {
            /* Micro to Milli Volts */
            (*cfg).btn_high[i as usize] = (*cfg).btn_high[i as usize] / 1000;
        }
        i += 1;
    }
    0
}

pub unsafe extern "C" fn wcd_mbhc_init(
    component: *mut snd_soc_component,
    mbhc_cb: *const wcd_mbhc_cb,
    intr_ids: *const wcd_mbhc_intr,
    fields: *const wcd_mbhc_field,
    impedance_det_en: bool,
) -> *mut wcd_mbhc {
    let dev = (*component).dev;
    let mbhc: *mut wcd_mbhc;
    let mut ret: c_int;
    if intr_ids.is_null()
        || fields.is_null()
        || mbhc_cb.is_null()
        || (*mbhc_cb).mbhc_bias.is_none()
        || (*mbhc_cb).set_btn_thr.is_none()
    {
        dev_err(dev, c"%s: Insufficient mbhc configuration\n".as_ptr(), c"wcd_mbhc_init".as_ptr());
        return ERR_PTR(-(EINVAL as c_long));
    }
    mbhc = kzalloc(core::mem::size_of::<wcd_mbhc>(), GFP_KERNEL) as *mut wcd_mbhc;
    if mbhc.is_null() {
        return ERR_PTR(-(ENOMEM as c_long));
    }
    (*mbhc).component = component;
    (*mbhc).dev = dev;
    (*mbhc).intr_ids = intr_ids;
    (*mbhc).mbhc_cb = mbhc_cb;
    (*mbhc).fields = fields;
    (*mbhc).mbhc_detection_logic = WCD_DETECTION_ADC;
    if (*mbhc_cb).compute_impedance.is_some() {
        (*mbhc).impedance_detect = impedance_det_en;
    }
    INIT_DELAYED_WORK(&mut (*mbhc).mbhc_btn_dwork, wcd_btn_long_press_fn);
    mutex_init(&mut (*mbhc).lock);
    INIT_WORK(&mut (*mbhc).correct_plug_swch, wcd_correct_swch_plug);
    INIT_WORK(&mut (*mbhc).mbhc_plug_detect_work, mbhc_plug_detect_fn);
    ret = request_threaded_irq(
        (*(*mbhc).intr_ids).mbhc_sw_intr,
        None,
        Some(wcd_mbhc_mech_plug_detect_irq),
        IRQF_ONESHOT | IRQF_TRIGGER_RISING,
        c"mbhc sw intr".as_ptr(),
        mbhc as *mut c_void,
    );
    if ret != 0 {
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    ret = request_threaded_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, None, Some(wcd_mbhc_btn_press_handler), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"Button Press detect".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    ret = request_threaded_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, None, Some(wcd_mbhc_btn_release_handler), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"Button Release detect".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    ret = request_threaded_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr, None, Some(wcd_mbhc_adc_hs_ins_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"Elect Insert".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_ins_intr);
    ret = request_threaded_irq((*(*mbhc).intr_ids).mbhc_hs_rem_intr, None, Some(wcd_mbhc_adc_hs_rem_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"Elect Remove".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    disable_irq_nosync((*(*mbhc).intr_ids).mbhc_hs_rem_intr);
    ret = request_threaded_irq((*(*mbhc).intr_ids).hph_left_ocp, None, Some(wcd_mbhc_hphl_ocp_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"HPH_L OCP detect".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).mbhc_hs_rem_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    ret = request_threaded_irq((*(*mbhc).intr_ids).hph_right_ocp, None, Some(wcd_mbhc_hphr_ocp_irq), IRQF_ONESHOT | IRQF_TRIGGER_RISING, c"HPH_R OCP detect".as_ptr(), mbhc as *mut c_void);
    if ret != 0 {
        free_irq((*(*mbhc).intr_ids).hph_left_ocp, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_hs_rem_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
        free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
        kfree(mbhc as *mut c_void);
        dev_err(dev, c"Failed to request mbhc interrupts %d\n".as_ptr(), ret);
        return ERR_PTR(ret as c_long);
    }
    mbhc
}

pub unsafe extern "C" fn wcd_mbhc_deinit(mbhc: *mut wcd_mbhc) {
    free_irq((*(*mbhc).intr_ids).hph_right_ocp, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).hph_left_ocp, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).mbhc_hs_rem_intr, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).mbhc_hs_ins_intr, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).mbhc_btn_release_intr, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).mbhc_btn_press_intr, mbhc as *mut c_void);
    free_irq((*(*mbhc).intr_ids).mbhc_sw_intr, mbhc as *mut c_void);
    mutex_lock(&mut (*mbhc).lock);
    wcd_cancel_hs_detect_plug(mbhc, &mut (*mbhc).correct_plug_swch);
    cancel_work_sync(&mut (*mbhc).mbhc_plug_detect_work);
    mutex_unlock(&mut (*mbhc).lock);
    kfree(mbhc as *mut c_void);
}

// MODULE_DESCRIPTION("wcd MBHC v2 module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
