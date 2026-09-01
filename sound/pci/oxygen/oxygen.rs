// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver for C-Media's reference design and similar models
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * CMI8788:
 *
 *   SPI 0 -> 1st AK4396 (front)
 *   SPI 1 -> 2nd AK4396 (surround)
 *   SPI 2 -> 3rd AK4396 (center/LFE)
 *   SPI 3 -> WM8785
 *   SPI 4 -> 4th AK4396 (back)
 *
 *   GPIO 0 -> DFS0 of AK5385
 *   GPIO 1 -> DFS1 of AK5385
 *
 * X-Meridian models:
 *   GPIO 4 -> enable extension S/PDIF input
 *   GPIO 6 -> enable on-board S/PDIF input
 *
 * Claro models:
 *   GPIO 6 -> S/PDIF from optical (0) or coaxial (1) input
 *   GPIO 8 -> enable headphone amplifier
 *
 * eClaro model:
 *   GPIO 2 -> M0 of CS5361
 *   GPIO 3 -> M1 of CS5361
 *   GPIO 8 -> enable headphone amplifier
 *
 * CM9780:
 *
 *   LINE_OUT -> input of ADC
 *
 *   AUX_IN <- aux
 *   CD_IN  <- CD
 *   MIC_IN <- mic
 *
 *   GPO 0 -> route line-in (0) or AC97 output (1) to ADC input
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;

/* Includes removed: linux/delay.h, linux/mutex.h, linux/pci.h, linux/module.h,
 * ALSA sound headers, oxygen.h, xonar_dg.h, ak4396.h, cs4362a.h, wm8785.h.
 * The referenced kernel/ALSA symbols are external dependencies.
 */

