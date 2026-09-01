/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_ulong};

/* Original C dependency: #include <sound/jack.h> */

macro_rules! WCD_MBHC_FIELD {
    ($id:expr, $rreg:expr, $rmask:expr) => {
        (
            $id,
            wcd_mbhc_field {
                reg: $rreg,
                mask: $rmask,
            },
        )
    };
}

pub(crate) use WCD_MBHC_FIELD;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_field_function {
    WCD_MBHC_L_DET_EN,
    WCD_MBHC_GND_DET_EN,
    WCD_MBHC_MECH_DETECTION_TYPE,
    WCD_MBHC_MIC_CLAMP_CTL,
    WCD_MBHC_ELECT_DETECTION_TYPE,
    WCD_MBHC_HS_L_DET_PULL_UP_CTRL,
    WCD_MBHC_HS_L_DET_PULL_UP_COMP_CTRL,
    WCD_MBHC_HPHL_PLUG_TYPE,
    WCD_MBHC_GND_PLUG_TYPE,
    WCD_MBHC_SW_HPH_LP_100K_TO_GND,
    WCD_MBHC_ELECT_SCHMT_ISRC,
    WCD_MBHC_FSM_EN,
    WCD_MBHC_INSREM_DBNC,
    WCD_MBHC_BTN_DBNC,
    WCD_MBHC_HS_VREF,
    WCD_MBHC_HS_COMP_RESULT,
    WCD_MBHC_IN2P_CLAMP_STATE,
    WCD_MBHC_MIC_SCHMT_RESULT,
    WCD_MBHC_HPHL_SCHMT_RESULT,
    WCD_MBHC_HPHR_SCHMT_RESULT,
    WCD_MBHC_OCP_FSM_EN,
    WCD_MBHC_BTN_RESULT,
    WCD_MBHC_BTN_ISRC_CTL,
    WCD_MBHC_ELECT_RESULT,
    WCD_MBHC_MICB_CTRL, /* Pull-up and micb control */
    WCD_MBHC_HPH_CNP_WG_TIME,
    WCD_MBHC_HPHR_PA_EN,
    WCD_MBHC_HPHL_PA_EN,
    WCD_MBHC_HPH_PA_EN,
    WCD_MBHC_SWCH_LEVEL_REMOVE,
    WCD_MBHC_PULLDOWN_CTRL,
    WCD_MBHC_ANC_DET_EN,
    WCD_MBHC_FSM_STATUS,
    WCD_MBHC_MUX_CTL,
    WCD_MBHC_MOISTURE_STATUS,
    WCD_MBHC_HPHR_GND,
    WCD_MBHC_HPHL_GND,
    WCD_MBHC_HPHL_OCP_DET_EN,
    WCD_MBHC_HPHR_OCP_DET_EN,
    WCD_MBHC_HPHL_OCP_STATUS,
    WCD_MBHC_HPHR_OCP_STATUS,
    WCD_MBHC_ADC_EN,
    WCD_MBHC_ADC_COMPLETE,
    WCD_MBHC_ADC_TIMEOUT,
    WCD_MBHC_ADC_RESULT,
    WCD_MBHC_MICB2_VOUT,
    WCD_MBHC_ADC_MODE,
    WCD_MBHC_DETECTION_DONE,
    WCD_MBHC_ELECT_ISRC_EN,
    WCD_MBHC_REG_FUNC_MAX,
}

