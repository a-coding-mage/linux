// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/sound/oss/dmasound/dmasound_paula.c
 *
 *  Amiga `Paula' DMA Sound Driver
 *
 *  See linux/sound/oss/dmasound/dmasound_core.c for copyright and credits
 *  prior to 28/01/2001
 *
 *  28/01/2001 [0.1] Iain Sandoe
 *                 - added versioning
 *                 - put in and populated the hardware_afmts field.
 *             [0.2] - put in SNDCTL_DSP_GETCAPS value.
 *             [0.3] - put in constraint on state buffer usage.
 *             [0.4] - put in default hard/soft settings
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type u_char = u8;
type u_short = u16;
type u_int = c_uint;
type u_long = c_ulong;
type size_t = usize;
type ssize_t = isize;
type gfp_t = c_uint;
type irqreturn_t = c_int;

const DMASOUND_PAULA_REVISION: c_int = 0;
const DMASOUND_PAULA_EDITION: c_int = 4;

/* The original file includes Linux, m68k Amiga, and local dmasound headers.
 * Their constants, structs, extern globals, and helper macros are referenced
 * here in Rust form as dependencies supplied by the surrounding kernel tree.
 */

unsafe extern "C" {
    static mut amiga_audio_min_period: u_short;
    static mut amiga_audio_period: u_short;
    static mut amiga_colorclock: c_int;
    static mut dmasound: DMASOUND;
    static mut write_sq: WRITE_SQ;
    static mut amiga_custom: CUSTOM;
    static mut ciaa: CIAA;
    static mut mach_heartbeat: Option<unsafe extern "C" fn(c_int)>;
    static dmasound_ulaw2dma8: [u_char; 256];
    static dmasound_alaw2dma8: [u_char; 256];

    fn amiga_chip_alloc(size: c_long, name: *const c_char) -> *mut c_void;
    fn amiga_chip_free(obj: *mut c_void);
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn dmasound_init() -> c_int;
    fn dmasound_deinit();
    fn dmasound_set_volume(volume: c_int) -> c_int;
    fn dmasound_set_treble(treble: c_int) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn IOCTL_OUT(arg: u_long, value: c_int) -> c_int;
    fn IOCTL_IN(arg: u_long, data: *mut c_int) -> c_int;
    fn WAKE_UP(queue: *mut c_void);
    fn ZTWO_PADDR(ptr: *mut u_char) -> c_ulong;
}

#[repr(C)]
struct AUDIO {
    audlc: *mut u_short,
    audlen: u_short,
    audper: u_short,
    audvol: u_short,
}

#[repr(C)]
struct CUSTOM {
    aud: [AUDIO; 4],
    dmacon: u_short,
    intena: u_short,
}

#[repr(C)]
struct CIAA {
    pra: u_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SETTINGS {
    format: c_int,
    stereo: c_int,
    size: c_int,
    speed: c_int,
}

#[repr(C)]
struct TRANS {
    ct_ulaw: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_alaw: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s8: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u8: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s16be: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u16be: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s16le: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u16le: Option<unsafe extern "C" fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
}

#[repr(C)]
struct MACHINE {
    name: *const c_char,
    name2: *const c_char,
    owner: *mut c_void,
    dma_alloc: Option<unsafe extern "C" fn(c_uint, gfp_t) -> *mut c_void>,
    dma_free: Option<unsafe extern "C" fn(*mut c_void, c_uint)>,
    irqinit: Option<unsafe extern "C" fn() -> c_int>,
    irqcleanup: Option<unsafe extern "C" fn()>,
    init: Option<unsafe extern "C" fn()>,
    silence: Option<unsafe extern "C" fn()>,
    setFormat: Option<unsafe extern "C" fn(c_int) -> c_int>,
    setVolume: Option<unsafe extern "C" fn(c_int) -> c_int>,
    setTreble: Option<unsafe extern "C" fn(c_int) -> c_int>,
    play: Option<unsafe extern "C" fn()>,
    mixer_init: Option<unsafe extern "C" fn()>,
    mixer_ioctl: Option<unsafe extern "C" fn(u_int, u_long) -> c_int>,
    write_sq_setup: Option<unsafe extern "C" fn() -> c_int>,
    state_info: Option<unsafe extern "C" fn(*mut c_char, size_t) -> c_int>,
    min_dsp_speed: c_int,
    version: c_int,
    hardware_afmts: c_int,
    capabilities: c_int,
    default_hard: SETTINGS,
    default_soft: SETTINGS,
}

#[repr(C)]
struct DSP_STATE {
    format: c_int,
    size: c_int,
}

#[repr(C)]
struct DMASOUND {
    soft: SETTINGS,
    hard: SETTINGS,
    dsp: DSP_STATE,
    minDev: c_int,
    trans_write: *mut TRANS,
    volume_left: c_int,
    volume_right: c_int,
    treble: c_int,
    mach: MACHINE,
}

#[repr(C)]
struct WRITE_SQ {
    buffers: *mut *mut u_char,
    front: c_int,
    count: c_int,
    rear_size: u_long,
    block_size: u_long,
    max_count: c_int,
    active: c_int,
    syncing: c_int,
    sync_queue: *mut c_void,
    action_queue: *mut c_void,
}

#[repr(C)]
struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
}

const DMAF_AUD0: u_short = 0x0001;
const DMAF_AUD1: u_short = 0x0002;
const DMAF_AUD2: u_short = 0x0004;
const DMAF_AUD3: u_short = 0x0008;
const DMAF_SETCLR: u_short = 0x8000;
const DMAF_MASTER: u_short = 0x0200;
const AMI_AUDIO_OFF: u_short = DMAF_AUD0 | DMAF_AUD1 | DMAF_AUD2 | DMAF_AUD3;
const AMI_AUDIO_8: u_short = DMAF_SETCLR | DMAF_MASTER | DMAF_AUD0 | DMAF_AUD1;
const AMI_AUDIO_14: u_short = AMI_AUDIO_8 | DMAF_AUD2 | DMAF_AUD3;

const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_AMIGA_AUD0: c_int = 0;
const IF_AUD0: u_short = 0x0080;
const IF_SETCLR: u_short = 0x8000;
const AFMT_QUERY: c_int = 0;
const AFMT_MU_LAW: c_int = 0x00000001;
const AFMT_A_LAW: c_int = 0x00000002;
const AFMT_U8: c_int = 0x00000008;
const AFMT_S8: c_int = 0x00000040;
const AFMT_S16_LE: c_int = 0x00000010;
const AFMT_S16_BE: c_int = 0x00000020;
const AFMT_U16_LE: c_int = 0x00000080;
const AFMT_U16_BE: c_int = 0x00000100;
const SND_DEV_DSP: c_int = 3;
const SOUND_MIXER_READ_DEVMASK: u_int = 0;
const SOUND_MIXER_READ_RECMASK: u_int = 1;
const SOUND_MIXER_READ_STEREODEVS: u_int = 2;
const SOUND_MIXER_READ_VOLUME: u_int = 3;
const SOUND_MIXER_WRITE_VOLUME: u_int = 4;
const SOUND_MIXER_READ_TREBLE: u_int = 5;
const SOUND_MIXER_WRITE_TREBLE: u_int = 6;
const SOUND_MASK_VOLUME: c_int = 1 << 0;
const SOUND_MASK_TREBLE: c_int = 1 << 4;
const DSP_CAP_BATCH: c_int = 0x00001000;

static mut write_sq_block_size_half: c_int = 0;
static mut write_sq_block_size_quarter: c_int = 0;

unsafe fn copy_from_user(dst: *mut c_void, src: *const u_char, count: size_t) -> c_int {
    core::ptr::copy_nonoverlapping(src, dst as *mut u_char, count);
    0
}

unsafe fn get_user_u8(dst: *mut u_char, src: *const u_char) -> c_int {
    *dst = core::ptr::read(src);
    0
}

unsafe fn get_user_u16(dst: *mut u_short, src: *const u_short) -> c_int {
    *dst = core::ptr::read(src);
    0
}

fn le2be16(x: u_short) -> u_short {
    u_short::from_be(u_short::from_le(x))
}

unsafe extern "C" fn AmiSetTreble(treble: c_int) -> c_int {
    dmasound.treble = treble;
    if treble < 50 {
        ciaa.pra &= !0x02;
    } else {
        ciaa.pra |= 0x02;
    }
    treble
}

#[cfg(CONFIG_HEARTBEAT)]
static mut saved_heartbeat: Option<unsafe extern "C" fn(c_int)> = None;

#[cfg(CONFIG_HEARTBEAT)]
unsafe fn disable_heartbeat() {
    if mach_heartbeat.is_some() {
        saved_heartbeat = mach_heartbeat;
        mach_heartbeat = None;
    }
    AmiSetTreble(dmasound.treble);
}

#[cfg(CONFIG_HEARTBEAT)]
unsafe fn enable_heartbeat() {
    if saved_heartbeat.is_some() {
        mach_heartbeat = saved_heartbeat;
    }
}

#[cfg(not(CONFIG_HEARTBEAT))]
unsafe fn disable_heartbeat() {}

#[cfg(not(CONFIG_HEARTBEAT))]
unsafe fn enable_heartbeat() {}

unsafe extern "C" fn ami_ct_s8(
    mut userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
) -> ssize_t {
    let count: ssize_t;
    let used: ssize_t;

    if dmasound.soft.stereo == 0 {
        let p = frame.offset(*frameUsed) as *mut c_void;
        count = core::cmp::min(userCount, frameLeft as size_t) as ssize_t & !1;
        used = count;
        if copy_from_user(p, userPtr, count as size_t) != 0 {
            return -(EFAULT as ssize_t);
        }
    } else {
        let mut left = frame.offset(*frameUsed >> 1);
        let mut right = left.offset(write_sq_block_size_half as isize);
        count = ((core::cmp::min(userCount, frameLeft as size_t) as ssize_t) >> 1) & !1;
        used = count * 2;
        let mut c = count;
        while c > 0 {
            if get_user_u8(left, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            left = left.offset(1);
            userPtr = userPtr.offset(1);
            if get_user_u8(right, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            right = right.offset(1);
            userPtr = userPtr.offset(1);
            c -= 1;
        }
    }
    *frameUsed += used;
    used
}

unsafe fn ami_ct8_common(
    mut userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
    convsample: unsafe fn(u_char) -> u_char,
) -> ssize_t {
    let count: ssize_t;
    let used: ssize_t;

    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed);
        count = core::cmp::min(userCount, frameLeft as size_t) as ssize_t & !1;
        used = count;
        let mut c = count;
        while c > 0 {
            let mut data: u_char = 0;
            if get_user_u8(&mut data, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            *p = convsample(data);
            p = p.offset(1);
            c -= 1;
        }
    } else {
        let mut left = frame.offset(*frameUsed >> 1);
        let mut right = left.offset(write_sq_block_size_half as isize);
        count = ((core::cmp::min(userCount, frameLeft as size_t) as ssize_t) >> 1) & !1;
        used = count * 2;
        let mut c = count;
        while c > 0 {
            let mut data: u_char = 0;
            if get_user_u8(&mut data, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            *left = convsample(data);
            left = left.offset(1);
            if get_user_u8(&mut data, userPtr) != 0 {
                return -(EFAULT as ssize_t);
            }
            userPtr = userPtr.offset(1);
            *right = convsample(data);
            right = right.offset(1);
            c -= 1;
        }
    }
    *frameUsed += used;
    used
}

unsafe fn AMI_CT_ULAW(x: u_char) -> u_char {
    dmasound_ulaw2dma8[x as usize]
}

unsafe fn AMI_CT_ALAW(x: u_char) -> u_char {
    dmasound_alaw2dma8[x as usize]
}

unsafe fn AMI_CT_U8(x: u_char) -> u_char {
    x ^ 0x80
}

unsafe extern "C" fn ami_ct_ulaw(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct8_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_ULAW)
}

unsafe extern "C" fn ami_ct_alaw(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct8_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_ALAW)
}

unsafe extern "C" fn ami_ct_u8(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct8_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_U8)
}

unsafe fn ami_ct_16_common(
    userPtr: *const u_char,
    userCount: size_t,
    frame: *mut u_char,
    frameUsed: *mut ssize_t,
    frameLeft: ssize_t,
    convsample: unsafe fn(u_short) -> u_short,
) -> ssize_t {
    let mut ptr = userPtr as *const u_short;
    let count: ssize_t;
    let used: ssize_t;
    let mut data: u_short = 0;

    if dmasound.soft.stereo == 0 {
        let mut high = frame.offset(*frameUsed >> 1);
        let mut low = high.offset(write_sq_block_size_half as isize);
        count = ((core::cmp::min(userCount, frameLeft as size_t) as ssize_t) >> 1) & !1;
        used = count * 2;
        let mut c = count;
        while c > 0 {
            if get_user_u16(&mut data, ptr) != 0 {
                return -(EFAULT as ssize_t);
            }
            ptr = ptr.offset(1);
            data = convsample(data);
            *high = (data >> 8) as u_char;
            high = high.offset(1);
            *low = ((data >> 2) & 0x3f) as u_char;
            low = low.offset(1);
            c -= 1;
        }
    } else {
        let mut lefth = frame.offset(*frameUsed >> 2);
        let mut leftl = lefth.offset(write_sq_block_size_quarter as isize);
        let mut righth = lefth.offset(write_sq_block_size_half as isize);
        let mut rightl = righth.offset(write_sq_block_size_quarter as isize);
        count = ((core::cmp::min(userCount, frameLeft as size_t) as ssize_t) >> 2) & !1;
        used = count * 4;
        let mut c = count;
        while c > 0 {
            if get_user_u16(&mut data, ptr) != 0 {
                return -(EFAULT as ssize_t);
            }
            ptr = ptr.offset(1);
            data = convsample(data);
            *lefth = (data >> 8) as u_char;
            lefth = lefth.offset(1);
            *leftl = ((data >> 2) & 0x3f) as u_char;
            leftl = leftl.offset(1);
            if get_user_u16(&mut data, ptr) != 0 {
                return -(EFAULT as ssize_t);
            }
            ptr = ptr.offset(1);
            data = convsample(data);
            *righth = (data >> 8) as u_char;
            righth = righth.offset(1);
            *rightl = ((data >> 2) & 0x3f) as u_char;
            rightl = rightl.offset(1);
            c -= 1;
        }
    }
    *frameUsed += used;
    used
}

unsafe fn AMI_CT_S16BE(x: u_short) -> u_short { x }
unsafe fn AMI_CT_U16BE(x: u_short) -> u_short { x ^ 0x8000 }
unsafe fn AMI_CT_S16LE(x: u_short) -> u_short { le2be16(x) }
unsafe fn AMI_CT_U16LE(x: u_short) -> u_short { le2be16(x) ^ 0x8000 }

unsafe extern "C" fn ami_ct_s16be(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct_16_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_S16BE)
}
unsafe extern "C" fn ami_ct_u16be(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct_16_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_U16BE)
}
unsafe extern "C" fn ami_ct_s16le(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct_16_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_S16LE)
}
unsafe extern "C" fn ami_ct_u16le(userPtr: *const u_char, userCount: size_t, frame: *mut u_char, frameUsed: *mut ssize_t, frameLeft: ssize_t) -> ssize_t {
    ami_ct_16_common(userPtr, userCount, frame, frameUsed, frameLeft, AMI_CT_U16LE)
}

static mut transAmiga: TRANS = TRANS {
    ct_ulaw: Some(ami_ct_ulaw),
    ct_alaw: Some(ami_ct_alaw),
    ct_s8: Some(ami_ct_s8),
    ct_u8: Some(ami_ct_u8),
    ct_s16be: Some(ami_ct_s16be),
    ct_u16be: Some(ami_ct_u16be),
    ct_s16le: Some(ami_ct_s16le),
    ct_u16le: Some(ami_ct_u16le),
};

unsafe fn StopDMA() {
    amiga_custom.aud[1].audvol = 0;
    amiga_custom.aud[0].audvol = amiga_custom.aud[1].audvol;
    amiga_custom.aud[3].audvol = 0;
    amiga_custom.aud[2].audvol = amiga_custom.aud[3].audvol;
    amiga_custom.dmacon = AMI_AUDIO_OFF;
    enable_heartbeat();
}

unsafe extern "C" fn AmiAlloc(size: c_uint, _flags: gfp_t) -> *mut c_void {
    amiga_chip_alloc(size as c_long, c"dmasound [Paula]".as_ptr())
}

unsafe extern "C" fn AmiFree(obj: *mut c_void, _size: c_uint) {
    amiga_chip_free(obj);
}

unsafe extern "C" fn AmiIrqInit() -> c_int {
    /* turn off DMA for audio channels */
    StopDMA();

    /* Register interrupt handler. */
    if request_irq(
        IRQ_AMIGA_AUD0,
        AmiInterrupt,
        0,
        c"DMA sound".as_ptr(),
        AmiInterrupt as *mut c_void,
    ) != 0 {
        return 0;
    }
    1
}

#[cfg(MODULE)]
unsafe extern "C" fn AmiIrqCleanUp() {
    /* turn off DMA for audio channels */
    StopDMA();
    /* release the interrupt */
    free_irq(IRQ_AMIGA_AUD0, AmiInterrupt as *mut c_void);
}

#[cfg(not(MODULE))]
unsafe extern "C" fn AmiIrqCleanUp() {}

unsafe extern "C" fn AmiSilence() {
    /* turn off DMA for audio channels */
    StopDMA();
}

unsafe extern "C" fn AmiInit() {
    let mut period: c_int;

    AmiSilence();

    if dmasound.soft.speed != 0 {
        period = amiga_colorclock / dmasound.soft.speed - 1;
    } else {
        period = amiga_audio_min_period as c_int;
    }
    dmasound.hard = dmasound.soft;
    dmasound.trans_write = &mut transAmiga;

    if period < amiga_audio_min_period as c_int {
        /* we would need to squeeze the sound, but we won't do that */
        period = amiga_audio_min_period as c_int;
    } else if period > 65535 {
        period = 65535;
    }
    dmasound.hard.speed = amiga_colorclock / (period + 1);

    let mut i = 0;
    while i < 4 {
        amiga_custom.aud[i].audper = period as u_short;
        i += 1;
    }
    amiga_audio_period = period as u_short;
}

unsafe extern "C" fn AmiSetFormat(mut format: c_int) -> c_int {
    let size: c_int;

    /* Amiga sound DMA supports 8bit and 16bit (pseudo 14 bit) modes */
    match format {
        AFMT_QUERY => return dmasound.soft.format,
        AFMT_MU_LAW | AFMT_A_LAW | AFMT_U8 | AFMT_S8 => size = 8,
        AFMT_S16_BE | AFMT_U16_BE | AFMT_S16_LE | AFMT_U16_LE => size = 16,
        _ => {
            /* :-) */
            size = 8;
            format = AFMT_S8;
        }
    }

    dmasound.soft.format = format;
    dmasound.soft.size = size;
    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.format = format;
        dmasound.dsp.size = dmasound.soft.size;
    }
    AmiInit();

    format
}

fn VOLUME_VOXWARE_TO_AMI(v: c_int) -> c_int {
    if v < 0 {
        0
    } else if v > 100 {
        64
    } else {
        (v * 64) / 100
    }
}

fn VOLUME_AMI_TO_VOXWARE(v: c_int) -> c_int {
    v * 100 / 64
}

unsafe extern "C" fn AmiSetVolume(volume: c_int) -> c_int {
    dmasound.volume_left = VOLUME_VOXWARE_TO_AMI(volume & 0xff);
    amiga_custom.aud[0].audvol = dmasound.volume_left as u_short;
    dmasound.volume_right = VOLUME_VOXWARE_TO_AMI((volume & 0xff00) >> 8);
    amiga_custom.aud[1].audvol = dmasound.volume_right as u_short;
    if dmasound.hard.size == 16 {
        if dmasound.volume_left == 64 && dmasound.volume_right == 64 {
            amiga_custom.aud[2].audvol = 1;
            amiga_custom.aud[3].audvol = 1;
        } else {
            amiga_custom.aud[2].audvol = 0;
            amiga_custom.aud[3].audvol = 0;
        }
    }
    VOLUME_AMI_TO_VOXWARE(dmasound.volume_left)
        | (VOLUME_AMI_TO_VOXWARE(dmasound.volume_right) << 8)
}

const AMI_PLAY_LOADED: c_int = 1;
const AMI_PLAY_PLAYING: c_int = 2;
const AMI_PLAY_MASK: c_int = 3;

unsafe extern "C" fn AmiPlayNextFrame(index: c_int) {
    let start: *mut u_char;
    let ch0: *mut u_char;
    let ch1: *mut u_char;
    let ch2: *mut u_char;
    let ch3: *mut u_char;
    let mut size: u_long;

    /* used by AmiPlay() if all doubts whether there really is something
     * to be played are already wiped out.
     */
    start = *write_sq.buffers.offset(write_sq.front as isize);
    size = (if write_sq.count == index {
        write_sq.rear_size
    } else {
        write_sq.block_size
    }) >> 1;

    if dmasound.hard.stereo != 0 {
        ch0 = start;
        ch1 = start.offset(write_sq_block_size_half as isize);
        size >>= 1;
    } else {
        ch0 = start;
        ch1 = start;
    }

    disable_heartbeat();
    amiga_custom.aud[0].audvol = dmasound.volume_left as u_short;
    amiga_custom.aud[1].audvol = dmasound.volume_right as u_short;
    if dmasound.hard.size == 8 {
        amiga_custom.aud[0].audlc = ZTWO_PADDR(ch0) as *mut u_short;
        amiga_custom.aud[0].audlen = size as u_short;
        amiga_custom.aud[1].audlc = ZTWO_PADDR(ch1) as *mut u_short;
        amiga_custom.aud[1].audlen = size as u_short;
        amiga_custom.dmacon = AMI_AUDIO_8;
    } else {
        size >>= 1;
        amiga_custom.aud[0].audlc = ZTWO_PADDR(ch0) as *mut u_short;
        amiga_custom.aud[0].audlen = size as u_short;
        amiga_custom.aud[1].audlc = ZTWO_PADDR(ch1) as *mut u_short;
        amiga_custom.aud[1].audlen = size as u_short;
        if dmasound.volume_left == 64 && dmasound.volume_right == 64 {
            /* We can play pseudo 14-bit only with the maximum volume */
            ch3 = ch0.offset(write_sq_block_size_quarter as isize);
            ch2 = ch1.offset(write_sq_block_size_quarter as isize);
            amiga_custom.aud[2].audvol = 1; /* we are being affected by the beeps */
            amiga_custom.aud[3].audvol = 1; /* restoring volume here helps a bit */
            amiga_custom.aud[2].audlc = ZTWO_PADDR(ch2) as *mut u_short;
            amiga_custom.aud[2].audlen = size as u_short;
            amiga_custom.aud[3].audlc = ZTWO_PADDR(ch3) as *mut u_short;
            amiga_custom.aud[3].audlen = size as u_short;
            amiga_custom.dmacon = AMI_AUDIO_14;
        } else {
            amiga_custom.aud[2].audvol = 0;
            amiga_custom.aud[3].audvol = 0;
            amiga_custom.dmacon = AMI_AUDIO_8;
        }
    }
    write_sq.front = (write_sq.front + 1) % write_sq.max_count;
    write_sq.active |= AMI_PLAY_LOADED;
}

unsafe extern "C" fn AmiPlay() {
    let mut minframes: c_int = 1;

    amiga_custom.intena = IF_AUD0;

    if (write_sq.active & AMI_PLAY_LOADED) != 0 {
        /* There's already a frame loaded */
        amiga_custom.intena = IF_SETCLR | IF_AUD0;
        return;
    }

    if (write_sq.active & AMI_PLAY_PLAYING) != 0 {
        /* Increase threshold: frame 1 is already being played */
        minframes = 2;
    }

    if write_sq.count < minframes {
        /* Nothing to do */
        amiga_custom.intena = IF_SETCLR | IF_AUD0;
        return;
    }

    if write_sq.count <= minframes
        && write_sq.rear_size < write_sq.block_size
        && write_sq.syncing == 0
    {
        /* hmmm, the only existing frame is not
         * yet filled and we're not syncing?
         */
        amiga_custom.intena = IF_SETCLR | IF_AUD0;
        return;
    }

    AmiPlayNextFrame(minframes);

    amiga_custom.intena = IF_SETCLR | IF_AUD0;
}

unsafe extern "C" fn AmiInterrupt(_irq: c_int, _dummy: *mut c_void) -> irqreturn_t {
    let mut minframes: c_int = 1;

    amiga_custom.intena = IF_AUD0;

    if write_sq.active == 0 {
        /* Playing was interrupted and sq_reset() has already cleared
         * the sq variables, so better don't do anything here.
         */
        WAKE_UP(write_sq.sync_queue);
        return IRQ_HANDLED;
    }

    if (write_sq.active & AMI_PLAY_PLAYING) != 0 {
        /* We've just finished a frame */
        write_sq.count -= 1;
        WAKE_UP(write_sq.action_queue);
    }

    if (write_sq.active & AMI_PLAY_LOADED) != 0 {
        /* Increase threshold: frame 1 is already being played */
        minframes = 2;
    }

    /* Shift the flags */
    write_sq.active = (write_sq.active << 1) & AMI_PLAY_MASK;

    if write_sq.active == 0 {
        /* No frame is playing, disable audio DMA */
        StopDMA();
    }

    amiga_custom.intena = IF_SETCLR | IF_AUD0;

    if write_sq.count >= minframes {
        /* Try to play the next frame */
        AmiPlay();
    }

    if write_sq.active == 0 {
        /* Nothing to play anymore.
           Wake up a process waiting for audio output to drain. */
        WAKE_UP(write_sq.sync_queue);
    }
    IRQ_HANDLED
}

/*
 * /dev/mixer abstraction
 */
unsafe extern "C" fn AmiMixerInit() {
    dmasound.volume_left = 64;
    dmasound.volume_right = 64;
    amiga_custom.aud[0].audvol = dmasound.volume_left as u_short;
    amiga_custom.aud[3].audvol = 1; /* For pseudo 14bit */
    amiga_custom.aud[1].audvol = dmasound.volume_right as u_short;
    amiga_custom.aud[2].audvol = 1; /* For pseudo 14bit */
    dmasound.treble = 50;
}

unsafe extern "C" fn AmiMixerIoctl(cmd: u_int, arg: u_long) -> c_int {
    let mut data: c_int = 0;
    match cmd {
        SOUND_MIXER_READ_DEVMASK => IOCTL_OUT(arg, SOUND_MASK_VOLUME | SOUND_MASK_TREBLE),
        SOUND_MIXER_READ_RECMASK => IOCTL_OUT(arg, 0),
        SOUND_MIXER_READ_STEREODEVS => IOCTL_OUT(arg, SOUND_MASK_VOLUME),
        SOUND_MIXER_READ_VOLUME => IOCTL_OUT(
            arg,
            VOLUME_AMI_TO_VOXWARE(dmasound.volume_left)
                | (VOLUME_AMI_TO_VOXWARE(dmasound.volume_right) << 8),
        ),
        SOUND_MIXER_WRITE_VOLUME => {
            IOCTL_IN(arg, &mut data);
            IOCTL_OUT(arg, dmasound_set_volume(data))
        }
        SOUND_MIXER_READ_TREBLE => IOCTL_OUT(arg, dmasound.treble),
        SOUND_MIXER_WRITE_TREBLE => {
            IOCTL_IN(arg, &mut data);
            IOCTL_OUT(arg, dmasound_set_treble(data))
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn AmiWriteSqSetup() -> c_int {
    write_sq_block_size_half = (write_sq.block_size >> 1) as c_int;
    write_sq_block_size_quarter = write_sq_block_size_half >> 1;
    0
}

unsafe extern "C" fn AmiStateInfo(buffer: *mut c_char, space: size_t) -> c_int {
    let mut len: c_int = 0;
    len += sprintf(
        buffer.offset(len as isize),
        c"\tsound.volume_left = %d [0...64]\n".as_ptr(),
        dmasound.volume_left,
    );
    len += sprintf(
        buffer.offset(len as isize),
        c"\tsound.volume_right = %d [0...64]\n".as_ptr(),
        dmasound.volume_right,
    );
    if len as size_t >= space {
        printk(c"dmasound_paula: overflowed state buffer alloc.\n".as_ptr());
        len = space as c_int;
    }
    len
}

static mut def_hard: SETTINGS = SETTINGS {
    format: AFMT_S8,
    stereo: 0,
    size: 8,
    speed: 8000,
};

static mut def_soft: SETTINGS = SETTINGS {
    format: AFMT_U8,
    stereo: 0,
    size: 8,
    speed: 8000,
};

static mut machAmiga: MACHINE = MACHINE {
    name: c"Amiga".as_ptr(),
    name2: c"AMIGA".as_ptr(),
    owner: core::ptr::null_mut(),
    dma_alloc: Some(AmiAlloc),
    dma_free: Some(AmiFree),
    irqinit: Some(AmiIrqInit),
    irqcleanup: Some(AmiIrqCleanUp),
    init: Some(AmiInit),
    silence: Some(AmiSilence),
    setFormat: Some(AmiSetFormat),
    setVolume: Some(AmiSetVolume),
    setTreble: Some(AmiSetTreble),
    play: Some(AmiPlay),
    mixer_init: Some(AmiMixerInit),
    mixer_ioctl: Some(AmiMixerIoctl),
    write_sq_setup: Some(AmiWriteSqSetup),
    state_info: Some(AmiStateInfo),
    min_dsp_speed: 8000,
    version: (DMASOUND_PAULA_REVISION << 8) | DMASOUND_PAULA_EDITION,
    hardware_afmts: AFMT_S8 | AFMT_S16_BE, /* h'ware-supported formats *only* here */
    capabilities: DSP_CAP_BATCH,           /* As per SNDCTL_DSP_GETCAPS */
    default_hard: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
    default_soft: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
};

unsafe extern "C" fn amiga_audio_probe(_pdev: *mut platform_device) -> c_int {
    machAmiga.owner = THIS_MODULE;
    dmasound.mach = machAmiga;
    dmasound.mach.default_hard = def_hard;
    dmasound.mach.default_soft = def_soft;
    dmasound_init()
}

unsafe extern "C" fn amiga_audio_remove(_pdev: *mut platform_device) {
    dmasound_deinit();
}

/*
 * amiga_audio_remove() lives in .exit.text. For drivers registered via
 * module_platform_driver_probe() this is ok because they cannot get unbound at
 * runtime. So mark the driver struct with __refdata to prevent modpost
 * triggering a section mismatch warning.
 */
static mut amiga_audio_driver: platform_driver = platform_driver {
    remove: Some(amiga_audio_remove),
    driver: device_driver {
        name: c"amiga-audio".as_ptr(),
    },
};

/* module_platform_driver_probe(amiga_audio_driver, amiga_audio_probe); */

/* MODULE_DESCRIPTION("Amiga Paula DMA Sound Driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:amiga-audio"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
