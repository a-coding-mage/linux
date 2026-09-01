// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PMac Tumbler/Snapper lowlevel functions
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 *
 *   Rene Rebe <rene.rebe@gmx.net>:
 *     * update from shadow registers on wakeup and headphone plug
 *     * automatically toggle DRC on headphone plug
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type u8 = u8;
type u32 = u32;
type irqreturn_t = c_int;

const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const PMAC_FTR_WRITE_GPIO: c_int = 0;
const PMAC_FTR_READ_GPIO: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 0;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 0;
const PMAC_TUMBLER: c_int = 0;
const PMAC_SNAPPER: c_int = 1;

macro_rules! DBG {
    ($($arg:tt)*) => {};
}

macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ($x.len())
    };
}

unsafe fn IS_G4DA() -> bool {
    of_machine_is_compatible(c"PowerMac3,4".as_ptr()) != 0
}

const TAS_I2C_ADDR: c_uint = 0x34;
const TAS_REG_MCS: c_int = 0x01; /* main control */
const TAS_REG_DRC: c_int = 0x02;
const TAS_REG_VOL: c_int = 0x04;
const TAS_REG_TREBLE: c_int = 0x05;
const TAS_REG_BASS: c_int = 0x06;
const TAS_REG_INPUT1: c_int = 0x07;
const TAS_REG_INPUT2: c_int = 0x08;
const TAS_REG_PCM: c_int = TAS_REG_INPUT1;
const TAS_REG_LMIX: c_int = TAS_REG_INPUT1;
const TAS_REG_RMIX: c_int = TAS_REG_INPUT2;
const TAS_REG_MCS2: c_int = 0x43; /* main control 2 */
const TAS_REG_ACS: c_int = 0x40; /* analog control */

const VOL_IDX_PCM_MONO: usize = 0; /* tas3001c only */
const VOL_IDX_BASS: usize = 1;
const VOL_IDX_TREBLE: usize = 2;
const VOL_IDX_LAST_MONO: usize = 3;