extern "C" {
    static mut model_xonar_dg: oxygen_model;
    static mut oxygen_pci_pm: c_void;

    fn oxygen_write_spi(chip: *mut oxygen, control: c_uint, data: c_uint) -> c_int;
    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_clear_bits16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_write16_masked(chip: *mut oxygen, reg: c_uint, value: u16, mask: u16);
    fn oxygen_read16(chip: *mut oxygen, reg: c_uint) -> u16;
    fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_pci_probe(
        pci: *mut pci_dev,
        index: c_int,
        id: *mut c_char,
        module: *mut c_void,
        ids: *const pci_device_id,
        get_model: unsafe extern "C" fn(*mut oxygen, *const pci_device_id) -> c_int,
    ) -> c_int;

    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn snd_ctl_enum_info(
        info: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oxygen {
    pub model_data: *mut c_void,
    pub card: *mut snd_card,
    pub dac_volume: [u8; 8],
    pub dac_mute: bool,
    pub mutex: mutex,
    pub model: oxygen_model,
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_data: *mut oxygen,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oxygen_model {
    pub shortname: *const c_char,
    pub longname: *const c_char,
    pub chip: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub mixer_init: Option<unsafe extern "C" fn(*mut oxygen) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub suspend: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub resume: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub set_dac_params: Option<unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params)>,
    pub set_adc_params: Option<unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params)>,
    pub update_dac_volume: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub update_dac_mute: Option<unsafe extern "C" fn(*mut oxygen)>,
    pub dump_registers: Option<unsafe extern "C" fn(*mut oxygen, *mut snd_info_buffer)>,
    pub dac_tlv: *const c_uint,
    pub model_data_size: usize,
    pub device_config: c_uint,
    pub dac_channels_pcm: c_uint,
    pub dac_channels_mixer: c_uint,
    pub dac_volume_min: c_int,
    pub dac_volume_max: c_int,
    pub function_flags: c_uint,
    pub dac_mclks: c_uint,
    pub adc_mclks: c_uint,
    pub dac_i2s_format: c_uint,
    pub adc_i2s_format: c_uint,
    pub misc_flags: c_uint,
}

#[repr(C)]
pub struct pci_driver_driver {
    pub pm: *mut c_void,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_driver,
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

extern "C" {
    static mut THIS_MODULE: c_void;
}

const fn oxygen_pci_subid(subvendor: u32, subdevice: u32) -> pci_device_id {
    pci_device_id {
        vendor: 0x13f6,
        device: 0x8788,
        subvendor,
        subdevice,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

const OXYGEN_SPI_TRIGGER: c_uint = 0;
const OXYGEN_SPI_DATA_LENGTH_2: c_uint = 0;
const OXYGEN_SPI_DATA_LENGTH_3: c_uint = 0;
const OXYGEN_SPI_CLOCK_160: c_uint = 0;
const OXYGEN_SPI_CLOCK_1280: c_uint = 0;
const OXYGEN_SPI_CODEC_SHIFT: c_uint = 0;
const OXYGEN_SPI_CEN_LATCH_CLOCK_HI: c_uint = 0;
const OXYGEN_SPI_CEN_LATCH_CLOCK_LO: c_uint = 0;
const OXYGEN_GPIO_CONTROL: c_uint = 0;
const OXYGEN_GPIO_DATA: c_uint = 0;
const OXYGEN_FUNCTION_SPI: c_uint = 0;
const OXYGEN_FUNCTION_ENABLE_SPI_4_5: c_uint = 0;
const OXYGEN_I2S_FORMAT_LJUST: c_uint = 0;
const OXYGEN_MISC_MIDI: c_uint = 0;

const PLAYBACK_0_TO_I2S: c_uint = 0;
const PLAYBACK_1_TO_SPDIF: c_uint = 0;
const PLAYBACK_2_TO_AC97_1: c_uint = 0;
const CAPTURE_0_FROM_I2S_1: c_uint = 0;
const CAPTURE_0_FROM_I2S_2: c_uint = 0;
const CAPTURE_1_FROM_SPDIF: c_uint = 0;
const CAPTURE_2_FROM_AC97_1: c_uint = 0;
const AC97_CD_INPUT: c_uint = 0;
const MIDI_OUTPUT: c_uint = 0;
const MIDI_INPUT: c_uint = 0;

const AK4396_WRITE: u16 = 0;
const AK4396_CONTROL_1: usize = 0;
const AK4396_CONTROL_2: usize = 1;
const AK4396_CONTROL_3: usize = 2;
const AK4396_LCH_ATT: usize = 3;
const AK4396_RCH_ATT: usize = 4;
const AK4396_DIF_24_MSB: u8 = 0;
const AK4396_RSTN: u8 = 0;
const AK4396_SMUTE: u8 = 0;
const AK4396_DEM_OFF: u8 = 0;
const AK4396_DFS_NORMAL: u8 = 0;
const AK4396_DFS_DOUBLE: u8 = 0;
const AK4396_DFS_QUAD: u8 = 0;
const AK4396_DFS_MASK: u8 = 0;
const AK4396_PCM: u8 = 0;
const AK4396_ACKS: u8 = 0;
const AK4396_SLOW: u8 = 0;

const CS4362A_CPEN: u8 = 0;
const CS4362A_PDN: u8 = 0;
const CS4362A_DIF_LJUST: u8 = 0;
const CS4362A_SOFT_RAMP: u8 = 0;
const CS4362A_AMUTE: u8 = 0;
const CS4362A_RMP_DN: u8 = 0;
const CS4362A_DEM_NONE: u8 = 0;
const CS4362A_FM_SINGLE: u8 = 0;
const CS4362A_FM_DOUBLE: u8 = 0;
const CS4362A_FM_QUAD: u8 = 0;
const CS4362A_FM_MASK: u8 = 0;
const CS4362A_ATAPI_B_R: u8 = 0;
const CS4362A_ATAPI_A_L: u8 = 0;
const CS4362A_MUTE: u8 = 0;

const WM8785_R0: u8 = 0;
const WM8785_R2: u8 = 2;
const WM8785_R7: u8 = 7;
const WM8785_MCR_SLAVE: c_uint = 0;
const WM8785_OSR_SINGLE: c_uint = 0;
const WM8785_OSR_DOUBLE: c_uint = 0;
const WM8785_OSR_QUAD: c_uint = 0;
const WM8785_FORMAT_LJUST: c_uint = 0;
const WM8785_HPFR: c_uint = 0;
const WM8785_HPFL: c_uint = 0;

const fn OXYGEN_MCLKS(_a: c_uint, _b: c_uint, _c: c_uint) -> c_uint {
    0
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR };
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

const MODEL_CMEDIA_REF: usize = 0;
const MODEL_MERIDIAN: usize = 1;
const MODEL_MERIDIAN_2G: usize = 2;
const MODEL_CLARO: usize = 3;
const MODEL_CLARO_HALO: usize = 4;
const MODEL_ECLARO: usize = 5;
const MODEL_FANTASIA: usize = 6;
const MODEL_SERENADE: usize = 7;
const MODEL_2CH_OUTPUT: usize = 8;
const MODEL_HG2PCI: usize = 9;
const MODEL_XONAR_DG: usize = 10;
const MODEL_XONAR_DGX: usize = 11;

static mut oxygen_ids: [pci_device_id; 22] = [
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x10b0, 0x0216) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x10b0, 0x0217) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x10b0, 0x0218) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x10b0, 0x0219) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x13f6, 0x0001) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x13f6, 0x0010) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x13f6, 0x8788) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x147a, 0xa017) },
    pci_device_id { driver_data: MODEL_CMEDIA_REF, ..oxygen_pci_subid(0x1a58, 0x0910) },
    pci_device_id { driver_data: MODEL_XONAR_DG, ..oxygen_pci_subid(0x1043, 0x8467) },
    pci_device_id { driver_data: MODEL_XONAR_DGX, ..oxygen_pci_subid(0x1043, 0x8521) },
    pci_device_id { driver_data: MODEL_2CH_OUTPUT, ..oxygen_pci_subid(0x13f6, 0x8782) },
    pci_device_id { driver_data: MODEL_HG2PCI, ..oxygen_pci_subid(0x13f6, 0xffff) },
    pci_device_id { driver_data: MODEL_FANTASIA, ..oxygen_pci_subid(0x14c3, 0x1710) },
    pci_device_id { driver_data: MODEL_SERENADE, ..oxygen_pci_subid(0x14c3, 0x1711) },
    pci_device_id { driver_data: MODEL_MERIDIAN, ..oxygen_pci_subid(0x415a, 0x5431) },
    pci_device_id { driver_data: MODEL_MERIDIAN_2G, ..oxygen_pci_subid(0x5431, 0x017a) },
    pci_device_id { driver_data: MODEL_CLARO, ..oxygen_pci_subid(0x7284, 0x9761) },
    pci_device_id { driver_data: MODEL_CLARO_HALO, ..oxygen_pci_subid(0x7284, 0x9781) },
    pci_device_id { driver_data: MODEL_ECLARO, ..oxygen_pci_subid(0x7284, 0x9783) },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

const GPIO_AK5385_DFS_MASK: u16 = 0x0003;
const GPIO_AK5385_DFS_NORMAL: u16 = 0x0000;
const GPIO_AK5385_DFS_DOUBLE: u16 = 0x0001;
const GPIO_AK5385_DFS_QUAD: u16 = 0x0002;

const GPIO_MERIDIAN_DIG_MASK: u16 = 0x0050;
const GPIO_MERIDIAN_DIG_EXT: u16 = 0x0010;
const GPIO_MERIDIAN_DIG_BOARD: u16 = 0x0040;

const GPIO_CLARO_DIG_COAX: u16 = 0x0040;
const GPIO_CLARO_HP: u16 = 0x0100;

const GPIO_ECLARO_CS4362A_NRESET: u16 = 0x0001;
const GPIO_ECLARO_FRONT_ENABLE: u16 = 0x0020;

/* CS4362A SPI: 3-byte frame [0x30, reg, value] on CE1, 1280 ns/bit clock */
const ECLARO_CS4362A_SPI_CONTROL: c_uint = OXYGEN_SPI_TRIGGER
    | OXYGEN_SPI_DATA_LENGTH_3
    | OXYGEN_SPI_CLOCK_1280
    | (1 << OXYGEN_SPI_CODEC_SHIFT)
    | OXYGEN_SPI_CEN_LATCH_CLOCK_HI;

