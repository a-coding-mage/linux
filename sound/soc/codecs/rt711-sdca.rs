// SPDX-License-Identifier: GPL-2.0-only
//
// rt711-sdca.c -- rt711 SDCA ALSA SoC audio driver
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// C includes removed. External Linux, SoundWire, ASoC, and rt711-sdca.h
// declarations/constants/macros are expected to be supplied by the surrounding
// translated repository.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
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
pub struct sdw_slave {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_union,
}
#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub reg2: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
    pub mask: c_uint,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_stream_config {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_port_config {
    pub num: c_uint,
}
#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct rt711_sdca_priv {
    pub slave: *mut sdw_slave,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub hs_jack: *mut snd_soc_jack,
    pub calibrate_mutex: mutex,
    pub disable_irq_lock: mutex,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub disable_irq: bool,
    pub fu0f_dapm_mute: bool,
    pub fu1e_dapm_mute: bool,
    pub fu0f_mixer_l_mute: bool,
    pub fu0f_mixer_r_mute: bool,
    pub fu1e_mixer_l_mute: bool,
    pub fu1e_mixer_r_mute: bool,
    pub jd_src: c_uint,
    pub ge_mode_override: c_uint,
    pub jack_type: c_int,
    pub scp_sdca_stat1: c_uint,
    pub scp_sdca_stat2: c_uint,
    pub hw_ver: c_uint,
}

type c_long = isize;

extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_get_device(map: *mut regmap) -> *mut device;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn set_mask_bits(ptr: *mut c_uint, mask: c_uint, val: c_uint);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn usleep_range(min: c_uint, max: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn mod_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn sdw_write_no_pm(slave: *mut sdw_slave, addr: c_uint, value: c_uint) -> c_int;
    fn snd_soc_enum_val_to_item(e: *mut soc_enum, val: c_uint) -> c_uint;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, item: c_uint, e: *mut soc_enum, update: *mut c_void) -> c_int;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_sdw_params_to_config(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, stream: *mut sdw_stream_config, port: *mut sdw_port_config);
    fn sdw_stream_add_slave(slave: *mut sdw_slave, stream_config: *mut sdw_stream_config, port_config: *mut sdw_port_config, num_ports: c_uint, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(slave: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err_ratelimited(fmt: *const c_char, ...);
}

type c_ulong = usize;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! BIT {
    ($n:expr) => {
        (1u32 << $n)
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_uint)
    };
}

extern "C" {
    static RT711_VENDOR_REG: c_uint;
    static RT711_VENDOR_HDA_CTL: c_uint;
    static RT711_VENDOR_ANALOG_CTL: c_uint;
    static RT711_VENDOR_CALI: c_uint;
    static RT711_VENDOR_VAD: c_uint;
    static RT711_PARA_VERB_CTL: c_uint;
    static RT711_HIDDEN_REG_SW_RESET: c_uint;
    static RT711_HDA_LEGACY_RESET_CTL: c_uint;
    static RT711_COMBO_JACK_AUTO_CTL1: c_uint;
    static RT711_PUSH_BTN_INT_CTL0: c_uint;
    static RT711_RC_CAL_STATUS: c_uint;
    static RT711_MISC_POWER_CTL0: c_uint;
    static RT711_FSM_CTL: c_uint;
    static RT711_CALI_CTL: c_uint;
    static RT711_DAC_DC_CALI_CTL1: c_uint;
    static RT711_DAC_DC_FORCE_CALI_RST: c_uint;
    static RT711_DAC_DC_CALI_CLK_EN: c_uint;
    static RT711_DAC_DC_CALI_TRIGGER: c_uint;
    static RT711_VER_VD0: c_uint;
    static RT711_FSM_IMP_EN: c_uint;
    static RT711_DIGITAL_MISC_CTRL4: c_uint;
    static FUNC_NUM_HID: c_uint;
    static FUNC_NUM_JACK_CODEC: c_uint;
    static FUNC_NUM_MIC_ARRAY: c_uint;
    static RT711_SDCA_ENT_HID01: c_uint;
    static RT711_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint;
    static RT711_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint;
    static RT711_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE: c_uint;
    static RT711_BUF_ADDR_HID1: c_uint;
    static RT711_SDCA_ENT_GE49: c_uint;
    static RT711_SDCA_CTL_DETECTED_MODE: c_uint;
    static RT711_SDCA_CTL_SELECTED_MODE: c_uint;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SDW_SCP_SDCA_INT_SDCA_0: c_uint;
    static SDW_SCP_SDCA_INT_SDCA_8: c_uint;
    static RT711_PUSH_BTN_INT_CTL6: c_uint;
    static RT711_PUSH_BTN_INT_CTL2: c_uint;
    static RT711_PUSH_BTN_INT_CTL7: c_uint;
    static RT711_PUSH_BTN_INT_CTL9: c_uint;
    static RT711_GE_MODE_RELATED_CTL: c_uint;
    static RT711_JD1: c_uint;
    static RT711_JD2: c_uint;
    static RT711_JD2_100K: c_uint;
    static RT711_JD_CTL1: c_uint;
    static RT711_JD2_DIGITAL_MODE_SEL: c_uint;
    static RT711_JD_CTL2: c_uint;
    static RT711_JD2_2PORT_200K_DECODE_HP: c_uint;
    static RT711_HP_JD_SEL_JD2: c_uint;
    static RT711_CC_DET1: c_uint;
    static RT711_HP_JD_FINAL_RESULT_CTL_JD12: c_uint;
    static RT711_COMBO_JACK_AUTO_CTL3: c_uint;
    static RT711_JD2_2PORT_100K_DECODE_MASK: c_uint;
    static RT711_JD2_2PORT_100K_DECODE_HP: c_uint;
    static RT711_POW_CC1_AGPI: c_uint;
    static RT711_POW_CC1_AGPI_OFF: c_uint;
    static SDW_SCP_SDCA_INTMASK1: c_uint;
    static SDW_SCP_SDCA_INTMASK2: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_0: c_uint;
    static SDW_SCP_SDCA_INTMASK_SDCA_8: c_uint;
    static SND_SOC_NOPM: c_uint;
    static RT711_SDCA_ENT_USER_FU05: c_uint;
    static RT711_SDCA_ENT_USER_FU0F: c_uint;
    static RT711_SDCA_ENT_USER_FU1E: c_uint;
    static RT711_SDCA_ENT_PLATFORM_FU44: c_uint;
    static RT711_SDCA_ENT_PLATFORM_FU15: c_uint;
    static RT711_SDCA_CTL_FU_VOLUME: c_uint;
    static RT711_SDCA_CTL_FU_CH_GAIN: c_uint;
    static RT711_SDCA_CTL_FU_MUTE: c_uint;
    static CH_L: c_uint;
    static CH_R: c_uint;
    static RT711_HDA_LEGACY_MUX_CTL1: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static RT711_SDCA_ENT_PDE28: c_uint;
    static RT711_SDCA_ENT_PDE29: c_uint;
    static RT711_SDCA_ENT_PDE2A: c_uint;
    static RT711_SDCA_ENT_PDELINE2: c_uint;
    static RT711_SDCA_ENT_LINE1: c_uint;
    static RT711_SDCA_ENT_LINE2: c_uint;
    static RT711_SDCA_CTL_REQ_POWER_STATE: c_uint;
    static RT711_SDCA_CTL_VENDOR_DEF: c_uint;
    static RT711_AIF1: c_int;
    static RT711_AIF2: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static RT711_SDCA_ENT_CS01: c_uint;
    static RT711_SDCA_ENT_CS11: c_uint;
    static RT711_SDCA_ENT_CS1F: c_uint;
    static RT711_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint;
    static RT711_SDCA_RATE_44100HZ: c_uint;
    static RT711_SDCA_RATE_48000HZ: c_uint;
    static RT711_SDCA_RATE_96000HZ: c_uint;
    static RT711_SDCA_RATE_192000HZ: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static GFP_KERNEL: c_uint;
    static RT711_JD_PRODUCT_NUM: c_uint;
    static RT711_GPIO_TEST_MODE_CTL2: c_uint;
    static RT711_HDA_LEGACY_GPIO_CTL: c_uint;
    static RT711_ADC27_VOL_SET: c_uint;
    static RT711_MISC_POWER_CTL4: c_uint;
    static RT711_HDA_LEGACY_CONFIG_CTL: c_uint;
    static RT711_VAD_SRAM_CTL1: c_uint;
    static RT711_HDA_LEGACY_UNSOLICITED_CTL: c_uint;
    static RT711_JD_CTRL6: c_uint;
    static RT711_DMIC_CTL1: c_uint;
    static RT711_FILTER_SRC_SEL: c_uint;
}