const VOL_IDX_PCM: usize = 0;
const VOL_IDX_PCM2: usize = 1;
const VOL_IDX_ADC: usize = 2;
const VOL_IDX_LAST_MIX: usize = 3;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct pmac_keywest {
    pub client: *mut i2c_client,
    pub addr: c_uint,
    pub init_client: Option<unsafe extern "C" fn(*mut pmac_keywest) -> c_int>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_ctl_integer_info {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_integer_info,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_integer_value {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_enumerated_value {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_integer_value,
    pub enumerated: snd_ctl_enumerated_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub index: c_uint,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pmac {
    pub mixer_data: *mut pmac_tumbler,
    pub mixer_free: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub model: c_int,
    pub card: *mut snd_card,
    pub node: *mut device_node,
    pub master_sw_ctl: *mut snd_kcontrol,
    pub speaker_sw_ctl: *mut snd_kcontrol,
    pub lineout_sw_ctl: *mut snd_kcontrol,
    pub drc_sw_ctl: *mut snd_kcontrol,
    pub hp_detect_ctl: *mut snd_kcontrol,
    pub auto_mute: c_int,
    pub initialized: c_int,
    pub detect_headphone: Option<unsafe extern "C" fn(*mut snd_pmac) -> c_int>,
    pub update_automute: Option<unsafe extern "C" fn(*mut snd_pmac, c_int)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_pmac)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_pmac)>,
}

#[repr(C)]
pub struct pmac_gpio {
    pub addr: c_uint,
    pub active_val: u8,
    pub inactive_val: u8,
    pub active_state: u8,
}

#[repr(C)]
pub struct pmac_tumbler {
    pub i2c: pmac_keywest,
    pub audio_reset: pmac_gpio,
    pub amp_mute: pmac_gpio,
    pub line_mute: pmac_gpio,
    pub line_detect: pmac_gpio,
    pub hp_mute: pmac_gpio,
    pub hp_detect: pmac_gpio,
    pub headphone_irq: c_int,
    pub lineout_irq: c_int,
    pub save_master_vol: [c_uint; 2],
    pub master_vol: [c_uint; 2],
    pub save_master_switch: [c_uint; 2],
    pub master_switch: [c_uint; 2],
    pub mono_vol: [c_uint; VOL_IDX_LAST_MONO],
    pub mix_vol: [[c_uint; 2]; VOL_IDX_LAST_MIX], /* stereo volumes for tas3004 */
    pub drc_range: c_int,
    pub drc_enable: c_int,
    pub capture_source: c_int,
    pub anded_reset: c_int,
    pub auto_mute_notify: c_int,
    pub reset_on_sleep: c_int,
    pub acs: u8,
}

unsafe extern "C" {
    static master_volume_table: [c_uint; 0];
    static mixer_volume_table: [c_uint; 0];
    static bass_volume_table: [c_uint; 0];
    static treble_volume_table: [c_uint; 0];
    static snapper_bass_volume_table: [c_uint; 0];
    static snapper_treble_volume_table: [c_uint; 0];

    fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: c_int, value: c_int) -> c_int;
    fn i2c_smbus_write_i2c_block_data(client: *mut i2c_client, command: c_int, length: c_int, values: *const u8) -> c_int;
    fn mdelay(msecs: c_uint);
    fn msleep(msecs: c_uint);
    fn pmac_call_feature(selector: c_int, node: *mut c_void, param: c_uint, value: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pmac;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn snd_BUG_ON(condition: bool) -> bool;
    fn of_machine_is_compatible(name: *const c_char) -> c_int;
    fn of_find_node_by_name(from: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_get_property(node: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *const u32;
    fn of_node_put(node: *mut device_node);
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> c_int;
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> c_int;
    fn of_property_read_bool(node: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_present(node: *mut device_node, propname: *const c_char) -> bool;
    fn irq_of_parse_and_map(dev: *mut device_node, index: c_int) -> c_long;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn request_module(name: *const c_char) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_pmac_keywest_init(i2c: *mut pmac_keywest) -> c_int;
    fn snd_pmac_keywest_cleanup(i2c: *mut pmac_keywest);
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_pmac_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_pmac_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_pmac_add_automute(chip: *mut snd_pmac) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ... ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

unsafe fn do_gpio_write(gp: *mut pmac_gpio, val: c_int) -> c_int {
    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), (*gp).addr, val)
}

unsafe fn do_gpio_read(gp: *mut pmac_gpio) -> c_int {
    pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), (*gp).addr, 0)
}

unsafe fn tumbler_gpio_free(_gp: *mut pmac_gpio) {}

unsafe extern "C" fn send_init_client(i2c: *mut pmac_keywest, mut regs: *const c_uint) -> c_int {
    while *regs > 0 {
        let mut count: c_int = 10;
        let mut err: c_int;
        loop {
            err = i2c_smbus_write_byte_data((*i2c).client, *regs as c_int, *regs.add(1) as c_int);
            if err >= 0 {
                break;
            }
            DBG!("(W) i2c error %d\n", err);
            mdelay(10);
            let old = count;
            count -= 1;
            if old == 0 {
                break;
            }
        }
        if err < 0 {
            return -ENXIO;
        }
        regs = regs.add(2);
    }
    0
}

unsafe extern "C" fn tumbler_init_client(i2c: *mut pmac_keywest) -> c_int {
    static regs: [c_uint; 3] = [
        TAS_REG_MCS as c_uint, ((1 << 6) | (2 << 4) | (2 << 2) | 0) as c_uint,
        0,
    ];
    DBG!("(I) tumbler init client\n");
    send_init_client(i2c, regs.as_ptr())
}

unsafe extern "C" fn snapper_init_client(i2c: *mut pmac_keywest) -> c_int {
    static regs: [c_uint; 7] = [
        TAS_REG_MCS as c_uint, ((1 << 6) | (2 << 4) | 0) as c_uint,
        TAS_REG_MCS2 as c_uint, (1 << 1) as c_uint,
        TAS_REG_ACS as c_uint, 0,
        0,
    ];
    DBG!("(I) snapper init client\n");
    send_init_client(i2c, regs.as_ptr())
}

unsafe extern "C" fn write_audio_gpio(gp: *mut pmac_gpio, active: c_int) {
    if (*gp).addr == 0 {
        return;
    }
    let active = if active != 0 { (*gp).active_val } else { (*gp).inactive_val };
    do_gpio_write(gp, active as c_int);
    DBG!("(I) gpio %x write %d\n", (*gp).addr, active);
}

unsafe extern "C" fn check_audio_gpio(gp: *mut pmac_gpio) -> c_int {
    if (*gp).addr == 0 {
        return 0;
    }
    let ret = do_gpio_read(gp);
    (((ret & 0x1) == (((*gp).active_val as c_int) & 0x1)) as c_int)
}

unsafe extern "C" fn read_audio_gpio(gp: *mut pmac_gpio) -> c_int {
    if (*gp).addr == 0 {
        return 0;
    }
    let ret = (do_gpio_read(gp) & 0x02) != 0;
    (ret == ((*gp).active_state != 0)) as c_int
}

unsafe extern "C" fn tumbler_set_master_volume(mix: *mut pmac_tumbler) -> c_int {
    let mut block = [0u8; 6];
    if (*mix).i2c.client.is_null() {
        return -ENODEV;
    }
    let mut left_vol = if (*mix).master_switch[0] == 0 { 0 } else {
        let mut v = (*mix).master_vol[0] as usize;
        if v >= ARRAY_SIZE!(master_volume_table) { v = ARRAY_SIZE!(master_volume_table) - 1; }
        master_volume_table[v]
    };
    let mut right_vol = if (*mix).master_switch[1] == 0 { 0 } else {
        let mut v = (*mix).master_vol[1] as usize;
        if v >= ARRAY_SIZE!(master_volume_table) { v = ARRAY_SIZE!(master_volume_table) - 1; }
        master_volume_table[v]
    };
    block[0] = ((left_vol >> 16) & 0xff) as u8;
    block[1] = ((left_vol >> 8) & 0xff) as u8;
    block[2] = ((left_vol >> 0) & 0xff) as u8;
    block[3] = ((right_vol >> 16) & 0xff) as u8;
    block[4] = ((right_vol >> 8) & 0xff) as u8;
    block[5] = ((right_vol >> 0) & 0xff) as u8;
    if i2c_smbus_write_i2c_block_data((*mix).i2c.client, TAS_REG_VOL, 6, block.as_ptr()) < 0 {
        dev_err(&mut (*(*mix).i2c.client).dev, c"failed to set volume\n".as_ptr());
        return -EINVAL;
    }
    DBG!("(I) succeeded to set volume (%u, %u)\n", left_vol, right_vol);
    0
}

unsafe extern "C" fn tumbler_info_master_volume(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (ARRAY_SIZE!(master_volume_table) - 1) as c_long;
    0
}

unsafe extern "C" fn tumbler_get_master_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    (*ucontrol).value.integer.value[0] = (*mix).master_vol[0] as c_long;
    (*ucontrol).value.integer.value[1] = (*mix).master_vol[1] as c_long;
    0
}

unsafe extern "C" fn tumbler_put_master_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    let vol = [(*ucontrol).value.integer.value[0] as c_uint, (*ucontrol).value.integer.value[1] as c_uint];
    if vol[0] as usize >= ARRAY_SIZE!(master_volume_table) || vol[1] as usize >= ARRAY_SIZE!(master_volume_table) {
        return -EINVAL;
    }
    let change = ((*mix).master_vol[0] != vol[0] || (*mix).master_vol[1] != vol[1]) as c_int;
    if change != 0 {
        (*mix).master_vol[0] = vol[0];
        (*mix).master_vol[1] = vol[1];
        tumbler_set_master_volume(mix);
    }
    change
}

unsafe extern "C" fn tumbler_get_master_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    (*ucontrol).value.integer.value[0] = (*mix).master_switch[0] as c_long;
    (*ucontrol).value.integer.value[1] = (*mix).master_switch[1] as c_long;
    0
}

unsafe extern "C" fn tumbler_put_master_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    let change = ((*mix).master_switch[0] as c_long != (*ucontrol).value.integer.value[0]
        || (*mix).master_switch[1] as c_long != (*ucontrol).value.integer.value[1]) as c_int;
    if change != 0 {
        (*mix).master_switch[0] = ((*ucontrol).value.integer.value[0] != 0) as c_uint;
        (*mix).master_switch[1] = ((*ucontrol).value.integer.value[1] != 0) as c_uint;
        tumbler_set_master_volume(mix);
    }
    change
}

