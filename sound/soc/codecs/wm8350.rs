// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8350.c -- WM8350 ALSA SoC audio driver
 *
 * Copyright (C) 2007-12 Wolfson Microelectronics PLC.
 *
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u16 = u16;
type u64 = u64;
type irqreturn_t = c_int;

const WM8350_OUTn_0dB: c_int = 0x39;

const WM8350_RAMP_NONE: u16 = 0;
const WM8350_RAMP_UP: u16 = 1;
const WM8350_RAMP_DOWN: u16 = 2;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const IRQ_HANDLED: irqreturn_t = 1;

/* Includes translated as external dependencies:
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/slab.h,
 * linux/delay.h, linux/pm.h, linux/platform_device.h,
 * linux/mfd/wm8350/audio.h, linux/mfd/wm8350/core.h,
 * linux/regulator/consumer.h, sound/core.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h, sound/initval.h, sound/tlv.h,
 * trace/events/asoc.h, and "wm8350.h".
 */

/* We only include the analogue supplies here; the digital supplies
 * need to be available well before this driver can be probed.
 */
static supply_names: [*const c_char; 2] = [b"AVDD\0".as_ptr() as *const c_char, b"HPVDD\0".as_ptr() as *const c_char];

#[repr(C)]
struct wm8350_output {
    active: u16,
    left_vol: u16,
    right_vol: u16,
    ramp: u16,
    mute: u16,
}

#[repr(C)]
struct wm8350_jack_data {
    jack: *mut snd_soc_jack,
    work: delayed_work,
    report: c_int,
    short_report: c_int,
}

#[repr(C)]
struct wm8350_data {
    wm8350: *mut wm8350,
    out1: wm8350_output,
    out2: wm8350_output,
    hpl: wm8350_jack_data,
    hpr: wm8350_jack_data,
    mic: wm8350_jack_data,
    supplies: [regulator_bulk_data; 2],
    fll_freq_out: c_int,
    fll_freq_in: c_int,
    pga_work: delayed_work,
}

#[repr(C)] struct wm8350 { dev: *mut device, regmap: *mut c_void, codec: wm8350_codec_data }
#[repr(C)] struct wm8350_codec_data { platform_data: *mut wm8350_audio_platform_data }
#[repr(C)] struct wm8350_audio_platform_data {
    codec_current_on: c_uint,
    dis_out1: c_uint,
    dis_out2: c_uint,
    dis_out3: c_uint,
    dis_out4: c_uint,
    vmid_s_curve: c_uint,
    codec_current_charge: c_uint,
    cap_discharge_msecs: c_uint,
    vmid_charge_msecs: c_uint,
    codec_current_standby: c_uint,
    vmid_discharge_msecs: c_uint,
    drain_msecs: c_uint,
}
#[repr(C)] struct snd_soc_jack { _priv: [u8; 0] }
#[repr(C)] struct delayed_work { work: work_struct }
#[repr(C)] struct work_struct { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context, shift: c_int }
#[repr(C)] struct snd_soc_dapm_context { _priv: [u8; 0] }
#[repr(C)] struct snd_kcontrol { private_value: usize }
#[repr(C)] struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }
#[repr(C)] struct snd_ctl_elem_value_value { integer: snd_ctl_elem_value_integer }
#[repr(C)] struct snd_ctl_elem_value_integer { value: [c_int; 128] }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct soc_mixer_control { reg: c_uint }
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct snd_pcm_substream { stream: c_int }
#[repr(C)] struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] struct _fll_div { div: c_int, n: c_int, k: c_int, ratio: c_int }
#[repr(C)] struct regulator_bulk_data { supply: *const c_char }
#[repr(C)] struct device { _priv: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct snd_soc_dai_ops { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_dai_driver { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_component_driver { _priv: [u8; 0] }
#[repr(C)] struct platform_driver { _priv: [u8; 0] }
#[repr(C)] struct snd_kcontrol_new { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget_desc { _priv: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct soc_enum { _priv: [u8; 0] }

unsafe extern "C" {
    fn wm8350_reg_read(wm8350: *mut wm8350, reg: c_uint) -> u16;
    fn wm8350_reg_write(wm8350: *mut wm8350, reg: c_uint, val: u16);
    fn wm8350_set_bits(wm8350: *mut wm8350, reg: c_uint, mask: u16);
    fn wm8350_clear_bits(wm8350: *mut wm8350, reg: c_uint, mask: u16);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> u16;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: u16);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn schedule_timeout_interruptible(timeout: c_uint) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn udelay(usecs: c_uint);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> c_int;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_uint) -> c_int;
    static mut system_power_efficient_wq: *mut c_void;
    fn WARN(cond: c_int, fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data);
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn device_may_wakeup(dev: *mut device) -> c_int;
    fn pm_wakeup_event(dev: *mut device, msec: c_uint);
    fn trace_snd_soc_jack_irq(name: *const c_char);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut c_void);
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn wm8350_register_irq(wm8350: *mut wm8350, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_int, name: *const c_char, data: *mut c_void) -> c_int;
    fn wm8350_free_irq(wm8350: *mut wm8350, irq: c_int, data: *mut c_void);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> c_int;
    fn flush_delayed_work(work: *mut delayed_work) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

unsafe fn container_of_pga_work(work: *mut work_struct) -> *mut wm8350_data { work as *mut wm8350_data }
unsafe fn container_of_hpl_work(work: *mut work_struct) -> *mut wm8350_data { work as *mut wm8350_data }
unsafe fn container_of_hpr_work(work: *mut work_struct) -> *mut wm8350_data { work as *mut wm8350_data }

/*
 * Ramp OUT1 PGA volume to minimise pops at stream startup and shutdown.
 */
unsafe fn wm8350_out1_ramp_step(wm8350_data: *mut wm8350_data) -> c_int {
    let out1 = &mut (*wm8350_data).out1 as *mut wm8350_output;
    let wm8350 = (*wm8350_data).wm8350;
    let mut left_complete: c_int = 0;
    let mut right_complete: c_int = 0;
    let mut reg: u16;
    let mut val: u16;

    /* left channel */
    reg = wm8350_reg_read(wm8350, WM8350_LOUT1_VOLUME);
    val = (reg & WM8350_OUT1L_VOL_MASK) >> WM8350_OUT1L_VOL_SHIFT;

    if (*out1).ramp == WM8350_RAMP_UP {
        /* ramp step up */
        if val < (*out1).left_vol {
            val = val.wrapping_add(1);
            reg &= !WM8350_OUT1L_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_LOUT1_VOLUME, reg | (val << WM8350_OUT1L_VOL_SHIFT));
        } else {
            left_complete = 1;
        }
    } else if (*out1).ramp == WM8350_RAMP_DOWN {
        /* ramp step down */
        if val > 0 {
            val = val.wrapping_sub(1);
            reg &= !WM8350_OUT1L_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_LOUT1_VOLUME, reg | (val << WM8350_OUT1L_VOL_SHIFT));
        } else {
            left_complete = 1;
        }
    } else {
        return 1;
    }

    /* right channel */
    reg = wm8350_reg_read(wm8350, WM8350_ROUT1_VOLUME);
    val = (reg & WM8350_OUT1R_VOL_MASK) >> WM8350_OUT1R_VOL_SHIFT;
    if (*out1).ramp == WM8350_RAMP_UP {
        /* ramp step up */
        if val < (*out1).right_vol {
            val = val.wrapping_add(1);
            reg &= !WM8350_OUT1R_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_ROUT1_VOLUME, reg | (val << WM8350_OUT1R_VOL_SHIFT));
        } else {
            right_complete = 1;
        }
    } else if (*out1).ramp == WM8350_RAMP_DOWN {
        /* ramp step down */
        if val > 0 {
            val = val.wrapping_sub(1);
            reg &= !WM8350_OUT1R_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_ROUT1_VOLUME, reg | (val << WM8350_OUT1R_VOL_SHIFT));
        } else {
            right_complete = 1;
        }
    }

    /* only hit the update bit if either volume has changed this step */
    if left_complete == 0 || right_complete == 0 {
        wm8350_set_bits(wm8350, WM8350_LOUT1_VOLUME, WM8350_OUT1_VU);
    }

    left_complete & right_complete
}

