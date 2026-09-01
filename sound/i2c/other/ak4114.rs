// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for control of the AK4114 via I2C and 4-wire serial interface
 *  IEC958 (S/PDIF) receiver by Asahi Kasei
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Dependencies originally supplied by Linux/ALSA headers:
// linux/slab.h, linux/delay.h, linux/module.h, sound/core.h, sound/control.h,
// sound/pcm.h, sound/ak4114.h, sound/asoundef.h, sound/info.h.

const AK4114_ADDR: u8 = 0x00; /* fixed address */

type ak4114_read_t = unsafe extern "C" fn(*mut c_void, u8) -> u8;
type ak4114_write_t = unsafe extern "C" fn(*mut c_void, u8, u8);
type ak4114_change_callback_t = unsafe extern "C" fn(*mut ak4114, u8, u8);

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct spinlock_t {
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
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
    pub number: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_iec958 {
    pub status: [u8; 24],
}

#[repr(C)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub iec958: snd_ctl_elem_value_iec958,
    pub bytes: snd_ctl_elem_value_bytes,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct ak4114 {
    pub lock: spinlock_t,
    pub card: *mut snd_card,
    pub read: Option<ak4114_read_t>,
    pub write: Option<ak4114_write_t>,
    pub private_data: *mut c_void,
    pub work: delayed_work,
    pub wq_processing: atomic_t,
    pub reinit_mutex: mutex,
    pub regmap: [u8; 6],
    pub txcsb: [u8; 5],
    pub rcs0: u8,
    pub rcs1: u8,
    pub errors: [c_long; 4],
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub kctls: [*mut snd_kcontrol; AK4114_CONTROLS as usize],
    pub change_callback: Option<ak4114_change_callback_t>,
    pub check_flags: c_uint,
}

unsafe extern "C" {
    fn kzalloc_obj_ak4114() -> *mut ak4114;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_inc_return(v: *mut atomic_t) -> c_int;
    fn atomic_dec(v: *mut atomic_t);
    fn atomic_dec_and_test(v: *mut atomic_t) -> c_int;
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> c_int;
    fn udelay(usecs: c_ulong);
    fn snd_device_new(card: *mut snd_card, ty: c_int, data: *mut c_void, ops: *const snd_device_ops) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_free_one(kcontrol: *mut snd_kcontrol);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, data: *mut c_void,
                            read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn snd_pcm_stream_lock_irqsave(substream: *mut snd_pcm_substream, flags: *mut c_ulong);
    fn snd_pcm_stream_unlock_irqrestore(substream: *mut snd_pcm_substream, flags: *mut c_ulong);
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int) -> c_int;
}

unsafe extern "C" {
    static HZ: c_ulong;
    static ENOMEM: c_int;
    static EINVAL: c_int;
    static SNDRV_DEV_CODEC: c_int;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_CTL_ELEM_TYPE_IEC958: c_uint;
    static SNDRV_CTL_ELEM_TYPE_BYTES: c_uint;
    static SNDRV_CTL_ELEM_IFACE_PCM: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static SNDRV_PCM_STATE_DRAINING: c_int;

    static AK4114_REG_INT1_MASK: u8;
    static AK4114_REG_TXCSB0: u8;
    static AK4114_REG_TXCSB1: u8;
    static AK4114_REG_TXCSB2: u8;
    static AK4114_REG_TXCSB3: u8;
    static AK4114_REG_TXCSB4: u8;
    static AK4114_REG_PWRDN: u8;
    static AK4114_REG_RCS0: u8;
    static AK4114_REG_RCS1: u8;
    static AK4114_REG_RXCSB0: u8;
    static AK4114_REG_Pc0: u8;
    static AK4114_REG_Pc1: u8;
    static AK4114_REG_Pd0: u8;
    static AK4114_REG_Pd1: u8;
    static AK4114_REG_QSUB_ADDR: u8;
    static AK4114_REG_RXCSB_SIZE: c_uint;
    static AK4114_REG_TXCSB_SIZE: c_uint;
    static AK4114_REG_QSUB_SIZE: c_uint;
    static AK4114_CONTROLS: c_uint;

    static AK4114_QINT: u8;
    static AK4114_CINT: u8;
    static AK4114_RST: u8;
    static AK4114_PWN: u8;
    static AK4114_FS0: u8;
    static AK4114_FS1: u8;
    static AK4114_FS2: u8;
    static AK4114_FS3: u8;
    static AK4114_FS_32000HZ: u8;
    static AK4114_FS_44100HZ: u8;
    static AK4114_FS_48000HZ: u8;
    static AK4114_FS_88200HZ: u8;
    static AK4114_FS_96000HZ: u8;
    static AK4114_FS_176400HZ: u8;
    static AK4114_FS_192000HZ: u8;
    static AK4114_PARITY_ERRORS: c_ulong;
    static AK4114_V_BIT_ERRORS: c_ulong;
    static AK4114_CCRC_ERRORS: c_ulong;
    static AK4114_QCRC_ERRORS: c_ulong;
    static AK4114_PAR: u8;
    static AK4114_V: u8;
    static AK4114_CCRC: u8;
    static AK4114_QCRC: u8;
    static AK4114_PEM: u8;
    static AK4114_AUDION: u8;
    static AK4114_AUTO: u8;
    static AK4114_DTSCD: u8;
    static AK4114_UNLCK: u8;
    static AK4114_CHECK_NO_STAT: c_uint;
    static AK4114_CHECK_NO_RATE: c_uint;
}

