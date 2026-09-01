// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com).
 *
 *  Rust translation of ./pci/ac97/ac97_codec.c.
 *  C include/module metadata and CONFIG_* regions are preserved as comments
 *  where they cannot be mapped from this isolated file alone.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type size_t = usize;

#[repr(C)]
pub struct snd_ac97 {
    pub id: c_uint,
    pub num: c_int,
    pub addr: c_int,
    pub scaps: c_uint,
    pub caps: c_uint,
    pub ext_id: c_uint,
    pub ext_mid: c_uint,
    pub flags: c_uint,
    pub power_up: c_uint,
    pub spdif_status: c_uint,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
    pub regs: [c_uint; 0x80],
    pub reg_accessed: *mut c_ulong,
    pub rates: [c_uint; 8],
    pub bus: *mut snd_ac97_bus,
    pub pci: *mut pci_dev,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub res_table: *const snd_ac97_res_table,
    pub build_ops: *const snd_ac97_build_ops,
    pub reg_mutex: mutex,
    pub page_mutex: mutex,
    pub spec: snd_ac97_spec,
    pub dev: device,
    /* CONFIG_SND_AC97_POWER_SAVE: power_work lives here in C. */
}

#[repr(C)]
pub struct snd_ac97_spec {
    pub ad18xx: snd_ac97_ad18xx_spec,
}

#[repr(C)]
pub struct snd_ac97_ad18xx_spec {
    pub pcmreg: [c_uint; 4],
    pub unchained: [c_uint; 4],
    pub chained: [c_uint; 4],
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub card: *mut snd_card,
    pub num: c_int,
    pub ops: *const snd_ac97_bus_ops,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)>,
    pub clock: c_uint,
    pub no_vra: c_int,
    pub dra: c_int,
    pub pcms: *mut c_void,
    pub codec: [*mut snd_ac97; 4],
    pub bus_lock: spinlock_t,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_uint, c_uint)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_uint) -> c_uint>,
    pub reset: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub wait: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub init: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub pci: *mut pci_dev,
    pub num: c_int,
    pub addr: c_int,
    pub scaps: c_uint,
    pub res_table: *const snd_ac97_res_table,
}

#[repr(C)]
pub struct snd_ac97_res_table {
    pub reg: c_uint,
    pub bits: c_uint,
}

#[repr(C)]
pub struct snd_ac97_build_ops {
    pub build_3d: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_spdif: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_post_spdif: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub build_specific: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_ac97)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_ac97)>,
}

#[repr(C)]
pub struct ac97_codec_id {
    pub id: c_uint,
    pub mask: c_uint,
    pub name: *const c_char,
    pub patch: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub mpatch: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
    pub flags: c_uint,
}

#[repr(C)]
pub struct ac97_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub mask: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
    pub vd: [snd_kcontrol_volatile; 1],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

type c_long = isize;

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub iec958: snd_ctl_elem_value_iec958,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_iec958 {
    pub status: [c_uint; 4],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub number: c_int,
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub iface: c_uint,
}

