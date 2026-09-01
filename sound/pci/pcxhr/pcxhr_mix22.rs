// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * mixer interface for stereo cards
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* Dependencies from linux/delay.h, linux/io.h, linux/pci.h, sound/core.h,
 * sound/control.h, sound/tlv.h, sound/asoundef.h, pcxhr.h, pcxhr_core.h,
 * and pcxhr_mix22.h are expected to be supplied by the surrounding driver.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_ushort = u16;
type c_char = i8;
type c_long = i64;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcxhr_mgr {
    pub port: [usize; 3],
    pub capture_chips: c_int,
    pub board_has_analog: c_int,
    pub xlx_cfg: c_uchar,
    pub board_has_mic: c_int,
    pub pci: *mut pci_dev,
    pub dsp_reset: c_uchar,
    pub use_clock_type: pcxhr_clock_type,
    pub board_has_aes1: c_int,
    pub codec_speed: c_uint,
    pub sample_rate_real: c_uint,
    pub cur_clock_type: pcxhr_clock_type,
    pub last_reg_stat: c_uchar,
    pub xlx_selmic: c_uchar,
    pub mixer_mutex: mutex,
}

pub type pcxhr_clock_type = c_int;

#[repr(C)]
pub struct snd_pcxhr {
    pub card: *mut snd_card,
    pub mgr: *mut pcxhr_mgr,
    pub analog_capture_active: c_int,
    pub analog_capture_volume: [c_int; 2],
    pub mic_active: c_int,
    pub mic_volume: c_long,
    pub analog_playback_active: [c_int; 2],
    pub analog_playback_volume: [c_int; 2],
    pub audio_capture_source: c_int,
    pub aes_bits: [c_uchar; 24],
    pub chip_idx: c_int,
    pub mic_boost: c_long,
    pub phantom_power: c_int,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info_value_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_value_integer,
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
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub access: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
}

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x00000003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x00040000;

const HR22_CLOCK_TYPE_INTERNAL: pcxhr_clock_type = 0;
const HR22_CLOCK_TYPE_AES_SYNC: pcxhr_clock_type = 1;
const HR22_CLOCK_TYPE_AES_1: pcxhr_clock_type = 2;
const HR222_LINE_CAPTURE_LEVEL_MIN: c_int = 0;
const HR222_MICRO_CAPTURE_LEVEL_MIN: c_int = 0;
const HR222_MICRO_CAPTURE_LEVEL_MAX: c_int = 210;
const HR222_LINE_PLAYBACK_LEVEL_MIN: c_int = 0;