const TAS3001_DRC_MAX: c_int = 0x5f;

unsafe extern "C" fn tumbler_set_drc(mix: *mut pmac_tumbler) -> c_int {
    let mut val = [0u8; 2];
    if (*mix).i2c.client.is_null() {
        return -ENODEV;
    }
    if (*mix).drc_enable != 0 {
        val[0] = 0xc1; /* enable, 3:1 compression */
        if (*mix).drc_range > TAS3001_DRC_MAX { val[1] = 0xf0; }
        else if (*mix).drc_range < 0 { val[1] = 0x91; }
        else { val[1] = ((*mix).drc_range + 0x91) as u8; }
    }
    if i2c_smbus_write_i2c_block_data((*mix).i2c.client, TAS_REG_DRC, 2, val.as_ptr()) < 0 {
        dev_err(&mut (*(*mix).i2c.client).dev, c"failed to set DRC\n".as_ptr());
        return -EINVAL;
    }
    DBG!("(I) succeeded to set DRC (%u, %u)\n", val[0], val[1]);
    0
}

const TAS3004_DRC_MAX: c_int = 0xef;

unsafe extern "C" fn snapper_set_drc(mix: *mut pmac_tumbler) -> c_int {
    let mut val = [0u8; 6];
    if (*mix).i2c.client.is_null() {
        return -ENODEV;
    }
    val[0] = if (*mix).drc_enable != 0 { 0x50 } else { 0x51 };
    val[1] = 0x02;
    val[2] = if (*mix).drc_range > 0xef { 0xef } else if (*mix).drc_range < 0 { 0x00 } else { (*mix).drc_range as u8 };
    val[3] = 0xb0;
    val[4] = 0x60;
    val[5] = 0xa0;
    if i2c_smbus_write_i2c_block_data((*mix).i2c.client, TAS_REG_DRC, 6, val.as_ptr()) < 0 {
        dev_err(&mut (*(*mix).i2c.client).dev, c"failed to set DRC\n".as_ptr());
        return -EINVAL;
    }
    DBG!("(I) succeeded to set DRC (%u, %u)\n", val[0], val[1]);
    0
}

unsafe extern "C" fn tumbler_info_drc_value(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = if (*chip).model == PMAC_TUMBLER { TAS3001_DRC_MAX } else { TAS3004_DRC_MAX } as c_long;
    0
}

unsafe extern "C" fn tumbler_get_drc_value(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    (*ucontrol).value.integer.value[0] = (*mix).drc_range as c_long;
    0
}

unsafe extern "C" fn tumbler_put_drc_value(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let val = (*ucontrol).value.integer.value[0] as c_uint;
    if (*chip).model == PMAC_TUMBLER {
        if val > TAS3001_DRC_MAX as c_uint { return -EINVAL; }
    } else if val > TAS3004_DRC_MAX as c_uint { return -EINVAL; }
    let change = ((*mix).drc_range as c_uint != val) as c_int;
    if change != 0 {
        (*mix).drc_range = val as c_int;
        if (*chip).model == PMAC_TUMBLER { tumbler_set_drc(mix); } else { snapper_set_drc(mix); }
    }
    change
}

unsafe extern "C" fn tumbler_get_drc_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    (*ucontrol).value.integer.value[0] = (*mix).drc_enable as c_long;
    0
}

unsafe extern "C" fn tumbler_put_drc_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let change = ((*mix).drc_enable as c_long != (*ucontrol).value.integer.value[0]) as c_int;
    if change != 0 {
        (*mix).drc_enable = ((*ucontrol).value.integer.value[0] != 0) as c_int;
        if (*chip).model == PMAC_TUMBLER { tumbler_set_drc(mix); } else { snapper_set_drc(mix); }
    }
    change
}

#[repr(C)]
pub struct tumbler_mono_vol {
    pub index: c_int,
    pub reg: c_int,
    pub bytes: c_int,
    pub max: c_uint,
    pub table: *const c_uint,
}

