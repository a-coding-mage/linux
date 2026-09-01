// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for Terratec EWS88MT/D, EWX24/96, DMX 6Fire
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 *                    2002 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// Dependencies originally provided by Linux/ALSA headers and local ice1712/ews headers.
extern "C" {
    fn udelay(usecs: c_ulong);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_ice1712_write(ice: *mut snd_ice1712, reg: c_uint, val: c_uint);
    fn snd_ice1712_read(ice: *mut snd_ice1712, reg: c_uint) -> c_uint;
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_gpio_write_bits(ice: *mut snd_ice1712, mask: c_uint, bits: c_uint);
    fn snd_i2c_lock(bus: *mut snd_i2c_bus);
    fn snd_i2c_unlock(bus: *mut snd_i2c_bus);
    fn snd_i2c_readbytes(dev: *mut snd_i2c_device, bytes: *mut u8, count: c_int) -> c_int;
    fn snd_i2c_sendbytes(dev: *mut snd_i2c_device, bytes: *const u8, count: c_int) -> c_int;
    fn snd_i2c_bus_create(card: *mut snd_card, name: *const c_char, ops: *mut c_void, bus: *mut *mut snd_i2c_bus) -> c_int;
    fn snd_i2c_device_create(bus: *mut snd_i2c_bus, name: *const c_char, addr: c_uint, dev: *mut *mut snd_i2c_device) -> c_int;
    fn snd_cs8404_decode_spdif_bits(dst: *mut snd_aes_iec958, bits: c_uint);
    fn snd_cs8404_encode_spdif_bits(src: *mut snd_aes_iec958) -> c_uint;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ice1712_init_cs8427(ice: *mut snd_ice1712, addr: c_uint) -> c_int;
    fn snd_cs8427_reg_write(cs8427: *mut c_void, reg: c_uint, val: c_uint);
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ice1712;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ice1712_spdif_build_controls(ice: *mut snd_ice1712) -> c_int;
    fn snd_ice1712_akm4xxx_init(ak: *mut snd_akm4xxx, template: *const snd_akm4xxx, priv_: *const snd_ak4xxx_private, ice: *mut snd_ice1712) -> c_int;
    fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;

const EWS_I2C_CS8404: usize = 0;
const EWS_I2C_PCF1: usize = 1;
const EWS_I2C_PCF2: usize = 2;
const EWS_I2C_88D: usize = 0;
const EWS_I2C_6FIRE: usize = 0;

#[repr(C)]
pub struct snd_i2c_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_i2c_bus_hw_ops {
    pub bit: *const snd_i2c_bit_ops,
}

#[repr(C)]
pub struct snd_i2c_bus {
    pub private_data: *mut snd_ice1712,
    pub hw_ops: snd_i2c_bus_hw_ops,
}

/* additional i2c devices for EWS boards */
#[repr(C)]
pub struct ews_spec {
    pub i2cdevs: [*mut snd_i2c_device; 3],
}

#[repr(C)]
pub struct snd_i2c_bit_ops {
    pub start: Option<unsafe extern "C" fn(*mut snd_i2c_bus)>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_i2c_bus)>,
    pub direction: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int, c_int)>,
    pub setlines: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int, c_int)>,
    pub getclock: Option<unsafe extern "C" fn(*mut snd_i2c_bus) -> c_int>,
    pub getdata: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_ice1712_eeprom {
    pub subvendor: c_uint,
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    pub direction: u8,
}