#[repr(C)]
struct generic_data {
    dacs: c_uint,
    spi_map: [u8; 4],
    spi_prefix: [u16; 4],
    ak4396_regs: [[u8; 5]; 4],
    cs4362a_regs: [u8; 15],
    wm8785_regs: [u16; 3],
}

unsafe extern "C" fn ak4396_write(chip: *mut oxygen, codec: c_uint, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut generic_data;
    oxygen_write_spi(
        chip,
        OXYGEN_SPI_TRIGGER
            | OXYGEN_SPI_DATA_LENGTH_2
            | OXYGEN_SPI_CLOCK_160
            | (((*data).spi_map[codec as usize] as c_uint) << OXYGEN_SPI_CODEC_SHIFT)
            | OXYGEN_SPI_CEN_LATCH_CLOCK_HI,
        ((*data).spi_prefix[codec as usize] | ((reg as u16) << 8) | value as u16) as c_uint,
    );
    (*data).ak4396_regs[codec as usize][reg as usize] = value;
}

unsafe extern "C" fn ak4396_write_cached(chip: *mut oxygen, codec: c_uint, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut generic_data;
    if value != (*data).ak4396_regs[codec as usize][reg as usize] {
        ak4396_write(chip, codec, reg, value);
    }
}

unsafe extern "C" fn eclaro_cs4362a_write(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut generic_data;
    let err: c_int;

    if (reg as usize) < (*data).cs4362a_regs.len() {
        (*data).cs4362a_regs[reg as usize] = value;
    }

    err = oxygen_write_spi(
        chip,
        ECLARO_CS4362A_SPI_CONTROL,
        (0x30u32 << 16) | ((reg as u32) << 8) | value as u32,
    );
    if err != 0 {
        dev_err((*(*chip).card).dev, b"CS4362A SPI timeout: reg=0x%02x val=0x%02x\n\0".as_ptr() as *const c_char, reg as c_uint, value as c_uint);
    }
}

unsafe extern "C" fn eclaro_cs4362a_write_cached(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut generic_data;

    if value != (*data).cs4362a_regs[reg as usize] {
        eclaro_cs4362a_write(chip, reg, value);
    }
}

unsafe extern "C" fn eclaro_cs4362a_registers_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;

    eclaro_cs4362a_write(chip, 1, CS4362A_CPEN | CS4362A_PDN);
    eclaro_cs4362a_write(chip, 2, CS4362A_DIF_LJUST);
    eclaro_cs4362a_write(chip, 3, CS4362A_SOFT_RAMP | CS4362A_AMUTE);
    eclaro_cs4362a_write(chip, 4, (*data).cs4362a_regs[4]);
    eclaro_cs4362a_write(chip, 5, 0);
    eclaro_cs4362a_write(chip, 6, (*data).cs4362a_regs[6]);
    eclaro_cs4362a_write(chip, 7, (*data).cs4362a_regs[7]);
    eclaro_cs4362a_write(chip, 8, (*data).cs4362a_regs[8]);
    eclaro_cs4362a_write(chip, 9, (*data).cs4362a_regs[9]);
    eclaro_cs4362a_write(chip, 10, (*data).cs4362a_regs[10]);
    eclaro_cs4362a_write(chip, 11, (*data).cs4362a_regs[11]);
    eclaro_cs4362a_write(chip, 12, (*data).cs4362a_regs[12]);
    eclaro_cs4362a_write(chip, 13, (*data).cs4362a_regs[13]);
    eclaro_cs4362a_write(chip, 14, (*data).cs4362a_regs[14]);
    eclaro_cs4362a_write(chip, 1, CS4362A_CPEN);
}

unsafe extern "C" fn wm8785_write(chip: *mut oxygen, reg: u8, value: c_uint) {
    let data = (*chip).model_data as *mut generic_data;

    oxygen_write_spi(
        chip,
        OXYGEN_SPI_TRIGGER
            | OXYGEN_SPI_DATA_LENGTH_2
            | OXYGEN_SPI_CLOCK_160
            | (3 << OXYGEN_SPI_CODEC_SHIFT)
            | OXYGEN_SPI_CEN_LATCH_CLOCK_LO,
        ((reg as c_uint) << 9) | value,
    );
    if (reg as usize) < (*data).wm8785_regs.len() {
        (*data).wm8785_regs[reg as usize] = value as u16;
    }
}

unsafe extern "C" fn ak4396_registers_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;

    i = 0;
    while i < (*data).dacs {
        ak4396_write(chip, i, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_RSTN);
        ak4396_write(chip, i, AK4396_CONTROL_2 as u8, (*data).ak4396_regs[0][AK4396_CONTROL_2]);
        ak4396_write(chip, i, AK4396_CONTROL_3 as u8, AK4396_PCM);
        ak4396_write(chip, i, AK4396_LCH_ATT as u8, (*chip).dac_volume[(i * 2) as usize]);
        ak4396_write(chip, i, AK4396_RCH_ATT as u8, (*chip).dac_volume[(i * 2 + 1) as usize]);
        i += 1;
    }
}

unsafe extern "C" fn ak4396_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;
    static default_spi_map: [u8; 4] = [0, 1, 2, 4];
    let mut i: c_uint;

    (*data).dacs = (*chip).model.dac_channels_pcm / 2;
    (*data).spi_map = default_spi_map;
    i = 0;
    while i < 4 {
        (*data).spi_prefix[i as usize] = AK4396_WRITE;
        i += 1;
    }
    (*data).ak4396_regs[0][AK4396_CONTROL_2] = AK4396_SMUTE | AK4396_DEM_OFF | AK4396_DFS_NORMAL;
    ak4396_registers_init(chip);
    snd_component_add((*chip).card, b"AK4396\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn ak5385_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_AK5385_DFS_MASK);
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_AK5385_DFS_MASK);
    snd_component_add((*chip).card, b"AK5385\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn wm8785_registers_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;

    wm8785_write(chip, WM8785_R7, 0);
    wm8785_write(chip, WM8785_R0, (*data).wm8785_regs[0] as c_uint);
    wm8785_write(chip, WM8785_R2, (*data).wm8785_regs[2] as c_uint);
}

