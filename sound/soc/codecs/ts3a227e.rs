// SPDX-License-Identifier: GPL-2.0-only
/*
 * TS3A227E Autonomous Audio Accessory Detection and Configuration Switch
 *
 * Copyright (C) 2014 Google, Inc.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub set_jack: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            jack: *mut snd_soc_jack,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub get_jack_type: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(i2c: *mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
struct ts3a227e {
    dev: *mut device,
    regmap: *mut regmap,
    jack: *mut snd_soc_jack,
    plugged: bool,
    mic_present: bool,
    buttons_held: c_uint,
    irq: c_int,
}

unsafe extern "C" {
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_uint,
        handler: *mut c_void,
        thread_fn: Option<unsafe extern "C" fn(irq: c_int, data: *mut c_void) -> irqreturn_t>,
        irqflags: c_uint,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn disable_irq(irq: c_uint);
    fn enable_irq(irq: c_uint);
}

type irqreturn_t = c_uint;

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 2;
const IRQF_TRIGGER_LOW: c_uint = 0x00000008;
const IRQF_ONESHOT: c_uint = 0x00002000;

/* Button values to be reported on the jack */
static ts3a227e_buttons: [c_int; 4] = [
    SND_JACK_BTN_0,
    SND_JACK_BTN_1,
    SND_JACK_BTN_2,
    SND_JACK_BTN_3,
];

const TS3A227E_NUM_BUTTONS: c_uint = 4;
const TS3A227E_JACK_MASK: c_int = SND_JACK_HEADPHONE
    | SND_JACK_MICROPHONE
    | SND_JACK_BTN_0
    | SND_JACK_BTN_1
    | SND_JACK_BTN_2
    | SND_JACK_BTN_3;

/* TS3A227E registers */
const TS3A227E_REG_DEVICE_ID: c_uint = 0x00;
const TS3A227E_REG_INTERRUPT: c_uint = 0x01;
const TS3A227E_REG_KP_INTERRUPT: c_uint = 0x02;
const TS3A227E_REG_INTERRUPT_DISABLE: c_uint = 0x03;
const TS3A227E_REG_SETTING_1: c_uint = 0x04;
const TS3A227E_REG_SETTING_2: c_uint = 0x05;
const TS3A227E_REG_SETTING_3: c_uint = 0x06;
const TS3A227E_REG_SWITCH_CONTROL_1: c_uint = 0x07;
const TS3A227E_REG_SWITCH_CONTROL_2: c_uint = 0x08;
const TS3A227E_REG_SWITCH_STATUS_1: c_uint = 0x09;
const TS3A227E_REG_SWITCH_STATUS_2: c_uint = 0x0a;
const TS3A227E_REG_ACCESSORY_STATUS: c_uint = 0x0b;
const TS3A227E_REG_ADC_OUTPUT: c_uint = 0x0c;
const TS3A227E_REG_KP_THRESHOLD_1: c_uint = 0x0d;
const TS3A227E_REG_KP_THRESHOLD_2: c_uint = 0x0e;
const TS3A227E_REG_KP_THRESHOLD_3: c_uint = 0x0f;

/* TS3A227E_REG_INTERRUPT 0x01 */
const INS_REM_EVENT: c_uint = 0x01;
const DETECTION_COMPLETE_EVENT: c_uint = 0x02;

/* TS3A227E_REG_KP_INTERRUPT 0x02 */
const fn PRESS_MASK(idx: c_uint) -> c_uint {
    0x01 << (2 * idx)
}
const fn RELEASE_MASK(idx: c_uint) -> c_uint {
    0x02 << (2 * idx)
}

/* TS3A227E_REG_INTERRUPT_DISABLE 0x03 */
const INS_REM_INT_DISABLE: c_uint = 0x01;
const DETECTION_COMPLETE_INT_DISABLE: c_uint = 0x02;
const ADC_COMPLETE_INT_DISABLE: c_uint = 0x04;
const INTB_DISABLE: c_uint = 0x08;