const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;

unsafe fn SDW_SDCA_CTL(function: c_uint, entity: c_uint, control: c_uint, channel: c_uint) -> c_uint {
    (function << 16) | (entity << 8) | (control << 4) | channel
}

unsafe extern "C" fn rt711_sdca_index_write(
    rt711: *mut rt711_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let regmap = (*rt711).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_write(regmap, addr, value);
    if ret < 0 {
        dev_err(
            &mut (*(*rt711).slave).dev,
            cstr!("%s: Failed to set private value: %06x <= %04x ret=%d\n"),
            cstr!("rt711_sdca_index_write"),
            addr,
            value,
            ret,
        );
    }
    ret
}

unsafe extern "C" fn rt711_sdca_index_read(
    rt711: *mut rt711_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let regmap = (*rt711).mbq_regmap;
    let addr = (nid << 20) | reg;
    let ret = regmap_read(regmap, addr, value);
    if ret < 0 {
        dev_err(
            &mut (*(*rt711).slave).dev,
            cstr!("%s: Failed to get private value: %06x => %04x ret=%d\n"),
            cstr!("rt711_sdca_index_read"),
            addr,
            *value,
            ret,
        );
    }
    ret
}

unsafe extern "C" fn rt711_sdca_index_update_bits(
    rt711: *mut rt711_sdca_priv,
    nid: c_uint,
    reg: c_uint,
    mask: c_uint,
    val: c_uint,
) -> c_int {
    let mut tmp: c_uint = 0;
    let ret = rt711_sdca_index_read(rt711, nid, reg, &mut tmp);
    if ret < 0 {
        return ret;
    }
    set_mask_bits(&mut tmp, mask, val);
    rt711_sdca_index_write(rt711, nid, reg, tmp)
}

unsafe extern "C" fn rt711_sdca_reset(rt711: *mut rt711_sdca_priv) {
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_PARA_VERB_CTL, RT711_HIDDEN_REG_SW_RESET, RT711_HIDDEN_REG_SW_RESET);
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_RESET_CTL, 0x1, 0x1);
}

unsafe extern "C" fn rt711_sdca_ge_force_jack_type(rt711: *mut rt711_sdca_priv, det_mode: c_uint) {
    match det_mode {
        0x00 => {
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL1, 0x8400, 0x0000);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL0, 0x10, 0x00);
        }
        0x03 => {
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL1, 0x8400, 0x8000);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL0, 0x17, 0x13);
        }
        0x05 => {
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL1, 0x8400, 0x8400);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL0, 0x17, 0x15);
        }
        _ => {}
    }
}

unsafe extern "C" fn rt711_sdca_calibration(rt711: *mut rt711_sdca_priv) -> c_int {
    let mut val: c_uint = 0;
    let mut loop_rc: c_uint = 0;
    let mut loop_dc: c_uint = 0;
    let regmap = (*rt711).regmap;
    let chk_cnt: c_int = 100;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*rt711).calibrate_mutex);
    let dev = regmap_get_device(regmap);

    regmap_read((*rt711).regmap, RT711_RC_CAL_STATUS, &mut val);
    /* RC calibration */
    if (val & 0x40) == 0 {
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_ANALOG_CTL, RT711_MISC_POWER_CTL0, 0x0010, 0x0010);
    }

    while loop_rc < chk_cnt as c_uint && (val & 0x40) == 0 {
        usleep_range(10000, 11000);
        ret = regmap_read((*rt711).regmap, RT711_RC_CAL_STATUS, &mut val);
        if ret < 0 {
            break;
        }
        loop_rc += 1;
    }
    if ret >= 0 && loop_rc == chk_cnt as c_uint {
        dev_err(dev, cstr!("%s, RC calibration time-out!\n"), cstr!("rt711_sdca_calibration"));
    }

    if ret >= 0 {
        /* HP calibration by manual mode setting */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_FSM_CTL, 0x2000, 0x2000);
        /* Calibration manual mode */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_FSM_CTL, 0xf, RT711_CALI_CTL);
        /* reset HP calibration */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, RT711_DAC_DC_FORCE_CALI_RST, 0x00);
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, RT711_DAC_DC_FORCE_CALI_RST, RT711_DAC_DC_FORCE_CALI_RST);
        /* cal_clk_en_reg */
        if (*rt711).hw_ver == RT711_VER_VD0 {
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, RT711_DAC_DC_CALI_CLK_EN, RT711_DAC_DC_CALI_CLK_EN);
        }
        /* trigger */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, RT711_DAC_DC_CALI_TRIGGER, RT711_DAC_DC_CALI_TRIGGER);
        /* wait for calibration process */
        rt711_sdca_index_read(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, &mut val);
        while loop_dc < chk_cnt as c_uint && (val & RT711_DAC_DC_CALI_TRIGGER) != 0 {
            usleep_range(10000, 11000);
            ret = rt711_sdca_index_read(rt711, RT711_VENDOR_CALI, RT711_DAC_DC_CALI_CTL1, &mut val);
            if ret < 0 {
                break;
            }
            loop_dc += 1;
        }
        if ret >= 0 && loop_dc == chk_cnt as c_uint {
            dev_err(dev, cstr!("%s, calibration time-out!\n"), cstr!("rt711_sdca_calibration"));
        }
        if ret >= 0 && (loop_dc == chk_cnt as c_uint || loop_rc == chk_cnt as c_uint) {
            ret = -ETIMEDOUT;
        }
    }

    /* enable impedance sense */
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_FSM_CTL, RT711_FSM_IMP_EN, RT711_FSM_IMP_EN);
    /* release HP-JD and trigger FSM */
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_DIGITAL_MISC_CTRL4, 0x201b);
    mutex_unlock(&mut (*rt711).calibrate_mutex);
    dev_dbg(dev, cstr!("%s calibration complete, ret=%d\n"), cstr!("rt711_sdca_calibration"), ret);
    ret
}

unsafe fn rt711_sdca_decode_button_buf(buf: &[u8; 3]) -> c_uint {
    let mut btn_type: c_uint = 0;
    if buf[0] == 0x11 {
        match buf[1] & 0xf0 {
            0x10 => btn_type |= SND_JACK_BTN_2 as c_uint,
            0x20 => btn_type |= SND_JACK_BTN_3 as c_uint,
            0x40 => btn_type |= SND_JACK_BTN_0 as c_uint,
            0x80 => btn_type |= SND_JACK_BTN_1 as c_uint,
            _ => {}
        }
        match buf[2] {
            0x01 | 0x10 => btn_type |= SND_JACK_BTN_2 as c_uint,
            0x02 | 0x20 => btn_type |= SND_JACK_BTN_3 as c_uint,
            0x04 | 0x40 => btn_type |= SND_JACK_BTN_0 as c_uint,
            0x08 | 0x80 => btn_type |= SND_JACK_BTN_1 as c_uint,
            _ => {}
        }
    }
    btn_type
}

