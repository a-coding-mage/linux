// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/sound/oss/dmasound/dmasound_atari.c
 *
 *  Atari TT and Falcon DMA Sound Driver
 *
 *  See linux/sound/oss/dmasound/dmasound_core.c for copyright and credits
 *  prior to 28/01/2001
 *
 *  28/01/2001 [0.1] Iain Sandoe
 *                  - added versioning
 *                  - put in and populated the hardware_afmts field.
 *             [0.2] - put in SNDCTL_DSP_GETCAPS value.
 *  01/02/2001 [0.3] - put in default hard/soft settings.
 */

// C includes removed. External Linux/Atari OSS symbols are expected from
// the surrounding translated repository.

const DMASOUND_ATARI_REVISION: i32 = 0;
const DMASOUND_ATARI_EDITION: i32 = 3;

type ssize_t = isize;
type size_t = usize;
type u_char = u8;
type u_short = u16;
type u_int = u32;
type u_long = u64;
type gfp_t = u32;
type fmode_t = u32;
type irqreturn_t = i32;

extern "C" {
    fn atari_microwire_cmd(cmd: i32);
    fn atari_stram_alloc(size: u_int, name: *const i8) -> *mut core::ffi::c_void;
    fn atari_stram_free(obj: *mut core::ffi::c_void);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u_long, name: *const i8, dev: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t) -> i32;
    fn free_irq(irq: i32, dev: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t);
    fn atari_disable_irq(irq: i32);
    fn atari_enable_irq(irq: i32);
    fn virt_to_phys(ptr: *mut i8) -> u_long;
    fn dmasound_set_volume(v: i32) -> i32;
    fn dmasound_set_bass(v: i32) -> i32;
    fn dmasound_set_treble(v: i32) -> i32;
    fn dmasound_set_gain(v: i32) -> i32;
    fn dmasound_init() -> i32;
    fn dmasound_deinit();
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn printk(fmt: *const i8, ...) -> i32;
}

extern "C" {
    static mut dmasound: DmaSound;
    static mut write_sq: WriteSq;
    static mut st_mfp: StMfp;
    static mut tt_dmasnd: TtDmaSnd;
    static mut sound_ym: SoundYm;
    static dmasound_ulaw2dma8: [i8; 256];
    static dmasound_alaw2dma8: [i8; 256];
    static catchRadius: i32;
}

