// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * Common mixer part
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;

const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

extern "C" {
    static VX_STAT_IS_STALE: c_uint;
    static VX_TYPE_VXPOCKET: c_int;
    static XX_CODEC_SELECTOR: c_uint;
    static XX_CODEC_LEVEL_LEFT_REGISTER: c_int;
    static XX_CODEC_LEVEL_RIGHT_REGISTER: c_int;
    static XX_CODEC_DAC_CONTROL_REGISTER: c_int;
    static XX_CODEC_ADC_CONTROL_REGISTER: c_int;
    static XX_CODEC_PORT_MODE_REGISTER: c_int;
    static XX_CODEC_CLOCK_CONTROL_REGISTER: c_int;
    static CMD_AUDIO_LEVEL_ADJUST: c_uint;
    static CMD_GET_AUDIO_LEVELS: c_uint;
    static CMD_AUDIO_VU_PIC_METER: c_uint;
    static COMMAND_RECORD_MASK: c_uint;
    static VALID_AUDIO_IO_DIGITAL_LEVEL: c_uint;
    static VALID_AUDIO_IO_MONITORING_LEVEL: c_uint;
    static VALID_AUDIO_IO_MUTE_LEVEL: c_uint;
    static VALID_AUDIO_IO_MUTE_MONITORING_1: c_uint;
    static VALID_AUDIO_IO_MUTE_MONITORING_2: c_uint;
    static AUDIO_IO_HAS_MUTE_LEVEL: c_uint;
    static AUDIO_IO_HAS_MUTE_MONITORING_1: c_uint;
    static MASK_DSP_WORD_LEVEL: c_uint;
    static CVAL_0DB: c_int;
    static CVAL_MAX: c_uint;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_int;
    static SNDRV_CTL_ELEM_TYPE_IEC958: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static SNDRV_CTL_ELEM_IFACE_PCM: c_int;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;

    fn snd_BUG_ON(cond: bool) -> bool;
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn vx_init_rmh(rmh: *mut vx_rmh, cmd: c_uint);
    fn vx_send_msg(chip: *mut vx_core, rmh: *mut vx_rmh) -> c_int;
    fn vx_set_clock(chip: *mut vx_core, freq: c_uint);
    fn vx_set_iec958_status(chip: *mut vx_core, bits: c_uint);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut vx_core;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_boolean_stereo_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct vx_core {
    pub ops: *mut vx_ops,
    pub hw: *mut vx_hw,
    pub chip_status: c_uint,
    pub lock: c_void,
    pub mixer_mutex: c_void,
    pub type_: c_int,
    pub audio_source_target: c_uint,
    pub audio_source: c_uint,
    pub pcm_running: c_int,
    pub output_level: [[c_uint; 2]; 8],
    pub audio_monitor: [c_int; 64],
    pub audio_monitor_active: [c_int; 64],
    pub audio_active: [c_int; 64],
    pub audio_gain: [[c_int; 64]; 2],
    pub clock_mode: c_uint,
    pub freq: c_uint,
    pub uer_bits: c_uint,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct vx_ops {
    pub write_codec: Option<unsafe extern "C" fn(*mut vx_core, c_int, c_uint)>,
    pub akm_write: Option<unsafe extern "C" fn(*mut vx_core, c_int, c_int)>,
    pub reset_codec: Option<unsafe extern "C" fn(*mut vx_core)>,
    pub change_audio_source: Option<unsafe extern "C" fn(*mut vx_core, c_int)>,
}

#[repr(C)]
pub struct vx_hw {
    pub output_level_max: c_uint,
    pub num_codecs: c_uint,
    pub num_ins: c_uint,
    pub num_outs: c_uint,
    pub output_level_db_scale: *const c_uint,
}

#[repr(C)]
pub struct vx_rmh {
    pub Cmd: [c_uint; 8],
    pub Stat: [c_uint; 16],
    pub LgStat: c_uint,
}

#[repr(C)]
pub struct snd_card {
    pub mixername: [c_char; 80],
    pub driver: [c_char; 16],
}

#[repr(C)]
pub struct snd_kcontrol_id {
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
    pub private_value: isize,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_iec958 {
    pub status: [u8; 24],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub iec958: snd_ctl_elem_value_iec958,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub count: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
    pub private_value: isize,
}

struct MutexGuard {
    mutex: *mut c_void,
}

impl MutexGuard {
    unsafe fn new(mutex: *mut c_void) -> Self {
        mutex_lock(mutex);
        Self { mutex }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.mutex) };
    }
}

#[repr(C)]
union vx_codec_data {
    l: u32,
    b: vx_codec_data_b,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct vx_codec_data_b {
    #[cfg(target_endian = "big")]
    hh: u8,
    #[cfg(target_endian = "big")]
    mh: u8,
    #[cfg(target_endian = "big")]
    ml: u8,
    #[cfg(target_endian = "big")]
    ll: u8,
    #[cfg(target_endian = "little")]
    ll: u8,
    #[cfg(target_endian = "little")]
    ml: u8,
    #[cfg(target_endian = "little")]
    mh: u8,
    #[cfg(target_endian = "little")]
    hh: u8,
}

unsafe fn SET_CDC_DATA_SEL(di: *mut vx_codec_data, s: c_uint) {
    (*di).b.mh = s as u8;
}

unsafe fn SET_CDC_DATA_REG(di: *mut vx_codec_data, r: c_int) {
    (*di).b.ml = r as u8;
}

unsafe fn SET_CDC_DATA_VAL(di: *mut vx_codec_data, d: c_int) {
    (*di).b.ll = d as u8;
}

unsafe fn SET_CDC_DATA_INIT(di: *mut vx_codec_data) {
    (*di).l = 0;
    SET_CDC_DATA_SEL(di, XX_CODEC_SELECTOR);
}

unsafe fn vx_write_codec_reg(chip: *mut vx_core, codec: c_int, data: c_uint) {
    if snd_BUG_ON((*(*chip).ops).write_codec.is_none()) {
        return;
    }
    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return;
    }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).lock).cast());
    ((*(*chip).ops).write_codec.unwrap())(chip, codec, data);
}