unsafe extern "C" fn tumbler_set_mono_volume(mix: *mut pmac_tumbler, info: *const tumbler_mono_vol) -> c_int {
    let mut block = [0u8; 4];
    if (*mix).i2c.client.is_null() { return -ENODEV; }
    let mut vol = (*mix).mono_vol[(*info).index as usize];
    if vol >= (*info).max { vol = (*info).max - 1; }
    vol = *(*info).table.add(vol as usize);
    let mut i = 0;
    while i < (*info).bytes {
        block[i as usize] = ((vol >> (((*info).bytes - i - 1) * 8)) & 0xff) as u8;
        i += 1;
    }
    if i2c_smbus_write_i2c_block_data((*mix).i2c.client, (*info).reg, (*info).bytes, block.as_ptr()) < 0 {
        dev_err(&mut (*(*mix).i2c.client).dev, c"failed to set mono volume %d\n".as_ptr(), (*info).index);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn tumbler_info_mono(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let info = (*kcontrol).private_value as *mut tumbler_mono_vol;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = ((*info).max - 1) as c_long;
    0
}

unsafe extern "C" fn tumbler_get_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let info = (*kcontrol).private_value as *mut tumbler_mono_vol;
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    (*ucontrol).value.integer.value[0] = (*mix).mono_vol[(*info).index as usize] as c_long;
    0
}

unsafe extern "C" fn tumbler_put_mono(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let info = (*kcontrol).private_value as *mut tumbler_mono_vol;
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let vol = (*ucontrol).value.integer.value[0] as c_uint;
    if vol >= (*info).max { return -EINVAL; }
    let change = ((*mix).mono_vol[(*info).index as usize] != vol) as c_int;
    if change != 0 {
        (*mix).mono_vol[(*info).index as usize] = vol;
        tumbler_set_mono_volume(mix, info);
    }
    change
}

static tumbler_pcm_vol_info: tumbler_mono_vol = tumbler_mono_vol { index: VOL_IDX_PCM_MONO as c_int, reg: TAS_REG_PCM, bytes: 3, max: 0, table: unsafe { mixer_volume_table.as_ptr() } };
static tumbler_bass_vol_info: tumbler_mono_vol = tumbler_mono_vol { index: VOL_IDX_BASS as c_int, reg: TAS_REG_BASS, bytes: 1, max: 0, table: unsafe { bass_volume_table.as_ptr() } };
static tumbler_treble_vol_info: tumbler_mono_vol = tumbler_mono_vol { index: VOL_IDX_TREBLE as c_int, reg: TAS_REG_TREBLE, bytes: 1, max: 0, table: unsafe { treble_volume_table.as_ptr() } };
static snapper_bass_vol_info: tumbler_mono_vol = tumbler_mono_vol { index: VOL_IDX_BASS as c_int, reg: TAS_REG_BASS, bytes: 1, max: 0, table: unsafe { snapper_bass_volume_table.as_ptr() } };
static snapper_treble_vol_info: tumbler_mono_vol = tumbler_mono_vol { index: VOL_IDX_TREBLE as c_int, reg: TAS_REG_TREBLE, bytes: 1, max: 0, table: unsafe { snapper_treble_volume_table.as_ptr() } };

unsafe extern "C" fn snapper_set_mix_vol1(mix: *mut pmac_tumbler, idx: c_int, ch: c_int, reg: c_int) -> c_int {
    let mut block = [0u8; 9];
    let mut vol = (*mix).mix_vol[idx as usize][ch as usize] as usize;
    if vol >= ARRAY_SIZE!(mixer_volume_table) {
        vol = ARRAY_SIZE!(mixer_volume_table) - 1;
        (*mix).mix_vol[idx as usize][ch as usize] = vol as c_uint;
    }
    let mut i = 0;
    while i < 3 {
        let v = mixer_volume_table[(*mix).mix_vol[i][ch as usize] as usize];
        let mut j = 0;
        while j < 3 {
            block[i * 3 + j] = ((v >> ((2 - j) * 8)) & 0xff) as u8;
            j += 1;
        }
        i += 1;
    }
    if i2c_smbus_write_i2c_block_data((*mix).i2c.client, reg, 9, block.as_ptr()) < 0 {
        dev_err(&mut (*(*mix).i2c.client).dev, c"failed to set mono volume %d\n".as_ptr(), reg);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snapper_set_mix_vol(mix: *mut pmac_tumbler, idx: c_int) -> c_int {
    if (*mix).i2c.client.is_null() { return -ENODEV; }
    if snapper_set_mix_vol1(mix, idx, 0, TAS_REG_LMIX) < 0 || snapper_set_mix_vol1(mix, idx, 1, TAS_REG_RMIX) < 0 {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snapper_info_mix(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (ARRAY_SIZE!(mixer_volume_table) - 1) as c_long;
    0
}

unsafe extern "C" fn snapper_get_mix(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let idx = (*kcontrol).private_value as usize;
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    (*ucontrol).value.integer.value[0] = (*mix).mix_vol[idx][0] as c_long;
    (*ucontrol).value.integer.value[1] = (*mix).mix_vol[idx][1] as c_long;
    0
}

unsafe extern "C" fn snapper_put_mix(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let idx = (*kcontrol).private_value as usize;
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let vol = [(*ucontrol).value.integer.value[0] as c_uint, (*ucontrol).value.integer.value[1] as c_uint];
    if vol[0] as usize >= ARRAY_SIZE!(mixer_volume_table) || vol[1] as usize >= ARRAY_SIZE!(mixer_volume_table) { return -EINVAL; }
    let change = ((*mix).mix_vol[idx][0] != vol[0] || (*mix).mix_vol[idx][1] != vol[1]) as c_int;
    if change != 0 {
        (*mix).mix_vol[idx][0] = vol[0];
        (*mix).mix_vol[idx][1] = vol[1];
        snapper_set_mix_vol(mix, idx as c_int);
    }
    change
}

const TUMBLER_MUTE_HP: c_ulong = 0;
const TUMBLER_MUTE_AMP: c_ulong = 1;
const TUMBLER_MUTE_LINE: c_ulong = 2;

unsafe fn tumbler_mute_gpio(mix: *mut pmac_tumbler, private_value: c_ulong) -> *mut pmac_gpio {
    match private_value {
        TUMBLER_MUTE_HP => &mut (*mix).hp_mute,
        TUMBLER_MUTE_AMP => &mut (*mix).amp_mute,
        TUMBLER_MUTE_LINE => &mut (*mix).line_mute,
        _ => ptr::null_mut(),
    }
}

unsafe extern "C" fn tumbler_get_mute_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let gp = tumbler_mute_gpio(mix, (*kcontrol).private_value);
    if gp.is_null() { return -EINVAL; }
    (*ucontrol).value.integer.value[0] = (check_audio_gpio(gp) == 0) as c_long;
    0
}

unsafe extern "C" fn tumbler_put_mute_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    /* PMAC_SUPPORT_AUTOMUTE: if chip->update_automute && chip->auto_mute, return 0. */
    if (*chip).update_automute.is_some() && (*chip).auto_mute != 0 { return 0; }
    let mix = (*chip).mixer_data;
    if mix.is_null() { return -ENODEV; }
    let gp = tumbler_mute_gpio(mix, (*kcontrol).private_value);
    if gp.is_null() { return -EINVAL; }
    let val = (check_audio_gpio(gp) == 0) as c_int;
    if val as c_long != (*ucontrol).value.integer.value[0] {
        write_audio_gpio(gp, ((*ucontrol).value.integer.value[0] == 0) as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn snapper_set_capture_source(mix: *mut pmac_tumbler) -> c_int {
    if (*mix).i2c.client.is_null() { return -ENODEV; }
    if (*mix).capture_source != 0 { (*mix).acs |= 2; } else { (*mix).acs &= !2; }
    i2c_smbus_write_byte_data((*mix).i2c.client, TAS_REG_ACS, (*mix).acs as c_int)
}

unsafe extern "C" fn snapper_info_capture_source(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 2] = [c"Line".as_ptr(), c"Mic".as_ptr()];
    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn snapper_get_capture_source(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    (*ucontrol).value.enumerated.item[0] = (*mix).capture_source as c_uint;
    0
}

unsafe extern "C" fn snapper_put_capture_source(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mix = (*chip).mixer_data;
    let change = ((*ucontrol).value.enumerated.item[0] as c_int != (*mix).capture_source) as c_int;
    if change != 0 {
        (*mix).capture_source = ((*ucontrol).value.enumerated.item[0] != 0) as c_int;
        snapper_set_capture_source(mix);
    }
    change
}

macro_rules! ctl {
    ($name:expr, $info:expr, $get:expr, $put:expr, $index:expr, $private:expr) => {
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER as c_uint, name: $name.as_ptr(), info: $info, get: $get, put: $put, index: $index, private_value: $private }
    };
}

static tumbler_mixers: [snd_kcontrol_new; 6] = [
    ctl!(c"Master Playback Volume", Some(tumbler_info_master_volume), Some(tumbler_get_master_volume), Some(tumbler_put_master_volume), 0, 0),
    ctl!(c"Master Playback Switch", Some(snd_pmac_boolean_stereo_info), Some(tumbler_get_master_switch), Some(tumbler_put_master_switch), 0, 0),
    ctl!(c"Tone Control - Bass", Some(tumbler_info_mono), Some(tumbler_get_mono), Some(tumbler_put_mono), 0, unsafe { &tumbler_bass_vol_info as *const _ as c_ulong }),
    ctl!(c"Tone Control - Treble", Some(tumbler_info_mono), Some(tumbler_get_mono), Some(tumbler_put_mono), 0, unsafe { &tumbler_treble_vol_info as *const _ as c_ulong }),
    ctl!(c"PCM Playback Volume", Some(tumbler_info_mono), Some(tumbler_get_mono), Some(tumbler_put_mono), 0, unsafe { &tumbler_pcm_vol_info as *const _ as c_ulong }),
    ctl!(c"DRC Range", Some(tumbler_info_drc_value), Some(tumbler_get_drc_value), Some(tumbler_put_drc_value), 0, 0),
];

static snapper_mixers: [snd_kcontrol_new; 9] = [
    ctl!(c"Master Playback Volume", Some(tumbler_info_master_volume), Some(tumbler_get_master_volume), Some(tumbler_put_master_volume), 0, 0),
    ctl!(c"Master Playback Switch", Some(snd_pmac_boolean_stereo_info), Some(tumbler_get_master_switch), Some(tumbler_put_master_switch), 0, 0),
    ctl!(c"PCM Playback Volume", Some(snapper_info_mix), Some(snapper_get_mix), Some(snapper_put_mix), 0, VOL_IDX_PCM as c_ulong),
    /* Alternative PCM is assigned to Mic analog loopback on iBook G4 */
    ctl!(c"Mic Playback Volume", Some(snapper_info_mix), Some(snapper_get_mix), Some(snapper_put_mix), 0, VOL_IDX_PCM2 as c_ulong),
    ctl!(c"Monitor Mix Volume", Some(snapper_info_mix), Some(snapper_get_mix), Some(snapper_put_mix), 0, VOL_IDX_ADC as c_ulong),
    ctl!(c"Tone Control - Bass", Some(tumbler_info_mono), Some(tumbler_get_mono), Some(tumbler_put_mono), 0, unsafe { &snapper_bass_vol_info as *const _ as c_ulong }),
    ctl!(c"Tone Control - Treble", Some(tumbler_info_mono), Some(tumbler_get_mono), Some(tumbler_put_mono), 0, unsafe { &snapper_treble_vol_info as *const _ as c_ulong }),
    ctl!(c"DRC Range", Some(tumbler_info_drc_value), Some(tumbler_get_drc_value), Some(tumbler_put_drc_value), 0, 0),
    ctl!(c"Input Source", Some(snapper_info_capture_source), Some(snapper_get_capture_source), Some(snapper_put_capture_source), 0, 0),
];

static tumbler_hp_sw: snd_kcontrol_new = ctl!(c"Headphone Playback Switch", Some(snd_pmac_boolean_mono_info), Some(tumbler_get_mute_switch), Some(tumbler_put_mute_switch), 0, TUMBLER_MUTE_HP);
static tumbler_speaker_sw: snd_kcontrol_new = ctl!(c"Speaker Playback Switch", Some(snd_pmac_boolean_mono_info), Some(tumbler_get_mute_switch), Some(tumbler_put_mute_switch), 0, TUMBLER_MUTE_AMP);
static tumbler_lineout_sw: snd_kcontrol_new = ctl!(c"Line Out Playback Switch", Some(snd_pmac_boolean_mono_info), Some(tumbler_get_mute_switch), Some(tumbler_put_mute_switch), 0, TUMBLER_MUTE_LINE);
static tumbler_drc_sw: snd_kcontrol_new = ctl!(c"DRC Switch", Some(snd_pmac_boolean_mono_info), Some(tumbler_get_drc_switch), Some(tumbler_put_drc_switch), 0, 0);

/* PMAC_SUPPORT_AUTOMUTE */
unsafe extern "C" fn tumbler_detect_headphone(chip: *mut snd_pmac) -> c_int {
    let mix = (*chip).mixer_data;
    let mut detect = 0;
    if (*mix).hp_detect.addr != 0 { detect |= read_audio_gpio(&mut (*mix).hp_detect); }
    detect
}

unsafe extern "C" fn tumbler_detect_lineout(chip: *mut snd_pmac) -> c_int {
    let mix = (*chip).mixer_data;
    let mut detect = 0;
    if (*mix).line_detect.addr != 0 { detect |= read_audio_gpio(&mut (*mix).line_detect); }
    detect
}

unsafe extern "C" fn check_mute(chip: *mut snd_pmac, gp: *mut pmac_gpio, val: c_int, do_notify: c_int, sw: *mut snd_kcontrol) {
    if check_audio_gpio(gp) != val {
        write_audio_gpio(gp, val);
        if do_notify != 0 {
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*sw).id);
        }
    }
}

static mut device_change: work_struct = work_struct { _private: [] };
static mut device_change_chip: *mut snd_pmac = ptr::null_mut();

unsafe extern "C" fn device_change_handler(_work: *mut work_struct) {
    let chip = device_change_chip;
    if chip.is_null() { return; }
    let mix = (*chip).mixer_data;
    if snd_BUG_ON(mix.is_null()) { return; }
    let headphone = tumbler_detect_headphone(chip);
    let lineout = tumbler_detect_lineout(chip);
    DBG!("headphone: %d, lineout: %d\n", headphone, lineout);
    if headphone != 0 || lineout != 0 {
        if headphone != 0 { check_mute(chip, &mut (*mix).hp_mute, 0, (*mix).auto_mute_notify, (*chip).master_sw_ctl); }
        if lineout != 0 && (*mix).line_mute.addr != 0 { check_mute(chip, &mut (*mix).line_mute, 0, (*mix).auto_mute_notify, (*chip).lineout_sw_ctl); }
        if (*mix).anded_reset != 0 { msleep(10); }
        check_mute(chip, &mut (*mix).amp_mute, (!IS_G4DA()) as c_int, (*mix).auto_mute_notify, (*chip).speaker_sw_ctl);
    } else {
        check_mute(chip, &mut (*mix).amp_mute, 0, (*mix).auto_mute_notify, (*chip).speaker_sw_ctl);
        if (*mix).anded_reset != 0 { msleep(10); }
        check_mute(chip, &mut (*mix).hp_mute, 1, (*mix).auto_mute_notify, (*chip).master_sw_ctl);
        if (*mix).line_mute.addr != 0 { check_mute(chip, &mut (*mix).line_mute, 1, (*mix).auto_mute_notify, (*chip).lineout_sw_ctl); }
    }
    if (*mix).auto_mute_notify != 0 {
        snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hp_detect_ctl).id);
    }
    /* CONFIG_SND_POWERMAC_AUTO_DRC: toggle DRC when headphone or lineout changes. */
    tumbler_set_master_volume(mix);
}

unsafe extern "C" fn tumbler_update_automute(chip: *mut snd_pmac, do_notify: c_int) {
    if (*chip).auto_mute != 0 {
        let mix = (*chip).mixer_data;
        if snd_BUG_ON(mix.is_null()) { return; }
        (*mix).auto_mute_notify = do_notify;
        schedule_work(&mut device_change);
    }
}

unsafe extern "C" fn headphone_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let chip = devid as *mut snd_pmac;
    if (*chip).update_automute.is_some() && (*chip).initialized != 0 {
        (*chip).update_automute.unwrap()(chip, 1);
        return IRQ_HANDLED;
    }
    IRQ_NONE
}