unsafe extern "C" {
    fn inb(port: usize) -> c_uchar;
    fn outb(value: c_uchar, port: usize);
    fn msleep(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn dev_dbg(dev: *const device, fmt: *const c_char, ...);
    fn pcxhr_pll_freq_register(rate: c_uint, ref_freq: c_uint, pllreg: *mut c_uint, realfreq: *mut c_uint) -> c_int;
    fn snd_pcm_direction_name(direction: c_int) -> *const c_char;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pcxhr;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut snd_pcxhr) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

/* registers used on the DSP and Xilinx (port 2) : HR stereo cards only */
const PCXHR_DSP_RESET: c_uchar = 0x20;
const PCXHR_XLX_CFG: c_uchar = 0x24;
const PCXHR_XLX_RUER: c_uchar = 0x28;
const PCXHR_XLX_DATA: c_uchar = 0x2C;
const PCXHR_XLX_STATUS: c_uchar = 0x30;
const PCXHR_XLX_LOFREQ: c_uchar = 0x34;
const PCXHR_XLX_HIFREQ: c_uchar = 0x38;
const PCXHR_XLX_CSUER: c_uchar = 0x3C;
const PCXHR_XLX_SELMIC: c_uchar = 0x40;

const PCXHR_DSP: usize = 2;

/* byte access only ! */
unsafe fn PCXHR_INPB(mgr: *mut pcxhr_mgr, x: c_uchar) -> c_uchar {
    unsafe { inb((*mgr).port[PCXHR_DSP].wrapping_add(x as usize)) }
}

unsafe fn PCXHR_OUTPB(mgr: *mut pcxhr_mgr, x: c_uchar, data: c_uchar) {
    unsafe { outb(data, (*mgr).port[PCXHR_DSP].wrapping_add(x as usize)) };
}

/* values for PCHR_DSP_RESET register */
const PCXHR_DSP_RESET_DSP: c_uchar = 0x01;
const PCXHR_DSP_RESET_MUTE: c_uchar = 0x02;
const PCXHR_DSP_RESET_CODEC: c_uchar = 0x08;
const PCXHR_DSP_RESET_SMPTE: c_uchar = 0x10;
const PCXHR_DSP_RESET_GPO_OFFSET: c_int = 5;
const PCXHR_DSP_RESET_GPO_MASK: c_uchar = 0x60;

/* values for PCHR_XLX_CFG register */
const PCXHR_CFG_SYNCDSP_MASK: c_uchar = 0x80;
const PCXHR_CFG_DEPENDENCY_MASK: c_uchar = 0x60;
const PCXHR_CFG_INDEPENDANT_SEL: c_uchar = 0x00;
const PCXHR_CFG_MASTER_SEL: c_uchar = 0x40;
const PCXHR_CFG_SLAVE_SEL: c_uchar = 0x20;
const PCXHR_CFG_DATA_UER1_SEL_MASK: c_uchar = 0x10; /* 0 (UER0), 1(UER1) */
const PCXHR_CFG_DATAIN_SEL_MASK: c_uchar = 0x08; /* 0 (ana), 1 (UER) */
const PCXHR_CFG_SRC_MASK: c_uchar = 0x04; /* 0 (Bypass), 1 (SRC Actif) */
const PCXHR_CFG_CLOCK_UER1_SEL_MASK: c_uchar = 0x02; /* 0 (UER0), 1(UER1) */
const PCXHR_CFG_CLOCKIN_SEL_MASK: c_uchar = 0x01; /* 0 (internal), 1 (AES/EBU) */

/* values for PCHR_XLX_DATA register */
const PCXHR_DATA_CODEC: c_uchar = 0x80;
const AKM_POWER_CONTROL_CMD: c_ushort = 0xA007;
const AKM_RESET_ON_CMD: c_ushort = 0xA100;
const AKM_RESET_OFF_CMD: c_ushort = 0xA103;
const AKM_CLOCK_INF_55K_CMD: c_ushort = 0xA240;
const AKM_CLOCK_SUP_55K_CMD: c_ushort = 0xA24D;
const AKM_MUTE_CMD: c_ushort = 0xA38D;
const AKM_UNMUTE_CMD: c_ushort = 0xA30D;
const AKM_LEFT_LEVEL_CMD: c_ushort = 0xA600;
const AKM_RIGHT_LEVEL_CMD: c_ushort = 0xA700;

/* values for PCHR_XLX_STATUS register - READ */
const PCXHR_STAT_SRC_LOCK: c_uchar = 0x01;
const PCXHR_STAT_LEVEL_IN: c_uchar = 0x02;
const PCXHR_STAT_GPI_OFFSET: c_int = 2;
const PCXHR_STAT_GPI_MASK: c_uchar = 0x0C;
const PCXHR_STAT_MIC_CAPS: c_uchar = 0x10;
/* values for PCHR_XLX_STATUS register - WRITE */
const PCXHR_STAT_FREQ_SYNC_MASK: c_uchar = 0x01;
const PCXHR_STAT_FREQ_UER1_MASK: c_uchar = 0x02;
const PCXHR_STAT_FREQ_SAVE_MASK: c_uchar = 0x80;

/* values for PCHR_XLX_CSUER register */
const PCXHR_SUER1_BIT_U_READ_MASK: c_uchar = 0x80;
const PCXHR_SUER1_BIT_C_READ_MASK: c_uchar = 0x40;
const PCXHR_SUER1_DATA_PRESENT_MASK: c_uchar = 0x20;
const PCXHR_SUER1_CLOCK_PRESENT_MASK: c_uchar = 0x10;
const PCXHR_SUER_BIT_U_READ_MASK: c_uchar = 0x08;
const PCXHR_SUER_BIT_C_READ_MASK: c_uchar = 0x04;
const PCXHR_SUER_DATA_PRESENT_MASK: c_uchar = 0x02;
const PCXHR_SUER_CLOCK_PRESENT_MASK: c_uchar = 0x01;

const PCXHR_SUER_BIT_U_WRITE_MASK: c_uchar = 0x02;
const PCXHR_SUER_BIT_C_WRITE_MASK: c_uchar = 0x01;

/* values for PCXHR_XLX_SELMIC register - WRITE */
const PCXHR_SELMIC_PREAMPLI_OFFSET: c_int = 2;
const PCXHR_SELMIC_PREAMPLI_MASK: c_uchar = 0x0C;
const PCXHR_SELMIC_PHANTOM_ALIM: c_uchar = 0x80;

static g_hr222_p_level: [c_uchar; 100] = [
    0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05,
    0x05, 0x05, 0x05, 0x06, 0x06, 0x06, 0x07, 0x07, 0x08, 0x08,
    0x09, 0x09, 0x0a, 0x0a, 0x0b, 0x0b, 0x0c, 0x0d, 0x0e, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x17, 0x18, 0x1a,
    0x1b, 0x1d, 0x1e, 0x20, 0x22, 0x24, 0x26, 0x28, 0x2b, 0x2d,
    0x30, 0x33, 0x36, 0x39, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x51,
    0x55, 0x5a, 0x60, 0x66, 0x6c, 0x72, 0x79, 0x80, 0x87, 0x8f,
    0x98, 0xa1, 0xaa, 0xb5, 0xbf, 0xcb, 0xd7, 0xe3, 0xf1, 0xff,
];

unsafe fn hr222_config_akm(mgr: *mut pcxhr_mgr, data: c_ushort) {
    let mut mask: c_ushort = 0x8000;
    /* activate access to codec registers */
    unsafe { PCXHR_INPB(mgr, PCXHR_XLX_HIFREQ) };

    while mask != 0 {
        unsafe {
            PCXHR_OUTPB(
                mgr,
                PCXHR_XLX_DATA,
                if (data & mask) != 0 { PCXHR_DATA_CODEC } else { 0 },
            )
        };
        mask >>= 1;
    }
    /* termiate access to codec registers */
    unsafe { PCXHR_INPB(mgr, PCXHR_XLX_RUER) };
}

unsafe fn hr222_set_hw_playback_level(mgr: *mut pcxhr_mgr, idx: c_int, level: c_int) -> c_int {
    let mut cmd: c_ushort;
    if idx > 1 || level < 0 || level as usize >= g_hr222_p_level.len() {
        return -EINVAL;
    }

    if idx == 0 {
        cmd = AKM_LEFT_LEVEL_CMD;
    } else {
        cmd = AKM_RIGHT_LEVEL_CMD;
    }

    /* conversion from PmBoardCodedLevel to AKM nonlinear programming */
    cmd = cmd.wrapping_add(g_hr222_p_level[level as usize] as c_ushort);

    unsafe { hr222_config_akm(mgr, cmd) };
    0
}

unsafe fn hr222_set_hw_capture_level(
    mgr: *mut pcxhr_mgr,
    level_l: c_int,
    level_r: c_int,
    level_mic: c_int,
) -> c_int {
    /* program all input levels at the same time */
    let mut data: c_uint;
    let mut i: c_int;

    if unsafe { (*mgr).capture_chips } == 0 {
        return -EINVAL; /* no PCX22 */
    }

    data = ((level_mic & 0xff) as c_uint) << 24; /* micro is mono, but apply */
    data |= ((level_mic & 0xff) as c_uint) << 16; /* level on both channels */
    data |= ((level_r & 0xff) as c_uint) << 8; /* line input right channel */
    data |= (level_l & 0xff) as c_uint; /* line input left channel */

    unsafe { PCXHR_INPB(mgr, PCXHR_XLX_DATA) }; /* activate input codec */
    /* send 32 bits (4 x 8 bits) */
    i = 0;
    while i < 32 {
        unsafe {
            PCXHR_OUTPB(
                mgr,
                PCXHR_XLX_DATA,
                if (data & 0x80000000) != 0 { PCXHR_DATA_CODEC } else { 0 },
            )
        };
        i += 1;
        data <<= 1;
    }
    unsafe { PCXHR_INPB(mgr, PCXHR_XLX_RUER) }; /* close input level codec */
    0
}

unsafe fn hr222_micro_boost(mgr: *mut pcxhr_mgr, level: c_int);

#[no_mangle]
pub unsafe extern "C" fn hr222_sub_init(mgr: *mut pcxhr_mgr) -> c_int {
    let reg: c_uchar;

    unsafe { (*mgr).board_has_analog = 1 }; /* analog always available */
    unsafe { (*mgr).xlx_cfg = PCXHR_CFG_SYNCDSP_MASK };

    reg = unsafe { PCXHR_INPB(mgr, PCXHR_XLX_STATUS) };
    if (reg & PCXHR_STAT_MIC_CAPS) != 0 {
        unsafe { (*mgr).board_has_mic = 1 }; /* microphone available */
    }
    unsafe {
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"MIC input available = %d\n".as_ptr(),
            (*mgr).board_has_mic,
        )
    };

    /* reset codec */
    unsafe { PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, PCXHR_DSP_RESET_DSP) };
    unsafe { msleep(5) };
    unsafe {
        (*mgr).dsp_reset = PCXHR_DSP_RESET_DSP | PCXHR_DSP_RESET_MUTE | PCXHR_DSP_RESET_CODEC
    };
    unsafe { PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, (*mgr).dsp_reset) };
    /* hr222_write_gpo(mgr, 0); does the same */
    unsafe { msleep(5) };

    /* config AKM */
    unsafe { hr222_config_akm(mgr, AKM_POWER_CONTROL_CMD) };
    unsafe { hr222_config_akm(mgr, AKM_CLOCK_INF_55K_CMD) };
    unsafe { hr222_config_akm(mgr, AKM_UNMUTE_CMD) };
    unsafe { hr222_config_akm(mgr, AKM_RESET_OFF_CMD) };

    /* init micro boost */
    unsafe { hr222_micro_boost(mgr, 0) };

    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_sub_set_clock(
    mgr: *mut pcxhr_mgr,
    rate: c_uint,
    changed: *mut c_int,
) -> c_int {
    let speed: c_uint;
    let mut pllreg: c_uint = 0;
    let err: c_int;
    let mut realfreq: c_uint = rate;

    match unsafe { (*mgr).use_clock_type } {
        HR22_CLOCK_TYPE_INTERNAL => {
            err = unsafe { pcxhr_pll_freq_register(rate, 219000, &mut pllreg, &mut realfreq) };
            if err != 0 {
                return err;
            }

            unsafe { (*mgr).xlx_cfg &= !(PCXHR_CFG_CLOCKIN_SEL_MASK | PCXHR_CFG_CLOCK_UER1_SEL_MASK) };
        }
        HR22_CLOCK_TYPE_AES_SYNC => {
            unsafe { (*mgr).xlx_cfg |= PCXHR_CFG_CLOCKIN_SEL_MASK };
            unsafe { (*mgr).xlx_cfg &= !PCXHR_CFG_CLOCK_UER1_SEL_MASK };
        }
        HR22_CLOCK_TYPE_AES_1 => {
            if unsafe { (*mgr).board_has_aes1 } == 0 {
                return -EINVAL;
            }

            unsafe { (*mgr).xlx_cfg |= PCXHR_CFG_CLOCKIN_SEL_MASK | PCXHR_CFG_CLOCK_UER1_SEL_MASK };
        }
        _ => return -EINVAL,
    }
    unsafe { hr222_config_akm(mgr, AKM_MUTE_CMD) };

    if unsafe { (*mgr).use_clock_type } == HR22_CLOCK_TYPE_INTERNAL {
        unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_HIFREQ, (pllreg >> 8) as c_uchar) };
        unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_LOFREQ, (pllreg & 0xff) as c_uchar) };
    }

    /* set clock source */
    unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_CFG, (*mgr).xlx_cfg) };

    /* codec speed modes */
    speed = if rate < 55000 { 0 } else { 1 };
    if unsafe { (*mgr).codec_speed } != speed {
        unsafe { (*mgr).codec_speed = speed };
        if speed == 0 {
            unsafe { hr222_config_akm(mgr, AKM_CLOCK_INF_55K_CMD) };
        } else {
            unsafe { hr222_config_akm(mgr, AKM_CLOCK_SUP_55K_CMD) };
        }
    }

    unsafe { (*mgr).sample_rate_real = realfreq };
    unsafe { (*mgr).cur_clock_type = (*mgr).use_clock_type };

    if !changed.is_null() {
        unsafe { *changed = 1 };
    }

    unsafe { hr222_config_akm(mgr, AKM_UNMUTE_CMD) };

    unsafe {
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"set_clock to %dHz (realfreq=%d pllreg=%x)\n".as_ptr(),
            rate,
            realfreq,
            pllreg,
        )
    };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_get_external_clock(
    mgr: *mut pcxhr_mgr,
    clock_type: pcxhr_clock_type,
    sample_rate: *mut c_int,
) -> c_int {
    let rate: c_int;
    let mut calc_rate: c_int = 0;
    let mut ticks: c_uint;
    let mask: c_uchar;
    let mut reg: c_uchar;

    if clock_type == HR22_CLOCK_TYPE_AES_SYNC {
        mask = PCXHR_SUER_CLOCK_PRESENT_MASK | PCXHR_SUER_DATA_PRESENT_MASK;
        reg = PCXHR_STAT_FREQ_SYNC_MASK;
    } else if clock_type == HR22_CLOCK_TYPE_AES_1 && unsafe { (*mgr).board_has_aes1 } != 0 {
        mask = PCXHR_SUER1_CLOCK_PRESENT_MASK | PCXHR_SUER1_DATA_PRESENT_MASK;
        reg = PCXHR_STAT_FREQ_UER1_MASK;
    } else {
        unsafe {
            dev_dbg(
                &(*(*mgr).pci).dev,
                c"get_external_clock : type %d not supported\n".as_ptr(),
                clock_type,
            )
        };
        return -EINVAL; /* other clocks not supported */
    }

    if (unsafe { PCXHR_INPB(mgr, PCXHR_XLX_CSUER) } & mask) != mask {
        unsafe {
            dev_dbg(
                &(*(*mgr).pci).dev,
                c"get_external_clock(%d) = 0 Hz\n".as_ptr(),
                clock_type,
            )
        };
        unsafe { *sample_rate = 0 };
        return 0; /* no external clock locked */
    }

    unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_STATUS, reg) }; /* calculate freq */

    /* save the measured clock frequency */
    reg |= PCXHR_STAT_FREQ_SAVE_MASK;

    if unsafe { (*mgr).last_reg_stat } != reg {
        unsafe { udelay(500) }; /* wait min 2 cycles of lowest freq (8000) */
        unsafe { (*mgr).last_reg_stat = reg };
    }

    unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_STATUS, reg) }; /* save */

    /* get the frequency */
    ticks = unsafe { PCXHR_INPB(mgr, PCXHR_XLX_CFG) } as c_uint;
    ticks = (ticks & 0x03) << 8;
    ticks |= unsafe { PCXHR_INPB(mgr, PCXHR_DSP_RESET) } as c_uint;

    if ticks != 0 {
        calc_rate = (28224000 / ticks) as c_int;
    }
    /* rounding */
    if calc_rate > 184200 {
        rate = 192000;
    } else if calc_rate > 152200 {
        rate = 176400;
    } else if calc_rate > 112000 {
        rate = 128000;
    } else if calc_rate > 92100 {
        rate = 96000;
    } else if calc_rate > 76100 {
        rate = 88200;
    } else if calc_rate > 56000 {
        rate = 64000;
    } else if calc_rate > 46050 {
        rate = 48000;
    } else if calc_rate > 38050 {
        rate = 44100;
    } else if calc_rate > 28000 {
        rate = 32000;
    } else if calc_rate > 23025 {
        rate = 24000;
    } else if calc_rate > 19025 {
        rate = 22050;
    } else if calc_rate > 14000 {
        rate = 16000;
    } else if calc_rate > 11512 {
        rate = 12000;
    } else if calc_rate > 9512 {
        rate = 11025;
    } else if calc_rate > 7000 {
        rate = 8000;
    } else {
        rate = 0;
    }

    unsafe {
        dev_dbg(
            &(*(*mgr).pci).dev,
            c"External clock is at %d Hz (measured %d Hz)\n".as_ptr(),
            rate,
            calc_rate,
        )
    };
    unsafe { *sample_rate = rate };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_read_gpio(
    mgr: *mut pcxhr_mgr,
    is_gpi: c_int,
    value: *mut c_int,
) -> c_int {
    if is_gpi != 0 {
        let reg: c_uchar = unsafe { PCXHR_INPB(mgr, PCXHR_XLX_STATUS) };
        unsafe { *value = ((reg & PCXHR_STAT_GPI_MASK) as c_int) >> PCXHR_STAT_GPI_OFFSET };
    } else {
        unsafe {
            *value = (((*mgr).dsp_reset & PCXHR_DSP_RESET_GPO_MASK) as c_int)
                >> PCXHR_DSP_RESET_GPO_OFFSET
        };
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_write_gpo(mgr: *mut pcxhr_mgr, value: c_int) -> c_int {
    let mut reg: c_uchar = unsafe { (*mgr).dsp_reset & !PCXHR_DSP_RESET_GPO_MASK };

    reg |= ((value << PCXHR_DSP_RESET_GPO_OFFSET) as c_uchar) & PCXHR_DSP_RESET_GPO_MASK;

    unsafe { PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, reg) };
    unsafe { (*mgr).dsp_reset = reg };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_manage_timecode(mgr: *mut pcxhr_mgr, enable: c_int) -> c_int {
    if enable != 0 {
        unsafe { (*mgr).dsp_reset |= PCXHR_DSP_RESET_SMPTE };
    } else {
        unsafe { (*mgr).dsp_reset &= !PCXHR_DSP_RESET_SMPTE };
    }

    unsafe { PCXHR_OUTPB(mgr, PCXHR_DSP_RESET, (*mgr).dsp_reset) };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_update_analog_audio_level(
    chip: *mut snd_pcxhr,
    is_capture: c_int,
    channel: c_int,
) -> c_int {
    unsafe {
        dev_dbg(
            (*(*chip).card).dev,
            c"hr222_update_analog_audio_level(%s chan=%d)\n".as_ptr(),
            snd_pcm_direction_name(is_capture),
            channel,
        )
    };
    if is_capture != 0 {
        let level_l: c_int;
        let level_r: c_int;
        let level_mic: c_int;
        /* we have to update all levels */
        if unsafe { (*chip).analog_capture_active } != 0 {
            level_l = unsafe { (*chip).analog_capture_volume[0] };
            level_r = unsafe { (*chip).analog_capture_volume[1] };
        } else {
            level_l = HR222_LINE_CAPTURE_LEVEL_MIN;
            level_r = HR222_LINE_CAPTURE_LEVEL_MIN;
        }
        if unsafe { (*chip).mic_active } != 0 {
            level_mic = unsafe { (*chip).mic_volume as c_int };
        } else {
            level_mic = HR222_MICRO_CAPTURE_LEVEL_MIN;
        }
        unsafe { hr222_set_hw_capture_level((*chip).mgr, level_l, level_r, level_mic) }
    } else {
        let vol: c_int;
        if unsafe { (*chip).analog_playback_active[channel as usize] } != 0 {
            vol = unsafe { (*chip).analog_playback_volume[channel as usize] };
        } else {
            vol = HR222_LINE_PLAYBACK_LEVEL_MIN;
        }
        unsafe { hr222_set_hw_playback_level((*chip).mgr, channel, vol) }
    }
}

/*texts[5] = {"Line", "Digital", "Digi+SRC", "Mic", "Line+Mic"}*/
const SOURCE_LINE: c_int = 0;
const SOURCE_DIGITAL: c_int = 1;
const SOURCE_DIGISRC: c_int = 2;
const SOURCE_MIC: c_int = 3;
const SOURCE_LINEMIC: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn hr222_set_audio_source(chip: *mut snd_pcxhr) -> c_int {
    let mut digital: c_int = 0;
    /* default analog source */
    unsafe {
        (*(*chip).mgr).xlx_cfg &=
            !(PCXHR_CFG_SRC_MASK | PCXHR_CFG_DATAIN_SEL_MASK | PCXHR_CFG_DATA_UER1_SEL_MASK)
    };

    if unsafe { (*chip).audio_capture_source } == SOURCE_DIGISRC {
        unsafe { (*(*chip).mgr).xlx_cfg |= PCXHR_CFG_SRC_MASK };
        digital = 1;
    } else if unsafe { (*chip).audio_capture_source } == SOURCE_DIGITAL {
        digital = 1;
    }
    if digital != 0 {
        unsafe { (*(*chip).mgr).xlx_cfg |= PCXHR_CFG_DATAIN_SEL_MASK };
        if unsafe { (*(*chip).mgr).board_has_aes1 } != 0 {
            /* get data from the AES1 plug */
            unsafe { (*(*chip).mgr).xlx_cfg |= PCXHR_CFG_DATA_UER1_SEL_MASK };
        }
        /* chip->mic_active = 0; */
        /* chip->analog_capture_active = 0; */
    } else {
        let mut update_lvl: c_int = 0;
        unsafe { (*chip).analog_capture_active = 0 };
        unsafe { (*chip).mic_active = 0 };
        if unsafe { (*chip).audio_capture_source } == SOURCE_LINE
            || unsafe { (*chip).audio_capture_source } == SOURCE_LINEMIC
        {
            if unsafe { (*chip).analog_capture_active } == 0 {
                update_lvl = 1;
            }
            unsafe { (*chip).analog_capture_active = 1 };
        }
        if unsafe { (*chip).audio_capture_source } == SOURCE_MIC
            || unsafe { (*chip).audio_capture_source } == SOURCE_LINEMIC
        {
            if unsafe { (*chip).mic_active } == 0 {
                update_lvl = 1;
            }
            unsafe { (*chip).mic_active = 1 };
        }
        if update_lvl != 0 {
            /* capture: update all 3 mutes/unmutes with one call */
            unsafe { hr222_update_analog_audio_level(chip, 1, 0) };
        }
    }
    /* set the source infos (max 3 bits modified) */
    unsafe { PCXHR_OUTPB((*chip).mgr, PCXHR_XLX_CFG, (*(*chip).mgr).xlx_cfg) };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_iec958_capture_byte(
    chip: *mut snd_pcxhr,
    aes_idx: c_int,
    aes_bits: *mut c_uchar,
) -> c_int {
    let mut idx: c_uchar = (aes_idx * 8) as c_uchar;
    let mut temp: c_uchar = 0;
    let mask: c_uchar = if unsafe { (*(*chip).mgr).board_has_aes1 } != 0 {
        PCXHR_SUER1_BIT_C_READ_MASK
    } else {
        PCXHR_SUER_BIT_C_READ_MASK
    };
    let mut i: c_int = 0;
    while i < 8 {
        unsafe { PCXHR_OUTPB((*chip).mgr, PCXHR_XLX_RUER, idx) }; /* idx < 192 */
        idx = idx.wrapping_add(1);
        temp <<= 1;
        if (unsafe { PCXHR_INPB((*chip).mgr, PCXHR_XLX_CSUER) } & mask) != 0 {
            temp |= 1;
        }
        i += 1;
    }
    unsafe {
        dev_dbg(
            (*(*chip).card).dev,
            c"read iec958 AES %d byte %d = 0x%x\n".as_ptr(),
            (*chip).chip_idx,
            aes_idx,
            temp as c_int,
        )
    };
    unsafe { *aes_bits = temp };
    0
}

#[no_mangle]
pub unsafe extern "C" fn hr222_iec958_update_byte(
    chip: *mut snd_pcxhr,
    aes_idx: c_int,
    aes_bits: c_uchar,
) -> c_int {
    let mut i: c_int;
    let mut new_bits: c_uchar = aes_bits;
    let mut old_bits: c_uchar = unsafe { (*chip).aes_bits[aes_idx as usize] };
    let mut idx: c_uchar = (aes_idx * 8) as c_uchar;
    i = 0;
    while i < 8 {
        if (old_bits & 0x01) != (new_bits & 0x01) {
            /* idx < 192 */
            unsafe { PCXHR_OUTPB((*chip).mgr, PCXHR_XLX_RUER, idx) };
            /* write C and U bit */
            unsafe {
                PCXHR_OUTPB(
                    (*chip).mgr,
                    PCXHR_XLX_CSUER,
                    if (new_bits & 0x01) != 0 {
                        PCXHR_SUER_BIT_C_WRITE_MASK
                    } else {
                        0
                    },
                )
            };
        }
        idx = idx.wrapping_add(1);
        old_bits >>= 1;
        new_bits >>= 1;
        i += 1;
    }
    unsafe { (*chip).aes_bits[aes_idx as usize] = aes_bits };
    0
}

unsafe fn hr222_micro_boost(mgr: *mut pcxhr_mgr, level: c_int) {
    let boost_mask: c_uchar;
    boost_mask = (level << PCXHR_SELMIC_PREAMPLI_OFFSET) as c_uchar;
    if (boost_mask & (!PCXHR_SELMIC_PREAMPLI_MASK)) != 0 {
        return; /* only values form 0 to 3 accepted */
    }

    unsafe { (*mgr).xlx_selmic &= !PCXHR_SELMIC_PREAMPLI_MASK };
    unsafe { (*mgr).xlx_selmic |= boost_mask };

    unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_SELMIC, (*mgr).xlx_selmic) };

    unsafe { dev_dbg(&(*(*mgr).pci).dev, c"hr222_micro_boost : set %x\n".as_ptr(), boost_mask as c_int) };
}

unsafe fn hr222_phantom_power(mgr: *mut pcxhr_mgr, power: c_int) {
    if power != 0 {
        unsafe { (*mgr).xlx_selmic |= PCXHR_SELMIC_PHANTOM_ALIM };
    } else {
        unsafe { (*mgr).xlx_selmic &= !PCXHR_SELMIC_PHANTOM_ALIM };
    }

    unsafe { PCXHR_OUTPB(mgr, PCXHR_XLX_SELMIC, (*mgr).xlx_selmic) };

    unsafe { dev_dbg(&(*(*mgr).pci).dev, c"hr222_phantom_power : set %d\n".as_ptr(), power) };
}

/* mic level */
static db_scale_mic_hr222: [c_uint; 4] = [0, (-9850i32) as c_uint, 50, 650];

unsafe extern "C" fn hr222_mic_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER };
    unsafe { (*uinfo).count = 1 };
    unsafe { (*uinfo).value.integer.min = HR222_MICRO_CAPTURE_LEVEL_MIN as c_long }; /* -98 dB */
    /* gains from 9 dB to 31.5 dB not recommended; use micboost instead */
    unsafe { (*uinfo).value.integer.max = HR222_MICRO_CAPTURE_LEVEL_MAX as c_long }; /*  +7 dB */
    0
}

