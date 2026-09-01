// SPDX-License-Identifier: GPL-2.0
//
// cs35l45.c - CS35L45 ALSA SoC audio driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>

// Translated from C. Linux/ALSA/regmap symbols and constructor macros are
// supplied by the surrounding driver bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type irqreturn_t = c_uint;

extern "C" {
    static mut cs35l45_pm_ops: dev_pm_ops;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_uint, count: usize) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, count: usize) -> c_int;
    fn regmap_irq_get_virq(data: *mut regmap_irq_chip_data, irq: c_int) -> c_int;
    fn devm_regmap_add_irq_chip(dev: *mut device, map: *mut regmap, irq: c_int,
                                flags: c_ulong, irq_base: c_int,
                                chip: *const regmap_irq_chip,
                                data: *mut *mut regmap_irq_chip_data) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;

    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cs35l45_private;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_kcontrol(component: *mut snd_soc_component,
                                      ctl_name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_get_ioff(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_uint;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;

    fn wm_adsp_early_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol,
                           event: c_int) -> c_int;
    fn wm_adsp_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol,
                     event: c_int) -> c_int;
    fn wm_adsp_hibernate(dsp: *mut wm_adsp, hibernate: bool_);
    fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int;
    fn wm_adsp2_component_remove(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_remove(dsp: *mut wm_adsp);
    fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_tdm_params_to_bclk(params: *mut snd_pcm_hw_params, width: c_uint,
                                  slots: c_int, slot_multiple: c_uint) -> c_int;
    fn cs35l45_get_clk_freq_id(freq: c_uint) -> c_int;
    fn cs35l45_apply_patch(cs35l45: *mut cs35l45_private) -> c_int;

    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_property_read_u32(node: *mut device_node, propname: *const c_char,
                            out_value: *mut c_uint) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn device_property_read_u32(dev: *mut device, propname: *const c_char,
                                val: *mut c_uint) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char,
                               flags: c_int) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn enable_irq(irq: c_int);
    fn disable_irq(irq: c_int);
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int,
                                 handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
                                 thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
                                 irqflags: c_ulong, devname: *const c_char,
                                 dev_id: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device,
                                       component_driver: *const snd_soc_component_driver,
                                       dai_drv: *mut snd_soc_dai_driver,
                                       num_dai: c_int) -> c_int;
}

#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_volatile { pub access: c_uint }
#[repr(C)] pub struct snd_kcontrol { pub id: snd_ctl_elem_id, pub vd: *mut snd_kcontrol_volatile }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct regmap_irq_chip_data { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }

#[repr(C)] pub struct cs_dsp {
    pub running: bool_,
    pub booted: bool_,
    pub num: c_int,
    pub type_: c_int,
    pub rev: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: c_uint,
    pub base_sysinfo: c_uint,
    pub mem: *const cs_dsp_region,
    pub num_mems: c_int,
    pub lock_regions: c_uint,
}
#[repr(C)] pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub preloaded: bool_,
    pub part: *const c_char,
    pub fw: c_int,
    pub toggle_preload: bool_,
}
#[repr(C)] pub struct cs35l45_private {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dsp: wm_adsp,
    pub amplifier_mode: i64,
    pub slot_width: c_uint,
    pub slot_count: c_int,
    pub sysclk_set: bool_,
    pub bus_type: c_int,
    pub i2c_addr: c_uint,
    pub irq_invert: c_uint,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
    pub vdd_batt: *mut regulator,
    pub vdd_a: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
}

#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct cs_dsp_region { pub type_: c_int, pub base: c_uint }
#[repr(C)] pub struct cs35l45_irq {
    pub irq: c_int,
    pub name: *const c_char,
    pub handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
}
#[repr(C)] pub struct regmap_irq { _private: [u8; 0] }
#[repr(C)] pub struct regmap_irq_chip {
    pub name: *const c_char,
    pub main_status: c_uint,
    pub status_base: c_uint,
    pub mask_base: c_uint,
    pub ack_base: c_uint,
    pub num_regs: c_int,
    pub irqs: *const regmap_irq,
    pub num_irqs: c_int,
    pub runtime_pm: bool_,
}
#[repr(C)] pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
}
#[repr(C)] pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}
#[repr(C)] pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub symmetric_rate: bool_,
    pub symmetric_sample_bits: bool_,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub name: *const c_char,
    pub endianness: c_uint,
}

const EPERM: c_int = 1;
const ENOMSG: c_int = 42;
const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const IRQ_HANDLED: irqreturn_t = 1;

unsafe fn IRQ_RETVAL(ret: c_int) -> irqreturn_t {
    if ret != 0 { 1 } else { 0 }
}

unsafe extern "C" fn cs35l45_check_cspl_mbox_sts(
    cmd: cs35l45_cspl_mboxcmd,
    sts: cs35l45_cspl_mboxstate,
) -> bool_ {
    match cmd {
        CSPL_MBOX_CMD_NONE | CSPL_MBOX_CMD_UNKNOWN_CMD => true,
        CSPL_MBOX_CMD_PAUSE | CSPL_MBOX_CMD_OUT_OF_HIBERNATE => sts == CSPL_MBOX_STS_PAUSED,
        CSPL_MBOX_CMD_RESUME => sts == CSPL_MBOX_STS_RUNNING,
        CSPL_MBOX_CMD_REINIT => sts == CSPL_MBOX_STS_RUNNING,
        CSPL_MBOX_CMD_STOP_PRE_REINIT => sts == CSPL_MBOX_STS_RDY_FOR_REINIT,
        CSPL_MBOX_CMD_HIBERNATE => sts == CSPL_MBOX_STS_HIBERNATE,
        _ => false,
    }
}

unsafe extern "C" fn cs35l45_set_cspl_mbox_cmd(
    cs35l45: *mut cs35l45_private,
    regmap_: *mut regmap,
    cmd: cs35l45_cspl_mboxcmd,
) -> c_int {
    let mut sts: c_uint = 0;
    let mut ret: c_int;

    if !(*cs35l45).dsp.cs_dsp.running {
        dev_err((*cs35l45).dev, c"DSP not running\n".as_ptr());
        return -EPERM;
    }

    // Set mailbox cmd
    ret = regmap_write(regmap_, CS35L45_DSP_VIRT1_MBOX_1, cmd as c_uint);
    if ret < 0 {
        if cmd != CSPL_MBOX_CMD_OUT_OF_HIBERNATE {
            dev_err((*cs35l45).dev, c"Failed to write MBOX: %d\n".as_ptr(), ret);
        }
        return ret;
    }

    // Read mailbox status and verify it is appropriate for the given cmd
    for i in 0..5u32 {
        usleep_range(1000, 1100);
        ret = regmap_read(regmap_, CS35L45_DSP_MBOX_2, &mut sts);
        if ret < 0 {
            dev_err((*cs35l45).dev, c"Failed to read MBOX STS: %d\n".as_ptr(), ret);
            continue;
        }

        if !cs35l45_check_cspl_mbox_sts(cmd, sts as cs35l45_cspl_mboxstate) {
            dev_dbg((*cs35l45).dev, c"[%u] cmd %u returned invalid sts %u".as_ptr(), i, cmd as c_uint, sts);
        } else {
            return 0;
        }
    }

    if cmd != CSPL_MBOX_CMD_OUT_OF_HIBERNATE {
        dev_err((*cs35l45).dev, c"Failed to set mailbox cmd %u (status %u)\n".as_ptr(), cmd as c_uint, sts);
    }

    -ENOMSG
}

unsafe extern "C" fn cs35l45_global_en_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l45 = snd_soc_component_get_drvdata(component);

    dev_dbg((*cs35l45).dev, c"%s event : %x\n".as_ptr(), c"cs35l45_global_en_ev".as_ptr(), event);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_write((*cs35l45).regmap, CS35L45_GLOBAL_ENABLES, CS35L45_GLOBAL_EN_MASK);
            usleep_range(CS35L45_POST_GLOBAL_EN_US, CS35L45_POST_GLOBAL_EN_US + 100);
        }
        SND_SOC_DAPM_PRE_PMD => {
            usleep_range(CS35L45_PRE_GLOBAL_DIS_US, CS35L45_PRE_GLOBAL_DIS_US + 100);
            regmap_write((*cs35l45).regmap, CS35L45_GLOBAL_ENABLES, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn cs35l45_dsp_preload_ev(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l45 = snd_soc_component_get_drvdata(component);
    let mut ret: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*cs35l45).dsp.cs_dsp.booted {
                return 0;
            }
            wm_adsp_early_event(w, kcontrol, event)
        }
        SND_SOC_DAPM_POST_PMU => {
            if (*cs35l45).dsp.cs_dsp.running {
                return 0;
            }
            regmap_set_bits((*cs35l45).regmap, CS35L45_PWRMGT_CTL, CS35L45_MEM_RDY_MASK);
            wm_adsp_event(w, kcontrol, event)
        }
        SND_SOC_DAPM_PRE_PMD => {
            if (*cs35l45).dsp.preloaded {
                return 0;
            }
            if (*cs35l45).dsp.cs_dsp.running {
                ret = wm_adsp_event(w, kcontrol, event);
                if ret != 0 {
                    return ret;
                }
            }
            wm_adsp_early_event(w, kcontrol, event)
        }
        _ => 0,
    }
}