#[repr(C)]
pub struct snd_ice1712_spdif_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
    pub setup_rate: Option<unsafe extern "C" fn(*mut snd_ice1712, c_int)>,
    pub default_get: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_ctl_elem_value)>,
    pub default_put: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_ctl_elem_value) -> c_int>,
    pub stream_get: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_ctl_elem_value)>,
    pub stream_put: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_ice1712_spdif {
    pub cs8403_bits: c_uint,
    pub cs8403_stream_bits: c_uint,
    pub ops: snd_ice1712_spdif_ops,
    pub stream_ctl: *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub private_data: *mut c_void,
    pub spec: *mut ews_spec,
    pub i2c: *mut snd_i2c_bus,
    pub card: *mut snd_card,
    pub eeprom: snd_ice1712_eeprom,
    pub gpio: snd_ice1712_gpio,
    pub spdif: snd_ice1712_spdif,
    pub reg_lock: c_ulong,
    pub playback_pro_substream: *mut snd_pcm_substream,
    pub cs8427: *mut c_void,
    pub num_total_dacs: c_uint,
    pub num_total_adcs: c_uint,
    pub akm: *mut snd_akm4xxx,
    pub akm_codecs: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_aes_iec958 {
    _private: [u8; 0],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_aes_iec958,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub count: c_uint,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_akm4xxx_ops {
    pub lock: Option<unsafe extern "C" fn(*mut snd_akm4xxx, c_int)>,
    pub unlock: Option<unsafe extern "C" fn(*mut snd_akm4xxx, c_int)>,
}

#[repr(C)]
pub struct snd_akm4xxx {
    pub num_adcs: c_uint,
    pub num_dacs: c_uint,
    pub type_: c_uint,
    pub ops: snd_akm4xxx_ops,
    pub private_data: [*mut snd_ice1712; 4],
    pub private_value: [c_ulong; 4],
}

#[repr(C)]
pub struct snd_ak4xxx_private {
    pub caddr: c_uint,
    pub cif: c_uint,
    pub data_mask: c_uint,
    pub clk_mask: c_uint,
    pub cs_mask: c_uint,
    pub cs_addr: c_uint,
    pub cs_none: c_uint,
    pub add_flags: c_uint,
    pub mask_flags: c_uint,
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub mpu401_1_name: *const c_char,
    pub mpu401_2_name: *const c_char,
    pub mpu401_2_info_flags: c_uint,
}

// Constants from the included headers.
extern "C" {
    static ICE1712_EWX2496_SERIAL_CLOCK: c_uint;
    static ICE1712_EWX2496_SERIAL_DATA: c_uint;
    static ICE1712_EWX2496_RW: c_uint;
    static ICE1712_EWX2496_AK4524_CS: c_uint;
    static ICE1712_6FIRE_AK4524_CS_MASK: c_uint;
    static ICE1712_EWS88_SERIAL_DATA: c_uint;
    static ICE1712_EWS88_SERIAL_CLOCK: c_uint;
    static ICE1712_EWS88_RW: c_uint;
    static ICE1712_6FIRE_SERIAL_DATA: c_uint;
    static ICE1712_6FIRE_SERIAL_CLOCK: c_uint;
    static ICE1712_6FIRE_RW: c_uint;
    static ICE1712_EWS88MT_OUTPUT_SENSE: c_uint;
    static ICE1712_EWX2496_AIN_SEL: c_uint;
    static ICE1712_EWX2496_AOUT_SEL: c_uint;
    static ICE1712_IREG_GPIO_DATA: c_uint;
    static ICE1712_IREG_GPIO_WRITE_MASK: c_uint;
    static ICE1712_IREG_GPIO_DIRECTION: c_uint;
    static ICE1712_SUBDEVICE_EWX2496: c_uint;
    static ICE1712_SUBDEVICE_DMX6FIRE: c_uint;
    static ICE1712_SUBDEVICE_EWS88MT: c_uint;
    static ICE1712_SUBDEVICE_EWS88MT_NEW: c_uint;
    static ICE1712_SUBDEVICE_PHASE88: c_uint;
    static ICE1712_SUBDEVICE_TS88: c_uint;
    static ICE1712_SUBDEVICE_EWS88D: c_uint;
    static ICE1712_6FIRE_PCF9554_ADDR: c_uint;
    static ICE1712_EWS88MT_CS8404_ADDR: c_uint;
    static ICE1712_EWS88MT_INPUT_ADDR: c_uint;
    static ICE1712_EWS88MT_OUTPUT_ADDR: c_uint;
    static ICE1712_EWS88D_PCF_ADDR: c_uint;
    static CS8427_BASE_ADDR: c_uint;
    static ICE1712_6FIRE_CS8427_ADDR: c_uint;
    static CS8427_REG_RECVERRMASK: c_uint;
    static CS8427_UNLOCK: c_uint;
    static CS8427_CONF: c_uint;
    static CS8427_BIP: c_uint;
    static CS8427_PAR: c_uint;
    static SND_AK4524: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static MPU401_INFO_OUTPUT: c_uint;
}

unsafe fn snd_bug_on(cond: bool) -> bool {
    cond
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T
}

/*
 * access via i2c mode (for EWX 24/96, EWS 88MT&D)
 */

/* send SDA and SCL */
unsafe extern "C" fn ewx_i2c_setlines(bus: *mut snd_i2c_bus, clk: c_int, data: c_int) {
    let ice = (*bus).private_data;
    let mut tmp: u8 = 0;
    if clk != 0 {
        tmp |= ICE1712_EWX2496_SERIAL_CLOCK as u8;
    }
    if data != 0 {
        tmp |= ICE1712_EWX2496_SERIAL_DATA as u8;
    }
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp as c_uint);
    udelay(5);
}

unsafe extern "C" fn ewx_i2c_getclock(bus: *mut snd_i2c_bus) -> c_int {
    let ice = (*bus).private_data;
    if snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) & ICE1712_EWX2496_SERIAL_CLOCK != 0 { 1 } else { 0 }
}

unsafe extern "C" fn ewx_i2c_getdata(bus: *mut snd_i2c_bus, ack: c_int) -> c_int {
    let ice = (*bus).private_data;
    let bit: c_int;
    /* set RW pin to low */
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !ICE1712_EWX2496_RW);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, 0);
    if ack != 0 {
        udelay(5);
    }
    bit = if snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) & ICE1712_EWX2496_SERIAL_DATA != 0 { 1 } else { 0 };
    /* set RW pin to high */
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, ICE1712_EWX2496_RW);
    /* reset write mask */
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !ICE1712_EWX2496_SERIAL_CLOCK);
    bit
}

unsafe extern "C" fn ewx_i2c_start(bus: *mut snd_i2c_bus) {
    let ice = (*bus).private_data;
    let mut mask: u8;

    snd_ice1712_save_gpio_status(ice);
    /* set RW high */
    mask = ICE1712_EWX2496_RW as u8;
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWX2496 => {
            mask |= ICE1712_EWX2496_AK4524_CS as u8; /* CS high also */
        }
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            mask |= ICE1712_6FIRE_AK4524_CS_MASK as u8; /* CS high also */
        }
        _ => {}
    }
    snd_ice1712_gpio_write_bits(ice, mask as c_uint, mask as c_uint);
}

unsafe extern "C" fn ewx_i2c_stop(bus: *mut snd_i2c_bus) {
    let ice = (*bus).private_data;
    snd_ice1712_restore_gpio_status(ice);
}

unsafe extern "C" fn ewx_i2c_direction(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    let ice = (*bus).private_data;
    let mut mask: u8 = 0;

    if clock != 0 {
        mask |= ICE1712_EWX2496_SERIAL_CLOCK as u8; /* write SCL */
    }
    if data != 0 {
        mask |= ICE1712_EWX2496_SERIAL_DATA as u8; /* write SDA */
    }
    (*ice).gpio.direction &= !((ICE1712_EWX2496_SERIAL_CLOCK | ICE1712_EWX2496_SERIAL_DATA) as u8);
    (*ice).gpio.direction |= mask;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DIRECTION, (*ice).gpio.direction as c_uint);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !(mask as c_uint));
}