unsafe extern "C" fn wm8785_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;

    (*data).wm8785_regs[0] = (WM8785_MCR_SLAVE | WM8785_OSR_SINGLE | WM8785_FORMAT_LJUST) as u16;
    (*data).wm8785_regs[2] = (WM8785_HPFR | WM8785_HPFL) as u16;
    wm8785_registers_init(chip);
    snd_component_add((*chip).card, b"WM8785\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn generic_init(chip: *mut oxygen) {
    ak4396_init(chip);
    wm8785_init(chip);
}

unsafe extern "C" fn meridian_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_MERIDIAN_DIG_MASK);
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA, GPIO_MERIDIAN_DIG_BOARD, GPIO_MERIDIAN_DIG_MASK);
    ak4396_init(chip);
    ak5385_init(chip);
}

unsafe extern "C" fn claro_enable_hp(chip: *mut oxygen) {
    msleep(300);
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CLARO_HP);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, GPIO_CLARO_HP);
}

unsafe extern "C" fn claro_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CLARO_DIG_COAX);
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_CLARO_DIG_COAX);
    ak4396_init(chip);
    wm8785_init(chip);
    claro_enable_hp(chip);
}

unsafe extern "C" fn claro_halo_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CLARO_DIG_COAX);
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_CLARO_DIG_COAX);
    ak4396_init(chip);
    ak5385_init(chip);
    claro_enable_hp(chip);
}

unsafe extern "C" fn fantasia_init(chip: *mut oxygen) {
    ak4396_init(chip);
    snd_component_add((*chip).card, b"CS5340\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn stereo_output_init(chip: *mut oxygen) {
    ak4396_init(chip);
}

unsafe extern "C" fn generic_cleanup(_chip: *mut oxygen) {}

unsafe extern "C" fn claro_disable_hp(chip: *mut oxygen) {
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_CLARO_HP);
}

unsafe extern "C" fn claro_cleanup(chip: *mut oxygen) {
    claro_disable_hp(chip);
}

unsafe extern "C" fn claro_suspend(chip: *mut oxygen) {
    claro_disable_hp(chip);
}

unsafe extern "C" fn generic_resume(chip: *mut oxygen) {
    ak4396_registers_init(chip);
    wm8785_registers_init(chip);
}

unsafe extern "C" fn meridian_resume(chip: *mut oxygen) {
    ak4396_registers_init(chip);
}

unsafe extern "C" fn claro_resume(chip: *mut oxygen) {
    ak4396_registers_init(chip);
    claro_enable_hp(chip);
}

const GPIO_CS5361_M_MASK: u16 = 0x000c;
const GPIO_CS5361_M_SINGLE: u16 = 0x0000;
const GPIO_CS5361_M_DOUBLE: u16 = 0x0004;
const GPIO_CS5361_M_QUAD: u16 = 0x0008;

unsafe extern "C" fn cs5361_init(chip: *mut oxygen) {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CS5361_M_MASK);
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA, GPIO_CS5361_M_SINGLE, GPIO_CS5361_M_MASK);
}

unsafe extern "C" fn set_cs5361_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let value: c_uint;

    if params_rate(params) <= 54000 {
        value = GPIO_CS5361_M_SINGLE as c_uint;
    } else if params_rate(params) <= 108000 {
        value = GPIO_CS5361_M_DOUBLE as c_uint;
    } else {
        value = GPIO_CS5361_M_QUAD as c_uint;
    }
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA, value as u16, GPIO_CS5361_M_MASK);
}

unsafe extern "C" fn eclaro_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;

    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CLARO_DIG_COAX);
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_CLARO_DIG_COAX);

    /* Single AK4396VF on SPI CE0/CA=00 handles front L/R */
    (*data).dacs = 1;
    (*data).spi_map[0] = 0;
    (*data).spi_prefix[0] = AK4396_WRITE;
    (*data).ak4396_regs[0][AK4396_CONTROL_2] = AK4396_SMUTE | AK4396_DEM_OFF | AK4396_DFS_NORMAL;

    ak4396_write(chip, 0, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_ACKS);
    ak4396_write(chip, 0, AK4396_CONTROL_2 as u8, (*data).ak4396_regs[0][AK4396_CONTROL_2]);
    ak4396_write(chip, 0, AK4396_CONTROL_3 as u8, AK4396_PCM);
    ak4396_write(chip, 0, AK4396_LCH_ATT as u8, (*chip).dac_volume[0].wrapping_mul(2));
    ak4396_write(chip, 0, AK4396_RCH_ATT as u8, (*chip).dac_volume[1].wrapping_mul(2));
    ak4396_write(chip, 0, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_ACKS | AK4396_RSTN);

    /* CS4362A (SPI CE1): surround/center-LFE/side L/R.
     * GPIO 0 (RESET#, active-low) and GPIO 5 (front output enable) must
     * be driven high. GPIOs 1 and 7 are outputs driven high.
     */
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, 0x00a3);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, 0x00a3);
    usleep_range(1000, 2000);

    (*data).cs4362a_regs[4] = CS4362A_RMP_DN | CS4362A_DEM_NONE;
    (*data).cs4362a_regs[6] = CS4362A_FM_SINGLE | CS4362A_ATAPI_B_R | CS4362A_ATAPI_A_L;
    (*data).cs4362a_regs[7] = CS4362A_MUTE;
    (*data).cs4362a_regs[9] = (*data).cs4362a_regs[6];
    (*data).cs4362a_regs[12] = (*data).cs4362a_regs[6];

    eclaro_cs4362a_registers_init(chip);

    snd_component_add((*chip).card, b"AK4396\0".as_ptr() as *const c_char);
    snd_component_add((*chip).card, b"CS4362A\0".as_ptr() as *const c_char);
    cs5361_init(chip);
    claro_enable_hp(chip);
    snd_component_add((*chip).card, b"CS5361\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn eclaro_resume(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;

    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_ECLARO_CS4362A_NRESET | GPIO_ECLARO_FRONT_ENABLE);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, GPIO_ECLARO_CS4362A_NRESET | GPIO_ECLARO_FRONT_ENABLE);

    /* AK4396 chip 0 */
    ak4396_write(chip, 0, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_ACKS | AK4396_RSTN);
    ak4396_write(chip, 0, AK4396_CONTROL_2 as u8, (*data).ak4396_regs[0][AK4396_CONTROL_2]);
    ak4396_write(chip, 0, AK4396_CONTROL_3 as u8, AK4396_PCM);
    ak4396_write(chip, 0, AK4396_LCH_ATT as u8, (*chip).dac_volume[0].wrapping_mul(2));
    ak4396_write(chip, 0, AK4396_RCH_ATT as u8, (*chip).dac_volume[1].wrapping_mul(2));

    eclaro_cs4362a_registers_init(chip);

    cs5361_init(chip);
    claro_enable_hp(chip);
}