unsafe extern "C" fn cs35l45_dsp_audio_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs35l45 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_POST_PMU => cs35l45_set_cspl_mbox_cmd(cs35l45, (*cs35l45).regmap, CSPL_MBOX_CMD_RESUME),
        SND_SOC_DAPM_PRE_PMD => cs35l45_set_cspl_mbox_cmd(cs35l45, (*cs35l45).regmap, CSPL_MBOX_CMD_PAUSE),
        _ => 0,
    }
}

unsafe extern "C" fn cs35l45_activate_ctl(
    component: *mut snd_soc_component,
    ctl_name: *const c_char,
    active: bool_,
) -> c_int {
    let card = (*(*component).card).snd_card;
    let kcontrol = snd_soc_component_get_kcontrol(component, ctl_name);
    let mut index_offset: c_uint;

    if kcontrol.is_null() {
        dev_err((*component).dev, c"Can't find kcontrol %s\n".as_ptr(), ctl_name);
        return -EINVAL;
    }

    index_offset = snd_ctl_get_ioff(kcontrol, &mut (*kcontrol).id);
    let vd = (*kcontrol).vd.add(index_offset as usize);
    if active {
        (*vd).access |= SNDRV_CTL_ELEM_ACCESS_WRITE;
    } else {
        (*vd).access &= !SNDRV_CTL_ELEM_ACCESS_WRITE;
    }

    snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*kcontrol).id);
    0
}

unsafe extern "C" fn cs35l45_amplifier_mode_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs35l45 = snd_soc_component_get_drvdata(component);
    (*ucontrol).value.integer.value[0] = (*cs35l45).amplifier_mode;
    0
}

unsafe extern "C" fn cs35l45_amplifier_mode_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs35l45 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    let mut amp_state: c_uint = 0;
    let mut ret: c_int;

    if (*ucontrol).value.integer.value[0] == (*cs35l45).amplifier_mode
        || (*ucontrol).value.integer.value[0] > AMP_MODE_RCV as i64
    {
        return 0;
    }

    snd_soc_dapm_mutex_lock(dapm);
    ret = regmap_read((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, &mut amp_state);
    if ret < 0 {
        dev_err((*cs35l45).dev, c"Failed to read AMP state: %d\n".as_ptr(), ret);
        snd_soc_dapm_mutex_unlock(dapm);
        return ret;
    }

    regmap_clear_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_AMP_EN_MASK);
    snd_soc_dapm_disable_pin_unlocked(dapm, c"SPK".as_ptr());
    snd_soc_dapm_sync_unlocked(dapm);

    if (*ucontrol).value.integer.value[0] == AMP_MODE_SPK as i64 {
        regmap_clear_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_RCV_EN_MASK);
        regmap_update_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_BST_EN_MASK,
                           CS35L45_BST_ENABLE << CS35L45_BST_EN_SHIFT);
        regmap_update_bits((*cs35l45).regmap, CS35L45_HVLV_CONFIG, CS35L45_HVLV_MODE_MASK,
                           CS35L45_HVLV_OPERATION << CS35L45_HVLV_MODE_SHIFT);

        ret = cs35l45_activate_ctl(component, c"Analog PCM Volume".as_ptr(), true);
        if ret < 0 {
            dev_err((*cs35l45).dev, c"Unable to deactivate ctl (%d)\n".as_ptr(), ret);
        }
    } else {
        /* AMP_MODE_RCV */
        regmap_set_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_RCV_EN_MASK);
        regmap_update_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_BST_EN_MASK,
                           CS35L45_BST_DISABLE_FET_OFF << CS35L45_BST_EN_SHIFT);
        regmap_update_bits((*cs35l45).regmap, CS35L45_HVLV_CONFIG, CS35L45_HVLV_MODE_MASK,
                           CS35L45_FORCE_LV_OPERATION << CS35L45_HVLV_MODE_SHIFT);
        regmap_clear_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES2, CS35L45_AMP_DRE_EN_MASK);
        regmap_update_bits((*cs35l45).regmap, CS35L45_AMP_GAIN, CS35L45_AMP_GAIN_PCM_MASK,
                           CS35L45_AMP_GAIN_PCM_13DBV << CS35L45_AMP_GAIN_PCM_SHIFT);

        ret = cs35l45_activate_ctl(component, c"Analog PCM Volume".as_ptr(), false);
        if ret < 0 {
            dev_err((*cs35l45).dev, c"Unable to deactivate ctl (%d)\n".as_ptr(), ret);
        }
    }

    if (amp_state & CS35L45_AMP_EN_MASK) != 0 {
        regmap_set_bits((*cs35l45).regmap, CS35L45_BLOCK_ENABLES, CS35L45_AMP_EN_MASK);
    }

    snd_soc_dapm_enable_pin_unlocked(dapm, c"SPK".as_ptr());
    snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm);

    (*cs35l45).amplifier_mode = (*ucontrol).value.integer.value[0];
    1
}

static cs35l45_asp_tx_txt: [&[u8]; 12] = [
    b"Zero\0", b"ASP_RX1\0", b"ASP_RX2\0", b"VMON\0", b"IMON\0", b"ERR_VOL\0",
    b"VDD_BATTMON\0", b"VDD_BSTMON\0", b"DSP_TX1\0", b"DSP_TX2\0",
    b"Interpolator\0", b"IL_TARGET\0",
];

static cs35l45_asp_tx_val: [c_uint; 12] = [
    CS35L45_PCM_SRC_ZERO, CS35L45_PCM_SRC_ASP_RX1, CS35L45_PCM_SRC_ASP_RX2,
    CS35L45_PCM_SRC_VMON, CS35L45_PCM_SRC_IMON, CS35L45_PCM_SRC_ERR_VOL,
    CS35L45_PCM_SRC_VDD_BATTMON, CS35L45_PCM_SRC_VDD_BSTMON,
    CS35L45_PCM_SRC_DSP_TX1, CS35L45_PCM_SRC_DSP_TX2,
    CS35L45_PCM_SRC_INTERPOLATOR, CS35L45_PCM_SRC_IL_TARGET,
];

// static const struct soc_enum cs35l45_asp_tx_enums[] =
static cs35l45_asp_tx_enums: [soc_enum; 5] = [
    SOC_VALUE_ENUM_SINGLE!(CS35L45_ASPTX1_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_asp_tx_txt.len(), cs35l45_asp_tx_txt, cs35l45_asp_tx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_ASPTX2_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_asp_tx_txt.len(), cs35l45_asp_tx_txt, cs35l45_asp_tx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_ASPTX3_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_asp_tx_txt.len(), cs35l45_asp_tx_txt, cs35l45_asp_tx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_ASPTX4_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_asp_tx_txt.len(), cs35l45_asp_tx_txt, cs35l45_asp_tx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_ASPTX5_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_asp_tx_txt.len(), cs35l45_asp_tx_txt, cs35l45_asp_tx_val),
];

static cs35l45_dsp_rx_txt: [&[u8]; 10] = [
    b"Zero\0", b"ASP_RX1\0", b"ASP_RX2\0", b"VMON\0", b"IMON\0", b"ERR_VOL\0",
    b"CLASSH_TGT\0", b"VDD_BATTMON\0", b"VDD_BSTMON\0", b"TEMPMON\0",
];

static cs35l45_dsp_rx_val: [c_uint; 10] = [
    CS35L45_PCM_SRC_ZERO, CS35L45_PCM_SRC_ASP_RX1, CS35L45_PCM_SRC_ASP_RX2,
    CS35L45_PCM_SRC_VMON, CS35L45_PCM_SRC_IMON, CS35L45_PCM_SRC_ERR_VOL,
    CS35L45_PCM_SRC_CLASSH_TGT, CS35L45_PCM_SRC_VDD_BATTMON,
    CS35L45_PCM_SRC_VDD_BSTMON, CS35L45_PCM_SRC_TEMPMON,
];

static cs35l45_dsp_rx_enums: [soc_enum; 8] = [
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX1_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX2_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX3_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX4_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX5_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX6_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX7_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DSP1RX8_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dsp_rx_txt.len(), cs35l45_dsp_rx_txt, cs35l45_dsp_rx_val),
];