/* TS3A227E_REG_SETTING_1 0x4 */
const DEBOUNCE_INSERTION_SETTING_SFT: c_uint = 0;
const DEBOUNCE_INSERTION_SETTING_MASK: c_uint = 0x7 << DEBOUNCE_PRESS_SETTING_SFT;

/* TS3A227E_REG_SETTING_2 0x05 */
const KP_ENABLE: c_uint = 0x04;

/* TS3A227E_REG_SETTING_3 0x06 */
const MICBIAS_SETTING_SFT: c_uint = 3;
const MICBIAS_SETTING_MASK: c_uint = 0x7 << MICBIAS_SETTING_SFT;
const DEBOUNCE_RELEASE_SETTING_SFT: c_uint = 2;
const DEBOUNCE_RELEASE_SETTING_MASK: c_uint = 0x1 << DEBOUNCE_RELEASE_SETTING_SFT;
const DEBOUNCE_PRESS_SETTING_SFT: c_uint = 0;
const DEBOUNCE_PRESS_SETTING_MASK: c_uint = 0x3 << DEBOUNCE_PRESS_SETTING_SFT;

/* TS3A227E_REG_ACCESSORY_STATUS  0x0b */
const TYPE_3_POLE: c_uint = 0x01;
const TYPE_4_POLE_OMTP: c_uint = 0x02;
const TYPE_4_POLE_STANDARD: c_uint = 0x04;
const JACK_INSERTED: c_uint = 0x08;
const EITHER_MIC_MASK: c_uint = TYPE_4_POLE_OMTP | TYPE_4_POLE_STANDARD;

static ts3a227e_reg_defaults: [reg_default; 16] = [
    reg_default { reg: TS3A227E_REG_DEVICE_ID, def: 0x10 },
    reg_default { reg: TS3A227E_REG_INTERRUPT, def: 0x00 },
    reg_default { reg: TS3A227E_REG_KP_INTERRUPT, def: 0x00 },
    reg_default { reg: TS3A227E_REG_INTERRUPT_DISABLE, def: 0x08 },
    reg_default { reg: TS3A227E_REG_SETTING_1, def: 0x23 },
    reg_default { reg: TS3A227E_REG_SETTING_2, def: 0x00 },
    reg_default { reg: TS3A227E_REG_SETTING_3, def: 0x0e },
    reg_default { reg: TS3A227E_REG_SWITCH_CONTROL_1, def: 0x00 },
    reg_default { reg: TS3A227E_REG_SWITCH_CONTROL_2, def: 0x00 },
    reg_default { reg: TS3A227E_REG_SWITCH_STATUS_1, def: 0x0c },
    reg_default { reg: TS3A227E_REG_SWITCH_STATUS_2, def: 0x00 },
    reg_default { reg: TS3A227E_REG_ACCESSORY_STATUS, def: 0x00 },
    reg_default { reg: TS3A227E_REG_ADC_OUTPUT, def: 0x00 },
    reg_default { reg: TS3A227E_REG_KP_THRESHOLD_1, def: 0x20 },
    reg_default { reg: TS3A227E_REG_KP_THRESHOLD_2, def: 0x40 },
    reg_default { reg: TS3A227E_REG_KP_THRESHOLD_3, def: 0x68 },
];

unsafe extern "C" fn ts3a227e_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TS3A227E_REG_DEVICE_ID..=TS3A227E_REG_KP_THRESHOLD_3 => true,
        _ => false,
    }
}

unsafe extern "C" fn ts3a227e_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TS3A227E_REG_INTERRUPT_DISABLE..=TS3A227E_REG_SWITCH_CONTROL_2
        | TS3A227E_REG_KP_THRESHOLD_1..=TS3A227E_REG_KP_THRESHOLD_3 => true,
        _ => false,
    }
}

unsafe extern "C" fn ts3a227e_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TS3A227E_REG_INTERRUPT..=TS3A227E_REG_INTERRUPT_DISABLE
        | TS3A227E_REG_SETTING_1..=TS3A227E_REG_SETTING_2
        | TS3A227E_REG_SWITCH_STATUS_1..=TS3A227E_REG_ADC_OUTPUT => true,
        _ => false,
    }
}