static mut snd_ice1712_ewx_cs8427_bit_ops: snd_i2c_bit_ops = snd_i2c_bit_ops {
    start: Some(ewx_i2c_start),
    stop: Some(ewx_i2c_stop),
    direction: Some(ewx_i2c_direction),
    setlines: Some(ewx_i2c_setlines),
    getclock: Some(ewx_i2c_getclock),
    getdata: Some(ewx_i2c_getdata),
};

/*
 * AK4524 access
 */

/* AK4524 chip select; address 0x48 bit 0-3 */
unsafe extern "C" fn snd_ice1712_ews88mt_chip_select(ice: *mut snd_ice1712, chip_mask: c_int) -> c_int {
    let spec = (*ice).spec;
    let mut data: u8 = 0;
    let mut ndata: u8;

    if snd_bug_on(chip_mask < 0 || chip_mask > 0x0f) {
        return -EINVAL;
    }
    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_PCF2], &mut data, 1) != 1 {
        goto_error_ews88mt_chip_select(ice);
        return -EIO;
    }
    ndata = (data & 0xf0) | chip_mask as u8;
    if ndata != data {
        if snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_PCF2], &ndata, 1) != 1 {
            goto_error_ews88mt_chip_select(ice);
            return -EIO;
        }
    }
    snd_i2c_unlock((*ice).i2c);
    0
}

unsafe fn goto_error_ews88mt_chip_select(ice: *mut snd_ice1712) {
    snd_i2c_unlock((*ice).i2c);
    dev_err((*(*ice).card).dev, c"AK4524 chip select failed, check cable to the front module\n".as_ptr());
}

/* start callback for EWS88MT, needs to select a certain chip mask */
unsafe extern "C" fn ews88mt_ak4524_lock(ak: *mut snd_akm4xxx, chip: c_int) {
    let ice = (*ak).private_data[0];
    let tmp: u8;
    /* assert AK4524 CS */
    if snd_ice1712_ews88mt_chip_select(ice, (!(1 << chip) & 0x0f) as c_int) < 0 {
        dev_err((*(*ice).card).dev, c"fatal error (ews88mt chip select)\n".as_ptr());
    }
    snd_ice1712_save_gpio_status(ice);
    tmp = (ICE1712_EWS88_SERIAL_DATA | ICE1712_EWS88_SERIAL_CLOCK | ICE1712_EWS88_RW) as u8;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DIRECTION, ((*ice).gpio.direction | tmp) as c_uint);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !(tmp as c_uint));
}

/* stop callback for EWS88MT, needs to deselect chip mask */
unsafe extern "C" fn ews88mt_ak4524_unlock(ak: *mut snd_akm4xxx, _chip: c_int) {
    let ice = (*ak).private_data[0];
    snd_ice1712_restore_gpio_status(ice);
    udelay(1);
    snd_ice1712_ews88mt_chip_select(ice, 0x0f);
}

/* start callback for EWX24/96 */
unsafe extern "C" fn ewx2496_ak4524_lock(ak: *mut snd_akm4xxx, _chip: c_int) {
    let ice = (*ak).private_data[0];
    let tmp: u8;
    snd_ice1712_save_gpio_status(ice);
    tmp = (ICE1712_EWX2496_SERIAL_DATA
        | ICE1712_EWX2496_SERIAL_CLOCK
        | ICE1712_EWX2496_AK4524_CS
        | ICE1712_EWX2496_RW) as u8;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DIRECTION, ((*ice).gpio.direction | tmp) as c_uint);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !(tmp as c_uint));
}

/* start callback for DMX 6fire */
unsafe extern "C" fn dmx6fire_ak4524_lock(ak: *mut snd_akm4xxx, chip: c_int) {
    let priv_ = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice = (*ak).private_data[0];
    let mut tmp: u8;
    snd_ice1712_save_gpio_status(ice);
    tmp = ((1 << chip) as c_uint & ICE1712_6FIRE_AK4524_CS_MASK) as u8;
    (*priv_).cs_addr = tmp as c_uint;
    (*priv_).cs_mask = (*priv_).cs_addr;
    tmp |= (ICE1712_6FIRE_SERIAL_DATA | ICE1712_6FIRE_SERIAL_CLOCK | ICE1712_6FIRE_RW) as u8;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DIRECTION, ((*ice).gpio.direction | tmp) as c_uint);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !(tmp as c_uint));
}

/*
 * CS8404 interface on EWS88MT/D
 */

unsafe extern "C" fn snd_ice1712_ews_cs8404_spdif_write(ice: *mut snd_ice1712, bits: u8) {
    let spec = (*ice).spec;
    let mut bytes: [u8; 2] = [0; 2];

    snd_i2c_lock((*ice).i2c);
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88 =>
        {
            if snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_CS8404], &bits, 1) != 1 {
                snd_i2c_unlock((*ice).i2c);
                return;
            }
        }
        x if x == ICE1712_SUBDEVICE_EWS88D => {
            if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_88D], bytes.as_mut_ptr(), 2) != 2 {
                snd_i2c_unlock((*ice).i2c);
                return;
            }
            if bits != bytes[1] {
                bytes[1] = bits;
                if snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_88D], bytes.as_ptr(), 2) != 2 {
                    snd_i2c_unlock((*ice).i2c);
                    return;
                }
            }
        }
        _ => {}
    }
    snd_i2c_unlock((*ice).i2c);
}

/*
 */

unsafe extern "C" fn ews88_spdif_default_get(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) {
    snd_cs8404_decode_spdif_bits(&mut (*ucontrol).value.iec958, (*ice).spdif.cs8403_bits);
}

unsafe extern "C" fn ews88_spdif_default_put(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let val: c_uint;
    let change: c_int;

    val = snd_cs8404_encode_spdif_bits(&mut (*ucontrol).value.iec958);
    change = if (*ice).spdif.cs8403_bits != val { 1 } else { 0 };
    (*ice).spdif.cs8403_bits = val;
    if change == 0 || !(*ice).playback_pro_substream.is_null() {
        return change;
    }
    snd_ice1712_ews_cs8404_spdif_write(ice, val as u8);
    change
}