static cs35l45_dac_txt: [&[u8]; 5] = [b"Zero\0", b"ASP_RX1\0", b"ASP_RX2\0", b"DSP_TX1\0", b"DSP_TX2\0"];
static cs35l45_dac_val: [c_uint; 5] = [
    CS35L45_PCM_SRC_ZERO, CS35L45_PCM_SRC_ASP_RX1, CS35L45_PCM_SRC_ASP_RX2,
    CS35L45_PCM_SRC_DSP_TX1, CS35L45_PCM_SRC_DSP_TX2,
];

static cs35l45_dacpcm_enums: [soc_enum; 1] = [
    SOC_VALUE_ENUM_SINGLE!(CS35L45_DACPCM1_INPUT, 0, CS35L45_PCM_SRC_MASK, cs35l45_dac_txt.len(), cs35l45_dac_txt, cs35l45_dac_val),
];

static cs35l45_asp_muxes: [snd_kcontrol_new; 5] = [
    SOC_DAPM_ENUM!(c"ASP_TX1 Source", cs35l45_asp_tx_enums[0]),
    SOC_DAPM_ENUM!(c"ASP_TX2 Source", cs35l45_asp_tx_enums[1]),
    SOC_DAPM_ENUM!(c"ASP_TX3 Source", cs35l45_asp_tx_enums[2]),
    SOC_DAPM_ENUM!(c"ASP_TX4 Source", cs35l45_asp_tx_enums[3]),
    SOC_DAPM_ENUM!(c"ASP_TX5 Source", cs35l45_asp_tx_enums[4]),
];

static cs35l45_dsp_muxes: [snd_kcontrol_new; 8] = [
    SOC_DAPM_ENUM!(c"DSP_RX1 Source", cs35l45_dsp_rx_enums[0]),
    SOC_DAPM_ENUM!(c"DSP_RX2 Source", cs35l45_dsp_rx_enums[1]),
    SOC_DAPM_ENUM!(c"DSP_RX3 Source", cs35l45_dsp_rx_enums[2]),
    SOC_DAPM_ENUM!(c"DSP_RX4 Source", cs35l45_dsp_rx_enums[3]),
    SOC_DAPM_ENUM!(c"DSP_RX5 Source", cs35l45_dsp_rx_enums[4]),
    SOC_DAPM_ENUM!(c"DSP_RX6 Source", cs35l45_dsp_rx_enums[5]),
    SOC_DAPM_ENUM!(c"DSP_RX7 Source", cs35l45_dsp_rx_enums[6]),
    SOC_DAPM_ENUM!(c"DSP_RX8 Source", cs35l45_dsp_rx_enums[7]),
];

static cs35l45_dac_muxes: [snd_kcontrol_new; 1] = [
    SOC_DAPM_ENUM!(c"DACPCM Source", cs35l45_dacpcm_enums[0]),
];
static amp_en_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!(c"Switch", SND_SOC_NOPM, 0, 1, 0);

