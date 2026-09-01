// SPDX-License-Identifier: GPL-2.0
//
// mt6359-accdet.c  --  ALSA SoC mt6359 accdet driver
//
// Copyright (C) 2021 MediaTek Inc.
// Author: Argus Lin <argus.lin@mediatek.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* C include dependencies removed:
 * linux/cleanup.h, linux/of.h, linux/input.h, linux/kthread.h, linux/io.h,
 * linux/sched/clock.h, linux/workqueue.h, linux/timer.h, linux/delay.h,
 * linux/module.h, linux/platform_device.h, linux/init.h, linux/irqdomain.h,
 * linux/irq.h, linux/regmap.h, sound/soc.h, sound/jack.h,
 * linux/mfd/mt6397/core.h, mt6359-accdet.h, mt6359.h
 */

type bool_ = bool;
type irqreturn_t = c_uint;

extern "C" {
    static mut mt6359_accdet_soc_driver: snd_soc_component_driver;

    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_read_poll_timeout(
        map: *mut regmap,
        addr: c_uint,
        val: c_uint,
        cond: bool_,
        sleep_us: c_uint,
        timeout_us: c_uint,
    ) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn mdelay(msecs: c_uint);
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, report: c_int, mask: c_int);
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_uint,
    ) -> c_int;
    fn of_property_read_u32_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut c_int,
        sz: usize,
    ) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool_;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *mut device) -> *const c_char;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct mt6397_chip {
    pub regmap: *mut regmap,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

/* External data layout comes from mt6359-accdet.h and mt6359.h. */
#[repr(C)]
pub struct pwm_deb_settings {
    pub debounce0: c_uint,
    pub debounce1: c_uint,
    pub debounce2: c_uint,
    pub debounce3: c_uint,
    pub debounce4: c_uint,
    pub pwm_width: c_uint,
    pub pwm_thresh: c_uint,
    pub fall_delay: c_uint,
    pub rise_delay: c_uint,
    pub eint_pwm_width: c_uint,
    pub eint_pwm_thresh: c_uint,
    pub eint_debounce0: c_uint,
    pub eint_debounce1: c_uint,
    pub eint_debounce2: c_uint,
    pub eint_debounce3: c_uint,
    pub eint_inverter_debounce: c_uint,
}
#[repr(C)]
pub struct three_key_threshold {
    pub mid: c_uint,
    pub up: c_uint,
    pub down: c_uint,
}
#[repr(C)]
pub struct four_key_threshold {
    pub mid: c_uint,
    pub voice: c_uint,
    pub up: c_uint,
    pub down: c_uint,
}
#[repr(C)]
pub struct dts_data {
    pub mic_vol: c_uint,
    pub plugout_deb: c_uint,
    pub mic_mode: c_uint,
    pub pwm_deb: *mut pwm_deb_settings,
    pub eint_pol: c_uint,
    pub eint_detect_mode: c_uint,
    pub eint_use_ext_res: c_uint,
    pub eint_comp_vth: c_uint,
    pub three_key: three_key_threshold,
    pub four_key: four_key_threshold,
}
#[repr(C)]
pub struct mt6359_accdet {
    pub data: *mut dts_data,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub caps: c_uint,
    pub jd_sts: c_uint,
    pub jack_type: c_int,
    pub btn_type: c_int,
    pub accdet_status: c_uint,
    pub pre_accdet_status: c_uint,
    pub jack_plugged: bool_,
    pub cali_voltage: c_uint,
    pub jack: *mut snd_soc_jack,
    pub res_lock: mutex,
    pub accdet_irq: c_int,
    pub accdet_eint0: c_int,
    pub accdet_eint1: c_int,
    pub accdet_workqueue: *mut workqueue_struct,
    pub jd_workqueue: *mut workqueue_struct,
    pub accdet_work: work_struct,
    pub jd_work: work_struct,
}

fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

/* global variable definitions */
fn REGISTER_VAL(x: c_uint) -> c_uint {
    x.wrapping_sub(1)
}

/* mt6359 accdet capability */
const ACCDET_PMIC_EINT_IRQ: c_uint = 1 << 0;
const ACCDET_AP_GPIO_EINT: c_uint = 1 << 1;
const ACCDET_PMIC_EINT0: c_uint = 1 << 2;
const ACCDET_PMIC_EINT1: c_uint = 1 << 3;
const ACCDET_PMIC_BI_EINT: c_uint = 1 << 4;
const ACCDET_PMIC_GPIO_TRIG_EINT: c_uint = 1 << 5;
const ACCDET_PMIC_INVERTER_TRIG_EINT: c_uint = 1 << 6;
const ACCDET_PMIC_RSV_EINT: c_uint = 1 << 7;
const ACCDET_THREE_KEY: c_uint = 1 << 8;
const ACCDET_FOUR_KEY: c_uint = 1 << 9;
const ACCDET_TRI_KEY_CDD: c_uint = 1 << 10;
const ACCDET_RSV_KEY: c_uint = 1 << 11;
const ACCDET_ANALOG_FASTDISCHARGE: c_uint = 1 << 12;
const ACCDET_DIGITAL_FASTDISCHARGE: c_uint = 1 << 13;
const ACCDET_AD_FASTDISCHRAGE: c_uint = 1 << 14;

unsafe fn adjust_eint_analog_setting(priv_: *mut mt6359_accdet) -> c_uint {
    if (*(*priv_).data).eint_detect_mode == 0x3 || (*(*priv_).data).eint_detect_mode == 0x4 {
        /* ESD switches off */
        regmap_update_bits((*priv_).regmap, RG_ACCDETSPARE_ADDR, 1 << 8, 0);
    }
    if (*(*priv_).data).eint_detect_mode == 0x4 {
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            /* enable RG_EINT0CONFIGACCDET */
            regmap_update_bits(
                (*priv_).regmap,
                RG_EINT0CONFIGACCDET_ADDR,
                RG_EINT0CONFIGACCDET_MASK_SFT,
                BIT(RG_EINT0CONFIGACCDET_SFT),
            );
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            /* enable RG_EINT1CONFIGACCDET */
            regmap_update_bits(
                (*priv_).regmap,
                RG_EINT1CONFIGACCDET_ADDR,
                RG_EINT1CONFIGACCDET_MASK_SFT,
                BIT(RG_EINT1CONFIGACCDET_SFT),
            );
        }
        if (*(*priv_).data).eint_use_ext_res == 0x3 || (*(*priv_).data).eint_use_ext_res == 0x4 {
            /*select 500k, use internal resistor */
            regmap_update_bits(
                (*priv_).regmap,
                RG_EINT0HIRENB_ADDR,
                RG_EINT0HIRENB_MASK_SFT,
                BIT(RG_EINT0HIRENB_SFT),
            );
        }
    }
    0
}