const IEC958_PARITY_ERRORS: &[u8] = b"IEC958 Parity Errors\0";
const IEC958_V_BIT_ERRORS: &[u8] = b"IEC958 V-Bit Errors\0";
const IEC958_C_CRC_ERRORS: &[u8] = b"IEC958 C-CRC Errors\0";
const IEC958_Q_CRC_ERRORS: &[u8] = b"IEC958 Q-CRC Errors\0";
const IEC958_EXTERNAL_RATE: &[u8] = b"IEC958 External Rate\0";
const IEC958_PLAYBACK_MASK: &[u8] = b"IEC958 Playback Mask\0";
const IEC958_PLAYBACK_DEFAULT: &[u8] = b"IEC958 Playback Default\0";
const IEC958_CAPTURE_MASK: &[u8] = b"IEC958 Capture Mask\0";
const IEC958_CAPTURE_DEFAULT: &[u8] = b"IEC958 Capture Default\0";
const IEC958_PREAMBLE_CAPTURE_DEFAULT: &[u8] = b"IEC958 Preamble Capture Default\0";
const IEC958_Q_SUBCODE_CAPTURE_DEFAULT: &[u8] = b"IEC958 Q-subcode Capture Default\0";
const IEC958_AUDIO: &[u8] = b"IEC958 Audio\0";
const IEC958_NON_PCM_BITSTREAM: &[u8] = b"IEC958 Non-PCM Bitstream\0";
const IEC958_DTS_BITSTREAM: &[u8] = b"IEC958 DTS Bitstream\0";
const IEC958_PPL_LOCK_STATUS: &[u8] = b"IEC958 PPL Lock Status\0";
const AK4114_PROC_NAME: &[u8] = b"ak4114\0";
const REG_PRINT_FMT: &[u8] = b"0x%02x = 0x%02x\n\0";
const PLAYBACK_STR: &[u8] = b"Playback\0";

unsafe extern "C" fn ak4114_stats(work: *mut work_struct) {
    let chip = (work as *mut u8).sub(core::mem::offset_of!(ak4114, work) + core::mem::offset_of!(delayed_work, work)) as *mut ak4114;

    if atomic_inc_return(&mut (*chip).wq_processing) == 1 {
        snd_ak4114_check_rate_and_errors(chip, (*chip).check_flags);
    }
    if atomic_dec_and_test(&mut (*chip).wq_processing) != 0 {
        schedule_delayed_work(&mut (*chip).work, HZ / 10);
    }
}

unsafe fn reg_write(ak4114: *mut ak4114, reg: u8, val: u8) {
    ((*ak4114).write.unwrap())((*ak4114).private_data, reg, val);
    if reg <= AK4114_REG_INT1_MASK {
        (*ak4114).regmap[reg as usize] = val;
    } else if reg >= AK4114_REG_TXCSB0 && reg <= AK4114_REG_TXCSB4 {
        (*ak4114).txcsb[(reg - AK4114_REG_TXCSB0) as usize] = val;
    }
}

unsafe fn reg_read(ak4114: *mut ak4114, reg: u8) -> u8 {
    ((*ak4114).read.unwrap())((*ak4114).private_data, reg)
}

unsafe fn snd_ak4114_free(chip: *mut ak4114) {
    atomic_inc(&mut (*chip).wq_processing); /* don't schedule new work */
    cancel_delayed_work_sync(&mut (*chip).work);
    kfree(chip as *mut c_void);
}