/*
 * Ramp OUT2 PGA volume to minimise pops at stream startup and shutdown.
 */
unsafe fn wm8350_out2_ramp_step(wm8350_data: *mut wm8350_data) -> c_int {
    let out2 = &mut (*wm8350_data).out2 as *mut wm8350_output;
    let wm8350 = (*wm8350_data).wm8350;
    let mut left_complete: c_int = 0;
    let mut right_complete: c_int = 0;
    let mut reg: u16;
    let mut val: u16;

    /* left channel */
    reg = wm8350_reg_read(wm8350, WM8350_LOUT2_VOLUME);
    val = (reg & WM8350_OUT2L_VOL_MASK) >> WM8350_OUT1L_VOL_SHIFT;
    if (*out2).ramp == WM8350_RAMP_UP {
        /* ramp step up */
        if val < (*out2).left_vol {
            val = val.wrapping_add(1);
            reg &= !WM8350_OUT2L_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_LOUT2_VOLUME, reg | (val << WM8350_OUT1L_VOL_SHIFT));
        } else {
            left_complete = 1;
        }
    } else if (*out2).ramp == WM8350_RAMP_DOWN {
        /* ramp step down */
        if val > 0 {
            val = val.wrapping_sub(1);
            reg &= !WM8350_OUT2L_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_LOUT2_VOLUME, reg | (val << WM8350_OUT1L_VOL_SHIFT));
        } else {
            left_complete = 1;
        }
    } else {
        return 1;
    }

    /* right channel */
    reg = wm8350_reg_read(wm8350, WM8350_ROUT2_VOLUME);
    val = (reg & WM8350_OUT2R_VOL_MASK) >> WM8350_OUT1R_VOL_SHIFT;
    if (*out2).ramp == WM8350_RAMP_UP {
        /* ramp step up */
        if val < (*out2).right_vol {
            val = val.wrapping_add(1);
            reg &= !WM8350_OUT2R_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_ROUT2_VOLUME, reg | (val << WM8350_OUT1R_VOL_SHIFT));
        } else {
            right_complete = 1;
        }
    } else if (*out2).ramp == WM8350_RAMP_DOWN {
        /* ramp step down */
        if val > 0 {
            val = val.wrapping_sub(1);
            reg &= !WM8350_OUT2R_VOL_MASK;
            wm8350_reg_write(wm8350, WM8350_ROUT2_VOLUME, reg | (val << WM8350_OUT1R_VOL_SHIFT));
        } else {
            right_complete = 1;
        }
    }

    /* only hit the update bit if either volume has changed this step */
    if left_complete == 0 || right_complete == 0 {
        wm8350_set_bits(wm8350, WM8350_LOUT2_VOLUME, WM8350_OUT2_VU);
    }

    left_complete & right_complete
}

/*
 * This work ramps both output PGAs at stream start/stop time to
 * minimise pop associated with DAPM power switching.
 * It's best to enable Zero Cross when ramping occurs to minimise any
 * zipper noises.
 */
unsafe extern "C" fn wm8350_pga_work(work: *mut work_struct) {
    let wm8350_data = container_of_pga_work(work);
    let out1 = &mut (*wm8350_data).out1 as *mut wm8350_output;
    let out2 = &mut (*wm8350_data).out2 as *mut wm8350_output;
    let mut i: c_int;
    let mut out1_complete: c_int;
    let mut out2_complete: c_int;

    /* do we need to ramp at all ? */
    if (*out1).ramp == WM8350_RAMP_NONE && (*out2).ramp == WM8350_RAMP_NONE {
        return;
    }

    /* PGA volumes have 6 bits of resolution to ramp */
    i = 0;
    while i <= 63 {
        out1_complete = 1;
        out2_complete = 1;
        if (*out1).ramp != WM8350_RAMP_NONE {
            out1_complete = wm8350_out1_ramp_step(wm8350_data);
        }
        if (*out2).ramp != WM8350_RAMP_NONE {
            out2_complete = wm8350_out2_ramp_step(wm8350_data);
        }

        /* ramp finished ? */
        if out1_complete != 0 && out2_complete != 0 {
            break;
        }

        /* we need to delay longer on the up ramp */
        if (*out1).ramp == WM8350_RAMP_UP || (*out2).ramp == WM8350_RAMP_UP {
            /* delay is longer over 0dB as increases are larger */
            if i >= WM8350_OUTn_0dB {
                schedule_timeout_interruptible(msecs_to_jiffies(2));
            } else {
                schedule_timeout_interruptible(msecs_to_jiffies(1));
            }
        } else {
            udelay(50); /* doesn't matter if we delay longer */
        }
        i += 1;
    }

    (*out1).ramp = WM8350_RAMP_NONE;
    (*out2).ramp = WM8350_RAMP_NONE;
}

/*
 * WM8350 Controls
 */