unsafe extern "C" fn ews88_spdif_stream_get(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) {
    snd_cs8404_decode_spdif_bits(&mut (*ucontrol).value.iec958, (*ice).spdif.cs8403_stream_bits);
}

unsafe extern "C" fn ews88_spdif_stream_put(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let val: c_uint;
    let change: c_int;

    val = snd_cs8404_encode_spdif_bits(&mut (*ucontrol).value.iec958);
    change = if (*ice).spdif.cs8403_stream_bits != val { 1 } else { 0 };
    (*ice).spdif.cs8403_stream_bits = val;
    if change == 0 || !(*ice).playback_pro_substream.is_null() {
        return change;
    }
    snd_ice1712_ews_cs8404_spdif_write(ice, val as u8);
    change
}

/* open callback */
unsafe extern "C" fn ews88_open_spdif(ice: *mut snd_ice1712, _substream: *mut snd_pcm_substream) {
    (*ice).spdif.cs8403_stream_bits = (*ice).spdif.cs8403_bits;
}

/* set up SPDIF for EWS88MT / EWS88D */
unsafe extern "C" fn ews88_setup_spdif(ice: *mut snd_ice1712, rate: c_int) {
    let mut tmp: u8;
    let change: c_int;

    tmp = (*ice).spdif.cs8403_stream_bits as u8;
    if tmp & 0x10 != 0 {
        /* consumer */
        tmp &= if tmp & 0x01 != 0 { !0x06 } else { !0x60 };
    }
    match rate {
        32000 => tmp |= if tmp & 0x01 != 0 { 0x02 } else { 0x00 },
        44100 => tmp |= if tmp & 0x01 != 0 { 0x06 } else { 0x40 },
        48000 => tmp |= if tmp & 0x01 != 0 { 0x04 } else { 0x20 },
        _ => tmp |= if tmp & 0x01 != 0 { 0x06 } else { 0x40 },
    }
    change = if (*ice).spdif.cs8403_stream_bits != tmp as c_uint { 1 } else { 0 };
    (*ice).spdif.cs8403_stream_bits = tmp as c_uint;
    if change != 0 {
        snd_ctl_notify((*ice).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ice).spdif.stream_ctl).id);
    }
    snd_ice1712_ews_cs8404_spdif_write(ice, tmp);
}

/*
 */
static akm_ews88mt: snd_akm4xxx = snd_akm4xxx {
    num_adcs: 8,
    num_dacs: 8,
    type_: unsafe { SND_AK4524 },
    ops: snd_akm4xxx_ops {
        lock: Some(ews88mt_ak4524_lock),
        unlock: Some(ews88mt_ak4524_unlock),
    },
    private_data: [ptr::null_mut(); 4],
    private_value: [0; 4],
};

static akm_ews88mt_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 1, /* CIF high */
    data_mask: unsafe { ICE1712_EWS88_SERIAL_DATA },
    clk_mask: unsafe { ICE1712_EWS88_SERIAL_CLOCK },
    cs_mask: 0,
    cs_addr: 0,
    cs_none: 0, /* no chip select on gpio */
    add_flags: unsafe { ICE1712_EWS88_RW }, /* set rw bit high */
    mask_flags: 0,
};

static akm_ewx2496: snd_akm4xxx = snd_akm4xxx {
    num_adcs: 2,
    num_dacs: 2,
    type_: unsafe { SND_AK4524 },
    ops: snd_akm4xxx_ops {
        lock: Some(ewx2496_ak4524_lock),
        unlock: None,
    },
    private_data: [ptr::null_mut(); 4],
    private_value: [0; 4],
};

static akm_ewx2496_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 1, /* CIF high */
    data_mask: unsafe { ICE1712_EWS88_SERIAL_DATA },
    clk_mask: unsafe { ICE1712_EWS88_SERIAL_CLOCK },
    cs_mask: unsafe { ICE1712_EWX2496_AK4524_CS },
    cs_addr: unsafe { ICE1712_EWX2496_AK4524_CS },
    cs_none: 0,
    add_flags: unsafe { ICE1712_EWS88_RW }, /* set rw bit high */
    mask_flags: 0,
};

static akm_6fire: snd_akm4xxx = snd_akm4xxx {
    num_adcs: 6,
    num_dacs: 6,
    type_: unsafe { SND_AK4524 },
    ops: snd_akm4xxx_ops {
        lock: Some(dmx6fire_ak4524_lock),
        unlock: None,
    },
    private_data: [ptr::null_mut(); 4],
    private_value: [0; 4],
};

static akm_6fire_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 1, /* CIF high */
    data_mask: unsafe { ICE1712_6FIRE_SERIAL_DATA },
    clk_mask: unsafe { ICE1712_6FIRE_SERIAL_CLOCK },
    cs_mask: 0,
    cs_addr: 0, /* set later */
    cs_none: 0,
    add_flags: unsafe { ICE1712_6FIRE_RW }, /* set rw bit high */
    mask_flags: 0,
};

/*
 * initialize the chip
 */

/* 6fire specific */
const PCF9554_REG_INPUT: u8 = 0;
const PCF9554_REG_OUTPUT: u8 = 1;
const PCF9554_REG_POLARITY: u8 = 2;
const PCF9554_REG_CONFIG: u8 = 3;