unsafe extern "C" fn snd_ak4114_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut ak4114;
    snd_ak4114_free(chip);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_create(
    card: *mut snd_card,
    read: Option<ak4114_read_t>,
    write: Option<ak4114_write_t>,
    pgm: *const u8,
    txcsb: *const u8,
    private_data: *mut c_void,
    r_ak4114: *mut *mut ak4114,
) -> c_int {
    let mut err: c_int = 0;
    let mut reg: u8;
    let ops = snd_device_ops {
        dev_free: Some(snd_ak4114_dev_free),
    };

    let chip = kzalloc_obj_ak4114();
    if chip.is_null() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*chip).lock);
    (*chip).card = card;
    (*chip).read = read;
    (*chip).write = write;
    (*chip).private_data = private_data;
    INIT_DELAYED_WORK(&mut (*chip).work, ak4114_stats);
    atomic_set(&mut (*chip).wq_processing, 0);
    mutex_init(&mut (*chip).reinit_mutex);

    reg = 0;
    while reg < 6 {
        (*chip).regmap[reg as usize] = *pgm.add(reg as usize);
        reg = reg.wrapping_add(1);
    }
    reg = 0;
    while reg < 5 {
        (*chip).txcsb[reg as usize] = *txcsb.add(reg as usize);
        reg = reg.wrapping_add(1);
    }

    ak4114_init_regs(chip);

    (*chip).rcs0 = reg_read(chip, AK4114_REG_RCS0) & !(AK4114_QINT | AK4114_CINT);
    (*chip).rcs1 = reg_read(chip, AK4114_REG_RCS1);

    err = snd_device_new(card, SNDRV_DEV_CODEC, chip as *mut c_void, &ops);
    if err < 0 {
        snd_ak4114_free(chip);
        return err;
    }

    if !r_ak4114.is_null() {
        *r_ak4114 = chip;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_reg_write(chip: *mut ak4114, reg: u8, mask: u8, val: u8) {
    if reg <= AK4114_REG_INT1_MASK {
        reg_write(chip, reg, ((*chip).regmap[reg as usize] & !mask) | val);
    } else if reg >= AK4114_REG_TXCSB0 && reg <= AK4114_REG_TXCSB4 {
        reg_write(chip, reg, ((*chip).txcsb[(reg - AK4114_REG_TXCSB0) as usize] & !mask) | val);
    }
}

unsafe fn ak4114_init_regs(chip: *mut ak4114) {
    let old = (*chip).regmap[AK4114_REG_PWRDN as usize];
    let mut reg: u8;

    /* bring the chip to reset state and powerdown state */
    reg_write(chip, AK4114_REG_PWRDN, old & !(AK4114_RST | AK4114_PWN));
    udelay(200);
    /* release reset, but leave powerdown */
    reg_write(chip, AK4114_REG_PWRDN, (old | AK4114_RST) & !AK4114_PWN);
    udelay(200);
    reg = 1;
    while reg < 6 {
        reg_write(chip, reg, (*chip).regmap[reg as usize]);
        reg = reg.wrapping_add(1);
    }
    reg = 0;
    while reg < 5 {
        reg_write(chip, reg.wrapping_add(AK4114_REG_TXCSB0), (*chip).txcsb[reg as usize]);
        reg = reg.wrapping_add(1);
    }
    /* release powerdown, everything is initialized now */
    reg_write(chip, AK4114_REG_PWRDN, old | AK4114_RST | AK4114_PWN);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_reinit(chip: *mut ak4114) {
    if atomic_inc_return(&mut (*chip).wq_processing) == 1 {
        cancel_delayed_work_sync(&mut (*chip).work);
    }
    mutex_lock(&mut (*chip).reinit_mutex);
    ak4114_init_regs(chip);
    mutex_unlock(&mut (*chip).reinit_mutex);
    /* bring up statistics / event queing */
    if atomic_dec_and_test(&mut (*chip).wq_processing) != 0 {
        schedule_delayed_work(&mut (*chip).work, HZ / 10);
    }
}

unsafe fn external_rate(rcs1: u8) -> c_uint {
    let rate = rcs1 & (AK4114_FS0 | AK4114_FS1 | AK4114_FS2 | AK4114_FS3);
    if rate == AK4114_FS_32000HZ {
        32000
    } else if rate == AK4114_FS_44100HZ {
        44100
    } else if rate == AK4114_FS_48000HZ {
        48000
    } else if rate == AK4114_FS_88200HZ {
        88200
    } else if rate == AK4114_FS_96000HZ {
        96000
    } else if rate == AK4114_FS_176400HZ {
        176400
    } else if rate == AK4114_FS_192000HZ {
        192000
    } else {
        0
    }
}

unsafe extern "C" fn snd_ak4114_in_error_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = c_long::MAX;
    0
}

unsafe extern "C" fn snd_ak4114_in_error_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;

    (*ucontrol).value.integer.value[0] = (*chip).errors[(*kcontrol).private_value as usize];
    (*chip).errors[(*kcontrol).private_value as usize] = 0;
    0
}