unsafe extern "C" fn hr222_mic_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    unsafe { (*ucontrol).value.integer.value[0] = (*chip).mic_volume };
    0
}

unsafe extern "C" fn hr222_mic_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut changed: c_int = 0;

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    if unsafe { (*chip).mic_volume != (*ucontrol).value.integer.value[0] } {
        changed = 1;
        unsafe { (*chip).mic_volume = (*ucontrol).value.integer.value[0] };
        unsafe { hr222_update_analog_audio_level(chip, 1, 0) };
    }
    changed
}

static hr222_control_mic_level: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"Mic Capture Volume".as_ptr(),
    info: Some(hr222_mic_vol_info),
    get: Some(hr222_mic_vol_get),
    put: Some(hr222_mic_vol_put),
    tlv: snd_kcontrol_tlv {
        p: db_scale_mic_hr222.as_ptr(),
    },
};

/* mic boost level */
static db_scale_micboost_hr222: [c_uint; 4] = [0, 0, 1800, 5400];

unsafe extern "C" fn hr222_mic_boost_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER };
    unsafe { (*uinfo).count = 1 };
    unsafe { (*uinfo).value.integer.min = 0 }; /*  0 dB */
    unsafe { (*uinfo).value.integer.max = 3 }; /* 54 dB */
    0
}