unsafe extern "C" fn stereo_resume(chip: *mut oxygen) {
    ak4396_registers_init(chip);
}

unsafe extern "C" fn set_ak4396_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;
    let mut value: u8;

    value = (*data).ak4396_regs[0][AK4396_CONTROL_2] & !AK4396_DFS_MASK;
    if params_rate(params) <= 54000 {
        value |= AK4396_DFS_NORMAL;
    } else if params_rate(params) <= 108000 {
        value |= AK4396_DFS_DOUBLE;
    } else {
        value |= AK4396_DFS_QUAD;
    }

    msleep(1); /* wait for the new MCLK to become stable */

    if value != (*data).ak4396_regs[0][AK4396_CONTROL_2] {
        i = 0;
        while i < (*data).dacs {
            ak4396_write(chip, i, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB);
            ak4396_write(chip, i, AK4396_CONTROL_2 as u8, value);
            ak4396_write(chip, i, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_RSTN);
            i += 1;
        }
    }
}

unsafe extern "C" fn eclaro_set_dac_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let data = (*chip).model_data as *mut generic_data;
    let mut ak_value: u8;
    let mut cs_fm: u8;

    ak_value = (*data).ak4396_regs[0][AK4396_CONTROL_2] & !AK4396_DFS_MASK;
    if params_rate(params) <= 54000 {
        ak_value |= AK4396_DFS_NORMAL;
        cs_fm = CS4362A_FM_SINGLE;
    } else if params_rate(params) <= 108000 {
        ak_value |= AK4396_DFS_DOUBLE;
        cs_fm = CS4362A_FM_DOUBLE;
    } else {
        ak_value |= AK4396_DFS_QUAD;
        cs_fm = CS4362A_FM_QUAD;
    }

    usleep_range(1000, 2000);

    if ak_value != (*data).ak4396_regs[0][AK4396_CONTROL_2] {
        ak4396_write(chip, 0, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_ACKS);
        ak4396_write(chip, 0, AK4396_CONTROL_2 as u8, ak_value);
        ak4396_write(chip, 0, AK4396_CONTROL_1 as u8, AK4396_DIF_24_MSB | AK4396_ACKS | AK4396_RSTN);
        (*data).ak4396_regs[0][AK4396_CONTROL_2] = ak_value;
    }

    /* Update CS4362A FM mode for all three DAC pairs */
    cs_fm |= (*data).cs4362a_regs[6] & !CS4362A_FM_MASK;
    eclaro_cs4362a_write_cached(chip, 6, cs_fm);
    eclaro_cs4362a_write_cached(chip, 12, cs_fm);
    cs_fm &= CS4362A_FM_MASK;
    cs_fm |= (*data).cs4362a_regs[9] & !CS4362A_FM_MASK;
    eclaro_cs4362a_write_cached(chip, 9, cs_fm);
}

unsafe extern "C" fn update_eclaro_volume(chip: *mut oxygen) {
    let mute: u8 = if (*chip).dac_mute { CS4362A_MUTE } else { 0 };

    ak4396_write_cached(chip, 0, AK4396_LCH_ATT as u8, (*chip).dac_volume[0].wrapping_mul(2));
    ak4396_write_cached(chip, 0, AK4396_RCH_ATT as u8, (*chip).dac_volume[1].wrapping_mul(2));

    /* CS4362A attenuation is inverse: 0 = 0 dB, 127 = max attenuation.
     * Pair 1 (regs 7/8) is wired to the side outputs (ALSA ch 6/7);
     * pair 3 (regs 13/14) is wired to the rear outputs (ALSA ch 2/3).
     */
    eclaro_cs4362a_write_cached(chip, 7, mute | (127u8.wrapping_sub((*chip).dac_volume[6])));
    eclaro_cs4362a_write_cached(chip, 8, mute | (127u8.wrapping_sub((*chip).dac_volume[7])));
    eclaro_cs4362a_write_cached(chip, 10, mute | (127u8.wrapping_sub((*chip).dac_volume[4])));
    eclaro_cs4362a_write_cached(chip, 11, mute | (127u8.wrapping_sub((*chip).dac_volume[5])));
    eclaro_cs4362a_write_cached(chip, 13, mute | (127u8.wrapping_sub((*chip).dac_volume[2])));
    eclaro_cs4362a_write_cached(chip, 14, mute | (127u8.wrapping_sub((*chip).dac_volume[3])));
}

unsafe extern "C" fn update_eclaro_mute(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;
    let mut value: u8;

    value = (*data).ak4396_regs[0][AK4396_CONTROL_2] & !AK4396_SMUTE;
    if (*chip).dac_mute {
        value |= AK4396_SMUTE;
    }
    ak4396_write_cached(chip, 0, AK4396_CONTROL_2 as u8, value);

    /* Re-apply volume+mute to CS4362A so the mute bit is set correctly */
    update_eclaro_volume(chip);
}