const snd_ak4114_in_bit_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn snd_ak4114_in_bit_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let bit = (((*kcontrol).private_value >> 8) & 0xff) as u8;
    let inv = (((*kcontrol).private_value >> 31) & 1) as c_long;

    (*ucontrol).value.integer.value[0] = (if (reg_read(chip, reg) & (1u8 << bit)) != 0 { 1 } else { 0 }) ^ inv;
    0
}

unsafe extern "C" fn snd_ak4114_rate_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 192000;
    0
}

unsafe extern "C" fn snd_ak4114_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;

    (*ucontrol).value.integer.value[0] = external_rate(reg_read(chip, AK4114_REG_RCS1)) as c_long;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let mut i: c_uint = 0;

    while i < AK4114_REG_RXCSB_SIZE {
        (*ucontrol).value.iec958.status[i as usize] = reg_read(chip, AK4114_REG_RXCSB0.wrapping_add(i as u8));
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_ak4114_spdif_playback_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let mut i: c_uint = 0;

    while i < AK4114_REG_TXCSB_SIZE {
        (*ucontrol).value.iec958.status[i as usize] = (*chip).txcsb[i as usize];
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_ak4114_spdif_playback_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let mut i: c_uint = 0;

    while i < AK4114_REG_TXCSB_SIZE {
        reg_write(chip, AK4114_REG_TXCSB0.wrapping_add(i as u8), (*ucontrol).value.iec958.status[i as usize]);
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_ak4114_spdif_mask_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_mask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    memset((*ucontrol).value.iec958.status.as_mut_ptr() as *mut c_void, 0xff, AK4114_REG_RXCSB_SIZE as usize);
    0
}

unsafe extern "C" fn snd_ak4114_spdif_pinfo(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0xffff;
    (*uinfo).count = 4;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_pget(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let mut tmp: u16;

    (*ucontrol).value.integer.value[0] = 0xf8f2;
    (*ucontrol).value.integer.value[1] = 0x4e1f;
    tmp = (reg_read(chip, AK4114_REG_Pc0) as u16) | ((reg_read(chip, AK4114_REG_Pc1) as u16) << 8);
    (*ucontrol).value.integer.value[2] = tmp as c_long;
    tmp = (reg_read(chip, AK4114_REG_Pd0) as u16) | ((reg_read(chip, AK4114_REG_Pd1) as u16) << 8);
    (*ucontrol).value.integer.value[3] = tmp as c_long;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_qinfo(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = AK4114_REG_QSUB_SIZE;
    0
}

unsafe extern "C" fn snd_ak4114_spdif_qget(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol) as *mut ak4114;
    let mut i: c_uint = 0;

    while i < AK4114_REG_QSUB_SIZE {
        (*ucontrol).value.bytes.data[i as usize] = reg_read(chip, AK4114_REG_QSUB_ADDR.wrapping_add(i as u8));
        i += 1;
    }
    0
}

/* Don't forget to change AK4114_CONTROLS define!!! */
static snd_ak4114_iec958_controls: [snd_kcontrol_new; 15] = [
    snd_kcontrol_new { iface: 0, name: IEC958_PARITY_ERRORS.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_in_error_info), get: Some(snd_ak4114_in_error_get), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_V_BIT_ERRORS.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_in_error_info), get: Some(snd_ak4114_in_error_get), put: None, private_value: 1 },
    snd_kcontrol_new { iface: 0, name: IEC958_C_CRC_ERRORS.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_in_error_info), get: Some(snd_ak4114_in_error_get), put: None, private_value: 2 },
    snd_kcontrol_new { iface: 0, name: IEC958_Q_CRC_ERRORS.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_in_error_info), get: Some(snd_ak4114_in_error_get), put: None, private_value: 3 },
    snd_kcontrol_new { iface: 0, name: IEC958_EXTERNAL_RATE.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_rate_info), get: Some(snd_ak4114_rate_get), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_PLAYBACK_MASK.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_mask_info), get: Some(snd_ak4114_spdif_mask_get), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_PLAYBACK_DEFAULT.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_info), get: Some(snd_ak4114_spdif_playback_get), put: Some(snd_ak4114_spdif_playback_put), private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_CAPTURE_MASK.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_mask_info), get: Some(snd_ak4114_spdif_mask_get), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_CAPTURE_DEFAULT.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_info), get: Some(snd_ak4114_spdif_get), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_PREAMBLE_CAPTURE_DEFAULT.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_pinfo), get: Some(snd_ak4114_spdif_pget), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_Q_SUBCODE_CAPTURE_DEFAULT.as_ptr() as *const c_char, access: 0, info: Some(snd_ak4114_spdif_qinfo), get: Some(snd_ak4114_spdif_qget), put: None, private_value: 0 },
    snd_kcontrol_new { iface: 0, name: IEC958_AUDIO.as_ptr() as *const c_char, access: 0, info: snd_ak4114_in_bit_info, get: Some(snd_ak4114_in_bit_get), put: None, private_value: ((1u32 << 31) | (1 << 8)) as c_ulong },
    snd_kcontrol_new { iface: 0, name: IEC958_NON_PCM_BITSTREAM.as_ptr() as *const c_char, access: 0, info: snd_ak4114_in_bit_info, get: Some(snd_ak4114_in_bit_get), put: None, private_value: (6 << 8) as c_ulong },
    snd_kcontrol_new { iface: 0, name: IEC958_DTS_BITSTREAM.as_ptr() as *const c_char, access: 0, info: snd_ak4114_in_bit_info, get: Some(snd_ak4114_in_bit_get), put: None, private_value: (3 << 8) as c_ulong },
    snd_kcontrol_new { iface: 0, name: IEC958_PPL_LOCK_STATUS.as_ptr() as *const c_char, access: 0, info: snd_ak4114_in_bit_info, get: Some(snd_ak4114_in_bit_get), put: None, private_value: ((1u32 << 31) | (4 << 8)) as c_ulong },
];