// External constants/macros from Linux and dmasound headers.
extern "C" {
    static AFMT_QUERY: i32;
    static AFMT_MU_LAW: i32;
    static AFMT_A_LAW: i32;
    static AFMT_S8: i32;
    static AFMT_U8: i32;
    static AFMT_S16_BE: i32;
    static AFMT_U16_BE: i32;
    static AFMT_S16_LE: i32;
    static AFMT_U16_LE: i32;
    static SND_DEV_DSP: i32;
    static IRQ_MFP_TIMA: i32;
    static IRQ_HANDLED: i32;
    static EFAULT: i32;
    static EINVAL: i32;
    static ENODEV: i32;
    static EBUSY: i32;
    static DMASND_CTRL_OFF: i32;
    static DMASND_CTRL_ON: i32;
    static DMASND_CTRL_REPEAT: i32;
    static DMASND_MODE_50KHZ: i32;
    static DMASND_MODE_25KHZ: i32;
    static DMASND_MODE_12KHZ: i32;
    static DMASND_MODE_6KHZ: i32;
    static DMASND_MODE_STEREO: i32;
    static DMASND_MODE_MONO: i32;
    static DMASND_MODE_8BIT: i32;
    static DMASND_MODE_16BIT: i32;
    static SOUND_MIXER_READ_SPEAKER: u_int;
    static SOUND_MIXER_WRITE_VOLUME: u_int;
    static SOUND_MIXER_WRITE_SPEAKER: u_int;
    static SOUND_MIXER_READ_RECMASK: u_int;
    static SOUND_MIXER_READ_DEVMASK: u_int;
    static SOUND_MIXER_READ_STEREODEVS: u_int;
    static SOUND_MIXER_READ_VOLUME: u_int;
    static SOUND_MIXER_READ_BASS: u_int;
    static SOUND_MIXER_READ_TREBLE: u_int;
    static SOUND_MIXER_READ_OGAIN: u_int;
    static SOUND_MIXER_WRITE_BASS: u_int;
    static SOUND_MIXER_WRITE_TREBLE: u_int;
    static SOUND_MIXER_WRITE_OGAIN: u_int;
    static SOUND_MIXER_READ_CAPS: u_int;
    static SOUND_MIXER_WRITE_MIC: u_int;
    static SOUND_MIXER_READ_MIC: u_int;
    static SOUND_MASK_VOLUME: i32;
    static SOUND_MASK_TREBLE: i32;
    static SOUND_MASK_BASS: i32;
    static SOUND_MASK_SPEAKER: i32;
    static SOUND_MASK_MIC: i32;
    static SOUND_CAP_EXCL_INPUT: i32;
    static DSP_CAP_BATCH: i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SETTINGS {
    format: i32,
    stereo: i32,
    size: i32,
    speed: i32,
}

#[repr(C)]
struct TRANS {
    ct_ulaw: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_alaw: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s8: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u8: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s16be: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u16be: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_s16le: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
    ct_u16le: Option<unsafe fn(*const u_char, size_t, *mut u_char, *mut ssize_t, ssize_t) -> ssize_t>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MACHINE {
    name: *const i8,
    name2: *const i8,
    owner: *mut core::ffi::c_void,
    dma_alloc: Option<unsafe fn(u_int, gfp_t) -> *mut core::ffi::c_void>,
    dma_free: Option<unsafe fn(*mut core::ffi::c_void, u_int)>,
    irqinit: Option<unsafe fn() -> i32>,
    // MODULE-only irqcleanup field exists in the C initializer when built as a module.
    init: Option<unsafe fn()>,
    silence: Option<unsafe fn()>,
    setFormat: Option<unsafe fn(i32) -> i32>,
    setVolume: Option<unsafe fn(i32) -> i32>,
    setBass: Option<unsafe fn(i32) -> i32>,
    setTreble: Option<unsafe fn(i32) -> i32>,
    setGain: Option<unsafe fn(i32) -> i32>,
    play: Option<unsafe fn()>,
    mixer_init: Option<unsafe fn()>,
    mixer_ioctl: Option<unsafe fn(u_int, u_long) -> i32>,
    write_sq_setup: Option<unsafe fn() -> i32>,
    sq_open: Option<unsafe fn(fmode_t) -> i32>,
    state_info: Option<unsafe fn(*mut i8, size_t) -> i32>,
    min_dsp_speed: i32,
    version: i32,
    hardware_afmts: i32,
    capabilities: i32,
    default_soft: SETTINGS,
    default_hard: SETTINGS,
}

#[repr(C)]
struct DmaSound {
    soft: SETTINGS,
    hard: SETTINGS,
    dsp: SETTINGS,
    mach: MACHINE,
    trans_write: *mut TRANS,
    minDev: i32,
    bass: i32,
    treble: i32,
    volume_left: i32,
    volume_right: i32,
    gain: i32,
    lock: core::ffi::c_void,
}

#[repr(C)]
struct WriteSq {
    buffers: *mut *mut i8,
    front: i32,
    count: i32,
    rear_size: i32,
    block_size: i32,
    max_count: i32,
    active: i32,
    syncing: i32,
    sync_queue: core::ffi::c_void,
    action_queue: core::ffi::c_void,
}

#[repr(C)]
struct StMfp {
    tim_ct_a: i32,
    tim_dt_a: i32,
    int_en_a: i32,
    int_mk_a: i32,
}

#[repr(C)]
struct TtDmaSnd {
    ctrl: i32,
    mode: i32,
    int_div: i32,
    int_ctrl: i32,
    cbar_src: i32,
    cbar_dst: i32,
    dac_src: i32,
    adc_src: i32,
    track_select: i32,
    rec_track_select: i32,
    output_atten: i32,
    input_gain: i32,
}

#[repr(C)]
struct SoundYm {
    rd_data_reg_sel: i32,
    wd_data: i32,
}

static mut is_falcon: i32 = 0;
static mut write_sq_ignore_int: i32 = 0; /* ++TeSche: used for Falcon */

static mut expand_bal: i32 = 0; /* Balance factor for expanding (not volume!) */
static mut expand_data: i32 = 0; /* Data for expanding */

unsafe fn min_ul(a: size_t, b: ssize_t) -> ssize_t {
    if a < b as size_t { a as ssize_t } else { b }
}

unsafe fn get_user_u8(dst: *mut u8, src: *const u8) -> i32 {
    *dst = core::ptr::read(src);
    0
}

unsafe fn get_user_u16(dst: *mut u16, src: *const u8) -> i32 {
    *dst = core::ptr::read_unaligned(src as *const u16);
    0
}

unsafe fn get_user_u32(dst: *mut u32, src: *const u8) -> i32 {
    *dst = core::ptr::read_unaligned(src as *const u32);
    0
}

unsafe fn copy_from_user(dst: *mut core::ffi::c_void, src: *const u8, count: ssize_t) -> i32 {
    core::ptr::copy_nonoverlapping(src, dst as *mut u8, count as usize);
    0
}

fn le2be16(v: u16) -> u16 {
    v.swap_bytes()
}

fn le2be16dbl(v: u64) -> u64 {
    ((v & 0x00ff00ff) << 8) | ((v & 0xff00ff00) >> 8)
}

unsafe fn DMASNDSetEnd(v: u_long) { let _ = v; /* external macro/register write */ }
unsafe fn DMASNDSetBase(v: u_long) { let _ = v; /* external macro/register write */ }
unsafe fn WAKE_UP<T>(_q: T) {}
unsafe fn spin_lock(_lock: *mut core::ffi::c_void) {}
unsafe fn spin_unlock(_lock: *mut core::ffi::c_void) {}
unsafe fn spin_lock_irqsave(_lock: *mut core::ffi::c_void, flags: *mut u_long) { *flags = 0; }
unsafe fn spin_unlock_irqrestore(_lock: *mut core::ffi::c_void, _flags: u_long) {}
unsafe fn IOCTL_OUT(_arg: u_long, value: i32) -> i32 { value }
unsafe fn IOCTL_IN(_arg: u_long, data: *mut i32) { *data = 0; }
unsafe fn MACH_IS_TT() -> bool { false }
unsafe fn MACH_IS_ATARI() -> bool { false }
unsafe fn ATARIHW_PRESENT(_hw: i32) -> bool { false }
unsafe fn MW_LM1992_BASS(v: i32) -> i32 { v }
unsafe fn MW_LM1992_TREBLE(v: i32) -> i32 { v }
unsafe fn MW_LM1992_PSG_HIGH() -> i32 { 0 }
unsafe fn MW_LM1992_BALLEFT(v: i32) -> i32 { v }
unsafe fn MW_LM1992_BALRIGHT(v: i32) -> i32 { v }
unsafe fn MW_LM1992_VOLUME(v: i32) -> i32 { v }

/*** Translations ************************************************************/

unsafe fn ata_ct_law(mut userPtr: *const u_char, userCount: size_t,
                     frame: *mut u_char, frameUsed: *mut ssize_t,
                     frameLeft: ssize_t) -> ssize_t {
    let table = if dmasound.soft.format == AFMT_MU_LAW {
        dmasound_ulaw2dma8.as_ptr()
    } else {
        dmasound_alaw2dma8.as_ptr()
    };
    let mut count = min_ul(userCount, frameLeft);
    if dmasound.soft.stereo != 0 { count &= !1; }
    let used = count;
    let mut p = frame.offset(*frameUsed);
    while count > 0 {
        let mut data: u_char = 0;
        if get_user_u8(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
        userPtr = userPtr.add(1);
        *p = *table.add(data as usize) as u_char;
        p = p.add(1);
        count -= 1;
    }
    *frameUsed += used;
    used
}

unsafe fn ata_ct_s8(userPtr: *const u_char, userCount: size_t,
                    frame: *mut u_char, frameUsed: *mut ssize_t,
                    frameLeft: ssize_t) -> ssize_t {
    let mut count = min_ul(userCount, frameLeft);
    if dmasound.soft.stereo != 0 { count &= !1; }
    let used = count;
    let p = frame.offset(*frameUsed) as *mut core::ffi::c_void;
    if copy_from_user(p, userPtr, count) != 0 { return -(EFAULT as ssize_t); }
    *frameUsed += used;
    used
}

unsafe fn ata_ct_u8(mut userPtr: *const u_char, userCount: size_t,
                    frame: *mut u_char, frameUsed: *mut ssize_t,
                    frameLeft: ssize_t) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed);
        count = min_ul(userCount, frameLeft);
        used = count;
        while count > 0 {
            let mut data: u_char = 0;
            if get_user_u8(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(1);
            *p = data ^ 0x80;
            p = p.add(1);
            count -= 1;
        }
    } else {
        let mut p = frame.offset(*frameUsed) as *mut u_short;
        count = min_ul(userCount, frameLeft) >> 1;
        used = count * 2;
        while count > 0 {
            let mut data: u_short = 0;
            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(2);
            *p = data ^ 0x8080;
            p = p.add(1);
            count -= 1;
        }
    }
    *frameUsed += used;
    used
}

unsafe fn ata_ct_s16be(mut userPtr: *const u_char, userCount: size_t,
                       frame: *mut u_char, frameUsed: *mut ssize_t,
                       frameLeft: ssize_t) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed) as *mut u_short;
        count = min_ul(userCount, frameLeft) >> 1;
        used = count * 2;
        while count > 0 {
            let mut data: u_short = 0;
            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(2);
            *p = data; p = p.add(1);
            *p = data; p = p.add(1);
            count -= 1;
        }
        *frameUsed += used * 2;
    } else {
        let p = frame.offset(*frameUsed) as *mut core::ffi::c_void;
        count = min_ul(userCount, frameLeft) & !3;
        used = count;
        if copy_from_user(p, userPtr, count) != 0 { return -(EFAULT as ssize_t); }
        *frameUsed += used;
    }
    used
}