unsafe extern "C" fn rt711_sdca_button_detect(rt711: *mut rt711_sdca_priv) -> c_uint {
    let mut btn_type: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut owner: c_uint = 0;
    let mut buf = [0u8; 3];

    /* get current UMP message owner */
    let mut ret = regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT711_SDCA_ENT_HID01, RT711_SDCA_CTL_HIDTX_CURRENT_OWNER, 0), &mut owner);
    if ret < 0 {
        return 0;
    }

    /* if owner is device then there is no button event from device */
    if owner == 1 {
        return 0;
    }

    /* read UMP message offset */
    ret = regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT711_SDCA_ENT_HID01, RT711_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
    if ret >= 0 {
        for idx in 0..buf.len() {
            ret = regmap_read((*rt711).regmap, RT711_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 {
                break;
            }
            buf[idx] = (val & 0xff) as u8;
        }
        if ret >= 0 {
            btn_type = rt711_sdca_decode_button_buf(&buf);
        }
    }

    /* Host is owner, so set back to device */
    if owner == 0 {
        /* set owner to device */
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT711_SDCA_ENT_HID01, RT711_SDCA_CTL_HIDTX_SET_OWNER_TO_DEVICE, 0), 0x01);
    }
    btn_type
}

unsafe extern "C" fn rt711_sdca_headset_detect(rt711: *mut rt711_sdca_priv) -> c_int {
    let mut det_mode: c_uint = 0;
    rt711_sdca_ge_force_jack_type(rt711, (*rt711).ge_mode_override);

    /* get detected_mode */
    let mut ret = regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_sdca_headset_detect"), ret);
        return ret;
    }

    match det_mode {
        0x00 => (*rt711).jack_type = 0,
        0x03 => (*rt711).jack_type = SND_JACK_HEADPHONE,
        0x05 => (*rt711).jack_type = SND_JACK_HEADSET,
        _ => {}
    }

    /* write selected_mode */
    if det_mode != 0 {
        ret = regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_SELECTED_MODE, 0), det_mode);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_sdca_headset_detect"), ret);
            return ret;
        }
    }

    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("%s, detected_mode=0x%x\n"), cstr!("rt711_sdca_headset_detect"), det_mode);
    0
}

unsafe extern "C" fn rt711_sdca_jack_detect_handler(work: *mut work_struct) {
    let rt711 = container_of!(work, rt711_sdca_priv, jack_detect_work.work);
    let mut btn_type: c_int = 0;

    if (*rt711).hs_jack.is_null() {
        return;
    }
    if !snd_soc_card_is_instantiated((*(*rt711).component).card) {
        return;
    }

    /* SDW_SCP_SDCA_INT_SDCA_0 is used for jack detection */
    if ((*rt711).scp_sdca_stat1 & SDW_SCP_SDCA_INT_SDCA_0) != 0 {
        let ret = rt711_sdca_headset_detect(rt711);
        if ret < 0 {
            return;
        }
    }

    /* SDW_SCP_SDCA_INT_SDCA_8 is used for button detection */
    if ((*rt711).scp_sdca_stat2 & SDW_SCP_SDCA_INT_SDCA_8) != 0 {
        btn_type = rt711_sdca_button_detect(rt711) as c_int;
    }
    if (*rt711).jack_type == 0 {
        btn_type = 0;
    }

    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s, jack_type=0x%x\n"), cstr!("rt711_sdca_jack_detect_handler"), (*rt711).jack_type);
    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s, btn_type=0x%x\n"), cstr!("rt711_sdca_jack_detect_handler"), btn_type);
    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s, scp_sdca_stat1=0x%x, scp_sdca_stat2=0x%x\n"), cstr!("rt711_sdca_jack_detect_handler"), (*rt711).scp_sdca_stat1, (*rt711).scp_sdca_stat2);

    snd_soc_jack_report((*rt711).hs_jack, (*rt711).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt711).hs_jack, (*rt711).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt711).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe extern "C" fn rt711_sdca_btn_check_handler(work: *mut work_struct) {
    let rt711 = container_of!(work, rt711_sdca_priv, jack_btn_check_work.work);
    let mut btn_type: c_int = 0;
    let mut det_mode: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut val: c_uint = 0;
    let mut buf = [0u8; 3];

    let mut ret = regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_DETECTED_MODE, 0), &mut det_mode);
    if ret < 0 {
        pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_sdca_btn_check_handler"), ret);
        return;
    }

    /* pin attached */
    if det_mode != 0 {
        /* read UMP message offset */
        ret = regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_HID, RT711_SDCA_ENT_HID01, RT711_SDCA_CTL_HIDTX_MESSAGE_OFFSET, 0), &mut offset);
        if ret < 0 {
            pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_sdca_btn_check_handler"), ret);
            return;
        }
        for idx in 0..buf.len() {
            ret = regmap_read((*rt711).regmap, RT711_BUF_ADDR_HID1 + offset + idx as c_uint, &mut val);
            if ret < 0 {
                pr_err_ratelimited(cstr!("IO error in %s, ret %d\n"), cstr!("rt711_sdca_btn_check_handler"), ret);
                return;
            }
            buf[idx] = (val & 0xff) as u8;
        }
        btn_type = rt711_sdca_decode_button_buf(&buf) as c_int;
    } else {
        (*rt711).jack_type = 0;
    }

    dev_dbg(&mut (*(*rt711).slave).dev, cstr!("%s, btn_type=0x%x\n"), cstr!("rt711_sdca_btn_check_handler"), btn_type);
    snd_soc_jack_report((*rt711).hs_jack, (*rt711).jack_type | btn_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
    if btn_type != 0 {
        /* button released */
        snd_soc_jack_report((*rt711).hs_jack, (*rt711).jack_type, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
        mod_delayed_work(system_power_efficient_wq, &mut (*rt711).jack_btn_check_work, msecs_to_jiffies(200));
    }
}

unsafe extern "C" fn rt711_sdca_jack_init(rt711: *mut rt711_sdca_priv) {
    mutex_lock(&mut (*rt711).calibrate_mutex);
    if !(*rt711).hs_jack.is_null() {
        /* Enable HID1 event & set button RTC mode */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL6, 0x80f0, 0x8000);
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL2, 0x11dd, 0x11dd);
        rt711_sdca_index_write(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL7, 0xffff);
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL9, 0xf000, 0x0000);
        /* GE_mode_change_event_en & Hid1_push_button_event_en */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_GE_MODE_RELATED_CTL, 0x0c00, 0x0c00);

        if (*rt711).jd_src == RT711_JD1 {
            /* default settings was already for JD1 */
        } else if (*rt711).jd_src == RT711_JD2 {
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_JD_CTL1, RT711_JD2_DIGITAL_MODE_SEL, RT711_JD2_DIGITAL_MODE_SEL);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_JD_CTL2, RT711_JD2_2PORT_200K_DECODE_HP | RT711_HP_JD_SEL_JD2, RT711_JD2_2PORT_200K_DECODE_HP | RT711_HP_JD_SEL_JD2);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_CC_DET1, RT711_HP_JD_FINAL_RESULT_CTL_JD12, RT711_HP_JD_FINAL_RESULT_CTL_JD12);
        } else if (*rt711).jd_src == RT711_JD2_100K {
            rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL3, 0xa47e);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_JD_CTL1, RT711_JD2_DIGITAL_MODE_SEL, RT711_JD2_DIGITAL_MODE_SEL);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_JD_CTL2, RT711_JD2_2PORT_200K_DECODE_HP | RT711_JD2_2PORT_100K_DECODE_MASK | RT711_HP_JD_SEL_JD2, RT711_JD2_2PORT_100K_DECODE_HP | RT711_HP_JD_SEL_JD2);
            rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_CC_DET1, RT711_HP_JD_FINAL_RESULT_CTL_JD12 | RT711_POW_CC1_AGPI, RT711_HP_JD_FINAL_RESULT_CTL_JD12 | RT711_POW_CC1_AGPI_OFF);
        } else {
            dev_warn((*(*rt711).component).dev, cstr!("Wrong JD source\n"));
        }

        /* set SCP_SDCA_IntMask1[0]=1 */
        sdw_write_no_pm((*rt711).slave, SDW_SCP_SDCA_INTMASK1, SDW_SCP_SDCA_INTMASK_SDCA_0);
        /* set SCP_SDCA_IntMask2[0]=1 */
        sdw_write_no_pm((*rt711).slave, SDW_SCP_SDCA_INTMASK2, SDW_SCP_SDCA_INTMASK_SDCA_8);
        dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s enable\n"), cstr!("rt711_sdca_jack_init"));
    } else {
        /* disable HID 1/2 event */
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_GE_MODE_RELATED_CTL, 0x0c00, 0x0000);
        dev_dbg(&mut (*(*rt711).slave).dev, cstr!("in %s disable\n"), cstr!("rt711_sdca_jack_init"));
    }
    mutex_unlock(&mut (*rt711).calibrate_mutex);
}