unsafe fn adjust_eint_digital_setting(priv_: *mut mt6359_accdet) -> c_uint {
    if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        /* disable inverter */
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT0_INVERTER_SW_EN_ADDR,
            ACCDET_EINT0_INVERTER_SW_EN_MASK_SFT,
            0,
        );
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        /* disable inverter */
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT1_INVERTER_SW_EN_ADDR,
            ACCDET_EINT1_INVERTER_SW_EN_MASK_SFT,
            0,
        );
    }

    if (*(*priv_).data).eint_detect_mode == 0x4 {
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            /* set DA stable signal */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_DA_STABLE_ADDR,
                ACCDET_EINT0_CEN_STABLE_MASK_SFT,
                0,
            );
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            /* set DA stable signal */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_DA_STABLE_ADDR,
                ACCDET_EINT1_CEN_STABLE_MASK_SFT,
                0,
            );
        }
    }
    0
}

unsafe fn mt6359_accdet_jd_setting(priv_: *mut mt6359_accdet) -> c_uint {
    if (*priv_).jd_sts == M_PLUG_IN {
        /* adjust digital setting */
        adjust_eint_digital_setting(priv_);
        /* adjust analog setting */
        adjust_eint_analog_setting(priv_);
    } else if (*priv_).jd_sts == M_PLUG_OUT {
        /* set debounce to 1ms */
        accdet_set_debounce(
            priv_,
            eint_state000,
            (*(*(*priv_).data).pwm_deb).eint_debounce0,
        );
    } else {
        dev_dbg((*priv_).dev, c"should not be here %s()\n".as_ptr(), c"mt6359_accdet_jd_setting".as_ptr());
    }

    0
}

unsafe fn recover_eint_analog_setting(priv_: *mut mt6359_accdet) {
    if (*(*priv_).data).eint_detect_mode == 0x3 || (*(*priv_).data).eint_detect_mode == 0x4 {
        /* ESD switches on */
        regmap_update_bits((*priv_).regmap, RG_ACCDETSPARE_ADDR, 1 << 8, 1 << 8);
    }
    if (*(*priv_).data).eint_detect_mode == 0x4 {
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            /* disable RG_EINT0CONFIGACCDET */
            regmap_update_bits(
                (*priv_).regmap,
                RG_EINT0CONFIGACCDET_ADDR,
                RG_EINT0CONFIGACCDET_MASK_SFT,
                0,
            );
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            /* disable RG_EINT1CONFIGACCDET */
            regmap_update_bits(
                (*priv_).regmap,
                RG_EINT1CONFIGACCDET_ADDR,
                RG_EINT1CONFIGACCDET_MASK_SFT,
                0,
            );
        }
        regmap_update_bits((*priv_).regmap, RG_EINT0HIRENB_ADDR, RG_EINT0HIRENB_MASK_SFT, 0);
    }
}

unsafe fn recover_eint_digital_setting(priv_: *mut mt6359_accdet) {
    if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT0_M_SW_EN_ADDR,
            ACCDET_EINT0_M_SW_EN_MASK_SFT,
            0,
        );
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT1_M_SW_EN_ADDR,
            ACCDET_EINT1_M_SW_EN_MASK_SFT,
            0,
        );
    }
    if (*(*priv_).data).eint_detect_mode == 0x4 {
        /* enable eint0cen */
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            /* enable eint0cen */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_DA_STABLE_ADDR,
                ACCDET_EINT0_CEN_STABLE_MASK_SFT,
                BIT(ACCDET_EINT0_CEN_STABLE_SFT),
            );
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            /* enable eint1cen */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_DA_STABLE_ADDR,
                ACCDET_EINT1_CEN_STABLE_MASK_SFT,
                BIT(ACCDET_EINT1_CEN_STABLE_SFT),
            );
        }
    }

    if (*(*priv_).data).eint_detect_mode != 0x1 {
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            /* enable inverter */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT0_INVERTER_SW_EN_ADDR,
                ACCDET_EINT0_INVERTER_SW_EN_MASK_SFT,
                BIT(ACCDET_EINT0_INVERTER_SW_EN_SFT),
            );
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            /* enable inverter */
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT1_INVERTER_SW_EN_ADDR,
                ACCDET_EINT1_INVERTER_SW_EN_MASK_SFT,
                BIT(ACCDET_EINT1_INVERTER_SW_EN_SFT),
            );
        }
    }
}

unsafe fn recover_eint_setting(priv_: *mut mt6359_accdet) {
    if (*priv_).jd_sts == M_PLUG_OUT {
        recover_eint_analog_setting(priv_);
        recover_eint_digital_setting(priv_);
    }
}

unsafe fn mt6359_accdet_recover_jd_setting(priv_: *mut mt6359_accdet) {
    let mut value: c_uint = 0;

    regmap_update_bits(
        (*priv_).regmap,
        ACCDET_IRQ_ADDR,
        ACCDET_IRQ_CLR_MASK_SFT,
        BIT(ACCDET_IRQ_CLR_SFT),
    );
    usleep_range(200, 300);
    let ret = regmap_read_poll_timeout(
        (*priv_).regmap,
        ACCDET_IRQ_ADDR,
        value,
        (value & ACCDET_IRQ_MASK_SFT) == 0,
        0,
        1000,
    );
    if ret != 0 {
        dev_warn((*priv_).dev, c"%s(), ret %d\n".as_ptr(), c"mt6359_accdet_recover_jd_setting".as_ptr(), ret);
    }
    /* clear accdet int, modify  for fix interrupt trigger twice error */
    regmap_update_bits((*priv_).regmap, ACCDET_IRQ_ADDR, ACCDET_IRQ_CLR_MASK_SFT, 0);
    regmap_update_bits(
        (*priv_).regmap,
        RG_INT_STATUS_ACCDET_ADDR,
        RG_INT_STATUS_ACCDET_MASK_SFT,
        BIT(RG_INT_STATUS_ACCDET_SFT),
    );

    /* recover accdet debounce0,3 */
    accdet_set_debounce(priv_, accdet_state000, (*(*(*priv_).data).pwm_deb).debounce0);
    accdet_set_debounce(priv_, accdet_state001, (*(*(*priv_).data).pwm_deb).debounce1);
    accdet_set_debounce(priv_, accdet_state011, (*(*(*priv_).data).pwm_deb).debounce3);

    (*priv_).jack_type = 0;
    (*priv_).btn_type = 0;
    (*priv_).accdet_status = 0x3;
    mt6359_accdet_jack_report(priv_);
}