unsafe extern "C" fn snd_ice1712_ews_init(ice: *mut snd_ice1712) -> c_int {
    let mut err: c_int;
    let ak: *mut snd_akm4xxx;
    let spec: *mut ews_spec;

    /* set the analog DACs */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWX2496 => {
            (*ice).num_total_dacs = 2;
            (*ice).num_total_adcs = 2;
        }
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88 =>
        {
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 8;
        }
        x if x == ICE1712_SUBDEVICE_EWS88D => {
            /* Note: not analog but ADAT I/O */
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 8;
        }
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            (*ice).num_total_dacs = 6;
            (*ice).num_total_adcs = 6;
        }
        _ => {}
    }

    spec = kzalloc_obj::<ews_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec;

    /* create i2c */
    err = snd_i2c_bus_create((*ice).card, c"ICE1712 GPIO 1".as_ptr(), ptr::null_mut(), &mut (*ice).i2c);
    if err < 0 {
        dev_err((*(*ice).card).dev, c"unable to create I2C bus\n".as_ptr());
        return err;
    }
    (*(*ice).i2c).private_data = ice;
    (*(*ice).i2c).hw_ops.bit = &raw const snd_ice1712_ewx_cs8427_bit_ops;

    /* create i2c devices */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            err = snd_i2c_device_create((*ice).i2c, c"PCF9554".as_ptr(), ICE1712_6FIRE_PCF9554_ADDR, &mut (*spec).i2cdevs[EWS_I2C_6FIRE]);
            if err < 0 {
                dev_err((*(*ice).card).dev, c"PCF9554 initialization failed\n".as_ptr());
                return err;
            }
            snd_ice1712_6fire_write_pca(ice, PCF9554_REG_CONFIG, 0x80);
        }
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88 =>
        {
            err = snd_i2c_device_create((*ice).i2c, c"CS8404".as_ptr(), ICE1712_EWS88MT_CS8404_ADDR, &mut (*spec).i2cdevs[EWS_I2C_CS8404]);
            if err < 0 {
                return err;
            }
            err = snd_i2c_device_create((*ice).i2c, c"PCF8574 (1st)".as_ptr(), ICE1712_EWS88MT_INPUT_ADDR, &mut (*spec).i2cdevs[EWS_I2C_PCF1]);
            if err < 0 {
                return err;
            }
            err = snd_i2c_device_create((*ice).i2c, c"PCF8574 (2nd)".as_ptr(), ICE1712_EWS88MT_OUTPUT_ADDR, &mut (*spec).i2cdevs[EWS_I2C_PCF2]);
            if err < 0 {
                return err;
            }
            /* Check if the front module is connected */
            err = snd_ice1712_ews88mt_chip_select(ice, 0x0f);
            if err < 0 {
                return err;
            }
        }
        x if x == ICE1712_SUBDEVICE_EWS88D => {
            err = snd_i2c_device_create((*ice).i2c, c"PCF8575".as_ptr(), ICE1712_EWS88D_PCF_ADDR, &mut (*spec).i2cdevs[EWS_I2C_88D]);
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    /* set up SPDIF interface */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWX2496 => {
            err = snd_ice1712_init_cs8427(ice, CS8427_BASE_ADDR);
            if err < 0 {
                return err;
            }
            snd_cs8427_reg_write((*ice).cs8427, CS8427_REG_RECVERRMASK, CS8427_UNLOCK | CS8427_CONF | CS8427_BIP | CS8427_PAR);
        }
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            err = snd_ice1712_init_cs8427(ice, ICE1712_6FIRE_CS8427_ADDR);
            if err < 0 {
                return err;
            }
            snd_cs8427_reg_write((*ice).cs8427, CS8427_REG_RECVERRMASK, CS8427_UNLOCK | CS8427_CONF | CS8427_BIP | CS8427_PAR);
        }
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88
            || x == ICE1712_SUBDEVICE_EWS88D =>
        {
            /* set up CS8404 */
            (*ice).spdif.ops.open = Some(ews88_open_spdif);
            (*ice).spdif.ops.setup_rate = Some(ews88_setup_spdif);
            (*ice).spdif.ops.default_get = Some(ews88_spdif_default_get);
            (*ice).spdif.ops.default_put = Some(ews88_spdif_default_put);
            (*ice).spdif.ops.stream_get = Some(ews88_spdif_stream_get);
            (*ice).spdif.ops.stream_put = Some(ews88_spdif_stream_put);
            /* Set spdif defaults */
            snd_ice1712_ews_cs8404_spdif_write(ice, (*ice).spdif.cs8403_bits as u8);
        }
        _ => {}
    }

    /* no analog? */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWS88D => return 0,
        _ => {}
    }

    /* analog section */
    ak = kzalloc_obj::<snd_akm4xxx>();
    (*ice).akm = ak;
    if ak.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88 =>
        {
            err = snd_ice1712_akm4xxx_init(ak, &akm_ews88mt, &akm_ews88mt_priv, ice);
        }
        x if x == ICE1712_SUBDEVICE_EWX2496 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_ewx2496, &akm_ewx2496_priv, ice);
        }
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_6fire, &akm_6fire_priv, ice);
        }
        _ => {
            err = 0;
        }
    }

    err
}

/*
 * EWX 24/96 specific controls
 */

/* i/o sensitivity - this callback is shared among other devices, too */
unsafe extern "C" fn snd_ice1712_ewx_io_sense_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXT_0: &[u8] = b"+4dBu\0";
    static TEXT_1: &[u8] = b"-10dBV\0";
    let texts: [*const c_char; 2] = [TEXT_0.as_ptr() as *const c_char, TEXT_1.as_ptr() as *const c_char];
    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn snd_ice1712_ewx_io_sense_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let mask = ((*kcontrol).private_value & 0xff) as u8;

    snd_ice1712_save_gpio_status(ice);
    (*ucontrol).value.enumerated.item[0] = if snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) as u8 & mask != 0 { 1 } else { 0 };
    snd_ice1712_restore_gpio_status(ice);
    0
}