unsafe fn ts3a227e_jack_report(ts3a227e: *mut ts3a227e) {
    let mut i: c_uint;
    let mut report: c_int = 0;

    if (*ts3a227e).jack.is_null() {
        return;
    }

    if (*ts3a227e).plugged {
        report = SND_JACK_HEADPHONE;
    }
    if (*ts3a227e).mic_present {
        report |= SND_JACK_MICROPHONE;
    }
    i = 0;
    while i < TS3A227E_NUM_BUTTONS {
        if ((*ts3a227e).buttons_held & (1 << i)) != 0 {
            report |= ts3a227e_buttons[i as usize];
        }
        i += 1;
    }
    snd_soc_jack_report((*ts3a227e).jack, report, TS3A227E_JACK_MASK);
}

unsafe fn ts3a227e_new_jack_state(ts3a227e: *mut ts3a227e, acc_reg: c_uint) {
    let plugged: bool;
    let mic_present: bool;

    plugged = (acc_reg & JACK_INSERTED) != 0;
    mic_present = plugged && (acc_reg & EITHER_MIC_MASK) != 0;

    (*ts3a227e).plugged = plugged;

    if mic_present != (*ts3a227e).mic_present {
        (*ts3a227e).mic_present = mic_present;
        (*ts3a227e).buttons_held = 0;
        if mic_present {
            /* Enable key press detection. */
            regmap_update_bits(
                (*ts3a227e).regmap,
                TS3A227E_REG_SETTING_2,
                KP_ENABLE,
                KP_ENABLE,
            );
        }
    }
}

unsafe extern "C" fn ts3a227e_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let ts3a227e = data as *mut ts3a227e;
    let regmap = (*ts3a227e).regmap;
    let mut int_reg: c_uint = 0;
    let mut kp_int_reg: c_uint = 0;
    let mut acc_reg: c_uint = 0;
    let mut i: c_uint;
    let dev = (*ts3a227e).dev;
    let mut ret: c_int;

    let _ = irq;

    /* Check for plug/unplug. */
    ret = regmap_read(regmap, TS3A227E_REG_INTERRUPT, &mut int_reg);
    if ret != 0 {
        dev_err(dev, c"failed to clear interrupt ret=%d\n".as_ptr(), ret);
        return IRQ_NONE;
    }

    if (int_reg & (DETECTION_COMPLETE_EVENT | INS_REM_EVENT)) != 0 {
        regmap_read(regmap, TS3A227E_REG_ACCESSORY_STATUS, &mut acc_reg);
        ts3a227e_new_jack_state(ts3a227e, acc_reg);
    }

    /* Report any key events. */
    ret = regmap_read(regmap, TS3A227E_REG_KP_INTERRUPT, &mut kp_int_reg);
    if ret != 0 {
        dev_err(dev, c"failed to clear key interrupt ret=%d\n".as_ptr(), ret);
        return IRQ_NONE;
    }

    i = 0;
    while i < TS3A227E_NUM_BUTTONS {
        if (kp_int_reg & PRESS_MASK(i)) != 0 {
            (*ts3a227e).buttons_held |= 1 << i;
        }
        if (kp_int_reg & RELEASE_MASK(i)) != 0 {
            (*ts3a227e).buttons_held &= !(1 << i);
        }
        i += 1;
    }

    ts3a227e_jack_report(ts3a227e);

    IRQ_HANDLED
}

/**
 * ts3a227e_enable_jack_detect - Specify a jack for event reporting
 *
 * @component:  component to register the jack with
 * @jack: jack to use to report headset and button events on
 *
 * After this function has been called the headset insert/remove and button
 * events 0-3 will be routed to the given jack.  Jack can be null to stop
 * reporting.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts3a227e_enable_jack_detect(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
) -> c_int {
    let ts3a227e = snd_soc_component_get_drvdata(component) as *mut ts3a227e;

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    (*ts3a227e).jack = jack;
    ts3a227e_jack_report(ts3a227e);

    0
}
/* EXPORT_SYMBOL_GPL(ts3a227e_enable_jack_detect); */