unsafe fn accdet_set_debounce(priv_: *mut mt6359_accdet, state: c_int, debounce: c_uint) {
    match state {
        x if x == accdet_state000 => {
            regmap_write((*priv_).regmap, ACCDET_DEBOUNCE0_ADDR, debounce);
        }
        x if x == accdet_state001 => {
            regmap_write((*priv_).regmap, ACCDET_DEBOUNCE1_ADDR, debounce);
        }
        x if x == accdet_state010 => {
            regmap_write((*priv_).regmap, ACCDET_DEBOUNCE2_ADDR, debounce);
        }
        x if x == accdet_state011 => {
            regmap_write((*priv_).regmap, ACCDET_DEBOUNCE3_ADDR, debounce);
        }
        x if x == accdet_auxadc => {
            regmap_write((*priv_).regmap, ACCDET_CONNECT_AUXADC_TIME_DIG_ADDR, debounce);
        }
        x if x == eint_state000 => {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT_DEBOUNCE0_ADDR,
                0xF << ACCDET_EINT_DEBOUNCE0_SFT,
                debounce << ACCDET_EINT_DEBOUNCE0_SFT,
            );
        }
        x if x == eint_state001 => {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT_DEBOUNCE1_ADDR,
                0xF << ACCDET_EINT_DEBOUNCE1_SFT,
                debounce << ACCDET_EINT_DEBOUNCE1_SFT,
            );
        }
        x if x == eint_state010 => {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT_DEBOUNCE2_ADDR,
                0xF << ACCDET_EINT_DEBOUNCE2_SFT,
                debounce << ACCDET_EINT_DEBOUNCE2_SFT,
            );
        }
        x if x == eint_state011 => {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_EINT_DEBOUNCE3_ADDR,
                0xF << ACCDET_EINT_DEBOUNCE3_SFT,
                debounce << ACCDET_EINT_DEBOUNCE3_SFT,
            );
        }
        x if x == eint_inverter_state000 => {
            regmap_write((*priv_).regmap, ACCDET_EINT_INVERTER_DEBOUNCE_ADDR, debounce);
        }
        _ => {
            dev_warn((*priv_).dev, c"Error: %s error state (%d)\n".as_ptr(), c"accdet_set_debounce".as_ptr(), state);
        }
    }
}

unsafe fn mt6359_accdet_jack_report(priv_: *mut mt6359_accdet) {
    let report: c_int;

    if (*priv_).jack.is_null() {
        return;
    }

    report = (*priv_).jack_type | (*priv_).btn_type;
    snd_soc_jack_report((*priv_).jack, report, MT6359_ACCDET_JACK_MASK);
}

unsafe fn check_button(priv_: *mut mt6359_accdet, v: c_uint) -> c_uint {
    if (*priv_).caps & ACCDET_FOUR_KEY != 0 {
        if v < (*(*priv_).data).four_key.down && v >= (*(*priv_).data).four_key.up {
            (*priv_).btn_type = SND_JACK_BTN_1;
        }
        if v < (*(*priv_).data).four_key.up && v >= (*(*priv_).data).four_key.voice {
            (*priv_).btn_type = SND_JACK_BTN_2;
        }
        if v < (*(*priv_).data).four_key.voice && v >= (*(*priv_).data).four_key.mid {
            (*priv_).btn_type = SND_JACK_BTN_3;
        }
        if v < (*(*priv_).data).four_key.mid {
            (*priv_).btn_type = SND_JACK_BTN_0;
        }
    } else {
        if v < (*(*priv_).data).three_key.down && v >= (*(*priv_).data).three_key.up {
            (*priv_).btn_type = SND_JACK_BTN_1;
        }
        if v < (*(*priv_).data).three_key.up && v >= (*(*priv_).data).three_key.mid {
            (*priv_).btn_type = SND_JACK_BTN_2;
        }
        if v < (*(*priv_).data).three_key.mid {
            (*priv_).btn_type = SND_JACK_BTN_0;
        }
    }
    0
}

unsafe fn is_key_pressed(priv_: *mut mt6359_accdet, pressed: bool_) {
    (*priv_).btn_type = (*priv_).jack_type & !MT6359_ACCDET_BTN_MASK;

    if pressed {
        check_button(priv_, (*priv_).cali_voltage);
    }
}

unsafe fn check_jack_btn_type(priv_: *mut mt6359_accdet) {
    let mut val: c_uint = 0;

    regmap_read((*priv_).regmap, ACCDET_MEM_IN_ADDR, &mut val);

    (*priv_).accdet_status = (val >> ACCDET_STATE_MEM_IN_OFFSET) & ACCDET_STATE_AB_MASK;

    match (*priv_).accdet_status {
        0 => {
            if (*priv_).jack_type == SND_JACK_HEADSET {
                is_key_pressed(priv_, true);
            } else {
                (*priv_).jack_type = SND_JACK_HEADPHONE;
            }
        }
        1 => {
            if (*priv_).jack_type == SND_JACK_HEADSET {
                is_key_pressed(priv_, false);
            } else {
                (*priv_).jack_type = SND_JACK_HEADSET;
                accdet_set_debounce(priv_, eint_state011, 0x1);
            }
        }
        3 | _ => {
            (*priv_).jack_type = 0;
        }
    }
}