unsafe extern "C" fn rt711_sdca_set_jack_detect(component: *mut snd_soc_component, hs_jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    (*rt711).hs_jack = hs_jack;
    /* we can only resume if the device was initialized at least once */
    if !(*rt711).first_hw_init {
        return 0;
    }
    let ret = pm_runtime_resume_and_get((*component).dev);
    if ret < 0 {
        if ret != -EACCES {
            dev_err((*component).dev, cstr!("%s: failed to resume %d\n"), cstr!("rt711_sdca_set_jack_detect"), ret);
            return ret;
        }
        /* pm_runtime not enabled yet */
        dev_dbg((*component).dev, cstr!("%s: skipping jack init for now\n"), cstr!("rt711_sdca_set_jack_detect"));
        return 0;
    }
    rt711_sdca_jack_init(rt711);
    pm_runtime_put_autosuspend((*component).dev);
    0
}

unsafe extern "C" fn rt711_sdca_set_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut read_l: c_uint = 0;
    let mut read_r: c_uint = 0;
    let mut gain_l_val: c_uint;
    let mut gain_r_val: c_uint;
    let mut i: c_uint;
    let mut adc_vol_flag: c_uint = 0;
    let mut changed: c_uint = 0;
    let mut lvalue: c_uint = 0;
    let mut rvalue: c_uint = 0;

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null()
        || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null()
    {
        adc_vol_flag = 1;
    }

    regmap_read((*rt711).mbq_regmap, (*mc).reg, &mut lvalue);
    regmap_read((*rt711).mbq_regmap, (*mc).rreg, &mut rvalue);

    /* control value to 2's complement value */
    /* L Channel */
    gain_l_val = (*ucontrol).value.integer.value[0] as c_uint;
    if gain_l_val > (*mc).max {
        gain_l_val = (*mc).max;
    }
    read_l = gain_l_val;
    if (*mc).shift == 8 {
        /* boost gain */
        gain_l_val = (gain_l_val * 10) << (*mc).shift;
    } else {
        /* ADC/DAC gain */
        if adc_vol_flag != 0 && gain_l_val > (*mc).shift {
            gain_l_val = (gain_l_val - (*mc).shift) * 75;
        } else {
            gain_l_val = ((*mc).shift - gain_l_val) * 75;
        }
        gain_l_val <<= 8;
        gain_l_val /= 100;
        if !(adc_vol_flag != 0 && read_l > (*mc).shift) {
            gain_l_val = !gain_l_val;
            gain_l_val = gain_l_val.wrapping_add(1);
        }
        gain_l_val &= 0xffff;
    }

    /* R Channel */
    gain_r_val = (*ucontrol).value.integer.value[1] as c_uint;
    if gain_r_val > (*mc).max {
        gain_r_val = (*mc).max;
    }
    read_r = gain_r_val;
    if (*mc).shift == 8 {
        /* boost gain */
        gain_r_val = (gain_r_val * 10) << (*mc).shift;
    } else {
        /* ADC/DAC gain */
        if adc_vol_flag != 0 && gain_r_val > (*mc).shift {
            gain_r_val = (gain_r_val - (*mc).shift) * 75;
        } else {
            gain_r_val = ((*mc).shift - gain_r_val) * 75;
        }
        gain_r_val <<= 8;
        gain_r_val /= 100;
        if !(adc_vol_flag != 0 && read_r > (*mc).shift) {
            gain_r_val = !gain_r_val;
            gain_r_val = gain_r_val.wrapping_add(1);
        }
        gain_r_val &= 0xffff;
    }

    if lvalue != gain_l_val || rvalue != gain_r_val {
        changed = 1;
    } else {
        return 0;
    }

    i = 0;
    while i < 3 {
        /* Lch*/
        regmap_write((*rt711).mbq_regmap, (*mc).reg, gain_l_val);
        /* Rch */
        regmap_write((*rt711).mbq_regmap, (*mc).rreg, gain_r_val);
        regmap_read((*rt711).mbq_regmap, (*mc).reg, &mut read_l);
        regmap_read((*rt711).mbq_regmap, (*mc).rreg, &mut read_r);
        if read_r == gain_r_val && read_l == gain_l_val {
            break;
        }
        i += 1;
    }

    if i == 3 { -EIO } else { changed as c_int }
}

unsafe extern "C" fn rt711_sdca_set_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut read_l: c_uint = 0;
    let mut read_r: c_uint = 0;
    let mut ctl_l: c_uint = 0;
    let mut ctl_r: c_uint = 0;
    let mut adc_vol_flag: c_uint = 0;
    let mut neg_flag: c_uint = 0;

    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU1E Capture Volume")).is_null()
        || !strstr((*ucontrol).id.name.as_ptr(), cstr!("FU0F Capture Volume")).is_null()
    {
        adc_vol_flag = 1;
    }
    regmap_read((*rt711).mbq_regmap, (*mc).reg, &mut read_l);
    regmap_read((*rt711).mbq_regmap, (*mc).rreg, &mut read_r);

    /* 2's complement value to control value */
    if (*mc).shift == 8 {
        /* boost gain */
        ctl_l = (read_l >> (*mc).shift) / 10;
    } else {
        /* ADC/DAC gain */
        ctl_l = read_l;
        if (read_l & BIT!(15)) != 0 {
            ctl_l = 0xffff & !(read_l - 1);
            neg_flag = 1;
        }
        ctl_l *= 100;
        ctl_l >>= 8;
        if adc_vol_flag != 0 {
            if neg_flag != 0 {
                ctl_l = (*mc).shift - (ctl_l / 75);
            } else {
                ctl_l = (*mc).shift + (ctl_l / 75);
            }
        } else {
            ctl_l = (*mc).max - (ctl_l / 75);
        }
    }

    neg_flag = 0;
    if read_l != read_r {
        if (*mc).shift == 8 {
            /* boost gain */
            ctl_r = (read_r >> (*mc).shift) / 10;
        } else {
            /* ADC/DAC gain */
            ctl_r = read_r;
            if (read_r & BIT!(15)) != 0 {
                ctl_r = 0xffff & !(read_r - 1);
                neg_flag = 1;
            }
            ctl_r *= 100;
            ctl_r >>= 8;
            if adc_vol_flag != 0 {
                if neg_flag != 0 {
                    ctl_r = (*mc).shift - (ctl_r / 75);
                } else {
                    ctl_r = (*mc).shift + (ctl_r / 75);
                }
            } else {
                ctl_r = (*mc).max - (ctl_r / 75);
            }
        }
    } else {
        ctl_r = ctl_l;
    }
    (*ucontrol).value.integer.value[0] = ctl_l as c_long;
    (*ucontrol).value.integer.value[1] = ctl_r as c_long;
    0
}

