// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Matt Wu <Matt_Wu@acersoftech.com.cn>
 *  Apr 26, 2001
 *  Routines for control of ALi pci audio M5451
 *
 *  BUGS:
 *    --
 *
 *  TODO:
 *    --
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type bool_t = bool;

const MODULE_AUTHOR_TEXT: &[u8] = b"Matt Wu <Matt_Wu@acersoftech.com.cn>\0";
const MODULE_DESCRIPTION_TEXT: &[u8] = b"ALI M5451\0";
const MODULE_LICENSE_TEXT: &[u8] = b"GPL\0";

static mut index: c_int = SNDRV_DEFAULT_IDX1;
static mut id: *mut c_char = SNDRV_DEFAULT_STR1 as *mut c_char;
static mut pcm_channels: c_int = 32;
static mut spdif: bool_t = false;

/* just for backward compatibility */
static mut enable: bool_t = false;

/*
 *  Constants definition
 */

const DEVICE_ID_ALI5451: c_uint = (PCI_VENDOR_ID_AL << 16) | PCI_DEVICE_ID_AL_M5451;

const ALI_CHANNELS: usize = 32;

const ALI_PCM_IN_CHANNEL: c_uint = 31;
const ALI_SPDIF_IN_CHANNEL: c_uint = 19;
const ALI_SPDIF_OUT_CHANNEL: c_uint = 15;
const ALI_CENTER_CHANNEL: c_uint = 24;
const ALI_LEF_CHANNEL: c_uint = 23;
const ALI_SURR_LEFT_CHANNEL: c_uint = 26;
const ALI_SURR_RIGHT_CHANNEL: c_uint = 25;
const ALI_MODEM_IN_CHANNEL: c_uint = 21;
const ALI_MODEM_OUT_CHANNEL: c_uint = 20;

const SNDRV_ALI_VOICE_TYPE_PCM: c_int = 0o1;
const SNDRV_ALI_VOICE_TYPE_OTH: c_int = 0o2;

const ALI_5451_V02: u8 = 0x02;

/*
 *  Direct Registers
 */

const ALI_LEGACY_DMAR0: c_uint = 0x00; /* ADR0 */
const ALI_LEGACY_DMAR4: c_uint = 0x04; /* CNT0 */
const ALI_LEGACY_DMAR11: c_uint = 0x0b; /* MOD  */
const ALI_LEGACY_DMAR15: c_uint = 0x0f; /* MMR  */
const ALI_MPUR0: c_uint = 0x20;
const ALI_MPUR1: c_uint = 0x21;
const ALI_MPUR2: c_uint = 0x22;
const ALI_MPUR3: c_uint = 0x23;

const ALI_AC97_WRITE: c_uint = 0x40;
const ALI_AC97_READ: c_uint = 0x44;

const ALI_SCTRL: c_uint = 0x48;
const ALI_SPDIF_OUT_ENABLE: c_uint = 0x20;
const ALI_SCTRL_LINE_IN2: c_uint = 1 << 9;
const ALI_SCTRL_GPIO_IN2: c_uint = 1 << 13;
const ALI_SCTRL_LINE_OUT_EN: c_uint = 1 << 20;
const ALI_SCTRL_GPIO_OUT_EN: c_uint = 1 << 23;
const ALI_SCTRL_CODEC1_READY: c_uint = 1 << 24;
const ALI_SCTRL_CODEC2_READY: c_uint = 1 << 25;
const ALI_AC97_GPIO: c_uint = 0x4c;
const ALI_AC97_GPIO_ENABLE: c_uint = 0x8000;
const ALI_AC97_GPIO_DATA_SHIFT: c_uint = 16;
const ALI_SPDIF_CS: c_uint = 0x70;
const ALI_SPDIF_CTRL: c_uint = 0x74;
const ALI_SPDIF_IN_FUNC_ENABLE: c_uint = 0x02;
const ALI_SPDIF_IN_CH_STATUS: c_uint = 0x40;
const ALI_SPDIF_OUT_CH_STATUS: c_uint = 0xbf;
const ALI_START: c_uint = 0x80;
const ALI_STOP: c_uint = 0x84;
const ALI_CSPF: c_uint = 0x90;
const ALI_AINT: c_uint = 0x98;
const ALI_GC_CIR: c_uint = 0xa0;
const ENDLP_IE: c_uint = 0x00001000;
const MIDLP_IE: c_uint = 0x00002000;
const ALI_AINTEN: c_uint = 0xa4;
const ALI_VOLUME: c_uint = 0xa8;
const ALI_SBDELTA_DELTA_R: c_uint = 0xac;
const ALI_MISCINT: c_uint = 0xb0;
const ADDRESS_IRQ: c_uint = 0x00000020;
const TARGET_REACHED: c_uint = 0x00008000;
const MIXER_OVERFLOW: c_uint = 0x00000800;
const MIXER_UNDERFLOW: c_uint = 0x00000400;
const GPIO_IRQ: c_uint = 0x01000000;
const ALI_SBBL_SBCL: c_uint = 0xc0;
const ALI_SBCTRL_SBE2R_SBDD: c_uint = 0xc4;
const ALI_STIMER: c_uint = 0xc8;
const ALI_GLOBAL_CONTROL: c_uint = 0xd4;
const ALI_SPDIF_OUT_SEL_PCM: c_uint = 0x00000400; /* bit 10 */
const ALI_SPDIF_IN_SUPPORT: c_uint = 0x00000800; /* bit 11 */
const ALI_SPDIF_OUT_CH_ENABLE: c_uint = 0x00008000; /* bit 15 */
const ALI_SPDIF_IN_CH_ENABLE: c_uint = 0x00080000; /* bit 19 */
const ALI_PCM_IN_ENABLE: c_uint = 0x80000000; /* bit 31 */

const ALI_CSO_ALPHA_FMS: c_uint = 0xe0;
const ALI_LBA: c_uint = 0xe4;
const ALI_ESO_DELTA: c_uint = 0xe8;
const ALI_GVSEL_PAN_VOC_CTRL_EC: c_uint = 0xf0;
const ALI_EBUF1: c_uint = 0xf4;
const ALI_EBUF2: c_uint = 0xf8;

unsafe fn ALI_REG(codec: *mut snd_ali, x: c_uint) -> c_ulong {
    (*codec).port.wrapping_add(x as c_ulong)
}

const MAX_CODECS: usize = 2;

#[repr(C)]
pub struct snd_ali_channel_control {
    /* register data */
    pub data: REGDATA,
    /* register addresses */
    pub regs: REGS,
}

#[repr(C)]
pub struct REGDATA {
    pub start: c_uint,
    pub stop: c_uint,
    pub aint: c_uint,
    pub ainten: c_uint,
}

#[repr(C)]
pub struct REGS {
    pub start: c_uint,
    pub stop: c_uint,
    pub aint: c_uint,
    pub ainten: c_uint,
    pub ac97read: c_uint,
    pub ac97write: c_uint,
}

#[repr(C)]
pub struct snd_ali_voice {
    pub number: c_uint,
    /* C bitfields: use, pcm, midi, mode, synth, running */
    pub flags: c_uint,
    /* PCM data */
    pub codec: *mut snd_ali,
    pub substream: *mut snd_pcm_substream,
    pub extra: *mut snd_ali_voice,
    pub eso: c_int,   /* final ESO value for channel */
    pub count: c_int, /* runtime->period_size */
    /* --- */
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut c_void)>,
}

const VOICE_USE: c_uint = 1 << 0;
const VOICE_PCM: c_uint = 1 << 1;
const VOICE_MIDI: c_uint = 1 << 2;
const VOICE_MODE: c_uint = 1 << 3;
const VOICE_SYNTH: c_uint = 1 << 4;
const VOICE_RUNNING: c_uint = 1 << 5;

unsafe fn voice_get(v: *const snd_ali_voice, bit: c_uint) -> bool {
    ((*v).flags & bit) != 0
}
unsafe fn voice_set(v: *mut snd_ali_voice, bit: c_uint, on: bool) {
    if on {
        (*v).flags |= bit;
    } else {
        (*v).flags &= !bit;
    }
}

#[repr(C)]
pub struct snd_alidev {
    pub voices: [snd_ali_voice; ALI_CHANNELS],
    pub chcnt: c_uint,      /* num of opened channels */
    pub chmap: c_uint,      /* bitmap for opened channels */
    pub synthcount: c_uint,
}

const ALI_GLOBAL_REGS: usize = 56;
const ALI_CHANNEL_REGS: usize = 8;
#[repr(C)]
pub struct snd_ali_image {
    pub regs: [u32; ALI_GLOBAL_REGS],
    pub channel_regs: [[u32; ALI_CHANNEL_REGS]; ALI_CHANNELS],
}

#[repr(C)]
pub struct snd_ali {
    pub irq: c_int,
    pub port: c_ulong,
    pub revision: u8,
    /* C bitfields: hw_initialized, spdif_support */
    pub flags: c_uint,
    pub pci: *mut pci_dev,
    pub pci_m1533: *mut pci_dev,
    pub pci_m7101: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: [*mut snd_pcm; MAX_CODECS],
    pub synth: snd_alidev,
    pub chregs: snd_ali_channel_control,
    /* S/PDIF Mask */
    pub spdif_mask: c_uint,
    pub spurious_irq_count: c_uint,
    pub spurious_irq_max_delta: c_uint,
    pub num_of_codecs: c_uint,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: [*mut snd_ac97; MAX_CODECS],
    pub ac97_ext_id: u16,
    pub ac97_ext_status: u16,
    pub reg_lock: spinlock_t,
    pub voice_alloc: spinlock_t,
    pub image: snd_ali_image,
}

const ALI_HW_INITIALIZED: c_uint = 1 << 0;
const ALI_SPDIF_SUPPORT: c_uint = 1 << 1;

unsafe fn ali_get(chip: *const snd_ali, bit: c_uint) -> bool {
    ((*chip).flags & bit) != 0
}
unsafe fn ali_set(chip: *mut snd_ali, bit: c_uint, on: bool) {
    if on {
        (*chip).flags |= bit;
    } else {
        (*chip).flags &= !bit;
    }
}

static mut snd_ali_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_AL, device: PCI_DEVICE_ID_AL_M5451 },
    pci_device_id { vendor: 0, device: 0 },
];

/*
 *  AC97 ACCESS
 */