unsafe fn vx_set_codec_reg(chip: *mut vx_core, codec: c_int, reg: c_int, val: c_int) {
    let mut data = vx_codec_data { l: 0 };
    SET_CDC_DATA_INIT(&mut data);
    SET_CDC_DATA_REG(&mut data, reg);
    SET_CDC_DATA_VAL(&mut data, val);
    vx_write_codec_reg(chip, codec, data.l);
}

unsafe fn vx_set_analog_output_level(chip: *mut vx_core, codec: c_int, mut left: c_int, mut right: c_int) {
    left = (*(*chip).hw).output_level_max as c_int - left;
    right = (*(*chip).hw).output_level_max as c_int - right;
    if let Some(akm_write) = (*(*chip).ops).akm_write {
        akm_write(chip, XX_CODEC_LEVEL_LEFT_REGISTER, left);
        akm_write(chip, XX_CODEC_LEVEL_RIGHT_REGISTER, right);
    } else {
        /* convert to attenuation level: 0 = 0dB (max), 0xe3 = -113.5 dB (min) */
        vx_set_codec_reg(chip, codec, XX_CODEC_LEVEL_LEFT_REGISTER, left);
        vx_set_codec_reg(chip, codec, XX_CODEC_LEVEL_RIGHT_REGISTER, right);
    }
}

const DAC_ATTEN_MIN: c_int = 0x08;
const DAC_ATTEN_MAX: c_int = 0x38;

#[no_mangle]
pub unsafe extern "C" fn vx_toggle_dac_mute(chip: *mut vx_core, mute: c_int) {
    let mut i: c_uint = 0;
    while i < (*(*chip).hw).num_codecs {
        if let Some(akm_write) = (*(*chip).ops).akm_write {
            akm_write(chip, XX_CODEC_DAC_CONTROL_REGISTER, mute); /* XXX */
        } else {
            vx_set_codec_reg(
                chip,
                i as c_int,
                XX_CODEC_DAC_CONTROL_REGISTER,
                if mute != 0 { DAC_ATTEN_MAX } else { DAC_ATTEN_MIN },
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vx_reset_codec(chip: *mut vx_core, _cold_reset: c_int) {
    let mut i: c_uint;
    let port: c_int = if (*chip).type_ >= VX_TYPE_VXPOCKET { 0x75 } else { 0x65 };
    ((*(*chip).ops).reset_codec.unwrap())(chip);
    if (*(*chip).ops).akm_write.is_none() {
        i = 0;
        while i < (*(*chip).hw).num_codecs {
            vx_set_codec_reg(chip, i as c_int, XX_CODEC_DAC_CONTROL_REGISTER, DAC_ATTEN_MAX);
            vx_set_codec_reg(chip, i as c_int, XX_CODEC_ADC_CONTROL_REGISTER, 0x00);
            vx_set_codec_reg(chip, i as c_int, XX_CODEC_PORT_MODE_REGISTER, port);
            vx_set_codec_reg(chip, i as c_int, XX_CODEC_CLOCK_CONTROL_REGISTER, 0x00);
            i += 1;
        }
    }
    i = 0;
    while i < (*(*chip).hw).num_codecs {
        (*chip).output_level[i as usize][0] = 0;
        (*chip).output_level[i as usize][1] = 0;
        vx_set_analog_output_level(chip, i as c_int, 0, 0);
        i += 1;
    }
}

unsafe fn vx_change_audio_source(chip: *mut vx_core, src: c_int) {
    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return;
    }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).lock).cast());
    ((*(*chip).ops).change_audio_source.unwrap())(chip, src);
}