unsafe extern "C" fn rt711_sdca_set_fu0f_capture_ctl(rt711: *mut rt711_sdca_priv) -> c_int {
    let ch_l: c_uint = if (*rt711).fu0f_dapm_mute || (*rt711).fu0f_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_r: c_uint = if (*rt711).fu0f_dapm_mute || (*rt711).fu0f_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU0F, RT711_SDCA_CTL_FU_MUTE, CH_L), ch_l);
    if err < 0 { return err; }
    err = regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU0F, RT711_SDCA_CTL_FU_MUTE, CH_R), ch_r);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn rt711_sdca_set_fu1e_capture_ctl(rt711: *mut rt711_sdca_priv) -> c_int {
    let ch_l: c_uint = if (*rt711).fu1e_dapm_mute || (*rt711).fu1e_mixer_l_mute { 0x01 } else { 0x00 };
    let ch_r: c_uint = if (*rt711).fu1e_dapm_mute || (*rt711).fu1e_mixer_r_mute { 0x01 } else { 0x00 };
    let mut err = regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_USER_FU1E, RT711_SDCA_CTL_FU_MUTE, CH_L), ch_l);
    if err < 0 { return err; }
    err = regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_USER_FU1E, RT711_SDCA_CTL_FU_MUTE, CH_R), ch_r);
    if err < 0 { return err; }
    0
}

unsafe extern "C" fn rt711_sdca_fu1e_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    (*ucontrol).value.integer.value[0] = (!(*rt711).fu1e_mixer_l_mute) as c_long;
    (*ucontrol).value.integer.value[1] = (!(*rt711).fu1e_mixer_r_mute) as c_long;
    0
}

unsafe extern "C" fn rt711_sdca_fu1e_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut changed: c_int = 0;
    if (*rt711).fu1e_mixer_l_mute != ((*ucontrol).value.integer.value[0] == 0)
        || (*rt711).fu1e_mixer_r_mute != ((*ucontrol).value.integer.value[1] == 0)
    {
        changed = 1;
    }
    (*rt711).fu1e_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt711).fu1e_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt711_sdca_set_fu1e_capture_ctl(rt711);
    if err < 0 { return err; }
    changed
}

unsafe extern "C" fn rt711_sdca_fu0f_capture_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    (*ucontrol).value.integer.value[0] = (!(*rt711).fu0f_mixer_l_mute) as c_long;
    (*ucontrol).value.integer.value[1] = (!(*rt711).fu0f_mixer_r_mute) as c_long;
    0
}

unsafe extern "C" fn rt711_sdca_fu0f_capture_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut changed: c_int = 0;
    if (*rt711).fu0f_mixer_l_mute != ((*ucontrol).value.integer.value[0] == 0)
        || (*rt711).fu0f_mixer_r_mute != ((*ucontrol).value.integer.value[1] == 0)
    {
        changed = 1;
    }
    (*rt711).fu0f_mixer_l_mute = (*ucontrol).value.integer.value[0] == 0;
    (*rt711).fu0f_mixer_r_mute = (*ucontrol).value.integer.value[1] == 0;
    let err = rt711_sdca_set_fu0f_capture_ctl(rt711);
    if err < 0 { return err; }
    changed
}

unsafe extern "C" fn rt711_sdca_ge_select_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let e = (*kcontrol).private_value as *mut soc_enum;
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let val = ((*rt711).ge_mode_override >> (*e).shift_l) & (*e).mask;
    let item = snd_soc_enum_val_to_item(e, val);
    (*ucontrol).value.enumerated.item[0] = item;
    0
}

unsafe extern "C" fn rt711_sdca_ge_select_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let component = snd_kcontrol_chip(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut change: c_uint = 0;
    if *item >= (*e).items {
        return -EINVAL;
    }
    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    if (*rt711).ge_mode_override != val {
        (*rt711).ge_mode_override = val;
        change = 1;
    }
    change as c_int
}

static RT711_SDCA_GE_SELECT: [*const c_char; 3] = [
    cstr!("Auto"),
    cstr!("Headphone"),
    cstr!("Headset"),
];

static RT711_SDCA_GE_SELECT_VALUES: [c_int; 3] = [0, 3, 5];

// static SOC_VALUE_ENUM_SINGLE_DECL(rt711_sdca_ge_mode_enum, SND_SOC_NOPM,
// 	0, 0x7, rt711_sdca_ge_select, rt711_sdca_ge_select_values);
SOC_VALUE_ENUM_SINGLE_DECL!(rt711_sdca_ge_mode_enum, SND_SOC_NOPM, 0, 0x7, RT711_SDCA_GE_SELECT, RT711_SDCA_GE_SELECT_VALUES);

DECLARE_TLV_DB_SCALE!(out_vol_tlv, -6525, 75, 0);
DECLARE_TLV_DB_SCALE!(in_vol_tlv, -1725, 75, 0);
DECLARE_TLV_DB_SCALE!(mic_vol_tlv, 0, 1000, 0);

static RT711_SDCA_SND_CONTROLS: [snd_kcontrol_new; 8] = [
    SOC_DOUBLE_R_EXT_TLV!(cstr!("FU05 Playback Volume"), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_VOLUME, CH_L), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_VOLUME, CH_R), 0x57, 0x57, 0, rt711_sdca_set_gain_get, rt711_sdca_set_gain_put, out_vol_tlv),
    SOC_DOUBLE_EXT!(cstr!("FU1E Capture Switch"), SND_SOC_NOPM, 0, 1, 1, 0, rt711_sdca_fu1e_capture_get, rt711_sdca_fu1e_capture_put),
    SOC_DOUBLE_EXT!(cstr!("FU0F Capture Switch"), SND_SOC_NOPM, 0, 1, 1, 0, rt711_sdca_fu0f_capture_get, rt711_sdca_fu0f_capture_put),
    SOC_DOUBLE_R_EXT_TLV!(cstr!("FU1E Capture Volume"), SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_USER_FU1E, RT711_SDCA_CTL_FU_VOLUME, CH_L), SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_USER_FU1E, RT711_SDCA_CTL_FU_VOLUME, CH_R), 0x17, 0x3f, 0, rt711_sdca_set_gain_get, rt711_sdca_set_gain_put, in_vol_tlv),
    SOC_DOUBLE_R_EXT_TLV!(cstr!("FU0F Capture Volume"), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU0F, RT711_SDCA_CTL_FU_VOLUME, CH_L), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU0F, RT711_SDCA_CTL_FU_VOLUME, CH_R), 0x17, 0x3f, 0, rt711_sdca_set_gain_get, rt711_sdca_set_gain_put, in_vol_tlv),
    SOC_DOUBLE_R_EXT_TLV!(cstr!("FU44 Gain Volume"), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PLATFORM_FU44, RT711_SDCA_CTL_FU_CH_GAIN, CH_L), SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PLATFORM_FU44, RT711_SDCA_CTL_FU_CH_GAIN, CH_R), 8, 3, 0, rt711_sdca_set_gain_get, rt711_sdca_set_gain_put, mic_vol_tlv),
    SOC_DOUBLE_R_EXT_TLV!(cstr!("FU15 Gain Volume"), SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_PLATFORM_FU15, RT711_SDCA_CTL_FU_CH_GAIN, CH_L), SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_PLATFORM_FU15, RT711_SDCA_CTL_FU_CH_GAIN, CH_R), 8, 3, 0, rt711_sdca_set_gain_get, rt711_sdca_set_gain_put, mic_vol_tlv),
    SOC_ENUM_EXT!(cstr!("GE49 Selected Mode"), rt711_sdca_ge_mode_enum, rt711_sdca_ge_select_get, rt711_sdca_ge_select_put),
];