unsafe extern "C" fn snd_ice1712_ewx_io_sense_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let mask = ((*kcontrol).private_value & 0xff) as u8;
    let val: c_int;
    let mut nval: c_int;

    if (*kcontrol).private_value & (1 << 31) != 0 {
        return -EPERM;
    }
    nval = if (*ucontrol).value.enumerated.item[0] != 0 { mask as c_int } else { 0 };
    snd_ice1712_save_gpio_status(ice);
    val = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) as c_int;
    nval |= val & !(mask as c_int);
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, nval as c_uint);
    snd_ice1712_restore_gpio_status(ice);
    if val != nval { 1 } else { 0 }
}

static snd_ice1712_ewx2496_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
        name: c"Input Sensitivity Switch".as_ptr(),
        access: 0,
        info: Some(snd_ice1712_ewx_io_sense_info),
        get: Some(snd_ice1712_ewx_io_sense_get),
        put: Some(snd_ice1712_ewx_io_sense_put),
        count: 0,
        private_value: unsafe { ICE1712_EWX2496_AIN_SEL as c_ulong },
    },
    snd_kcontrol_new {
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
        name: c"Output Sensitivity Switch".as_ptr(),
        access: 0,
        info: Some(snd_ice1712_ewx_io_sense_info),
        get: Some(snd_ice1712_ewx_io_sense_get),
        put: Some(snd_ice1712_ewx_io_sense_put),
        count: 0,
        private_value: unsafe { ICE1712_EWX2496_AOUT_SEL as c_ulong },
    },
];

/*
 * EWS88MT specific controls
 */
/* analog output sensitivity;; address 0x48 bit 6 */
unsafe extern "C" fn snd_ice1712_ews88mt_output_sense_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let mut data: u8 = 0;

    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_PCF2], &mut data, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    (*ucontrol).value.enumerated.item[0] = if data as c_uint & ICE1712_EWS88MT_OUTPUT_SENSE != 0 { 1 } else { 0 }; /* high = -10dBV, low = +4dBu */
    0
}

/* analog output sensitivity;; address 0x48 bit 6 */
unsafe extern "C" fn snd_ice1712_ews88mt_output_sense_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let mut data: u8 = 0;
    let ndata: u8;

    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_PCF2], &mut data, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    ndata = ((data as c_uint & !ICE1712_EWS88MT_OUTPUT_SENSE)
        | if (*ucontrol).value.enumerated.item[0] != 0 { ICE1712_EWS88MT_OUTPUT_SENSE } else { 0 }) as u8;
    if ndata != data && snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_PCF2], &ndata, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    if ndata != data { 1 } else { 0 }
}

/* analog input sensitivity; address 0x46 */
unsafe extern "C" fn snd_ice1712_ews88mt_input_sense_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let channel = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let mut data: u8 = 0;

    if snd_bug_on(channel < 0 || channel > 7) {
        return 0;
    }
    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_PCF1], &mut data, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    /* reversed; high = +4dBu, low = -10dBV */
    (*ucontrol).value.enumerated.item[0] = if data & (1 << channel) as u8 != 0 { 0 } else { 1 };
    snd_i2c_unlock((*ice).i2c);
    0
}

/* analog output sensitivity; address 0x46 */
unsafe extern "C" fn snd_ice1712_ews88mt_input_sense_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let channel = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let mut data: u8 = 0;
    let ndata: u8;

    if snd_bug_on(channel < 0 || channel > 7) {
        return 0;
    }
    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_PCF1], &mut data, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    ndata = (data & !((1 << channel) as u8)) | if (*ucontrol).value.enumerated.item[0] != 0 { 0 } else { (1 << channel) as u8 };
    if ndata != data && snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_PCF1], &ndata, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    if ndata != data { 1 } else { 0 }
}

static snd_ice1712_ews88mt_input_sense: snd_kcontrol_new = snd_kcontrol_new {
    iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
    name: c"Input Sensitivity Switch".as_ptr(),
    access: 0,
    info: Some(snd_ice1712_ewx_io_sense_info),
    get: Some(snd_ice1712_ews88mt_input_sense_get),
    put: Some(snd_ice1712_ews88mt_input_sense_put),
    count: 8,
    private_value: 0,
};

static snd_ice1712_ews88mt_output_sense: snd_kcontrol_new = snd_kcontrol_new {
    iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
    name: c"Output Sensitivity Switch".as_ptr(),
    access: 0,
    info: Some(snd_ice1712_ewx_io_sense_info),
    get: Some(snd_ice1712_ews88mt_output_sense_get),
    put: Some(snd_ice1712_ews88mt_output_sense_put),
    count: 0,
    private_value: 0,
};

/*
 * EWS88D specific controls
 */

unsafe extern "C" fn snd_ice1712_ews88d_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let shift = ((*kcontrol).private_value & 0xff) as c_int;
    let invert = ((*kcontrol).private_value >> 8) as c_int & 1;
    let mut data: [u8; 2] = [0; 2];

    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_88D], data.as_mut_ptr(), 2) != 2 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    data[0] = (data[(shift >> 3) as usize] >> (shift & 7)) & 0x01;
    if invert != 0 {
        data[0] ^= 0x01;
    }
    (*ucontrol).value.integer.value[0] = data[0] as c_long;
    0
}

unsafe extern "C" fn snd_ice1712_ews88d_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = (*ice).spec;
    let shift = ((*kcontrol).private_value & 0xff) as c_int;
    let invert = ((*kcontrol).private_value >> 8) as c_int & 1;
    let mut data: [u8; 2] = [0; 2];
    let mut ndata: [u8; 2] = [0; 2];
    let change: c_int;

    snd_i2c_lock((*ice).i2c);
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_88D], data.as_mut_ptr(), 2) != 2 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    ndata[(shift >> 3) as usize] = data[(shift >> 3) as usize] & !((1 << (shift & 7)) as u8);
    if invert != 0 {
        if (*ucontrol).value.integer.value[0] == 0 {
            ndata[(shift >> 3) as usize] |= (1 << (shift & 7)) as u8;
        }
    } else if (*ucontrol).value.integer.value[0] != 0 {
        ndata[(shift >> 3) as usize] |= (1 << (shift & 7)) as u8;
    }
    change = if data[(shift >> 3) as usize] != ndata[(shift >> 3) as usize] { 1 } else { 0 };
    if change != 0 && snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_88D], data.as_ptr(), 2) != 2 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    change
}