unsafe extern "C" fn update_ak4396_volume(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;

    i = 0;
    while i < (*data).dacs {
        ak4396_write_cached(chip, i, AK4396_LCH_ATT as u8, (*chip).dac_volume[(i * 2) as usize]);
        ak4396_write_cached(chip, i, AK4396_RCH_ATT as u8, (*chip).dac_volume[(i * 2 + 1) as usize]);
        i += 1;
    }
}

unsafe extern "C" fn update_ak4396_mute(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;
    let mut value: u8;

    value = (*data).ak4396_regs[0][AK4396_CONTROL_2] & !AK4396_SMUTE;
    if (*chip).dac_mute {
        value |= AK4396_SMUTE;
    }
    i = 0;
    while i < (*data).dacs {
        ak4396_write_cached(chip, i, AK4396_CONTROL_2 as u8, value);
        i += 1;
    }
}

unsafe extern "C" fn set_wm8785_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let data = (*chip).model_data as *mut generic_data;
    let mut value: c_uint;

    value = WM8785_MCR_SLAVE | WM8785_FORMAT_LJUST;
    if params_rate(params) <= 48000 {
        value |= WM8785_OSR_SINGLE;
    } else if params_rate(params) <= 96000 {
        value |= WM8785_OSR_DOUBLE;
    } else {
        value |= WM8785_OSR_QUAD;
    }
    if value != (*data).wm8785_regs[0] as c_uint {
        wm8785_write(chip, WM8785_R7, 0);
        wm8785_write(chip, WM8785_R0, value);
        wm8785_write(chip, WM8785_R2, (*data).wm8785_regs[2] as c_uint);
    }
}

unsafe extern "C" fn set_ak5385_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let value: c_uint;

    if params_rate(params) <= 54000 {
        value = GPIO_AK5385_DFS_NORMAL as c_uint;
    } else if params_rate(params) <= 108000 {
        value = GPIO_AK5385_DFS_DOUBLE as c_uint;
    } else {
        value = GPIO_AK5385_DFS_QUAD as c_uint;
    }
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA, value as u16, GPIO_AK5385_DFS_MASK);
}

unsafe extern "C" fn set_no_params(_chip: *mut oxygen, _params: *mut snd_pcm_hw_params) {}

unsafe extern "C" fn rolloff_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [
        b"Sharp Roll-off\0".as_ptr() as *const c_char,
        b"Slow Roll-off\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn rolloff_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let data = (*chip).model_data as *mut generic_data;

    (*value).value.enumerated.item[0] =
        (((*data).ak4396_regs[0][AK4396_CONTROL_2] & AK4396_SLOW) != 0) as c_uint;
    0
}

unsafe extern "C" fn rolloff_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;
    let changed: c_int;
    let mut reg: u8;

    /* guard(mutex)(&chip->mutex); */
    reg = (*data).ak4396_regs[0][AK4396_CONTROL_2];
    if (*value).value.enumerated.item[0] != 0 {
        reg |= AK4396_SLOW;
    } else {
        reg &= !AK4396_SLOW;
    }
    changed = (reg != (*data).ak4396_regs[0][AK4396_CONTROL_2]) as c_int;
    if changed != 0 {
        i = 0;
        while i < (*data).dacs {
            ak4396_write(chip, i, AK4396_CONTROL_2 as u8, reg);
            i += 1;
        }
    }
    changed
}

static rolloff_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"DAC Filter Playback Enum\0".as_ptr() as *const c_char,
    info: Some(rolloff_info),
    get: Some(rolloff_get),
    put: Some(rolloff_put),
};

unsafe extern "C" fn hpf_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [
        b"None\0".as_ptr() as *const c_char,
        b"High-pass Filter\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn hpf_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let data = (*chip).model_data as *mut generic_data;

    (*value).value.enumerated.item[0] =
        (((*data).wm8785_regs[WM8785_R2 as usize] as c_uint & WM8785_HPFR) != 0) as c_uint;
    0
}

unsafe extern "C" fn hpf_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let data = (*chip).model_data as *mut generic_data;
    let mut reg: c_uint;
    let changed: c_int;

    /* guard(mutex)(&chip->mutex); */
    reg = (*data).wm8785_regs[WM8785_R2 as usize] as c_uint & !(WM8785_HPFR | WM8785_HPFL);
    if (*value).value.enumerated.item[0] != 0 {
        reg |= WM8785_HPFR | WM8785_HPFL;
    }
    changed = (reg != (*data).wm8785_regs[WM8785_R2 as usize] as c_uint) as c_int;
    if changed != 0 {
        wm8785_write(chip, WM8785_R2, reg);
    }
    changed
}

static hpf_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"ADC Filter Capture Enum\0".as_ptr() as *const c_char,
    info: Some(hpf_info),
    get: Some(hpf_get),
    put: Some(hpf_put),
};

unsafe extern "C" fn meridian_dig_source_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [
        b"On-board\0".as_ptr() as *const c_char,
        b"Extension\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn claro_dig_source_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [
        b"Optical\0".as_ptr() as *const c_char,
        b"Coaxial\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe extern "C" fn meridian_dig_source_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;

    (*value).value.enumerated.item[0] =
        ((oxygen_read16(chip, OXYGEN_GPIO_DATA) & GPIO_MERIDIAN_DIG_EXT) != 0) as c_uint;
    0
}

unsafe extern "C" fn claro_dig_source_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;

    (*value).value.enumerated.item[0] =
        ((oxygen_read16(chip, OXYGEN_GPIO_DATA) & GPIO_CLARO_DIG_COAX) != 0) as c_uint;
    0
}

unsafe extern "C" fn meridian_dig_source_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let old_reg: u16;
    let mut new_reg: u16;
    let changed: c_int;

    /* guard(mutex)(&chip->mutex); */
    old_reg = oxygen_read16(chip, OXYGEN_GPIO_DATA);
    new_reg = old_reg & !GPIO_MERIDIAN_DIG_MASK;
    if (*value).value.enumerated.item[0] == 0 {
        new_reg |= GPIO_MERIDIAN_DIG_BOARD;
    } else {
        new_reg |= GPIO_MERIDIAN_DIG_EXT;
    }
    changed = (new_reg != old_reg) as c_int;
    if changed != 0 {
        oxygen_write16(chip, OXYGEN_GPIO_DATA, new_reg);
    }
    changed
}