unsafe extern "C" fn rt711_sdca_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut val: c_uint = 0;
    let mask_sft: c_uint;
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 22 Mux")).is_null() {
        mask_sft = 10;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 23 Mux")).is_null() {
        mask_sft = 13;
    } else {
        return -EINVAL;
    }
    rt711_sdca_index_read(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_MUX_CTL1, &mut val);
    (*ucontrol).value.enumerated.item[0] = (val >> mask_sft) & 0x7;
    0
}

unsafe extern "C" fn rt711_sdca_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_mut_ptr();
    let mut val2: c_uint = 0;
    let mask_sft: c_uint;
    if *item >= (*e).items {
        return -EINVAL;
    }
    if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 22 Mux")).is_null() {
        mask_sft = 10;
    } else if !strstr((*ucontrol).id.name.as_ptr(), cstr!("ADC 23 Mux")).is_null() {
        mask_sft = 13;
    } else {
        return -EINVAL;
    }
    let val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    rt711_sdca_index_read(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_MUX_CTL1, &mut val2);
    val2 = (val2 >> mask_sft) & 0x7;
    let change = if val == val2 { 0 } else { 1 };
    if change != 0 {
        rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_MUX_CTL1, 0x7 << mask_sft, val << mask_sft);
    }
    snd_soc_dapm_mux_update_power(dapm, kcontrol, *item, e, ptr::null_mut());
    change
}

static ADC_MUX_TEXT: [*const c_char; 4] = [
    cstr!("MIC2"),
    cstr!("LINE1"),
    cstr!("LINE2"),
    cstr!("DMIC"),
];

SOC_ENUM_SINGLE_DECL!(rt711_adc22_enum, SND_SOC_NOPM, 0, ADC_MUX_TEXT);
SOC_ENUM_SINGLE_DECL!(rt711_adc23_enum, SND_SOC_NOPM, 0, ADC_MUX_TEXT);

static RT711_SDCA_ADC22_MUX: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!(cstr!("ADC 22 Mux"), rt711_adc22_enum, rt711_sdca_mux_get, rt711_sdca_mux_put);
static RT711_SDCA_ADC23_MUX: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!(cstr!("ADC 23 Mux"), rt711_adc23_enum, rt711_sdca_mux_get, rt711_sdca_mux_put);

unsafe extern "C" fn rt711_sdca_fu05_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let unmute: c_uint = 0x0;
    let mute: c_uint = 0x1;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_MUTE, CH_L), unmute);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_MUTE, CH_R), unmute);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_MUTE, CH_L), mute);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_USER_FU05, RT711_SDCA_CTL_FU_MUTE, CH_R), mute);
    }
    0
}

unsafe extern "C" fn rt711_sdca_fu0f_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    if event == SND_SOC_DAPM_POST_PMU {
        (*rt711).fu0f_dapm_mute = false;
        rt711_sdca_set_fu0f_capture_ctl(rt711);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        (*rt711).fu0f_dapm_mute = true;
        rt711_sdca_set_fu0f_capture_ctl(rt711);
    }
    0
}

unsafe extern "C" fn rt711_sdca_fu1e_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    if event == SND_SOC_DAPM_POST_PMU {
        (*rt711).fu1e_dapm_mute = false;
        rt711_sdca_set_fu1e_capture_ctl(rt711);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        (*rt711).fu1e_dapm_mute = true;
        rt711_sdca_set_fu1e_capture_ctl(rt711);
    }
    0
}

unsafe extern "C" fn rt711_sdca_pde28_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let ps0: c_uint = 0x0;
    let ps3: c_uint = 0x3;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDE28, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps0);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDE28, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps3);
    }
    0
}

unsafe extern "C" fn rt711_sdca_pde29_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let ps0: c_uint = 0x0;
    let ps3: c_uint = 0x3;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDE29, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps0);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDE29, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps3);
    }
    0
}

unsafe extern "C" fn rt711_sdca_pde2a_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let ps0: c_uint = 0x0;
    let ps3: c_uint = 0x3;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_PDE2A, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps0);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_PDE2A, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps3);
    }
    0
}

static mut SEL_MODE: c_uint = 0xffff;

unsafe extern "C" fn rt711_sdca_line1_power_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_read((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_SELECTED_MODE, 0), &mut SEL_MODE);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_LINE1, RT711_SDCA_CTL_VENDOR_DEF, 0), 0x1);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_SELECTED_MODE, 0), 0x7);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_LINE1, RT711_SDCA_CTL_VENDOR_DEF, 0), 0x0);
        if SEL_MODE != 0xffff {
            regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_GE49, RT711_SDCA_CTL_SELECTED_MODE, 0), SEL_MODE);
        }
    }
    0
}

unsafe extern "C" fn rt711_sdca_line2_power_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let ps0: c_uint = 0x0;
    let ps3: c_uint = 0x3;
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDELINE2, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps0);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_LINE2, RT711_SDCA_CTL_VENDOR_DEF, 0), 0x1);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_LINE2, RT711_SDCA_CTL_VENDOR_DEF, 0), 0x0);
        regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_PDELINE2, RT711_SDCA_CTL_REQ_POWER_STATE, 0), ps3);
    }
    0
}