unsafe extern "C" fn find_audio_device(name: *const c_char) -> *mut device_node {
    let gpiop = of_find_node_by_name(ptr::null_mut(), c"gpio".as_ptr());
    if gpiop.is_null() { return ptr::null_mut(); }
    let mut np: *mut device_node = ptr::null_mut();
    /* for_each_child_of_node(gpiop, np) translated as dependency-provided iteration. */
    let property = if !np.is_null() { of_get_property(np, c"audio-gpio".as_ptr(), ptr::null_mut()) as *const c_char } else { ptr::null() };
    if !property.is_null() && strcmp(property, name) == 0 {}
    of_node_put(gpiop);
    np
}

unsafe extern "C" fn find_compatible_audio_device(name: *const c_char) -> *mut device_node {
    let gpiop = of_find_node_by_name(ptr::null_mut(), c"gpio".as_ptr());
    if gpiop.is_null() { return ptr::null_mut(); }
    let np: *mut device_node = ptr::null_mut();
    /* for_each_child_of_node(gpiop, np) if of_device_is_compatible(np, name) break; */
    if !np.is_null() && of_device_is_compatible(np, name) != 0 {}
    of_node_put(gpiop);
    np
}

unsafe extern "C" fn tumbler_find_device(device: *const c_char, platform: *const c_char, gp: *mut pmac_gpio, is_compatible: c_int) -> c_long {
    let node = if is_compatible != 0 { find_compatible_audio_device(device) } else { find_audio_device(device) };
    if node.is_null() {
        DBG!("(W) cannot find audio device %s !\n", device);
        return -(ENODEV as c_long);
    }
    let mut base = of_get_property(node, c"AAPL,address".as_ptr(), ptr::null_mut());
    let addr: u32;
    if base.is_null() {
        base = of_get_property(node, c"reg".as_ptr(), ptr::null_mut());
        if base.is_null() {
            DBG!("(E) cannot find address for device %s !\n", device);
            of_node_put(node);
            return -(ENODEV as c_long);
        }
        addr = if *base < 0x50 { *base + 0x50 } else { *base };
    } else {
        addr = *base;
    }
    (*gp).addr = addr & 0x0000ffff;
    base = of_get_property(node, c"audio-gpio-active-state".as_ptr(), ptr::null_mut());
    if !base.is_null() {
        (*gp).active_state = *base as u8;
        (*gp).active_val = if *base != 0 { 0x5 } else { 0x4 };
        (*gp).inactive_val = if *base != 0 { 0x4 } else { 0x5 };
    } else {
        let mut prop: *const u32 = ptr::null();
        (*gp).active_state = (IS_G4DA() && strncmp(device, c"keywest-gpio1".as_ptr(), 13) == 0) as u8;
        (*gp).active_val = 0x4;
        (*gp).inactive_val = 0x5;
        if !platform.is_null() { prop = of_get_property(node, platform, ptr::null_mut()); }
        if !prop.is_null() {
            if *prop.add(3) == 0x9 && *prop.add(4) == 0x9 { (*gp).active_val = 0xd; (*gp).inactive_val = 0xc; }
            if *prop.add(3) == 0x1 && *prop.add(4) == 0x1 { (*gp).active_val = 0x5; (*gp).inactive_val = 0x4; }
        }
    }
    DBG!("(I) GPIO device %s found, offset: %x, active state: %d !\n", device, (*gp).addr, (*gp).active_state);
    let ret = irq_of_parse_and_map(node, 0);
    of_node_put(node);
    ret
}