pub const WCD_MBHC_DEF_BUTTONS: usize = 8;
pub const WCD_MBHC_KEYCODE_NUM: usize = 8;
pub const WCD_MBHC_USLEEP_RANGE_MARGIN_US: c_int = 100;
pub const WCD_MBHC_THR_HS_MICB_MV: c_int = 2700;
pub const WCD_MONO_HS_MIN_THR: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_detect_logic {
    WCD_DETECTION_LEGACY,
    WCD_DETECTION_ADC,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_cs_mb_en_flag {
    WCD_MBHC_EN_CS = 0,
    WCD_MBHC_EN_MB,
    WCD_MBHC_EN_PULLUP,
    WCD_MBHC_EN_NONE,
}

pub const WCD_MBHC_ELEC_HS_INS: c_int = 0;
pub const WCD_MBHC_ELEC_HS_REM: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_plug_type {
    MBHC_PLUG_TYPE_INVALID = -1,
    MBHC_PLUG_TYPE_NONE,
    MBHC_PLUG_TYPE_HEADSET,
    MBHC_PLUG_TYPE_HEADPHONE,
    MBHC_PLUG_TYPE_HIGH_HPH,
    MBHC_PLUG_TYPE_GND_MIC_SWAP,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pa_dac_ack_flags {
    WCD_MBHC_HPHL_PA_OFF_ACK = 0,
    WCD_MBHC_HPHR_PA_OFF_ACK,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_btn_det_mem {
    WCD_MBHC_BTN_DET_V_BTN_LOW,
    WCD_MBHC_BTN_DET_V_BTN_HIGH,
}

pub const MIC_BIAS_1: c_int = 1;
pub const MIC_BIAS_2: c_int = 2;
pub const MIC_BIAS_3: c_int = 3;
pub const MIC_BIAS_4: c_int = 4;

pub const MICB_PULLUP_ENABLE: c_int = 0;
pub const MICB_PULLUP_DISABLE: c_int = 1;
pub const MICB_ENABLE: c_int = 2;
pub const MICB_DISABLE: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_notify_event {
    WCD_EVENT_INVALID,
    /* events for micbias ON and OFF */
    WCD_EVENT_PRE_MICBIAS_2_OFF,
    WCD_EVENT_POST_MICBIAS_2_OFF,
    WCD_EVENT_PRE_MICBIAS_2_ON,
    WCD_EVENT_POST_MICBIAS_2_ON,
    WCD_EVENT_PRE_DAPM_MICBIAS_2_OFF,
    WCD_EVENT_POST_DAPM_MICBIAS_2_OFF,
    WCD_EVENT_PRE_DAPM_MICBIAS_2_ON,
    WCD_EVENT_POST_DAPM_MICBIAS_2_ON,
    /* events for PA ON and OFF */
    WCD_EVENT_PRE_HPHL_PA_ON,
    WCD_EVENT_POST_HPHL_PA_OFF,
    WCD_EVENT_PRE_HPHR_PA_ON,
    WCD_EVENT_POST_HPHR_PA_OFF,
    WCD_EVENT_PRE_HPHL_PA_OFF,
    WCD_EVENT_PRE_HPHR_PA_OFF,
    WCD_EVENT_OCP_OFF,
    WCD_EVENT_OCP_ON,
    WCD_EVENT_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_event_state {
    WCD_MBHC_EVENT_PA_HPHL,
    WCD_MBHC_EVENT_PA_HPHR,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum wcd_mbhc_hph_type {
    WCD_MBHC_HPH_NONE = 0,
    WCD_MBHC_HPH_MONO,
    WCD_MBHC_HPH_STEREO,
}

/*
 * These enum definitions are directly mapped to the register
 * definitions
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mbhc_hs_pullup_iref {
    I_DEFAULT = -1,
    I_OFF = 0,
    I_1P0_UA,
    I_2P0_UA,
    I_3P0_UA,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mbhc_hs_pullup_iref_v2 {
    HS_PULLUP_I_DEFAULT = -1,
    HS_PULLUP_I_3P0_UA = 0,
    HS_PULLUP_I_2P25_UA,
    HS_PULLUP_I_1P5_UA,
    HS_PULLUP_I_0P75_UA,
    HS_PULLUP_I_1P125_UA = 0x05,
    HS_PULLUP_I_0P375_UA = 0x07,
    HS_PULLUP_I_2P0_UA,
    HS_PULLUP_I_1P0_UA = 0x0A,
    HS_PULLUP_I_0P5_UA,
    HS_PULLUP_I_0P25_UA = 0x0F,
    HS_PULLUP_I_0P125_UA = 0x17,
    HS_PULLUP_I_OFF,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mbhc_moisture_rref {
    R_OFF,
    R_24_KOHM,
    R_84_KOHM,
    R_184_KOHM,
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wcd_mbhc_config {
    pub btn_high: [c_int; WCD_MBHC_DEF_BUTTONS],
    pub btn_low: [c_int; WCD_MBHC_DEF_BUTTONS],
    pub v_hs_max: c_int,
    pub num_btn: c_int,
    pub mono_stero_detection: bool,
    pub typec_analog_mux: bool,
    pub swap_gnd_mic: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> bool>,
    pub hs_ext_micbias: bool,
    pub gnd_det_en: bool,
    pub linein_th: u32,
    pub moisture_en: bool,
    pub mbhc_micbias: c_int,
    pub anc_micbias: c_int,
    pub moisture_duty_cycle_en: bool,
    pub hphl_swh: bool, /*track HPHL switch NC / NO */
    pub gnd_swh: bool,  /*track GND switch NC / NO */
    pub hs_thr: u32,
    pub hph_thr: u32,
    pub micb_mv: u32,
    pub moist_vref: u32,
    pub moist_iref: u32,
    pub moist_rref: u32,
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
pub struct wcd_mbhc_field {
    pub reg: u16,
    pub mask: u8,
}

#[repr(C)]
pub struct wcd_mbhc {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wcd_mbhc_cb {
    pub update_cross_conn_thr:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component)>,
    pub get_micbias_val:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, mb: *mut c_int)>,
    pub bcs_enable:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, bcs_enable: bool)>,
    pub compute_impedance: Option<
        unsafe extern "C" fn(component: *mut snd_soc_component, zl: *mut u32, zr: *mut u32),
    >,
    pub set_micbias_value:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component)>,
    pub set_auto_zeroing:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub clk_setup:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub micbias_enable_status:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, micb_num: c_int) -> bool>,
    pub mbhc_bias:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub set_btn_thr: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            btn_low: *mut c_int,
            btn_high: *mut c_int,
            num_btn: c_int,
            is_micbias: bool,
        ),
    >,
    pub hph_pull_up_control: Option<
        unsafe extern "C" fn(component: *mut snd_soc_component, _: mbhc_hs_pullup_iref),
    >,
    pub mbhc_micbias_control: Option<
        unsafe extern "C" fn(component: *mut snd_soc_component, micb_num: c_int, req: c_int)
            -> c_int,
    >,
    pub mbhc_micb_ramp_control:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub extn_use_mb:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> bool>,
    pub mbhc_micb_ctrl_thr_mic: Option<
        unsafe extern "C" fn(component: *mut snd_soc_component, micb_num: c_int, req_en: bool)
            -> c_int,
    >,
    pub mbhc_gnd_det_ctrl:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub hph_pull_down_ctrl:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub mbhc_moisture_config:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component)>,
    pub update_anc_state: Option<
        unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool, anc_num: c_int),
    >,
    pub hph_pull_up_control_v2:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, pull_up_cur: c_int)>,
    pub mbhc_get_moisture_status:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> bool>,
    pub mbhc_moisture_polling_ctrl:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
    pub mbhc_moisture_detect_en:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, enable: bool)>,
}