static RT711_SDCA_DAPM_WIDGETS: [snd_soc_dapm_widget_desc; 19] = [
    SND_SOC_DAPM_OUTPUT!(cstr!("HP")),
    SND_SOC_DAPM_INPUT!(cstr!("MIC2")),
    SND_SOC_DAPM_INPUT!(cstr!("DMIC1")),
    SND_SOC_DAPM_INPUT!(cstr!("DMIC2")),
    SND_SOC_DAPM_INPUT!(cstr!("LINE1")),
    SND_SOC_DAPM_INPUT!(cstr!("LINE2")),
    SND_SOC_DAPM_PGA_E!(cstr!("LINE1 Power"), SND_SOC_NOPM, 0, 0, ptr::null(), 0, rt711_sdca_line1_power_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_E!(cstr!("LINE2 Power"), SND_SOC_NOPM, 0, 0, ptr::null(), 0, rt711_sdca_line2_power_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!(cstr!("PDE 28"), SND_SOC_NOPM, 0, 0, rt711_sdca_pde28_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!(cstr!("PDE 29"), SND_SOC_NOPM, 0, 0, rt711_sdca_pde29_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!(cstr!("PDE 2A"), SND_SOC_NOPM, 0, 0, rt711_sdca_pde2a_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_DAC_E!(cstr!("FU 05"), ptr::null(), SND_SOC_NOPM, 0, 0, rt711_sdca_fu05_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_ADC_E!(cstr!("FU 0F"), ptr::null(), SND_SOC_NOPM, 0, 0, rt711_sdca_fu0f_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_ADC_E!(cstr!("FU 1E"), ptr::null(), SND_SOC_NOPM, 0, 0, rt711_sdca_fu1e_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX!(cstr!("ADC 22 Mux"), SND_SOC_NOPM, 0, 0, &RT711_SDCA_ADC22_MUX),
    SND_SOC_DAPM_MUX!(cstr!("ADC 23 Mux"), SND_SOC_NOPM, 0, 0, &RT711_SDCA_ADC23_MUX),
    SND_SOC_DAPM_AIF_IN!(cstr!("DP3RX"), cstr!("DP3 Playback"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(cstr!("DP2TX"), cstr!("DP2 Capture"), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(cstr!("DP4TX"), cstr!("DP4 Capture"), 0, SND_SOC_NOPM, 0, 0),
];

static RT711_SDCA_AUDIO_MAP: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: cstr!("FU 05"), control: ptr::null(), source: cstr!("DP3RX") },
    snd_soc_dapm_route { sink: cstr!("DP2TX"), control: ptr::null(), source: cstr!("FU 0F") },
    snd_soc_dapm_route { sink: cstr!("DP4TX"), control: ptr::null(), source: cstr!("FU 1E") },
    snd_soc_dapm_route { sink: cstr!("LINE1 Power"), control: ptr::null(), source: cstr!("LINE1") },
    snd_soc_dapm_route { sink: cstr!("LINE2 Power"), control: ptr::null(), source: cstr!("LINE2") },
    snd_soc_dapm_route { sink: cstr!("HP"), control: ptr::null(), source: cstr!("PDE 28") },
    snd_soc_dapm_route { sink: cstr!("FU 0F"), control: ptr::null(), source: cstr!("PDE 29") },
    snd_soc_dapm_route { sink: cstr!("FU 1E"), control: ptr::null(), source: cstr!("PDE 2A") },
    snd_soc_dapm_route { sink: cstr!("FU 0F"), control: ptr::null(), source: cstr!("ADC 22 Mux") },
    snd_soc_dapm_route { sink: cstr!("FU 1E"), control: ptr::null(), source: cstr!("ADC 23 Mux") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("DMIC"), source: cstr!("DMIC1") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("LINE1"), source: cstr!("LINE1 Power") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("LINE2"), source: cstr!("LINE2 Power") },
    snd_soc_dapm_route { sink: cstr!("ADC 22 Mux"), control: cstr!("MIC2"), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("DMIC"), source: cstr!("DMIC2") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("LINE1"), source: cstr!("LINE1 Power") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("LINE2"), source: cstr!("LINE2 Power") },
    snd_soc_dapm_route { sink: cstr!("ADC 23 Mux"), control: cstr!("MIC2"), source: cstr!("MIC2") },
    snd_soc_dapm_route { sink: cstr!("HP"), control: ptr::null(), source: cstr!("FU 05") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn rt711_sdca_parse_dt(rt711: *mut rt711_sdca_priv, dev: *mut device) -> c_int {
    device_property_read_u32(dev, cstr!("realtek,jd-src"), &mut (*rt711).jd_src);
    0
}

unsafe extern "C" fn rt711_sdca_probe(component: *mut snd_soc_component) -> c_int {
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    rt711_sdca_parse_dt(rt711, &mut (*(*rt711).slave).dev);
    (*rt711).component = component;
    if !(*rt711).first_hw_init {
        return 0;
    }
    let ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }
    0
}

static SOC_SDCA_DEV_RT711: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt711_sdca_probe),
    controls: RT711_SDCA_SND_CONTROLS.as_ptr(),
    num_controls: ARRAY_SIZE!(RT711_SDCA_SND_CONTROLS),
    dapm_widgets: RT711_SDCA_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(RT711_SDCA_DAPM_WIDGETS),
    dapm_routes: RT711_SDCA_AUDIO_MAP.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(RT711_SDCA_AUDIO_MAP),
    set_jack: Some(rt711_sdca_set_jack_detect),
    endianness: 1,
};

unsafe extern "C" fn rt711_sdca_set_sdw_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int {
    snd_soc_dai_dma_data_set(dai, direction, sdw_stream);
    0
}

unsafe extern "C" fn rt711_sdca_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    snd_soc_dai_set_dma_data(dai, substream, ptr::null_mut());
}

unsafe extern "C" fn rt711_sdca_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let mut stream_config: sdw_stream_config = core::mem::zeroed();
    let mut port_config: sdw_port_config = core::mem::zeroed();
    let sdw_stream: *mut sdw_stream_runtime;
    let sampling_rate: c_uint;

    dev_dbg((*dai).dev, cstr!("%s %s"), cstr!("rt711_sdca_pcm_hw_params"), (*dai).name);
    sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if sdw_stream.is_null() {
        return -EINVAL;
    }
    if (*rt711).slave.is_null() {
        return -EINVAL;
    }

    /* SoundWire specific configuration */
    snd_sdw_params_to_config(substream, params, &mut stream_config, &mut port_config);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        port_config.num = 3;
    } else if (*dai).id == RT711_AIF1 {
        port_config.num = 2;
    } else if (*dai).id == RT711_AIF2 {
        port_config.num = 4;
    } else {
        return -EINVAL;
    }

    let retval = sdw_stream_add_slave((*rt711).slave, &mut stream_config, &mut port_config, 1, sdw_stream);
    if retval != 0 {
        dev_err((*dai).dev, cstr!("%s: Unable to configure port\n"), cstr!("rt711_sdca_pcm_hw_params"));
        return retval;
    }

    if params_channels(params) > 16 {
        dev_err((*component).dev, cstr!("%s: Unsupported channels %d\n"), cstr!("rt711_sdca_pcm_hw_params"), params_channels(params));
        return -EINVAL;
    }

    /* sampling rate configuration */
    match params_rate(params) {
        44100 => sampling_rate = RT711_SDCA_RATE_44100HZ,
        48000 => sampling_rate = RT711_SDCA_RATE_48000HZ,
        96000 => sampling_rate = RT711_SDCA_RATE_96000HZ,
        192000 => sampling_rate = RT711_SDCA_RATE_192000HZ,
        _ => {
            dev_err((*component).dev, cstr!("%s: Rate %d is not supported\n"), cstr!("rt711_sdca_pcm_hw_params"), params_rate(params));
            return -EINVAL;
        }
    }

    /* set sampling frequency */
    regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_CS01, RT711_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_CS11, RT711_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_MIC_ARRAY, RT711_SDCA_ENT_CS1F, RT711_SDCA_CTL_SAMPLE_FREQ_INDEX, 0), sampling_rate);
    0
}

unsafe extern "C" fn rt711_sdca_pcm_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt711 = snd_soc_component_get_drvdata(component) as *mut rt711_sdca_priv;
    let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
    if (*rt711).slave.is_null() {
        return -EINVAL;
    }
    sdw_stream_remove_slave((*rt711).slave, sdw_stream);
    0
}

const RT711_STEREO_RATES_EXPR_COMMENT: &str = "SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000";
unsafe fn RT711_STEREO_RATES() -> c_uint {
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000
}
const RT711_FORMATS_EXPR_COMMENT: &str = "SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE";
unsafe fn RT711_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE
}

static RT711_SDCA_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt711_sdca_pcm_hw_params),
    hw_free: Some(rt711_sdca_pcm_hw_free),
    set_stream: Some(rt711_sdca_set_sdw_stream),
    shutdown: Some(rt711_sdca_shutdown),
};

static mut RT711_SDCA_DAI: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: cstr!("rt711-sdca-aif1"),
        id: unsafe { RT711_AIF1 },
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("DP3 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { RT711_STEREO_RATES() },
            formats: unsafe { RT711_FORMATS() },
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("DP2 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { RT711_STEREO_RATES() },
            formats: unsafe { RT711_FORMATS() },
        },
        ops: &RT711_SDCA_OPS,
    },
    snd_soc_dai_driver {
        name: cstr!("rt711-sdca-aif2"),
        id: unsafe { RT711_AIF2 },
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("DP4 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { RT711_STEREO_RATES() },
            formats: unsafe { RT711_FORMATS() },
        },
        ops: &RT711_SDCA_OPS,
    },
];