unsafe extern "C" fn mt6359_accdet_work(work: *mut work_struct) {
    let priv_ = container_of!(work, mt6359_accdet, accdet_work);

    mutex_lock(&mut (*priv_).res_lock);
    (*priv_).pre_accdet_status = (*priv_).accdet_status;
    check_jack_btn_type(priv_);

    if (*priv_).jack_plugged && (*priv_).pre_accdet_status != (*priv_).accdet_status {
        mt6359_accdet_jack_report(priv_);
    }
    mutex_unlock(&mut (*priv_).res_lock);
}

unsafe extern "C" fn mt6359_accdet_jd_work(work: *mut work_struct) {
    let mut value: c_uint = 0;

    let priv_ = container_of!(work, mt6359_accdet, jd_work);

    mutex_lock(&mut (*priv_).res_lock);
    if (*priv_).jd_sts == M_PLUG_IN {
        (*priv_).jack_plugged = true;

        /* set and clear initial bit every eint interrupt */
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_SEQ_INIT_ADDR,
            ACCDET_SEQ_INIT_MASK_SFT,
            BIT(ACCDET_SEQ_INIT_SFT),
        );
        regmap_update_bits((*priv_).regmap, ACCDET_SEQ_INIT_ADDR, ACCDET_SEQ_INIT_MASK_SFT, 0);
        let ret = regmap_read_poll_timeout(
            (*priv_).regmap,
            ACCDET_SEQ_INIT_ADDR,
            value,
            (value & ACCDET_SEQ_INIT_MASK_SFT) == 0,
            0,
            1000,
        );
        if ret != 0 {
            dev_err((*priv_).dev, c"%s(), ret %d\n".as_ptr(), c"mt6359_accdet_jd_work".as_ptr(), ret);
        }

        /* enable ACCDET unit */
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_SW_EN_ADDR,
            ACCDET_SW_EN_MASK_SFT,
            BIT(ACCDET_SW_EN_SFT),
        );
    } else if (*priv_).jd_sts == M_PLUG_OUT {
        (*priv_).jack_plugged = false;

        accdet_set_debounce(priv_, accdet_state011, (*(*(*priv_).data).pwm_deb).debounce3);
        regmap_update_bits((*priv_).regmap, ACCDET_SW_EN_ADDR, ACCDET_SW_EN_MASK_SFT, 0);
        mt6359_accdet_recover_jd_setting(priv_);
    }

    if (*priv_).caps & ACCDET_PMIC_EINT_IRQ != 0 {
        recover_eint_setting(priv_);
    }
    mutex_unlock(&mut (*priv_).res_lock);
}

unsafe extern "C" fn mt6359_accdet_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut mt6359_accdet;
    let mut irq_val: c_uint = 0;
    let mut val: c_uint = 0;
    let mut value: c_uint = 0;

    mutex_lock(&mut (*priv_).res_lock);
    regmap_read((*priv_).regmap, ACCDET_IRQ_ADDR, &mut irq_val);

    if irq_val & ACCDET_IRQ_MASK_SFT != 0 {
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_IRQ_ADDR,
            ACCDET_IRQ_CLR_MASK_SFT,
            BIT(ACCDET_IRQ_CLR_SFT),
        );
        let ret = regmap_read_poll_timeout(
            (*priv_).regmap,
            ACCDET_IRQ_ADDR,
            value,
            (value & ACCDET_IRQ_MASK_SFT) == 0,
            0,
            1000,
        );
        if ret != 0 {
            dev_err((*priv_).dev, c"%s(), ret %d\n".as_ptr(), c"mt6359_accdet_irq".as_ptr(), ret);
            mutex_unlock(&mut (*priv_).res_lock);
            return IRQ_NONE;
        }
        regmap_update_bits((*priv_).regmap, ACCDET_IRQ_ADDR, ACCDET_IRQ_CLR_MASK_SFT, 0);
        regmap_update_bits(
            (*priv_).regmap,
            RG_INT_STATUS_ACCDET_ADDR,
            RG_INT_STATUS_ACCDET_MASK_SFT,
            BIT(RG_INT_STATUS_ACCDET_SFT),
        );

        queue_work((*priv_).accdet_workqueue, &mut (*priv_).accdet_work);
    } else {
        if irq_val & ACCDET_EINT0_IRQ_MASK_SFT != 0 {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_IRQ_ADDR,
                ACCDET_EINT0_IRQ_CLR_MASK_SFT,
                BIT(ACCDET_EINT0_IRQ_CLR_SFT),
            );
            let ret = regmap_read_poll_timeout(
                (*priv_).regmap,
                ACCDET_IRQ_ADDR,
                value,
                (value & ACCDET_EINT0_IRQ_MASK_SFT) == 0,
                0,
                1000,
            );
            if ret != 0 {
                dev_err((*priv_).dev, c"%s(), ret %d\n".as_ptr(), c"mt6359_accdet_irq".as_ptr(), ret);
                mutex_unlock(&mut (*priv_).res_lock);
                return IRQ_NONE;
            }
            regmap_update_bits((*priv_).regmap, ACCDET_IRQ_ADDR, ACCDET_EINT0_IRQ_CLR_MASK_SFT, 0);
            regmap_update_bits(
                (*priv_).regmap,
                RG_INT_STATUS_ACCDET_ADDR,
                RG_INT_STATUS_ACCDET_EINT0_MASK_SFT,
                BIT(RG_INT_STATUS_ACCDET_EINT0_SFT),
            );
        }
        if irq_val & ACCDET_EINT1_IRQ_MASK_SFT != 0 {
            regmap_update_bits(
                (*priv_).regmap,
                ACCDET_IRQ_ADDR,
                ACCDET_EINT1_IRQ_CLR_MASK_SFT,
                BIT(ACCDET_EINT1_IRQ_CLR_SFT),
            );
            let ret = regmap_read_poll_timeout(
                (*priv_).regmap,
                ACCDET_IRQ_ADDR,
                value,
                (value & ACCDET_EINT1_IRQ_MASK_SFT) == 0,
                0,
                1000,
            );
            if ret != 0 {
                dev_err((*priv_).dev, c"%s(), ret %d\n".as_ptr(), c"mt6359_accdet_irq".as_ptr(), ret);
                mutex_unlock(&mut (*priv_).res_lock);
                return IRQ_NONE;
            }
            regmap_update_bits((*priv_).regmap, ACCDET_IRQ_ADDR, ACCDET_EINT1_IRQ_CLR_MASK_SFT, 0);
            regmap_update_bits(
                (*priv_).regmap,
                RG_INT_STATUS_ACCDET_ADDR,
                RG_INT_STATUS_ACCDET_EINT1_MASK_SFT,
                BIT(RG_INT_STATUS_ACCDET_EINT1_SFT),
            );
        }
        /* get jack detection status */
        regmap_read((*priv_).regmap, ACCDET_EINT0_MEM_IN_ADDR, &mut val);
        (*priv_).jd_sts = (val >> ACCDET_EINT0_MEM_IN_SFT) & ACCDET_EINT0_MEM_IN_MASK;
        /* adjust eint digital/analog setting */
        mt6359_accdet_jd_setting(priv_);

        queue_work((*priv_).jd_workqueue, &mut (*priv_).jd_work);
    }

    mutex_unlock(&mut (*priv_).res_lock);
    IRQ_HANDLED
}