#[no_mangle]
pub unsafe extern "C" fn vx_sync_audio_source(chip: *mut vx_core) -> c_int {
    if (*chip).audio_source_target == (*chip).audio_source || (*chip).pcm_running != 0 {
        return 0;
    }
    vx_change_audio_source(chip, (*chip).audio_source_target as c_int);
    (*chip).audio_source = (*chip).audio_source_target;
    1
}

#[repr(C)]
#[derive(Copy, Clone)]
struct vx_audio_level {
    has_level: c_uint,
    has_monitor_level: c_uint,
    has_mute: c_uint,
    has_monitor_mute: c_uint,
    mute: c_uint,
    monitor_mute: c_uint,
    level: i16,
    monitor_level: i16,
}

unsafe fn vx_adjust_audio_level(
    chip: *mut vx_core,
    audio: c_int,
    capture: c_int,
    info: *mut vx_audio_level,
) -> c_int {
    let mut rmh = MaybeUninit::<vx_rmh>::zeroed().assume_init();
    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }
    vx_init_rmh(&mut rmh, CMD_AUDIO_LEVEL_ADJUST);
    if capture != 0 {
        rmh.Cmd[0] |= COMMAND_RECORD_MASK;
    }
    rmh.Cmd[1] = 1u32 << audio;
    rmh.Cmd[2] = 0;
    if (*info).has_level != 0 {
        rmh.Cmd[0] |= VALID_AUDIO_IO_DIGITAL_LEVEL;
        rmh.Cmd[2] |= (*info).level as c_uint;
    }
    if (*info).has_monitor_level != 0 {
        rmh.Cmd[0] |= VALID_AUDIO_IO_MONITORING_LEVEL;
        rmh.Cmd[2] |= ((*info).monitor_level as c_uint) << 10;
    }
    if (*info).has_mute != 0 {
        rmh.Cmd[0] |= VALID_AUDIO_IO_MUTE_LEVEL;
        if (*info).mute != 0 {
            rmh.Cmd[2] |= AUDIO_IO_HAS_MUTE_LEVEL;
        }
    }
    if (*info).has_monitor_mute != 0 {
        rmh.Cmd[0] |= VALID_AUDIO_IO_MUTE_MONITORING_1 | VALID_AUDIO_IO_MUTE_MONITORING_2;
        if (*info).monitor_mute != 0 {
            rmh.Cmd[2] |= AUDIO_IO_HAS_MUTE_MONITORING_1;
        }
    }
    vx_send_msg(chip, &mut rmh)
}

/* #if 0: not used in the C source; preserved as a non-compiled translation.
unsafe fn vx_read_audio_level(chip: *mut vx_core, audio: c_int, capture: c_int, info: *mut vx_audio_level) -> c_int {
    let mut err: c_int;
    let mut rmh = MaybeUninit::<vx_rmh>::zeroed().assume_init();
    ptr::write_bytes(info, 0, 1);
    vx_init_rmh(&mut rmh, CMD_GET_AUDIO_LEVELS);
    if capture != 0 { rmh.Cmd[0] |= COMMAND_RECORD_MASK; }
    rmh.Cmd[1] = 1u32 << audio;
    err = vx_send_msg(chip, &mut rmh);
    if err < 0 { return err; }
    (*info).level = (rmh.Stat[0] & MASK_DSP_WORD_LEVEL) as i16;
    (*info).monitor_level = ((rmh.Stat[0] >> 10) & MASK_DSP_WORD_LEVEL) as i16;
    (*info).mute = if (rmh.Stat[i] & AUDIO_IO_HAS_MUTE_LEVEL) != 0 { 1 } else { 0 };
    (*info).monitor_mute = if (rmh.Stat[i] & AUDIO_IO_HAS_MUTE_MONITORING_1) != 0 { 1 } else { 0 };
    0
}
*/