unsafe fn snd_ali_5451_peek(codec: *mut snd_ali, port: c_uint) -> c_uint {
    inl(ALI_REG(codec, port)) as c_uint
}

unsafe fn snd_ali_5451_poke(codec: *mut snd_ali, port: c_uint, val: c_uint) {
    outl(val, ALI_REG(codec, port));
}

unsafe fn snd_ali_codec_ready(codec: *mut snd_ali, port: c_uint) -> c_int {
    let mut end_time: c_ulong;
    let mut res: c_uint;

    end_time = jiffies.wrapping_add(msecs_to_jiffies(250));

    loop {
        res = snd_ali_5451_peek(codec, port);
        if (res & 0x8000) == 0 {
            return 0;
        }
        if time_after_eq(end_time, jiffies) == 0 {
            break;
        }
        schedule_timeout_uninterruptible(1);
    }

    snd_ali_5451_poke(codec, port, res & !0x8000);
    dev_dbg((*(*codec).card).dev, b"ali_codec_ready: codec is not ready.\n\0".as_ptr() as *const c_char);
    -EIO
}

unsafe fn snd_ali_stimer_ready(codec: *mut snd_ali) -> c_int {
    let end_time: c_ulong;
    let dwChk1: c_ulong;
    let mut dwChk2: c_ulong;

    dwChk1 = snd_ali_5451_peek(codec, ALI_STIMER) as c_ulong;
    end_time = jiffies.wrapping_add(msecs_to_jiffies(250));

    loop {
        dwChk2 = snd_ali_5451_peek(codec, ALI_STIMER) as c_ulong;
        if dwChk2 != dwChk1 {
            return 0;
        }
        if time_after_eq(end_time, jiffies) == 0 {
            break;
        }
        schedule_timeout_uninterruptible(1);
    }

    dev_err((*(*codec).card).dev, b"ali_stimer_read: stimer is not ready.\n\0".as_ptr() as *const c_char);
    -EIO
}

unsafe fn snd_ali_codec_poke(codec: *mut snd_ali, secondary: c_int, reg: u16, val: u16) {
    let mut dwVal: c_uint;
    let port: c_uint;

    if reg >= 0x80 {
        dev_err((*(*codec).card).dev, b"ali_codec_poke: reg(%xh) invalid.\n\0".as_ptr() as *const c_char, reg as c_uint);
        return;
    }

    port = (*codec).chregs.regs.ac97write;

    if snd_ali_codec_ready(codec, port) < 0 {
        return;
    }
    if snd_ali_stimer_ready(codec) < 0 {
        return;
    }

    dwVal = (reg & 0xff) as c_uint;
    dwVal |= 0x8000 | ((val as c_uint) << 16);
    if secondary != 0 {
        dwVal |= 0x0080;
    }
    if (*codec).revision == ALI_5451_V02 {
        dwVal |= 0x0100;
    }

    snd_ali_5451_poke(codec, port, dwVal);
}

unsafe fn snd_ali_codec_peek(codec: *mut snd_ali, secondary: c_int, reg: u16) -> u16 {
    let mut dwVal: c_uint;
    let port: c_uint;

    if reg >= 0x80 {
        dev_err((*(*codec).card).dev, b"ali_codec_peek: reg(%xh) invalid.\n\0".as_ptr() as *const c_char, reg as c_uint);
        return !0u16;
    }

    port = (*codec).chregs.regs.ac97read;

    if snd_ali_codec_ready(codec, port) < 0 {
        return !0u16;
    }
    if snd_ali_stimer_ready(codec) < 0 {
        return !0u16;
    }

    dwVal = (reg & 0xff) as c_uint;
    dwVal |= 0x8000; /* bit 15*/
    if secondary != 0 {
        dwVal |= 0x0080;
    }

    snd_ali_5451_poke(codec, port, dwVal);

    if snd_ali_stimer_ready(codec) < 0 {
        return !0u16;
    }
    if snd_ali_codec_ready(codec, port) < 0 {
        return !0u16;
    }

    ((snd_ali_5451_peek(codec, port) & 0xffff0000) >> 16) as u16
}

unsafe extern "C" fn snd_ali_codec_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let codec: *mut snd_ali = (*ac97).private_data as *mut snd_ali;

    dev_dbg((*(*codec).card).dev, b"codec_write: reg=%xh data=%xh.\n\0".as_ptr() as *const c_char, reg as c_uint, val as c_uint);
    if reg == AC97_GPIO_STATUS as u16 {
        outl(((val as c_uint) << ALI_AC97_GPIO_DATA_SHIFT) | ALI_AC97_GPIO_ENABLE, ALI_REG(codec, ALI_AC97_GPIO));
        return;
    }
    snd_ali_codec_poke(codec, (*ac97).num, reg, val);
}

unsafe extern "C" fn snd_ali_codec_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let codec: *mut snd_ali = (*ac97).private_data as *mut snd_ali;

    dev_dbg((*(*codec).card).dev, b"codec_read reg=%xh.\n\0".as_ptr() as *const c_char, reg as c_uint);
    snd_ali_codec_peek(codec, (*ac97).num, reg)
}

/*
 *	AC97 Reset
 */

unsafe fn snd_ali_reset_5451(codec: *mut snd_ali) -> c_int {
    let mut pci_dev: *mut pci_dev;
    let mut wCount: u16;
    let mut wReg: u16;
    let mut dwVal: c_uint = 0;

    pci_dev = (*codec).pci_m1533;
    if !pci_dev.is_null() {
        pci_read_config_dword(pci_dev, 0x7c, &mut dwVal);
        pci_write_config_dword(pci_dev, 0x7c, dwVal | 0x08000000);
        mdelay(5);
        pci_read_config_dword(pci_dev, 0x7c, &mut dwVal);
        pci_write_config_dword(pci_dev, 0x7c, dwVal & 0xf7ffffff);
        mdelay(5);
    }

    pci_dev = (*codec).pci;
    pci_read_config_dword(pci_dev, 0x44, &mut dwVal);
    pci_write_config_dword(pci_dev, 0x44, dwVal | 0x000c0000);
    udelay(500);
    pci_read_config_dword(pci_dev, 0x44, &mut dwVal);
    pci_write_config_dword(pci_dev, 0x44, dwVal & 0xfffbffff);
    mdelay(5);

    wCount = 200;
    while {
        let old = wCount;
        wCount = wCount.wrapping_sub(1);
        old != 0
    } {
        wReg = snd_ali_codec_peek(codec, 0, AC97_POWERDOWN as u16);
        if (wReg & 0x000f) == 0x000f {
            return 0;
        }
        mdelay(5);
    }

    /* non-fatal if you have a non PM capable codec */
    /* dev_warn(codec->card->dev, "ali5451: reset time out\n"); */
    0
}

/*
 *  ALI 5451 Controller
 */

unsafe fn snd_ali_enable_special_channel(codec: *mut snd_ali, channel: c_uint) {
    let mut dwVal: c_ulong;
    dwVal = inl(ALI_REG(codec, ALI_GLOBAL_CONTROL)) as c_ulong;
    dwVal |= 1u64.wrapping_shl(channel & 0x0000001f) as c_ulong;
    outl(dwVal as c_uint, ALI_REG(codec, ALI_GLOBAL_CONTROL));
}

unsafe fn snd_ali_disable_special_channel(codec: *mut snd_ali, channel: c_uint) {
    let mut dwVal: c_ulong;
    dwVal = inl(ALI_REG(codec, ALI_GLOBAL_CONTROL)) as c_ulong;
    dwVal &= !(1u64.wrapping_shl(channel & 0x0000001f) as c_ulong);
    outl(dwVal as c_uint, ALI_REG(codec, ALI_GLOBAL_CONTROL));
}

unsafe fn snd_ali_enable_address_interrupt(codec: *mut snd_ali) {
    let mut gc: c_uint;
    gc = inl(ALI_REG(codec, ALI_GC_CIR));
    gc |= ENDLP_IE;
    gc |= MIDLP_IE;
    outl(gc, ALI_REG(codec, ALI_GC_CIR));
}

unsafe fn snd_ali_disable_address_interrupt(codec: *mut snd_ali) {
    let mut gc: c_uint;
    gc = inl(ALI_REG(codec, ALI_GC_CIR));
    gc &= !ENDLP_IE;
    gc &= !MIDLP_IE;
    outl(gc, ALI_REG(codec, ALI_GC_CIR));
}

unsafe fn snd_ali_disable_voice_irq(codec: *mut snd_ali, channel: c_uint) {
    let mask: c_uint;
    let pchregs: *mut snd_ali_channel_control = &mut (*codec).chregs;

    dev_dbg((*(*codec).card).dev, b"disable_voice_irq channel=%d\n\0".as_ptr() as *const c_char, channel);

    mask = 1u32 << (channel & 0x1f);
    (*pchregs).data.ainten = inl(ALI_REG(codec, (*pchregs).regs.ainten));
    (*pchregs).data.ainten &= !mask;
    outl((*pchregs).data.ainten, ALI_REG(codec, (*pchregs).regs.ainten));
}