unsafe extern "C" fn ts3a227e_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    data: *mut c_void,
) -> c_int {
    let _ = data;

    if jack.is_null() {
        return -EINVAL;
    }

    ts3a227e_enable_jack_detect(component, jack)
}

unsafe extern "C" fn ts3a227e_get_jack_type(_component: *mut snd_soc_component) -> c_int {
    SND_JACK_HEADSET
}

static ts3a227e_soc_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: c"ti,ts3a227e".as_ptr(),
    set_jack: Some(ts3a227e_set_jack),
    get_jack_type: Some(ts3a227e_get_jack_type),
};

static ts3a227e_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,

    max_register: TS3A227E_REG_KP_THRESHOLD_3,
    readable_reg: Some(ts3a227e_readable_reg),
    writeable_reg: Some(ts3a227e_writeable_reg),
    volatile_reg: Some(ts3a227e_volatile_reg),

    cache_type: REGCACHE_RBTREE,
    reg_defaults: ts3a227e_reg_defaults.as_ptr(),
    num_reg_defaults: ts3a227e_reg_defaults.len() as c_uint,
};

unsafe fn ts3a227e_parse_device_property(ts3a227e: *mut ts3a227e, dev: *mut device) -> c_int {
    let mut value: u32 = 0;
    let mut value_ms: u32 = 0;
    let mut setting3_value: u32 = 0;
    let mut setting3_mask: u32 = 0;
    let mut err: c_int;

    err = device_property_read_u32(dev, c"ti,micbias".as_ptr(), &mut value);
    if err == 0 {
        setting3_mask = MICBIAS_SETTING_MASK;
        setting3_value = (value << MICBIAS_SETTING_SFT) & MICBIAS_SETTING_MASK;
    }

    err = device_property_read_u32(dev, c"ti,debounce-release-ms".as_ptr(), &mut value_ms);
    if err == 0 {
        value = (value_ms > 10) as u32;
        setting3_mask |= DEBOUNCE_RELEASE_SETTING_MASK;
        setting3_value |=
            (value << DEBOUNCE_RELEASE_SETTING_SFT) & DEBOUNCE_RELEASE_SETTING_MASK;
    }

    err = device_property_read_u32(dev, c"ti,debounce-press-ms".as_ptr(), &mut value_ms);
    if err == 0 {
        value = (value_ms + 20) / 40;
        if value > 3 {
            value = 3;
        }
        setting3_mask |= DEBOUNCE_PRESS_SETTING_MASK;
        setting3_value |= (value << DEBOUNCE_PRESS_SETTING_SFT) & DEBOUNCE_PRESS_SETTING_MASK;
    }

    if setting3_mask != 0 {
        regmap_update_bits(
            (*ts3a227e).regmap,
            TS3A227E_REG_SETTING_3,
            setting3_mask,
            setting3_value,
        );
    }

    err = device_property_read_u32(dev, c"ti,debounce-insertion-ms".as_ptr(), &mut value_ms);
    if err == 0 {
        if value_ms < 165 {
            value = (value_ms + 15) / 30;
        } else if value_ms < 1500 {
            value = 6;
        } else {
            value = 7;
        }
        regmap_update_bits(
            (*ts3a227e).regmap,
            TS3A227E_REG_SETTING_1,
            DEBOUNCE_INSERTION_SETTING_MASK,
            (value << DEBOUNCE_INSERTION_SETTING_SFT) & DEBOUNCE_INSERTION_SETTING_MASK,
        );
    }

    0
}