#[no_mangle]
pub unsafe extern "C" fn vx_set_monitor_level(
    chip: *mut vx_core,
    audio: c_int,
    level: c_int,
    active: c_int,
) -> c_int {
    let mut info = mem::zeroed::<vx_audio_level>();
    info.has_monitor_level = 1;
    info.monitor_level = level as i16;
    info.has_monitor_mute = 1;
    info.monitor_mute = (active == 0) as c_uint;
    (*chip).audio_monitor[audio as usize] = level;
    (*chip).audio_monitor_active[audio as usize] = active;
    vx_adjust_audio_level(chip, audio, 0, &mut info)
}

unsafe fn vx_set_audio_switch(chip: *mut vx_core, audio: c_int, active: c_int) -> c_int {
    let mut info = mem::zeroed::<vx_audio_level>();
    info.has_mute = 1;
    info.mute = (active == 0) as c_uint;
    (*chip).audio_active[audio as usize] = active;
    vx_adjust_audio_level(chip, audio, 0, &mut info)
}

unsafe fn vx_set_audio_gain(chip: *mut vx_core, audio: c_int, capture: c_int, level: c_int) -> c_int {
    let mut info = mem::zeroed::<vx_audio_level>();
    info.has_level = 1;
    info.level = level as i16;
    (*chip).audio_gain[capture as usize][audio as usize] = level;
    vx_adjust_audio_level(chip, audio, capture, &mut info)
}

unsafe fn vx_reset_audio_levels(chip: *mut vx_core) {
    ptr::write_bytes((*chip).audio_gain.as_mut_ptr(), 0, 1);
    ptr::write_bytes((*chip).audio_active.as_mut_ptr(), 0, 1);
    ptr::write_bytes((*chip).audio_monitor.as_mut_ptr(), 0, 1);
    ptr::write_bytes((*chip).audio_monitor_active.as_mut_ptr(), 0, 1);
    let mut c: c_uint = 0;
    while c < 2 {
        let mut i: c_uint = 0;
        while i < (*(*chip).hw).num_ins * 2 {
            let mut info = mem::zeroed::<vx_audio_level>();
            if c == 0 {
                info.has_monitor_level = 1;
                info.has_mute = 1;
                info.has_monitor_mute = 1;
            }
            info.has_level = 1;
            info.level = CVAL_0DB as i16;
            vx_adjust_audio_level(chip, i as c_int, c as c_int, &mut info);
            (*chip).audio_gain[c as usize][i as usize] = CVAL_0DB;
            (*chip).audio_monitor[i as usize] = CVAL_0DB;
            i += 1;
        }
        c += 1;
    }
}

const VU_METER_CHANNELS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
struct vx_vu_meter {
    saturated: c_int,
    vu_level: c_int,
    peak_level: c_int,
}

unsafe fn vx_get_audio_vu_meter(
    chip: *mut vx_core,
    audio: c_int,
    capture: c_int,
    mut info: *mut vx_vu_meter,
) -> c_int {
    let mut rmh = MaybeUninit::<vx_rmh>::zeroed().assume_init();
    let mut i: c_int;
    let err: c_int;
    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }
    vx_init_rmh(&mut rmh, CMD_AUDIO_VU_PIC_METER);
    rmh.LgStat += 2 * VU_METER_CHANNELS as c_uint;
    if capture != 0 {
        rmh.Cmd[0] |= COMMAND_RECORD_MASK;
    }
    rmh.Cmd[1] = 0;
    i = 0;
    while i < VU_METER_CHANNELS {
        rmh.Cmd[1] |= 1u32 << (audio + i);
        i += 1;
    }
    err = vx_send_msg(chip, &mut rmh);
    if err < 0 {
        return err;
    }
    i = 0;
    while i < 2 * VU_METER_CHANNELS {
        (*info).saturated = if (rmh.Stat[0] & (1u32 << (audio + i))) != 0 { 1 } else { 0 };
        (*info).vu_level = rmh.Stat[(i + 1) as usize] as c_int;
        (*info).peak_level = rmh.Stat[(i + 2) as usize] as c_int;
        info = info.add(1);
        i += 2;
    }
    0
}

unsafe extern "C" fn vx_output_level_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*(*chip).hw).output_level_max as i64;
    0
}

unsafe extern "C" fn vx_output_level_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let codec = (*kcontrol).id.index as usize;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.integer.value[0] = (*chip).output_level[codec][0] as i64;
    (*ucontrol).value.integer.value[1] = (*chip).output_level[codec][1] as i64;
    0
}

unsafe extern "C" fn vx_output_level_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let codec = (*kcontrol).id.index as usize;
    let vmax = (*(*chip).hw).output_level_max;
    let val = [
        (*ucontrol).value.integer.value[0] as c_uint,
        (*ucontrol).value.integer.value[1] as c_uint,
    ];
    if val[0] > vmax || val[1] > vmax {
        return -EINVAL;
    }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if val[0] != (*chip).output_level[codec][0] || val[1] != (*chip).output_level[codec][1] {
        vx_set_analog_output_level(chip, codec as c_int, val[0] as c_int, val[1] as c_int);
        (*chip).output_level[codec][0] = val[0];
        (*chip).output_level[codec][1] = val[1];
        return 1;
    }
    0
}