unsafe extern "C" fn tumbler_reset_audio(chip: *mut snd_pmac) {
    let mix = (*chip).mixer_data;
    if (*mix).anded_reset != 0 {
        DBG!("(I) codec anded reset !\n");
        write_audio_gpio(&mut (*mix).hp_mute, 0);
        write_audio_gpio(&mut (*mix).amp_mute, 0);
        msleep(200);
        write_audio_gpio(&mut (*mix).hp_mute, 1);
        write_audio_gpio(&mut (*mix).amp_mute, 1);
        msleep(100);
        write_audio_gpio(&mut (*mix).hp_mute, 0);
        write_audio_gpio(&mut (*mix).amp_mute, 0);
        msleep(100);
    } else {
        DBG!("(I) codec normal reset !\n");
        write_audio_gpio(&mut (*mix).audio_reset, 0);
        msleep(200);
        write_audio_gpio(&mut (*mix).audio_reset, 1);
        msleep(100);
        write_audio_gpio(&mut (*mix).audio_reset, 0);
        msleep(100);
    }
}

/* CONFIG_PM */
unsafe extern "C" fn tumbler_suspend(chip: *mut snd_pmac) {
    let mix = (*chip).mixer_data;
    if (*mix).headphone_irq >= 0 { disable_irq((*mix).headphone_irq); }
    if (*mix).lineout_irq >= 0 { disable_irq((*mix).lineout_irq); }
    (*mix).save_master_switch = (*mix).master_switch;
    (*mix).save_master_vol = (*mix).master_vol;
    (*mix).master_switch[0] = 0;
    (*mix).master_switch[1] = 0;
    tumbler_set_master_volume(mix);
    if (*mix).anded_reset == 0 {
        write_audio_gpio(&mut (*mix).amp_mute, 1);
        write_audio_gpio(&mut (*mix).hp_mute, 1);
    }
    if (*chip).model == PMAC_SNAPPER {
        (*mix).acs |= 1;
        i2c_smbus_write_byte_data((*mix).i2c.client, TAS_REG_ACS, (*mix).acs as c_int);
    }
    if (*mix).anded_reset != 0 {
        write_audio_gpio(&mut (*mix).amp_mute, 1);
        write_audio_gpio(&mut (*mix).hp_mute, 1);
    } else {
        write_audio_gpio(&mut (*mix).audio_reset, 1);
    }
}