static cs35l45_dapm_widgets: [snd_soc_dapm_widget_item; 45] = [
    SND_SOC_DAPM_SPK!(c"DSP1 Preload", core::ptr::null()),
    SND_SOC_DAPM_SUPPLY_S!(c"DSP1 Preloader", 100, SND_SOC_NOPM, 0, 0, cs35l45_dsp_preload_ev, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_OUT_DRV_E!(c"DSP1", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0, cs35l45_dsp_audio_ev, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!(c"GLOBAL_EN", SND_SOC_NOPM, 0, 0, cs35l45_global_en_ev, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY!(c"ASP_EN", CS35L45_BLOCK_ENABLES2, CS35L45_ASP_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SIGGEN!(c"VMON_SRC"), SND_SOC_DAPM_SIGGEN!(c"IMON_SRC"),
    SND_SOC_DAPM_SIGGEN!(c"TEMPMON_SRC"), SND_SOC_DAPM_SIGGEN!(c"VDD_BATTMON_SRC"),
    SND_SOC_DAPM_SIGGEN!(c"VDD_BSTMON_SRC"), SND_SOC_DAPM_SIGGEN!(c"ERR_VOL"),
    SND_SOC_DAPM_SIGGEN!(c"AMP_INTP"), SND_SOC_DAPM_SIGGEN!(c"IL_TARGET"),
    SND_SOC_DAPM_SUPPLY!(c"VMON_EN", CS35L45_BLOCK_ENABLES, CS35L45_VMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"IMON_EN", CS35L45_BLOCK_ENABLES, CS35L45_IMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"TEMPMON_EN", CS35L45_BLOCK_ENABLES, CS35L45_TEMPMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"VDD_BATTMON_EN", CS35L45_BLOCK_ENABLES, CS35L45_VDD_BATTMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(c"VDD_BSTMON_EN", CS35L45_BLOCK_ENABLES, CS35L45_VDD_BSTMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC!(c"VMON", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!(c"IMON", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!(c"TEMPMON", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!(c"VDD_BATTMON", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!(c"VDD_BSTMON", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c"ASP_RX1", core::ptr::null(), 0, CS35L45_ASP_ENABLES1, CS35L45_ASP_RX1_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_IN!(c"ASP_RX2", core::ptr::null(), 1, CS35L45_ASP_ENABLES1, CS35L45_ASP_RX2_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_OUT!(c"ASP_TX1", core::ptr::null(), 0, CS35L45_ASP_ENABLES1, CS35L45_ASP_TX1_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_OUT!(c"ASP_TX2", core::ptr::null(), 1, CS35L45_ASP_ENABLES1, CS35L45_ASP_TX2_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_OUT!(c"ASP_TX3", core::ptr::null(), 2, CS35L45_ASP_ENABLES1, CS35L45_ASP_TX3_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_OUT!(c"ASP_TX4", core::ptr::null(), 3, CS35L45_ASP_ENABLES1, CS35L45_ASP_TX4_EN_SHIFT, 0),
    SND_SOC_DAPM_AIF_OUT!(c"ASP_TX5", core::ptr::null(), 4, CS35L45_ASP_ENABLES1, CS35L45_ASP_TX5_EN_SHIFT, 0),
    SND_SOC_DAPM_MUX!(c"ASP_TX1 Source", SND_SOC_NOPM, 0, 0, &cs35l45_asp_muxes[0]),
    SND_SOC_DAPM_MUX!(c"ASP_TX2 Source", SND_SOC_NOPM, 0, 0, &cs35l45_asp_muxes[1]),
    SND_SOC_DAPM_MUX!(c"ASP_TX3 Source", SND_SOC_NOPM, 0, 0, &cs35l45_asp_muxes[2]),
    SND_SOC_DAPM_MUX!(c"ASP_TX4 Source", SND_SOC_NOPM, 0, 0, &cs35l45_asp_muxes[3]),
    SND_SOC_DAPM_MUX!(c"ASP_TX5 Source", SND_SOC_NOPM, 0, 0, &cs35l45_asp_muxes[4]),
    SND_SOC_DAPM_MUX!(c"DSP_RX1 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[0]),
    SND_SOC_DAPM_MUX!(c"DSP_RX2 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[1]),
    SND_SOC_DAPM_MUX!(c"DSP_RX3 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[2]),
    SND_SOC_DAPM_MUX!(c"DSP_RX4 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[3]),
    SND_SOC_DAPM_MUX!(c"DSP_RX5 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[4]),
    SND_SOC_DAPM_MUX!(c"DSP_RX6 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[5]),
    SND_SOC_DAPM_MUX!(c"DSP_RX7 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[6]),
    SND_SOC_DAPM_MUX!(c"DSP_RX8 Source", SND_SOC_NOPM, 0, 0, &cs35l45_dsp_muxes[7]),
    SND_SOC_DAPM_MUX!(c"DACPCM Source", SND_SOC_NOPM, 0, 0, &cs35l45_dac_muxes[0]),
    SND_SOC_DAPM_SWITCH!(c"AMP Enable", SND_SOC_NOPM, 0, 0, &amp_en_ctl),
    SND_SOC_DAPM_OUT_DRV!(c"AMP", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!(c"SPK"),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}
macro_rules! CS35L45_ASP_MUX_ROUTE {
    ($name:literal) => {
        route!(concat!($name, " Source"), "ASP_RX1", "ASP_RX1"),
        route!(concat!($name, " Source"), "ASP_RX2", "ASP_RX2"),
        route!(concat!($name, " Source"), "DSP_TX1", "DSP1"),
        route!(concat!($name, " Source"), "DSP_TX2", "DSP1"),
        route!(concat!($name, " Source"), "VMON", "VMON"),
        route!(concat!($name, " Source"), "IMON", "IMON"),
        route!(concat!($name, " Source"), "ERR_VOL", "ERR_VOL"),
        route!(concat!($name, " Source"), "VDD_BATTMON", "VDD_BATTMON"),
        route!(concat!($name, " Source"), "VDD_BSTMON", "VDD_BSTMON"),
        route!(concat!($name, " Source"), "Interpolator", "AMP_INTP"),
        route!(concat!($name, " Source"), "IL_TARGET", "IL_TARGET")
    };
}
macro_rules! CS35L45_DSP_MUX_ROUTE {
    ($name:literal) => {
        route!(concat!($name, " Source"), "ASP_RX1", "ASP_RX1"),
        route!(concat!($name, " Source"), "ASP_RX2", "ASP_RX2")
    };
}
macro_rules! CS35L45_DAC_MUX_ROUTE {
    ($name:literal) => {
        route!(concat!($name, " Source"), "ASP_RX1", "ASP_RX1"),
        route!(concat!($name, " Source"), "ASP_RX2", "ASP_RX2"),
        route!(concat!($name, " Source"), "DSP_TX1", "DSP1"),
        route!(concat!($name, " Source"), "DSP_TX2", "DSP1")
    };
}

static cs35l45_dapm_routes: [snd_soc_dapm_route; 95] = [
    /* Feedback */
    route!("VMON", NULL, "VMON_SRC"), route!("IMON", NULL, "IMON_SRC"),
    route!("TEMPMON", NULL, "TEMPMON_SRC"), route!("VDD_BATTMON", NULL, "VDD_BATTMON_SRC"),
    route!("VDD_BSTMON", NULL, "VDD_BSTMON_SRC"), route!("VMON", NULL, "VMON_EN"),
    route!("IMON", NULL, "IMON_EN"), route!("TEMPMON", NULL, "TEMPMON_EN"),
    route!("VDD_BATTMON", NULL, "VDD_BATTMON_EN"), route!("VDD_BSTMON", NULL, "VDD_BSTMON_EN"),
    route!("Capture", NULL, "ASP_TX1"), route!("Capture", NULL, "ASP_TX2"),
    route!("Capture", NULL, "ASP_TX3"), route!("Capture", NULL, "ASP_TX4"),
    route!("Capture", NULL, "ASP_TX5"), route!("ASP_TX1", NULL, "ASP_TX1 Source"),
    route!("ASP_TX2", NULL, "ASP_TX2 Source"), route!("ASP_TX3", NULL, "ASP_TX3 Source"),
    route!("ASP_TX4", NULL, "ASP_TX4 Source"), route!("ASP_TX5", NULL, "ASP_TX5 Source"),
    route!("ASP_TX1", NULL, "ASP_EN"), route!("ASP_TX2", NULL, "ASP_EN"),
    route!("ASP_TX3", NULL, "ASP_EN"), route!("ASP_TX4", NULL, "ASP_EN"),
    route!("ASP_TX1", NULL, "GLOBAL_EN"), route!("ASP_TX2", NULL, "GLOBAL_EN"),
    route!("ASP_TX3", NULL, "GLOBAL_EN"), route!("ASP_TX4", NULL, "GLOBAL_EN"),
    route!("ASP_TX5", NULL, "GLOBAL_EN"),
    CS35L45_ASP_MUX_ROUTE!("ASP_TX1"), CS35L45_ASP_MUX_ROUTE!("ASP_TX2"),
    CS35L45_ASP_MUX_ROUTE!("ASP_TX3"), CS35L45_ASP_MUX_ROUTE!("ASP_TX4"),
    CS35L45_ASP_MUX_ROUTE!("ASP_TX5"),
    /* Playback */
    route!("ASP_RX1", NULL, "Playback"), route!("ASP_RX2", NULL, "Playback"),
    route!("ASP_RX1", NULL, "ASP_EN"), route!("ASP_RX2", NULL, "ASP_EN"),
    route!("AMP", NULL, "DACPCM Source"), route!("AMP", NULL, "GLOBAL_EN"),
    CS35L45_DSP_MUX_ROUTE!("DSP_RX1"), CS35L45_DSP_MUX_ROUTE!("DSP_RX2"),
    CS35L45_DSP_MUX_ROUTE!("DSP_RX3"), CS35L45_DSP_MUX_ROUTE!("DSP_RX4"),
    CS35L45_DSP_MUX_ROUTE!("DSP_RX5"), CS35L45_DSP_MUX_ROUTE!("DSP_RX6"),
    CS35L45_DSP_MUX_ROUTE!("DSP_RX7"), CS35L45_DSP_MUX_ROUTE!("DSP_RX8"),
    route!("DSP1", NULL, "DSP_RX1 Source"), route!("DSP1", NULL, "DSP_RX2 Source"),
    route!("DSP1", NULL, "DSP_RX3 Source"), route!("DSP1", NULL, "DSP_RX4 Source"),
    route!("DSP1", NULL, "DSP_RX5 Source"), route!("DSP1", NULL, "DSP_RX6 Source"),
    route!("DSP1", NULL, "DSP_RX7 Source"), route!("DSP1", NULL, "DSP_RX8 Source"),
    route!("DSP1", NULL, "VMON_EN"), route!("DSP1", NULL, "IMON_EN"),
    route!("DSP1", NULL, "VDD_BATTMON_EN"), route!("DSP1", NULL, "VDD_BSTMON_EN"),
    route!("DSP1", NULL, "TEMPMON_EN"), route!("DSP1 Preload", NULL, "DSP1 Preloader"),
    route!("DSP1", NULL, "DSP1 Preloader"), CS35L45_DAC_MUX_ROUTE!("DACPCM"),
    route!("AMP Enable", "Switch", "AMP"), route!("SPK", NULL, "AMP Enable"),
];

static amplifier_mode_texts: [&[u8]; 2] = [b"SPK\0", b"RCV\0"];
static amplifier_mode_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(SND_SOC_NOPM, 0, amplifier_mode_texts);
static amp_gain_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(1000, 300, 0);
static cs35l45_dig_pcm_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-10225, 25, true);

static cs35l45_controls: [snd_kcontrol_new; 5] = [
    SOC_ENUM_EXT!(c"Amplifier Mode", amplifier_mode_enum, cs35l45_amplifier_mode_get, cs35l45_amplifier_mode_put),
    SOC_SINGLE_TLV!(c"Analog PCM Volume", CS35L45_AMP_GAIN, CS35L45_AMP_GAIN_PCM_SHIFT,
                    CS35L45_AMP_GAIN_PCM_MASK >> CS35L45_AMP_GAIN_PCM_SHIFT, 0, amp_gain_tlv),
    /* Ignore bit 0: it is beyond the resolution of TLV_DB_SCALE */
    SOC_SINGLE_S_TLV!(c"Digital PCM Volume", CS35L45_AMP_PCM_CONTROL,
                      CS35L45_AMP_VOL_PCM_SHIFT + 1, -409, 48,
                      (CS35L45_AMP_VOL_PCM_WIDTH - 1) - 1, 0, cs35l45_dig_pcm_vol_tlv),
    WM_ADSP2_PRELOAD_SWITCH!(c"DSP1", 1),
    WM_ADSP_FW_CONTROL!(c"DSP1", 0),
];

unsafe extern "C" fn cs35l45_set_pll(cs35l45: *mut cs35l45_private, freq: c_uint) -> c_int {
    let mut val: c_uint = 0;
    let freq_id = cs35l45_get_clk_freq_id(freq);
    if freq_id < 0 {
        dev_err((*cs35l45).dev, c"Invalid freq: %u\n".as_ptr(), freq);
        return -EINVAL;
    }

    regmap_read((*cs35l45).regmap, CS35L45_REFCLK_INPUT, &mut val);
    val = (val & CS35L45_PLL_REFCLK_FREQ_MASK) >> CS35L45_PLL_REFCLK_FREQ_SHIFT;
    if val == freq_id as c_uint {
        return 0;
    }

    regmap_set_bits((*cs35l45).regmap, CS35L45_REFCLK_INPUT, CS35L45_PLL_OPEN_LOOP_MASK);
    regmap_update_bits((*cs35l45).regmap, CS35L45_REFCLK_INPUT, CS35L45_PLL_REFCLK_FREQ_MASK,
                       (freq_id as c_uint) << CS35L45_PLL_REFCLK_FREQ_SHIFT);
    regmap_clear_bits((*cs35l45).regmap, CS35L45_REFCLK_INPUT, CS35L45_PLL_REFCLK_EN_MASK);
    regmap_clear_bits((*cs35l45).regmap, CS35L45_REFCLK_INPUT, CS35L45_PLL_OPEN_LOOP_MASK);
    regmap_set_bits((*cs35l45).regmap, CS35L45_REFCLK_INPUT, CS35L45_PLL_REFCLK_EN_MASK);
    0
}

unsafe extern "C" fn cs35l45_asp_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata((*codec_dai).component);
    let asp_fmt: c_uint;
    let fsync_inv: c_uint;
    let bclk_inv: c_uint;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            dev_err((*cs35l45).dev, c"Invalid DAI clocking\n".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => asp_fmt = CS35l45_ASP_FMT_DSP_A,
        SND_SOC_DAIFMT_I2S => asp_fmt = CS35L45_ASP_FMT_I2S,
        _ => {
            dev_err((*cs35l45).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF => { fsync_inv = 1; bclk_inv = 0; }
        SND_SOC_DAIFMT_IB_NF => { fsync_inv = 0; bclk_inv = 1; }
        SND_SOC_DAIFMT_IB_IF => { fsync_inv = 1; bclk_inv = 1; }
        SND_SOC_DAIFMT_NB_NF => { fsync_inv = 0; bclk_inv = 0; }
        _ => {
            dev_warn((*cs35l45).dev, c"Invalid DAI clock polarity\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_CONTROL2,
                       CS35L45_ASP_FMT_MASK | CS35L45_ASP_FSYNC_INV_MASK | CS35L45_ASP_BCLK_INV_MASK,
                       (asp_fmt << CS35L45_ASP_FMT_SHIFT)
                           | (fsync_inv << CS35L45_ASP_FSYNC_INV_SHIFT)
                           | (bclk_inv << CS35L45_ASP_BCLK_INV_SHIFT));
    0
}

unsafe extern "C" fn cs35l45_asp_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata((*dai).component);
    let asp_width: c_uint;
    let asp_wl: c_uint;
    let global_fs: c_uint;
    let slot_multiple: c_uint;
    let mut asp_fmt: c_uint = 0;

    match params_rate(params) {
        44100 => global_fs = CS35L45_44P100_KHZ,
        48000 => global_fs = CS35L45_48P0_KHZ,
        88200 => global_fs = CS35L45_88P200_KHZ,
        96000 => global_fs = CS35L45_96P0_KHZ,
        _ => {
            dev_warn((*cs35l45).dev, c"Unsupported sample rate (%d)\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }

    regmap_update_bits((*cs35l45).regmap, CS35L45_GLOBAL_SAMPLE_RATE,
                       CS35L45_GLOBAL_FS_MASK, global_fs << CS35L45_GLOBAL_FS_SHIFT);

    asp_wl = params_width(params);
    if (*cs35l45).slot_width != 0 {
        asp_width = (*cs35l45).slot_width;
    } else {
        asp_width = params_width(params);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_CONTROL2,
                           CS35L45_ASP_WIDTH_RX_MASK, asp_width << CS35L45_ASP_WIDTH_RX_SHIFT);
        regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_DATA_CONTROL5,
                           CS35L45_ASP_WL_MASK, asp_wl << CS35L45_ASP_WL_SHIFT);
    } else {
        regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_CONTROL2,
                           CS35L45_ASP_WIDTH_TX_MASK, asp_width << CS35L45_ASP_WIDTH_TX_SHIFT);
        regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_DATA_CONTROL1,
                           CS35L45_ASP_WL_MASK, asp_wl << CS35L45_ASP_WL_SHIFT);
    }

    if (*cs35l45).sysclk_set {
        return 0;
    }

    /* I2S always has an even number of channels */
    regmap_read((*cs35l45).regmap, CS35L45_ASP_CONTROL2, &mut asp_fmt);
    asp_fmt = (asp_fmt & CS35L45_ASP_FMT_MASK) >> CS35L45_ASP_FMT_SHIFT;
    if asp_fmt == CS35L45_ASP_FMT_I2S {
        slot_multiple = 2;
    } else {
        slot_multiple = 1;
    }

    let bclk = snd_soc_tdm_params_to_bclk(params, asp_width, (*cs35l45).slot_count, slot_multiple);
    cs35l45_set_pll(cs35l45, bclk as c_uint)
}

unsafe extern "C" fn cs35l45_asp_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata((*dai).component);

    if slot_width != 0 && (slot_width < 16 || slot_width > 128) {
        return -EINVAL;
    }

    (*cs35l45).slot_width = slot_width as c_uint;
    (*cs35l45).slot_count = slots;
    0
}

unsafe extern "C" fn cs35l45_asp_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata((*dai).component);

    if clk_id != 0 {
        dev_err((*cs35l45).dev, c"Invalid clk_id %d\n".as_ptr(), clk_id);
        return -EINVAL;
    }

    (*cs35l45).sysclk_set = false;
    if freq == 0 {
        return 0;
    }

    let ret = cs35l45_set_pll(cs35l45, freq);
    if ret < 0 {
        return -EINVAL;
    }

    (*cs35l45).sysclk_set = true;
    0
}

unsafe extern "C" fn cs35l45_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata((*dai).component);
    let mut global_fs: c_uint = 0;
    let mut val: c_uint = 0;
    let hpf_tune: c_uint;

    if mute != 0 {
        return 0;
    }

    regmap_read((*cs35l45).regmap, CS35L45_GLOBAL_SAMPLE_RATE, &mut global_fs);
    global_fs = (global_fs & CS35L45_GLOBAL_FS_MASK) >> CS35L45_GLOBAL_FS_SHIFT;
    match global_fs {
        CS35L45_44P100_KHZ => hpf_tune = CS35L45_HPF_44P1,
        CS35L45_88P200_KHZ => hpf_tune = CS35L45_HPF_88P2,
        _ => hpf_tune = CS35l45_HPF_DEFAULT,
    }

    regmap_read((*cs35l45).regmap, CS35L45_AMP_PCM_HPF_TST, &mut val);
    if val != hpf_tune {
        let hpf_override_seq: [reg_sequence; 7] = [
            reg_sequence { reg: 0x00000040, def: 0x00000055 },
            reg_sequence { reg: 0x00000040, def: 0x000000AA },
            reg_sequence { reg: 0x00000044, def: 0x00000055 },
            reg_sequence { reg: 0x00000044, def: 0x000000AA },
            reg_sequence { reg: CS35L45_AMP_PCM_HPF_TST, def: hpf_tune },
            reg_sequence { reg: 0x00000040, def: 0x00000000 },
            reg_sequence { reg: 0x00000044, def: 0x00000000 },
        ];
        regmap_multi_reg_write((*cs35l45).regmap, hpf_override_seq.as_ptr(), hpf_override_seq.len());
    }

    0
}

static cs35l45_asp_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs35l45_asp_set_fmt),
    hw_params: Some(cs35l45_asp_hw_params),
    set_tdm_slot: Some(cs35l45_asp_set_tdm_slot),
    set_sysclk: Some(cs35l45_asp_set_sysclk),
    mute_stream: Some(cs35l45_mute_stream),
};

static mut cs35l45_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: c"cs35l45".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: CS35L45_RATES,
            formats: CS35L45_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Capture".as_ptr(),
            channels_min: 1,
            channels_max: 5,
            rates: CS35L45_RATES,
            formats: CS35L45_FORMATS,
        },
        symmetric_rate: true,
        symmetric_sample_bits: true,
        ops: &cs35l45_asp_dai_ops,
    },
];

unsafe extern "C" fn cs35l45_component_probe(component: *mut snd_soc_component) -> c_int {
    let cs35l45 = snd_soc_component_get_drvdata(component);
    wm_adsp2_component_probe(&mut (*cs35l45).dsp, component)
}

unsafe extern "C" fn cs35l45_component_remove(component: *mut snd_soc_component) {
    let cs35l45 = snd_soc_component_get_drvdata(component);
    wm_adsp2_component_remove(&mut (*cs35l45).dsp, component);
}

static cs35l45_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs35l45_component_probe),
    remove: Some(cs35l45_component_remove),
    dapm_widgets: cs35l45_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs35l45_dapm_widgets.len() as c_uint,
    dapm_routes: cs35l45_dapm_routes.as_ptr(),
    num_dapm_routes: cs35l45_dapm_routes.len() as c_uint,
    controls: cs35l45_controls.as_ptr(),
    num_controls: cs35l45_controls.len() as c_uint,
    name: c"cs35l45".as_ptr(),
    endianness: 1,
};

unsafe extern "C" fn cs35l45_setup_hibernate(cs35l45: *mut cs35l45_private) {
    let wksrc: c_uint = if (*cs35l45).bus_type == CONTROL_BUS_I2C {
        CS35L45_WKSRC_I2C
    } else {
        CS35L45_WKSRC_SPI
    };

    regmap_update_bits((*cs35l45).regmap, CS35L45_WAKESRC_CTL, CS35L45_WKSRC_EN_MASK,
                       wksrc << CS35L45_WKSRC_EN_SHIFT);
    regmap_set_bits((*cs35l45).regmap, CS35L45_WAKESRC_CTL, CS35L45_UPDT_WKCTL_MASK);
    regmap_update_bits((*cs35l45).regmap, CS35L45_WKI2C_CTL, CS35L45_WKI2C_ADDR_MASK, (*cs35l45).i2c_addr);
    regmap_set_bits((*cs35l45).regmap, CS35L45_WKI2C_CTL, CS35L45_UPDT_WKI2C_MASK);
}

unsafe extern "C" fn cs35l45_enter_hibernate(cs35l45: *mut cs35l45_private) -> c_int {
    dev_dbg((*cs35l45).dev, c"Enter hibernate\n".as_ptr());
    cs35l45_setup_hibernate(cs35l45);
    regmap_set_bits((*cs35l45).regmap, CS35L45_IRQ1_MASK_2, CS35L45_DSP_VIRT2_MBOX_MASK);
    // Don't wait for ACK since bus activity would wake the device
    regmap_write((*cs35l45).regmap, CS35L45_DSP_VIRT1_MBOX_1, CSPL_MBOX_CMD_HIBERNATE as c_uint);
    0
}

unsafe extern "C" fn cs35l45_exit_hibernate(cs35l45: *mut cs35l45_private) -> c_int {
    let wake_retries: c_int = 20;
    let sleep_retries: c_int = 5;
    let mut ret: c_int = 0;

    for i in 0..sleep_retries {
        dev_dbg((*cs35l45).dev, c"Exit hibernate\n".as_ptr());
        for j in 0..wake_retries {
            ret = cs35l45_set_cspl_mbox_cmd(cs35l45, (*cs35l45).regmap, CSPL_MBOX_CMD_OUT_OF_HIBERNATE);
            if ret == 0 {
                dev_dbg((*cs35l45).dev, c"Wake success at cycle: %d\n".as_ptr(), j);
                regmap_clear_bits((*cs35l45).regmap, CS35L45_IRQ1_MASK_2, CS35L45_DSP_VIRT2_MBOX_MASK);
                return 0;
            }
            usleep_range(100, 200);
        }

        dev_err((*cs35l45).dev, c"Wake failed, re-enter hibernate: %d\n".as_ptr(), ret);
        cs35l45_setup_hibernate(cs35l45);
    }

    dev_err((*cs35l45).dev, c"Timed out waking device\n".as_ptr());
    -ETIMEDOUT
}

unsafe extern "C" fn cs35l45_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    if !(*cs35l45).dsp.preloaded || !(*cs35l45).dsp.cs_dsp.running {
        return 0;
    }
    wm_adsp_hibernate(&mut (*cs35l45).dsp, true);
    cs35l45_enter_hibernate(cs35l45);
    regcache_cache_only((*cs35l45).regmap, true);
    regcache_mark_dirty((*cs35l45).regmap);
    dev_dbg((*cs35l45).dev, c"Runtime suspended\n".as_ptr());
    0
}

unsafe extern "C" fn cs35l45_runtime_resume(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    let mut ret: c_int;
    if !(*cs35l45).dsp.preloaded || !(*cs35l45).dsp.cs_dsp.running {
        return 0;
    }
    dev_dbg((*cs35l45).dev, c"Runtime resume\n".as_ptr());
    regcache_cache_only((*cs35l45).regmap, false);
    ret = cs35l45_exit_hibernate(cs35l45);
    if ret != 0 {
        return ret;
    }
    ret = regcache_sync((*cs35l45).regmap);
    if ret != 0 {
        dev_warn((*cs35l45).dev, c"regcache_sync failed: %d\n".as_ptr(), ret);
    }
    wm_adsp_hibernate(&mut (*cs35l45).dsp, false);
    /* Clear global error status */
    regmap_clear_bits((*cs35l45).regmap, CS35L45_ERROR_RELEASE, CS35L45_GLOBAL_ERR_RLS_MASK);
    regmap_set_bits((*cs35l45).regmap, CS35L45_ERROR_RELEASE, CS35L45_GLOBAL_ERR_RLS_MASK);
    regmap_clear_bits((*cs35l45).regmap, CS35L45_ERROR_RELEASE, CS35L45_GLOBAL_ERR_RLS_MASK);
    ret
}

unsafe extern "C" fn cs35l45_sys_suspend(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"System suspend, disabling IRQ\n".as_ptr());
    disable_irq((*cs35l45).irq);
    0
}

unsafe extern "C" fn cs35l45_sys_suspend_noirq(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"Late system suspend, reenabling IRQ\n".as_ptr());
    enable_irq((*cs35l45).irq);
    0
}

unsafe extern "C" fn cs35l45_sys_resume_noirq(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"Early system resume, disabling IRQ\n".as_ptr());
    disable_irq((*cs35l45).irq);
    0
}

unsafe extern "C" fn cs35l45_sys_resume(dev: *mut device) -> c_int {
    let cs35l45 = dev_get_drvdata(dev) as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"System resume, reenabling IRQ\n".as_ptr());
    enable_irq((*cs35l45).irq);
    0
}

unsafe extern "C" fn cs35l45_apply_property_config(cs35l45: *mut cs35l45_private) -> c_int {
    let node = (*(*cs35l45).dev).of_node;
    let gpio_regs: [c_uint; 3] = [CS35L45_GPIO1_CTRL1, CS35L45_GPIO2_CTRL1, CS35L45_GPIO3_CTRL1];
    let pad_regs: [c_uint; 3] = [CS35L45_SYNC_GPIO1, CS35L45_INTB_GPIO2_MCLK_REF, CS35L45_GPIO3];
    let mut child: *mut device_node;
    let mut val: c_uint = 0;
    let mut of_name: [c_char; 32] = [0; 32];
    let mut ret: c_int;

    if node.is_null() {
        return 0;
    }

    for i in 0..CS35L45_NUM_GPIOS as usize {
        sprintf(of_name.as_mut_ptr(), c"cirrus,gpio-ctrl%d".as_ptr(), (i + 1) as c_int);
        child = of_get_child_by_name(node, of_name.as_ptr());
        if child.is_null() {
            continue;
        }

        ret = of_property_read_u32(child, c"gpio-dir".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, gpio_regs[i], CS35L45_GPIO_DIR_MASK, val << CS35L45_GPIO_DIR_SHIFT);
        }
        ret = of_property_read_u32(child, c"gpio-lvl".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, gpio_regs[i], CS35L45_GPIO_LVL_MASK, val << CS35L45_GPIO_LVL_SHIFT);
        }
        ret = of_property_read_u32(child, c"gpio-op-cfg".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, gpio_regs[i], CS35L45_GPIO_OP_CFG_MASK, val << CS35L45_GPIO_OP_CFG_SHIFT);
        }
        ret = of_property_read_u32(child, c"gpio-pol".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, gpio_regs[i], CS35L45_GPIO_POL_MASK, val << CS35L45_GPIO_POL_SHIFT);
        }
        ret = of_property_read_u32(child, c"gpio-ctrl".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, pad_regs[i], CS35L45_GPIO_CTRL_MASK, val << CS35L45_GPIO_CTRL_SHIFT);
        }
        ret = of_property_read_u32(child, c"gpio-invert".as_ptr(), &mut val);
        if ret == 0 {
            regmap_update_bits((*cs35l45).regmap, pad_regs[i], CS35L45_GPIO_INVERT_MASK, val << CS35L45_GPIO_INVERT_SHIFT);
            if i == 1 {
                (*cs35l45).irq_invert = val;
            }
        }
        of_node_put(child);
    }

    if device_property_read_u32((*cs35l45).dev, c"cirrus,asp-sdout-hiz-ctrl".as_ptr(), &mut val) == 0 {
        regmap_update_bits((*cs35l45).regmap, CS35L45_ASP_CONTROL3,
                           CS35L45_ASP_DOUT_HIZ_CTRL_MASK, val << CS35L45_ASP_DOUT_HIZ_CTRL_SHIFT);
    }
    0
}

unsafe extern "C" fn cs35l45_dsp_virt2_mbox3_irq_handle(
    cs35l45: *mut cs35l45_private,
    cmd: c_uint,
    data: c_uint,
) -> c_int {
    static mut speak_status: *const c_char = c"Unknown".as_ptr();

    match cmd {
        EVENT_SPEAKER_STATUS => {
            match data {
                1 => speak_status = c"All Clear".as_ptr(),
                2 => speak_status = c"Open Circuit".as_ptr(),
                4 => speak_status = c"Short Circuit".as_ptr(),
                _ => {}
            }
            dev_info((*cs35l45).dev, c"MBOX event (SPEAKER_STATUS): %s\n".as_ptr(), speak_status);
        }
        EVENT_BOOT_DONE => {
            dev_dbg((*cs35l45).dev, c"MBOX event (BOOT_DONE)\n".as_ptr());
        }
        _ => {
            dev_err((*cs35l45).dev, c"MBOX event not supported %u\n".as_ptr(), cmd);
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn cs35l45_dsp_virt2_mbox_cb(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l45 = data as *mut cs35l45_private;
    let mut mbox_val: c_uint = 0;
    let mut ret: c_int = 0;

    ret = regmap_read((*cs35l45).regmap, CS35L45_DSP_VIRT2_MBOX_3, &mut mbox_val);
    if ret == 0 && mbox_val != 0 {
        cs35l45_dsp_virt2_mbox3_irq_handle(
            cs35l45,
            mbox_val & CS35L45_MBOX3_CMD_MASK,
            (mbox_val & CS35L45_MBOX3_DATA_MASK) >> CS35L45_MBOX3_DATA_SHIFT,
        );
    }

    /* Handle DSP trace log IRQ */
    ret = regmap_read((*cs35l45).regmap, CS35L45_DSP_VIRT2_MBOX_4, &mut mbox_val);
    if ret == 0 && mbox_val != 0 {
        dev_err((*cs35l45).dev, c"Spurious DSP MBOX4 IRQ\n".as_ptr());
    }

    IRQ_RETVAL(ret)
}

unsafe extern "C" fn cs35l45_pll_unlock(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l45 = data as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"PLL unlock detected!".as_ptr());
    IRQ_HANDLED
}

unsafe extern "C" fn cs35l45_pll_lock(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l45 = data as *mut cs35l45_private;
    dev_dbg((*cs35l45).dev, c"PLL lock detected!".as_ptr());
    IRQ_HANDLED
}

unsafe extern "C" fn cs35l45_spk_safe_err(irq: c_int, data: *mut c_void) -> irqreturn_t;

static cs35l45_irqs: [cs35l45_irq; 12] = [
    CS35L45_IRQ!(AMP_SHORT_ERR, c"Amplifier short error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(UVLO_VDDBATT_ERR, c"VDDBATT undervoltage error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(BST_SHORT_ERR, c"Boost inductor error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(BST_UVP_ERR, c"Boost undervoltage error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(TEMP_ERR, c"Overtemperature error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(AMP_CAL_ERR, c"Amplifier calibration error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(UVLO_VDDLV_ERR, c"LV threshold detector error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(GLOBAL_ERROR, c"Global error", cs35l45_spk_safe_err),
    CS35L45_IRQ!(DSP_WDT_EXPIRE, c"DSP Watchdog Timer", cs35l45_spk_safe_err),
    CS35L45_IRQ!(PLL_UNLOCK_FLAG_RISE, c"PLL unlock", cs35l45_pll_unlock),
    CS35L45_IRQ!(PLL_LOCK_FLAG, c"PLL lock", cs35l45_pll_lock),
    CS35L45_IRQ!(DSP_VIRT2_MBOX, c"DSP virtual MBOX 2 write flag", cs35l45_dsp_virt2_mbox_cb),
];

unsafe extern "C" fn cs35l45_spk_safe_err(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l45 = data as *mut cs35l45_private;
    let i = irq - regmap_irq_get_virq((*cs35l45).irq_data, 0);

    if i < 0 || i >= cs35l45_irqs.len() as c_int {
        dev_err((*cs35l45).dev, c"Unspecified global error condition (%d) detected!\n".as_ptr(), irq);
    } else {
        dev_err((*cs35l45).dev, c"%s condition detected!\n".as_ptr(), cs35l45_irqs[i as usize].name);
    }
    IRQ_HANDLED
}

static cs35l45_reg_irqs: [regmap_irq; 12] = [
    CS35L45_REG_IRQ!(IRQ1_EINT_1, AMP_SHORT_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_1, UVLO_VDDBATT_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_1, BST_SHORT_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_1, BST_UVP_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_1, TEMP_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_3, AMP_CAL_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_18, UVLO_VDDLV_ERR),
    CS35L45_REG_IRQ!(IRQ1_EINT_18, GLOBAL_ERROR),
    CS35L45_REG_IRQ!(IRQ1_EINT_2, DSP_WDT_EXPIRE),
    CS35L45_REG_IRQ!(IRQ1_EINT_3, PLL_UNLOCK_FLAG_RISE),
    CS35L45_REG_IRQ!(IRQ1_EINT_3, PLL_LOCK_FLAG),
    CS35L45_REG_IRQ!(IRQ1_EINT_2, DSP_VIRT2_MBOX),
];

static cs35l45_regmap_irq_chip: regmap_irq_chip = regmap_irq_chip {
    name: c"cs35l45 IRQ1 Controller".as_ptr(),
    main_status: CS35L45_IRQ1_STATUS,
    status_base: CS35L45_IRQ1_EINT_1,
    mask_base: CS35L45_IRQ1_MASK_1,
    ack_base: CS35L45_IRQ1_EINT_1,
    num_regs: 18,
    irqs: cs35l45_reg_irqs.as_ptr(),
    num_irqs: cs35l45_reg_irqs.len() as c_int,
    runtime_pm: true,
};

unsafe extern "C" fn cs35l45_initialize(cs35l45: *mut cs35l45_private) -> c_int {
    let dev = (*cs35l45).dev;
    let mut dev_id: [c_uint; 5] = [0; 5];
    let mut sts: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_read_poll_timeout!(CS35L45_IRQ1_EINT_4, sts, (sts & CS35L45_OTP_BOOT_DONE_STS_MASK) != 0, 1000, 5000, (*cs35l45).regmap);
    if ret < 0 {
        dev_err((*cs35l45).dev, c"Timeout waiting for OTP boot\n".as_ptr());
        return ret;
    }

    ret = regmap_bulk_read((*cs35l45).regmap, CS35L45_DEVID, dev_id.as_mut_ptr(), dev_id.len());
    if ret != 0 {
        dev_err((*cs35l45).dev, c"Get Device ID failed: %d\n".as_ptr(), ret);
        return ret;
    }

    match dev_id[0] {
        0x35A450 | 0x35A460 => {}
        _ => {
            dev_err((*cs35l45).dev, c"Bad DEVID 0x%x\n".as_ptr(), dev_id[0]);
            return -ENODEV;
        }
    }

    dev_info((*cs35l45).dev, c"Cirrus Logic CS35L45: REVID %02X OTPID %02X\n".as_ptr(), dev_id[1], dev_id[4]);
    regmap_write((*cs35l45).regmap, CS35L45_IRQ1_EINT_4, CS35L45_OTP_BOOT_DONE_STS_MASK | CS35L45_OTP_BUSY_MASK);

    ret = cs35l45_apply_patch(cs35l45);
    if ret < 0 {
        dev_err(dev, c"Failed to apply init patch %d\n".as_ptr(), ret);
        return ret;
    }

    ret = cs35l45_apply_property_config(cs35l45);
    if ret < 0 {
        return ret;
    }

    (*cs35l45).amplifier_mode = AMP_MODE_SPK as i64;
    0
}

static cs35l45_fs_errata_patch: [reg_sequence; 16] = [
    reg_sequence { reg: 0x02B80080, def: 0x00000001 },
    reg_sequence { reg: 0x02B80088, def: 0x00000001 },
    reg_sequence { reg: 0x02B80090, def: 0x00000001 },
    reg_sequence { reg: 0x02B80098, def: 0x00000001 },
    reg_sequence { reg: 0x02B800A0, def: 0x00000001 },
    reg_sequence { reg: 0x02B800A8, def: 0x00000001 },
    reg_sequence { reg: 0x02B800B0, def: 0x00000001 },
    reg_sequence { reg: 0x02B800B8, def: 0x00000001 },
    reg_sequence { reg: 0x02B80280, def: 0x00000001 },
    reg_sequence { reg: 0x02B80288, def: 0x00000001 },
    reg_sequence { reg: 0x02B80290, def: 0x00000001 },
    reg_sequence { reg: 0x02B80298, def: 0x00000001 },
    reg_sequence { reg: 0x02B802A0, def: 0x00000001 },
    reg_sequence { reg: 0x02B802A8, def: 0x00000001 },
    reg_sequence { reg: 0x02B802B0, def: 0x00000001 },
    reg_sequence { reg: 0x02B802B8, def: 0x00000001 },
];

static cs35l45_dsp1_regions: [cs_dsp_region; 5] = [
    cs_dsp_region { type_: WMFW_HALO_PM_PACKED, base: CS35L45_DSP1_PMEM_0 },
    cs_dsp_region { type_: WMFW_HALO_XM_PACKED, base: CS35L45_DSP1_XMEM_PACK_0 },
    cs_dsp_region { type_: WMFW_HALO_YM_PACKED, base: CS35L45_DSP1_YMEM_PACK_0 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: CS35L45_DSP1_XMEM_UNPACK24_0 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: CS35L45_DSP1_YMEM_UNPACK24_0 },
];

unsafe extern "C" fn cs35l45_dsp_init(cs35l45: *mut cs35l45_private) -> c_int {
    let dsp = &mut (*cs35l45).dsp as *mut wm_adsp;

    (*dsp).part = c"cs35l45".as_ptr();
    (*dsp).fw = 9; /* 9 is WM_ADSP_FW_SPK_PROT in wm_adsp.c */
    (*dsp).toggle_preload = true;
    (*dsp).cs_dsp.num = 1;
    (*dsp).cs_dsp.type_ = WMFW_HALO;
    (*dsp).cs_dsp.rev = 0;
    (*dsp).cs_dsp.dev = (*cs35l45).dev;
    (*dsp).cs_dsp.regmap = (*cs35l45).regmap;
    (*dsp).cs_dsp.base = CS35L45_DSP1_CLOCK_FREQ;
    (*dsp).cs_dsp.base_sysinfo = CS35L45_DSP1_SYS_ID;
    (*dsp).cs_dsp.mem = cs35l45_dsp1_regions.as_ptr();
    (*dsp).cs_dsp.num_mems = cs35l45_dsp1_regions.len() as c_int;
    (*dsp).cs_dsp.lock_regions = 0xFFFFFFFF;

    let ret = wm_halo_init(dsp);
    regmap_multi_reg_write((*cs35l45).regmap, cs35l45_fs_errata_patch.as_ptr(), cs35l45_fs_errata_patch.len());
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l45_probe(cs35l45: *mut cs35l45_private) -> c_int {
    let dev = (*cs35l45).dev;
    let mut irq_pol: c_ulong = IRQF_ONESHOT | IRQF_SHARED;
    let mut ret: c_int;
    let mut irq: c_int;

    (*cs35l45).vdd_batt = devm_regulator_get(dev, c"vdd-batt".as_ptr());
    if IS_ERR((*cs35l45).vdd_batt as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*cs35l45).vdd_batt as *const c_void), c"Failed to request vdd-batt\n".as_ptr());
    }

    (*cs35l45).vdd_a = devm_regulator_get(dev, c"vdd-a".as_ptr());
    if IS_ERR((*cs35l45).vdd_a as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*cs35l45).vdd_a as *const c_void), c"Failed to request vdd-a\n".as_ptr());
    }

    /* VDD_BATT must always be enabled before other supplies */
    ret = regulator_enable((*cs35l45).vdd_batt);
    if ret < 0 {
        return dev_err_probe(dev, ret, c"Failed to enable vdd-batt\n".as_ptr());
    }

    ret = regulator_enable((*cs35l45).vdd_a);
    if ret < 0 {
        return dev_err_probe(dev, ret, c"Failed to enable vdd-a\n".as_ptr());
    }

    /* If reset is shared only one instance can claim it */
    (*cs35l45).reset_gpio = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*cs35l45).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs35l45).reset_gpio as *const c_void);
        (*cs35l45).reset_gpio = core::ptr::null_mut();
        if ret == -EBUSY {
            dev_dbg(dev, c"Reset line busy, assuming shared reset\n".as_ptr());
        } else {
            dev_err_probe(dev, ret, c"Failed to get reset GPIO\n".as_ptr());
            goto_err(cs35l45, ret);
            return ret;
        }
    }

    if !(*cs35l45).reset_gpio.is_null() {
        usleep_range(CS35L45_RESET_HOLD_US, CS35L45_RESET_HOLD_US + 100);
        gpiod_set_value_cansleep((*cs35l45).reset_gpio, 1);
    }

    usleep_range(CS35L45_RESET_US, CS35L45_RESET_US + 100);

    ret = cs35l45_initialize(cs35l45);
    if ret < 0 {
        goto_err_reset(cs35l45);
        return ret;
    }

    ret = cs35l45_dsp_init(cs35l45);
    if ret < 0 {
        goto_err_reset(cs35l45);
        return ret;
    }

    pm_runtime_set_autosuspend_delay((*cs35l45).dev, 3000);
    pm_runtime_use_autosuspend((*cs35l45).dev);
    pm_runtime_set_active((*cs35l45).dev);
    pm_runtime_get_noresume((*cs35l45).dev);
    pm_runtime_enable((*cs35l45).dev);

    if (*cs35l45).irq != 0 {
        if (*cs35l45).irq_invert != 0 {
            irq_pol |= IRQF_TRIGGER_HIGH;
        } else {
            irq_pol |= IRQF_TRIGGER_LOW;
        }

        ret = devm_regmap_add_irq_chip(dev, (*cs35l45).regmap, (*cs35l45).irq, irq_pol, 0,
                                       &cs35l45_regmap_irq_chip, &mut (*cs35l45).irq_data);
        if ret != 0 {
            dev_err(dev, c"Failed to register IRQ chip: %d\n".as_ptr(), ret);
            goto_err_dsp(cs35l45);
            return ret;
        }

        for i in 0..cs35l45_irqs.len() {
            irq = regmap_irq_get_virq((*cs35l45).irq_data, cs35l45_irqs[i].irq);
            if irq < 0 {
                dev_err(dev, c"Failed to get %s\n".as_ptr(), cs35l45_irqs[i].name);
                ret = irq;
                goto_err_dsp(cs35l45);
                return ret;
            }

            ret = devm_request_threaded_irq(dev, irq, None, cs35l45_irqs[i].handler,
                                            irq_pol, cs35l45_irqs[i].name, cs35l45 as *mut c_void);
            if ret != 0 {
                dev_err(dev, c"Failed to request IRQ %s: %d\n".as_ptr(), cs35l45_irqs[i].name, ret);
                goto_err_dsp(cs35l45);
                return ret;
            }
        }
    }

    ret = devm_snd_soc_register_component(dev, &cs35l45_component, cs35l45_dai.as_mut_ptr(), cs35l45_dai.len() as c_int);
    if ret < 0 {
        goto_err_dsp(cs35l45);
        return ret;
    }

    pm_runtime_put_autosuspend((*cs35l45).dev);
    0
}