const MASTER_PLAYBACK_VOLUME: &[u8] = b"Master Playback Volume\0";
static vx_control_output_level: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, device: 0, subdevice: 0, name: MASTER_PLAYBACK_VOLUME.as_ptr() as *const c_char,
    index: 0, access: 0, count: 0, info: Some(vx_output_level_info),
    get: Some(vx_output_level_get), put: Some(vx_output_level_put),
    tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0,
};

unsafe extern "C" fn vx_audio_src_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static TEXT_DIGITAL: &[u8] = b"Digital\0";
    static TEXT_LINE: &[u8] = b"Line\0";
    static TEXT_MIC: &[u8] = b"Mic\0";
    static TEXT_ANALOG: &[u8] = b"Analog\0";
    let texts_mic = [TEXT_DIGITAL.as_ptr() as *const c_char, TEXT_LINE.as_ptr() as *const c_char, TEXT_MIC.as_ptr() as *const c_char];
    let texts_vx2 = [TEXT_DIGITAL.as_ptr() as *const c_char, TEXT_ANALOG.as_ptr() as *const c_char];
    let chip = snd_kcontrol_chip(kcontrol);
    if (*chip).type_ >= VX_TYPE_VXPOCKET {
        snd_ctl_enum_info(uinfo, 1, 3, texts_mic.as_ptr())
    } else {
        snd_ctl_enum_info(uinfo, 1, 2, texts_vx2.as_ptr())
    }
}

unsafe extern "C" fn vx_audio_src_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (*chip).audio_source_target;
    0
}

unsafe extern "C" fn vx_audio_src_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let item = (*ucontrol).value.enumerated.item[0];
    if (*chip).type_ >= VX_TYPE_VXPOCKET {
        if item > 2 { return -EINVAL; }
    } else if item > 1 {
        return -EINVAL;
    }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if (*chip).audio_source_target != item {
        (*chip).audio_source_target = item;
        vx_sync_audio_source(chip);
        return 1;
    }
    0
}

const CAPTURE_SOURCE: &[u8] = b"Capture Source\0";
static vx_control_audio_src: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, device: 0, subdevice: 0, name: CAPTURE_SOURCE.as_ptr() as *const c_char,
    index: 0, access: 0, count: 0, info: Some(vx_audio_src_info),
    get: Some(vx_audio_src_get), put: Some(vx_audio_src_put),
    tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0,
};

unsafe extern "C" fn vx_clock_mode_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static AUTO: &[u8] = b"Auto\0";
    static INTERNAL: &[u8] = b"Internal\0";
    static EXTERNAL: &[u8] = b"External\0";
    let texts = [AUTO.as_ptr() as *const c_char, INTERNAL.as_ptr() as *const c_char, EXTERNAL.as_ptr() as *const c_char];
    snd_ctl_enum_info(uinfo, 1, 3, texts.as_ptr())
}

unsafe extern "C" fn vx_clock_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = (*chip).clock_mode;
    0
}

unsafe extern "C" fn vx_clock_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let item = (*ucontrol).value.enumerated.item[0];
    if item > 2 { return -EINVAL; }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if (*chip).clock_mode != item {
        (*chip).clock_mode = item;
        vx_set_clock(chip, (*chip).freq);
        return 1;
    }
    0
}

const CLOCK_MODE: &[u8] = b"Clock Mode\0";
static vx_control_clock_mode: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0, device: 0, subdevice: 0, name: CLOCK_MODE.as_ptr() as *const c_char,
    index: 0, access: 0, count: 0, info: Some(vx_clock_mode_info),
    get: Some(vx_clock_mode_get), put: Some(vx_clock_mode_put),
    tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0,
};

unsafe extern "C" fn vx_audio_gain_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = CVAL_MAX as i64;
    0
}

unsafe extern "C" fn vx_audio_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as usize;
    let capture = (((*kcontrol).private_value >> 8) & 1) as usize;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.integer.value[0] = (*chip).audio_gain[capture][audio] as i64;
    (*ucontrol).value.integer.value[1] = (*chip).audio_gain[capture][audio + 1] as i64;
    0
}