unsafe extern "C" fn claro_dig_source_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data;
    let old_reg: u16;
    let mut new_reg: u16;
    let changed: c_int;

    /* guard(mutex)(&chip->mutex); */
    old_reg = oxygen_read16(chip, OXYGEN_GPIO_DATA);
    new_reg = old_reg & !GPIO_CLARO_DIG_COAX;
    if (*value).value.enumerated.item[0] != 0 {
        new_reg |= GPIO_CLARO_DIG_COAX;
    }
    changed = (new_reg != old_reg) as c_int;
    if changed != 0 {
        oxygen_write16(chip, OXYGEN_GPIO_DATA, new_reg);
    }
    changed
}

static meridian_dig_source_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"IEC958 Source Capture Enum\0".as_ptr() as *const c_char,
    info: Some(meridian_dig_source_info),
    get: Some(meridian_dig_source_get),
    put: Some(meridian_dig_source_put),
};

static claro_dig_source_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"IEC958 Source Capture Enum\0".as_ptr() as *const c_char,
    info: Some(claro_dig_source_info),
    get: Some(claro_dig_source_get),
    put: Some(claro_dig_source_put),
};

unsafe extern "C" fn generic_mixer_init(chip: *mut oxygen) -> c_int {
    snd_ctl_add((*chip).card, snd_ctl_new1(&rolloff_control, chip as *mut c_void))
}