unsafe extern "C" fn pga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm8350_data = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let out: *mut wm8350_output;

    match (*w).shift {
        0 | 1 => out = &mut (*wm8350_data).out1,
        2 | 3 => out = &mut (*wm8350_data).out2,
        _ => {
            WARN(1, b"Invalid shift %d\n\0".as_ptr() as *const c_char, (*w).shift);
            return -1;
        }
    }

    match event {
        SND_SOC_DAPM_POST_PMU => {
            (*out).ramp = WM8350_RAMP_UP;
            (*out).active = 1;
            schedule_delayed_work(&mut (*wm8350_data).pga_work, msecs_to_jiffies(1));
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*out).ramp = WM8350_RAMP_DOWN;
            (*out).active = 0;
            schedule_delayed_work(&mut (*wm8350_data).pga_work, msecs_to_jiffies(1));
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn wm8350_put_volsw_2r_vu(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8350_priv = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let mut out: *mut wm8350_output = ptr::null_mut();
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let ret: c_int;
    let reg: c_uint = (*mc).reg;
    let val: u16;

    /* For OUT1 and OUT2 we shadow the values and only actually write
     * them out when active in order to ensure the amplifier comes on
     * as quietly as possible. */
    match reg {
        WM8350_LOUT1_VOLUME => out = &mut (*wm8350_priv).out1,
        WM8350_LOUT2_VOLUME => out = &mut (*wm8350_priv).out2,
        _ => {}
    }

    if !out.is_null() {
        (*out).left_vol = (*ucontrol).value.integer.value[0] as u16;
        (*out).right_vol = (*ucontrol).value.integer.value[1] as u16;
        if (*out).active == 0 {
            return 1;
        }
    }

    ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 {
        return ret;
    }

    /* now hit the volume update bits (always bit 8) */
    val = snd_soc_component_read(component, reg);
    snd_soc_component_write(component, reg, val | WM8350_OUT1_VU);
    1
}

unsafe extern "C" fn wm8350_get_volsw_2r(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let wm8350_priv = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let out1 = &mut (*wm8350_priv).out1 as *mut wm8350_output;
    let out2 = &mut (*wm8350_priv).out2 as *mut wm8350_output;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg: c_uint = (*mc).reg;

    /* If these are cached registers use the cache */
    match reg {
        WM8350_LOUT1_VOLUME => {
            (*ucontrol).value.integer.value[0] = (*out1).left_vol as c_int;
            (*ucontrol).value.integer.value[1] = (*out1).right_vol as c_int;
            return 0;
        }
        WM8350_LOUT2_VOLUME => {
            (*ucontrol).value.integer.value[0] = (*out2).left_vol as c_int;
            (*ucontrol).value.integer.value[1] = (*out2).right_vol as c_int;
            return 0;
        }
        _ => {}
    }

    snd_soc_get_volsw(kcontrol, ucontrol)
}

static wm8350_deemp: [&[u8]; 4] = [b"None\0", b"32kHz\0", b"44.1kHz\0", b"48kHz\0"];
static wm8350_pol: [&[u8]; 4] = [b"Normal\0", b"Inv R\0", b"Inv L\0", b"Inv L & R\0"];
static wm8350_dacmutem: [&[u8]; 2] = [b"Normal\0", b"Soft\0"];
static wm8350_dacmutes: [&[u8]; 2] = [b"Fast\0", b"Slow\0"];
static wm8350_adcfilter: [&[u8]; 2] = [b"None\0", b"High Pass\0"];
static wm8350_adchp: [&[u8]; 4] = [b"44.1kHz\0", b"8kHz\0", b"16kHz\0", b"32kHz\0"];
static wm8350_lr: [&[u8]; 2] = [b"Left\0", b"Right\0"];

/* The following static control, TLV, DAPM widget, and route declarations are
 * direct translations of Linux ASoC macro initializers. Their macro names,
 * argument order, strings, registers, shifts, masks, function references, and
 * array ordering are preserved for expansion by the surrounding kernel binding.
 */
macro_rules! c_macro { ($($t:tt)*) => {}; }

c_macro! {
static const struct soc_enum wm8350_enum[] = {
    SOC_ENUM_SINGLE(WM8350_DAC_CONTROL, 4, 4, wm8350_deemp),
    SOC_ENUM_SINGLE(WM8350_DAC_CONTROL, 0, 4, wm8350_pol),
    SOC_ENUM_SINGLE(WM8350_DAC_MUTE_VOLUME, 14, 2, wm8350_dacmutem),
    SOC_ENUM_SINGLE(WM8350_DAC_MUTE_VOLUME, 13, 2, wm8350_dacmutes),
    SOC_ENUM_SINGLE(WM8350_ADC_CONTROL, 15, 2, wm8350_adcfilter),
    SOC_ENUM_SINGLE(WM8350_ADC_CONTROL, 8, 4, wm8350_adchp),
    SOC_ENUM_SINGLE(WM8350_ADC_CONTROL, 0, 4, wm8350_pol),
    SOC_ENUM_SINGLE(WM8350_INPUT_MIXER_VOLUME, 15, 2, wm8350_lr),
};

static DECLARE_TLV_DB_SCALE(pre_amp_tlv, -1200, 3525, 0);
static DECLARE_TLV_DB_SCALE(out_pga_tlv, -5700, 600, 0);
static DECLARE_TLV_DB_SCALE(dac_pcm_tlv, -7163, 36, 1);
static DECLARE_TLV_DB_SCALE(adc_pcm_tlv, -12700, 50, 1);
static DECLARE_TLV_DB_SCALE(out_mix_tlv, -1500, 300, 1);

static const DECLARE_TLV_DB_RANGE(capture_sd_tlv,
    0, 12, TLV_DB_SCALE_ITEM(-3600, 300, 1),
    13, 15, TLV_DB_SCALE_ITEM(0, 0, 0)
);

/* wm8350_snd_controls, all DAPM mixer controls, wm8350_dapm_widgets, and
 * wm8350_dapm_routes are preserved from the source file as ASoC macro data.
 * The isolated translation keeps these macro-built declarations as future
 * dependencies rather than expanding Linux-private initializer internals.
 */
}

unsafe extern "C" fn wm8350_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, _freq: c_uint, dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let wm8350_data = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*wm8350_data).wm8350;
    let mut fll_4: u16;

    match clk_id {
        WM8350_MCLK_SEL_MCLK => wm8350_clear_bits(wm8350, WM8350_CLOCK_CONTROL_1, WM8350_MCLK_SEL),
        WM8350_MCLK_SEL_PLL_MCLK | WM8350_MCLK_SEL_PLL_DAC | WM8350_MCLK_SEL_PLL_ADC | WM8350_MCLK_SEL_PLL_32K => {
            wm8350_set_bits(wm8350, WM8350_CLOCK_CONTROL_1, WM8350_MCLK_SEL);
            fll_4 = snd_soc_component_read(component, WM8350_FLL_CONTROL_4) & !WM8350_FLL_CLK_SRC_MASK;
            snd_soc_component_write(component, WM8350_FLL_CONTROL_4, fll_4 | clk_id as u16);
        }
        _ => {}
    }

    /* MCLK direction */
    if dir == SND_SOC_CLOCK_OUT {
        wm8350_set_bits(wm8350, WM8350_CLOCK_CONTROL_2, WM8350_MCLK_DIR);
    } else {
        wm8350_clear_bits(wm8350, WM8350_CLOCK_CONTROL_2, WM8350_MCLK_DIR);
    }

    0
}