unsafe fn mt6359_accdet_parse_dt(priv_: *mut mt6359_accdet) -> c_int {
    let mut ret: c_int;
    let dev = (*priv_).dev;
    let mut node: *mut device_node = ptr::null_mut();
    let mut pwm_deb: [c_int; 15] = [0; 15];
    let mut tmp: c_uint = 0;

    node = of_get_child_by_name((*(*dev).parent).of_node, c"accdet".as_ptr());
    if node.is_null() {
        return -EINVAL;
    }

    ret = of_property_read_u32(node, c"mediatek,mic-vol".as_ptr(), &mut (*(*priv_).data).mic_vol);
    if ret != 0 {
        (*(*priv_).data).mic_vol = 8;
    }

    ret = of_property_read_u32(node, c"mediatek,plugout-debounce".as_ptr(), &mut (*(*priv_).data).plugout_deb);
    if ret != 0 {
        (*(*priv_).data).plugout_deb = 1;
    }

    ret = of_property_read_u32(node, c"mediatek,mic-mode".as_ptr(), &mut (*(*priv_).data).mic_mode);
    if ret != 0 {
        (*(*priv_).data).mic_mode = 2;
    }

    ret = of_property_read_u32_array(node, c"mediatek,pwm-deb-setting".as_ptr(), pwm_deb.as_mut_ptr(), pwm_deb.len());
    /* debounce8(auxadc debounce) is default, needn't get from dts */
    if ret == 0 {
        ptr::copy_nonoverlapping(
            pwm_deb.as_ptr() as *const pwm_deb_settings,
            (*(*priv_).data).pwm_deb,
            1,
        );
    }

    ret = of_property_read_u32(node, c"mediatek,eint-level-pol".as_ptr(), &mut (*(*priv_).data).eint_pol);
    if ret != 0 {
        (*(*priv_).data).eint_pol = 8;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-use-ap".as_ptr(), &mut tmp);
    if ret != 0 {
        tmp = 0;
    }
    if tmp == 0 {
        (*priv_).caps |= ACCDET_PMIC_EINT_IRQ;
    } else if tmp == 1 {
        (*priv_).caps |= ACCDET_AP_GPIO_EINT;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-detect-mode".as_ptr(), &mut (*(*priv_).data).eint_detect_mode);
    if ret != 0 {
        /* eint detection mode equals to EINT HW Mode */
        (*(*priv_).data).eint_detect_mode = 0x4;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-num".as_ptr(), &mut tmp);
    if ret != 0 {
        tmp = 0;
    }
    if tmp == 0 {
        (*priv_).caps |= ACCDET_PMIC_EINT0;
    } else if tmp == 1 {
        (*priv_).caps |= ACCDET_PMIC_EINT1;
    } else if tmp == 2 {
        (*priv_).caps |= ACCDET_PMIC_BI_EINT;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-trig-mode".as_ptr(), &mut tmp);
    if ret != 0 {
        tmp = 0;
    }
    if tmp == 0 {
        (*priv_).caps |= ACCDET_PMIC_GPIO_TRIG_EINT;
    } else if tmp == 1 {
        (*priv_).caps |= ACCDET_PMIC_INVERTER_TRIG_EINT;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-use-ext-res".as_ptr(), &mut (*(*priv_).data).eint_use_ext_res);
    if ret != 0 {
        /* eint use internal resister */
        (*(*priv_).data).eint_use_ext_res = 0x0;
    }

    ret = of_property_read_u32(node, c"mediatek,eint-comp-vth".as_ptr(), &mut (*(*priv_).data).eint_comp_vth);
    if ret != 0 {
        (*(*priv_).data).eint_comp_vth = 0x0;
    }

    ret = of_property_read_u32(node, c"mediatek,key-mode".as_ptr(), &mut tmp);
    if ret != 0 {
        tmp = 0;
    }
    if tmp == 0 {
        let mut three_key: [c_int; 4] = [0; 4];

        (*priv_).caps |= ACCDET_THREE_KEY;
        ret = of_property_read_u32_array(node, c"mediatek,three-key-thr".as_ptr(), three_key.as_mut_ptr(), three_key.len());
        if ret == 0 {
            ptr::copy_nonoverlapping(
                three_key.as_ptr().add(1) as *const three_key_threshold,
                &mut (*(*priv_).data).three_key,
                1,
            );
        }
    } else if tmp == 1 {
        let mut four_key: [c_int; 5] = [0; 5];

        (*priv_).caps |= ACCDET_FOUR_KEY;
        ret = of_property_read_u32_array(node, c"mediatek,four-key-thr".as_ptr(), four_key.as_mut_ptr(), four_key.len());
        if ret == 0 {
            ptr::copy_nonoverlapping(
                four_key.as_ptr().add(1) as *const four_key_threshold,
                &mut (*(*priv_).data).four_key,
                1,
            );
        } else {
            dev_warn((*priv_).dev, c"accdet no 4-key-thrsh dts, use efuse\n".as_ptr());
        }
    } else if tmp == 2 {
        let mut three_key: [c_int; 4] = [0; 4];

        (*priv_).caps |= ACCDET_TRI_KEY_CDD;
        ret = of_property_read_u32_array(node, c"mediatek,tri-key-cdd-thr".as_ptr(), three_key.as_mut_ptr(), three_key.len());
        if ret == 0 {
            ptr::copy_nonoverlapping(
                three_key.as_ptr().add(1) as *const three_key_threshold,
                &mut (*(*priv_).data).three_key,
                1,
            );
        }
    }

    of_node_put(node);
    dev_warn((*priv_).dev, c"accdet caps=%x\n".as_ptr(), (*priv_).caps);

    0
}

unsafe fn config_digital_init_by_mode(priv_: *mut mt6359_accdet) {
    /* enable eint cmpmem pwm */
    regmap_write(
        (*priv_).regmap,
        ACCDET_EINT_CMPMEN_PWM_THRESH_ADDR,
        ((*(*(*priv_).data).pwm_deb).eint_pwm_width << 4) | (*(*(*priv_).data).pwm_deb).eint_pwm_thresh,
    );
    /* DA signal stable */
    if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        regmap_write((*priv_).regmap, ACCDET_DA_STABLE_ADDR, ACCDET_EINT0_STABLE_VAL);
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        regmap_write((*priv_).regmap, ACCDET_DA_STABLE_ADDR, ACCDET_EINT1_STABLE_VAL);
    }
    /* after receive n+1 number, interrupt issued. */
    regmap_update_bits(
        (*priv_).regmap,
        ACCDET_EINT_M_PLUG_IN_NUM_ADDR,
        ACCDET_EINT_M_PLUG_IN_NUM_MASK_SFT,
        BIT(ACCDET_EINT_M_PLUG_IN_NUM_SFT),
    );
    /* setting HW mode, enable digital fast discharge
     * if use EINT0 & EINT1 detection, please modify
     * ACCDET_HWMODE_EN_ADDR[2:1]
     */
    regmap_write((*priv_).regmap, ACCDET_HWMODE_EN_ADDR, 0x100);

    regmap_update_bits(
        (*priv_).regmap,
        ACCDET_EINT_M_DETECT_EN_ADDR,
        ACCDET_EINT_M_DETECT_EN_MASK_SFT,
        0,
    );

    /* enable PWM */
    regmap_write((*priv_).regmap, ACCDET_CMP_PWM_EN_ADDR, 0x67);
    /* enable inverter detection */
    if (*(*priv_).data).eint_detect_mode == 0x1 {
        /* disable inverter detection */
        if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            regmap_update_bits((*priv_).regmap, ACCDET_EINT0_INVERTER_SW_EN_ADDR, ACCDET_EINT0_INVERTER_SW_EN_MASK_SFT, 0);
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            regmap_update_bits((*priv_).regmap, ACCDET_EINT1_INVERTER_SW_EN_ADDR, ACCDET_EINT1_INVERTER_SW_EN_MASK_SFT, 0);
        }
    } else if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT0_INVERTER_SW_EN_ADDR,
            ACCDET_EINT0_INVERTER_SW_EN_MASK_SFT,
            BIT(ACCDET_EINT0_INVERTER_SW_EN_SFT),
        );
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        regmap_update_bits(
            (*priv_).regmap,
            ACCDET_EINT1_INVERTER_SW_EN_ADDR,
            ACCDET_EINT1_INVERTER_SW_EN_MASK_SFT,
            BIT(ACCDET_EINT1_INVERTER_SW_EN_SFT),
        );
    }
}

unsafe fn config_eint_init_by_mode(priv_: *mut mt6359_accdet) {
    let val: c_uint = 0;

    if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        regmap_update_bits((*priv_).regmap, RG_EINT0EN_ADDR, RG_EINT0EN_MASK_SFT, BIT(RG_EINT0EN_SFT));
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        regmap_update_bits((*priv_).regmap, RG_EINT1EN_ADDR, RG_EINT1EN_MASK_SFT, BIT(RG_EINT1EN_SFT));
    }
    /* ESD switches on */
    regmap_update_bits((*priv_).regmap, RG_ACCDETSPARE_ADDR, 1 << 8, 1 << 8);
    /* before playback, set NCP pull low before nagative voltage */
    regmap_update_bits((*priv_).regmap, RG_NCP_PDDIS_EN_ADDR, RG_NCP_PDDIS_EN_MASK_SFT, BIT(RG_NCP_PDDIS_EN_SFT));

    if (*(*priv_).data).eint_detect_mode == 0x1
        || (*(*priv_).data).eint_detect_mode == 0x2
        || (*(*priv_).data).eint_detect_mode == 0x3
    {
        if (*(*priv_).data).eint_use_ext_res == 0x1 {
            if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
                regmap_update_bits((*priv_).regmap, RG_EINT0CONFIGACCDET_ADDR, RG_EINT0CONFIGACCDET_MASK_SFT, 0);
            } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
                regmap_update_bits((*priv_).regmap, RG_EINT1CONFIGACCDET_ADDR, RG_EINT1CONFIGACCDET_MASK_SFT, 0);
            }
        } else if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
            regmap_update_bits((*priv_).regmap, RG_EINT0CONFIGACCDET_ADDR, RG_EINT0CONFIGACCDET_MASK_SFT, BIT(RG_EINT0CONFIGACCDET_SFT));
        } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
            regmap_update_bits((*priv_).regmap, RG_EINT1CONFIGACCDET_ADDR, RG_EINT1CONFIGACCDET_MASK_SFT, BIT(RG_EINT1CONFIGACCDET_SFT));
        }
    }

    if (*(*priv_).data).eint_detect_mode != 0x1 {
        /* current detect set 0.25uA */
        regmap_update_bits((*priv_).regmap, RG_ACCDETSPARE_ADDR, 0x3 << RG_ACCDETSPARE_SFT, 0x3 << RG_ACCDETSPARE_SFT);
    }
    regmap_write((*priv_).regmap, RG_EINTCOMPVTH_ADDR, val | ((*(*priv_).data).eint_comp_vth << RG_EINTCOMPVTH_SFT));
}