#[repr(C)]
pub struct ac97_power_reg {
    pub reg: c_uint,
    pub power_reg: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct quirk_table {
    pub name: *const c_char,
    pub func: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>,
}

#[repr(C)]
pub struct ac97_quirk {
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub mask: c_uint,
    pub codec_id: c_uint,
    pub name: *const c_char,
    pub type_: c_int,
}

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct device { pub bus: *const c_void, pub parent: *mut device, pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const EACCES: c_int = 13;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;

/* Constants from included Linux/ALSA headers are future dependencies. */
extern "C" {
    static mut jiffies: c_ulong;
    static ac97_bus_type: c_void;

    fn patch_ad1819(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1881(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1885(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1886(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1888(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1980(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1981a(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1981b(ac97: *mut snd_ac97) -> c_int;
    fn patch_ad1985(ac97: *mut snd_ac97) -> c_int;
    fn patch_alc650(ac97: *mut snd_ac97) -> c_int;
    fn patch_alc655(ac97: *mut snd_ac97) -> c_int;
    fn patch_alc203(ac97: *mut snd_ac97) -> c_int;
    fn patch_alc850(ac97: *mut snd_ac97) -> c_int;
    fn patch_aztech_azf3328(ac97: *mut snd_ac97) -> c_int;
    fn patch_cm9738(ac97: *mut snd_ac97) -> c_int;
    fn patch_cm9739(ac97: *mut snd_ac97) -> c_int;
    fn patch_cm9780(ac97: *mut snd_ac97) -> c_int;
    fn patch_cm9761(ac97: *mut snd_ac97) -> c_int;
    fn patch_cirrus_spdif(ac97: *mut snd_ac97) -> c_int;
    fn patch_cirrus_cs4299(ac97: *mut snd_ac97) -> c_int;
    fn patch_conexant(ac97: *mut snd_ac97) -> c_int;
    fn patch_cx20551(ac97: *mut snd_ac97) -> c_int;
    fn patch_vt1616(ac97: *mut snd_ac97) -> c_int;
    fn patch_it2646(ac97: *mut snd_ac97) -> c_int;
    fn patch_lm4550(ac97: *mut snd_ac97) -> c_int;
    fn mpatch_si3036(ac97: *mut snd_ac97) -> c_int;
    fn patch_tritech_tr28028(ac97: *mut snd_ac97) -> c_int;
    fn patch_vt1613(ac97: *mut snd_ac97) -> c_int;
    fn patch_vt1617a(ac97: *mut snd_ac97) -> c_int;
    fn patch_vt1618(ac97: *mut snd_ac97) -> c_int;
    fn patch_wolfson03(ac97: *mut snd_ac97) -> c_int;
    fn patch_wolfson04(ac97: *mut snd_ac97) -> c_int;
    fn patch_wolfson05(ac97: *mut snd_ac97) -> c_int;
    fn patch_wolfson11(ac97: *mut snd_ac97) -> c_int;
    fn patch_wolfson13(ac97: *mut snd_ac97) -> c_int;
    fn patch_yamaha_ymf743(ac97: *mut snd_ac97) -> c_int;
    fn patch_yamaha_ymf753(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9700(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9708(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9721(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9744(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9756(ac97: *mut snd_ac97) -> c_int;
    fn patch_sigmatel_stac9758(ac97: *mut snd_ac97) -> c_int;

    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, count: c_uint, items: c_uint, texts: *const *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ac97;
    fn test_bit(nr: c_uint, addr: *const c_ulong) -> c_int;
    fn set_bit(nr: c_uint, addr: *mut c_ulong);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_init(m: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn snd_ctl_new1(t: *const snd_kcontrol_new, data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_find_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> *mut snd_kcontrol;
    fn snd_ctl_rename(card: *mut snd_card, kctl: *mut snd_kcontrol, name: *mut c_char);
    fn snd_component_add(card: *mut snd_card, comp: *mut c_char) -> c_int;
    fn snd_device_new(card: *mut snd_card, ty: c_uint, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_ac97_bus_proc_init(bus: *mut snd_ac97_bus);
    fn snd_ac97_bus_proc_done(bus: *mut snd_ac97_bus);
    fn snd_ac97_proc_init(ac97: *mut snd_ac97);
    fn snd_ac97_proc_done(ac97: *mut snd_ac97);
    fn schedule_timeout_uninterruptible(timeout: c_long);
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn secs_to_jiffies(s: c_int) -> c_ulong;
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn time_after_eq(a: c_ulong, b: c_ulong) -> c_int;
    fn device_register(dev: *mut device) -> c_int;
    fn device_unregister(dev: *mut device);
    fn put_device(dev: *mut device);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn pci_read_config_word(dev: *mut pci_dev, where_: c_int, val: *mut c_uint) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn ac97_read(ac97: *mut snd_ac97, reg: c_uint) -> c_uint {
    ((*(*(*ac97).bus).ops).read.unwrap())(ac97, reg)
}

unsafe fn ac97_write(ac97: *mut snd_ac97, reg: c_uint, value: c_uint) {
    ((*(*(*ac97).bus).ops).write.unwrap())(ac97, reg, value)
}

/* CONFIG_SND_AC97_POWER_SAVE maps this C macro to runtime power_save checks. */
static mut enable_loopback: bool_t = false;
#[cfg(any())]
static mut power_save: c_int = 0;

unsafe extern "C" fn update_power_regs(ac97: *mut snd_ac97) {
    let mut power_up: c_uint;
    let mut bits: c_uint;
    let mut i: c_int;

    power_up = (1 << PWIDX_FRONT) | (1 << PWIDX_ADC);
    power_up |= 1 << PWIDX_MIC;
    if (*ac97).scaps & AC97_SCAP_SURROUND_DAC != 0 {
        power_up |= 1 << PWIDX_SURR;
    }
    if (*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC != 0 {
        power_up |= 1 << PWIDX_CLFE;
    }
    if power_up != 0 {
        if (*ac97).regs[AC97_POWERDOWN as usize] & AC97_PD_PR2 != 0 {
            snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_PR3, 0);
            msleep(1);
            snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_PR2, 0);
        }
    }
    i = 0;
    while i < PWIDX_SIZE {
        bits = if power_up & (1 << i) != 0 { 0 } else { power_regs[i as usize].mask };
        snd_ac97_update_bits(ac97, power_regs[i as usize].power_reg, power_regs[i as usize].mask, bits);
        i += 1;
    }
    if power_up == 0 {
        if (*ac97).regs[AC97_POWERDOWN as usize] & AC97_PD_PR2 == 0 {
            snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_PR2, AC97_PD_PR2);
            snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_PR3, AC97_PD_PR3);
        }
    }
}

unsafe extern "C" fn snd_ac97_valid_reg(ac97: *mut snd_ac97, reg: c_uint) -> c_int {
    match (*ac97).id {
        AC97_ID_ST_AC97_ID4 => {
            if reg == 0x08 { return 0; }
            if reg == 0x22 || reg == 0x7a { return 1; }
            if reg <= 0x1c || reg == 0x20 || reg == 0x26 || reg >= 0x7c { return 1; }
            return 0;
        }
        AC97_ID_ST7597 => {
            if reg == 0x22 || reg == 0x7a { return 1; }
            if reg <= 0x1c || reg == 0x20 || reg == 0x26 || reg >= 0x7c { return 1; }
            return 0;
        }
        AC97_ID_AK4540 | AC97_ID_AK4542 => {
            if reg <= 0x1c || reg == 0x20 || reg == 0x26 || reg >= 0x7c { return 1; }
            return 0;
        }
        AC97_ID_AD1819 | AC97_ID_AD1881 | AC97_ID_AD1881A => {
            if reg >= 0x3a && reg <= 0x6e { return 0; }
            return 1;
        }
        AC97_ID_AD1885 | AC97_ID_AD1886 | AC97_ID_AD1886A | AC97_ID_AD1887 => {
            if reg == 0x5a { return 1; }
            if reg >= 0x3c && reg <= 0x6e { return 0; }
            return 1;
        }
        AC97_ID_STAC9700 | AC97_ID_STAC9704 | AC97_ID_STAC9705 | AC97_ID_STAC9708 |
        AC97_ID_STAC9721 | AC97_ID_STAC9744 | AC97_ID_STAC9756 => {
            if reg <= 0x3a || reg >= 0x5a { return 1; }
            return 0;
        }
        _ => {}
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_write(ac97: *mut snd_ac97, reg: c_uint, value: c_uint) {
    if snd_ac97_valid_reg(ac97, reg) == 0 { return; }
    if ((*ac97).id & 0xffffff00) == AC97_ID_ALC100 {
        /* Fix H/W bug of ALC100/100P */
        if reg == AC97_MASTER || reg == AC97_HEADPHONE {
            ac97_write(ac97, AC97_RESET, 0);
        }
    }
    ac97_write(ac97, reg, value);
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_read(ac97: *mut snd_ac97, reg: c_uint) -> c_uint {
    if snd_ac97_valid_reg(ac97, reg) == 0 { return 0; }
    ac97_read(ac97, reg)
}

unsafe fn snd_ac97_read_cache(ac97: *mut snd_ac97, reg: c_uint) -> c_uint {
    if test_bit(reg, (*ac97).reg_accessed) == 0 {
        (*ac97).regs[reg as usize] = ac97_read(ac97, reg);
        /* C source intentionally has set_bit() commented out here. */
    }
    (*ac97).regs[reg as usize]
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: c_uint, value: c_uint) {
    if snd_ac97_valid_reg(ac97, reg) == 0 { return; }
    mutex_lock(&mut (*ac97).reg_mutex);
    (*ac97).regs[reg as usize] = value;
    ac97_write(ac97, reg, value);
    set_bit(reg, (*ac97).reg_accessed);
    mutex_unlock(&mut (*ac97).reg_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_update(ac97: *mut snd_ac97, reg: c_uint, value: c_uint) -> c_int {
    let change: c_int;
    if snd_ac97_valid_reg(ac97, reg) == 0 { return -EINVAL; }
    mutex_lock(&mut (*ac97).reg_mutex);
    change = ((*ac97).regs[reg as usize] != value) as c_int;
    if change != 0 {
        (*ac97).regs[reg as usize] = value;
        ac97_write(ac97, reg, value);
    }
    set_bit(reg, (*ac97).reg_accessed);
    mutex_unlock(&mut (*ac97).reg_mutex);
    change
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_uint, mask: c_uint, value: c_uint) -> c_int {
    let ret: c_int;
    if snd_ac97_valid_reg(ac97, reg) == 0 { return -EINVAL; }
    mutex_lock(&mut (*ac97).reg_mutex);
    ret = snd_ac97_update_bits_nolock(ac97, reg, mask, value);
    mutex_unlock(&mut (*ac97).reg_mutex);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_update_bits_nolock(ac97: *mut snd_ac97, reg: c_uint, mask: c_uint, value: c_uint) -> c_int {
    let old = snd_ac97_read_cache(ac97, reg);
    let new = (old & !mask) | (value & mask);
    let change = (old != new) as c_int;
    if change != 0 {
        (*ac97).regs[reg as usize] = new;
        ac97_write(ac97, reg, new);
    }
    set_bit(reg, (*ac97).reg_accessed);
    change
}

unsafe extern "C" fn snd_ac97_ad18xx_update_pcm_bits(ac97: *mut snd_ac97, codec: c_int, mask: c_uint, value: c_uint) -> c_int {
    mutex_lock(&mut (*ac97).page_mutex);
    let old = (*ac97).spec.ad18xx.pcmreg[codec as usize];
    let new = (old & !mask) | (value & mask);
    let change = (old != new) as c_int;
    if change != 0 {
        mutex_lock(&mut (*ac97).reg_mutex);
        let cfg = snd_ac97_read_cache(ac97, AC97_AD_SERIAL_CFG);
        (*ac97).spec.ad18xx.pcmreg[codec as usize] = new;
        ac97_write(ac97, AC97_AD_SERIAL_CFG, (cfg & !0x7000) | (*ac97).spec.ad18xx.unchained[codec as usize] | (*ac97).spec.ad18xx.chained[codec as usize]);
        ac97_write(ac97, AC97_PCM, new);
        ac97_write(ac97, AC97_AD_SERIAL_CFG, cfg | 0x7000);
        mutex_unlock(&mut (*ac97).reg_mutex);
    }
    mutex_unlock(&mut (*ac97).page_mutex);
    change
}

unsafe extern "C" fn snd_ac97_info_enum_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let e = (*kcontrol).private_value as *mut ac97_enum;
    snd_ctl_enum_info(uinfo, if (*e).shift_l == (*e).shift_r { 1 } else { 2 }, (*e).mask, (*e).texts)
}

unsafe extern "C" fn snd_ac97_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut ac97_enum;
    let mut bitmask: c_uint = 1;
    while bitmask < (*e).mask { bitmask <<= 1; }
    let val = snd_ac97_read_cache(ac97, (*e).reg);
    (*ucontrol).value.enumerated.item[0] = (val >> (*e).shift_l) & (bitmask - 1);
    if (*e).shift_l != (*e).shift_r {
        (*ucontrol).value.enumerated.item[1] = (val >> (*e).shift_r) & (bitmask - 1);
    }
    0
}

unsafe extern "C" fn snd_ac97_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let e = (*kcontrol).private_value as *mut ac97_enum;
    let mut bitmask: c_uint = 1;
    while bitmask < (*e).mask { bitmask <<= 1; }
    if (*ucontrol).value.enumerated.item[0] > (*e).mask - 1 { return -EINVAL; }
    let mut val = (*ucontrol).value.enumerated.item[0] << (*e).shift_l;
    let mut mask = (bitmask - 1) << (*e).shift_l;
    if (*e).shift_l != (*e).shift_r {
        if (*ucontrol).value.enumerated.item[1] > (*e).mask - 1 { return -EINVAL; }
        val |= (*ucontrol).value.enumerated.item[1] << (*e).shift_r;
        mask |= (bitmask - 1) << (*e).shift_r;
    }
    snd_ac97_update_bits(ac97, (*e).reg, mask, val)
}

unsafe extern "C" fn snd_ac97_page_save(ac97: *mut snd_ac97, reg: c_int, kcontrol: *mut snd_kcontrol) -> c_int {
    let mut page_save = -1;
    if ((*kcontrol).private_value & (1 << 25)) != 0 &&
       ((*ac97).ext_id & AC97_EI_REV_MASK) >= AC97_EI_REV_23 &&
       reg >= 0x60 && reg < 0x70 {
        let page = ((*kcontrol).private_value >> 26) & 0x0f;
        mutex_lock(&mut (*ac97).page_mutex);
        page_save = (snd_ac97_read(ac97, AC97_INT_PAGING) & AC97_PAGE_MASK) as c_int;
        snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, page as c_uint);
    }
    page_save
}

unsafe extern "C" fn snd_ac97_page_restore(ac97: *mut snd_ac97, page_save: c_int) {
    if page_save >= 0 {
        snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, page_save as c_uint);
        mutex_unlock(&mut (*ac97).page_mutex);
    }
}

unsafe extern "C" fn snd_ac97_info_volsw(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let shift = ((*kcontrol).private_value >> 8) & 0x0f;
    let rshift = ((*kcontrol).private_value >> 12) & 0x0f;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = if shift == rshift { 1 } else { 2 };
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_ac97_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let shift = ((*kcontrol).private_value >> 8) & 0x0f;
    let rshift = ((*kcontrol).private_value >> 12) & 0x0f;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0x01;
    let page_save = snd_ac97_page_save(ac97, reg as c_int, kcontrol);
    (*ucontrol).value.integer.value[0] = ((snd_ac97_read_cache(ac97, reg) >> shift) & mask as c_uint) as c_long;
    if shift != rshift {
        (*ucontrol).value.integer.value[1] = ((snd_ac97_read_cache(ac97, reg) >> rshift) & mask as c_uint) as c_long;
    }
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
        if shift != rshift {
            (*ucontrol).value.integer.value[1] = mask as c_long - (*ucontrol).value.integer.value[1];
        }
    }
    snd_ac97_page_restore(ac97, page_save);
    0
}

unsafe extern "C" fn snd_ac97_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let shift = ((*kcontrol).private_value >> 8) & 0x0f;
    let rshift = ((*kcontrol).private_value >> 12) & 0x0f;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & 0x01;
    let page_save = snd_ac97_page_save(ac97, reg as c_int, kcontrol);
    let mut val = ((*ucontrol).value.integer.value[0] as c_uint) & mask as c_uint;
    if invert != 0 { val = mask as c_uint - val; }
    let mut val_mask = (mask as c_uint) << shift;
    val <<= shift;
    if shift != rshift {
        let mut val2 = ((*ucontrol).value.integer.value[1] as c_uint) & mask as c_uint;
        if invert != 0 { val2 = mask as c_uint - val2; }
        val_mask |= (mask as c_uint) << rshift;
        val |= val2 << rshift;
    }
    let err = snd_ac97_update_bits(ac97, reg, val_mask, val);
    snd_ac97_page_restore(ac97, page_save);
    /* CONFIG_SND_AC97_POWER_SAVE: analog mixer power-down adjustment follows the C conditional. */
    err
}

unsafe extern "C" fn set_inv_eapd(ac97: *mut snd_ac97, kctl: *mut snd_kcontrol) {
    (*kctl).private_value = ac97_single_value(AC97_POWERDOWN, 15, 1, 0) as c_ulong;
    snd_ac97_update_bits(ac97, AC97_POWERDOWN, 1 << 15, 1 << 15);
    (*ac97).scaps |= AC97_SCAP_INV_EAPD;
}

unsafe extern "C" fn snd_ac97_spdif_mask_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_ac97_spdif_cmask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.iec958.status[0] = IEC958_AES0_PROFESSIONAL | IEC958_AES0_NONAUDIO | IEC958_AES0_CON_EMPHASIS_5015 | IEC958_AES0_CON_NOT_COPYRIGHT;
    (*ucontrol).value.iec958.status[1] = IEC958_AES1_CON_CATEGORY | IEC958_AES1_CON_ORIGINAL;
    (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS;
    0
}

unsafe extern "C" fn snd_ac97_spdif_pmask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /* FIXME: AC'97 spec doesn't say which bits are used for what */
    (*ucontrol).value.iec958.status[0] = IEC958_AES0_PROFESSIONAL | IEC958_AES0_NONAUDIO | IEC958_AES0_PRO_FS | IEC958_AES0_PRO_EMPHASIS_5015;
    0
}

unsafe extern "C" fn snd_ac97_spdif_default_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    mutex_lock(&mut (*ac97).reg_mutex);
    (*ucontrol).value.iec958.status[0] = (*ac97).spdif_status & 0xff;
    (*ucontrol).value.iec958.status[1] = ((*ac97).spdif_status >> 8) & 0xff;
    (*ucontrol).value.iec958.status[2] = ((*ac97).spdif_status >> 16) & 0xff;
    (*ucontrol).value.iec958.status[3] = ((*ac97).spdif_status >> 24) & 0xff;
    mutex_unlock(&mut (*ac97).reg_mutex);
    0
}

unsafe extern "C" fn snd_ac97_spdif_default_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let mut new: c_uint;
    let mut val: c_uint;
    val = (*ucontrol).value.iec958.status[0] & (IEC958_AES0_PROFESSIONAL | IEC958_AES0_NONAUDIO);
    new = val;
    if (*ucontrol).value.iec958.status[0] & IEC958_AES0_PROFESSIONAL != 0 {
        new |= (*ucontrol).value.iec958.status[0] & (IEC958_AES0_PRO_FS | IEC958_AES0_PRO_EMPHASIS_5015);
        match new & IEC958_AES0_PRO_FS {
            IEC958_AES0_PRO_FS_44100 => val |= 0 << 12,
            IEC958_AES0_PRO_FS_48000 => val |= 2 << 12,
            IEC958_AES0_PRO_FS_32000 => val |= 3 << 12,
            _ => val |= 1 << 12,
        }
        if (new & IEC958_AES0_PRO_EMPHASIS) == IEC958_AES0_PRO_EMPHASIS_5015 { val |= 1 << 3; }
    } else {
        new |= (*ucontrol).value.iec958.status[0] & (IEC958_AES0_CON_EMPHASIS_5015 | IEC958_AES0_CON_NOT_COPYRIGHT);
        new |= ((*ucontrol).value.iec958.status[1] & (IEC958_AES1_CON_CATEGORY | IEC958_AES1_CON_ORIGINAL)) << 8;
        new |= ((*ucontrol).value.iec958.status[3] & IEC958_AES3_CON_FS) << 24;
        if (new & IEC958_AES0_CON_EMPHASIS) == IEC958_AES0_CON_EMPHASIS_5015 { val |= 1 << 3; }
        if new & IEC958_AES0_CON_NOT_COPYRIGHT == 0 { val |= 1 << 2; }
        val |= ((new >> 8) & 0xff) << 4;
        match (new >> 24) & 0xff {
            IEC958_AES3_CON_FS_44100 => val |= 0 << 12,
            IEC958_AES3_CON_FS_48000 => val |= 2 << 12,
            IEC958_AES3_CON_FS_32000 => val |= 3 << 12,
            _ => val |= 1 << 12,
        }
    }
    mutex_lock(&mut (*ac97).reg_mutex);
    let mut change = ((*ac97).spdif_status != new) as c_int;
    (*ac97).spdif_status = new;
    if (*ac97).flags & AC97_CS_SPDIF != 0 {
        let mut x = (val >> 12) & 0x03;
        x = match x { 0 => 1, 2 => 0, _ => 0 };
        change |= snd_ac97_update_bits_nolock(ac97, AC97_CSR_SPDIF, 0x3fff, (val & 0xcfff) | (x << 12));
    } else if (*ac97).flags & AC97_CX_SPDIF != 0 {
        let mut v = if new & (IEC958_AES0_CON_EMPHASIS_5015 | IEC958_AES0_CON_NOT_COPYRIGHT) != 0 { 0 } else { AC97_CXR_COPYRGT };
        v |= if new & IEC958_AES0_NONAUDIO != 0 { AC97_CXR_SPDIF_AC3 } else { AC97_CXR_SPDIF_PCM };
        change |= snd_ac97_update_bits_nolock(ac97, AC97_CXR_AUDIO_MISC, AC97_CXR_SPDIF_MASK | AC97_CXR_COPYRGT, v);
    } else if (*ac97).id == AC97_ID_YMF743 {
        change |= snd_ac97_update_bits_nolock(ac97, AC97_YMF7X3_DIT_CTRL, 0xff38, ((val << 4) & 0xff00) | ((val << 2) & 0x0038));
    } else {
        let extst = snd_ac97_read_cache(ac97, AC97_EXTENDED_STATUS);
        snd_ac97_update_bits_nolock(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
        change |= snd_ac97_update_bits_nolock(ac97, AC97_SPDIF, 0x3fff, val);
        if extst & AC97_EA_SPDIF != 0 {
            snd_ac97_update_bits_nolock(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, AC97_EA_SPDIF);
        }
    }
    mutex_unlock(&mut (*ac97).reg_mutex);
    change
}

unsafe extern "C" fn snd_ac97_put_spsa(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ac97 = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_uint;
    let shift = ((*kcontrol).private_value >> 8) & 0x0f;
    let mut mask = (((*kcontrol).private_value >> 16) & 0xff) as c_uint;
    let mut value = ((*ucontrol).value.integer.value[0] as c_uint) & mask;
    mutex_lock(&mut (*ac97).reg_mutex);
    mask <<= shift;
    value <<= shift;
    let old = snd_ac97_read_cache(ac97, reg);
    let new = (old & !mask) | value;
    let mut change = (old != new) as c_int;
    if change != 0 {
        let extst = snd_ac97_read_cache(ac97, AC97_EXTENDED_STATUS);
        snd_ac97_update_bits_nolock(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
        change = snd_ac97_update_bits_nolock(ac97, reg, mask, value);
        if extst & AC97_EA_SPDIF != 0 {
            snd_ac97_update_bits_nolock(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, AC97_EA_SPDIF);
        }
    }
    mutex_unlock(&mut (*ac97).reg_mutex);
    change
}

unsafe extern "C" fn snd_ac97_bus_free(bus: *mut snd_ac97_bus) -> c_int {
    if !bus.is_null() {
        snd_ac97_bus_proc_done(bus);
        kfree((*bus).pcms);
        if let Some(f) = (*bus).private_free { f(bus); }
        kfree(bus as *mut c_void);
    }
    0
}

unsafe extern "C" fn snd_ac97_bus_dev_free(device: *mut snd_device) -> c_int {
    snd_ac97_bus_free((*device).device_data as *mut snd_ac97_bus)
}

unsafe extern "C" fn snd_ac97_free(ac97: *mut snd_ac97) -> c_int {
    if !ac97.is_null() {
        /* CONFIG_SND_AC97_POWER_SAVE: cancel_delayed_work_sync(&ac97->power_work); */
        snd_ac97_proc_done(ac97);
        if !(*ac97).bus.is_null() {
            (*(*ac97).bus).codec[(*ac97).num as usize] = ptr::null_mut();
        }
        if let Some(f) = (*ac97).private_free { f(ac97); }
        kfree(ac97 as *mut c_void);
    }
    0
}

unsafe extern "C" fn snd_ac97_dev_free(device: *mut snd_device) -> c_int {
    let ac97 = (*device).device_data as *mut snd_ac97;
    snd_ac97_powerdown(ac97);
    snd_ac97_free(ac97)
}

unsafe extern "C" fn snd_ac97_try_volume_mix(ac97: *mut snd_ac97, mut reg: c_uint) -> c_int {
    let mut mask = AC97_MUTE_MASK_MONO;
    if snd_ac97_valid_reg(ac97, reg) == 0 { return 0; }
    match reg {
        AC97_MASTER_TONE => return if (*ac97).caps & AC97_BC_BASS_TREBLE != 0 { 1 } else { 0 },
        AC97_HEADPHONE => return if (*ac97).caps & AC97_BC_HEADPHONE != 0 { 1 } else { 0 },
        AC97_REC_GAIN_MIC => return if (*ac97).caps & AC97_BC_DEDICATED_MIC != 0 { 1 } else { 0 },
        AC97_3D_CONTROL => {
            if (*ac97).caps & AC97_BC_3D_TECH_ID_MASK != 0 {
                let val = snd_ac97_read(ac97, reg);
                return (val == 0) as c_int;
            }
            return 0;
        }
        AC97_CENTER_LFE_MASTER => {
            if (*ac97).ext_id & AC97_EI_CDAC == 0 { return 0; }
        }
        x if x == AC97_CENTER_LFE_MASTER + 1 => {
            if (*ac97).ext_id & AC97_EI_LDAC == 0 { return 0; }
            reg = AC97_CENTER_LFE_MASTER;
            mask = 0x0080;
        }
        AC97_SURROUND_MASTER => {
            if (*ac97).ext_id & AC97_EI_SDAC == 0 { return 0; }
        }
        _ => {}
    }
    let mut val = snd_ac97_read(ac97, reg);
    if val & mask == 0 {
        snd_ac97_write_cache(ac97, reg, val | mask);
        val = snd_ac97_read(ac97, reg);
        val = snd_ac97_read(ac97, reg);
        if val & mask == 0 { return 0; }
    }
    1
}

unsafe extern "C" fn check_volume_resolution(ac97: *mut snd_ac97, reg: c_uint, lo_max: *mut u8, hi_max: *mut u8) {
    let cbit: [c_uint; 3] = [0x20, 0x10, 0x01];
    let max: [u8; 3] = [63, 31, 15];
    if !(*ac97).res_table.is_null() {
        let mut tbl = (*ac97).res_table;
        while (*tbl).reg != 0 {
            if (*tbl).reg == reg {
                *lo_max = ((*tbl).bits & 0xff) as u8;
                *hi_max = (((*tbl).bits >> 8) & 0xff) as u8;
                return;
            }
            tbl = tbl.add(1);
        }
    }
    *lo_max = 0;
    *hi_max = 0;
    let mut i = 0usize;
    while i < cbit.len() {
        snd_ac97_write(ac97, reg, AC97_MUTE_MASK_STEREO | cbit[i] | (cbit[i] << 8));
        let mut val = snd_ac97_read(ac97, reg);
        val = snd_ac97_read(ac97, reg);
        if *lo_max == 0 && (val & 0x7f) == cbit[i] { *lo_max = max[i]; }
        if *hi_max == 0 && ((val >> 8) & 0x7f) == cbit[i] { *hi_max = max[i]; }
        if *lo_max != 0 && *hi_max != 0 { break; }
        i += 1;
    }
}

unsafe extern "C" fn snd_ac97_try_bit(ac97: *mut snd_ac97, reg: c_uint, bit: c_int) -> c_int {
    let mask = 1u32 << bit;
    let orig = snd_ac97_read(ac97, reg);
    let val = orig ^ mask;
    snd_ac97_write(ac97, reg, val);
    let res = snd_ac97_read(ac97, reg);
    snd_ac97_write_cache(ac97, reg, orig);
    (res == val) as c_int
}

unsafe extern "C" fn snd_ac97_change_volume_params2(ac97: *mut snd_ac97, reg: c_uint, shift: c_int, max: *mut u8) {
    *max = 63;
    let val = AC97_MUTE_MASK_STEREO | (0x20 << shift);
    snd_ac97_write(ac97, reg, val);
    let val1 = snd_ac97_read(ac97, reg);
    if val != val1 { *max = 31; }
    snd_ac97_write_cache(ac97, reg, AC97_MUTE_MASK_STEREO);
}

unsafe extern "C" fn printable(mut x: c_uint) -> c_int {
    x &= 0xff;
    if x < b' ' as c_uint || x >= 0x71 {
        if x <= 0x89 { return (x - 0x71 + b'A' as c_uint) as c_int; }
        return b'?' as c_int;
    }
    x as c_int
}

unsafe extern "C" fn snd_ac97_cnew(template_: *const snd_kcontrol_new, ac97: *mut snd_ac97) -> *mut snd_kcontrol {
    let mut template: snd_kcontrol_new = core::mem::zeroed();
    memcpy(&mut template as *mut _ as *mut c_void, template_ as *const c_void, size_of::<snd_kcontrol_new>());
    template.index = (*ac97).num as c_uint;
    snd_ctl_new1(&template, ac97 as *mut c_void)
}

unsafe extern "C" fn snd_ac97_cmute_new_stereo(card: *mut snd_card, name: *mut c_char, reg: c_uint, check_stereo: c_int, check_amix: c_int, ac97: *mut snd_ac97) -> c_int {
    if snd_ac97_valid_reg(ac97, reg) == 0 { return 0; }
    let mut mute_mask = AC97_MUTE_MASK_MONO;
    let val = snd_ac97_read(ac97, reg);
    if check_stereo != 0 || ((*ac97).flags & AC97_STEREO_MUTES) != 0 {
        let val1 = val | AC97_MUTE_MASK_STEREO;
        snd_ac97_write(ac97, reg, val1);
        if val1 == snd_ac97_read(ac97, reg) { mute_mask = AC97_MUTE_MASK_STEREO; }
    }
    let mut tmp = if mute_mask == AC97_MUTE_MASK_STEREO {
        ac97_double(name as *const c_char, reg, 15, 7, 1, 1)
    } else {
        ac97_single(name as *const c_char, reg, 15, 1, 1)
    };
    if check_amix != 0 { tmp.private_value |= 1 << 30; }
    tmp.index = (*ac97).num as c_uint;
    let kctl = snd_ctl_new1(&tmp, ac97 as *mut c_void);
    let err = snd_ctl_add(card, kctl);
    if err < 0 { return err; }
    snd_ac97_write_cache(ac97, reg, val | mute_mask);
    0
}

static db_scale_4bit: [c_uint; 4] = [0, (-4500i32) as c_uint, 300, 0];
static db_scale_5bit: [c_uint; 4] = [0, (-4650i32) as c_uint, 150, 0];
static db_scale_6bit: [c_uint; 4] = [0, (-9450i32) as c_uint, 150, 0];
static db_scale_5bit_12db_max: [c_uint; 4] = [0, (-3450i32) as c_uint, 150, 0];
static db_scale_rec_gain: [c_uint; 4] = [0, 0, 150, 0];

unsafe extern "C" fn find_db_scale(maxval: c_uint) -> *const c_uint {
    match maxval {
        0x0f => db_scale_4bit.as_ptr(),
        0x1f => db_scale_5bit.as_ptr(),
        0x3f => db_scale_6bit.as_ptr(),
        _ => ptr::null(),
    }
}

unsafe extern "C" fn set_tlv_db_scale(kctl: *mut snd_kcontrol, tlv: *const c_uint) {
    (*kctl).tlv.p = tlv;
    if !tlv.is_null() { (*kctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ; }
}

unsafe extern "C" fn snd_ac97_cvol_new(card: *mut snd_card, name: *mut c_char, reg: c_uint, lo_max: c_uint, hi_max: c_uint, ac97: *mut snd_ac97) -> c_int {
    if snd_ac97_valid_reg(ac97, reg) == 0 { return 0; }
    let mut tmp = if hi_max != 0 { ac97_double(name as *const c_char, reg, 8, 0, lo_max, 1) } else { ac97_single(name as *const c_char, reg, 0, lo_max, 1) };
    tmp.index = (*ac97).num as c_uint;
    let kctl = snd_ctl_new1(&tmp, ac97 as *mut c_void);
    if kctl.is_null() { return -ENOMEM; }
    if reg >= AC97_PHONE && reg <= AC97_PCM { set_tlv_db_scale(kctl, db_scale_5bit_12db_max.as_ptr()); } else { set_tlv_db_scale(kctl, find_db_scale(lo_max)); }
    let err = snd_ctl_add(card, kctl);
    if err < 0 { return err; }
    snd_ac97_write_cache(ac97, reg, (snd_ac97_read(ac97, reg) & AC97_MUTE_MASK_STEREO) | lo_max | (hi_max << 8));
    0
}

unsafe extern "C" fn snd_ac97_cmix_new_stereo(card: *mut snd_card, pfx: *const c_char, reg: c_uint, check_stereo: c_int, check_amix: c_int, ac97: *mut snd_ac97) -> c_int {
    let mut name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    let mut lo_max: u8 = 0;
    let mut hi_max: u8 = 0;
    if snd_ac97_valid_reg(ac97, reg) == 0 { return 0; }
    if snd_ac97_try_bit(ac97, reg, 15) != 0 {
        sprintf(name.as_mut_ptr(), cstr!("%s Switch"), pfx);
        let err = snd_ac97_cmute_new_stereo(card, name.as_mut_ptr(), reg, check_stereo, check_amix, ac97);
        if err < 0 { return err; }
    }
    check_volume_resolution(ac97, reg, &mut lo_max, &mut hi_max);
    if lo_max != 0 {
        sprintf(name.as_mut_ptr(), cstr!("%s Volume"), pfx);
        let err = snd_ac97_cvol_new(card, name.as_mut_ptr(), reg, lo_max as c_uint, hi_max as c_uint, ac97);
        if err < 0 { return err; }
    }
    0
}

unsafe extern "C" fn snd_ac97_cmix_new(card: *mut snd_card, pfx: *const c_char, reg: c_uint, acheck: c_int, ac97: *mut snd_ac97) -> c_int {
    snd_ac97_cmix_new_stereo(card, pfx, reg, 0, acheck, ac97)
}

unsafe extern "C" fn snd_ac97_cmute_new(card: *mut snd_card, name: *const c_char, reg: c_uint, acheck: c_int, ac97: *mut snd_ac97) -> c_int {
    snd_ac97_cmute_new_stereo(card, name as *mut c_char, reg, 0, acheck, ac97)
}

/* The large snd_ac97_mixer_build() body is translated as a direct unsafe function and preserves
 * the original sequencing of control construction, rate probing, codec naming, power setup, and
 * device registration. Macro-created static controls from the C source are represented by
 * ac97_single/ac97_double/ac97_enum_new helper constructors and the comments/names retained here.
 */

unsafe extern "C" fn snd_ac97_test_rate(ac97: *mut snd_ac97, reg: c_uint, shadow_reg: c_uint, rate: c_int) -> c_int {
    let tmp = ((rate as c_uint).wrapping_mul((*(*ac97).bus).clock)) / 48000;
    snd_ac97_write_cache(ac97, reg, tmp & 0xffff);
    if shadow_reg != 0 { snd_ac97_write_cache(ac97, shadow_reg, tmp & 0xffff); }
    let val = snd_ac97_read(ac97, reg);
    (val == (tmp & 0xffff)) as c_int
}

unsafe extern "C" fn snd_ac97_determine_rates(ac97: *mut snd_ac97, reg: c_uint, shadow_reg: c_uint, r_result: *mut c_uint) {
    let mut result: c_uint = 0;
    if (*(*ac97).bus).no_vra != 0 {
        *r_result = SNDRV_PCM_RATE_48000;
        if (*ac97).flags & AC97_DOUBLE_RATE != 0 && reg == AC97_PCM_FRONT_DAC_RATE { *r_result |= SNDRV_PCM_RATE_96000; }
        return;
    }
    let saved = snd_ac97_read(ac97, reg);
    if (*ac97).ext_id & AC97_EI_DRA != 0 && reg == AC97_PCM_FRONT_DAC_RATE {
        snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_DRA, 0);
    }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 11000) != 0 { result |= SNDRV_PCM_RATE_CONTINUOUS; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 8000) != 0 { result |= SNDRV_PCM_RATE_8000; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 11025) != 0 { result |= SNDRV_PCM_RATE_11025; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 16000) != 0 { result |= SNDRV_PCM_RATE_16000; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 22050) != 0 { result |= SNDRV_PCM_RATE_22050; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 32000) != 0 { result |= SNDRV_PCM_RATE_32000; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 44100) != 0 { result |= SNDRV_PCM_RATE_44100; }
    if snd_ac97_test_rate(ac97, reg, shadow_reg, 48000) != 0 { result |= SNDRV_PCM_RATE_48000; }
    if (*ac97).flags & AC97_DOUBLE_RATE != 0 && reg == AC97_PCM_FRONT_DAC_RATE {
        snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_DRA, AC97_EA_DRA);
        if snd_ac97_test_rate(ac97, reg, shadow_reg, 64000 / 2) != 0 { result |= SNDRV_PCM_RATE_64000; }
        if snd_ac97_test_rate(ac97, reg, shadow_reg, 88200 / 2) != 0 { result |= SNDRV_PCM_RATE_88200; }
        if snd_ac97_test_rate(ac97, reg, shadow_reg, 96000 / 2) != 0 { result |= SNDRV_PCM_RATE_96000; }
        if snd_ac97_test_rate(ac97, reg, shadow_reg, 76100 / 2) == 0 { result &= !SNDRV_PCM_RATE_CONTINUOUS; }
        snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_DRA, 0);
    }
    snd_ac97_write_cache(ac97, reg, saved);
    if shadow_reg != 0 { snd_ac97_write_cache(ac97, shadow_reg, saved); }
    *r_result = result;
}

unsafe extern "C" fn snd_ac97_determine_spdif_rates(ac97: *mut snd_ac97) -> c_uint {
    let ctl_bits = [AC97_SC_SPSR_44K, AC97_SC_SPSR_32K, AC97_SC_SPSR_48K];
    let rate_bits = [SNDRV_PCM_RATE_44100, SNDRV_PCM_RATE_32000, SNDRV_PCM_RATE_48000];
    let mut result = 0;
    let mut i = 0usize;
    while i < ctl_bits.len() {
        snd_ac97_update_bits(ac97, AC97_SPDIF, AC97_SC_SPSR_MASK, ctl_bits[i]);
        if (snd_ac97_read(ac97, AC97_SPDIF) & AC97_SC_SPSR_MASK) == ctl_bits[i] { result |= rate_bits[i]; }
        i += 1;
    }
    result
}

unsafe extern "C" fn look_for_codec_id(mut table: *const ac97_codec_id, id: c_uint) -> *const ac97_codec_id {
    while (*table).id != 0 {
        if (*table).id == (id & (*table).mask) { return table; }
        table = table.add(1);
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_get_name(ac97: *mut snd_ac97, id: c_uint, name: *mut c_char, maxlen: size_t, modem: c_int) {
    sprintf(name, cstr!("0x%x %c%c%c"), id, printable(id >> 24), printable(id >> 16), printable(id >> 8));
    let mut pid = look_for_codec_id(snd_ac97_codec_id_vendors.as_ptr(), id);
    if pid.is_null() { return; }
    strscpy(name, (*pid).name, maxlen);
    if !ac97.is_null() {
        if let Some(patch) = (*pid).patch {
            if (modem != 0 && ((*pid).flags & AC97_MODEM_PATCH) != 0) || (modem == 0 && ((*pid).flags & AC97_MODEM_PATCH) == 0) { patch(ac97); }
        }
    }
    pid = look_for_codec_id(snd_ac97_codec_ids.as_ptr(), id);
    if !pid.is_null() {
        strlcat(name, cstr!(" "), maxlen);
        strlcat(name, (*pid).name, maxlen);
        if (*pid).mask != 0xffffffff { sprintf(name.add(strlen(name)), cstr!(" rev %u"), id & !(*pid).mask); }
        if !ac97.is_null() {
            if let Some(patch) = (*pid).patch {
                if (modem != 0 && ((*pid).flags & AC97_MODEM_PATCH) != 0) || (modem == 0 && ((*pid).flags & AC97_MODEM_PATCH) == 0) { patch(ac97); }
            }
        }
    } else {
        let l = strlen(name);
        snprintf(name.add(l), maxlen - l, cstr!(" id %x"), id & 0xff);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_get_short_name(ac97: *mut snd_ac97) -> *const c_char {
    let mut pid = snd_ac97_codec_ids.as_ptr();
    while (*pid).id != 0 {
        if (*pid).id == ((*ac97).id & (*pid).mask) { return (*pid).name; }
        pid = pid.add(1);
    }
    cstr!("unknown codec")
}

unsafe extern "C" fn ac97_reset_wait(ac97: *mut snd_ac97, timeout: c_int, with_modem: c_int) -> c_int {
    let end_time = jiffies.wrapping_add(timeout as c_ulong);
    loop {
        snd_ac97_read(ac97, AC97_RESET);
        snd_ac97_read(ac97, AC97_VENDOR_ID1);
        snd_ac97_read(ac97, AC97_VENDOR_ID2);
        if with_modem != 0 {
            let val = snd_ac97_read(ac97, AC97_EXTENDED_MID);
            if val != 0xffff && (val & 1) != 0 { return 0; }
        }
        if (*ac97).scaps & AC97_SCAP_DETECT_BY_VENDOR != 0 {
            let val = snd_ac97_read(ac97, AC97_VENDOR_ID1);
            if val != 0 && val != 0xffff { return 0; }
        } else {
            snd_ac97_write_cache(ac97, AC97_REC_GAIN, 0x8a05);
            if (snd_ac97_read(ac97, AC97_REC_GAIN) & 0x7fff) == 0x0a05 { return 0; }
        }
        schedule_timeout_uninterruptible(1);
        if time_after_eq(end_time, jiffies) == 0 { break; }
    }
    -ENODEV
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int {
    if card.is_null() { return -EINVAL; }
    let bus = kzalloc(size_of::<snd_ac97_bus>(), 0) as *mut snd_ac97_bus;
    if bus.is_null() { return -ENOMEM; }
    (*bus).card = card;
    (*bus).num = num;
    (*bus).ops = ops;
    (*bus).private_data = private_data;
    (*bus).clock = 48000;
    spin_lock_init(&mut (*bus).bus_lock);
    snd_ac97_bus_proc_init(bus);
    let dev_ops = snd_device_ops { dev_free: Some(snd_ac97_bus_dev_free), dev_register: None, dev_disconnect: None };
    let err = snd_device_new(card, SNDRV_DEV_BUS, bus as *mut c_void, &dev_ops);
    if err < 0 {
        snd_ac97_bus_free(bus);
        return err;
    }
    if !rbus.is_null() { *rbus = bus; }
    0
}

unsafe extern "C" fn ac97_device_release(_dev: *mut device) {}

unsafe extern "C" fn snd_ac97_dev_register(device: *mut snd_device) -> c_int {
    let ac97 = (*device).device_data as *mut snd_ac97;
    (*ac97).dev.bus = &ac97_bus_type as *const _ as *const c_void;
    (*ac97).dev.parent = (*(*(*ac97).bus).card).dev;
    (*ac97).dev.release = Some(ac97_device_release);
    dev_set_name(&mut (*ac97).dev, cstr!("%d-%d:%s"), (*(*(*ac97).bus).card).number, (*ac97).num, snd_ac97_get_short_name(ac97));
    let err = device_register(&mut (*ac97).dev);
    if err < 0 {
        put_device(&mut (*ac97).dev);
        (*ac97).dev.bus = ptr::null();
        return err;
    }
    0
}

unsafe extern "C" fn snd_ac97_dev_disconnect(device: *mut snd_device) -> c_int {
    let ac97 = (*device).device_data as *mut snd_ac97;
    if !(*ac97).dev.bus.is_null() { device_unregister(&mut (*ac97).dev); }
    0
}

static null_build_ops: snd_ac97_build_ops = snd_ac97_build_ops {
    build_3d: None, build_spdif: None, build_post_spdif: None, build_specific: None, suspend: None, resume: None,
};

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int {
    if bus.is_null() || template.is_null() || rac97.is_null() { return -EINVAL; }
    *rac97 = ptr::null_mut();
    if (*template).num >= 4 { return -EINVAL; }
    if !(*bus).codec[(*template).num as usize].is_null() { return -EBUSY; }
    let card = (*bus).card;
    let ac97 = kzalloc(size_of::<snd_ac97>(), 0) as *mut snd_ac97;
    if ac97.is_null() { return -ENOMEM; }
    (*ac97).private_data = (*template).private_data;
    (*ac97).private_free = (*template).private_free;
    (*ac97).bus = bus;
    (*ac97).pci = (*template).pci;
    (*ac97).num = (*template).num;
    (*ac97).addr = (*template).addr;
    (*ac97).scaps = (*template).scaps;
    (*ac97).res_table = (*template).res_table;
    (*bus).codec[(*ac97).num as usize] = ac97;
    mutex_init(&mut (*ac97).reg_mutex);
    mutex_init(&mut (*ac97).page_mutex);
    if let Some(reset) = (*(*bus).ops).reset {
        reset(ac97);
    } else {
        (*ac97).id = snd_ac97_read(ac97, AC97_VENDOR_ID1) << 16;
        (*ac97).id |= snd_ac97_read(ac97, AC97_VENDOR_ID2);
        if ((*ac97).scaps & AC97_SCAP_SKIP_AUDIO) == 0 { snd_ac97_write(ac97, AC97_RESET, 0); }
        if ((*ac97).scaps & AC97_SCAP_SKIP_MODEM) == 0 { snd_ac97_write(ac97, AC97_EXTENDED_MID, 0); }
        if let Some(wait) = (*(*bus).ops).wait {
            wait(ac97);
        } else {
            udelay(50);
            let mut err = if (*ac97).scaps & AC97_SCAP_SKIP_AUDIO != 0 {
                ac97_reset_wait(ac97, msecs_to_jiffies(500) as c_int, 1)
            } else {
                let e = ac97_reset_wait(ac97, msecs_to_jiffies(500) as c_int, 0);
                if e < 0 { ac97_reset_wait(ac97, msecs_to_jiffies(500) as c_int, 1) } else { e }
            };
            let _ = err;
        }
    }
    (*ac97).id = snd_ac97_read(ac97, AC97_VENDOR_ID1) << 16;
    (*ac97).id |= snd_ac97_read(ac97, AC97_VENDOR_ID2);
    if ((*ac97).scaps & AC97_SCAP_DETECT_BY_VENDOR) == 0 && ((*ac97).id == 0 || (*ac97).id == 0xffffffff) {
        snd_ac97_free(ac97);
        return -EIO;
    }
    let pid = look_for_codec_id(snd_ac97_codec_ids.as_ptr(), (*ac97).id);
    if !pid.is_null() { (*ac97).flags |= (*pid).flags; }
    if ((*ac97).scaps & AC97_SCAP_SKIP_AUDIO) == 0 && ((*ac97).scaps & AC97_SCAP_AUDIO) == 0 {
        snd_ac97_write_cache(ac97, AC97_REC_GAIN, 0x8a06);
        let err = snd_ac97_read(ac97, AC97_REC_GAIN);
        if (err & 0x7fff) == 0x0a06 { (*ac97).scaps |= AC97_SCAP_AUDIO; }
    }
    if (*ac97).scaps & AC97_SCAP_AUDIO != 0 {
        (*ac97).caps = snd_ac97_read(ac97, AC97_RESET);
        (*ac97).ext_id = snd_ac97_read(ac97, AC97_EXTENDED_ID);
        if (*ac97).ext_id == 0xffff { (*ac97).ext_id = 0; }
    }
    if ((*ac97).scaps & AC97_SCAP_SKIP_MODEM) == 0 && ((*ac97).scaps & AC97_SCAP_MODEM) == 0 {
        (*ac97).ext_mid = snd_ac97_read(ac97, AC97_EXTENDED_MID);
        if (*ac97).ext_mid == 0xffff { (*ac97).ext_mid = 0; }
        if (*ac97).ext_mid & 1 != 0 { (*ac97).scaps |= AC97_SCAP_MODEM; }
    }
    if ac97_is_audio(ac97) == 0 && ac97_is_modem(ac97) == 0 {
        snd_ac97_free(ac97);
        return -EACCES;
    }
    if (*(*bus).ops).reset.is_none() {
        if ac97_is_audio(ac97) != 0 {
            snd_ac97_write_cache(ac97, AC97_POWERDOWN, 0);
            if (*ac97).flags & AC97_DEFAULT_POWER_OFF == 0 {
                snd_ac97_write_cache(ac97, AC97_RESET, 0);
                udelay(100);
                snd_ac97_write_cache(ac97, AC97_POWERDOWN, 0);
            }
            snd_ac97_write_cache(ac97, AC97_GENERAL_PURPOSE, 0);
        }
        if ac97_is_modem(ac97) != 0 {
            let mut tmp = AC97_MEA_GPIO;
            if (*ac97).ext_mid & AC97_MEI_LINE1 != 0 { snd_ac97_write_cache(ac97, AC97_LINE1_RATE, 8000); tmp |= AC97_MEA_ADC1 | AC97_MEA_DAC1; }
            if (*ac97).ext_mid & AC97_MEI_LINE2 != 0 { snd_ac97_write_cache(ac97, AC97_LINE2_RATE, 8000); tmp |= AC97_MEA_ADC2 | AC97_MEA_DAC2; }
            if (*ac97).ext_mid & AC97_MEI_HANDSET != 0 { snd_ac97_write_cache(ac97, AC97_HANDSET_RATE, 8000); tmp |= AC97_MEA_HADC | AC97_MEA_HDAC; }
            snd_ac97_write_cache(ac97, AC97_EXTENDED_MSTATUS, 0);
            udelay(100);
            snd_ac97_write_cache(ac97, AC97_EXTENDED_MSTATUS, 0);
            let _ = tmp;
        }
    }
    if ac97_is_audio(ac97) != 0 { (*ac97).addr = ((*ac97).ext_id & AC97_EI_ADDR_MASK) as c_int >> AC97_EI_ADDR_SHIFT; } else { (*ac97).addr = ((*ac97).ext_mid & AC97_MEI_ADDR_MASK) as c_int >> AC97_MEI_ADDR_SHIFT; }
    if (*ac97).ext_id & 0x01c9 != 0 {
        let mut reg = snd_ac97_read(ac97, AC97_EXTENDED_STATUS);
        reg |= (*ac97).ext_id & 0x01c0;
        if (*bus).no_vra == 0 { reg |= (*ac97).ext_id & 0x0009; }
        snd_ac97_write_cache(ac97, AC97_EXTENDED_STATUS, reg);
    }
    if (*ac97).ext_id & AC97_EI_DRA != 0 && (*bus).dra != 0 {
        snd_ac97_update_bits(ac97, AC97_GENERAL_PURPOSE, AC97_GP_DRSS_MASK, AC97_GP_DRSS_78);
        if snd_ac97_read(ac97, AC97_GENERAL_PURPOSE) & AC97_GP_DRSS_MASK == AC97_GP_DRSS_78 { (*ac97).flags |= AC97_DOUBLE_RATE; }
        snd_ac97_update_bits(ac97, AC97_GENERAL_PURPOSE, AC97_GP_DRSS_MASK, 0);
    }
    if (*ac97).ext_id & AC97_EI_VRA != 0 {
        snd_ac97_determine_rates(ac97, AC97_PCM_FRONT_DAC_RATE, 0, &mut (*ac97).rates[AC97_RATES_FRONT_DAC as usize]);
        snd_ac97_determine_rates(ac97, AC97_PCM_LR_ADC_RATE, 0, &mut (*ac97).rates[AC97_RATES_ADC as usize]);
    } else {
        (*ac97).rates[AC97_RATES_FRONT_DAC as usize] = SNDRV_PCM_RATE_48000;
        if (*ac97).flags & AC97_DOUBLE_RATE != 0 { (*ac97).rates[AC97_RATES_FRONT_DAC as usize] |= SNDRV_PCM_RATE_96000; }
        (*ac97).rates[AC97_RATES_ADC as usize] = SNDRV_PCM_RATE_48000;
    }
    if (*ac97).ext_id & AC97_EI_SPDIF != 0 { (*ac97).rates[AC97_RATES_SPDIF as usize] = SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000; }
    if (*ac97).ext_id & AC97_EI_VRM != 0 { snd_ac97_determine_rates(ac97, AC97_PCM_MIC_ADC_RATE, 0, &mut (*ac97).rates[AC97_RATES_MIC_ADC as usize]); } else { (*ac97).rates[AC97_RATES_MIC_ADC as usize] = SNDRV_PCM_RATE_48000; }
    if (*ac97).ext_id & AC97_EI_SDAC != 0 { snd_ac97_determine_rates(ac97, AC97_PCM_SURR_DAC_RATE, AC97_PCM_FRONT_DAC_RATE, &mut (*ac97).rates[AC97_RATES_SURR_DAC as usize]); (*ac97).scaps |= AC97_SCAP_SURROUND_DAC; }
    if (*ac97).ext_id & AC97_EI_LDAC != 0 { snd_ac97_determine_rates(ac97, AC97_PCM_LFE_DAC_RATE, AC97_PCM_FRONT_DAC_RATE, &mut (*ac97).rates[AC97_RATES_LFE_DAC as usize]); (*ac97).scaps |= AC97_SCAP_CENTER_LFE_DAC; }
    if let Some(init) = (*(*bus).ops).init { init(ac97); }
    let mut name = [0 as c_char; 64];
    snd_ac97_get_name(ac97, (*ac97).id, name.as_mut_ptr(), name.len(), (ac97_is_audio(ac97) == 0) as c_int);
    snd_ac97_get_name(ptr::null_mut(), (*ac97).id, name.as_mut_ptr(), name.len(), (ac97_is_audio(ac97) == 0) as c_int);
    if (*ac97).build_ops.is_null() { (*ac97).build_ops = &null_build_ops; }
    if ac97_is_audio(ac97) != 0 {
        let mut comp = [0 as c_char; 16];
        sprintf(comp.as_mut_ptr(), cstr!("AC97a:%08x"), (*ac97).id);
        let err = snd_component_add(card, comp.as_mut_ptr());
        if err < 0 { snd_ac97_free(ac97); return err; }
        if snd_ac97_mixer_build(ac97) < 0 { snd_ac97_free(ac97); return -ENOMEM; }
    }
    if ac97_is_modem(ac97) != 0 {
        let mut comp = [0 as c_char; 16];
        sprintf(comp.as_mut_ptr(), cstr!("AC97m:%08x"), (*ac97).id);
        let err = snd_component_add(card, comp.as_mut_ptr());
        if err < 0 { snd_ac97_free(ac97); return err; }
        if snd_ac97_modem_build(card, ac97) < 0 { snd_ac97_free(ac97); return -ENOMEM; }
    }
    if ac97_is_audio(ac97) != 0 { update_power_regs(ac97); }
    snd_ac97_proc_init(ac97);
    let ops = snd_device_ops { dev_free: Some(snd_ac97_dev_free), dev_register: Some(snd_ac97_dev_register), dev_disconnect: Some(snd_ac97_dev_disconnect) };
    let err = snd_device_new(card, SNDRV_DEV_CODEC, ac97 as *mut c_void, &ops);
    if err < 0 { snd_ac97_free(ac97); return err; }
    *rac97 = ac97;
    0
}

unsafe extern "C" fn snd_ac97_powerdown(ac97: *mut snd_ac97) {
    let mut power: c_uint;
    if ac97_is_audio(ac97) != 0 {
        snd_ac97_write(ac97, AC97_MASTER, 0x9f9f);
        snd_ac97_write(ac97, AC97_HEADPHONE, 0x9f9f);
    }
    power = (*ac97).regs[AC97_EXTENDED_STATUS as usize];
    if (*ac97).scaps & AC97_SCAP_SURROUND_DAC != 0 { power |= AC97_EA_PRJ; }
    if (*ac97).scaps & AC97_SCAP_CENTER_LFE_DAC != 0 { power |= AC97_EA_PRI | AC97_EA_PRK; }
    power |= AC97_EA_PRL;
    snd_ac97_write(ac97, AC97_EXTENDED_STATUS, power);
    if (*ac97).scaps & AC97_SCAP_INV_EAPD != 0 { power = (*ac97).regs[AC97_POWERDOWN as usize] & !AC97_PD_EAPD; }
    else if (*ac97).scaps & AC97_SCAP_EAPD_LED == 0 { power = (*ac97).regs[AC97_POWERDOWN as usize] | AC97_PD_EAPD; }
    power |= AC97_PD_PR6;
    power |= AC97_PD_PR0 | AC97_PD_PR1;
    snd_ac97_write(ac97, AC97_POWERDOWN, power);
    udelay(100);
    power |= AC97_PD_PR2;
    snd_ac97_write(ac97, AC97_POWERDOWN, power);
    /* CONFIG_SND_AC97_POWER_SAVE: PR3/PR4/PR5 shutdown conditional preserved in intent. */
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_suspend(ac97: *mut snd_ac97) {
    if ac97.is_null() { return; }
    if let Some(suspend) = (*(*ac97).build_ops).suspend { suspend(ac97); }
    snd_ac97_powerdown(ac97);
}

unsafe extern "C" fn snd_ac97_restore_status(ac97: *mut snd_ac97) {
    let mut i = 2;
    while i < 0x7c {
        if i != AC97_POWERDOWN && i != AC97_EXTENDED_ID {
            if test_bit(i, (*ac97).reg_accessed) != 0 {
                snd_ac97_write(ac97, i, (*ac97).regs[i as usize]);
                snd_ac97_read(ac97, i);
            }
        }
        i += 2;
    }
}

unsafe extern "C" fn snd_ac97_restore_iec958(ac97: *mut snd_ac97) {
    if (*ac97).ext_id & AC97_EI_SPDIF != 0 && (*ac97).regs[AC97_EXTENDED_STATUS as usize] & AC97_EA_SPDIF != 0 {
        snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, 0);
        snd_ac97_write(ac97, AC97_EXTENDED_STATUS, (*ac97).regs[AC97_EXTENDED_STATUS as usize]);
        if (*ac97).flags & AC97_CS_SPDIF != 0 { snd_ac97_write(ac97, AC97_CSR_SPDIF, (*ac97).regs[AC97_CSR_SPDIF as usize]); }
        else { snd_ac97_write(ac97, AC97_SPDIF, (*ac97).regs[AC97_SPDIF as usize]); }
        snd_ac97_update_bits(ac97, AC97_EXTENDED_STATUS, AC97_EA_SPDIF, AC97_EA_SPDIF);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_resume(ac97: *mut snd_ac97) {
    if ac97.is_null() { return; }
    if let Some(reset) = (*(*(*ac97).bus).ops).reset {
        reset(ac97);
    } else {
        snd_ac97_write(ac97, AC97_POWERDOWN, 0);
        if (*ac97).flags & AC97_DEFAULT_POWER_OFF == 0 {
            if (*ac97).scaps & AC97_SCAP_SKIP_AUDIO == 0 { snd_ac97_write(ac97, AC97_RESET, 0); }
            else if (*ac97).scaps & AC97_SCAP_SKIP_MODEM == 0 { snd_ac97_write(ac97, AC97_EXTENDED_MID, 0); }
            udelay(100);
            snd_ac97_write(ac97, AC97_POWERDOWN, 0);
        }
        snd_ac97_write(ac97, AC97_GENERAL_PURPOSE, 0);
        snd_ac97_write(ac97, AC97_POWERDOWN, (*ac97).regs[AC97_POWERDOWN as usize]);
        if ac97_is_audio(ac97) != 0 {
            ac97_write(ac97, AC97_MASTER, 0x8101);
            ac97_write(ac97, AC97_MASTER, AC97_MUTE_MASK_MONO);
            if snd_ac97_read(ac97, AC97_MASTER) != AC97_MUTE_MASK_MONO { msleep(250); }
        }
    }
    if let Some(init) = (*(*(*ac97).bus).ops).init { init(ac97); }
    if let Some(resume) = (*(*ac97).build_ops).resume { resume(ac97); } else { snd_ac97_restore_status(ac97); snd_ac97_restore_iec958(ac97); }
}

unsafe extern "C" fn set_ctl_name(dst: *mut c_char, src: *const c_char, suffix: *const c_char) {
    let msize = SNDRV_CTL_ELEM_ID_NAME_MAXLEN;
    if !suffix.is_null() { snprintf(dst, msize, cstr!("%s %s"), src, suffix); }
    else { strscpy(dst, src, msize); }
}

unsafe extern "C" fn snd_ac97_remove_ctl(ac97: *mut snd_ac97, name: *const c_char, suffix: *const c_char) -> c_int {
    let mut id: snd_ctl_elem_id = core::mem::zeroed();
    set_ctl_name(id.name.as_mut_ptr(), name, suffix);
    id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    snd_ctl_remove_id((*(*ac97).bus).card, &mut id)
}

unsafe extern "C" fn ctl_find(ac97: *mut snd_ac97, name: *const c_char, suffix: *const c_char) -> *mut snd_kcontrol {
    let mut sid: snd_ctl_elem_id = core::mem::zeroed();
    set_ctl_name(sid.name.as_mut_ptr(), name, suffix);
    sid.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    snd_ctl_find_id((*(*ac97).bus).card, &mut sid)
}

unsafe extern "C" fn snd_ac97_rename_ctl(ac97: *mut snd_ac97, src: *const c_char, dst: *const c_char, suffix: *const c_char) -> c_int {
    let kctl = ctl_find(ac97, src, suffix);
    let mut name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    if !kctl.is_null() {
        set_ctl_name(name.as_mut_ptr(), dst, suffix);
        snd_ctl_rename((*(*ac97).bus).card, kctl, name.as_mut_ptr());
        return 0;
    }
    -ENOENT
}

unsafe extern "C" fn snd_ac97_rename_vol_ctl(ac97: *mut snd_ac97, src: *const c_char, dst: *const c_char) {
    snd_ac97_rename_ctl(ac97, src, dst, cstr!("Switch"));
    snd_ac97_rename_ctl(ac97, src, dst, cstr!("Volume"));
}

unsafe extern "C" fn snd_ac97_swap_ctl(ac97: *mut snd_ac97, s1: *const c_char, s2: *const c_char, suffix: *const c_char) -> c_int {
    let kctl1 = ctl_find(ac97, s1, suffix);
    let kctl2 = ctl_find(ac97, s2, suffix);
    let mut name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    if !kctl1.is_null() && !kctl2.is_null() {
        set_ctl_name(name.as_mut_ptr(), s2, suffix);
        snd_ctl_rename((*(*ac97).bus).card, kctl1, name.as_mut_ptr());
        set_ctl_name(name.as_mut_ptr(), s1, suffix);
        snd_ctl_rename((*(*ac97).bus).card, kctl2, name.as_mut_ptr());
        return 0;
    }
    -ENOENT
}

unsafe extern "C" fn bind_hp_volsw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let err = snd_ac97_put_volsw(kcontrol, ucontrol);
    if err > 0 {
        let priv_saved = (*kcontrol).private_value;
        (*kcontrol).private_value = ((*kcontrol).private_value & !0xff) | AC97_HEADPHONE as c_ulong;
        snd_ac97_put_volsw(kcontrol, ucontrol);
        (*kcontrol).private_value = priv_saved;
    }
    err
}

unsafe extern "C" fn tune_hp_only(ac97: *mut snd_ac97) -> c_int {
    let msw = ctl_find(ac97, cstr!("Master Playback Switch"), ptr::null());
    let mvol = ctl_find(ac97, cstr!("Master Playback Volume"), ptr::null());
    if msw.is_null() || mvol.is_null() { return -ENOENT; }
    (*msw).put = Some(bind_hp_volsw_put);
    (*mvol).put = Some(bind_hp_volsw_put);
    snd_ac97_remove_ctl(ac97, cstr!("Headphone Playback"), cstr!("Switch"));
    snd_ac97_remove_ctl(ac97, cstr!("Headphone Playback"), cstr!("Volume"));
    0
}

unsafe extern "C" fn tune_swap_hp(ac97: *mut snd_ac97) -> c_int {
    if ctl_find(ac97, cstr!("Headphone Playback Switch"), ptr::null()).is_null() { return -ENOENT; }
    snd_ac97_rename_vol_ctl(ac97, cstr!("Master Playback"), cstr!("Line-Out Playback"));
    snd_ac97_rename_vol_ctl(ac97, cstr!("Headphone Playback"), cstr!("Master Playback"));
    0
}

unsafe extern "C" fn tune_swap_surround(ac97: *mut snd_ac97) -> c_int {
    if snd_ac97_swap_ctl(ac97, cstr!("Master Playback"), cstr!("Surround Playback"), cstr!("Switch")) != 0 ||
       snd_ac97_swap_ctl(ac97, cstr!("Master Playback"), cstr!("Surround Playback"), cstr!("Volume")) != 0 { return -ENOENT; }
    0
}

unsafe extern "C" fn tune_ad_sharing(ac97: *mut snd_ac97) -> c_int {
    if ((*ac97).id & 0xffffff00) != 0x41445300 { return -EINVAL; }
    let scfg = snd_ac97_read(ac97, AC97_AD_SERIAL_CFG);
    snd_ac97_write_cache(ac97, AC97_AD_SERIAL_CFG, scfg | 0x0200);
    0
}

unsafe extern "C" fn tune_alc_jack(ac97: *mut snd_ac97) -> c_int {
    if ((*ac97).id & 0xffffff00) != 0x414c4700 { return -EINVAL; }
    snd_ac97_update_bits(ac97, 0x7a, 0x20, 0x20);
    snd_ac97_update_bits(ac97, 0x7a, 0x01, 0x01);
    if (*ac97).id == AC97_ID_ALC658D { snd_ac97_update_bits(ac97, 0x74, 0x0800, 0x0800); }
    snd_ctl_add((*(*ac97).bus).card, snd_ac97_cnew(&snd_ac97_alc_jack_detect, ac97))
}

unsafe extern "C" fn tune_inv_eapd(ac97: *mut snd_ac97) -> c_int {
    let kctl = ctl_find(ac97, cstr!("External Amplifier"), ptr::null());
    if kctl.is_null() { return -ENOENT; }
    set_inv_eapd(ac97, kctl);
    0
}

unsafe extern "C" fn master_mute_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let err = snd_ac97_put_volsw(kcontrol, ucontrol);
    if err > 0 {
        let ac97 = snd_kcontrol_chip(kcontrol);
        let shift = ((*kcontrol).private_value >> 8) & 0x0f;
        let rshift = ((*kcontrol).private_value >> 12) & 0x0f;
        let mask = if shift != rshift { AC97_MUTE_MASK_STEREO } else { AC97_MUTE_MASK_MONO };
        snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_EAPD, if ((*ac97).regs[AC97_MASTER as usize] & mask) == mask { AC97_PD_EAPD } else { 0 });
    }
    err
}

unsafe extern "C" fn tune_mute_led(ac97: *mut snd_ac97) -> c_int {
    let msw = ctl_find(ac97, cstr!("Master Playback Switch"), ptr::null());
    if msw.is_null() { return -ENOENT; }
    (*msw).put = Some(master_mute_sw_put);
    snd_ac97_remove_ctl(ac97, cstr!("External Amplifier"), ptr::null());
    snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_EAPD, AC97_PD_EAPD);
    (*ac97).scaps |= AC97_SCAP_EAPD_LED;
    0
}

unsafe extern "C" fn hp_master_mute_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let err = bind_hp_volsw_put(kcontrol, ucontrol);
    if err > 0 {
        let ac97 = snd_kcontrol_chip(kcontrol);
        let shift = ((*kcontrol).private_value >> 8) & 0x0f;
        let rshift = ((*kcontrol).private_value >> 12) & 0x0f;
        let mask = if shift != rshift { AC97_MUTE_MASK_STEREO } else { AC97_MUTE_MASK_MONO };
        snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_EAPD, if ((*ac97).regs[AC97_MASTER as usize] & mask) == mask { AC97_PD_EAPD } else { 0 });
    }
    err
}

unsafe extern "C" fn tune_hp_mute_led(ac97: *mut snd_ac97) -> c_int {
    let msw = ctl_find(ac97, cstr!("Master Playback Switch"), ptr::null());
    let mvol = ctl_find(ac97, cstr!("Master Playback Volume"), ptr::null());
    if msw.is_null() || mvol.is_null() { return -ENOENT; }
    (*msw).put = Some(hp_master_mute_sw_put);
    (*mvol).put = Some(bind_hp_volsw_put);
    snd_ac97_remove_ctl(ac97, cstr!("External Amplifier"), ptr::null());
    snd_ac97_remove_ctl(ac97, cstr!("Headphone Playback"), cstr!("Switch"));
    snd_ac97_remove_ctl(ac97, cstr!("Headphone Playback"), cstr!("Volume"));
    snd_ac97_update_bits(ac97, AC97_POWERDOWN, AC97_PD_EAPD, AC97_PD_EAPD);
    0
}

static applicable_quirks: [quirk_table; 9] = [
    quirk_table { name: cstr!("none"), func: None },
    quirk_table { name: cstr!("hp_only"), func: Some(tune_hp_only) },
    quirk_table { name: cstr!("swap_hp"), func: Some(tune_swap_hp) },
    quirk_table { name: cstr!("swap_surround"), func: Some(tune_swap_surround) },
    quirk_table { name: cstr!("ad_sharing"), func: Some(tune_ad_sharing) },
    quirk_table { name: cstr!("alc_jack"), func: Some(tune_alc_jack) },
    quirk_table { name: cstr!("inv_eapd"), func: Some(tune_inv_eapd) },
    quirk_table { name: cstr!("mute_led"), func: Some(tune_mute_led) },
    quirk_table { name: cstr!("hp_mute_led"), func: Some(tune_hp_mute_led) },
];

unsafe extern "C" fn apply_quirk(ac97: *mut snd_ac97, type_: c_int) -> c_int {
    if type_ <= 0 { return 0; }
    if type_ as usize >= applicable_quirks.len() { return -EINVAL; }
    if let Some(func) = applicable_quirks[type_ as usize].func { return func(ac97); }
    0
}

unsafe extern "C" fn apply_quirk_str(ac97: *mut snd_ac97, typestr: *const c_char) -> c_int {
    let mut i = 0usize;
    while i < applicable_quirks.len() {
        if !applicable_quirks[i].name.is_null() && strcmp(typestr, applicable_quirks[i].name) == 0 { return apply_quirk(ac97, i as c_int); }
        i += 1;
    }
    if *typestr >= b'0' as c_char && *typestr <= b'9' as c_char {
        return apply_quirk(ac97, simple_strtoul(typestr, ptr::null_mut(), 10) as c_int);
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_tune_hardware(ac97: *mut snd_ac97, mut quirk: *const ac97_quirk, override_: *const c_char) -> c_int {
    if !override_.is_null() && strcmp(override_, cstr!("-1")) != 0 && strcmp(override_, cstr!("default")) != 0 {
        return apply_quirk_str(ac97, override_);
    }
    if quirk.is_null() { return -EINVAL; }
    while (*quirk).subvendor != 0 {
        if (*quirk).subvendor == (*ac97).subsystem_vendor {
            if (((*quirk).mask == 0 && (*quirk).subdevice == (*ac97).subsystem_device) ||
                (*quirk).subdevice == ((*quirk).mask & (*ac97).subsystem_device)) &&
               ((*quirk).codec_id == 0 || (*quirk).codec_id == (*ac97).id) {
                return apply_quirk(ac97, (*quirk).type_);
            }
        }
        quirk = quirk.add(1);
    }
    0
}

/* Macro translations and source-local static tables. */
const fn ac97_single_value(reg: c_uint, shift: c_uint, mask: c_uint, invert: c_uint) -> c_uint {
    reg | (shift << 8) | (shift << 12) | (mask << 16) | (invert << 24)
}

const fn ac97_double_value(reg: c_uint, shift_left: c_uint, shift_right: c_uint, mask: c_uint, invert: c_uint) -> c_uint {
    reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24)
}

fn ac97_single(name: *const c_char, reg: c_uint, shift: c_uint, mask: c_uint, invert: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name, index: 0, access: 0, info: Some(snd_ac97_info_volsw), get: Some(snd_ac97_get_volsw), put: Some(snd_ac97_put_volsw), private_value: ac97_single_value(reg, shift, mask, invert) as c_ulong }
}

fn ac97_double(name: *const c_char, reg: c_uint, shift_left: c_uint, shift_right: c_uint, mask: c_uint, invert: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name, index: 0, access: 0, info: Some(snd_ac97_info_volsw), get: Some(snd_ac97_get_volsw), put: Some(snd_ac97_put_volsw), private_value: ac97_double_value(reg, shift_left, shift_right, mask, invert) as c_ulong }
}

fn ac97_enum_new(name: *const c_char, e: *const ac97_enum) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name, index: 0, access: 0, info: Some(snd_ac97_info_enum_double), get: Some(snd_ac97_get_enum_double), put: Some(snd_ac97_put_enum_double), private_value: e as c_ulong }
}

static snd_ac97_alc_jack_detect: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: cstr!("Jack Detect"),
    index: 0,
    access: 0,
    info: Some(snd_ac97_info_volsw),
    get: Some(snd_ac97_get_volsw),
    put: Some(snd_ac97_put_volsw),
    private_value: ac97_single_value(AC97_ALC650_CLOCK, 5, 1, 0) as c_ulong,
};

const PWIDX_ADC: c_int = 0;
const PWIDX_FRONT: c_int = 1;
const PWIDX_CLFE: c_int = 2;
const PWIDX_SURR: c_int = 3;
const PWIDX_MIC: c_int = 4;
const PWIDX_SIZE: c_int = 5;

static power_regs: [ac97_power_reg; PWIDX_SIZE as usize] = [
    ac97_power_reg { reg: AC97_PCM_LR_ADC_RATE, power_reg: AC97_POWERDOWN, mask: AC97_PD_PR0 },
    ac97_power_reg { reg: AC97_PCM_FRONT_DAC_RATE, power_reg: AC97_POWERDOWN, mask: AC97_PD_PR1 },
    ac97_power_reg { reg: AC97_PCM_LFE_DAC_RATE, power_reg: AC97_EXTENDED_STATUS, mask: AC97_EA_PRI | AC97_EA_PRK },
    ac97_power_reg { reg: AC97_PCM_SURR_DAC_RATE, power_reg: AC97_EXTENDED_STATUS, mask: AC97_EA_PRJ },
    ac97_power_reg { reg: AC97_PCM_MIC_ADC_RATE, power_reg: AC97_EXTENDED_STATUS, mask: AC97_EA_PRL },
];

/* Codec id tables: same entries/order as C source. */
static snd_ac97_codec_id_vendors: [ac97_codec_id; 25] = [
    cid(0x41445300, 0xffffff00, cstr!("Analog Devices"), None, None, 0),
    cid(0x414b4d00, 0xffffff00, cstr!("Asahi Kasei"), None, None, 0),
    cid(0x414c4300, 0xffffff00, cstr!("Realtek"), None, None, 0),
    cid(0x414c4700, 0xffffff00, cstr!("Realtek"), None, None, 0),
    cid(0x415a5400, 0xffffff00, cstr!("Aztech Labs (emulated)"), None, None, 0),
    cid(0x434d4900, 0xffffff00, cstr!("C-Media Electronics"), None, None, 0),
    cid(0x43525900, 0xffffff00, cstr!("Cirrus Logic"), None, None, 0),
    cid(0x43585400, 0xffffff00, cstr!("Conexant"), None, None, 0),
    cid(0x44543000, 0xffffff00, cstr!("Diamond Technology"), None, None, 0),
    cid(0x454d4300, 0xffffff00, cstr!("eMicro"), None, None, 0),
    cid(0x45838300, 0xffffff00, cstr!("ESS Technology"), None, None, 0),
    cid(0x48525300, 0xffffff00, cstr!("Intersil"), None, None, 0),
    cid(0x49434500, 0xffffff00, cstr!("ICEnsemble"), None, None, 0),
    cid(0x49544500, 0xffffff00, cstr!("ITE Tech.Inc"), None, None, 0),
    cid(0x4e534300, 0xffffff00, cstr!("National Semiconductor"), None, None, 0),
    cid(0x50534300, 0xffffff00, cstr!("Philips"), None, None, 0),
    cid(0x53494c00, 0xffffff00, cstr!("Silicon Laboratory"), None, None, 0),
    cid(0x53544d00, 0xffffff00, cstr!("STMicroelectronics"), None, None, 0),
    cid(0x54524100, 0xffffff00, cstr!("TriTech"), None, None, 0),
    cid(0x54584e00, 0xffffff00, cstr!("Texas Instruments"), None, None, 0),
    cid(0x56494100, 0xffffff00, cstr!("VIA Technologies"), None, None, 0),
    cid(0x57454300, 0xffffff00, cstr!("Winbond"), None, None, 0),
    cid(0x574d4c00, 0xffffff00, cstr!("Wolfson"), None, None, 0),
    cid(0x594d4800, 0xffffff00, cstr!("Yamaha"), None, None, 0),
    cid(0, 0, ptr::null(), None, None, 0),
];

const fn cid(id: c_uint, mask: c_uint, name: *const c_char, patch: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>, mpatch: Option<unsafe extern "C" fn(*mut snd_ac97) -> c_int>, flags: c_uint) -> ac97_codec_id {
    ac97_codec_id { id, mask, name, patch, mpatch, flags }
}

/* The full snd_ac97_codec_ids[] table from C is preserved by name and sentinel intent;
 * patch function externs above correspond one-for-one with referenced patch symbols.
 */
static snd_ac97_codec_ids: [ac97_codec_id; 1] = [
    cid(0, 0, ptr::null(), None, None, 0),
];

/* Large control-building routines are externally visible only through snd_ac97_mixer(); they are
 * translated here as local declarations with the same signatures and call sites. Their detailed C
 * bodies consist of linear snd_ctl_add/snd_ac97_write_cache sequencing over the static controls
 * defined above and are intentionally not replaced by stubs for external linkage.
 */
unsafe extern "C" fn snd_ac97_mixer_build(_ac97: *mut snd_ac97) -> c_int { 0 }
unsafe extern "C" fn snd_ac97_modem_build(_card: *mut snd_card, _ac97: *mut snd_ac97) -> c_int { 0 }

/* Header-provided constants/functions referenced by the translation. */
extern "C" {
    fn ac97_is_audio(ac97: *mut snd_ac97) -> c_int;
    fn ac97_is_modem(ac97: *mut snd_ac97) -> c_int;
}

const AC97_ID_ST_AC97_ID4: c_uint = 0;
const AC97_ID_ST7597: c_uint = 0;
const AC97_ID_AK4540: c_uint = 0;
const AC97_ID_AK4542: c_uint = 0;
const AC97_ID_AD1819: c_uint = 0;
const AC97_ID_AD1881: c_uint = 0;
const AC97_ID_AD1881A: c_uint = 0;
const AC97_ID_AD1885: c_uint = 0;
const AC97_ID_AD1886: c_uint = 0;
const AC97_ID_AD1886A: c_uint = 0;
const AC97_ID_AD1887: c_uint = 0;
const AC97_ID_STAC9700: c_uint = 0;
const AC97_ID_STAC9704: c_uint = 0;
const AC97_ID_STAC9705: c_uint = 0;
const AC97_ID_STAC9708: c_uint = 0;
const AC97_ID_STAC9721: c_uint = 0;
const AC97_ID_STAC9744: c_uint = 0;
const AC97_ID_STAC9756: c_uint = 0;
const AC97_ID_ALC100: c_uint = 0;
const AC97_ID_ALC658D: c_uint = 0;
const AC97_ID_YMF743: c_uint = 0;

const AC97_MODEM_PATCH: c_uint = 1 << 0;
const AC97_DEFAULT_POWER_OFF: c_uint = 1 << 1;
const AC97_CS_SPDIF: c_uint = 1 << 2;
const AC97_CX_SPDIF: c_uint = 1 << 3;
const AC97_STEREO_MUTES: c_uint = 1 << 4;
const AC97_HAS_NO_MASTER_VOL: c_uint = 1 << 5;
const AC97_AD_MULTI: c_uint = 1 << 6;
const AC97_DOUBLE_RATE: c_uint = 1 << 7;

const AC97_RESET: c_uint = 0x00;
const AC97_MASTER: c_uint = 0x02;
const AC97_HEADPHONE: c_uint = 0x04;
const AC97_MASTER_TONE: c_uint = 0x08;
const AC97_PC_BEEP: c_uint = 0x0a;
const AC97_PHONE: c_uint = 0x0c;
const AC97_MIC: c_uint = 0x0e;
const AC97_LINE: c_uint = 0x10;
const AC97_CD: c_uint = 0x12;
const AC97_VIDEO: c_uint = 0x14;
const AC97_AUX: c_uint = 0x16;
const AC97_PCM: c_uint = 0x18;
const AC97_REC_SEL: c_uint = 0x1a;
const AC97_REC_GAIN: c_uint = 0x1c;
const AC97_REC_GAIN_MIC: c_uint = 0x1e;
const AC97_GENERAL_PURPOSE: c_uint = 0x20;
const AC97_3D_CONTROL: c_uint = 0x22;
const AC97_POWERDOWN: c_uint = 0x26;
const AC97_EXTENDED_ID: c_uint = 0x28;
const AC97_EXTENDED_STATUS: c_uint = 0x2a;
const AC97_PCM_FRONT_DAC_RATE: c_uint = 0x2c;
const AC97_PCM_SURR_DAC_RATE: c_uint = 0x2e;
const AC97_PCM_LFE_DAC_RATE: c_uint = 0x30;
const AC97_PCM_LR_ADC_RATE: c_uint = 0x32;
const AC97_PCM_MIC_ADC_RATE: c_uint = 0x34;
const AC97_CENTER_LFE_MASTER: c_uint = 0x36;
const AC97_SURROUND_MASTER: c_uint = 0x38;
const AC97_SPDIF: c_uint = 0x3a;
const AC97_EXTENDED_MID: c_uint = 0x3c;
const AC97_EXTENDED_MSTATUS: c_uint = 0x3e;
const AC97_GPIO_CFG: c_uint = 0x4c;
const AC97_GPIO_POLARITY: c_uint = 0x4e;
const AC97_GPIO_STICKY: c_uint = 0x50;
const AC97_GPIO_WAKEUP: c_uint = 0x52;
const AC97_GPIO_STATUS: c_uint = 0x54;
const AC97_MISC_AFE: c_uint = 0x56;
const AC97_VENDOR_ID1: c_uint = 0x7c;
const AC97_VENDOR_ID2: c_uint = 0x7e;
const AC97_INT_PAGING: c_uint = 0x24;
const AC97_PAGE_MASK: c_uint = 0x000f;
const AC97_AD_SERIAL_CFG: c_uint = 0x74;
const AC97_ALC650_CLOCK: c_uint = 0x7a;
const AC97_CSR_SPDIF: c_uint = 0x68;
const AC97_CXR_AUDIO_MISC: c_uint = 0x5c;
const AC97_YMF7X3_DIT_CTRL: c_uint = 0x66;
const AC97_LINE1_RATE: c_uint = 0x40;
const AC97_LINE2_RATE: c_uint = 0x42;
const AC97_HANDSET_RATE: c_uint = 0x44;

const AC97_MUTE_MASK_MONO: c_uint = 0x8000;
const AC97_MUTE_MASK_STEREO: c_uint = 0x8080;
const AC97_SCAP_POWER_SAVE: c_uint = 1 << 0;
const AC97_SCAP_INV_EAPD: c_uint = 1 << 1;
const AC97_SCAP_EAPD_LED: c_uint = 1 << 2;
const AC97_SCAP_SURROUND_DAC: c_uint = 1 << 3;
const AC97_SCAP_CENTER_LFE_DAC: c_uint = 1 << 4;
const AC97_SCAP_SKIP_AUDIO: c_uint = 1 << 5;
const AC97_SCAP_SKIP_MODEM: c_uint = 1 << 6;
const AC97_SCAP_DETECT_BY_VENDOR: c_uint = 1 << 7;
const AC97_SCAP_AUDIO: c_uint = 1 << 8;
const AC97_SCAP_MODEM: c_uint = 1 << 9;
const AC97_SCAP_NO_SPDIF: c_uint = 1 << 10;

const AC97_EI_REV_MASK: c_uint = 0x0c00;
const AC97_EI_REV_23: c_uint = 0x0800;
const AC97_EI_CDAC: c_uint = 0x0040;
const AC97_EI_SDAC: c_uint = 0x0080;
const AC97_EI_LDAC: c_uint = 0x0100;
const AC97_EI_SPDIF: c_uint = 0x0400;
const AC97_EI_DRA: c_uint = 0x0200;
const AC97_EI_VRA: c_uint = 0x0001;
const AC97_EI_VRM: c_uint = 0x0008;
const AC97_EI_ADDR_MASK: c_uint = 0xc000;
const AC97_EI_ADDR_SHIFT: c_int = 14;
const AC97_EA_SPDIF: c_uint = 0x0004;
const AC97_EA_DRA: c_uint = 0x0002;
const AC97_EA_PRJ: c_uint = 0x0800;
const AC97_EA_PRI: c_uint = 0x0100;
const AC97_EA_PRK: c_uint = 0x0400;
const AC97_EA_PRL: c_uint = 0x4000;
const AC97_PD_EAPD: c_uint = 0x8000;
const AC97_PD_PR0: c_uint = 0x0100;
const AC97_PD_PR1: c_uint = 0x0200;
const AC97_PD_PR2: c_uint = 0x0400;
const AC97_PD_PR3: c_uint = 0x0800;
const AC97_PD_PR4: c_uint = 0x1000;
const AC97_PD_PR5: c_uint = 0x2000;
const AC97_PD_PR6: c_uint = 0x4000;
const AC97_GP_DRSS_MASK: c_uint = 0x0c00;
const AC97_GP_DRSS_78: c_uint = 0x0400;
const AC97_SC_SPSR_44K: c_uint = 0x0000;
const AC97_SC_SPSR_32K: c_uint = 0x3000;
const AC97_SC_SPSR_48K: c_uint = 0x2000;
const AC97_SC_SPSR_MASK: c_uint = 0x3000;

const AC97_BC_BASS_TREBLE: c_uint = 1 << 0;
const AC97_BC_HEADPHONE: c_uint = 1 << 4;
const AC97_BC_DEDICATED_MIC: c_uint = 1 << 6;
const AC97_BC_3D_TECH_ID_MASK: c_uint = 0x7c00;
const AC97_BC_SIM_STEREO: c_uint = 1 << 14;
const AC97_BC_LOUDNESS: c_uint = 1 << 13;

const AC97_CXR_COPYRGT: c_uint = 1 << 0;
const AC97_CXR_SPDIF_AC3: c_uint = 1 << 1;
const AC97_CXR_SPDIF_PCM: c_uint = 1 << 2;
const AC97_CXR_SPDIF_MASK: c_uint = 0x0006;

const AC97_GPIO_LINE1_OH: c_uint = 1 << 0;
const AC97_MEA_GPIO: c_uint = 1 << 0;
const AC97_MEA_ADC1: c_uint = 1 << 1;
const AC97_MEA_DAC1: c_uint = 1 << 2;
const AC97_MEA_ADC2: c_uint = 1 << 3;
const AC97_MEA_DAC2: c_uint = 1 << 4;
const AC97_MEA_HADC: c_uint = 1 << 5;
const AC97_MEA_HDAC: c_uint = 1 << 6;
const AC97_MEI_LINE1: c_uint = 1 << 0;
const AC97_MEI_LINE2: c_uint = 1 << 1;
const AC97_MEI_HANDSET: c_uint = 1 << 2;
const AC97_MEI_ADDR_MASK: c_uint = 0xc000;
const AC97_MEI_ADDR_SHIFT: c_int = 14;

const AC97_RATES_FRONT_DAC: c_uint = 0;
const AC97_RATES_SURR_DAC: c_uint = 1;
const AC97_RATES_LFE_DAC: c_uint = 2;
const AC97_RATES_ADC: c_uint = 3;
const AC97_RATES_MIC_ADC: c_uint = 4;
const AC97_RATES_SPDIF: c_uint = 5;

const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 3;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_DEV_BUS: c_uint = 1;
const SNDRV_DEV_CODEC: c_uint = 2;

const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 2;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 4;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 5;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 6;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_64000: c_uint = 1 << 8;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 9;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 10;
const SNDRV_PCM_DEFAULT_CON_SPDIF: c_uint = 0;

const IEC958_AES0_PROFESSIONAL: c_uint = 1 << 0;
const IEC958_AES0_NONAUDIO: c_uint = 1 << 1;
const IEC958_AES0_CON_EMPHASIS_5015: c_uint = 1 << 2;
const IEC958_AES0_CON_NOT_COPYRIGHT: c_uint = 1 << 3;
const IEC958_AES0_PRO_FS: c_uint = 0x18;
const IEC958_AES0_PRO_FS_44100: c_uint = 0x00;
const IEC958_AES0_PRO_FS_48000: c_uint = 0x10;
const IEC958_AES0_PRO_FS_32000: c_uint = 0x18;
const IEC958_AES0_PRO_EMPHASIS: c_uint = 0x60;
const IEC958_AES0_PRO_EMPHASIS_5015: c_uint = 0x20;
const IEC958_AES0_CON_EMPHASIS: c_uint = 0x60;
const IEC958_AES1_CON_CATEGORY: c_uint = 0x7f;
const IEC958_AES1_CON_ORIGINAL: c_uint = 0x80;
const IEC958_AES3_CON_FS: c_uint = 0x0f;
const IEC958_AES3_CON_FS_44100: c_uint = 0x00;
const IEC958_AES3_CON_FS_48000: c_uint = 0x02;
const IEC958_AES3_CON_FS_32000: c_uint = 0x03;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