unsafe fn snd_ali_alloc_pcm_channel(codec: *mut snd_ali, channel: c_int) -> c_int {
    let idx: c_uint = (channel as c_uint) & 0x1f;

    if (*codec).synth.chcnt >= ALI_CHANNELS as c_uint {
        dev_err((*(*codec).card).dev, b"ali_alloc_pcm_channel: no free channels.\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if ((*codec).synth.chmap & (1 << idx)) == 0 {
        (*codec).synth.chmap |= 1 << idx;
        (*codec).synth.chcnt = (*codec).synth.chcnt.wrapping_add(1);
        dev_dbg((*(*codec).card).dev, b"alloc_pcm_channel no. %d.\n\0".as_ptr() as *const c_char, idx);
        return idx as c_int;
    }
    -1
}

unsafe fn snd_ali_find_free_channel(codec: *mut snd_ali, rec: c_int) -> c_int {
    let mut idx: c_int;
    let mut result: c_int = -1;

    dev_dbg((*(*codec).card).dev, b"find_free_channel: for %s\n\0".as_ptr() as *const c_char, if rec != 0 { b"rec\0".as_ptr() } else { b"pcm\0".as_ptr() } as *const c_char);

    /* recording */
    if rec != 0 {
        if ali_get(codec, ALI_SPDIF_SUPPORT) && (inl(ALI_REG(codec, ALI_GLOBAL_CONTROL)) & ALI_SPDIF_IN_SUPPORT) != 0 {
            idx = ALI_SPDIF_IN_CHANNEL as c_int;
        } else {
            idx = ALI_PCM_IN_CHANNEL as c_int;
        }

        result = snd_ali_alloc_pcm_channel(codec, idx);
        if result >= 0 {
            return result;
        } else {
            dev_err((*(*codec).card).dev, b"ali_find_free_channel: record channel is busy now.\n\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    /* playback... */
    if ali_get(codec, ALI_SPDIF_SUPPORT) && (inl(ALI_REG(codec, ALI_GLOBAL_CONTROL)) & ALI_SPDIF_OUT_CH_ENABLE) != 0 {
        idx = ALI_SPDIF_OUT_CHANNEL as c_int;
        result = snd_ali_alloc_pcm_channel(codec, idx);
        if result >= 0 {
            return result;
        } else {
            dev_err((*(*codec).card).dev, b"ali_find_free_channel: S/PDIF out channel is in busy now.\n\0".as_ptr() as *const c_char);
        }
    }

    idx = 0;
    while idx < ALI_CHANNELS as c_int {
        result = snd_ali_alloc_pcm_channel(codec, idx);
        if result >= 0 {
            return result;
        }
        idx += 1;
    }
    dev_err((*(*codec).card).dev, b"ali_find_free_channel: no free channels.\n\0".as_ptr() as *const c_char);
    -1
}

unsafe fn snd_ali_free_channel_pcm(codec: *mut snd_ali, channel: c_int) {
    let idx: c_uint = (channel as c_uint) & 0x0000001f;

    dev_dbg((*(*codec).card).dev, b"free_channel_pcm channel=%d\n\0".as_ptr() as *const c_char, channel);

    if channel < 0 || channel >= ALI_CHANNELS as c_int {
        return;
    }

    if ((*codec).synth.chmap & (1 << idx)) == 0 {
        dev_err((*(*codec).card).dev, b"ali_free_channel_pcm: channel %d is not in use.\n\0".as_ptr() as *const c_char, channel);
        return;
    } else {
        (*codec).synth.chmap &= !(1 << idx);
        (*codec).synth.chcnt = (*codec).synth.chcnt.wrapping_sub(1);
    }
}

unsafe fn snd_ali_stop_voice(codec: *mut snd_ali, channel: c_uint) {
    let mask: c_uint = 1 << (channel & 0x1f);
    dev_dbg((*(*codec).card).dev, b"stop_voice: channel=%d\n\0".as_ptr() as *const c_char, channel);
    outl(mask, ALI_REG(codec, (*codec).chregs.regs.stop));
}

/*
 *    S/PDIF Part
 */

unsafe fn snd_ali_delay(codec: *mut snd_ali, interval: c_int) {
    let begintimer: c_ulong;
    let mut currenttimer: c_ulong;

    begintimer = inl(ALI_REG(codec, ALI_STIMER)) as c_ulong;
    currenttimer = inl(ALI_REG(codec, ALI_STIMER)) as c_ulong;

    while currenttimer < begintimer.wrapping_add(interval as c_ulong) {
        if snd_ali_stimer_ready(codec) < 0 {
            break;
        }
        currenttimer = inl(ALI_REG(codec, ALI_STIMER)) as c_ulong;
        cpu_relax();
    }
}

unsafe fn snd_ali_detect_spdif_rate(codec: *mut snd_ali) {
    let mut wval: u16;
    let mut count: u16 = 0;
    let mut bval: u8;
    let mut R1: u8 = 0;
    let mut R2: u8 = 0;

    bval = inb(ALI_REG(codec, ALI_SPDIF_CTRL + 1));
    bval |= 0x1F;
    outb(bval as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL + 1));

    while (R1 < 0x0b || R1 > 0x0e) && R1 != 0x12 && count <= 50000 {
        count = count.wrapping_add(1);
        snd_ali_delay(codec, 6);
        bval = inb(ALI_REG(codec, ALI_SPDIF_CTRL + 1));
        R1 = bval & 0x1F;
    }

    if count > 50000 {
        dev_err((*(*codec).card).dev, b"ali_detect_spdif_rate: timeout!\n\0".as_ptr() as *const c_char);
        return;
    }

    count = 0;
    while count <= 50000 {
        snd_ali_delay(codec, 6);
        bval = inb(ALI_REG(codec, ALI_SPDIF_CTRL + 1));
        R2 = bval & 0x1F;
        if R2 != R1 {
            R1 = R2;
        } else {
            break;
        }
        count = count.wrapping_add(1);
    }

    if count > 50000 {
        dev_err((*(*codec).card).dev, b"ali_detect_spdif_rate: timeout!\n\0".as_ptr() as *const c_char);
        return;
    }

    if R2 >= 0x0b && R2 <= 0x0e {
        wval = inw(ALI_REG(codec, ALI_SPDIF_CTRL + 2));
        wval &= 0xe0f0;
        wval |= (0x09 << 8) | 0x05;
        outw(wval as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL + 2));

        bval = inb(ALI_REG(codec, ALI_SPDIF_CS + 3)) & 0xf0;
        outb((bval | 0x02) as c_uint, ALI_REG(codec, ALI_SPDIF_CS + 3));
    } else if R2 == 0x12 {
        wval = inw(ALI_REG(codec, ALI_SPDIF_CTRL + 2));
        wval &= 0xe0f0;
        wval |= (0x0e << 8) | 0x08;
        outw(wval as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL + 2));

        bval = inb(ALI_REG(codec, ALI_SPDIF_CS + 3)) & 0xf0;
        outb((bval | 0x03) as c_uint, ALI_REG(codec, ALI_SPDIF_CS + 3));
    }
}

unsafe fn snd_ali_get_spdif_in_rate(codec: *mut snd_ali) -> c_uint {
    let dwRate: u32;
    let mut bval: u8;

    bval = inb(ALI_REG(codec, ALI_SPDIF_CTRL));
    bval &= 0x7f;
    bval |= 0x40;
    outb(bval as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL));

    snd_ali_detect_spdif_rate(codec);

    bval = inb(ALI_REG(codec, ALI_SPDIF_CS + 3));
    bval &= 0x0f;

    match bval {
        0 => dwRate = 44100,
        1 => dwRate = 48000,
        2 => dwRate = 32000,
        _ => dwRate = 0,
    }

    dwRate
}

unsafe fn snd_ali_enable_spdif_in(codec: *mut snd_ali) {
    let mut dwVal: c_uint;
    dwVal = inl(ALI_REG(codec, ALI_GLOBAL_CONTROL));
    dwVal |= ALI_SPDIF_IN_SUPPORT;
    outl(dwVal, ALI_REG(codec, ALI_GLOBAL_CONTROL));

    dwVal = inb(ALI_REG(codec, ALI_SPDIF_CTRL)) as c_uint;
    dwVal |= 0x02;
    outb(dwVal, ALI_REG(codec, ALI_SPDIF_CTRL));

    snd_ali_enable_special_channel(codec, ALI_SPDIF_IN_CHANNEL);
}

unsafe fn snd_ali_disable_spdif_in(codec: *mut snd_ali) {
    let mut dwVal: c_uint;
    dwVal = inl(ALI_REG(codec, ALI_GLOBAL_CONTROL));
    dwVal &= !ALI_SPDIF_IN_SUPPORT;
    outl(dwVal, ALI_REG(codec, ALI_GLOBAL_CONTROL));
    snd_ali_disable_special_channel(codec, ALI_SPDIF_IN_CHANNEL);
}

unsafe fn snd_ali_set_spdif_out_rate(codec: *mut snd_ali, rate: c_uint) {
    let mut bVal: u8;
    let dwRate: c_uint;

    match rate {
        32000 => dwRate = 0x300,
        48000 => dwRate = 0x200,
        _ => dwRate = 0,
    }

    bVal = inb(ALI_REG(codec, ALI_SPDIF_CTRL));
    bVal &= !(1 << 6) as u8;

    bVal |= 0x80; /* select right */
    outb(bVal as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL));
    outb(dwRate | 0x20, ALI_REG(codec, ALI_SPDIF_CS + 2));

    bVal &= !0x80; /* select left */
    outb(bVal as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL));
    outw(rate | 0x10, ALI_REG(codec, ALI_SPDIF_CS + 2));
}

unsafe fn snd_ali_enable_spdif_out(codec: *mut snd_ali) {
    let mut wVal: u16;
    let mut bVal: u8 = 0;
    let pci_dev: *mut pci_dev;

    pci_dev = (*codec).pci_m1533;
    if pci_dev.is_null() {
        return;
    }
    pci_read_config_byte(pci_dev, 0x61, &mut bVal);
    bVal |= 0x40;
    pci_write_config_byte(pci_dev, 0x61, bVal);
    pci_read_config_byte(pci_dev, 0x7d, &mut bVal);
    bVal |= 0x01;
    pci_write_config_byte(pci_dev, 0x7d, bVal);

    pci_read_config_byte(pci_dev, 0x7e, &mut bVal);
    bVal &= !0x20;
    bVal |= 0x10;
    pci_write_config_byte(pci_dev, 0x7e, bVal);

    bVal = inb(ALI_REG(codec, ALI_SCTRL));
    outb((bVal as c_uint) | ALI_SPDIF_OUT_ENABLE, ALI_REG(codec, ALI_SCTRL));

    bVal = inb(ALI_REG(codec, ALI_SPDIF_CTRL));
    outb((bVal as c_uint) & ALI_SPDIF_OUT_CH_STATUS, ALI_REG(codec, ALI_SPDIF_CTRL));

    wVal = inw(ALI_REG(codec, ALI_GLOBAL_CONTROL));
    wVal |= ALI_SPDIF_OUT_SEL_PCM as u16;
    outw(wVal as c_uint, ALI_REG(codec, ALI_GLOBAL_CONTROL));
    snd_ali_disable_special_channel(codec, ALI_SPDIF_OUT_CHANNEL);
}