unsafe extern "C" fn tumbler_resume(chip: *mut snd_pmac) {
    let mix = (*chip).mixer_data;
    (*mix).acs &= !1;
    (*mix).master_switch = (*mix).save_master_switch;
    (*mix).master_vol = (*mix).save_master_vol;
    tumbler_reset_audio(chip);
    if !(*mix).i2c.client.is_null() && (*mix).i2c.init_client.is_some() {
        if (*mix).i2c.init_client.unwrap()(&mut (*mix).i2c) < 0 {
            dev_err((*(*chip).card).dev, c"tumbler_init_client error\n".as_ptr());
        }
    } else {
        dev_err((*(*chip).card).dev, c"tumbler: i2c is not initialized\n".as_ptr());
    }
    if (*chip).model == PMAC_TUMBLER {
        tumbler_set_mono_volume(mix, &tumbler_pcm_vol_info);
        tumbler_set_mono_volume(mix, &tumbler_bass_vol_info);
        tumbler_set_mono_volume(mix, &tumbler_treble_vol_info);
        tumbler_set_drc(mix);
    } else {
        snapper_set_mix_vol(mix, VOL_IDX_PCM as c_int);
        snapper_set_mix_vol(mix, VOL_IDX_PCM2 as c_int);
        snapper_set_mix_vol(mix, VOL_IDX_ADC as c_int);
        tumbler_set_mono_volume(mix, &snapper_bass_vol_info);
        tumbler_set_mono_volume(mix, &snapper_treble_vol_info);
        snapper_set_drc(mix);
        snapper_set_capture_source(mix);
    }
    tumbler_set_master_volume(mix);
    if let Some(update) = (*chip).update_automute { update(chip, 0); }
    if (*mix).headphone_irq >= 0 {
        enable_irq((*mix).headphone_irq);
        let val = do_gpio_read(&mut (*mix).hp_detect);
        do_gpio_write(&mut (*mix).hp_detect, val | 0x80);
    }
    if (*mix).lineout_irq >= 0 { enable_irq((*mix).lineout_irq); }
}

unsafe extern "C" fn tumbler_init(chip: *mut snd_pmac) -> c_int {
    let mix = (*chip).mixer_data;
    if tumbler_find_device(c"audio-hw-reset".as_ptr(), c"platform-do-hw-reset".as_ptr(), &mut (*mix).audio_reset, 0) < 0 {
        tumbler_find_device(c"hw-reset".as_ptr(), c"platform-do-hw-reset".as_ptr(), &mut (*mix).audio_reset, 1);
    }
    if tumbler_find_device(c"amp-mute".as_ptr(), c"platform-do-amp-mute".as_ptr(), &mut (*mix).amp_mute, 0) < 0 {
        tumbler_find_device(c"amp-mute".as_ptr(), c"platform-do-amp-mute".as_ptr(), &mut (*mix).amp_mute, 1);
    }
    if tumbler_find_device(c"headphone-mute".as_ptr(), c"platform-do-headphone-mute".as_ptr(), &mut (*mix).hp_mute, 0) < 0 {
        tumbler_find_device(c"headphone-mute".as_ptr(), c"platform-do-headphone-mute".as_ptr(), &mut (*mix).hp_mute, 1);
    }
    if tumbler_find_device(c"line-output-mute".as_ptr(), c"platform-do-lineout-mute".as_ptr(), &mut (*mix).line_mute, 0) < 0 {
        tumbler_find_device(c"line-output-mute".as_ptr(), c"platform-do-lineout-mute".as_ptr(), &mut (*mix).line_mute, 1);
    }
    let mut irq = tumbler_find_device(c"headphone-detect".as_ptr(), ptr::null(), &mut (*mix).hp_detect, 0);
    if irq <= 0 { irq = tumbler_find_device(c"headphone-detect".as_ptr(), ptr::null(), &mut (*mix).hp_detect, 1); }
    if irq <= 0 { irq = tumbler_find_device(c"keywest-gpio15".as_ptr(), ptr::null(), &mut (*mix).hp_detect, 1); }
    (*mix).headphone_irq = irq as c_int;
    irq = tumbler_find_device(c"line-output-detect".as_ptr(), ptr::null(), &mut (*mix).line_detect, 0);
    if irq <= 0 { irq = tumbler_find_device(c"line-output-detect".as_ptr(), ptr::null(), &mut (*mix).line_detect, 1); }
    if IS_G4DA() && irq <= 0 { irq = tumbler_find_device(c"keywest-gpio16".as_ptr(), ptr::null(), &mut (*mix).line_detect, 1); }
    (*mix).lineout_irq = irq as c_int;
    tumbler_reset_audio(chip);
    0
}