unsafe extern "C" fn hr222_mic_boost_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    unsafe { (*ucontrol).value.integer.value[0] = (*chip).mic_boost };
    0
}

unsafe extern "C" fn hr222_mic_boost_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut changed: c_int = 0;

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    if unsafe { (*chip).mic_boost != (*ucontrol).value.integer.value[0] } {
        changed = 1;
        unsafe { (*chip).mic_boost = (*ucontrol).value.integer.value[0] };
        unsafe { hr222_micro_boost((*chip).mgr, (*chip).mic_boost as c_int) };
    }
    changed
}

static hr222_control_mic_boost: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"MicBoost Capture Volume".as_ptr(),
    info: Some(hr222_mic_boost_info),
    get: Some(hr222_mic_boost_get),
    put: Some(hr222_mic_boost_put),
    tlv: snd_kcontrol_tlv {
        p: db_scale_micboost_hr222.as_ptr(),
    },
};

/******************* Phantom power switch *******************/
const hr222_phantom_power_info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int =
    snd_ctl_boolean_mono_info;

unsafe extern "C" fn hr222_phantom_power_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    unsafe { (*ucontrol).value.integer.value[0] = (*chip).phantom_power as c_long };
    0
}

unsafe extern "C" fn hr222_phantom_power_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pcxhr = unsafe { snd_kcontrol_chip(kcontrol) };
    let power: c_int;
    let mut changed: c_int = 0;

    let _guard = unsafe { MutexGuard::new(&mut (*(*chip).mgr).mixer_mutex) };
    power = if unsafe { (*ucontrol).value.integer.value[0] } != 0 { 1 } else { 0 };
    if unsafe { (*chip).phantom_power } != power {
        unsafe { hr222_phantom_power((*chip).mgr, power) };
        unsafe { (*chip).phantom_power = power };
        changed = 1;
    }
    changed
}

static hr222_phantom_power_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: 0,
    name: c"Phantom Power Switch".as_ptr(),
    info: Some(hr222_phantom_power_info),
    get: Some(hr222_phantom_power_get),
    put: Some(hr222_phantom_power_put),
    tlv: snd_kcontrol_tlv {
        p: core::ptr::null(),
    },
};

#[no_mangle]
pub unsafe extern "C" fn hr222_add_mic_controls(chip: *mut snd_pcxhr) -> c_int {
    let mut err: c_int;
    if unsafe { (*(*chip).mgr).board_has_mic } == 0 {
        return 0;
    }

    /* controls */
    err = unsafe { snd_ctl_add((*chip).card, snd_ctl_new1(&hr222_control_mic_level, chip)) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_ctl_add((*chip).card, snd_ctl_new1(&hr222_control_mic_boost, chip)) };
    if err < 0 {
        return err;
    }

    err = unsafe { snd_ctl_add((*chip).card, snd_ctl_new1(&hr222_phantom_power_switch, chip)) };
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