unsafe fn snd_ali_enable_spdif_chnout(codec: *mut snd_ali) {
    let mut wVal: u16;
    wVal = inw(ALI_REG(codec, ALI_GLOBAL_CONTROL));
    wVal &= !(ALI_SPDIF_OUT_SEL_PCM as u16);
    outw(wVal as c_uint, ALI_REG(codec, ALI_GLOBAL_CONTROL));
    /*
        wVal = inw(ALI_REG(codec, ALI_SPDIF_CS));
        if (flag & ALI_SPDIF_OUT_NON_PCM)
            wVal |= 0x0002;
        else
            wVal &= (~0x0002);
        outw(wVal, ALI_REG(codec, ALI_SPDIF_CS));
    */
    snd_ali_enable_special_channel(codec, ALI_SPDIF_OUT_CHANNEL);
}

unsafe fn snd_ali_disable_spdif_chnout(codec: *mut snd_ali) {
    let mut wVal: u16;
    wVal = inw(ALI_REG(codec, ALI_GLOBAL_CONTROL));
    wVal |= ALI_SPDIF_OUT_SEL_PCM as u16;
    outw(wVal as c_uint, ALI_REG(codec, ALI_GLOBAL_CONTROL));
    snd_ali_enable_special_channel(codec, ALI_SPDIF_OUT_CHANNEL);
}

unsafe fn snd_ali_disable_spdif_out(codec: *mut snd_ali) {
    let bVal: u8;
    bVal = inb(ALI_REG(codec, ALI_SCTRL));
    outb((bVal as c_uint) & !ALI_SPDIF_OUT_ENABLE, ALI_REG(codec, ALI_SCTRL));
    snd_ali_disable_spdif_chnout(codec);
}

unsafe fn snd_ali_update_ptr(codec: *mut snd_ali, channel: c_int) {
    let pvoice: *mut snd_ali_voice;
    let pchregs: *mut snd_ali_channel_control;
    let old: c_uint;
    let mask: c_uint;

    pchregs = &mut (*codec).chregs;

    /* check if interrupt occurred for channel */
    old = (*pchregs).data.aint;
    mask = 1u32 << ((channel as c_uint) & 0x1f);

    if (old & mask) == 0 {
        return;
    }

    pvoice = &mut (*codec).synth.voices[channel as usize];

    udelay(100);
    spin_lock(&mut (*codec).reg_lock);

    if voice_get(pvoice, VOICE_PCM) && !(*pvoice).substream.is_null() {
        /* pcm interrupt */
        if voice_get(pvoice, VOICE_RUNNING) {
            dev_dbg((*(*codec).card).dev, b"update_ptr: cso=%4.4x cspf=%d.\n\0".as_ptr() as *const c_char, inw(ALI_REG(codec, ALI_CSO_ALPHA_FMS + 2)) as c_uint, (inl(ALI_REG(codec, ALI_CSPF)) & mask) == mask);
            spin_unlock(&mut (*codec).reg_lock);
            snd_pcm_period_elapsed((*pvoice).substream);
            spin_lock(&mut (*codec).reg_lock);
        } else {
            snd_ali_stop_voice(codec, channel as c_uint);
            snd_ali_disable_voice_irq(codec, channel as c_uint);
        }
    } else if voice_get(&(*codec).synth.voices[channel as usize], VOICE_SYNTH) {
        /* synth interrupt */
    } else if voice_get(&(*codec).synth.voices[channel as usize], VOICE_MIDI) {
        /* midi interrupt */
    } else {
        /* unknown interrupt */
        snd_ali_stop_voice(codec, channel as c_uint);
        snd_ali_disable_voice_irq(codec, channel as c_uint);
    }
    spin_unlock(&mut (*codec).reg_lock);
    outl(mask, ALI_REG(codec, (*pchregs).regs.aint));
    (*pchregs).data.aint = old & !mask;
}

unsafe extern "C" fn snd_ali_card_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let codec: *mut snd_ali = dev_id as *mut snd_ali;
    let mut channel: c_int;
    let audio_int: c_uint;
    let pchregs: *mut snd_ali_channel_control;

    if codec.is_null() || !ali_get(codec, ALI_HW_INITIALIZED) {
        return IRQ_NONE;
    }

    audio_int = inl(ALI_REG(codec, ALI_MISCINT));
    if audio_int == 0 {
        return IRQ_NONE;
    }

    pchregs = &mut (*codec).chregs;
    if (audio_int & ADDRESS_IRQ) != 0 {
        /* get interrupt status for all channels */
        (*pchregs).data.aint = inl(ALI_REG(codec, (*pchregs).regs.aint));
        channel = 0;
        while channel < ALI_CHANNELS as c_int {
            snd_ali_update_ptr(codec, channel);
            channel += 1;
        }
    }
    outl(TARGET_REACHED | MIXER_OVERFLOW | MIXER_UNDERFLOW, ALI_REG(codec, ALI_MISCINT));

    IRQ_HANDLED
}

unsafe fn snd_ali_alloc_voice(codec: *mut snd_ali, type_: c_int, rec: c_int, channel: c_int) -> *mut snd_ali_voice {
    let pvoice: *mut snd_ali_voice;
    let idx: c_int;

    dev_dbg((*(*codec).card).dev, b"alloc_voice: type=%d rec=%d\n\0".as_ptr() as *const c_char, type_, rec);

    spin_lock_irq(&mut (*codec).voice_alloc);
    if type_ == SNDRV_ALI_VOICE_TYPE_PCM {
        idx = if channel > 0 { snd_ali_alloc_pcm_channel(codec, channel) } else { snd_ali_find_free_channel(codec, rec) };
        if idx < 0 {
            spin_unlock_irq(&mut (*codec).voice_alloc);
            dev_err((*(*codec).card).dev, b"ali_alloc_voice: err.\n\0".as_ptr() as *const c_char);
            return ptr::null_mut();
        }
        pvoice = &mut (*codec).synth.voices[idx as usize];
        (*pvoice).codec = codec;
        voice_set(pvoice, VOICE_USE, true);
        voice_set(pvoice, VOICE_PCM, true);
        voice_set(pvoice, VOICE_MODE, rec != 0);
        spin_unlock_irq(&mut (*codec).voice_alloc);
        return pvoice;
    }
    spin_unlock_irq(&mut (*codec).voice_alloc);
    ptr::null_mut()
}

unsafe fn snd_ali_free_voice(codec: *mut snd_ali, pvoice: *mut snd_ali_voice) {
    let private_free: Option<unsafe extern "C" fn(*mut c_void)>;
    let private_data: *mut c_void;

    dev_dbg((*(*codec).card).dev, b"free_voice: channel=%d\n\0".as_ptr() as *const c_char, (*pvoice).number);
    if !voice_get(pvoice, VOICE_USE) {
        return;
    }
    snd_ali_clear_voices(codec, (*pvoice).number, (*pvoice).number);
    spin_lock_irq(&mut (*codec).voice_alloc);
    private_free = (*pvoice).private_free;
    private_data = (*pvoice).private_data;
    (*pvoice).private_free = None;
    (*pvoice).private_data = ptr::null_mut();
    if voice_get(pvoice, VOICE_PCM) {
        snd_ali_free_channel_pcm(codec, (*pvoice).number as c_int);
    }
    voice_set(pvoice, VOICE_USE, false);
    voice_set(pvoice, VOICE_PCM, false);
    voice_set(pvoice, VOICE_SYNTH, false);
    (*pvoice).substream = ptr::null_mut();
    spin_unlock_irq(&mut (*codec).voice_alloc);
    if let Some(f) = private_free {
        f(private_data);
    }
}

unsafe fn snd_ali_clear_voices(codec: *mut snd_ali, v_min: c_uint, v_max: c_uint) {
    let mut i: c_uint = v_min;
    while i <= v_max {
        snd_ali_stop_voice(codec, i);
        snd_ali_disable_voice_irq(codec, i);
        i = i.wrapping_add(1);
    }
}

unsafe fn snd_ali_write_voice_regs(
    codec: *mut snd_ali,
    Channel: c_uint,
    LBA: c_uint,
    CSO: c_uint,
    ESO: c_uint,
    DELTA: c_uint,
    ALPHA_FMS: c_uint,
    GVSEL: c_uint,
    PAN: c_uint,
    VOL: c_uint,
    CTRL: c_uint,
    EC: c_uint,
) {
    let mut ctlcmds: [c_uint; 4] = [0; 4];

    outb(Channel & 0x001f, ALI_REG(codec, ALI_GC_CIR));

    ctlcmds[0] = (CSO << 16) | (ALPHA_FMS & 0x0000ffff);
    ctlcmds[1] = LBA;
    ctlcmds[2] = (ESO << 16) | (DELTA & 0x0ffff);
    ctlcmds[3] = (GVSEL << 31)
        | ((PAN & 0x0000007f) << 24)
        | ((VOL & 0x000000ff) << 16)
        | ((CTRL & 0x0000000f) << 12)
        | (EC & 0x00000fff);

    outb(Channel, ALI_REG(codec, ALI_GC_CIR));

    outl(ctlcmds[0], ALI_REG(codec, ALI_CSO_ALPHA_FMS));
    outl(ctlcmds[1], ALI_REG(codec, ALI_LBA));
    outl(ctlcmds[2], ALI_REG(codec, ALI_ESO_DELTA));
    outl(ctlcmds[3], ALI_REG(codec, ALI_GVSEL_PAN_VOC_CTRL_EC));

    outl(0x30000000, ALI_REG(codec, ALI_EBUF1)); /* Still Mode */
    outl(0x30000000, ALI_REG(codec, ALI_EBUF2)); /* Still Mode */
}

unsafe fn snd_ali_convert_rate(mut rate: c_uint, rec: c_int) -> c_uint {
    let delta: c_uint;

    if rate < 4000 {
        rate = 4000;
    }
    if rate > 48000 {
        rate = 48000;
    }

    if rec != 0 {
        if rate == 44100 {
            delta = 0x116a;
        } else if rate == 8000 {
            delta = 0x6000;
        } else if rate == 48000 {
            delta = 0x1000;
        } else {
            delta = ((48000 << 12) / rate) & 0x0000ffff;
        }
    } else {
        if rate == 44100 {
            delta = 0xeb3;
        } else if rate == 8000 {
            delta = 0x2ab;
        } else if rate == 48000 {
            delta = 0x1000;
        } else {
            delta = (((rate << 12) + rate) / 48000) & 0x0000ffff;
        }
    }

    delta
}