unsafe extern "C" fn vx_audio_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let capture = (((*kcontrol).private_value >> 8) & 1) as c_int;
    let val = [(*ucontrol).value.integer.value[0] as c_uint, (*ucontrol).value.integer.value[1] as c_uint];
    if val[0] > CVAL_MAX || val[1] > CVAL_MAX { return -EINVAL; }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if val[0] as c_int != (*chip).audio_gain[capture as usize][audio as usize]
        || val[1] as c_int != (*chip).audio_gain[capture as usize][audio as usize + 1] {
        vx_set_audio_gain(chip, audio, capture, val[0] as c_int);
        vx_set_audio_gain(chip, audio + 1, capture, val[1] as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn vx_audio_monitor_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as usize;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.integer.value[0] = (*chip).audio_monitor[audio] as i64;
    (*ucontrol).value.integer.value[1] = (*chip).audio_monitor[audio + 1] as i64;
    0
}

unsafe extern "C" fn vx_audio_monitor_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let val = [(*ucontrol).value.integer.value[0] as c_uint, (*ucontrol).value.integer.value[1] as c_uint];
    if val[0] > CVAL_MAX || val[1] > CVAL_MAX { return -EINVAL; }
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if val[0] as c_int != (*chip).audio_monitor[audio as usize]
        || val[1] as c_int != (*chip).audio_monitor[audio as usize + 1] {
        vx_set_monitor_level(chip, audio, val[0] as c_int, (*chip).audio_monitor_active[audio as usize]);
        vx_set_monitor_level(chip, audio + 1, val[1] as c_int, (*chip).audio_monitor_active[audio as usize + 1]);
        return 1;
    }
    0
}

unsafe extern "C" fn vx_audio_sw_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_stereo_info(kcontrol, uinfo)
}

unsafe extern "C" fn vx_audio_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as usize;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.integer.value[0] = (*chip).audio_active[audio] as i64;
    (*ucontrol).value.integer.value[1] = (*chip).audio_active[audio + 1] as i64;
    0
}

unsafe extern "C" fn vx_audio_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if (*ucontrol).value.integer.value[0] as c_int != (*chip).audio_active[audio as usize]
        || (*ucontrol).value.integer.value[1] as c_int != (*chip).audio_active[audio as usize + 1] {
        vx_set_audio_switch(chip, audio, ((*ucontrol).value.integer.value[0] != 0) as c_int);
        vx_set_audio_switch(chip, audio + 1, ((*ucontrol).value.integer.value[1] != 0) as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn vx_monitor_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as usize;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.integer.value[0] = (*chip).audio_monitor_active[audio] as i64;
    (*ucontrol).value.integer.value[1] = (*chip).audio_monitor_active[audio + 1] as i64;
    0
}

unsafe extern "C" fn vx_monitor_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if (*ucontrol).value.integer.value[0] as c_int != (*chip).audio_monitor_active[audio as usize]
        || (*ucontrol).value.integer.value[1] as c_int != (*chip).audio_monitor_active[audio as usize + 1] {
        vx_set_monitor_level(chip, audio, (*chip).audio_monitor[audio as usize], ((*ucontrol).value.integer.value[0] != 0) as c_int);
        vx_set_monitor_level(chip, audio + 1, (*chip).audio_monitor[audio as usize + 1], ((*ucontrol).value.integer.value[1] != 0) as c_int);
        return 1;
    }
    0
}

/* DECLARE_TLV_DB_SCALE(db_scale_audio_gain, -10975, 25, 0) */
static db_scale_audio_gain: [c_uint; 4] = [0, (-10975i32) as c_uint, 25, 0];

const PCM_PLAYBACK_SWITCH: &[u8] = b"PCM Playback Switch\0";
const MONITORING_VOLUME: &[u8] = b"Monitoring Volume\0";
const MONITORING_SWITCH: &[u8] = b"Monitoring Switch\0";

static vx_control_audio_gain: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: ptr::null(), index: 0, access: 0, count: 0, info: Some(vx_audio_gain_info), get: Some(vx_audio_gain_get), put: Some(vx_audio_gain_put), tlv: snd_kcontrol_tlv { p: db_scale_audio_gain.as_ptr() }, private_value: 0 };
static vx_control_output_switch: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: PCM_PLAYBACK_SWITCH.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_audio_sw_info), get: Some(vx_audio_sw_get), put: Some(vx_audio_sw_put), tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };
static vx_control_monitor_gain: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: MONITORING_VOLUME.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_audio_gain_info), get: Some(vx_audio_monitor_get), put: Some(vx_audio_monitor_put), tlv: snd_kcontrol_tlv { p: db_scale_audio_gain.as_ptr() }, private_value: 0 };
static vx_control_monitor_switch: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: MONITORING_SWITCH.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_audio_sw_info), get: Some(vx_monitor_sw_get), put: Some(vx_monitor_sw_put), tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };

unsafe extern "C" fn vx_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn vx_iec958_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    (*ucontrol).value.iec958.status[0] = (((*chip).uer_bits >> 0) & 0xff) as u8;
    (*ucontrol).value.iec958.status[1] = (((*chip).uer_bits >> 8) & 0xff) as u8;
    (*ucontrol).value.iec958.status[2] = (((*chip).uer_bits >> 16) & 0xff) as u8;
    (*ucontrol).value.iec958.status[3] = (((*chip).uer_bits >> 24) & 0xff) as u8;
    0
}

unsafe extern "C" fn vx_iec958_mask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.iec958.status[0] = 0xff;
    (*ucontrol).value.iec958.status[1] = 0xff;
    (*ucontrol).value.iec958.status[2] = 0xff;
    (*ucontrol).value.iec958.status[3] = 0xff;
    0
}

unsafe extern "C" fn vx_iec958_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let val = ((*ucontrol).value.iec958.status[0] as c_uint) << 0
        | ((*ucontrol).value.iec958.status[1] as c_uint) << 8
        | ((*ucontrol).value.iec958.status[2] as c_uint) << 16
        | ((*ucontrol).value.iec958.status[3] as c_uint) << 24;
    let _guard = MutexGuard::new(ptr::addr_of_mut!((*chip).mixer_mutex).cast());
    if (*chip).uer_bits != val {
        (*chip).uer_bits = val;
        vx_set_iec958_status(chip, val);
        return 1;
    }
    0
}

const IEC958_PLAYBACK_MASK: &[u8] = b"IEC958 Playback Mask\0";
const IEC958_PLAYBACK_DEFAULT: &[u8] = b"IEC958 Playback Default\0";
static vx_control_iec958_mask: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: IEC958_PLAYBACK_MASK.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_iec958_info), get: Some(vx_iec958_mask_get), put: None, tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };
static vx_control_iec958: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: IEC958_PLAYBACK_DEFAULT.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_iec958_info), get: Some(vx_iec958_get), put: Some(vx_iec958_put), tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };

const METER_MAX: c_int = 0xff;
const METER_SHIFT: c_int = 16;

unsafe extern "C" fn vx_vu_meter_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = METER_MAX as i64;
    0
}

unsafe extern "C" fn vx_vu_meter_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut meter = [vx_vu_meter { saturated: 0, vu_level: 0, peak_level: 0 }; 2];
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let capture = (((*kcontrol).private_value >> 8) & 1) as c_int;
    vx_get_audio_vu_meter(chip, audio, capture, meter.as_mut_ptr());
    (*ucontrol).value.integer.value[0] = (meter[0].vu_level >> METER_SHIFT) as i64;
    (*ucontrol).value.integer.value[1] = (meter[1].vu_level >> METER_SHIFT) as i64;
    0
}

unsafe extern "C" fn vx_peak_meter_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut meter = [vx_vu_meter { saturated: 0, vu_level: 0, peak_level: 0 }; 2];
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    let capture = (((*kcontrol).private_value >> 8) & 1) as c_int;
    vx_get_audio_vu_meter(chip, audio, capture, meter.as_mut_ptr());
    (*ucontrol).value.integer.value[0] = (meter[0].peak_level >> METER_SHIFT) as i64;
    (*ucontrol).value.integer.value[1] = (meter[1].peak_level >> METER_SHIFT) as i64;
    0
}

unsafe extern "C" fn vx_saturation_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_stereo_info(kcontrol, uinfo)
}

unsafe extern "C" fn vx_saturation_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut meter = [vx_vu_meter { saturated: 0, vu_level: 0, peak_level: 0 }; 2];
    let audio = ((*kcontrol).private_value & 0xff) as c_int;
    vx_get_audio_vu_meter(chip, audio, 1, meter.as_mut_ptr());
    (*ucontrol).value.integer.value[0] = meter[0].saturated as i64;
    (*ucontrol).value.integer.value[1] = meter[1].saturated as i64;
    0
}

const INPUT_SATURATION: &[u8] = b"Input Saturation\0";
static vx_control_vu_meter: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: ptr::null(), index: 0, access: 0, count: 0, info: Some(vx_vu_meter_info), get: Some(vx_vu_meter_get), put: None, tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };
static vx_control_peak_meter: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: ptr::null(), index: 0, access: 0, count: 0, info: Some(vx_vu_meter_info), get: Some(vx_peak_meter_get), put: None, tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };
static vx_control_saturation: snd_kcontrol_new = snd_kcontrol_new { iface: 0, device: 0, subdevice: 0, name: INPUT_SATURATION.as_ptr() as *const c_char, index: 0, access: 0, count: 0, info: Some(vx_saturation_info), get: Some(vx_saturation_get), put: None, tlv: snd_kcontrol_tlv { p: ptr::null() }, private_value: 0 };