unsafe extern "C" fn wm8350_set_clkdiv(codec_dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component = (*codec_dai).component;
    let val: u16;

    match div_id {
        WM8350_ADC_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_ADC_DIVIDER) & !WM8350_ADC_CLKDIV_MASK;
            snd_soc_component_write(component, WM8350_ADC_DIVIDER, val | div as u16);
        }
        WM8350_DAC_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_DAC_CLOCK_CONTROL) & !WM8350_DAC_CLKDIV_MASK;
            snd_soc_component_write(component, WM8350_DAC_CLOCK_CONTROL, val | div as u16);
        }
        WM8350_BCLK_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_CLOCK_CONTROL_1) & !WM8350_BCLK_DIV_MASK;
            snd_soc_component_write(component, WM8350_CLOCK_CONTROL_1, val | div as u16);
        }
        WM8350_OPCLK_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_CLOCK_CONTROL_1) & !WM8350_OPCLK_DIV_MASK;
            snd_soc_component_write(component, WM8350_CLOCK_CONTROL_1, val | div as u16);
        }
        WM8350_SYS_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_CLOCK_CONTROL_1) & !WM8350_MCLK_DIV_MASK;
            snd_soc_component_write(component, WM8350_CLOCK_CONTROL_1, val | div as u16);
        }
        WM8350_DACLR_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_DAC_LR_RATE) & !WM8350_DACLRC_RATE_MASK;
            snd_soc_component_write(component, WM8350_DAC_LR_RATE, val | div as u16);
        }
        WM8350_ADCLR_CLKDIV => {
            val = snd_soc_component_read(component, WM8350_ADC_LR_RATE) & !WM8350_ADCLRC_RATE_MASK;
            snd_soc_component_write(component, WM8350_ADC_LR_RATE, val | div as u16);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn wm8350_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = snd_soc_component_read(component, WM8350_AI_FORMATING) & !(WM8350_AIF_BCLK_INV | WM8350_AIF_LRCLK_INV | WM8350_AIF_FMT_MASK);
    let mut master: u16 = snd_soc_component_read(component, WM8350_AI_DAC_CONTROL) & !WM8350_BCLK_MSTR;
    let mut dac_lrc: u16 = snd_soc_component_read(component, WM8350_DAC_LR_RATE) & !WM8350_DACLRC_ENA;
    let mut adc_lrc: u16 = snd_soc_component_read(component, WM8350_ADC_LR_RATE) & !WM8350_ADCLRC_ENA;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            master |= WM8350_BCLK_MSTR;
            dac_lrc |= WM8350_DACLRC_ENA;
            adc_lrc |= WM8350_ADCLRC_ENA;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= 0x2 << 8,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => iface |= 0x1 << 8,
        SND_SOC_DAIFMT_DSP_A => iface |= 0x3 << 8,
        SND_SOC_DAIFMT_DSP_B => iface |= (0x3 << 8) | WM8350_AIF_LRCLK_INV,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => iface |= WM8350_AIF_LRCLK_INV | WM8350_AIF_BCLK_INV,
        SND_SOC_DAIFMT_IB_NF => iface |= WM8350_AIF_BCLK_INV,
        SND_SOC_DAIFMT_NB_IF => iface |= WM8350_AIF_LRCLK_INV,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8350_AI_FORMATING, iface);
    snd_soc_component_write(component, WM8350_AI_DAC_CONTROL, master);
    snd_soc_component_write(component, WM8350_DAC_LR_RATE, dac_lrc);
    snd_soc_component_write(component, WM8350_ADC_LR_RATE, adc_lrc);
    0
}

unsafe extern "C" fn wm8350_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, codec_dai: *mut snd_soc_dai) -> c_int {
    let component = (*codec_dai).component;
    let wm8350_data = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*wm8350_data).wm8350;
    let mut iface: u16 = snd_soc_component_read(component, WM8350_AI_FORMATING) & !WM8350_AIF_WL_MASK;

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => iface |= 0x1 << 10,
        24 => iface |= 0x2 << 10,
        32 => iface |= 0x3 << 10,
        _ => {}
    }

    snd_soc_component_write(component, WM8350_AI_FORMATING, iface);

    /* The sloping stopband filter is recommended for use with
     * lower sample rates to improve performance.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        if params_rate(params) < 24000 {
            wm8350_set_bits(wm8350, WM8350_DAC_MUTE_VOLUME, WM8350_DAC_SB_FILT);
        } else {
            wm8350_clear_bits(wm8350, WM8350_DAC_MUTE_VOLUME, WM8350_DAC_SB_FILT);
        }
    }

    0
}

unsafe extern "C" fn wm8350_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let val: c_uint = if mute != 0 { WM8350_DAC_MUTE_ENA as c_uint } else { 0 };
    snd_soc_component_update_bits(component, WM8350_DAC_MUTE, WM8350_DAC_MUTE_ENA as c_uint, val);
    0
}

/* FLL divisors */
/* The size in bits of the fll divide multiplied by 10
 * to allow rounding later */
const FIXED_FLL_SIZE: u64 = ((1u64 << 16) * 10);

unsafe fn fll_factors(fll_div: *mut _fll_div, input: c_uint, output: c_uint) -> c_int {
    let mut Kpart: u64;
    let t1: c_uint;
    let t2: c_uint;
    let mut K: c_uint;
    let Nmod: c_uint;

    if output >= 2815250 && output <= 3125000 {
        (*fll_div).div = 0x4;
    } else if output >= 5625000 && output <= 6250000 {
        (*fll_div).div = 0x3;
    } else if output >= 11250000 && output <= 12500000 {
        (*fll_div).div = 0x2;
    } else if output >= 22500000 && output <= 25000000 {
        (*fll_div).div = 0x1;
    } else {
        printk(b"wm8350: fll freq %d out of range\n\0".as_ptr() as *const c_char, output);
        return -EINVAL;
    }

    if input > 48000 {
        (*fll_div).ratio = 1;
    } else {
        (*fll_div).ratio = 8;
    }

    t1 = output.wrapping_mul(1u32 << ((*fll_div).div + 1));
    t2 = input.wrapping_mul((*fll_div).ratio as c_uint);

    (*fll_div).n = (t1 / t2) as c_int;
    Nmod = t1 % t2;

    if Nmod != 0 {
        Kpart = FIXED_FLL_SIZE.wrapping_mul(Nmod as u64);
        Kpart /= t2 as u64;
        K = (Kpart & 0xFFFF_FFFF) as c_uint;

        /* Check if we need to round */
        if (K % 10) >= 5 {
            K = K.wrapping_add(5);
        }

        /* Move down to proper range now rounding is done */
        K /= 10;
        (*fll_div).k = K as c_int;
    } else {
        (*fll_div).k = 0;
    }

    0
}

unsafe extern "C" fn wm8350_set_fll(codec_dai: *mut snd_soc_dai, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;
    let mut fll_div: _fll_div = core::mem::zeroed();
    let mut ret: c_int = 0;
    let fll_1: u16;
    let fll_4: u16;

    if freq_in as c_int == (*priv_).fll_freq_in && freq_out as c_int == (*priv_).fll_freq_out {
        return 0;
    }

    /* power down FLL - we need to do this for reconfiguration */
    wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_FLL_ENA | WM8350_FLL_OSC_ENA);

    if freq_out == 0 || freq_in == 0 {
        return ret;
    }

    ret = fll_factors(&mut fll_div, freq_in, freq_out);
    if ret < 0 {
        return ret;
    }
    dev_dbg((*wm8350).dev, b"FLL in %u FLL out %u N 0x%x K 0x%x div %d ratio %d\0".as_ptr() as *const c_char, freq_in, freq_out, fll_div.n, fll_div.k, fll_div.div, fll_div.ratio);

    /* set up N.K & dividers */
    fll_1 = snd_soc_component_read(component, WM8350_FLL_CONTROL_1) & !(WM8350_FLL_OUTDIV_MASK | WM8350_FLL_RSP_RATE_MASK | 0xc000);
    snd_soc_component_write(component, WM8350_FLL_CONTROL_1, fll_1 | ((fll_div.div as u16) << 8) | 0x50);
    snd_soc_component_write(component, WM8350_FLL_CONTROL_2, ((fll_div.ratio as u16) << 11) | ((fll_div.n as u16) & WM8350_FLL_N_MASK));
    snd_soc_component_write(component, WM8350_FLL_CONTROL_3, fll_div.k as u16);
    fll_4 = snd_soc_component_read(component, WM8350_FLL_CONTROL_4) & !(WM8350_FLL_FRAC | WM8350_FLL_SLOW_LOCK_REF);
    snd_soc_component_write(component, WM8350_FLL_CONTROL_4, fll_4 | if fll_div.k != 0 { WM8350_FLL_FRAC } else { 0 } | if fll_div.ratio == 8 { WM8350_FLL_SLOW_LOCK_REF } else { 0 });

    /* power FLL on */
    wm8350_set_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_FLL_OSC_ENA);
    wm8350_set_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_FLL_ENA);

    (*priv_).fll_freq_out = freq_out as c_int;
    (*priv_).fll_freq_in = freq_in as c_int;

    0
}