#[no_mangle]
pub unsafe extern "C" fn rt711_sdca_init(dev: *mut device, regmap: *mut regmap, mbq_regmap: *mut regmap, slave: *mut sdw_slave) -> c_int {
    let rt711 = devm_kzalloc(dev, core::mem::size_of::<rt711_sdca_priv>(), GFP_KERNEL) as *mut rt711_sdca_priv;
    if rt711.is_null() {
        return -ENOMEM;
    }
    dev_set_drvdata(dev, rt711 as *mut c_void);
    (*rt711).slave = slave;
    (*rt711).regmap = regmap;
    (*rt711).mbq_regmap = mbq_regmap;

    regcache_cache_only((*rt711).regmap, true);
    regcache_cache_only((*rt711).mbq_regmap, true);
    mutex_init(&mut (*rt711).calibrate_mutex);
    mutex_init(&mut (*rt711).disable_irq_lock);
    INIT_DELAYED_WORK(&mut (*rt711).jack_detect_work, rt711_sdca_jack_detect_handler);
    INIT_DELAYED_WORK(&mut (*rt711).jack_btn_check_work, rt711_sdca_btn_check_handler);

    /*
     * Mark hw_init to false
     * HW init will be performed when device reports present
     */
    (*rt711).hw_init = false;
    (*rt711).first_hw_init = false;
    (*rt711).fu0f_dapm_mute = true;
    (*rt711).fu1e_dapm_mute = true;
    (*rt711).fu0f_mixer_l_mute = true;
    (*rt711).fu0f_mixer_r_mute = true;
    (*rt711).fu1e_mixer_l_mute = true;
    (*rt711).fu1e_mixer_r_mute = true;

    /* JD source uses JD2 in default */
    (*rt711).jd_src = RT711_JD2;

    let ret = devm_snd_soc_register_component(dev, &SOC_SDCA_DEV_RT711, RT711_SDCA_DAI.as_mut_ptr(), ARRAY_SIZE!(RT711_SDCA_DAI) as c_int);
    if ret < 0 {
        return ret;
    }

    /* set autosuspend parameters */
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    /* make sure the device does not suspend immediately */
    pm_runtime_mark_last_busy(dev);
    pm_runtime_enable(dev);

    /* important note: the device is NOT tagged as 'active' and will remain
     * 'suspended' until the hardware is enumerated/initialized. This is required
     * to make sure the ASoC framework use of pm_runtime_get_sync() does not silently
     * fail with -EACCESS because of race conditions between card creation and enumeration
     */
    dev_dbg(dev, cstr!("%s\n"), cstr!("rt711_sdca_init"));
    0
}

unsafe extern "C" fn rt711_sdca_vd0_io_init(rt711: *mut rt711_sdca_priv) {
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_GPIO_TEST_MODE_CTL2, 0x0e00);
    rt711_sdca_index_write(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_GPIO_CTL, 0x0008);
    regmap_write((*rt711).regmap, 0x2f5a, 0x01);
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_ADC27_VOL_SET, 0x8728);
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL3, 0xa472);
    regmap_write((*rt711).regmap, 0x2f50, 0x02);
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_ANALOG_CTL, RT711_MISC_POWER_CTL4, 0x6000, 0x6000);
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL3, 0x000c, 0x000c);
    rt711_sdca_index_write(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_CONFIG_CTL, 0x0000);
    rt711_sdca_index_write(rt711, RT711_VENDOR_VAD, RT711_VAD_SRAM_CTL1, 0x0050);
}

unsafe extern "C" fn rt711_sdca_vd1_io_init(rt711: *mut rt711_sdca_priv) {
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_UNSOLICITED_CTL, 0x0300, 0x0000);
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_COMBO_JACK_AUTO_CTL3, 0xa43e);
    regmap_write((*rt711).regmap, 0x2f5a, 0x05);
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_JD_CTRL6, 0x0500);
    rt711_sdca_index_write(rt711, RT711_VENDOR_REG, RT711_DMIC_CTL1, 0x6173);
    rt711_sdca_index_write(rt711, RT711_VENDOR_HDA_CTL, RT711_HDA_LEGACY_CONFIG_CTL, 0x0000);
    rt711_sdca_index_write(rt711, RT711_VENDOR_VAD, RT711_VAD_SRAM_CTL1, 0x0050);
}

#[no_mangle]
pub unsafe extern "C" fn rt711_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int {
    let rt711 = dev_get_drvdata(dev) as *mut rt711_sdca_priv;
    let mut ret: c_int;
    let mut val: c_uint = 0;

    (*rt711).disable_irq = false;
    if (*rt711).hw_init {
        return 0;
    }

    regcache_cache_only((*rt711).regmap, false);
    regcache_cache_only((*rt711).mbq_regmap, false);
    if (*rt711).first_hw_init {
        regcache_cache_bypass((*rt711).regmap, true);
        regcache_cache_bypass((*rt711).mbq_regmap, true);
    } else {
        /*
         * PM runtime status is marked as 'active' only when a Slave reports as Attached
         */
        /* update count of parent 'active' children */
        pm_runtime_set_active(&mut (*slave).dev);
    }

    pm_runtime_get_noresume(&mut (*slave).dev);
    rt711_sdca_reset(rt711);
    rt711_sdca_index_read(rt711, RT711_VENDOR_REG, RT711_JD_PRODUCT_NUM, &mut val);
    (*rt711).hw_ver = val & 0xf;
    if (*rt711).hw_ver == RT711_VER_VD0 {
        rt711_sdca_vd0_io_init(rt711);
    } else {
        rt711_sdca_vd1_io_init(rt711);
    }

    /* DP4 mux select from 08_filter_Out_pri */
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_REG, RT711_FILTER_SRC_SEL, 0x1800, 0x0800);
    /* ge_exclusive_inbox_en disable */
    rt711_sdca_index_update_bits(rt711, RT711_VENDOR_HDA_CTL, RT711_PUSH_BTN_INT_CTL0, 0x20, 0x00);
    /* calibration */
    ret = rt711_sdca_calibration(rt711);
    if ret < 0 {
        dev_err(dev, cstr!("%s, calibration failed!\n"), cstr!("rt711_sdca_io_init"));
    }

    /* HP output enable */
    regmap_write((*rt711).regmap, SDW_SDCA_CTL(FUNC_NUM_JACK_CODEC, RT711_SDCA_ENT_OT1, RT711_SDCA_CTL_VENDOR_DEF, 0), 0x4);

    /*
     * if set_jack callback occurred early than io_init,
     * we set up the jack detection function now
     */
    if !(*rt711).hs_jack.is_null() {
        rt711_sdca_jack_init(rt711);
    }

    if (*rt711).first_hw_init {
        regcache_cache_bypass((*rt711).regmap, false);
        regcache_mark_dirty((*rt711).regmap);
        regcache_cache_bypass((*rt711).mbq_regmap, false);
        regcache_mark_dirty((*rt711).mbq_regmap);
    } else {
        (*rt711).first_hw_init = true;
    }

    /* Mark Slave initialization complete */
    (*rt711).hw_init = true;
    pm_runtime_put_autosuspend(&mut (*slave).dev);
    dev_dbg(&mut (*slave).dev, cstr!("%s hw_init complete\n"), cstr!("rt711_sdca_io_init"));
    0
}

// MODULE_DESCRIPTION("ASoC RT711 SDCA SDW driver");
// MODULE_AUTHOR("Shuming Fan <shumingf@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