/* Original C condition: #if IS_ENABLED(CONFIG_SND_SOC_WCD_MBHC) */
#[cfg(CONFIG_SND_SOC_WCD_MBHC)]
unsafe extern "C" {
    pub fn wcd_dt_parse_mbhc_data(dev: *mut device, cfg: *mut wcd_mbhc_config) -> c_int;
    pub fn wcd_mbhc_start(
        mbhc: *mut wcd_mbhc,
        mbhc_cfg: *mut wcd_mbhc_config,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    pub fn wcd_mbhc_stop(mbhc: *mut wcd_mbhc);
    pub fn wcd_mbhc_set_hph_type(mbhc: *mut wcd_mbhc, hph_type: c_int);
    pub fn wcd_mbhc_get_hph_type(mbhc: *mut wcd_mbhc) -> c_int;
    pub fn wcd_mbhc_typec_report_plug(mbhc: *mut wcd_mbhc) -> c_int;
    pub fn wcd_mbhc_typec_report_unplug(mbhc: *mut wcd_mbhc) -> c_int;
    pub fn wcd_mbhc_init(
        component: *mut snd_soc_component,
        mbhc_cb: *const wcd_mbhc_cb,
        mbhc_cdc_intr_ids: *const wcd_mbhc_intr,
        fields: *const wcd_mbhc_field,
        impedance_det_en: bool,
    ) -> *mut wcd_mbhc;
    pub fn wcd_mbhc_get_impedance(mbhc: *mut wcd_mbhc, zl: *mut u32, zr: *mut u32) -> c_int;
    pub fn wcd_mbhc_deinit(mbhc: *mut wcd_mbhc);
    pub fn wcd_mbhc_event_notify(mbhc: *mut wcd_mbhc, event: c_ulong) -> c_int;
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
unsafe extern "C" {
    static ENOTSUPP: c_int;
    static EINVAL: c_int;

    fn ERR_PTR(error: isize) -> *mut wcd_mbhc;
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_dt_parse_mbhc_data(
    _dev: *mut device,
    _cfg: *mut wcd_mbhc_config,
) -> c_int {
    unsafe { -ENOTSUPP }
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_stop(_mbhc: *mut wcd_mbhc) {}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_init(
    _component: *mut snd_soc_component,
    _mbhc_cb: *const wcd_mbhc_cb,
    _mbhc_cdc_intr_ids: *const wcd_mbhc_intr,
    _fields: *const wcd_mbhc_field,
    _impedance_det_en: bool,
) -> *mut wcd_mbhc {
    unsafe { ERR_PTR(-ENOTSUPP as isize) }
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_set_hph_type(_mbhc: *mut wcd_mbhc, _hph_type: c_int) {}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_get_hph_type(_mbhc: *mut wcd_mbhc) -> c_int {
    unsafe { -ENOTSUPP }
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_event_notify(_mbhc: *mut wcd_mbhc, _event: c_ulong) -> c_int {
    unsafe { -ENOTSUPP }
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_start(
    _mbhc: *mut wcd_mbhc,
    _mbhc_cfg: *mut wcd_mbhc_config,
    _jack: *mut snd_soc_jack,
) -> c_int {
    0
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_get_impedance(
    _mbhc: *mut wcd_mbhc,
    zl: *mut u32,
    zr: *mut u32,
) -> c_int {
    unsafe {
        *zl = 0;
        *zr = 0;
        -EINVAL
    }
}

#[cfg(not(CONFIG_SND_SOC_WCD_MBHC))]
pub unsafe fn wcd_mbhc_deinit(_mbhc: *mut wcd_mbhc) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