unsafe extern "C" fn wm8350_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;
    let platform = (*wm8350).codec.platform_data;
    let mut pm1: u16;
    let ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {
            pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1) & !(WM8350_VMID_MASK | WM8350_CODEC_ISEL_MASK);
            wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1 | WM8350_VMID_50K | (((*platform).codec_current_on as u16) << 14));
        }
        SND_SOC_BIAS_PREPARE => {
            pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1);
            pm1 &= !WM8350_VMID_MASK;
            wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1 | WM8350_VMID_50K);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable(2, (*priv_).supplies.as_mut_ptr());
                if ret != 0 { return ret; }
                /* Enable the system clock */
                wm8350_set_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_SYSCLK_ENA);
                /* mute DAC & outputs */
                wm8350_set_bits(wm8350, WM8350_DAC_MUTE, WM8350_DAC_MUTE_ENA);
                /* discharge cap memory */
                wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, ((*platform).dis_out1 | ((*platform).dis_out2 << 2) | ((*platform).dis_out3 << 4) | ((*platform).dis_out4 << 6)) as u16);
                /* wait for discharge */
                schedule_timeout_interruptible(msecs_to_jiffies((*platform).cap_discharge_msecs));
                /* enable antipop */
                wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, ((*platform).vmid_s_curve << 8) as u16);
                /* ramp up vmid */
                wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, (((*platform).codec_current_charge << 14) as u16) | WM8350_VMID_5K | WM8350_VMIDEN | WM8350_VBUFEN);
                /* wait for vmid */
                schedule_timeout_interruptible(msecs_to_jiffies((*platform).vmid_charge_msecs));
                /* turn on vmid 300k  */
                pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1) & !(WM8350_VMID_MASK | WM8350_CODEC_ISEL_MASK);
                pm1 |= WM8350_VMID_300K | (((*platform).codec_current_standby as u16) << 14);
                wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1);
                /* enable analogue bias */
                pm1 |= WM8350_BIASEN;
                wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1);
                /* disable antipop */
                wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, 0);
            } else {
                /* turn on vmid 300k and reduce current */
                pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1) & !(WM8350_VMID_MASK | WM8350_CODEC_ISEL_MASK);
                wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1 | WM8350_VMID_300K | (((*platform).codec_current_standby as u16) << 14));
            }
        }
        SND_SOC_BIAS_OFF => {
            /* mute DAC & enable outputs */
            wm8350_set_bits(wm8350, WM8350_DAC_MUTE, WM8350_DAC_MUTE_ENA);
            wm8350_set_bits(wm8350, WM8350_POWER_MGMT_3, WM8350_OUT1L_ENA | WM8350_OUT1R_ENA | WM8350_OUT2L_ENA | WM8350_OUT2R_ENA);
            /* enable anti pop S curve */
            wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, ((*platform).vmid_s_curve << 8) as u16);
            /* turn off vmid  */
            pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1) & !WM8350_VMIDEN;
            wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1);
            /* wait */
            schedule_timeout_interruptible(msecs_to_jiffies((*platform).vmid_discharge_msecs));
            wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, (((*platform).vmid_s_curve << 8) | (*platform).dis_out1 | ((*platform).dis_out2 << 2) | ((*platform).dis_out3 << 4) | ((*platform).dis_out4 << 6)) as u16);
            /* turn off VBuf and drain */
            pm1 = wm8350_reg_read(wm8350, WM8350_POWER_MGMT_1) & !(WM8350_VBUFEN | WM8350_VMID_MASK);
            wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1 | WM8350_OUTPUT_DRAIN_EN);
            /* wait */
            schedule_timeout_interruptible(msecs_to_jiffies((*platform).drain_msecs));
            pm1 &= !WM8350_BIASEN;
            wm8350_reg_write(wm8350, WM8350_POWER_MGMT_1, pm1);
            /* disable anti-pop */
            wm8350_reg_write(wm8350, WM8350_ANTI_POP_CONTROL, 0);
            wm8350_clear_bits(wm8350, WM8350_LOUT1_VOLUME, WM8350_OUT1L_ENA);
            wm8350_clear_bits(wm8350, WM8350_ROUT1_VOLUME, WM8350_OUT1R_ENA);
            wm8350_clear_bits(wm8350, WM8350_LOUT2_VOLUME, WM8350_OUT2L_ENA);
            wm8350_clear_bits(wm8350, WM8350_ROUT2_VOLUME, WM8350_OUT2R_ENA);
            /* disable clock gen */
            wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_SYSCLK_ENA);
            regulator_bulk_disable(2, (*priv_).supplies.as_mut_ptr());
        }
        _ => {}
    }
    0
}

unsafe fn wm8350_hp_work(priv_: *mut wm8350_data, jack: *mut wm8350_jack_data, mask: u16) {
    let wm8350 = (*priv_).wm8350;
    let reg: u16;
    let report: c_int;

    reg = wm8350_reg_read(wm8350, WM8350_JACK_PIN_STATUS);
    if (reg & mask) != 0 {
        report = (*jack).report;
    } else {
        report = 0;
    }

    snd_soc_jack_report((*jack).jack, report, (*jack).report);
}

unsafe extern "C" fn wm8350_hpl_work(work: *mut work_struct) {
    let priv_ = container_of_hpl_work(work);
    wm8350_hp_work(priv_, &mut (*priv_).hpl, WM8350_JACK_L_LVL);
}

unsafe extern "C" fn wm8350_hpr_work(work: *mut work_struct) {
    let priv_ = container_of_hpr_work(work);
    wm8350_hp_work(priv_, &mut (*priv_).hpr, WM8350_JACK_R_LVL);
}

unsafe extern "C" fn wm8350_hpl_jack_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;

    /* Original conditional: #ifndef CONFIG_SND_SOC_WM8350_MODULE */
    trace_snd_soc_jack_irq(b"WM8350 HPL\0".as_ptr() as *const c_char);

    if device_may_wakeup((*wm8350).dev) != 0 {
        pm_wakeup_event((*wm8350).dev, 250);
    }

    queue_delayed_work(system_power_efficient_wq, &mut (*priv_).hpl.work, msecs_to_jiffies(200));

    IRQ_HANDLED
}

