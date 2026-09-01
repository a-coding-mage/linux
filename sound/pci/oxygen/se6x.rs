// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8787 driver for the Studio Evolution SE6X
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * CMI8787:
 *
 *   SPI    -> microcontroller (not actually used)
 *   GPIO 0 -> do.
 *   GPIO 2 -> do.
 *
 *   DAC0   -> both PCM1792A (L+R, each in mono mode)
 *   ADC1  <-  1st PCM1804
 *   ADC2  <-  2nd PCM1804
 *   ADC3  <-  3rd PCM1804
 */

// C dependencies in the original source:
// <linux/pci.h>, <linux/module.h>, <sound/core.h>, <sound/control.h>,
// <sound/initval.h>, <sound/pcm.h>, and "oxygen.h".

extern "C" {
    static mut THIS_MODULE: *mut module;
    static oxygen_pci_pm: dev_pm_ops;

    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn oxygen_pci_probe(
        pci: *mut pci_dev,
        index: c_int,
        id: *mut c_char,
        owner: *mut module,
        ids: *const pci_device_id,
        get_model: Option<
            unsafe extern "C" fn(
                chip: *mut oxygen,
                pci_id: *const pci_device_id,
            ) -> c_int,
        >,
    ) -> c_int;
    fn pm_sleep_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops;
    fn oxygen_pci_shutdown(pci: *mut pci_dev);
}

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type bool_ = bool;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: device_driver,
    pub shutdown: Option<unsafe extern "C" fn(*mut pci_dev)>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oxygen_model {
    pub shortname: *const c_char,
    pub longname: *const c_char,
    pub chip: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub control_filter: Option<unsafe extern "C" fn(*mut snd_kcontrol_new) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub set_dac_params: Option<unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params)>,
    pub set_adc_params: Option<unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params)>,
    pub adjust_dac_routing: Option<unsafe extern "C" fn(*mut oxygen, c_uint) -> c_uint>,
    pub device_config: c_uint,
    pub dac_channels_pcm: c_uint,
    pub function_flags: c_uint,
    pub dac_mclks: c_uint,
    pub adc_mclks: c_uint,
    pub dac_i2s_format: c_uint,
    pub adc_i2s_format: c_uint,
}

#[repr(C)]
pub struct oxygen {
    pub card: *mut snd_card,
    pub model: oxygen_model,
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const OXYGEN_GPIO_CONTROL: c_uint = 0;
const OXYGEN_PLAY_DAC0_SOURCE_MASK: c_uint = 0;
const OXYGEN_PLAY_DAC1_SOURCE_MASK: c_uint = 0;
const PLAYBACK_0_TO_I2S: c_uint = 0;
const CAPTURE_0_FROM_I2S_1: c_uint = 0;
const CAPTURE_2_FROM_I2S_2: c_uint = 0;
const CAPTURE_3_FROM_I2S_3: c_uint = 0;
const OXYGEN_FUNCTION_SPI: c_uint = 0;
const OXYGEN_I2S_FORMAT_LJUST: c_uint = 0;
const OXYGEN_I2S_FORMAT_I2S: c_uint = 0;

const KBUILD_MODNAME: *const c_char = b"se6x\0".as_ptr() as *const c_char;

const fn OXYGEN_PCI_SUBID(subvendor: c_uint, subdevice: c_uint) -> pci_device_id {
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor,
        subdevice,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

const fn OXYGEN_MCLKS(a: c_uint, b: c_uint, c: c_uint) -> c_uint {
    (a << 16) | (b << 8) | c
}

// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_DESCRIPTION("Studio Evolution SE6X driver");
// MODULE_LICENSE("GPL v2");

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "card index");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "enable card");

static se6x_ids: [pci_device_id; 2] = [
    OXYGEN_PCI_SUBID(0x13f6, 0x8788),
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(pci, se6x_ids);

unsafe extern "C" fn se6x_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, 0x005);

    snd_component_add((*chip).card, b"PCM1792A\0".as_ptr() as *const c_char);
    snd_component_add((*chip).card, b"PCM1804\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn se6x_control_filter(template: *mut snd_kcontrol_new) -> c_int {
    /* no DAC volume/mute */
    if strncmp(
        (*template).name,
        b"Master Playback \0".as_ptr() as *const c_char,
        16,
    ) == 0
    {
        return 1;
    }
    0
}

unsafe extern "C" fn se6x_cleanup(_chip: *mut oxygen) {}

unsafe extern "C" fn set_pcm1792a_params(
    _chip: *mut oxygen,
    _params: *mut snd_pcm_hw_params,
) {
    /* nothing to do (the microcontroller monitors DAC_LRCK) */
}

unsafe extern "C" fn set_pcm1804_params(
    _chip: *mut oxygen,
    _params: *mut snd_pcm_hw_params,
) {
}

unsafe extern "C" fn se6x_adjust_dac_routing(
    _chip: *mut oxygen,
    play_routing: c_uint,
) -> c_uint {
    /* route the same stereo pair to DAC0 and DAC1 */
    (play_routing & OXYGEN_PLAY_DAC0_SOURCE_MASK)
        | ((play_routing << 2) & OXYGEN_PLAY_DAC1_SOURCE_MASK)
}

static model_se6x: oxygen_model = oxygen_model {
    shortname: b"Studio Evolution SE6X\0".as_ptr() as *const c_char,
    longname: b"C-Media Oxygen HD Audio\0".as_ptr() as *const c_char,
    chip: b"CMI8787\0".as_ptr() as *const c_char,
    init: Some(se6x_init),
    control_filter: Some(se6x_control_filter),
    cleanup: Some(se6x_cleanup),
    set_dac_params: Some(set_pcm1792a_params),
    set_adc_params: Some(set_pcm1804_params),
    adjust_dac_routing: Some(se6x_adjust_dac_routing),
    device_config: PLAYBACK_0_TO_I2S
        | CAPTURE_0_FROM_I2S_1
        | CAPTURE_2_FROM_I2S_2
        | CAPTURE_3_FROM_I2S_3,
    dac_channels_pcm: 2,
    function_flags: OXYGEN_FUNCTION_SPI,
    dac_mclks: OXYGEN_MCLKS(256, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 256, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_I2S,
};

unsafe extern "C" fn se6x_get_model(
    chip: *mut oxygen,
    _pci_id: *const pci_device_id,
) -> c_int {
    (*chip).model = model_se6x;
    0
}

unsafe extern "C" fn se6x_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let err: c_int;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = oxygen_pci_probe(
        pci,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        se6x_ids.as_ptr(),
        Some(se6x_get_model),
    );
    if err >= 0 {
        dev += 1;
    }
    err
}

static mut se6x_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: se6x_ids.as_ptr(),
    probe: Some(se6x_probe),
    driver: device_driver {
        pm: unsafe { pm_sleep_ptr(&oxygen_pci_pm as *const dev_pm_ops) },
    },
    shutdown: Some(oxygen_pci_shutdown),
};

// module_pci_driver(se6x_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