macro_rules! EWS88D_CONTROL {
    ($xiface:expr, $xname:expr, $xshift:expr, $xinvert:expr, $xaccess:expr) => {
        snd_kcontrol_new {
            iface: $xiface,
            name: $xname.as_ptr(),
            access: $xaccess,
            info: Some(snd_ctl_boolean_mono_info),
            get: Some(snd_ice1712_ews88d_control_get),
            put: Some(snd_ice1712_ews88d_control_put),
            count: 0,
            private_value: ($xshift | ($xinvert << 8)) as c_ulong,
        }
    };
}

static snd_ice1712_ews88d_controls: [snd_kcontrol_new; 5] = [
    EWS88D_CONTROL!(unsafe { SNDRV_CTL_ELEM_IFACE_MIXER }, c"IEC958 Input Optical", 0, 1, 0), /* inverted */
    EWS88D_CONTROL!(unsafe { SNDRV_CTL_ELEM_IFACE_MIXER }, c"ADAT Output Optical", 1, 0, 0),
    EWS88D_CONTROL!(unsafe { SNDRV_CTL_ELEM_IFACE_MIXER }, c"ADAT External Master Clock", 2, 0, 0),
    EWS88D_CONTROL!(unsafe { SNDRV_CTL_ELEM_IFACE_MIXER }, c"Enable ADAT", 3, 0, 0),
    EWS88D_CONTROL!(unsafe { SNDRV_CTL_ELEM_IFACE_MIXER }, c"ADAT Through", 4, 1, 0),
];

/*
 * DMX 6Fire specific controls
 */

unsafe extern "C" fn snd_ice1712_6fire_read_pca(ice: *mut snd_ice1712, reg: u8) -> c_int {
    let mut byte: u8;
    let spec = (*ice).spec;

    snd_i2c_lock((*ice).i2c);
    byte = reg;
    if snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_6FIRE], &byte, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        dev_err((*(*ice).card).dev, c"cannot send pca\n".as_ptr());
        return -EIO;
    }

    byte = 0;
    if snd_i2c_readbytes((*spec).i2cdevs[EWS_I2C_6FIRE], &mut byte, 1) != 1 {
        snd_i2c_unlock((*ice).i2c);
        dev_err((*(*ice).card).dev, c"cannot read pca\n".as_ptr());
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    byte as c_int
}

unsafe extern "C" fn snd_ice1712_6fire_write_pca(ice: *mut snd_ice1712, reg: u8, data: u8) -> c_int {
    let bytes: [u8; 2] = [reg, data];
    let spec = (*ice).spec;

    snd_i2c_lock((*ice).i2c);
    if snd_i2c_sendbytes((*spec).i2cdevs[EWS_I2C_6FIRE], bytes.as_ptr(), 2) != 2 {
        snd_i2c_unlock((*ice).i2c);
        return -EIO;
    }
    snd_i2c_unlock((*ice).i2c);
    0
}

unsafe extern "C" fn snd_ice1712_6fire_control_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let shift = ((*kcontrol).private_value & 0xff) as c_int;
    let invert = ((*kcontrol).private_value >> 8) as c_int & 1;
    let mut data: c_int;

    data = snd_ice1712_6fire_read_pca(ice, PCF9554_REG_OUTPUT);
    if data < 0 {
        return data;
    }
    data = (data >> shift) & 1;
    if invert != 0 {
        data ^= 1;
    }
    (*ucontrol).value.integer.value[0] = data as c_long;
    0
}

unsafe extern "C" fn snd_ice1712_6fire_control_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let shift = ((*kcontrol).private_value & 0xff) as c_int;
    let invert = ((*kcontrol).private_value >> 8) as c_int & 1;
    let data: c_int;
    let mut ndata: c_int;

    data = snd_ice1712_6fire_read_pca(ice, PCF9554_REG_OUTPUT);
    if data < 0 {
        return data;
    }
    ndata = data & !(1 << shift);
    if (*ucontrol).value.integer.value[0] != 0 {
        ndata |= 1 << shift;
    }
    if invert != 0 {
        ndata ^= 1 << shift;
    }
    if data != ndata {
        snd_ice1712_6fire_write_pca(ice, PCF9554_REG_OUTPUT, ndata as u8);
        return 1;
    }
    0
}

unsafe extern "C" fn snd_ice1712_6fire_select_input_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXT_0: &[u8] = b"Internal\0";
    static TEXT_1: &[u8] = b"Front Input\0";
    static TEXT_2: &[u8] = b"Rear Input\0";
    static TEXT_3: &[u8] = b"Wave Table\0";
    let texts: [*const c_char; 4] = [
        TEXT_0.as_ptr() as *const c_char,
        TEXT_1.as_ptr() as *const c_char,
        TEXT_2.as_ptr() as *const c_char,
        TEXT_3.as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, 4, texts.as_ptr())
}

unsafe extern "C" fn snd_ice1712_6fire_select_input_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let data: c_int;

    data = snd_ice1712_6fire_read_pca(ice, PCF9554_REG_OUTPUT);
    if data < 0 {
        return data;
    }
    (*ucontrol).value.integer.value[0] = (data & 3) as c_long;
    0
}