unsafe fn goto_err_dsp(cs35l45: *mut cs35l45_private) {
    pm_runtime_disable((*cs35l45).dev);
    pm_runtime_put_noidle((*cs35l45).dev);
    wm_adsp2_remove(&mut (*cs35l45).dsp);
    goto_err_reset(cs35l45);
}

unsafe fn goto_err_reset(cs35l45: *mut cs35l45_private) {
    gpiod_set_value_cansleep((*cs35l45).reset_gpio, 0);
    goto_err(cs35l45, 0);
}

unsafe fn goto_err(cs35l45: *mut cs35l45_private, _ret: c_int) {
    regulator_disable((*cs35l45).vdd_a);
    regulator_disable((*cs35l45).vdd_batt);
}

// EXPORT_SYMBOL_NS_GPL(cs35l45_probe, "SND_SOC_CS35L45");

#[no_mangle]
pub unsafe extern "C" fn cs35l45_remove(cs35l45: *mut cs35l45_private) {
    pm_runtime_get_sync((*cs35l45).dev);
    pm_runtime_disable((*cs35l45).dev);
    wm_adsp2_remove(&mut (*cs35l45).dsp);

    gpiod_set_value_cansleep((*cs35l45).reset_gpio, 0);

    pm_runtime_put_noidle((*cs35l45).dev);
    regulator_disable((*cs35l45).vdd_a);
    /* VDD_BATT must be the last to power-off */
    regulator_disable((*cs35l45).vdd_batt);
}
// EXPORT_SYMBOL_NS_GPL(cs35l45_remove, "SND_SOC_CS35L45");

// EXPORT_GPL_DEV_PM_OPS(cs35l45_pm_ops) = {
//     RUNTIME_PM_OPS(cs35l45_runtime_suspend, cs35l45_runtime_resume, NULL)
//     SYSTEM_SLEEP_PM_OPS(cs35l45_sys_suspend, cs35l45_sys_resume)
//     NOIRQ_SYSTEM_SLEEP_PM_OPS(cs35l45_sys_suspend_noirq, cs35l45_sys_resume_noirq)
// };

// MODULE_DESCRIPTION("ASoC CS35L45 driver");
// MODULE_AUTHOR("James Schulman, Cirrus Logic Inc, <james.schulman@cirrus.com>");
// MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