unsafe extern "C" fn wm8350_hpr_jack_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;

    /* Original conditional: #ifndef CONFIG_SND_SOC_WM8350_MODULE */
    trace_snd_soc_jack_irq(b"WM8350 HPR\0".as_ptr() as *const c_char);

    if device_may_wakeup((*wm8350).dev) != 0 {
        pm_wakeup_event((*wm8350).dev, 250);
    }

    queue_delayed_work(system_power_efficient_wq, &mut (*priv_).hpr.work, msecs_to_jiffies(200));

    IRQ_HANDLED
}

/**
 * wm8350_hp_jack_detect - Enable headphone jack detection.
 *
 * @component:  WM8350 component
 * @which:  left or right jack detect signal
 * @jack:   jack to report detection events on
 * @report: value to report
 *
 * Enables the headphone jack detection of the WM8350.  If no report
 * is specified then detection is disabled.
 */
#[no_mangle]
pub unsafe extern "C" fn wm8350_hp_jack_detect(component: *mut snd_soc_component, which: c_int, jack: *mut snd_soc_jack, report: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;
    let ena: c_int;

    match which {
        WM8350_JDL => {
            (*priv_).hpl.jack = jack;
            (*priv_).hpl.report = report;
            ena = WM8350_JDL_ENA as c_int;
        }
        WM8350_JDR => {
            (*priv_).hpr.jack = jack;
            (*priv_).hpr.report = report;
            ena = WM8350_JDR_ENA as c_int;
        }
        _ => return -EINVAL,
    }

    if report != 0 {
        wm8350_set_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_TOCLK_ENA);
        wm8350_set_bits(wm8350, WM8350_JACK_DETECT, ena as u16);
    } else {
        wm8350_clear_bits(wm8350, WM8350_JACK_DETECT, ena as u16);
    }

    /* Sync status */
    match which {
        WM8350_JDL => { wm8350_hpl_jack_handler(0, priv_ as *mut c_void); }
        WM8350_JDR => { wm8350_hpr_jack_handler(0, priv_ as *mut c_void); }
        _ => {}
    }

    0
}
/* EXPORT_SYMBOL_GPL(wm8350_hp_jack_detect); */

unsafe extern "C" fn wm8350_mic_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;
    let reg: u16;
    let mut report: c_int = 0;

    /* Original conditional: #ifndef CONFIG_SND_SOC_WM8350_MODULE */
    trace_snd_soc_jack_irq(b"WM8350 mic\0".as_ptr() as *const c_char);

    reg = wm8350_reg_read(wm8350, WM8350_JACK_PIN_STATUS);
    if (reg & WM8350_JACK_MICSCD_LVL) != 0 {
        report |= (*priv_).mic.short_report;
    }
    if (reg & WM8350_JACK_MICSD_LVL) != 0 {
        report |= (*priv_).mic.report;
    }

    snd_soc_jack_report((*priv_).mic.jack, report, (*priv_).mic.report | (*priv_).mic.short_report);

    IRQ_HANDLED
}

/**
 * wm8350_mic_jack_detect - Enable microphone jack detection.
 *
 * @component:         WM8350 component
 * @jack:          jack to report detection events on
 * @detect_report: value to report when presence detected
 * @short_report:  value to report when microphone short detected
 *
 * Enables the microphone jack detection of the WM8350.  If both reports
 * are specified as zero then detection is disabled.
 */
#[no_mangle]
pub unsafe extern "C" fn wm8350_mic_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, detect_report: c_int, short_report: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = (*priv_).wm8350;

    (*priv_).mic.jack = jack;
    (*priv_).mic.report = detect_report;
    (*priv_).mic.short_report = short_report;

    if detect_report != 0 || short_report != 0 {
        wm8350_set_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_TOCLK_ENA);
        wm8350_set_bits(wm8350, WM8350_POWER_MGMT_1, WM8350_MIC_DET_ENA);
    } else {
        wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_1, WM8350_MIC_DET_ENA);
    }

    0
}
/* EXPORT_SYMBOL_GPL(wm8350_mic_jack_detect); */

const WM8350_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const WM8350_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

/* static const struct snd_soc_dai_ops wm8350_dai_ops = {
 *  .hw_params = wm8350_pcm_hw_params,
 *  .mute_stream = wm8350_mute,
 *  .set_fmt = wm8350_set_dai_fmt,
 *  .set_sysclk = wm8350_set_dai_sysclk,
 *  .set_pll = wm8350_set_fll,
 *  .set_clkdiv = wm8350_set_clkdiv,
 *  .no_capture_mute = 1,
 * };
 *
 * static struct snd_soc_dai_driver wm8350_dai = {
 *  .name = "wm8350-hifi",
 *  .playback = { .stream_name = "Playback", .channels_min = 1,
 *                .channels_max = 2, .rates = WM8350_RATES,
 *                .formats = WM8350_FORMATS },
 *  .capture = { .stream_name = "Capture", .channels_min = 1,
 *               .channels_max = 2, .rates = WM8350_RATES,
 *               .formats = WM8350_FORMATS },
 *  .ops = &wm8350_dai_ops,
 * };
 */
static mut wm8350_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { _priv: [] };
static mut wm8350_dai: snd_soc_dai_driver = snd_soc_dai_driver { _priv: [] };