unsafe extern "C" fn tumbler_cleanup(chip: *mut snd_pmac) {
    let mix = (*chip).mixer_data;
    if mix.is_null() { return; }
    if (*mix).headphone_irq >= 0 { free_irq((*mix).headphone_irq, chip as *mut c_void); }
    if (*mix).lineout_irq >= 0 { free_irq((*mix).lineout_irq, chip as *mut c_void); }
    tumbler_gpio_free(&mut (*mix).audio_reset);
    tumbler_gpio_free(&mut (*mix).amp_mute);
    tumbler_gpio_free(&mut (*mix).hp_mute);
    tumbler_gpio_free(&mut (*mix).hp_detect);
    snd_pmac_keywest_cleanup(&mut (*mix).i2c);
    kfree(mix as *mut c_void);
    (*chip).mixer_data = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_tumbler_init(chip: *mut snd_pmac) -> c_int {
    let mut err: c_int;
    request_module(c"i2c-powermac".as_ptr());
    let mix = kzalloc(core::mem::size_of::<pmac_tumbler>(), 0) as *mut pmac_tumbler;
    if mix.is_null() { return -ENOMEM; }
    (*mix).headphone_irq = -1;
    (*chip).mixer_data = mix;
    (*chip).mixer_free = Some(tumbler_cleanup);
    (*mix).anded_reset = 0;
    (*mix).reset_on_sleep = 1;
    /* for_each_child_of_node(chip->node, np): find "sound", read has-anded-reset and layout-id. */
    let np: *mut device_node = ptr::null_mut();
    if !np.is_null() && of_node_name_eq(np, c"sound".as_ptr()) != 0 {
        if of_property_read_bool(np, c"has-anded-reset".as_ptr()) { (*mix).anded_reset = 1; }
        if of_property_present(np, c"layout-id".as_ptr()) { (*mix).reset_on_sleep = 0; }
        of_node_put(np);
    }
    err = tumbler_init(chip);
    if err < 0 { return err; }
    let mut tas_node = of_find_node_by_name(ptr::null_mut(), c"deq".as_ptr());
    if tas_node.is_null() { tas_node = of_find_node_by_name(ptr::null_mut(), c"codec".as_ptr()); }
    if tas_node.is_null() { return -ENODEV; }
    let mut paddr = of_get_property(tas_node, c"i2c-address".as_ptr(), ptr::null_mut());
    if paddr.is_null() { paddr = of_get_property(tas_node, c"reg".as_ptr(), ptr::null_mut()); }
    if !paddr.is_null() { (*mix).i2c.addr = *paddr >> 1; } else { (*mix).i2c.addr = TAS_I2C_ADDR; }
    of_node_put(tas_node);
    DBG!("(I) TAS i2c address is: %x\n", (*mix).i2c.addr);
    let chipname: *const c_char;
    if (*chip).model == PMAC_TUMBLER {
        (*mix).i2c.init_client = Some(tumbler_init_client);
        (*mix).i2c.name = c"TAS3001c".as_ptr();
        chipname = c"Tumbler".as_ptr();
    } else {
        (*mix).i2c.init_client = Some(snapper_init_client);
        (*mix).i2c.name = c"TAS3004".as_ptr();
        chipname = c"Snapper".as_ptr();
    }
    err = snd_pmac_keywest_init(&mut (*mix).i2c);
    if err < 0 { return err; }
    sprintf((*(*chip).card).mixername.as_mut_ptr(), c"PowerMac %s".as_ptr(), chipname);
    if (*chip).model == PMAC_TUMBLER {
        let mut i = 0;
        while i < tumbler_mixers.len() {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&tumbler_mixers[i], chip as *mut c_void));
            if err < 0 { return err; }
            i += 1;
        }
    } else {
        let mut i = 0;
        while i < snapper_mixers.len() {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&snapper_mixers[i], chip as *mut c_void));
            if err < 0 { return err; }
            i += 1;
        }
    }
    (*chip).master_sw_ctl = snd_ctl_new1(&tumbler_hp_sw, chip as *mut c_void);
    err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
    if err < 0 { return err; }
    (*chip).speaker_sw_ctl = snd_ctl_new1(&tumbler_speaker_sw, chip as *mut c_void);
    err = snd_ctl_add((*chip).card, (*chip).speaker_sw_ctl);
    if err < 0 { return err; }
    if (*mix).line_mute.addr != 0 {
        (*chip).lineout_sw_ctl = snd_ctl_new1(&tumbler_lineout_sw, chip as *mut c_void);
        err = snd_ctl_add((*chip).card, (*chip).lineout_sw_ctl);
        if err < 0 { return err; }
    }
    (*chip).drc_sw_ctl = snd_ctl_new1(&tumbler_drc_sw, chip as *mut c_void);
    err = snd_ctl_add((*chip).card, (*chip).drc_sw_ctl);
    if err < 0 { return err; }
    if (*chip).model == PMAC_TUMBLER { (*mix).drc_range = (TAS3001_DRC_MAX * 6) / 10; } else { (*mix).drc_range = (TAS3004_DRC_MAX * 6) / 10; }
    (*mix).drc_enable = 1;
    if (*chip).model == PMAC_TUMBLER { tumbler_set_drc(mix); } else { snapper_set_drc(mix); }
    /* CONFIG_PM */
    (*chip).suspend = Some(tumbler_suspend);
    (*chip).resume = Some(tumbler_resume);
    /* INIT_WORK(&device_change, device_change_handler); */
    device_change_chip = chip;
    /* PMAC_SUPPORT_AUTOMUTE */
    if (*mix).headphone_irq >= 0 || (*mix).lineout_irq >= 0 {
        err = snd_pmac_add_automute(chip);
        if err < 0 { return err; }
    }
    (*chip).detect_headphone = Some(tumbler_detect_headphone);
    (*chip).update_automute = Some(tumbler_update_automute);
    tumbler_update_automute(chip, 0);
    if (*mix).headphone_irq >= 0 {
        err = request_irq((*mix).headphone_irq, headphone_intr, 0, c"Sound Headphone Detection".as_ptr(), chip as *mut c_void);
        if err < 0 { return 0; }
        let val = do_gpio_read(&mut (*mix).hp_detect);
        do_gpio_write(&mut (*mix).hp_detect, val | 0x80);
    }
    if (*mix).lineout_irq >= 0 {
        err = request_irq((*mix).lineout_irq, headphone_intr, 0, c"Sound Lineout Detection".as_ptr(), chip as *mut c_void);
        if err < 0 { return 0; }
        let val = do_gpio_read(&mut (*mix).line_detect);
        do_gpio_write(&mut (*mix).line_detect, val | 0x80);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