unsafe fn mt6359_accdet_init(priv_: *mut mt6359_accdet) {
    let mut reg: c_uint = 0;

    regmap_update_bits((*priv_).regmap, ACCDET_SEQ_INIT_ADDR, ACCDET_SEQ_INIT_MASK_SFT, BIT(ACCDET_SEQ_INIT_SFT));
    mdelay(2);
    regmap_update_bits((*priv_).regmap, ACCDET_SEQ_INIT_ADDR, ACCDET_SEQ_INIT_MASK_SFT, 0);
    mdelay(1);
    /* init the debounce time (debounce/32768)sec */
    accdet_set_debounce(priv_, accdet_state000, (*(*(*priv_).data).pwm_deb).debounce0);
    accdet_set_debounce(priv_, accdet_state001, (*(*(*priv_).data).pwm_deb).debounce1);
    accdet_set_debounce(priv_, accdet_state011, (*(*(*priv_).data).pwm_deb).debounce3);
    accdet_set_debounce(priv_, accdet_auxadc, (*(*(*priv_).data).pwm_deb).debounce4);

    accdet_set_debounce(priv_, eint_state000, (*(*(*priv_).data).pwm_deb).eint_debounce0);
    accdet_set_debounce(priv_, eint_state001, (*(*(*priv_).data).pwm_deb).eint_debounce1);
    accdet_set_debounce(priv_, eint_state011, (*(*(*priv_).data).pwm_deb).eint_debounce3);
    accdet_set_debounce(priv_, eint_inverter_state000, (*(*(*priv_).data).pwm_deb).eint_inverter_debounce);

    regmap_update_bits((*priv_).regmap, RG_ACCDET_RST_ADDR, RG_ACCDET_RST_MASK_SFT, BIT(RG_ACCDET_RST_SFT));
    regmap_update_bits((*priv_).regmap, RG_ACCDET_RST_ADDR, RG_ACCDET_RST_MASK_SFT, 0);

    /* clear high micbias1 voltage setting */
    regmap_update_bits((*priv_).regmap, RG_AUDPWDBMICBIAS1_ADDR, 0x3 << RG_AUDMICBIAS1HVEN_SFT, 0);
    regmap_update_bits((*priv_).regmap, RG_AUDPWDBMICBIAS1_ADDR, 0x7 << RG_AUDMICBIAS1VREF_SFT, 0);

    /* init pwm frequency, duty & rise/falling delay */
    regmap_write((*priv_).regmap, ACCDET_PWM_WIDTH_ADDR, REGISTER_VAL((*(*(*priv_).data).pwm_deb).pwm_width));
    regmap_write((*priv_).regmap, ACCDET_PWM_THRESH_ADDR, REGISTER_VAL((*(*(*priv_).data).pwm_deb).pwm_thresh));
    regmap_write(
        (*priv_).regmap,
        ACCDET_RISE_DELAY_ADDR,
        ((*(*(*priv_).data).pwm_deb).fall_delay << 15) | (*(*(*priv_).data).pwm_deb).rise_delay,
    );

    regmap_read((*priv_).regmap, RG_AUDPWDBMICBIAS1_ADDR, &mut reg);
    if (*(*priv_).data).mic_vol <= 7 {
        /* micbias1 <= 2.7V */
        regmap_write(
            (*priv_).regmap,
            RG_AUDPWDBMICBIAS1_ADDR,
            reg | ((*(*priv_).data).mic_vol << RG_AUDMICBIAS1VREF_SFT) | RG_AUDMICBIAS1LOWPEN_MASK_SFT,
        );
    } else if (*(*priv_).data).mic_vol == 8 {
        /* micbias1 = 2.8v */
        regmap_write(
            (*priv_).regmap,
            RG_AUDPWDBMICBIAS1_ADDR,
            reg | (3 << RG_AUDMICBIAS1HVEN_SFT) | RG_AUDMICBIAS1LOWPEN_MASK_SFT,
        );
    } else if (*(*priv_).data).mic_vol == 9 {
        /* micbias1 = 2.85v */
        regmap_write(
            (*priv_).regmap,
            RG_AUDPWDBMICBIAS1_ADDR,
            reg | (1 << RG_AUDMICBIAS1HVEN_SFT) | RG_AUDMICBIAS1LOWPEN_MASK_SFT,
        );
    }
    /* mic mode setting */
    regmap_read((*priv_).regmap, RG_AUDACCDETMICBIAS0PULLLOW_ADDR, &mut reg);
    if (*(*priv_).data).mic_mode == HEADSET_MODE_1 {
        /* ACC mode*/
        regmap_write((*priv_).regmap, RG_AUDACCDETMICBIAS0PULLLOW_ADDR, reg | RG_ACCDET_MODE_ANA11_MODE1);
        /* enable analog fast discharge */
        regmap_update_bits((*priv_).regmap, RG_ANALOGFDEN_ADDR, RG_ANALOGFDEN_MASK_SFT, BIT(RG_ANALOGFDEN_SFT));
        regmap_update_bits((*priv_).regmap, RG_ACCDETSPARE_ADDR, 0x3 << 11, 0x3 << 11);
    } else if (*(*priv_).data).mic_mode == HEADSET_MODE_2 {
        /* DCC mode Low cost mode without internal bias */
        regmap_write((*priv_).regmap, RG_AUDACCDETMICBIAS0PULLLOW_ADDR, reg | RG_ACCDET_MODE_ANA11_MODE2);
        /* enable analog fast discharge */
        regmap_update_bits((*priv_).regmap, RG_ANALOGFDEN_ADDR, 0x3 << RG_ANALOGFDEN_SFT, 0x3 << RG_ANALOGFDEN_SFT);
    } else if (*(*priv_).data).mic_mode == HEADSET_MODE_6 {
        /* DCC mode Low cost mode with internal bias,
         * bit8 = 1 to use internal bias
         */
        regmap_write((*priv_).regmap, RG_AUDACCDETMICBIAS0PULLLOW_ADDR, reg | RG_ACCDET_MODE_ANA11_MODE6);
        regmap_update_bits((*priv_).regmap, RG_AUDPWDBMICBIAS1_ADDR, RG_AUDMICBIAS1DCSW1PEN_MASK_SFT, BIT(RG_AUDMICBIAS1DCSW1PEN_SFT));
        /* enable analog fast discharge */
        regmap_update_bits((*priv_).regmap, RG_ANALOGFDEN_ADDR, 0x3 << RG_ANALOGFDEN_SFT, 0x3 << RG_ANALOGFDEN_SFT);
    }

    if (*priv_).caps & ACCDET_PMIC_EINT_IRQ != 0 {
        config_eint_init_by_mode(priv_);
        config_digital_init_by_mode(priv_);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt6359_accdet_enable_jack_detect(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut mt6359_accdet;

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    (*priv_).jack = jack;

    mt6359_accdet_jack_report(priv_);

    0
}
/* EXPORT_SYMBOL_GPL(mt6359_accdet_enable_jack_detect); */

unsafe extern "C" fn mt6359_accdet_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mt6397 = dev_get_drvdata((*pdev).dev.parent) as *mut mt6397_chip;

    dev_dbg(
        &mut (*pdev).dev,
        c"%s(), dev name %s\n".as_ptr(),
        c"mt6359_accdet_probe".as_ptr(),
        dev_name(&mut (*pdev).dev),
    );

    let priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<mt6359_accdet>(), GFP_KERNEL) as *mut mt6359_accdet;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).data = devm_kzalloc(&mut (*pdev).dev, size_of::<dts_data>(), GFP_KERNEL) as *mut dts_data;
    if (*priv_).data.is_null() {
        return -ENOMEM;
    }

    (*(*priv_).data).pwm_deb =
        devm_kzalloc(&mut (*pdev).dev, size_of::<pwm_deb_settings>(), GFP_KERNEL) as *mut pwm_deb_settings;
    if (*(*priv_).data).pwm_deb.is_null() {
        return -ENOMEM;
    }

    (*priv_).regmap = (*mt6397).regmap;
    if IS_ERR((*priv_).regmap as *mut c_void) {
        ret = PTR_ERR((*priv_).regmap as *mut c_void);
        dev_err(&mut (*pdev).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }
    (*priv_).dev = &mut (*pdev).dev;

    ret = mt6359_accdet_parse_dt(priv_);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Failed to parse dts\n".as_ptr());
        return ret;
    }
    mutex_init(&mut (*priv_).res_lock);

    (*priv_).accdet_irq = platform_get_irq(pdev, 0);
    if (*priv_).accdet_irq >= 0 {
        ret = devm_request_threaded_irq(
            &mut (*pdev).dev,
            (*priv_).accdet_irq,
            None,
            Some(mt6359_accdet_irq),
            IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
            c"ACCDET_IRQ".as_ptr(),
            priv_ as *mut c_void,
        );
        if ret != 0 {
            dev_err(&mut (*pdev).dev, c"Failed to request IRQ: (%d)\n".as_ptr(), ret);
            return ret;
        }
    }

    if (*priv_).caps & ACCDET_PMIC_EINT0 != 0 {
        (*priv_).accdet_eint0 = platform_get_irq(pdev, 1);
        if (*priv_).accdet_eint0 >= 0 {
            ret = devm_request_threaded_irq(
                &mut (*pdev).dev,
                (*priv_).accdet_eint0,
                None,
                Some(mt6359_accdet_irq),
                IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
                c"ACCDET_EINT0".as_ptr(),
                priv_ as *mut c_void,
            );
            if ret != 0 {
                dev_err(&mut (*pdev).dev, c"Failed to request eint0 IRQ (%d)\n".as_ptr(), ret);
                return ret;
            }
        }
    } else if (*priv_).caps & ACCDET_PMIC_EINT1 != 0 {
        (*priv_).accdet_eint1 = platform_get_irq(pdev, 2);
        if (*priv_).accdet_eint1 >= 0 {
            ret = devm_request_threaded_irq(
                &mut (*pdev).dev,
                (*priv_).accdet_eint1,
                None,
                Some(mt6359_accdet_irq),
                IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
                c"ACCDET_EINT1".as_ptr(),
                priv_ as *mut c_void,
            );
            if ret != 0 {
                dev_err(&mut (*pdev).dev, c"Failed to request eint1 IRQ (%d)\n".as_ptr(), ret);
                return ret;
            }
        }
    }

    (*priv_).accdet_workqueue = create_singlethread_workqueue(c"accdet".as_ptr());
    INIT_WORK(&mut (*priv_).accdet_work, mt6359_accdet_work);
    if (*priv_).accdet_workqueue.is_null() {
        dev_err(&mut (*pdev).dev, c"Failed to create accdet workqueue\n".as_ptr());
        ret = -1;
        return mt6359_accdet_probe_err_accdet_wq(pdev, priv_, ret);
    }

    (*priv_).jd_workqueue = create_singlethread_workqueue(c"mt6359_accdet_jd".as_ptr());
    INIT_WORK(&mut (*priv_).jd_work, mt6359_accdet_jd_work);
    if (*priv_).jd_workqueue.is_null() {
        dev_err(&mut (*pdev).dev, c"Failed to create jack detect workqueue\n".as_ptr());
        ret = -1;
        return mt6359_accdet_probe_err_eint_wq(pdev, priv_, ret);
    }

    platform_set_drvdata(pdev, priv_ as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &mt6359_accdet_soc_driver, ptr::null_mut(), 0);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, c"Failed to register component\n".as_ptr());
        return ret;
    }

    (*priv_).jd_sts = M_PLUG_OUT;
    (*priv_).jack_type = 0;
    (*priv_).btn_type = 0;
    (*priv_).accdet_status = 0x3;
    mt6359_accdet_init(priv_);

    mt6359_accdet_jack_report(priv_);

    0
}

unsafe fn mt6359_accdet_probe_err_eint_wq(
    pdev: *mut platform_device,
    priv_: *mut mt6359_accdet,
    ret: c_int,
) -> c_int {
    destroy_workqueue((*priv_).accdet_workqueue);
    mt6359_accdet_probe_err_accdet_wq(pdev, priv_, ret)
}

unsafe fn mt6359_accdet_probe_err_accdet_wq(
    pdev: *mut platform_device,
    _priv: *mut mt6359_accdet,
    ret: c_int,
) -> c_int {
    dev_err(&mut (*pdev).dev, c"%s error. now exit.!\n".as_ptr(), c"mt6359_accdet_probe".as_ptr());
    ret
}

static mut mt6359_accdet_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"pmic-codec-accdet".as_ptr(),
    },
    probe: Some(mt6359_accdet_probe),
};

/* module_platform_driver(mt6359_accdet_driver) */

/* Module information */
/* MODULE_DESCRIPTION("MT6359 ALSA SoC codec jack driver"); */
/* MODULE_AUTHOR("Argus Lin <argus.lin@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