unsafe fn snd_ali_control_mode(substream: *mut snd_pcm_substream) -> c_uint {
    let mut CTRL: c_uint;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    /* set ctrl mode
       CTRL default: 8-bit (unsigned) mono, loop mode enabled
     */
    CTRL = 0x00000001;
    if snd_pcm_format_width((*runtime).format) == 16 {
        CTRL |= 0x00000008; /* 16-bit data */
    }
    if snd_pcm_format_unsigned((*runtime).format) == 0 {
        CTRL |= 0x00000002; /* signed data */
    }
    if (*runtime).channels > 1 {
        CTRL |= 0x00000004; /* stereo data */
    }
    CTRL
}

/*
 *  PCM part
 */

unsafe extern "C" fn snd_ali_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let mut s: *mut snd_pcm_substream;
    let mut what: c_uint;
    let mut whati: c_uint;
    let mut pvoice: *mut snd_ali_voice;
    let mut evoice: *mut snd_ali_voice;
    let mut val: c_uint;
    let do_start: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => do_start = 1,
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => do_start = 0,
        _ => return -EINVAL,
    }

    what = 0;
    whati = 0;
    s = substream;
    loop {
        if snd_pcm_substream_chip(s) as *mut snd_ali == codec {
            pvoice = (*(*s).runtime).private_data as *mut snd_ali_voice;
            evoice = (*pvoice).extra;
            what |= 1 << ((*pvoice).number & 0x1f);
            if evoice.is_null() {
                whati |= 1 << ((*pvoice).number & 0x1f);
            } else {
                whati |= 1 << ((*evoice).number & 0x1f);
                what |= 1 << ((*evoice).number & 0x1f);
            }
            if do_start != 0 {
                voice_set(pvoice, VOICE_RUNNING, true);
                if !evoice.is_null() {
                    voice_set(evoice, VOICE_RUNNING, true);
                }
            } else {
                voice_set(pvoice, VOICE_RUNNING, false);
                if !evoice.is_null() {
                    voice_set(evoice, VOICE_RUNNING, false);
                }
            }
            snd_pcm_trigger_done(s, substream);
        }
        s = snd_pcm_group_next_entry(s, substream);
        if s == substream {
            break;
        }
    }
    spin_lock(&mut (*codec).reg_lock);
    if do_start == 0 {
        outl(what, ALI_REG(codec, ALI_STOP));
    }
    val = inl(ALI_REG(codec, ALI_AINTEN));
    if do_start != 0 {
        val |= whati;
    } else {
        val &= !whati;
    }
    outl(val, ALI_REG(codec, ALI_AINTEN));
    if do_start != 0 {
        outl(what, ALI_REG(codec, ALI_START));
    }
    spin_unlock(&mut (*codec).reg_lock);
    dev_dbg((*(*codec).card).dev, b"trigger: what=%xh whati=%xh\n\0".as_ptr() as *const c_char, what, whati);

    0
}

unsafe extern "C" fn snd_ali_playback_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let mut evoice: *mut snd_ali_voice = (*pvoice).extra;

    /* voice management */

    if params_buffer_size(hw_params) / 2 != params_period_size(hw_params) {
        if evoice.is_null() {
            evoice = snd_ali_alloc_voice(codec, SNDRV_ALI_VOICE_TYPE_PCM, 0, -1);
            if evoice.is_null() {
                return -ENOMEM;
            }
            (*pvoice).extra = evoice;
            (*evoice).substream = substream;
        }
    } else if !evoice.is_null() {
        snd_ali_free_voice(codec, evoice);
        (*pvoice).extra = ptr::null_mut();
    }

    0
}

unsafe extern "C" fn snd_ali_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let evoice: *mut snd_ali_voice = if !pvoice.is_null() { (*pvoice).extra } else { ptr::null_mut() };

    if !evoice.is_null() {
        snd_ali_free_voice(codec, evoice);
        (*pvoice).extra = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn snd_ali_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let evoice: *mut snd_ali_voice = (*pvoice).extra;
    let LBA: c_uint;
    let mut Delta: c_uint;
    let mut ESO: c_uint;
    let CTRL: c_uint;
    let GVSEL: c_uint;
    let PAN: c_uint;
    let VOL: c_uint;
    let EC: c_uint;

    dev_dbg((*(*codec).card).dev, b"playback_prepare ...\n\0".as_ptr() as *const c_char);

    spin_lock_irq(&mut (*codec).reg_lock);
    Delta = snd_ali_convert_rate((*runtime).rate, 0);

    if (*pvoice).number == ALI_SPDIF_IN_CHANNEL || (*pvoice).number == ALI_PCM_IN_CHANNEL {
        snd_ali_disable_special_channel(codec, (*pvoice).number);
    } else if ali_get(codec, ALI_SPDIF_SUPPORT)
        && (inl(ALI_REG(codec, ALI_GLOBAL_CONTROL)) & ALI_SPDIF_OUT_CH_ENABLE) != 0
        && (*pvoice).number == ALI_SPDIF_OUT_CHANNEL
    {
        snd_ali_set_spdif_out_rate(codec, (*runtime).rate);
        Delta = 0x1000;
    }

    LBA = (*runtime).dma_addr as c_uint;
    (*pvoice).count = (*runtime).period_size as c_int;
    (*pvoice).eso = (*runtime).buffer_size as c_int;
    dev_dbg((*(*codec).card).dev, b"playback_prepare: eso=%xh count=%xh\n\0".as_ptr() as *const c_char, (*pvoice).eso, (*pvoice).count);

    ESO = ((*pvoice).eso - 1) as c_uint;
    CTRL = snd_ali_control_mode(substream);
    GVSEL = 1;
    PAN = 0;
    VOL = 0;
    EC = 0;
    dev_dbg((*(*codec).card).dev, b"playback_prepare:\n\0".as_ptr() as *const c_char);
    dev_dbg((*(*codec).card).dev, b"ch=%d, Rate=%d Delta=%xh,GVSEL=%xh,PAN=%xh,CTRL=%xh\n\0".as_ptr() as *const c_char, (*pvoice).number, (*runtime).rate, Delta, GVSEL, PAN, CTRL);
    snd_ali_write_voice_regs(codec, (*pvoice).number, LBA, 0, ESO, Delta, 0, GVSEL, PAN, VOL, CTRL, EC);
    if !evoice.is_null() {
        (*evoice).count = (*pvoice).count;
        (*evoice).eso = (*pvoice).count << 1;
        ESO = ((*evoice).eso - 1) as c_uint;
        snd_ali_write_voice_regs(codec, (*evoice).number, LBA, 0, ESO, Delta, 0, GVSEL, 0x7f, 0x3ff, CTRL, EC);
    }
    spin_unlock_irq(&mut (*codec).reg_lock);
    0
}

unsafe extern "C" fn snd_ali_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let LBA: c_uint;
    let mut Delta: c_uint;
    let ESO: c_uint;
    let CTRL: c_uint;
    let GVSEL: c_uint;
    let PAN: c_uint;
    let VOL: c_uint;
    let EC: c_uint;
    let mut bValue: u8;

    spin_lock_irq(&mut (*codec).reg_lock);
    dev_dbg((*(*codec).card).dev, b"ali_prepare...\n\0".as_ptr() as *const c_char);
    snd_ali_enable_special_channel(codec, (*pvoice).number);

    Delta = if (*pvoice).number == ALI_MODEM_IN_CHANNEL || (*pvoice).number == ALI_MODEM_OUT_CHANNEL {
        0x1000
    } else {
        snd_ali_convert_rate((*runtime).rate, if voice_get(pvoice, VOICE_MODE) { 1 } else { 0 })
    };

    /* Prepare capture intr channel */
    if (*pvoice).number == ALI_SPDIF_IN_CHANNEL {
        let mut rate: c_uint;
        spin_unlock_irq(&mut (*codec).reg_lock);
        if (*codec).revision != ALI_5451_V02 {
            return -1;
        }
        rate = snd_ali_get_spdif_in_rate(codec);
        if rate == 0 {
            dev_warn((*(*codec).card).dev, b"ali_capture_prepare: spdif rate detect err!\n\0".as_ptr() as *const c_char);
            rate = 48000;
        }
        spin_lock_irq(&mut (*codec).reg_lock);
        bValue = inb(ALI_REG(codec, ALI_SPDIF_CTRL));
        if (bValue & 0x10) != 0 {
            outb(bValue as c_uint, ALI_REG(codec, ALI_SPDIF_CTRL));
            dev_warn((*(*codec).card).dev, b"clear SPDIF parity error flag.\n\0".as_ptr() as *const c_char);
        }

        if rate != 48000 {
            Delta = ((rate << 12) / (*runtime).rate) & 0x00ffff;
        }
    }

    (*pvoice).eso = (*runtime).buffer_size as c_int;
    (*pvoice).count = (*runtime).period_size as c_int;
    LBA = (*runtime).dma_addr as c_uint;
    ESO = ((*pvoice).eso - 1) as c_uint;
    CTRL = snd_ali_control_mode(substream);
    GVSEL = 0;
    PAN = 0x00;
    VOL = 0x00;
    EC = 0;

    snd_ali_write_voice_regs(codec, (*pvoice).number, LBA, 0, ESO, Delta, 0, GVSEL, PAN, VOL, CTRL, EC);

    spin_unlock_irq(&mut (*codec).reg_lock);
    0
}

unsafe extern "C" fn snd_ali_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let mut cso: c_uint;

    spin_lock(&mut (*codec).reg_lock);
    if !voice_get(pvoice, VOICE_RUNNING) {
        spin_unlock(&mut (*codec).reg_lock);
        return 0;
    }
    outb((*pvoice).number, ALI_REG(codec, ALI_GC_CIR));
    cso = inw(ALI_REG(codec, ALI_CSO_ALPHA_FMS + 2)) as c_uint;
    dev_dbg((*(*codec).card).dev, b"playback pointer returned cso=%xh.\n\0".as_ptr() as *const c_char, cso);
    cso %= (*runtime).buffer_size as c_uint;
    spin_unlock(&mut (*codec).reg_lock);
    cso as snd_pcm_uframes_t
}

unsafe extern "C" fn snd_ali_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    let mut cso: c_uint;

    spin_lock(&mut (*codec).reg_lock);
    if !voice_get(pvoice, VOICE_RUNNING) {
        spin_unlock(&mut (*codec).reg_lock);
        return 0;
    }
    outb((*pvoice).number, ALI_REG(codec, ALI_GC_CIR));
    cso = inw(ALI_REG(codec, ALI_CSO_ALPHA_FMS + 2)) as c_uint;
    cso %= (*runtime).buffer_size as c_uint;
    spin_unlock(&mut (*codec).reg_lock);
    cso as snd_pcm_uframes_t
}