unsafe extern "C" fn snd_ak4114_proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ak4114 = (*entry).private_data as *mut ak4114;
    let mut reg: c_int = 0;
    let mut val: c_int;
    /* all ak4114 registers 0x00 - 0x1f */
    while reg < 0x20 {
        val = reg_read(ak4114, reg as u8) as c_int;
        snd_iprintf(buffer, REG_PRINT_FMT.as_ptr() as *const c_char, reg, val);
        reg += 1;
    }
}

unsafe fn snd_ak4114_proc_init(ak4114: *mut ak4114) {
    snd_card_ro_proc_new((*ak4114).card, AK4114_PROC_NAME.as_ptr() as *const c_char, ak4114 as *mut c_void,
                         snd_ak4114_proc_regs_read);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_build(
    ak4114: *mut ak4114,
    ply_substream: *mut snd_pcm_substream,
    cap_substream: *mut snd_pcm_substream,
) -> c_int {
    let mut kctl: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    if snd_BUG_ON(cap_substream.is_null()) != 0 {
        return -EINVAL;
    }
    (*ak4114).playback_substream = ply_substream;
    (*ak4114).capture_substream = cap_substream;
    idx = 0;
    while idx < AK4114_CONTROLS {
        kctl = snd_ctl_new1(&snd_ak4114_iec958_controls[idx as usize], ak4114 as *mut c_void);
        if kctl.is_null() {
            return -ENOMEM;
        }
        if !strstr((*kctl).id.name.as_ptr(), PLAYBACK_STR.as_ptr() as *const c_char).is_null() {
            if ply_substream.is_null() {
                snd_ctl_free_one(kctl);
                (*ak4114).kctls[idx as usize] = core::ptr::null_mut();
                idx += 1;
                continue;
            }
            (*kctl).id.device = (*(*ply_substream).pcm).device;
            (*kctl).id.subdevice = (*ply_substream).number;
        } else {
            (*kctl).id.device = (*(*cap_substream).pcm).device;
            (*kctl).id.subdevice = (*cap_substream).number;
        }
        err = snd_ctl_add((*ak4114).card, kctl);
        if err < 0 {
            return err;
        }
        (*ak4114).kctls[idx as usize] = kctl;
        idx += 1;
    }
    snd_ak4114_proc_init(ak4114);
    /* trigger workq */
    schedule_delayed_work(&mut (*ak4114).work, HZ / 10);
    0
}

/* notify kcontrols if any parameters are changed */
unsafe fn ak4114_notify(ak4114: *mut ak4114, rcs0: u8, rcs1: u8, c0: u8, c1: u8) {
    if (*ak4114).kctls[0].is_null() {
        return;
    }

    if (rcs0 & AK4114_PAR) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[0]).id); }
    if (rcs0 & AK4114_V) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[1]).id); }
    if (rcs1 & AK4114_CCRC) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[2]).id); }
    if (rcs1 & AK4114_QCRC) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[3]).id); }

    /* rate change */
    if (c1 & 0xf0) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[4]).id); }

    if ((c0 & AK4114_PEM) | (c0 & AK4114_CINT)) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[9]).id); }
    if (c0 & AK4114_QINT) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[10]).id); }
    if (c0 & AK4114_AUDION) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[11]).id); }
    if (c0 & AK4114_AUTO) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[12]).id); }
    if (c0 & AK4114_DTSCD) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[13]).id); }
    if (c0 & AK4114_UNLCK) != 0 { snd_ctl_notify((*ak4114).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4114).kctls[14]).id); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_external_rate(ak4114: *mut ak4114) -> c_int {
    let rcs1: u8 = reg_read(ak4114, AK4114_REG_RCS1);
    external_rate(rcs1) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_check_rate_and_errors(ak4114: *mut ak4114, flags: c_uint) -> c_int {
    let runtime: *mut snd_pcm_runtime = if !(*ak4114).capture_substream.is_null() {
        (*(*ak4114).capture_substream).runtime
    } else {
        core::ptr::null_mut()
    };
    let mut _flags: c_ulong = 0;
    let mut res: c_int = 0;
    let mut rcs0: u8 = 0;
    let rcs1: u8;
    let mut c0: u8 = 0;
    let mut c1: u8 = 0;

    rcs1 = reg_read(ak4114, AK4114_REG_RCS1);
    if (flags & AK4114_CHECK_NO_STAT) == 0 {
        rcs0 = reg_read(ak4114, AK4114_REG_RCS0);
        if (rcs0 & AK4114_PAR) != 0 { (*ak4114).errors[AK4114_PARITY_ERRORS as usize] += 1; }
        if (rcs1 & AK4114_V) != 0 { (*ak4114).errors[AK4114_V_BIT_ERRORS as usize] += 1; }
        if (rcs1 & AK4114_CCRC) != 0 { (*ak4114).errors[AK4114_CCRC_ERRORS as usize] += 1; }
        if (rcs1 & AK4114_QCRC) != 0 { (*ak4114).errors[AK4114_QCRC_ERRORS as usize] += 1; }
        c0 = ((*ak4114).rcs0 & (AK4114_QINT | AK4114_CINT | AK4114_PEM | AK4114_AUDION | AK4114_AUTO | AK4114_UNLCK)) ^
            (rcs0 & (AK4114_QINT | AK4114_CINT | AK4114_PEM | AK4114_AUDION | AK4114_AUTO | AK4114_UNLCK));
        c1 = ((*ak4114).rcs1 & 0xf0) ^ (rcs1 & 0xf0);
        (*ak4114).rcs0 = rcs0 & !(AK4114_QINT | AK4114_CINT);
        (*ak4114).rcs1 = rcs1;

        ak4114_notify(ak4114, rcs0, rcs1, c0, c1);
        if let Some(change_callback) = (*ak4114).change_callback {
            if (c0 | c1) != 0 {
                change_callback(ak4114, c0, c1);
            }
        }
    }

    /* compare rate */
    res = external_rate(rcs1) as c_int;
    if (flags & AK4114_CHECK_NO_RATE) == 0 && !runtime.is_null() && (*runtime).rate as c_int != res {
        snd_pcm_stream_lock_irqsave((*ak4114).capture_substream, &mut _flags);
        if snd_pcm_running((*ak4114).capture_substream) != 0 {
            snd_pcm_stop((*ak4114).capture_substream, SNDRV_PCM_STATE_DRAINING);
            res = 1;
        }
        snd_pcm_stream_unlock_irqrestore((*ak4114).capture_substream, &mut _flags);
    }
    res
}

// CONFIG_PM conditional code from the C source.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_suspend(chip: *mut ak4114) {
    atomic_inc(&mut (*chip).wq_processing); /* don't schedule new work */
    cancel_delayed_work_sync(&mut (*chip).work);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_ak4114_resume(chip: *mut ak4114) {
    atomic_dec(&mut (*chip).wq_processing);
    snd_ak4114_reinit(chip);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