unsafe fn ata_ct_u16be(mut userPtr: *const u_char, userCount: size_t,
                       frame: *mut u_char, frameUsed: *mut ssize_t,
                       frameLeft: ssize_t) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed) as *mut u_short;
        count = min_ul(userCount, frameLeft) >> 1;
        used = count * 2;
        while count > 0 {
            let mut data: u_short = 0;
            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(2);
            data ^= 0x8000;
            *p = data; p = p.add(1);
            *p = data; p = p.add(1);
            count -= 1;
        }
        *frameUsed += used * 2;
    } else {
        let mut p = frame.offset(*frameUsed) as *mut u_long;
        count = min_ul(userCount, frameLeft) >> 2;
        used = count * 4;
        while count > 0 {
            let mut data: u_int = 0;
            if get_user_u32(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(4);
            *p = (data ^ 0x80008000) as u_long;
            p = p.add(1);
            count -= 1;
        }
        *frameUsed += used;
    }
    used
}

unsafe fn ata_ct_s16le(mut userPtr: *const u_char, userCount: size_t,
                       frame: *mut u_char, frameUsed: *mut ssize_t,
                       frameLeft: ssize_t) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed) as *mut u_short;
        count = min_ul(userCount, frameLeft) >> 1;
        used = count * 2;
        while count > 0 {
            let mut data: u_short = 0;
            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(2);
            data = le2be16(data);
            *p = data; p = p.add(1);
            *p = data; p = p.add(1);
            count -= 1;
        }
        *frameUsed += used * 2;
    } else {
        let mut p = frame.offset(*frameUsed) as *mut u_long;
        count = min_ul(userCount, frameLeft) >> 2;
        used = count * 4;
        while count > 0 {
            let mut data: u_int = 0;
            if get_user_u32(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(4);
            *p = le2be16dbl(data as u_long);
            p = p.add(1);
            count -= 1;
        }
        *frameUsed += used;
    }
    used
}