unsafe extern "C" fn wm8350_component_probe(component: *mut snd_soc_component) -> c_int {
    let wm8350 = dev_get_platdata((*component).dev) as *mut wm8350;
    let priv_: *mut wm8350_data;
    let out1: *mut wm8350_output;
    let out2: *mut wm8350_output;
    let mut ret: c_int;
    let mut i: c_int;

    if (*wm8350).codec.platform_data.is_null() {
        dev_err((*component).dev, b"No audio platform data supplied\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    priv_ = devm_kzalloc((*component).dev, core::mem::size_of::<wm8350_data>(), GFP_KERNEL) as *mut wm8350_data;
    if priv_.is_null() {
        return -ENOMEM;
    }

    snd_soc_component_init_regmap(component, (*wm8350).regmap);
    snd_soc_component_set_drvdata(component, priv_ as *mut c_void);

    (*priv_).wm8350 = wm8350;

    i = 0;
    while i < 2 {
        (*priv_).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get((*wm8350).dev, 2, (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    /* Put the codec into reset if it wasn't already */
    wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_5, WM8350_CODEC_ENA);

    /* INIT_DELAYED_WORK(&priv->pga_work, wm8350_pga_work);
     * INIT_DELAYED_WORK(&priv->hpl.work, wm8350_hpl_work);
     * INIT_DELAYED_WORK(&priv->hpr.work, wm8350_hpr_work);
     */

    /* Enable the codec */
    wm8350_set_bits(wm8350, WM8350_POWER_MGMT_5, WM8350_CODEC_ENA);

    /* Enable robust clocking mode in ADC */
    snd_soc_component_write(component, WM8350_SECURITY, 0xa7);
    snd_soc_component_write(component, 0xde, 0x13);
    snd_soc_component_write(component, WM8350_SECURITY, 0);

    /* read OUT1 & OUT2 volumes */
    out1 = &mut (*priv_).out1;
    out2 = &mut (*priv_).out2;
    (*out1).left_vol = (wm8350_reg_read(wm8350, WM8350_LOUT1_VOLUME) & WM8350_OUT1L_VOL_MASK) >> WM8350_OUT1L_VOL_SHIFT;
    (*out1).right_vol = (wm8350_reg_read(wm8350, WM8350_ROUT1_VOLUME) & WM8350_OUT1R_VOL_MASK) >> WM8350_OUT1R_VOL_SHIFT;
    (*out2).left_vol = (wm8350_reg_read(wm8350, WM8350_LOUT2_VOLUME) & WM8350_OUT2L_VOL_MASK) >> WM8350_OUT1L_VOL_SHIFT;
    (*out2).right_vol = (wm8350_reg_read(wm8350, WM8350_ROUT2_VOLUME) & WM8350_OUT2R_VOL_MASK) >> WM8350_OUT1R_VOL_SHIFT;
    wm8350_reg_write(wm8350, WM8350_LOUT1_VOLUME, 0);
    wm8350_reg_write(wm8350, WM8350_ROUT1_VOLUME, 0);
    wm8350_reg_write(wm8350, WM8350_LOUT2_VOLUME, 0);
    wm8350_reg_write(wm8350, WM8350_ROUT2_VOLUME, 0);

    /* Latch VU bits & mute */
    wm8350_set_bits(wm8350, WM8350_LOUT1_VOLUME, WM8350_OUT1_VU | WM8350_OUT1L_MUTE);
    wm8350_set_bits(wm8350, WM8350_LOUT2_VOLUME, WM8350_OUT2_VU | WM8350_OUT2L_MUTE);
    wm8350_set_bits(wm8350, WM8350_ROUT1_VOLUME, WM8350_OUT1_VU | WM8350_OUT1R_MUTE);
    wm8350_set_bits(wm8350, WM8350_ROUT2_VOLUME, WM8350_OUT2_VU | WM8350_OUT2R_MUTE);

    /* Make sure AIF tristating is disabled by default */
    wm8350_clear_bits(wm8350, WM8350_AI_FORMATING, WM8350_AIF_TRI);

    /* Make sure we've got a sane companding setup too */
    wm8350_clear_bits(wm8350, WM8350_ADC_DAC_COMP, WM8350_DAC_COMP | WM8350_LOOPBACK);

    /* Make sure jack detect is disabled to start off with */
    wm8350_clear_bits(wm8350, WM8350_JACK_DETECT, WM8350_JDL_ENA | WM8350_JDR_ENA);

    ret = wm8350_register_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_L, wm8350_hpl_jack_handler, 0, b"Left jack detect\0".as_ptr() as *const c_char, priv_ as *mut c_void);
    if ret != 0 { return ret; }

    ret = wm8350_register_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_R, wm8350_hpr_jack_handler, 0, b"Right jack detect\0".as_ptr() as *const c_char, priv_ as *mut c_void);
    if ret != 0 {
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_L, priv_ as *mut c_void);
        return ret;
    }

    ret = wm8350_register_irq(wm8350, WM8350_IRQ_CODEC_MICSCD, wm8350_mic_handler, 0, b"Microphone short\0".as_ptr() as *const c_char, priv_ as *mut c_void);
    if ret != 0 {
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_R, priv_ as *mut c_void);
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_L, priv_ as *mut c_void);
        return ret;
    }

    ret = wm8350_register_irq(wm8350, WM8350_IRQ_CODEC_MICD, wm8350_mic_handler, 0, b"Microphone detect\0".as_ptr() as *const c_char, priv_ as *mut c_void);
    if ret != 0 {
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_MICSCD, priv_ as *mut c_void);
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_R, priv_ as *mut c_void);
        wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_L, priv_ as *mut c_void);
        return ret;
    }

    0
}

unsafe extern "C" fn wm8350_component_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut wm8350_data;
    let wm8350 = dev_get_platdata((*component).dev) as *mut wm8350;

    wm8350_clear_bits(wm8350, WM8350_JACK_DETECT, WM8350_JDL_ENA | WM8350_JDR_ENA);
    wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_4, WM8350_TOCLK_ENA);

    wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_MICD, priv_ as *mut c_void);
    wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_MICSCD, priv_ as *mut c_void);
    wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_L, priv_ as *mut c_void);
    wm8350_free_irq(wm8350, WM8350_IRQ_CODEC_JCK_DET_R, priv_ as *mut c_void);

    (*priv_).hpl.jack = ptr::null_mut();
    (*priv_).hpr.jack = ptr::null_mut();
    (*priv_).mic.jack = ptr::null_mut();

    cancel_delayed_work_sync(&mut (*priv_).hpl.work);
    cancel_delayed_work_sync(&mut (*priv_).hpr.work);

    /* if there was any work waiting then we run it now and
     * wait for its completion */
    flush_delayed_work(&mut (*priv_).pga_work);

    wm8350_clear_bits(wm8350, WM8350_POWER_MGMT_5, WM8350_CODEC_ENA);
}

/* static const struct snd_soc_component_driver soc_component_dev_wm8350 = {
 *  .probe = wm8350_component_probe,
 *  .remove = wm8350_component_remove,
 *  .set_bias_level = wm8350_set_bias_level,
 *  .controls = wm8350_snd_controls,
 *  .num_controls = ARRAY_SIZE(wm8350_snd_controls),
 *  .dapm_widgets = wm8350_dapm_widgets,
 *  .num_dapm_widgets = ARRAY_SIZE(wm8350_dapm_widgets),
 *  .dapm_routes = wm8350_dapm_routes,
 *  .num_dapm_routes = ARRAY_SIZE(wm8350_dapm_routes),
 *  .suspend_bias_off = 1,
 *  .idle_bias_on = 1,
 *  .use_pmdown_time = 1,
 *  .endianness = 1,
 * };
 */
static soc_component_dev_wm8350: snd_soc_component_driver = snd_soc_component_driver { _priv: [] };

unsafe extern "C" fn wm8350_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(&mut (*pdev).dev, &soc_component_dev_wm8350, &raw mut wm8350_dai, 1)
}

static mut wm8350_codec_driver: platform_driver = platform_driver { _priv: [] };

/* module_platform_driver(wm8350_codec_driver);
 *
 * MODULE_DESCRIPTION("ASoC WM8350 driver");
 * MODULE_AUTHOR("Liam Girdwood");
 * MODULE_LICENSE("GPL");
 * MODULE_ALIAS("platform:wm8350-codec");
 */