unsafe extern "C" fn ts3a227e_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ts3a227e: *mut ts3a227e;
    let dev: *mut device = &mut (*i2c).dev;
    let mut ret: c_int;
    let mut acc_reg: c_uint = 0;

    ts3a227e = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<ts3a227e>(),
        GFP_KERNEL,
    ) as *mut ts3a227e;
    if ts3a227e.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, ts3a227e as *mut c_void);
    (*ts3a227e).dev = dev;
    (*ts3a227e).irq = (*i2c).irq;

    (*ts3a227e).regmap = devm_regmap_init_i2c(i2c, &ts3a227e_regmap_config);
    if IS_ERR((*ts3a227e).regmap as *const c_void) {
        return PTR_ERR((*ts3a227e).regmap as *const c_void);
    }

    ret = ts3a227e_parse_device_property(ts3a227e, dev);
    if ret != 0 {
        dev_err(dev, c"Failed to parse device property: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = devm_request_threaded_irq(
        dev,
        (*i2c).irq as c_uint,
        ptr::null_mut(),
        Some(ts3a227e_interrupt),
        IRQF_TRIGGER_LOW | IRQF_ONESHOT,
        c"TS3A227E".as_ptr(),
        ts3a227e as *mut c_void,
    );
    if ret != 0 {
        dev_err(dev, c"Cannot request irq %d (%d)\n".as_ptr(), (*i2c).irq, ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &ts3a227e_soc_driver, ptr::null_mut(), 0);
    if ret != 0 {
        return ret;
    }

    /* Enable interrupts except for ADC complete. */
    regmap_update_bits(
        (*ts3a227e).regmap,
        TS3A227E_REG_INTERRUPT_DISABLE,
        INTB_DISABLE | ADC_COMPLETE_INT_DISABLE,
        ADC_COMPLETE_INT_DISABLE,
    );

    /* Read jack status because chip might not trigger interrupt at boot. */
    regmap_read(
        (*ts3a227e).regmap,
        TS3A227E_REG_ACCESSORY_STATUS,
        &mut acc_reg,
    );
    ts3a227e_new_jack_state(ts3a227e, acc_reg);
    ts3a227e_jack_report(ts3a227e);

    0
}

unsafe extern "C" fn ts3a227e_suspend(dev: *mut device) -> c_int {
    let ts3a227e = dev_get_drvdata(dev) as *mut ts3a227e;

    dev_dbg((*ts3a227e).dev, c"suspend disable irq\n".as_ptr());
    disable_irq((*ts3a227e).irq as c_uint);

    0
}

unsafe extern "C" fn ts3a227e_resume(dev: *mut device) -> c_int {
    let ts3a227e = dev_get_drvdata(dev) as *mut ts3a227e;

    dev_dbg((*ts3a227e).dev, c"resume enable irq\n".as_ptr());
    enable_irq((*ts3a227e).irq as c_uint);

    0
}

static ts3a227e_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(ts3a227e_suspend),
    resume: Some(ts3a227e_resume),
};

static ts3a227e_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b't' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'a' as c_char,
            b'2' as c_char,
            b'2' as c_char,
            b'7' as c_char,
            b'e' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(i2c, ts3a227e_i2c_ids); */

/* CONFIG_OF conditional device table from the C source. */
static ts3a227e_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,ts3a227e".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ts3a227e_of_match); */

/* CONFIG_ACPI conditional device table from the C source. */
static mut ts3a227e_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'1' as c_char,
            b'0' as c_char,
            b'4' as c_char,
            b'C' as c_char,
            b'2' as c_char,
            b'2' as c_char,
            b'7' as c_char,
            b'E' as c_char,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, ts3a227e_acpi_match); */

static mut ts3a227e_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ts3a227e".as_ptr(),
        pm: &ts3a227e_pm,
        of_match_table: ts3a227e_of_match.as_ptr(),
        acpi_match_table: unsafe { ts3a227e_acpi_match.as_ptr() },
    },
    probe: Some(ts3a227e_i2c_probe),
    id_table: ts3a227e_i2c_ids.as_ptr(),
};
/* module_i2c_driver(ts3a227e_driver); */

/* MODULE_DESCRIPTION("ASoC ts3a227e driver"); */
/* MODULE_AUTHOR("Dylan Reid <dgreid@chromium.org>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