unsafe extern "C" fn generic_wm8785_mixer_init(chip: *mut oxygen) -> c_int {
    let mut err: c_int;

    err = generic_mixer_init(chip);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&hpf_control, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn meridian_mixer_init(chip: *mut oxygen) -> c_int {
    let mut err: c_int;

    err = generic_mixer_init(chip);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&meridian_dig_source_control, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn claro_mixer_init(chip: *mut oxygen) -> c_int {
    let mut err: c_int;

    err = generic_wm8785_mixer_init(chip);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&claro_dig_source_control, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn claro_halo_mixer_init(chip: *mut oxygen) -> c_int {
    let mut err: c_int;

    err = generic_mixer_init(chip);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&claro_dig_source_control, chip as *mut c_void));
    if err < 0 {
        return err;
    }
    0
}

unsafe extern "C" fn dump_ak4396_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    let data = (*chip).model_data as *mut generic_data;
    let mut dac: c_uint;
    let mut i: c_uint;

    dac = 0;
    while dac < (*data).dacs {
        snd_iprintf(buffer, b"\nAK4396 %u:\0".as_ptr() as *const c_char, dac + 1);
        i = 0;
        while i < 5 {
            snd_iprintf(buffer, b" %02x\0".as_ptr() as *const c_char, (*data).ak4396_regs[dac as usize][i as usize] as c_uint);
            i += 1;
        }
        dac += 1;
    }
    snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn dump_wm8785_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    let data = (*chip).model_data as *mut generic_data;
    let mut i: c_uint;

    snd_iprintf(buffer, b"\nWM8785:\0".as_ptr() as *const c_char);
    i = 0;
    while i < 3 {
        snd_iprintf(buffer, b" %03x\0".as_ptr() as *const c_char, (*data).wm8785_regs[i as usize] as c_uint);
        i += 1;
    }
    snd_iprintf(buffer, b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn dump_oxygen_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    dump_ak4396_registers(chip, buffer);
    dump_wm8785_registers(chip, buffer);
}

static ak4396_db_scale: [c_uint; 2] = [0, 0]; /* DECLARE_TLV_DB_LINEAR(ak4396_db_scale, TLV_DB_GAIN_MUTE, 0) */
/* CS4362A: 0.5 dB/step, raw=127 -> 0 dB, raw=0 -> -63.5 dB */
static eclaro_db_scale: [c_uint; 4] = [0, (-6350i32) as c_uint, 50, 0]; /* DECLARE_TLV_DB_SCALE(eclaro_db_scale, -6350, 50, 0) */

static model_generic: oxygen_model = oxygen_model {
    shortname: b"C-Media CMI8788\0".as_ptr() as *const c_char,
    longname: b"C-Media Oxygen HD Audio\0".as_ptr() as *const c_char,
    chip: b"CMI8788\0".as_ptr() as *const c_char,
    init: Some(generic_init),
    mixer_init: Some(generic_wm8785_mixer_init),
    cleanup: Some(generic_cleanup),
    suspend: None,
    resume: Some(generic_resume),
    set_dac_params: Some(set_ak4396_params),
    set_adc_params: Some(set_wm8785_params),
    update_dac_volume: Some(update_ak4396_volume),
    update_dac_mute: Some(update_ak4396_mute),
    dump_registers: Some(dump_oxygen_registers),
    dac_tlv: ak4396_db_scale.as_ptr(),
    model_data_size: core::mem::size_of::<generic_data>(),
    device_config: PLAYBACK_0_TO_I2S
        | PLAYBACK_1_TO_SPDIF
        | PLAYBACK_2_TO_AC97_1
        | CAPTURE_0_FROM_I2S_1
        | CAPTURE_1_FROM_SPDIF
        | CAPTURE_2_FROM_AC97_1
        | AC97_CD_INPUT,
    dac_channels_pcm: 8,
    dac_channels_mixer: 8,
    dac_volume_min: 0,
    dac_volume_max: 255,
    function_flags: OXYGEN_FUNCTION_SPI | OXYGEN_FUNCTION_ENABLE_SPI_4_5,
    dac_mclks: OXYGEN_MCLKS(256, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 256, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    misc_flags: 0,
};

unsafe extern "C" fn get_oxygen_model(chip: *mut oxygen, pci_id: *const pci_device_id) -> c_int {
    static names: [*const c_char; 12] = [
        core::ptr::null(),
        b"AuzenTech X-Meridian\0".as_ptr() as *const c_char,
        b"AuzenTech X-Meridian 2G\0".as_ptr() as *const c_char,
        b"HT-Omega Claro\0".as_ptr() as *const c_char,
        b"HT-Omega Claro halo\0".as_ptr() as *const c_char,
        b"HT-Omega eClaro\0".as_ptr() as *const c_char,
        b"TempoTec HiFier Fantasia\0".as_ptr() as *const c_char,
        b"TempoTec HiFier Serenade\0".as_ptr() as *const c_char,
        core::ptr::null(),
        b"CMI8787-HG2PCI\0".as_ptr() as *const c_char,
        b"Xonar DG\0".as_ptr() as *const c_char,
        b"Xonar DGX\0".as_ptr() as *const c_char,
    ];

    (*chip).model = model_generic;
    match (*pci_id).driver_data {
        MODEL_MERIDIAN | MODEL_MERIDIAN_2G => {
            (*chip).model.init = Some(meridian_init);
            (*chip).model.mixer_init = Some(meridian_mixer_init);
            (*chip).model.resume = Some(meridian_resume);
            (*chip).model.set_adc_params = Some(set_ak5385_params);
            (*chip).model.dump_registers = Some(dump_ak4396_registers);
            (*chip).model.device_config =
                PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF;
            if (*pci_id).driver_data == MODEL_MERIDIAN {
                (*chip).model.device_config |= AC97_CD_INPUT;
            }
        }
        MODEL_CLARO => {
            (*chip).model.init = Some(claro_init);
            (*chip).model.mixer_init = Some(claro_mixer_init);
            (*chip).model.cleanup = Some(claro_cleanup);
            (*chip).model.suspend = Some(claro_suspend);
            (*chip).model.resume = Some(claro_resume);
        }
        MODEL_CLARO_HALO => {
            (*chip).model.init = Some(claro_halo_init);
            (*chip).model.mixer_init = Some(claro_halo_mixer_init);
            (*chip).model.cleanup = Some(claro_cleanup);
            (*chip).model.suspend = Some(claro_suspend);
            (*chip).model.resume = Some(claro_resume);
            (*chip).model.set_adc_params = Some(set_ak5385_params);
            (*chip).model.dump_registers = Some(dump_ak4396_registers);
            (*chip).model.device_config =
                PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF;
        }
        MODEL_ECLARO => {
            (*chip).model.init = Some(eclaro_init);
            (*chip).model.mixer_init = Some(generic_mixer_init);
            (*chip).model.cleanup = Some(claro_cleanup);
            (*chip).model.suspend = Some(claro_suspend);
            (*chip).model.resume = Some(eclaro_resume);
            (*chip).model.set_dac_params = Some(eclaro_set_dac_params);
            (*chip).model.set_adc_params = Some(set_cs5361_params);
            (*chip).model.update_dac_volume = Some(update_eclaro_volume);
            (*chip).model.update_dac_mute = Some(update_eclaro_mute);
            (*chip).model.dump_registers = Some(dump_ak4396_registers);
            (*chip).model.device_config =
                PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF;
            (*chip).model.function_flags = OXYGEN_FUNCTION_SPI | OXYGEN_FUNCTION_ENABLE_SPI_4_5;
            (*chip).model.dac_mclks = OXYGEN_MCLKS(256, 128, 128);
            (*chip).model.dac_volume_min = 0;
            (*chip).model.dac_volume_max = 127;
            (*chip).model.dac_tlv = eclaro_db_scale.as_ptr();
        }
        MODEL_FANTASIA | MODEL_SERENADE | MODEL_2CH_OUTPUT | MODEL_HG2PCI => {
            (*chip).model.shortname = b"C-Media CMI8787\0".as_ptr() as *const c_char;
            (*chip).model.chip = b"CMI8787\0".as_ptr() as *const c_char;
            if (*pci_id).driver_data == MODEL_FANTASIA {
                (*chip).model.init = Some(fantasia_init);
            } else {
                (*chip).model.init = Some(stereo_output_init);
            }
            (*chip).model.resume = Some(stereo_resume);
            (*chip).model.mixer_init = Some(generic_mixer_init);
            (*chip).model.set_adc_params = Some(set_no_params);
            (*chip).model.dump_registers = Some(dump_ak4396_registers);
            (*chip).model.device_config = PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF;
            if (*pci_id).driver_data == MODEL_FANTASIA {
                (*chip).model.device_config |= CAPTURE_0_FROM_I2S_1;
                (*chip).model.adc_mclks = OXYGEN_MCLKS(256, 128, 128);
            }
            (*chip).model.dac_channels_pcm = 2;
            (*chip).model.dac_channels_mixer = 2;
        }
        MODEL_XONAR_DG | MODEL_XONAR_DGX => {
            (*chip).model = model_xonar_dg;
        }
        _ => {}
    }
    if (*pci_id).driver_data == MODEL_MERIDIAN
        || (*pci_id).driver_data == MODEL_MERIDIAN_2G
        || (*pci_id).driver_data == MODEL_CLARO_HALO
    {
        (*chip).model.misc_flags = OXYGEN_MISC_MIDI;
        (*chip).model.device_config |= MIDI_OUTPUT | MIDI_INPUT;
    }
    if (*pci_id).driver_data < names.len() && !names[(*pci_id).driver_data].is_null() {
        (*chip).model.shortname = names[(*pci_id).driver_data];
    }
    0
}

unsafe extern "C" fn generic_oxygen_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
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
        &mut THIS_MODULE as *mut c_void,
        oxygen_ids.as_ptr(),
        get_oxygen_model,
    );
    if err >= 0 {
        dev += 1;
    }
    err
}

static mut oxygen_driver: pci_driver = pci_driver {
    name: b"KBUILD_MODNAME\0".as_ptr() as *const c_char,
    id_table: unsafe { oxygen_ids.as_ptr() },
    probe: Some(generic_oxygen_probe),
    driver: pci_driver_driver {
        pm: unsafe { &mut oxygen_pci_pm as *mut c_void },
    },
};

/* module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "card index");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "enable card");
 * MODULE_DEVICE_TABLE(pci, oxygen_ids);
 * module_pci_driver(oxygen_driver);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