static snd_ali_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 256 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

/*
 *  Capture support device description
 */

static snd_ali_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn snd_ali_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    let pvoice: *mut snd_ali_voice = (*runtime).private_data as *mut snd_ali_voice;
    if !pvoice.is_null() {
        snd_ali_free_voice((*pvoice).codec, pvoice);
    }
}

unsafe fn snd_ali_open(substream: *mut snd_pcm_substream, rec: c_int, channel: c_int, phw: *const snd_pcm_hardware) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pvoice: *mut snd_ali_voice;

    pvoice = snd_ali_alloc_voice(codec, SNDRV_ALI_VOICE_TYPE_PCM, rec, channel);
    if pvoice.is_null() {
        return -EAGAIN;
    }

    (*pvoice).substream = substream;
    (*runtime).private_data = pvoice as *mut c_void;
    (*runtime).private_free = Some(snd_ali_pcm_free_substream);
    (*runtime).hw = *phw;
    snd_pcm_set_sync(substream);
    snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 0, 64 * 1024);
    0
}

unsafe extern "C" fn snd_ali_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    snd_ali_open(substream, 0, -1, &snd_ali_playback)
}

unsafe extern "C" fn snd_ali_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    snd_ali_open(substream, 1, -1, &snd_ali_capture)
}

unsafe extern "C" fn snd_ali_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_ali_close(substream: *mut snd_pcm_substream) -> c_int {
    let codec: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let pvoice: *mut snd_ali_voice = (*(*substream).runtime).private_data as *mut snd_ali_voice;
    snd_ali_disable_special_channel(codec, (*pvoice).number);
    0
}

static snd_ali_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ali_playback_open),
    close: Some(snd_ali_playback_close),
    hw_params: Some(snd_ali_playback_hw_params),
    hw_free: Some(snd_ali_playback_hw_free),
    prepare: Some(snd_ali_playback_prepare),
    trigger: Some(snd_ali_trigger),
    pointer: Some(snd_ali_playback_pointer),
};

static snd_ali_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ali_capture_open),
    close: Some(snd_ali_close),
    hw_params: None,
    hw_free: None,
    prepare: Some(snd_ali_prepare),
    trigger: Some(snd_ali_trigger),
    pointer: Some(snd_ali_pointer),
};

/*
 * Modem PCM
 */

unsafe extern "C" fn snd_ali_modem_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip: *mut snd_ali = snd_pcm_substream_chip(substream) as *mut snd_ali;
    let modem_num: c_uint = (*chip).num_of_codecs - 1;
    snd_ac97_write((*chip).ac97[modem_num as usize], AC97_LINE1_RATE, params_rate(hw_params));
    snd_ac97_write((*chip).ac97[modem_num as usize], AC97_LINE1_LEVEL, 0);
    0
}

static snd_ali_modem: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_KNOT | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
    rate_min: 8000,
    rate_max: 16000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 256 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 256 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe fn snd_ali_modem_open(substream: *mut snd_pcm_substream, rec: c_int, channel: c_int) -> c_int {
    static rates: [c_uint; 4] = [8000, 9600, 12000, 16000];
    static hw_constraint_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 4,
        list: rates.as_ptr(),
        mask: 0,
    };
    let err: c_int = snd_ali_open(substream, rec, channel, &snd_ali_modem);

    if err != 0 {
        return err;
    }
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &hw_constraint_rates)
}

unsafe extern "C" fn snd_ali_modem_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    snd_ali_modem_open(substream, 0, ALI_MODEM_OUT_CHANNEL as c_int)
}

unsafe extern "C" fn snd_ali_modem_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    snd_ali_modem_open(substream, 1, ALI_MODEM_IN_CHANNEL as c_int)
}

static snd_ali_modem_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ali_modem_playback_open),
    close: Some(snd_ali_close),
    hw_params: Some(snd_ali_modem_hw_params),
    hw_free: None,
    prepare: Some(snd_ali_prepare),
    trigger: Some(snd_ali_trigger),
    pointer: Some(snd_ali_pointer),
};

static snd_ali_modem_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ali_modem_capture_open),
    close: Some(snd_ali_close),
    hw_params: Some(snd_ali_modem_hw_params),
    hw_free: None,
    prepare: Some(snd_ali_prepare),
    trigger: Some(snd_ali_trigger),
    pointer: Some(snd_ali_pointer),
};

#[repr(C)]
pub struct ali_pcm_description {
    pub name: *mut c_char,
    pub playback_num: c_uint,
    pub capture_num: c_uint,
    pub playback_ops: *const snd_pcm_ops,
    pub capture_ops: *const snd_pcm_ops,
    pub class: u16,
}

unsafe extern "C" fn snd_ali_pcm_free(pcm: *mut snd_pcm) {
    let codec: *mut snd_ali = (*pcm).private_data as *mut snd_ali;
    (*codec).pcm[(*pcm).device as usize] = ptr::null_mut();
}

unsafe fn snd_ali_pcm(codec: *mut snd_ali, device: c_int, desc: *mut ali_pcm_description) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new((*codec).card, (*desc).name, device, (*desc).playback_num, (*desc).capture_num, &mut pcm);
    if err < 0 {
        dev_err((*(*codec).card).dev, b"snd_ali_pcm: err called snd_pcm_new.\n\0".as_ptr() as *const c_char);
        return err;
    }
    (*pcm).private_data = codec as *mut c_void;
    (*pcm).private_free = Some(snd_ali_pcm_free);
    if !(*desc).playback_ops.is_null() {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, (*desc).playback_ops);
    }
    if !(*desc).capture_ops.is_null() {
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, (*desc).capture_ops);
    }

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*codec).pci).dev, 64 * 1024, 128 * 1024);

    (*pcm).info_flags = 0;
    (*pcm).dev_class = (*desc).class;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    strscpy((*pcm).name.as_mut_ptr(), (*desc).name);
    (*codec).pcm[0] = pcm;
    0
}

static mut ali_pcms: [ali_pcm_description; 2] = [
    ali_pcm_description {
        name: b"ALI 5451\0".as_ptr() as *mut c_char,
        playback_num: ALI_CHANNELS as c_uint,
        capture_num: 1,
        playback_ops: &snd_ali_playback_ops,
        capture_ops: &snd_ali_capture_ops,
        class: 0,
    },
    ali_pcm_description {
        name: b"ALI 5451 modem\0".as_ptr() as *mut c_char,
        playback_num: 1,
        capture_num: 1,
        playback_ops: &snd_ali_modem_playback_ops,
        capture_ops: &snd_ali_modem_capture_ops,
        class: SNDRV_PCM_CLASS_MODEM,
    },
];

unsafe fn snd_ali_build_pcms(codec: *mut snd_ali) -> c_int {
    let mut i: c_int = 0;
    let mut err: c_int;
    while i < (*codec).num_of_codecs as c_int && (i as usize) < ali_pcms.len() {
        err = snd_ali_pcm(codec, i, &mut ali_pcms[i as usize]);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

/* ALI5451_SPDIF initializer macro translated in snd_ali5451_mixer_spdif below. */
const snd_ali5451_spdif_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> = Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn snd_ali5451_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut snd_ali = snd_kcontrol_chip(kcontrol) as *mut snd_ali;
    let mut spdif_enable: c_uint;

    spdif_enable = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };

    spin_lock_irq(&mut (*codec).reg_lock);
    match (*kcontrol).private_value {
        0 => spdif_enable = if ((*codec).spdif_mask & 0x02) != 0 { 1 } else { 0 },
        1 => spdif_enable = if ((*codec).spdif_mask & 0x02) != 0 && ((*codec).spdif_mask & 0x04) != 0 { 1 } else { 0 },
        2 => spdif_enable = if ((*codec).spdif_mask & 0x01) != 0 { 1 } else { 0 },
        _ => {}
    }
    spin_unlock_irq(&mut (*codec).reg_lock);
    (*ucontrol).value.integer.value[0] = spdif_enable as c_long;
    0
}

unsafe extern "C" fn snd_ali5451_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec: *mut snd_ali = snd_kcontrol_chip(kcontrol) as *mut snd_ali;
    let mut change: c_uint = 0;
    let spdif_enable: c_uint;

    spdif_enable = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };

    spin_lock_irq(&mut (*codec).reg_lock);
    match (*kcontrol).private_value {
        0 => {
            change = if ((*codec).spdif_mask & 0x02) != 0 { 1 } else { 0 };
            change ^= spdif_enable;
            if change != 0 {
                if spdif_enable != 0 {
                    (*codec).spdif_mask |= 0x02;
                    snd_ali_enable_spdif_out(codec);
                } else {
                    (*codec).spdif_mask &= !0x02;
                    (*codec).spdif_mask &= !0x04;
                    snd_ali_disable_spdif_out(codec);
                }
            }
        }
        1 => {
            change = if ((*codec).spdif_mask & 0x04) != 0 { 1 } else { 0 };
            change ^= spdif_enable;
            if change != 0 && ((*codec).spdif_mask & 0x02) != 0 {
                if spdif_enable != 0 {
                    (*codec).spdif_mask |= 0x04;
                    snd_ali_enable_spdif_chnout(codec);
                } else {
                    (*codec).spdif_mask &= !0x04;
                    snd_ali_disable_spdif_chnout(codec);
                }
            }
        }
        2 => {
            change = if ((*codec).spdif_mask & 0x01) != 0 { 1 } else { 0 };
            change ^= spdif_enable;
            if change != 0 {
                if spdif_enable != 0 {
                    (*codec).spdif_mask |= 0x01;
                    snd_ali_enable_spdif_in(codec);
                } else {
                    (*codec).spdif_mask &= !0x01;
                    snd_ali_disable_spdif_in(codec);
                }
            }
        }
        _ => {}
    }
    spin_unlock_irq(&mut (*codec).reg_lock);

    change as c_int
}