unsafe extern "C" fn snd_ice1712_6fire_select_input_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice = snd_kcontrol_chip(kcontrol);
    let data: c_int;
    let mut ndata: c_int;

    data = snd_ice1712_6fire_read_pca(ice, PCF9554_REG_OUTPUT);
    if data < 0 {
        return data;
    }
    ndata = data & !3;
    ndata |= ((*ucontrol).value.integer.value[0] & 3) as c_int;
    if data != ndata {
        snd_ice1712_6fire_write_pca(ice, PCF9554_REG_OUTPUT, ndata as u8);
        return 1;
    }
    0
}

macro_rules! DMX6FIRE_CONTROL {
    ($xname:expr, $xshift:expr, $xinvert:expr) => {
        snd_kcontrol_new {
            iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
            name: $xname.as_ptr(),
            access: 0,
            info: Some(snd_ice1712_6fire_control_get_info),
            get: Some(snd_ice1712_6fire_control_get),
            put: Some(snd_ice1712_6fire_control_put),
            count: 0,
            private_value: ($xshift | ($xinvert << 8)) as c_ulong,
        }
    };
}

unsafe extern "C" fn snd_ice1712_6fire_control_get_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, uinfo)
}

static snd_ice1712_6fire_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new {
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_MIXER },
        name: c"Analog Input Select".as_ptr(),
        access: 0,
        info: Some(snd_ice1712_6fire_select_input_info),
        get: Some(snd_ice1712_6fire_select_input_get),
        put: Some(snd_ice1712_6fire_select_input_put),
        count: 0,
        private_value: 0,
    },
    DMX6FIRE_CONTROL!(c"Front Digital Input Switch", 2, 1),
    // DMX6FIRE_CONTROL("Master Clock Select", 3, 0),
    DMX6FIRE_CONTROL!(c"Optical Digital Input Switch", 4, 0),
    DMX6FIRE_CONTROL!(c"Phono Analog Input Switch", 5, 0),
    DMX6FIRE_CONTROL!(c"Breakbox LED", 6, 0),
];

unsafe extern "C" fn snd_ice1712_ews_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut idx: c_uint;
    let mut err: c_int;

    /* all terratec cards have spdif, but cs8427 module builds it's own controls */
    if (*ice).cs8427.is_null() {
        err = snd_ice1712_spdif_build_controls(ice);
        if err < 0 {
            return err;
        }
    }

    /* ak4524 controls */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWX2496
            || x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88
            || x == ICE1712_SUBDEVICE_DMX6FIRE =>
        {
            err = snd_ice1712_akm4xxx_build_controls(ice);
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    /* card specific controls */
    match (*ice).eeprom.subvendor {
        x if x == ICE1712_SUBDEVICE_EWX2496 => {
            idx = 0;
            while (idx as usize) < snd_ice1712_ewx2496_controls.len() {
                err = snd_ctl_add((*ice).card, snd_ctl_new1(&snd_ice1712_ewx2496_controls[idx as usize], ice as *mut c_void));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }
        x if x == ICE1712_SUBDEVICE_EWS88MT
            || x == ICE1712_SUBDEVICE_EWS88MT_NEW
            || x == ICE1712_SUBDEVICE_PHASE88
            || x == ICE1712_SUBDEVICE_TS88 =>
        {
            err = snd_ctl_add((*ice).card, snd_ctl_new1(&snd_ice1712_ews88mt_input_sense, ice as *mut c_void));
            if err < 0 {
                return err;
            }
            err = snd_ctl_add((*ice).card, snd_ctl_new1(&snd_ice1712_ews88mt_output_sense, ice as *mut c_void));
            if err < 0 {
                return err;
            }
        }
        x if x == ICE1712_SUBDEVICE_EWS88D => {
            idx = 0;
            while (idx as usize) < snd_ice1712_ews88d_controls.len() {
                err = snd_ctl_add((*ice).card, snd_ctl_new1(&snd_ice1712_ews88d_controls[idx as usize], ice as *mut c_void));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }
        x if x == ICE1712_SUBDEVICE_DMX6FIRE => {
            idx = 0;
            while (idx as usize) < snd_ice1712_6fire_controls.len() {
                err = snd_ctl_add((*ice).card, snd_ctl_new1(&snd_ice1712_6fire_controls[idx as usize], ice as *mut c_void));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }
        _ => {}
    }
    0
}

/* entry point */
#[no_mangle]
pub static mut snd_ice1712_ews_cards: [snd_ice1712_card_info; 8] = [
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_EWX2496 },
        name: c"TerraTec EWX24/96".as_ptr(),
        model: c"ewx2496".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_EWS88MT },
        name: c"TerraTec EWS88MT".as_ptr(),
        model: c"ews88mt".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_EWS88MT_NEW },
        name: c"TerraTec EWS88MT".as_ptr(),
        model: c"ews88mt_new".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_PHASE88 },
        name: c"TerraTec Phase88".as_ptr(),
        model: c"phase88".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_TS88 },
        name: c"terrasoniq TS88".as_ptr(),
        model: c"phase88".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_EWS88D },
        name: c"TerraTec EWS88D".as_ptr(),
        model: c"ews88d".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    },
    snd_ice1712_card_info {
        subvendor: unsafe { ICE1712_SUBDEVICE_DMX6FIRE },
        name: c"TerraTec DMX6Fire".as_ptr(),
        model: c"dmx6fire".as_ptr(),
        chip_init: Some(snd_ice1712_ews_init),
        build_controls: Some(snd_ice1712_ews_add_controls),
        mpu401_1_name: c"MIDI-Front DMX6fire".as_ptr(),
        mpu401_2_name: c"Wavetable DMX6fire".as_ptr(),
        mpu401_2_info_flags: unsafe { MPU401_INFO_OUTPUT },
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: ptr::null(),
        model: ptr::null(),
        chip_init: None,
        build_controls: None,
        mpu401_1_name: ptr::null(),
        mpu401_2_name: ptr::null(),
        mpu401_2_info_flags: 0,
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