unsafe fn ata_ct_u16le(mut userPtr: *const u_char, userCount: size_t,
                       frame: *mut u_char, frameUsed: *mut ssize_t,
                       frameLeft: ssize_t) -> ssize_t {
    let mut count: ssize_t;
    let used: ssize_t;
    if dmasound.soft.stereo == 0 {
        let mut p = frame.offset(*frameUsed) as *mut u_short;
        count = min_ul(userCount, frameLeft) >> 1;
        used = count * 2;
        while count > 0 {
            let mut data: u_short = 0;
            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(2);
            data = le2be16(data) ^ 0x8000;
            *p = data; p = p.add(1);
            *p = data; p = p.add(1);
            // The C source does not decrement count here.
        }
        *frameUsed += used * 2;
    } else {
        let mut p = frame.offset(*frameUsed) as *mut u_long;
        count = min_ul(userCount, frameLeft) >> 2;
        used = count;
        while count > 0 {
            let mut data: u_int = 0;
            if get_user_u32(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
            userPtr = userPtr.add(4);
            *p = le2be16dbl(data as u_long) ^ 0x80008000;
            p = p.add(1);
            count -= 1;
        }
        *frameUsed += used;
    }
    used
}

macro_rules! ata_ctx_body {
    ($name:ident, $law:expr, $xor8:expr, $xor16:expr, $le:expr, $wide:expr) => {
        unsafe fn $name(mut userPtr: *const u_char, mut userCount: size_t,
                        frame: *mut u_char, frameUsed: *mut ssize_t,
                        mut frameLeft: ssize_t) -> ssize_t {
            let table = if $law && dmasound.soft.format == AFMT_MU_LAW { dmasound_ulaw2dma8.as_ptr() } else { dmasound_alaw2dma8.as_ptr() };
            let mut bal: i64 = expand_bal as i64;
            let hSpeed: i64 = dmasound.hard.speed as i64;
            let sSpeed: i64 = dmasound.soft.speed as i64;
            let mut used: ssize_t = userCount as ssize_t;
            let usedf: ssize_t = frameLeft;
            if dmasound.soft.stereo == 0 {
                if $wide {
                    let mut p = frame.offset(*frameUsed) as *mut u_short;
                    let mut data: u_short = expand_data as u_short;
                    while frameLeft >= 4 {
                        if bal < 0 {
                            if userCount < 2 { break; }
                            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
                            userPtr = userPtr.add(2);
                            if $le { data = le2be16(data); }
                            data ^= $xor16;
                            userCount -= 2;
                            bal += hSpeed;
                        }
                        *p = data; p = p.add(1);
                        *p = data; p = p.add(1);
                        frameLeft -= 4;
                        bal -= sSpeed;
                    }
                    expand_data = data as i32;
                } else {
                    let mut p = frame.offset(*frameUsed);
                    let mut data: u_char = expand_data as u_char;
                    while frameLeft != 0 {
                        if bal < 0 {
                            if userCount == 0 { break; }
                            if get_user_u8(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
                            userPtr = userPtr.add(1);
                            if $law { data = *table.add(data as usize) as u_char; }
                            data ^= $xor8;
                            userCount -= 1;
                            bal += hSpeed;
                        }
                        *p = data; p = p.add(1);
                        frameLeft -= 1;
                        bal -= sSpeed;
                    }
                    expand_data = data as i32;
                }
            } else if $wide {
                let mut p = frame.offset(*frameUsed) as *mut u_long;
                let mut data: u_long = expand_data as u_long;
                while frameLeft >= 4 {
                    if bal < 0 {
                        if userCount < 4 { break; }
                        let mut tmp: u_int = 0;
                        if get_user_u32(&mut tmp, userPtr) != 0 { return -(EFAULT as ssize_t); }
                        userPtr = userPtr.add(4);
                        data = tmp as u_long;
                        if $le { data = le2be16dbl(data); }
                        data ^= $xor16 as u_long | (($xor16 as u_long) << 16);
                        userCount -= 4;
                        bal += hSpeed;
                    }
                    *p = data; p = p.add(1);
                    frameLeft -= 4;
                    bal -= sSpeed;
                }
                expand_data = data as i32;
            } else {
                let mut p = frame.offset(*frameUsed) as *mut u_short;
                let mut data: u_short = expand_data as u_short;
                while frameLeft >= 2 {
                    if bal < 0 {
                        if userCount < 2 { break; }
                        if $law {
                            let mut c: u_char = 0;
                            if get_user_u8(&mut c, userPtr) != 0 { return -(EFAULT as ssize_t); }
                            userPtr = userPtr.add(1);
                            data = (*table.add(c as usize) as u_short) << 8;
                            if get_user_u8(&mut c, userPtr) != 0 { return -(EFAULT as ssize_t); }
                            userPtr = userPtr.add(1);
                            data |= *table.add(c as usize) as u_short;
                        } else {
                            if get_user_u16(&mut data, userPtr) != 0 { return -(EFAULT as ssize_t); }
                            userPtr = userPtr.add(2);
                            data ^= $xor16;
                        }
                        userCount -= 2;
                        bal += hSpeed;
                    }
                    *p = data; p = p.add(1);
                    frameLeft -= 2;
                    bal -= sSpeed;
                }
                expand_data = data as i32;
            }
            expand_bal = bal as i32;
            used -= userCount as ssize_t;
            *frameUsed += usedf - frameLeft;
            used
        }
    };
}

ata_ctx_body!(ata_ctx_law, true, 0u8, 0u16, false, false);
ata_ctx_body!(ata_ctx_s8, false, 0u8, 0u16, false, false);
ata_ctx_body!(ata_ctx_u8, false, 0x80u8, 0x8080u16, false, false);
ata_ctx_body!(ata_ctx_s16be, false, 0u8, 0u16, false, true);
ata_ctx_body!(ata_ctx_u16be, false, 0u8, 0x8000u16, false, true);
ata_ctx_body!(ata_ctx_s16le, false, 0u8, 0u16, true, true);
ata_ctx_body!(ata_ctx_u16le, false, 0u8, 0x8000u16, true, true);

static mut transTTNormal: TRANS = TRANS {
    ct_ulaw: Some(ata_ct_law), ct_alaw: Some(ata_ct_law), ct_s8: Some(ata_ct_s8), ct_u8: Some(ata_ct_u8),
    ct_s16be: None, ct_u16be: None, ct_s16le: None, ct_u16le: None,
};

static mut transTTExpanding: TRANS = TRANS {
    ct_ulaw: Some(ata_ctx_law), ct_alaw: Some(ata_ctx_law), ct_s8: Some(ata_ctx_s8), ct_u8: Some(ata_ctx_u8),
    ct_s16be: None, ct_u16be: None, ct_s16le: None, ct_u16le: None,
};

static mut transFalconNormal: TRANS = TRANS {
    ct_ulaw: Some(ata_ct_law), ct_alaw: Some(ata_ct_law), ct_s8: Some(ata_ct_s8), ct_u8: Some(ata_ct_u8),
    ct_s16be: Some(ata_ct_s16be), ct_u16be: Some(ata_ct_u16be), ct_s16le: Some(ata_ct_s16le), ct_u16le: Some(ata_ct_u16le),
};

static mut transFalconExpanding: TRANS = TRANS {
    ct_ulaw: Some(ata_ctx_law), ct_alaw: Some(ata_ctx_law), ct_s8: Some(ata_ctx_s8), ct_u8: Some(ata_ctx_u8),
    ct_s16be: Some(ata_ctx_s16be), ct_u16be: Some(ata_ctx_u16be), ct_s16le: Some(ata_ctx_s16le), ct_u16le: Some(ata_ctx_u16le),
};

/*** Low level stuff *********************************************************/

unsafe fn AtaAlloc(size: u_int, _flags: gfp_t) -> *mut core::ffi::c_void {
    atari_stram_alloc(size, b"dmasound\0".as_ptr() as *const i8)
}

unsafe fn AtaFree(obj: *mut core::ffi::c_void, _size: u_int) {
    atari_stram_free(obj);
}

unsafe extern "C" fn AtaInterrupt(irq: i32, dummy: *mut core::ffi::c_void) -> irqreturn_t;

unsafe fn AtaIrqInit() -> i32 {
    st_mfp.tim_ct_a = 0;
    st_mfp.tim_dt_a = 1;
    st_mfp.tim_ct_a = 8;
    if request_irq(IRQ_MFP_TIMA, AtaInterrupt, 0, b"DMA sound\0".as_ptr() as *const i8, AtaInterrupt) != 0 {
        return 0;
    }
    st_mfp.int_en_a |= 0x20;
    st_mfp.int_mk_a |= 0x20;
    1
}

// MODULE-only in C.
unsafe fn AtaIrqCleanUp() {
    st_mfp.tim_ct_a = 0;
    st_mfp.int_en_a &= !0x20;
    free_irq(IRQ_MFP_TIMA, AtaInterrupt);
}

fn TONE_VOXWARE_TO_DB(v: i32) -> i32 { if v < 0 { -12 } else if v > 100 { 12 } else { (v - 50) * 6 / 25 } }
fn TONE_DB_TO_VOXWARE(v: i32) -> i32 { (v * 25 + if v > 0 { 5 } else { -5 }) / 6 + 50 }

unsafe fn AtaSetBass(bass: i32) -> i32 {
    dmasound.bass = TONE_VOXWARE_TO_DB(bass);
    atari_microwire_cmd(MW_LM1992_BASS(dmasound.bass));
    TONE_DB_TO_VOXWARE(dmasound.bass)
}

unsafe fn AtaSetTreble(treble: i32) -> i32 {
    dmasound.treble = TONE_VOXWARE_TO_DB(treble);
    atari_microwire_cmd(MW_LM1992_TREBLE(dmasound.treble));
    TONE_DB_TO_VOXWARE(dmasound.treble)
}

unsafe fn TTSilence() {
    tt_dmasnd.ctrl = DMASND_CTRL_OFF;
    atari_microwire_cmd(MW_LM1992_PSG_HIGH());
}

unsafe fn TTInit() {
    let freq = [50066, 25033, 12517, 6258];
    let mut idx = -1;
    for i in 0..freq.len() {
        if (100 * (dmasound.soft.speed - freq[i]).abs() / freq[i]) < catchRadius {
            idx = i as i32;
        }
    }
    if idx > -1 {
        dmasound.soft.speed = freq[idx as usize];
        dmasound.trans_write = &mut transTTNormal;
    } else {
        dmasound.trans_write = &mut transTTExpanding;
    }
    TTSilence();
    dmasound.hard = dmasound.soft;
    let mode;
    if dmasound.hard.speed > 50066 {
        dmasound.hard.speed = 50066; mode = DMASND_MODE_50KHZ; dmasound.trans_write = &mut transTTNormal;
    } else if dmasound.hard.speed > 25033 {
        dmasound.hard.speed = 50066; mode = DMASND_MODE_50KHZ;
    } else if dmasound.hard.speed > 12517 {
        dmasound.hard.speed = 25033; mode = DMASND_MODE_25KHZ;
    } else if dmasound.hard.speed > 6258 {
        dmasound.hard.speed = 12517; mode = DMASND_MODE_12KHZ;
    } else {
        dmasound.hard.speed = 6258; mode = DMASND_MODE_6KHZ;
    }
    tt_dmasnd.mode = (if dmasound.hard.stereo != 0 { DMASND_MODE_STEREO } else { DMASND_MODE_MONO }) | DMASND_MODE_8BIT | mode;
    expand_bal = -dmasound.soft.speed;
}

unsafe fn TTSetFormat(mut format: i32) -> i32 {
    if format == AFMT_QUERY { return dmasound.soft.format; }
    if !(format == AFMT_MU_LAW || format == AFMT_A_LAW || format == AFMT_S8 || format == AFMT_U8) {
        format = AFMT_S8;
    }
    dmasound.soft.format = format;
    dmasound.soft.size = 8;
    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.format = format;
        dmasound.dsp.size = 8;
    }
    TTInit();
    format
}

fn VOLUME_VOXWARE_TO_DB(v: i32) -> i32 { if v < 0 { -40 } else if v > 100 { 0 } else { v * 2 / 5 - 40 } }
fn VOLUME_DB_TO_VOXWARE(v: i32) -> i32 { ((v + 40) * 5 + 1) / 2 }

unsafe fn TTSetVolume(volume: i32) -> i32 {
    dmasound.volume_left = VOLUME_VOXWARE_TO_DB(volume & 0xff);
    atari_microwire_cmd(MW_LM1992_BALLEFT(dmasound.volume_left));
    dmasound.volume_right = VOLUME_VOXWARE_TO_DB((volume & 0xff00) >> 8);
    atari_microwire_cmd(MW_LM1992_BALRIGHT(dmasound.volume_right));
    VOLUME_DB_TO_VOXWARE(dmasound.volume_left) | (VOLUME_DB_TO_VOXWARE(dmasound.volume_right) << 8)
}

fn GAIN_VOXWARE_TO_DB(v: i32) -> i32 { if v < 0 { -80 } else if v > 100 { 0 } else { v * 4 / 5 - 80 } }
fn GAIN_DB_TO_VOXWARE(v: i32) -> i32 { ((v + 80) * 5 + 1) / 4 }

unsafe fn TTSetGain(gain: i32) -> i32 {
    dmasound.gain = GAIN_VOXWARE_TO_DB(gain);
    atari_microwire_cmd(MW_LM1992_VOLUME(dmasound.gain));
    GAIN_DB_TO_VOXWARE(dmasound.gain)
}

unsafe fn FalconSilence() {
    tt_dmasnd.ctrl = DMASND_CTRL_OFF;
    tt_dmasnd.mode = DMASND_MODE_50KHZ | DMASND_MODE_STEREO | DMASND_MODE_8BIT;
    tt_dmasnd.int_div = 0;
    tt_dmasnd.int_ctrl = 0x0;
    tt_dmasnd.cbar_src = 0x0000;
    tt_dmasnd.cbar_dst = 0x0000;
    tt_dmasnd.dac_src = 1;
    tt_dmasnd.adc_src = 3;
}

unsafe fn FalconInit() {
    let freq = [49170, 32780, 24585, 19668, 16390, 12292, 9834, 8195];
    let mut idx = -1;
    for i in 0..freq.len() {
        if (100 * (dmasound.soft.speed - freq[i]).abs() / freq[i]) < catchRadius { idx = i as i32; }
    }
    if idx > -1 {
        dmasound.soft.speed = freq[idx as usize];
        dmasound.trans_write = &mut transFalconNormal;
    } else {
        dmasound.trans_write = &mut transFalconExpanding;
    }
    FalconSilence();
    dmasound.hard = dmasound.soft;
    if dmasound.hard.size == 16 { dmasound.hard.stereo = 1; }
    let divider;
    if dmasound.hard.speed > 49170 { dmasound.hard.speed = 49170; divider = 1; dmasound.trans_write = &mut transFalconNormal; }
    else if dmasound.hard.speed > 32780 { dmasound.hard.speed = 49170; divider = 1; }
    else if dmasound.hard.speed > 24585 { dmasound.hard.speed = 32780; divider = 2; }
    else if dmasound.hard.speed > 19668 { dmasound.hard.speed = 24585; divider = 3; }
    else if dmasound.hard.speed > 16390 { dmasound.hard.speed = 19668; divider = 4; }
    else if dmasound.hard.speed > 12292 { dmasound.hard.speed = 16390; divider = 5; }
    else if dmasound.hard.speed > 9834 { dmasound.hard.speed = 12292; divider = 7; }
    else if dmasound.hard.speed > 8195 { dmasound.hard.speed = 9834; divider = 9; }
    else { dmasound.hard.speed = 8195; divider = 11; }
    tt_dmasnd.int_div = divider;
    tt_dmasnd.int_ctrl = 0x4;
    tt_dmasnd.track_select = 0x0;
    tt_dmasnd.cbar_src = 0x0001;
    tt_dmasnd.cbar_dst = 0x0000;
    tt_dmasnd.rec_track_select = 0;
    tt_dmasnd.dac_src = 2;
    tt_dmasnd.adc_src = 0;
    tt_dmasnd.mode = (if dmasound.hard.stereo != 0 { DMASND_MODE_STEREO } else { DMASND_MODE_MONO }) |
        (if dmasound.hard.size == 8 { DMASND_MODE_8BIT } else { DMASND_MODE_16BIT }) |
        DMASND_MODE_6KHZ;
    expand_bal = -dmasound.soft.speed;
}

unsafe fn FalconSetFormat(mut format: i32) -> i32 {
    let size;
    if format == AFMT_QUERY { return dmasound.soft.format; }
    if format == AFMT_MU_LAW || format == AFMT_A_LAW || format == AFMT_U8 || format == AFMT_S8 {
        size = 8;
    } else if format == AFMT_S16_BE || format == AFMT_U16_BE || format == AFMT_S16_LE || format == AFMT_U16_LE {
        size = 16;
    } else {
        size = 8;
        format = AFMT_S8;
    }
    dmasound.soft.format = format;
    dmasound.soft.size = size;
    if dmasound.minDev == SND_DEV_DSP {
        dmasound.dsp.format = format;
        dmasound.dsp.size = dmasound.soft.size;
    }
    FalconInit();
    format
}

fn VOLUME_VOXWARE_TO_ATT(v: i32) -> i32 { if v < 0 { 15 } else if v > 100 { 0 } else { 15 - v * 3 / 20 } }
fn VOLUME_ATT_TO_VOXWARE(v: i32) -> i32 { 100 - v * 20 / 3 }

unsafe fn FalconSetVolume(volume: i32) -> i32 {
    dmasound.volume_left = VOLUME_VOXWARE_TO_ATT(volume & 0xff);
    dmasound.volume_right = VOLUME_VOXWARE_TO_ATT((volume & 0xff00) >> 8);
    tt_dmasnd.output_atten = dmasound.volume_left << 8 | dmasound.volume_right << 4;
    VOLUME_ATT_TO_VOXWARE(dmasound.volume_left) | VOLUME_ATT_TO_VOXWARE(dmasound.volume_right) << 8
}

unsafe fn AtaPlayNextFrame(index: i32) {
    let start = *write_sq.buffers.offset(write_sq.front as isize);
    let end = start.offset(if write_sq.count == index { write_sq.rear_size as isize } else { write_sq.block_size as isize });
    DMASNDSetEnd(virt_to_phys(end.offset(-1)) + 1);
    DMASNDSetBase(virt_to_phys(start));
    write_sq.front = (write_sq.front + 1) % write_sq.max_count;
    write_sq.active += 1;
    tt_dmasnd.ctrl = DMASND_CTRL_ON | DMASND_CTRL_REPEAT;
}

unsafe fn AtaPlay() {
    atari_disable_irq(IRQ_MFP_TIMA);
    if write_sq.active == 2 || write_sq.count <= 0 {
        atari_enable_irq(IRQ_MFP_TIMA);
        return;
    }
    if write_sq.active == 0 {
        if write_sq.count == 1 && write_sq.rear_size < write_sq.block_size && write_sq.syncing == 0 {
            atari_enable_irq(IRQ_MFP_TIMA); return;
        }
        AtaPlayNextFrame(1);
        if write_sq.count == 1 { atari_enable_irq(IRQ_MFP_TIMA); return; }
        if write_sq.count == 2 && write_sq.rear_size < write_sq.block_size && write_sq.syncing == 0 {
            atari_enable_irq(IRQ_MFP_TIMA); return;
        }
        AtaPlayNextFrame(2);
    } else {
        if write_sq.count == 2 && write_sq.rear_size < write_sq.block_size && write_sq.syncing == 0 {
            atari_enable_irq(IRQ_MFP_TIMA); return;
        }
        AtaPlayNextFrame(2);
    }
    atari_enable_irq(IRQ_MFP_TIMA);
}

unsafe extern "C" fn AtaInterrupt(_irq: i32, _dummy: *mut core::ffi::c_void) -> irqreturn_t {
    spin_lock(&mut dmasound.lock);
    if write_sq_ignore_int != 0 && is_falcon != 0 {
        write_sq_ignore_int = 0;
        spin_unlock(&mut dmasound.lock);
        return IRQ_HANDLED;
    }
    if write_sq.active == 0 {
        WAKE_UP(&mut write_sq.sync_queue);
        spin_unlock(&mut dmasound.lock);
        return IRQ_HANDLED;
    }
    write_sq.count -= 1;
    write_sq.active -= 1;
    if write_sq.active == 0 {
        tt_dmasnd.ctrl = DMASND_CTRL_OFF;
        write_sq_ignore_int = 1;
    }
    WAKE_UP(&mut write_sq.action_queue);
    if write_sq.active != 1 || write_sq.count != 1 {
        AtaPlay();
    }
    if write_sq.active == 0 { WAKE_UP(&mut write_sq.sync_queue); }
    spin_unlock(&mut dmasound.lock);
    IRQ_HANDLED
}

/*** Mid level stuff *********************************************************/

fn RECLEVEL_VOXWARE_TO_GAIN(v: i32) -> i32 { if v < 0 { 0 } else if v > 100 { 15 } else { v * 3 / 20 } }
fn RECLEVEL_GAIN_TO_VOXWARE(v: i32) -> i32 { (v * 20 + 2) / 3 }

unsafe fn TTMixerInit() {
    atari_microwire_cmd(MW_LM1992_VOLUME(0));
    dmasound.volume_left = 0;
    atari_microwire_cmd(MW_LM1992_BALLEFT(0));
    dmasound.volume_right = 0;
    atari_microwire_cmd(MW_LM1992_BALRIGHT(0));
    atari_microwire_cmd(MW_LM1992_TREBLE(0));
    atari_microwire_cmd(MW_LM1992_BASS(0));
}

unsafe fn FalconMixerInit() {
    dmasound.volume_left = (tt_dmasnd.output_atten & 0xf00) >> 8;
    dmasound.volume_right = (tt_dmasnd.output_atten & 0xf0) >> 4;
}

unsafe fn AtaMixerIoctl(cmd: u_int, arg: u_long) -> i32 {
    let mut data: i32 = 0;
    let mut flags: u_long = 0;
    if cmd == SOUND_MIXER_READ_SPEAKER {
        if is_falcon != 0 || MACH_IS_TT() {
            spin_lock_irqsave(&mut dmasound.lock, &mut flags);
            sound_ym.rd_data_reg_sel = 14;
            let porta = sound_ym.rd_data_reg_sel;
            spin_unlock_irqrestore(&mut dmasound.lock, flags);
            return IOCTL_OUT(arg, if porta & 0x40 != 0 { 0 } else { 100 });
        }
    } else if cmd == SOUND_MIXER_WRITE_VOLUME {
        IOCTL_IN(arg, &mut data);
        return IOCTL_OUT(arg, dmasound_set_volume(data));
    } else if cmd == SOUND_MIXER_WRITE_SPEAKER {
        if is_falcon != 0 || MACH_IS_TT() {
            IOCTL_IN(arg, &mut data);
            spin_lock_irqsave(&mut dmasound.lock, &mut flags);
            sound_ym.rd_data_reg_sel = 14;
            let porta = (sound_ym.rd_data_reg_sel & !0x40) | if data < 50 { 0x40 } else { 0 };
            sound_ym.wd_data = porta;
            spin_unlock_irqrestore(&mut dmasound.lock, flags);
            return IOCTL_OUT(arg, if porta & 0x40 != 0 { 0 } else { 100 });
        }
    }
    -EINVAL
}

unsafe fn TTMixerIoctl(cmd: u_int, arg: u_long) -> i32 {
    let mut data: i32 = 0;
    if cmd == SOUND_MIXER_READ_RECMASK { return IOCTL_OUT(arg, 0); }
    if cmd == SOUND_MIXER_READ_DEVMASK { return IOCTL_OUT(arg, SOUND_MASK_VOLUME | SOUND_MASK_TREBLE | SOUND_MASK_BASS | if MACH_IS_TT() { SOUND_MASK_SPEAKER } else { 0 }); }
    if cmd == SOUND_MIXER_READ_STEREODEVS { return IOCTL_OUT(arg, SOUND_MASK_VOLUME); }
    if cmd == SOUND_MIXER_READ_VOLUME { return IOCTL_OUT(arg, VOLUME_DB_TO_VOXWARE(dmasound.volume_left) | (VOLUME_DB_TO_VOXWARE(dmasound.volume_right) << 8)); }
    if cmd == SOUND_MIXER_READ_BASS { return IOCTL_OUT(arg, TONE_DB_TO_VOXWARE(dmasound.bass)); }
    if cmd == SOUND_MIXER_READ_TREBLE { return IOCTL_OUT(arg, TONE_DB_TO_VOXWARE(dmasound.treble)); }
    if cmd == SOUND_MIXER_READ_OGAIN { return IOCTL_OUT(arg, GAIN_DB_TO_VOXWARE(dmasound.gain)); }
    if cmd == SOUND_MIXER_WRITE_BASS { IOCTL_IN(arg, &mut data); return IOCTL_OUT(arg, dmasound_set_bass(data)); }
    if cmd == SOUND_MIXER_WRITE_TREBLE { IOCTL_IN(arg, &mut data); return IOCTL_OUT(arg, dmasound_set_treble(data)); }
    if cmd == SOUND_MIXER_WRITE_OGAIN { IOCTL_IN(arg, &mut data); return IOCTL_OUT(arg, dmasound_set_gain(data)); }
    AtaMixerIoctl(cmd, arg)
}

unsafe fn FalconMixerIoctl(cmd: u_int, arg: u_long) -> i32 {
    let mut data: i32 = 0;
    if cmd == SOUND_MIXER_READ_RECMASK { return IOCTL_OUT(arg, SOUND_MASK_MIC); }
    if cmd == SOUND_MIXER_READ_DEVMASK { return IOCTL_OUT(arg, SOUND_MASK_VOLUME | SOUND_MASK_MIC | SOUND_MASK_SPEAKER); }
    if cmd == SOUND_MIXER_READ_STEREODEVS { return IOCTL_OUT(arg, SOUND_MASK_VOLUME | SOUND_MASK_MIC); }
    if cmd == SOUND_MIXER_READ_VOLUME { return IOCTL_OUT(arg, VOLUME_ATT_TO_VOXWARE(dmasound.volume_left) | VOLUME_ATT_TO_VOXWARE(dmasound.volume_right) << 8); }
    if cmd == SOUND_MIXER_READ_CAPS { return IOCTL_OUT(arg, SOUND_CAP_EXCL_INPUT); }
    if cmd == SOUND_MIXER_WRITE_MIC {
        IOCTL_IN(arg, &mut data);
        tt_dmasnd.input_gain = RECLEVEL_VOXWARE_TO_GAIN(data & 0xff) << 4 | RECLEVEL_VOXWARE_TO_GAIN((data >> 8) & 0xff);
        return IOCTL_OUT(arg, RECLEVEL_GAIN_TO_VOXWARE((tt_dmasnd.input_gain >> 4) & 0xf) | RECLEVEL_GAIN_TO_VOXWARE(tt_dmasnd.input_gain & 0xf) << 8);
    }
    if cmd == SOUND_MIXER_READ_MIC {
        return IOCTL_OUT(arg, RECLEVEL_GAIN_TO_VOXWARE((tt_dmasnd.input_gain >> 4) & 0xf) | RECLEVEL_GAIN_TO_VOXWARE(tt_dmasnd.input_gain & 0xf) << 8);
    }
    AtaMixerIoctl(cmd, arg)
}

unsafe fn AtaWriteSqSetup() -> i32 {
    write_sq_ignore_int = 0;
    0
}

unsafe fn AtaSqOpen(_mode: fmode_t) -> i32 {
    write_sq_ignore_int = 1;
    0
}

unsafe fn TTStateInfo(buffer: *mut i8, space: size_t) -> i32 {
    let mut len = 0;
    len += sprintf(buffer.offset(len as isize), b"\tvol left  %ddB [-40...  0]\n\0".as_ptr() as *const i8, dmasound.volume_left);
    len += sprintf(buffer.offset(len as isize), b"\tvol right %ddB [-40...  0]\n\0".as_ptr() as *const i8, dmasound.volume_right);
    len += sprintf(buffer.offset(len as isize), b"\tbass      %ddB [-12...+12]\n\0".as_ptr() as *const i8, dmasound.bass);
    len += sprintf(buffer.offset(len as isize), b"\ttreble    %ddB [-12...+12]\n\0".as_ptr() as *const i8, dmasound.treble);
    if len as size_t >= space {
        printk(b"dmasound_atari: overflowed state buffer alloc.\n\0".as_ptr() as *const i8);
        len = space as i32;
    }
    len
}

unsafe fn FalconStateInfo(buffer: *mut i8, space: size_t) -> i32 {
    let mut len = 0;
    len += sprintf(buffer.offset(len as isize), b"\tvol left  %ddB [-22.5 ... 0]\n\0".as_ptr() as *const i8, dmasound.volume_left);
    len += sprintf(buffer.offset(len as isize), b"\tvol right %ddB [-22.5 ... 0]\n\0".as_ptr() as *const i8, dmasound.volume_right);
    if len as size_t >= space {
        printk(b"dmasound_atari: overflowed state buffer alloc.\n\0".as_ptr() as *const i8);
        len = space as i32;
    }
    len
}

/*** Machine definitions *****************************************************/

static mut def_hard_falcon: SETTINGS = SETTINGS { format: 0, stereo: 0, size: 8, speed: 8195 };
static mut def_hard_tt: SETTINGS = SETTINGS { format: 0, stereo: 0, size: 8, speed: 12517 };
static mut def_soft: SETTINGS = SETTINGS { format: 0, stereo: 0, size: 8, speed: 8000 };

static mut machTT: MACHINE = MACHINE {
    name: b"Atari\0".as_ptr() as *const i8,
    name2: b"TT\0".as_ptr() as *const i8,
    owner: core::ptr::null_mut(),
    dma_alloc: Some(AtaAlloc),
    dma_free: Some(AtaFree),
    irqinit: Some(AtaIrqInit),
    init: Some(TTInit),
    silence: Some(TTSilence),
    setFormat: Some(TTSetFormat),
    setVolume: Some(TTSetVolume),
    setBass: Some(AtaSetBass),
    setTreble: Some(AtaSetTreble),
    setGain: Some(TTSetGain),
    play: Some(AtaPlay),
    mixer_init: Some(TTMixerInit),
    mixer_ioctl: Some(TTMixerIoctl),
    write_sq_setup: Some(AtaWriteSqSetup),
    sq_open: Some(AtaSqOpen),
    state_info: Some(TTStateInfo),
    min_dsp_speed: 6258,
    version: (DMASOUND_ATARI_REVISION << 8) | DMASOUND_ATARI_EDITION,
    hardware_afmts: 0,
    capabilities: 0,
    default_soft: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
    default_hard: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
};

static mut machFalcon: MACHINE = MACHINE {
    name: b"Atari\0".as_ptr() as *const i8,
    name2: b"FALCON\0".as_ptr() as *const i8,
    owner: core::ptr::null_mut(),
    dma_alloc: Some(AtaAlloc),
    dma_free: Some(AtaFree),
    irqinit: Some(AtaIrqInit),
    init: Some(FalconInit),
    silence: Some(FalconSilence),
    setFormat: Some(FalconSetFormat),
    setVolume: Some(FalconSetVolume),
    setBass: Some(AtaSetBass),
    setTreble: Some(AtaSetTreble),
    setGain: None,
    play: Some(AtaPlay),
    mixer_init: Some(FalconMixerInit),
    mixer_ioctl: Some(FalconMixerIoctl),
    write_sq_setup: Some(AtaWriteSqSetup),
    sq_open: Some(AtaSqOpen),
    state_info: Some(FalconStateInfo),
    min_dsp_speed: 8195,
    version: (DMASOUND_ATARI_REVISION << 8) | DMASOUND_ATARI_EDITION,
    hardware_afmts: 0,
    capabilities: 0,
    default_soft: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
    default_hard: SETTINGS { format: 0, stereo: 0, size: 0, speed: 0 },
};

/*** Config & Setup **********************************************************/

unsafe fn dmasound_atari_init() -> i32 {
    const PCM_8BIT: i32 = 0;
    const CODEC: i32 = 0;
    const MICROWIRE: i32 = 0;
    if MACH_IS_ATARI() && ATARIHW_PRESENT(PCM_8BIT) {
        if ATARIHW_PRESENT(CODEC) {
            dmasound.mach = machFalcon;
            dmasound.mach.default_soft = def_soft;
            dmasound.mach.default_hard = def_hard_falcon;
            is_falcon = 1;
        } else if ATARIHW_PRESENT(MICROWIRE) {
            dmasound.mach = machTT;
            dmasound.mach.default_soft = def_soft;
            dmasound.mach.default_hard = def_hard_tt;
            is_falcon = 0;
        } else {
            return -ENODEV;
        }
        if (st_mfp.int_en_a & st_mfp.int_mk_a & 0x20) == 0 {
            return dmasound_init();
        } else {
            printk(b"DMA sound driver: Timer A interrupt already in use\n\0".as_ptr() as *const i8);
            return -EBUSY;
        }
    }
    -ENODEV
}

unsafe fn dmasound_atari_cleanup() {
    dmasound_deinit();
}

// module_init(dmasound_atari_init);
// module_exit(dmasound_atari_cleanup);
// MODULE_DESCRIPTION("Atari TT and Falcon DMA Sound Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