static snd_ali5451_mixer_spdif: [snd_kcontrol_new; 3] = [
    /* spdif aplayback switch */
    /* FIXME: "IEC958 Playback Switch" may conflict with one on ac97_codec */
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SNDRV_CTL_NAME_IEC958_OUTPUT_NONE_SWITCH, index: 0, info: snd_ali5451_spdif_info, get: Some(snd_ali5451_spdif_get), put: Some(snd_ali5451_spdif_put), private_value: 0 },
    /* spdif out to spdif channel */
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SNDRV_CTL_NAME_IEC958_CHANNEL_OUTPUT_NONE_SWITCH, index: 0, info: snd_ali5451_spdif_info, get: Some(snd_ali5451_spdif_get), put: Some(snd_ali5451_spdif_put), private_value: 1 },
    /* spdif in from spdif channel */
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: SNDRV_CTL_NAME_IEC958_CAPTURE_SWITCH, index: 0, info: snd_ali5451_spdif_info, get: Some(snd_ali5451_spdif_get), put: Some(snd_ali5451_spdif_put), private_value: 2 },
];

unsafe fn snd_ali_mixer(codec: *mut snd_ali) -> c_int {
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut idx: c_uint;
    let mut i: c_int;
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_ali_codec_write),
        read: Some(snd_ali_codec_read),
    };

    err = snd_ac97_bus((*codec).card, 0, &ops, codec as *mut c_void, &mut (*codec).ac97_bus);
    if err < 0 {
        return err;
    }

    ac97.private_data = codec as *mut c_void;

    i = 0;
    while i < (*codec).num_of_codecs as c_int {
        ac97.num = i;
        err = snd_ac97_mixer((*codec).ac97_bus, &mut ac97, &mut (*codec).ac97[i as usize]);
        if err < 0 {
            dev_err((*(*codec).card).dev, b"ali mixer %d creating error.\n\0".as_ptr() as *const c_char, i);
            if i == 0 {
                return err;
            }
            (*codec).num_of_codecs = 1;
            break;
        }
        i += 1;
    }

    if ali_get(codec, ALI_SPDIF_SUPPORT) {
        idx = 0;
        while (idx as usize) < snd_ali5451_mixer_spdif.len() {
            err = snd_ctl_add((*codec).card, snd_ctl_new1(&snd_ali5451_mixer_spdif[idx as usize], codec as *mut c_void));
            if err < 0 {
                return err;
            }
            idx += 1;
        }
    }
    0
}

unsafe extern "C" fn ali_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_ali = (*card).private_data as *mut snd_ali;
    let im: *mut snd_ali_image = &mut (*chip).image;
    let mut i: c_int;
    let mut j: c_int;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    i = 0;
    while i < (*chip).num_of_codecs as c_int {
        snd_ac97_suspend((*chip).ac97[i as usize]);
        i += 1;
    }

    spin_lock_irq(&mut (*chip).reg_lock);
    (*im).regs[(ALI_MISCINT >> 2) as usize] = inl(ALI_REG(chip, ALI_MISCINT));
    /* im->regs[ALI_START >> 2] = inl(ALI_REG(chip, ALI_START)); */
    (*im).regs[(ALI_STOP >> 2) as usize] = inl(ALI_REG(chip, ALI_STOP));
    outl(0, ALI_REG(chip, ALI_MISCINT));

    i = 0;
    while i < ALI_GLOBAL_REGS as c_int {
        if i * 4 == ALI_MISCINT as c_int || i * 4 == ALI_STOP as c_int {
            i += 1;
            continue;
        }
        (*im).regs[i as usize] = inl(ALI_REG(chip, (i * 4) as c_uint));
        i += 1;
    }

    i = 0;
    while i < ALI_CHANNELS as c_int {
        outb(i as c_uint, ALI_REG(chip, ALI_GC_CIR));
        j = 0;
        while j < ALI_CHANNEL_REGS as c_int {
            (*im).channel_regs[i as usize][j as usize] = inl(ALI_REG(chip, (j * 4 + 0xe0) as c_uint));
            j += 1;
        }
        i += 1;
    }

    outl(0xffffffff, ALI_REG(chip, ALI_STOP));
    spin_unlock_irq(&mut (*chip).reg_lock);
    0
}