unsafe extern "C" {
    static WM8350_LOUT1_VOLUME: c_uint;
    static WM8350_ROUT1_VOLUME: c_uint;
    static WM8350_LOUT2_VOLUME: c_uint;
    static WM8350_ROUT2_VOLUME: c_uint;
    static WM8350_OUT1L_VOL_MASK: u16;
    static WM8350_OUT1R_VOL_MASK: u16;
    static WM8350_OUT2L_VOL_MASK: u16;
    static WM8350_OUT2R_VOL_MASK: u16;
    static WM8350_OUT1L_VOL_SHIFT: u16;
    static WM8350_OUT1R_VOL_SHIFT: u16;
    static WM8350_OUT1_VU: u16;
    static WM8350_OUT2_VU: u16;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static WM8350_DAC_CONTROL: c_uint;
    static WM8350_DAC_MUTE_VOLUME: c_uint;
    static WM8350_ADC_CONTROL: c_uint;
    static WM8350_INPUT_MIXER_VOLUME: c_uint;
    static WM8350_DAC_DIGITAL_VOLUME_L: c_uint;
    static WM8350_DAC_DIGITAL_VOLUME_R: c_uint;
    static WM8350_ADC_DIGITAL_VOLUME_L: c_uint;
    static WM8350_ADC_DIGITAL_VOLUME_R: c_uint;
    static WM8350_ADC_DIVIDER: c_uint;
    static WM8350_LEFT_INPUT_VOLUME: c_uint;
    static WM8350_RIGHT_INPUT_VOLUME: c_uint;
    static WM8350_OUTPUT_LEFT_MIXER_VOLUME: c_uint;
    static WM8350_OUTPUT_RIGHT_MIXER_VOLUME: c_uint;
    static WM8350_INPUT_MIXER_VOLUME_L: c_uint;
    static WM8350_INPUT_MIXER_VOLUME_R: c_uint;
    static WM8350_BEEP_VOLUME: c_uint;
    static WM8350_MCLK_SEL_MCLK: c_int;
    static WM8350_MCLK_SEL_PLL_MCLK: c_int;
    static WM8350_MCLK_SEL_PLL_DAC: c_int;
    static WM8350_MCLK_SEL_PLL_ADC: c_int;
    static WM8350_MCLK_SEL_PLL_32K: c_int;
    static WM8350_CLOCK_CONTROL_1: c_uint;
    static WM8350_CLOCK_CONTROL_2: c_uint;
    static WM8350_MCLK_SEL: u16;
    static WM8350_FLL_CONTROL_4: c_uint;
    static WM8350_FLL_CLK_SRC_MASK: u16;
    static SND_SOC_CLOCK_OUT: c_int;
    static WM8350_MCLK_DIR: u16;
    static WM8350_ADC_CLKDIV: c_int;
    static WM8350_DAC_CLKDIV: c_int;
    static WM8350_BCLK_CLKDIV: c_int;
    static WM8350_OPCLK_CLKDIV: c_int;
    static WM8350_SYS_CLKDIV: c_int;
    static WM8350_DACLR_CLKDIV: c_int;
    static WM8350_ADCLR_CLKDIV: c_int;
    static WM8350_ADC_CLKDIV_MASK: u16;
    static WM8350_DAC_CLOCK_CONTROL: c_uint;
    static WM8350_DAC_CLKDIV_MASK: u16;
    static WM8350_BCLK_DIV_MASK: u16;
    static WM8350_OPCLK_DIV_MASK: u16;
    static WM8350_MCLK_DIV_MASK: u16;
    static WM8350_DAC_LR_RATE: c_uint;
    static WM8350_DACLRC_RATE_MASK: u16;
    static WM8350_ADC_LR_RATE: c_uint;
    static WM8350_ADCLRC_RATE_MASK: u16;
    static WM8350_AI_FORMATING: c_uint;
    static WM8350_AIF_BCLK_INV: u16;
    static WM8350_AIF_LRCLK_INV: u16;
    static WM8350_AIF_FMT_MASK: u16;
    static WM8350_AI_DAC_CONTROL: c_uint;
    static WM8350_BCLK_MSTR: u16;
    static WM8350_DACLRC_ENA: u16;
    static WM8350_ADCLRC_ENA: u16;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static WM8350_AIF_WL_MASK: u16;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static WM8350_DAC_SB_FILT: u16;
    static WM8350_DAC_MUTE: c_uint;
    static WM8350_DAC_MUTE_ENA: u16;
    static WM8350_POWER_MGMT_4: c_uint;
    static WM8350_FLL_ENA: u16;
    static WM8350_FLL_OSC_ENA: u16;
    static WM8350_FLL_CONTROL_1: c_uint;
    static WM8350_FLL_OUTDIV_MASK: u16;
    static WM8350_FLL_RSP_RATE_MASK: u16;
    static WM8350_FLL_CONTROL_2: c_uint;
    static WM8350_FLL_N_MASK: u16;
    static WM8350_FLL_CONTROL_3: c_uint;
    static WM8350_FLL_FRAC: u16;
    static WM8350_FLL_SLOW_LOCK_REF: u16;
    static WM8350_POWER_MGMT_1: c_uint;
    static WM8350_VMID_MASK: u16;
    static WM8350_CODEC_ISEL_MASK: u16;
    static WM8350_VMID_50K: u16;
    static SND_SOC_BIAS_ON: c_int;
    static SND_SOC_BIAS_PREPARE: c_int;
    static SND_SOC_BIAS_STANDBY: c_int;
    static SND_SOC_BIAS_OFF: c_int;
    static WM8350_SYSCLK_ENA: u16;
    static WM8350_ANTI_POP_CONTROL: c_uint;
    static WM8350_VMID_5K: u16;
    static WM8350_VMIDEN: u16;
    static WM8350_VBUFEN: u16;
    static WM8350_VMID_300K: u16;
    static WM8350_BIASEN: u16;
    static WM8350_POWER_MGMT_3: c_uint;
    static WM8350_OUT1L_ENA: u16;
    static WM8350_OUT1R_ENA: u16;
    static WM8350_OUT2L_ENA: u16;
    static WM8350_OUT2R_ENA: u16;
    static WM8350_OUTPUT_DRAIN_EN: u16;
    static WM8350_JACK_PIN_STATUS: c_uint;
    static WM8350_JACK_L_LVL: u16;
    static WM8350_JACK_R_LVL: u16;
    static WM8350_JDL: c_int;
    static WM8350_JDR: c_int;
    static WM8350_JDL_ENA: u16;
    static WM8350_JDR_ENA: u16;
    static WM8350_TOCLK_ENA: u16;
    static WM8350_JACK_DETECT: c_uint;
    static WM8350_JACK_MICSCD_LVL: u16;
    static WM8350_JACK_MICSD_LVL: u16;
    static WM8350_MIC_DET_ENA: u16;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static WM8350_POWER_MGMT_5: c_uint;
    static WM8350_CODEC_ENA: u16;
    static WM8350_SECURITY: c_uint;
    static WM8350_OUT1L_MUTE: u16;
    static WM8350_OUT2L_MUTE: u16;
    static WM8350_OUT1R_MUTE: u16;
    static WM8350_OUT2R_MUTE: u16;
    static WM8350_AIF_TRI: u16;
    static WM8350_ADC_DAC_COMP: c_uint;
    static WM8350_DAC_COMP: u16;
    static WM8350_LOOPBACK: u16;
    static WM8350_IRQ_CODEC_JCK_DET_L: c_int;
    static WM8350_IRQ_CODEC_JCK_DET_R: c_int;
    static WM8350_IRQ_CODEC_MICSCD: c_int;
    static WM8350_IRQ_CODEC_MICD: c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