const PCM_PLAYBACK_VOLUME: &[u8] = b"PCM Playback Volume\0";
const PCM_CAPTURE_VOLUME: &[u8] = b"PCM Capture Volume\0";
const OUTPUT_VU_METER: &[u8] = b"Output VU Meter\0";
const INPUT_VU_METER: &[u8] = b"Input VU Meter\0";
const OUTPUT_PEAK_METER: &[u8] = b"Output Peak Meter\0";
const INPUT_PEAK_METER: &[u8] = b"Input Peak Meter\0";

#[no_mangle]
pub unsafe extern "C" fn snd_vx_mixer_new(chip: *mut vx_core) -> c_int {
    let mut i: c_uint;
    let mut c: c_uint;
    let mut err: c_int;
    let mut temp: snd_kcontrol_new;
    let card = (*chip).card;
    let mut name = [0 as c_char; 32];

    strscpy((*card).mixername.as_mut_ptr(), (*card).driver.as_ptr());

    i = 0;
    while i < (*(*chip).hw).num_outs {
        temp = vx_control_output_level;
        temp.index = i;
        temp.tlv.p = (*(*chip).hw).output_level_db_scale;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        i += 1;
    }

    i = 0;
    while i < (*(*chip).hw).num_outs {
        let val = (i * 2) as isize;
        temp = vx_control_audio_gain;
        temp.index = i;
        temp.name = PCM_PLAYBACK_VOLUME.as_ptr() as *const c_char;
        temp.private_value = val;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        temp = vx_control_output_switch;
        temp.index = i;
        temp.private_value = val;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        temp = vx_control_monitor_gain;
        temp.index = i;
        temp.private_value = val;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        temp = vx_control_monitor_switch;
        temp.index = i;
        temp.private_value = val;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        i += 1;
    }

    i = 0;
    while i < (*(*chip).hw).num_outs {
        temp = vx_control_audio_gain;
        temp.index = i;
        temp.name = PCM_CAPTURE_VOLUME.as_ptr() as *const c_char;
        temp.private_value = ((i * 2) | (1 << 8)) as isize;
        err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
        if err < 0 { return err; }
        i += 1;
    }

    err = snd_ctl_add(card, snd_ctl_new1(&vx_control_audio_src, chip.cast()));
    if err < 0 { return err; }
    err = snd_ctl_add(card, snd_ctl_new1(&vx_control_clock_mode, chip.cast()));
    if err < 0 { return err; }
    err = snd_ctl_add(card, snd_ctl_new1(&vx_control_iec958_mask, chip.cast()));
    if err < 0 { return err; }
    err = snd_ctl_add(card, snd_ctl_new1(&vx_control_iec958, chip.cast()));
    if err < 0 { return err; }

    c = 0;
    while c < 2 {
        i = 0;
        while i < (*(*chip).hw).num_ins {
            let val = ((i * 2) | (c << 8)) as isize;
            if c == 1 {
                temp = vx_control_saturation;
                temp.index = i;
                temp.private_value = val;
                err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
                if err < 0 { return err; }
            }
            sprintf(
                name.as_mut_ptr(),
                b"%s VU Meter\0".as_ptr() as *const c_char,
                if c == 0 { b"Output\0".as_ptr() } else { b"Input\0".as_ptr() },
            );
            temp = vx_control_vu_meter;
            temp.index = i;
            temp.name = if c == 0 { OUTPUT_VU_METER.as_ptr() as *const c_char } else { INPUT_VU_METER.as_ptr() as *const c_char };
            temp.private_value = val;
            err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
            if err < 0 { return err; }
            sprintf(
                name.as_mut_ptr(),
                b"%s Peak Meter\0".as_ptr() as *const c_char,
                if c == 0 { b"Output\0".as_ptr() } else { b"Input\0".as_ptr() },
            );
            temp = vx_control_peak_meter;
            temp.index = i;
            temp.name = if c == 0 { OUTPUT_PEAK_METER.as_ptr() as *const c_char } else { INPUT_PEAK_METER.as_ptr() as *const c_char };
            temp.private_value = val;
            err = snd_ctl_add(card, snd_ctl_new1(&temp, chip.cast()));
            if err < 0 { return err; }
            i += 1;
        }
        c += 1;
    }
    vx_reset_audio_levels(chip);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