unsafe extern "C" fn ali_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let chip: *mut snd_ali = (*card).private_data as *mut snd_ali;
    let im: *mut snd_ali_image = &mut (*chip).image;
    let mut i: c_int;
    let mut j: c_int;

    spin_lock_irq(&mut (*chip).reg_lock);
    i = 0;
    while i < ALI_CHANNELS as c_int {
        outb(i as c_uint, ALI_REG(chip, ALI_GC_CIR));
        j = 0;
        while j < ALI_CHANNEL_REGS as c_int {
            outl((*im).channel_regs[i as usize][j as usize], ALI_REG(chip, (j * 4 + 0xe0) as c_uint));
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < ALI_GLOBAL_REGS as c_int {
        if i * 4 == ALI_MISCINT as c_int || i * 4 == ALI_STOP as c_int || i * 4 == ALI_START as c_int {
            i += 1;
            continue;
        }
        outl((*im).regs[i as usize], ALI_REG(chip, (i * 4) as c_uint));
        i += 1;
    }

    outl((*im).regs[(ALI_START >> 2) as usize], ALI_REG(chip, ALI_START));
    outl((*im).regs[(ALI_MISCINT >> 2) as usize], ALI_REG(chip, ALI_MISCINT));
    spin_unlock_irq(&mut (*chip).reg_lock);

    i = 0;
    while i < (*chip).num_of_codecs as c_int {
        snd_ac97_resume((*chip).ac97[i as usize]);
        i += 1;
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static ali_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(ali_suspend),
    resume: Some(ali_resume),
};

unsafe extern "C" fn snd_ali_free(card: *mut snd_card) {
    let codec: *mut snd_ali = (*card).private_data as *mut snd_ali;

    if ali_get(codec, ALI_HW_INITIALIZED) {
        snd_ali_disable_address_interrupt(codec);
    }
    pci_dev_put((*codec).pci_m1533);
    pci_dev_put((*codec).pci_m7101);
}

unsafe fn snd_ali_chip_init(codec: *mut snd_ali) -> c_int {
    let mut legacy: c_uint = 0;
    let mut temp: u8 = 0;
    let mut pci_dev: *mut pci_dev;

    dev_dbg((*(*codec).card).dev, b"chip initializing ...\n\0".as_ptr() as *const c_char);

    if snd_ali_reset_5451(codec) != 0 {
        dev_err((*(*codec).card).dev, b"ali_chip_init: reset 5451 error.\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if (*codec).revision == ALI_5451_V02 {
        pci_dev = (*codec).pci_m1533;
        pci_read_config_byte(pci_dev, 0x59, &mut temp);
        temp |= 0x80;
        pci_write_config_byte(pci_dev, 0x59, temp);

        pci_dev = (*codec).pci_m7101;
        pci_read_config_byte(pci_dev, 0xb8, &mut temp);
        temp |= 0x20;
        pci_write_config_byte(pci_dev, 0xB8, temp);
    }

    pci_read_config_dword((*codec).pci, 0x44, &mut legacy);
    legacy &= 0xff00ff00;
    legacy |= 0x000800aa;
    pci_write_config_dword((*codec).pci, 0x44, legacy);

    outl(0x80000001, ALI_REG(codec, ALI_GLOBAL_CONTROL));
    outl(0x00000000, ALI_REG(codec, ALI_AINTEN));
    outl(0xffffffff, ALI_REG(codec, ALI_AINT));
    outl(0x00000000, ALI_REG(codec, ALI_VOLUME));
    outb(0x10, ALI_REG(codec, ALI_MPUR2));

    (*codec).ac97_ext_id = snd_ali_codec_peek(codec, 0, AC97_EXTENDED_ID as u16);
    (*codec).ac97_ext_status = snd_ali_codec_peek(codec, 0, AC97_EXTENDED_STATUS as u16);
    if ali_get(codec, ALI_SPDIF_SUPPORT) {
        snd_ali_enable_spdif_out(codec);
        (*codec).spdif_mask = 0x00000002;
    }

    (*codec).num_of_codecs = 1;

    /* secondary codec - modem */
    if (inl(ALI_REG(codec, ALI_SCTRL)) & ALI_SCTRL_CODEC2_READY) != 0 {
        (*codec).num_of_codecs += 1;
        outl(inl(ALI_REG(codec, ALI_SCTRL)) | (ALI_SCTRL_LINE_IN2 | ALI_SCTRL_GPIO_IN2 | ALI_SCTRL_LINE_OUT_EN), ALI_REG(codec, ALI_SCTRL));
    }

    dev_dbg((*(*codec).card).dev, b"chip initialize succeed.\n\0".as_ptr() as *const c_char);
    0
}

/* proc for register dump */
unsafe extern "C" fn snd_ali_proc_read(entry: *mut snd_info_entry, buf: *mut snd_info_buffer) {
    let codec: *mut snd_ali = (*entry).private_data as *mut snd_ali;
    let mut i: c_int = 0;
    while i < 256 {
        snd_iprintf(buf, b"%02x: %08x\n\0".as_ptr() as *const c_char, i, inl(ALI_REG(codec, i as c_uint)));
        i += 4;
    }
}

unsafe fn snd_ali_proc_init(codec: *mut snd_ali) {
    snd_card_ro_proc_new((*codec).card, b"ali5451\0".as_ptr() as *const c_char, codec as *mut c_void, Some(snd_ali_proc_read));
}

unsafe fn snd_ali_resources(codec: *mut snd_ali) -> c_int {
    let err: c_int;

    dev_dbg((*(*codec).card).dev, b"resources allocation ...\n\0".as_ptr() as *const c_char);
    err = pcim_request_all_regions((*codec).pci, b"ALI 5451\0".as_ptr() as *const c_char);
    if err < 0 {
        return err;
    }
    (*codec).port = pci_resource_start((*codec).pci, 0);

    if devm_request_irq(&mut (*(*codec).pci).dev, (*(*codec).pci).irq, Some(snd_ali_card_interrupt), IRQF_SHARED, KBUILD_MODNAME, codec as *mut c_void) != 0 {
        dev_err((*(*codec).card).dev, b"Unable to request irq.\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }
    (*codec).irq = (*(*codec).pci).irq;
    (*(*codec).card).sync_irq = (*codec).irq;
    dev_dbg((*(*codec).card).dev, b"resources allocated.\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn snd_ali_create(card: *mut snd_card, pci: *mut pci_dev, mut pcm_streams: c_int, spdif_support: c_int) -> c_int {
    let codec: *mut snd_ali = (*card).private_data as *mut snd_ali;
    let mut i: c_int;
    let mut err: c_int;
    let mut cmdw: u16 = 0;

    dev_dbg((*card).dev, b"creating ...\n\0".as_ptr() as *const c_char);

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(31)) != 0 {
        dev_err((*card).dev, b"architecture does not support 31bit PCI busmaster DMA\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }

    spin_lock_init(&mut (*codec).reg_lock);
    spin_lock_init(&mut (*codec).voice_alloc);

    (*codec).card = card;
    (*codec).pci = pci;
    (*codec).irq = -1;
    (*codec).revision = (*pci).revision;
    ali_set(codec, ALI_SPDIF_SUPPORT, spdif_support != 0);

    if pcm_streams < 1 {
        pcm_streams = 1;
    }
    if pcm_streams > 32 {
        pcm_streams = 32;
    }

    pci_set_master(pci);
    pci_read_config_word(pci, PCI_COMMAND, &mut cmdw);
    if (cmdw & PCI_COMMAND_IO as u16) != PCI_COMMAND_IO as u16 {
        cmdw |= PCI_COMMAND_IO as u16;
        pci_write_config_word(pci, PCI_COMMAND, cmdw);
    }

    if snd_ali_resources(codec) != 0 {
        return -EBUSY;
    }
    (*card).private_free = Some(snd_ali_free);

    (*codec).synth.chmap = 0;
    (*codec).synth.chcnt = 0;
    (*codec).spdif_mask = 0;
    (*codec).synth.synthcount = 0;

    if (*codec).revision == ALI_5451_V02 {
        (*codec).chregs.regs.ac97read = ALI_AC97_WRITE;
    } else {
        (*codec).chregs.regs.ac97read = ALI_AC97_READ;
    }
    (*codec).chregs.regs.ac97write = ALI_AC97_WRITE;
    (*codec).chregs.regs.start = ALI_START;
    (*codec).chregs.regs.stop = ALI_STOP;
    (*codec).chregs.regs.aint = ALI_AINT;
    (*codec).chregs.regs.ainten = ALI_AINTEN;

    (*codec).chregs.data.start = 0x00;
    (*codec).chregs.data.stop = 0x00;
    (*codec).chregs.data.aint = 0x00;
    (*codec).chregs.data.ainten = 0x00;

    /* M1533: southbridge */
    (*codec).pci_m1533 = pci_get_device(0x10b9, 0x1533, ptr::null_mut());
    if (*codec).pci_m1533.is_null() {
        dev_err((*card).dev, b"cannot find ALi 1533 chip.\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }
    /* M7101: power management */
    (*codec).pci_m7101 = pci_get_device(0x10b9, 0x7101, ptr::null_mut());
    if (*codec).pci_m7101.is_null() && (*codec).revision == ALI_5451_V02 {
        dev_err((*card).dev, b"cannot find ALi 7101 chip.\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    /* initialise synth voices*/
    i = 0;
    while i < ALI_CHANNELS as c_int {
        (*codec).synth.voices[i as usize].number = i as c_uint;
        i += 1;
    }

    err = snd_ali_chip_init(codec);
    if err < 0 {
        dev_err((*card).dev, b"ali create: chip init error.\n\0".as_ptr() as *const c_char);
        return err;
    }

    snd_ali_enable_address_interrupt(codec);
    ali_set(codec, ALI_HW_INITIALIZED, true);
    0
}

unsafe fn __snd_ali_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let codec: *mut snd_ali;
    let mut err: c_int;

    dev_dbg(&mut (*pci).dev, b"probe ...\n\0".as_ptr() as *const c_char);

    err = snd_devm_card_new(&mut (*pci).dev, index, id, THIS_MODULE, size_of::<snd_ali>(), &mut card);
    if err < 0 {
        return err;
    }
    codec = (*card).private_data as *mut snd_ali;

    err = snd_ali_create(card, pci, pcm_channels, if spdif { 1 } else { 0 });
    if err < 0 {
        return err;
    }

    dev_dbg(&mut (*pci).dev, b"mixer building ...\n\0".as_ptr() as *const c_char);
    err = snd_ali_mixer(codec);
    if err < 0 {
        return err;
    }

    dev_dbg(&mut (*pci).dev, b"pcm building ...\n\0".as_ptr() as *const c_char);
    err = snd_ali_build_pcms(codec);
    if err < 0 {
        return err;
    }

    snd_ali_proc_init(codec);

    strscpy((*card).driver.as_mut_ptr(), b"ALI5451\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"ALI 5451\0".as_ptr() as *const c_char);

    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*codec).port, (*codec).irq);

    dev_dbg(&mut (*pci).dev, b"register card.\n\0".as_ptr() as *const c_char);
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_ali_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_ali_probe(pci, pci_id))
}

static mut ali5451_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: unsafe { snd_ali_ids.as_ptr() },
    probe: Some(snd_ali_probe),
    driver: device_driver {
        pm: &ali_pm,
    },
};

/* module_pci_driver(ali5451_driver); */

extern "C" {
    static mut jiffies: c_ulong;
    static SNDRV_DEFAULT_IDX1: c_int;
    static SNDRV_DEFAULT_STR1: *const c_char;
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    static PCI_VENDOR_ID_AL: c_uint;
    static PCI_DEVICE_ID_AL_M5451: c_uint;
    static AC97_GPIO_STATUS: c_uint;
    static AC97_POWERDOWN: c_uint;
    static AC97_EXTENDED_ID: c_uint;
    static AC97_EXTENDED_STATUS: c_uint;
    static AC97_LINE1_RATE: c_uint;
    static AC97_LINE1_LEVEL: c_uint;

    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_SYNC_START: c_uint;
    static SNDRV_PCM_FMTBIT_U8: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static SNDRV_PCM_FMTBIT_U16_LE: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_KNOT: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SNDRV_PCM_SUBCLASS_GENERIC_MIX: u16;
    static SNDRV_PCM_CLASS_MODEM: u16;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static SNDRV_CTL_NAME_IEC958_OUTPUT_NONE_SWITCH: *const c_char;
    static SNDRV_CTL_NAME_IEC958_CHANNEL_OUTPUT_NONE_SWITCH: *const c_char;
    static SNDRV_CTL_NAME_IEC958_CAPTURE_SWITCH: *const c_char;
    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static IRQ_NONE: c_int;
    static IRQ_HANDLED: c_int;
    static IRQF_SHARED: c_ulong;
    static EIO: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static EAGAIN: c_int;
    static EBUSY: c_int;
    static ENXIO: c_int;
    static ENODEV: c_int;
    static PCI_COMMAND: c_int;
    static PCI_COMMAND_IO: c_int;
}

type c_long = isize;

#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: c_int, pub revision: u8 }
#[repr(C)] pub struct pci_device_id { pub vendor: c_uint, pub device: c_uint }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub id_table: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub driver: device_driver }
#[repr(C)] pub struct device_driver { pub pm: *const dev_pm_ops }
#[repr(C)] pub struct dev_pm_ops { pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int> }
#[repr(C)] pub struct snd_card { pub dev: *mut device, pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub sync_irq: c_int, pub driver: [c_char; 16], pub shortname: [c_char; 32], pub longname: [c_char; 80] }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>, pub device: c_int, pub info_flags: c_uint, pub dev_class: u16, pub dev_subclass: u16, pub name: [c_char; 80] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>, pub hw: snd_pcm_hardware, pub format: c_int, pub channels: c_uint, pub rate: c_uint, pub dma_addr: c_ulong, pub period_size: c_ulong, pub buffer_size: c_ulong }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_pcm_hardware { pub info: c_uint, pub formats: c_uint, pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub buffer_bytes_max: c_uint, pub period_bytes_min: c_uint, pub period_bytes_max: c_uint, pub periods_min: c_uint, pub periods_max: c_uint, pub fifo_size: c_uint }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct snd_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t> }
#[repr(C)] pub struct snd_ac97 { pub private_data: *mut c_void, pub num: c_int }
#[repr(C)] pub struct snd_ac97_bus { _priv: [u8; 0] }
#[repr(C)] pub struct snd_ac97_bus_ops { pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>, pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16> }
#[repr(C)] pub struct snd_ac97_template { pub private_data: *mut c_void, pub num: c_int }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_uint }
#[repr(C)] pub struct snd_ctl_elem_info { _priv: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 1] }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_int, pub name: *const c_char, pub index: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_uint }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }

extern "C" {
    fn inl(port: c_ulong) -> c_uint;
    fn outl(val: c_uint, port: c_ulong);
    fn inw(port: c_ulong) -> u16;
    fn outw(val: c_uint, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn outb(val: c_uint, port: c_ulong);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn time_after_eq(a: c_ulong, b: c_ulong) -> c_int;
    fn schedule_timeout_uninterruptible(timeout: c_ulong);
    fn mdelay(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn cpu_relax();
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pci_read_config_dword(dev: *mut pci_dev, where_: c_int, val: *mut c_uint);
    fn pci_write_config_dword(dev: *mut pci_dev, where_: c_int, val: c_uint);
    fn pci_read_config_byte(dev: *mut pci_dev, where_: c_int, val: *mut u8);
    fn pci_write_config_byte(dev: *mut pci_dev, where_: c_int, val: u8);
    fn pci_read_config_word(dev: *mut pci_dev, where_: c_int, val: *mut u16);
    fn pci_write_config_word(dev: *mut pci_dev, where_: c_int, val: u16);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut c_void;
    fn snd_pcm_group_next_entry(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream) -> *mut snd_pcm_substream;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_unsigned(format: c_int) -> c_int;
    fn params_buffer_size(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_uint, min: c_ulong, max: c_ulong) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_ac97_write(ac97: *mut snd_ac97, reg: c_uint, val: c_uint);
    fn snd_pcm_new(card: *mut snd_card, id: *mut c_char, device: c_int, playback_count: c_uint, capture_count: c_uint, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, dev: *mut device, size: c_ulong, max: c_ulong);
    fn strscpy(dst: *mut c_char, src: *const c_char);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn pci_dev_put(dev: *mut pci_dev);
    fn pcim_request_all_regions(pdev: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(dev: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pcim_enable_device(dev: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pci_set_master(dev: *mut pci_dev);
    fn pci_get_device(vendor: c_uint, device: c_uint, from: *mut pci_dev) -> *mut pci_dev;
    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0
    } else {
        (1u64 << n) - 1
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
