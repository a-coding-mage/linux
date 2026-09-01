// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * synth device handlers
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

/*
 * Rust translation of seq_oss_synth.c.
 *
 * The original C file depends on Linux/ALSA headers:
 * seq_oss_synth.h, seq_oss_midi.h, ../seq_lock.h, linux/init.h,
 * linux/module.h, linux/slab.h, and linux/nospec.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

/*
 * constants
 */
pub const SNDRV_SEQ_OSS_MAX_SYNTH_NAME: usize = 30;
pub const MAX_SYSEX_BUFLEN: usize = 128;
/* From seq_oss_synth.h / OSS headers; required here as C preprocessor constants. */
pub const SNDRV_SEQ_OSS_MAX_SYNTH_DEVS: usize = 16;
pub const SYNTH_TYPE_MIDI: c_int = 0;

/*
 * definition of synth info records
 */

/* synth info */
#[repr(C)]
pub struct seq_oss_synth {
    pub seq_device: c_int,

    /* for synth_info */
    pub synth_type: c_int,
    pub synth_subtype: c_int,
    pub nr_voices: c_int,

    pub name: [c_char; SNDRV_SEQ_OSS_MAX_SYNTH_NAME],
    pub oper: snd_seq_oss_callback,

    pub opened: c_int,

    pub private_data: *mut c_void,
    pub use_lock: snd_use_lock_t,
}

/*
 * device table
 */
static mut max_synth_devs: c_int = 0;
static mut synth_devs: [*mut seq_oss_synth; SNDRV_SEQ_OSS_MAX_SYNTH_DEVS] =
    [ptr::null_mut(); SNDRV_SEQ_OSS_MAX_SYNTH_DEVS];
static mut midi_synth_dev: seq_oss_synth = seq_oss_synth {
    seq_device: -1,
    synth_type: SYNTH_TYPE_MIDI,
    synth_subtype: 0,
    nr_voices: 16,
    name: [
        b'M' as c_char,
        b'I' as c_char,
        b'D' as c_char,
        b'I' as c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    oper: snd_seq_oss_callback {
        owner: ptr::null_mut(),
        open: None,
        close: None,
        ioctl: None,
        load_patch: None,
        reset: None,
    },
    opened: 0,
    private_data: ptr::null_mut(),
    use_lock: 0,
};

static mut register_lock: spinlock_t = 0;

extern "C" {
    static mut SNDRV_CARDS: c_int;

    static SNDRV_SEQ_OSS_MODE_SYNTH: c_int;
    static SNDRV_SEQ_OSS_PROCESS_EVENTS: c_int;
    static SNDRV_SEQ_OSS_PASS_EVENTS: c_int;
    static SNDRV_SEQ_EVENT_RESET: c_int;
    static SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_int;
    static SNDRV_SEQ_EVENT_OSS: c_int;
    static SNDRV_OSS_INFO_DEV_SYNTH: c_int;

    static ENOMEM: c_int;
    static ENXIO: c_int;
    static EINVAL: c_int;
    static GFP_KERNEL: gfp_t;

    fn snd_use_lock_init(lock: *mut snd_use_lock_t);
    fn snd_use_lock_use(lock: *mut snd_use_lock_t);
    fn snd_use_lock_free(lock: *mut snd_use_lock_t);
    fn snd_use_lock_sync(lock: *mut snd_use_lock_t);

    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn array_index_nospec(index: c_int, size: usize) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);

    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);

    fn snd_seq_oss_midi_open(dp: *mut seq_oss_devinfo, dev: c_int, mode: c_int) -> c_int;
    fn snd_seq_oss_midi_close(dp: *mut seq_oss_devinfo, dev: c_int);
    fn snd_seq_oss_midi_get_addr(dp: *mut seq_oss_devinfo, dev: c_int, addr: *mut snd_seq_addr);
    fn snd_seq_oss_midi_reset(dp: *mut seq_oss_devinfo, dev: c_int);
    fn snd_seq_oss_midi_make_info(
        dp: *mut seq_oss_devinfo,
        dev: c_int,
        inf: *mut midi_info,
    ) -> c_int;

    fn snd_seq_oss_fill_addr(
        dp: *mut seq_oss_devinfo,
        ev: *mut snd_seq_event,
        client: c_int,
        port: c_int,
    );
    fn snd_seq_oss_dispatch(
        dp: *mut seq_oss_devinfo,
        ev: *mut snd_seq_event,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;

    fn snd_oss_info_register(dev: c_int, num: c_int, name: *const c_char);
    fn snd_oss_info_unregister(dev: c_int, num: c_int);

    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn str_enabled_disabled(value: c_long) -> *const c_char;
}

#[repr(C)]
pub struct snd_seq_device {
    pub name: *const c_char,
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct snd_seq_oss_reg {
    pub type_: c_int,
    pub subtype: c_int,
    pub nvoices: c_int,
    pub oper: snd_seq_oss_callback,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_oss_callback {
    pub owner: *mut module,
    pub open: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg, *mut c_void) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg)>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg, c_uint, c_ulong) -> c_int>,
    pub load_patch: Option<
        unsafe extern "C" fn(*mut snd_seq_oss_arg, c_int, *const c_char, c_int, c_int) -> c_int,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut snd_seq_oss_arg)>,
}

#[repr(C)]
pub struct seq_oss_devinfo {
    pub max_synthdev: c_int,
    pub synth_opened: c_int,
    pub synths: [seq_oss_synthinfo; SNDRV_SEQ_OSS_MAX_SYNTH_DEVS],
    pub max_mididev: c_int,
    pub port: c_int,
    pub file_mode: c_int,
    pub seq_mode: c_int,
}

#[repr(C)]
pub struct seq_oss_synthinfo {
    pub arg: snd_seq_oss_arg,
    pub opened: c_int,
    pub nr_voices: c_int,
    pub ch: *mut seq_oss_chinfo,
    pub is_midi: c_int,
    pub midi_mapped: c_int,
}

#[repr(C)]
pub struct snd_seq_oss_arg {
    pub app_index: c_int,
    pub file_mode: c_int,
    pub seq_mode: c_int,
    pub event_passing: c_int,
    pub private_data: *mut c_void,
    pub addr: snd_seq_addr,
}

#[repr(C)]
pub struct seq_oss_chinfo {
    pub note: c_int,
    pub vel: c_int,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_int,
    pub flags: c_int,
    pub data: snd_seq_event_data,
}

#[repr(C)]
pub union snd_seq_event_data {
    pub ext: snd_seq_ev_ext,
    pub raw8: snd_seq_ev_raw8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_ext {
    pub len: c_int,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ev_raw8 {
    pub d: [u8; 8],
}

#[repr(C)]
pub struct synth_info {
    pub synth_type: c_int,
    pub synth_subtype: c_int,
    pub nr_voices: c_int,
    pub device: c_int,
    pub name: [c_char; SNDRV_SEQ_OSS_MAX_SYNTH_NAME],
}

#[repr(C)]
pub struct midi_info {
    pub name: [c_char; SNDRV_SEQ_OSS_MAX_SYNTH_NAME],
}

pub enum module {}
pub enum snd_info_buffer {}
pub type snd_use_lock_t = c_int;
pub type spinlock_t = c_int;
pub type gfp_t = c_uint;

unsafe fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut snd_seq_oss_reg {
    (*dev).driver_data as *mut snd_seq_oss_reg
}

unsafe fn lock_register_irqsave() -> c_ulong {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut register_lock, &mut flags);
    flags
}

unsafe fn unlock_register_irqrestore(flags: c_ulong) {
    spin_unlock_irqrestore(&mut register_lock, flags);
}

/*
 * global initialization
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_init() {
    snd_use_lock_init(&mut midi_synth_dev.use_lock);
}

/*
 * registration of the synth device
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_probe(dev: *mut snd_seq_device) -> c_int {
    let mut i: c_int;
    let rec: *mut seq_oss_synth;
    let reg: *mut snd_seq_oss_reg = SNDRV_SEQ_DEVICE_ARGPTR(dev);

    rec = kzalloc(mem::size_of::<seq_oss_synth>(), GFP_KERNEL) as *mut seq_oss_synth;
    if rec.is_null() {
        return -ENOMEM;
    }
    (*rec).seq_device = -1;
    (*rec).synth_type = (*reg).type_;
    (*rec).synth_subtype = (*reg).subtype;
    (*rec).nr_voices = (*reg).nvoices;
    (*rec).oper = (*reg).oper;
    (*rec).private_data = (*reg).private_data;
    (*rec).opened = 0;
    snd_use_lock_init(&mut (*rec).use_lock);

    /* copy and truncate the name of synth device */
    strscpy((*rec).name.as_mut_ptr(), (*dev).name, (*rec).name.len());

    /* registration */
    let flags = lock_register_irqsave();
    i = 0;
    while i < max_synth_devs {
        if synth_devs[i as usize].is_null() {
            break;
        }
        i += 1;
    }
    if i >= max_synth_devs {
        if max_synth_devs >= SNDRV_SEQ_OSS_MAX_SYNTH_DEVS as c_int {
            pr_err(b"ALSA: seq_oss: no more synth slot\n\0".as_ptr() as *const c_char);
            kfree(rec as *mut c_void);
            unlock_register_irqrestore(flags);
            return -ENOMEM;
        }
        max_synth_devs += 1;
    }
    (*rec).seq_device = i;
    synth_devs[i as usize] = rec;
    unlock_register_irqrestore(flags);

    (*dev).driver_data = rec as *mut c_void;

    /* #ifdef SNDRV_OSS_INFO_DEV_SYNTH */
    if i < SNDRV_CARDS {
        snd_oss_info_register(SNDRV_OSS_INFO_DEV_SYNTH, i, (*rec).name.as_ptr());
    }
    /* #endif */
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_remove(dev: *mut snd_seq_device) {
    let mut index: c_int;
    let rec: *mut seq_oss_synth = (*dev).driver_data as *mut seq_oss_synth;

    let flags = lock_register_irqsave();
    index = 0;
    while index < max_synth_devs {
        if synth_devs[index as usize] == rec {
            break;
        }
        index += 1;
    }
    if index >= max_synth_devs {
        pr_err(b"ALSA: seq_oss: can't unregister synth\n\0".as_ptr() as *const c_char);
        unlock_register_irqrestore(flags);
        return;
    }
    synth_devs[index as usize] = ptr::null_mut();
    if index == max_synth_devs - 1 {
        index -= 1;
        while index >= 0 {
            if !synth_devs[index as usize].is_null() {
                break;
            }
            index -= 1;
        }
        max_synth_devs = index + 1;
    }
    unlock_register_irqrestore(flags);

    /* #ifdef SNDRV_OSS_INFO_DEV_SYNTH */
    if (*rec).seq_device < SNDRV_CARDS {
        snd_oss_info_unregister(SNDRV_OSS_INFO_DEV_SYNTH, (*rec).seq_device);
    }
    /* #endif */

    snd_use_lock_sync(&mut (*rec).use_lock);
    kfree(rec as *mut c_void);
}

/*
 */
unsafe fn get_sdev(dev: c_int) -> *mut seq_oss_synth {
    let rec: *mut seq_oss_synth;

    let flags = lock_register_irqsave();
    rec = synth_devs[dev as usize];
    if !rec.is_null() {
        snd_use_lock_use(&mut (*rec).use_lock);
    }
    unlock_register_irqrestore(flags);
    rec
}

/*
 * set up synth tables
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_setup(dp: *mut seq_oss_devinfo) {
    let mut i: c_int;
    let mut info: *mut seq_oss_synthinfo;

    (*dp).max_synthdev = max_synth_devs;
    (*dp).synth_opened = 0;
    memset(
        (*dp).synths.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&(*dp).synths),
    );
    i = 0;
    while i < (*dp).max_synthdev {
        let rec: *mut seq_oss_synth = get_sdev(i);

        if rec.is_null() {
            i += 1;
            continue;
        }
        if (*rec).oper.open.is_none() || (*rec).oper.close.is_none() {
            snd_use_lock_free(&mut (*rec).use_lock);
            i += 1;
            continue;
        }
        info = &mut (*dp).synths[i as usize];
        (*info).arg.app_index = (*dp).port;
        (*info).arg.file_mode = (*dp).file_mode;
        (*info).arg.seq_mode = (*dp).seq_mode;
        if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_SYNTH {
            (*info).arg.event_passing = SNDRV_SEQ_OSS_PROCESS_EVENTS;
        } else {
            (*info).arg.event_passing = SNDRV_SEQ_OSS_PASS_EVENTS;
        }
        (*info).opened = 0;
        if !try_module_get((*rec).oper.owner) {
            snd_use_lock_free(&mut (*rec).use_lock);
            i += 1;
            continue;
        }
        if ((*rec).oper.open.unwrap())(&mut (*info).arg, (*rec).private_data) < 0 {
            module_put((*rec).oper.owner);
            snd_use_lock_free(&mut (*rec).use_lock);
            i += 1;
            continue;
        }
        (*info).nr_voices = (*rec).nr_voices;
        if (*info).nr_voices > 0 {
            (*info).ch = kzalloc(
                mem::size_of::<seq_oss_chinfo>() * (*info).nr_voices as usize,
                GFP_KERNEL,
            ) as *mut seq_oss_chinfo;
            if (*info).ch.is_null() {
                ((*rec).oper.close.unwrap())(&mut (*info).arg);
                module_put((*rec).oper.owner);
                snd_use_lock_free(&mut (*rec).use_lock);
                i += 1;
                continue;
            }
            reset_channels(info);
        }
        (*info).opened += 1;
        (*rec).opened += 1;
        (*dp).synth_opened += 1;
        snd_use_lock_free(&mut (*rec).use_lock);
        i += 1;
    }
}

/*
 * set up synth tables for MIDI emulation - /dev/music mode only
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_setup_midi(dp: *mut seq_oss_devinfo) {
    let mut i: c_int;

    if (*dp).max_synthdev >= SNDRV_SEQ_OSS_MAX_SYNTH_DEVS as c_int {
        return;
    }

    i = 0;
    while i < (*dp).max_mididev {
        let info: *mut seq_oss_synthinfo;
        info = &mut (*dp).synths[(*dp).max_synthdev as usize];
        if snd_seq_oss_midi_open(dp, i, (*dp).file_mode) < 0 {
            i += 1;
            continue;
        }
        (*info).arg.app_index = (*dp).port;
        (*info).arg.file_mode = (*dp).file_mode;
        (*info).arg.seq_mode = (*dp).seq_mode;
        (*info).arg.private_data = info as *mut c_void;
        (*info).is_midi = 1;
        (*info).midi_mapped = i;
        (*info).arg.event_passing = SNDRV_SEQ_OSS_PASS_EVENTS;
        snd_seq_oss_midi_get_addr(dp, i, &mut (*info).arg.addr);
        (*info).opened = 1;
        midi_synth_dev.opened += 1;
        (*dp).max_synthdev += 1;
        if (*dp).max_synthdev >= SNDRV_SEQ_OSS_MAX_SYNTH_DEVS as c_int {
            break;
        }
        i += 1;
    }
}

/*
 * clean up synth tables
 */

#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_cleanup(dp: *mut seq_oss_devinfo) {
    let mut i: c_int;
    let mut info: *mut seq_oss_synthinfo;

    if snd_BUG_ON((*dp).max_synthdev > SNDRV_SEQ_OSS_MAX_SYNTH_DEVS as c_int) {
        return;
    }
    i = 0;
    while i < (*dp).max_synthdev {
        info = &mut (*dp).synths[i as usize];
        if (*info).opened == 0 {
            i += 1;
            continue;
        }
        if (*info).is_midi != 0 {
            if midi_synth_dev.opened > 0 {
                snd_seq_oss_midi_close(dp, (*info).midi_mapped);
                midi_synth_dev.opened -= 1;
            }
        } else {
            let rec: *mut seq_oss_synth = get_sdev(i);

            if rec.is_null() {
                i += 1;
                continue;
            }
            if (*rec).opened > 0 {
                ((*rec).oper.close.unwrap())(&mut (*info).arg);
                module_put((*rec).oper.owner);
                (*rec).opened = 0;
            }
            snd_use_lock_free(&mut (*rec).use_lock);
        }
        kfree((*info).ch as *mut c_void);
        (*info).ch = ptr::null_mut();
        i += 1;
    }
    (*dp).synth_opened = 0;
    (*dp).max_synthdev = 0;
}

unsafe fn get_synthinfo_nospec(
    dp: *mut seq_oss_devinfo,
    mut dev: c_int,
) -> *mut seq_oss_synthinfo {
    if dev < 0 || dev >= (*dp).max_synthdev {
        return ptr::null_mut();
    }
    dev = array_index_nospec(dev, SNDRV_SEQ_OSS_MAX_SYNTH_DEVS);
    &mut (*dp).synths[dev as usize]
}

/*
 * return synth device information pointer
 */
unsafe fn get_synthdev(dp: *mut seq_oss_devinfo, dev: c_int) -> *mut seq_oss_synth {
    let rec: *mut seq_oss_synth;
    let info: *mut seq_oss_synthinfo = get_synthinfo_nospec(dp, dev);

    if info.is_null() {
        return ptr::null_mut();
    }
    if (*info).opened == 0 {
        return ptr::null_mut();
    }
    if (*info).is_midi != 0 {
        rec = &mut midi_synth_dev;
        snd_use_lock_use(&mut (*rec).use_lock);
    } else {
        rec = get_sdev(dev);
        if rec.is_null() {
            return ptr::null_mut();
        }
    }
    if (*rec).opened == 0 {
        snd_use_lock_free(&mut (*rec).use_lock);
        return ptr::null_mut();
    }
    rec
}

/*
 * reset note and velocity on each channel.
 */
unsafe fn reset_channels(info: *mut seq_oss_synthinfo) {
    let mut i: c_int;
    if (*info).ch.is_null() || (*info).nr_voices == 0 {
        return;
    }
    i = 0;
    while i < (*info).nr_voices {
        (*(*info).ch.add(i as usize)).note = -1;
        (*(*info).ch.add(i as usize)).vel = 0;
        i += 1;
    }
}

/*
 * reset synth device:
 * call reset callback.  if no callback is defined, send a heartbeat
 * event to the corresponding port.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_reset(dp: *mut seq_oss_devinfo, dev: c_int) {
    let info: *mut seq_oss_synthinfo;

    info = get_synthinfo_nospec(dp, dev);
    if info.is_null() || (*info).opened == 0 {
        return;
    }
    reset_channels(info);
    if (*info).is_midi != 0 {
        if midi_synth_dev.opened <= 0 {
            return;
        }
        snd_seq_oss_midi_reset(dp, (*info).midi_mapped);
        /* reopen the device */
        snd_seq_oss_midi_close(dp, dev);
        if snd_seq_oss_midi_open(dp, (*info).midi_mapped, (*dp).file_mode) < 0 {
            midi_synth_dev.opened -= 1;
            (*info).opened = 0;
            kfree((*info).ch as *mut c_void);
            (*info).ch = ptr::null_mut();
        }
        return;
    }

    let rec: *mut seq_oss_synth = get_sdev(dev);
    if rec.is_null() {
        return;
    }
    if let Some(reset) = (*rec).oper.reset {
        reset(&mut (*info).arg);
    } else {
        let mut ev: snd_seq_event = mem::zeroed();
        memset(
            &mut ev as *mut snd_seq_event as *mut c_void,
            0,
            mem::size_of::<snd_seq_event>(),
        );
        snd_seq_oss_fill_addr(
            dp,
            &mut ev,
            (*info).arg.addr.client,
            (*info).arg.addr.port,
        );
        ev.type_ = SNDRV_SEQ_EVENT_RESET;
        snd_seq_oss_dispatch(dp, &mut ev, 0, 0);
    }
    snd_use_lock_free(&mut (*rec).use_lock);
}

/*
 * load a patch record:
 * call load_patch callback function
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_load_patch(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    fmt: c_int,
    buf: *const c_char,
    p: c_int,
    c: c_int,
) -> c_int {
    let info: *mut seq_oss_synthinfo;

    info = get_synthinfo_nospec(dp, dev);
    if info.is_null() {
        return -ENXIO;
    }

    if (*info).is_midi != 0 {
        return 0;
    }

    let rec: *mut seq_oss_synth = get_synthdev(dp, dev);
    if rec.is_null() {
        return -ENXIO;
    }

    let ret = if let Some(load_patch) = (*rec).oper.load_patch {
        load_patch(&mut (*info).arg, fmt, buf, p, c)
    } else {
        -ENXIO
    };
    snd_use_lock_free(&mut (*rec).use_lock);
    ret
}

/*
 * check if the device is valid synth device and return the synth info
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_info(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
) -> *mut seq_oss_synthinfo {
    let rec: *mut seq_oss_synth = get_synthdev(dp, dev);

    if !rec.is_null() {
        snd_use_lock_free(&mut (*rec).use_lock);
        return get_synthinfo_nospec(dp, dev);
    }
    ptr::null_mut()
}

/*
 * receive OSS 6 byte sysex packet:
 * the event is filled and prepared for sending immediately
 * (i.e. sysex messages are fragmented)
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_sysex(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    buf: *mut u8,
    ev: *mut snd_seq_event,
) -> c_int {
    let p: *mut u8;
    let mut len: c_int = 6;

    p = memchr(buf as *const c_void, 0xff, 6) as *mut u8;
    if !p.is_null() {
        len = p.offset_from(buf) as c_int + 1;
    }

    /* copy the data to event record and send it */
    if snd_seq_oss_synth_addr(dp, dev, ev) != 0 {
        return -EINVAL;
    }
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_VARIABLE;
    (*ev).data.ext.len = len;
    (*ev).data.ext.ptr = buf as *mut c_void;
    0
}

/*
 * fill the event source/destination addresses
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_addr(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    ev: *mut snd_seq_event,
) -> c_int {
    let info: *mut seq_oss_synthinfo = snd_seq_oss_synth_info(dp, dev);

    if info.is_null() {
        return -EINVAL;
    }
    snd_seq_oss_fill_addr(dp, ev, (*info).arg.addr.client, (*info).arg.addr.port);
    0
}

/*
 * OSS compatible ioctl
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_ioctl(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    cmd: c_uint,
    addr: c_ulong,
) -> c_int {
    let info: *mut seq_oss_synthinfo;

    info = get_synthinfo_nospec(dp, dev);
    if info.is_null() || (*info).is_midi != 0 {
        return -ENXIO;
    }

    let rec: *mut seq_oss_synth = get_synthdev(dp, dev);
    if rec.is_null() {
        return -ENXIO;
    }
    let ret = if let Some(ioctl) = (*rec).oper.ioctl {
        ioctl(&mut (*info).arg, cmd, addr)
    } else {
        -ENXIO
    };
    snd_use_lock_free(&mut (*rec).use_lock);
    ret
}

/*
 * send OSS raw events - SEQ_PRIVATE and SEQ_VOLUME
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_raw_event(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    data: *mut u8,
    ev: *mut snd_seq_event,
) -> c_int {
    let info: *mut seq_oss_synthinfo;

    info = snd_seq_oss_synth_info(dp, dev);
    if info.is_null() || (*info).is_midi != 0 {
        return -ENXIO;
    }
    (*ev).type_ = SNDRV_SEQ_EVENT_OSS;
    memcpy(
        (*ev).data.raw8.d.as_mut_ptr() as *mut c_void,
        data as *const c_void,
        8,
    );
    snd_seq_oss_synth_addr(dp, dev, ev)
}

/*
 * create OSS compatible synth_info record
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_make_info(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    inf: *mut synth_info,
) -> c_int {
    let info: *mut seq_oss_synthinfo = get_synthinfo_nospec(dp, dev);

    if info.is_null() {
        return -ENXIO;
    }

    if (*info).is_midi != 0 {
        let mut minf: midi_info = mem::zeroed();
        if snd_seq_oss_midi_make_info(dp, (*info).midi_mapped, &mut minf) != 0 {
            return -ENXIO;
        }
        (*inf).synth_type = SYNTH_TYPE_MIDI;
        (*inf).synth_subtype = 0;
        (*inf).nr_voices = 16;
        (*inf).device = dev;
        strscpy((*inf).name.as_mut_ptr(), minf.name.as_ptr(), (*inf).name.len());
    } else {
        let rec: *mut seq_oss_synth = get_synthdev(dp, dev);

        if rec.is_null() {
            return -ENXIO;
        }
        (*inf).synth_type = (*rec).synth_type;
        (*inf).synth_subtype = (*rec).synth_subtype;
        (*inf).nr_voices = (*rec).nr_voices;
        (*inf).device = dev;
        strscpy((*inf).name.as_mut_ptr(), (*rec).name.as_ptr(), (*inf).name.len());
        snd_use_lock_free(&mut (*rec).use_lock);
    }
    0
}

/* #ifdef CONFIG_SND_PROC_FS */
/*
 * proc interface
 */
#[cfg(CONFIG_SND_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_synth_info_read(buf: *mut snd_info_buffer) {
    let mut i: c_int;

    snd_iprintf(
        buf,
        b"\nNumber of synth devices: %d\n\0".as_ptr() as *const c_char,
        max_synth_devs,
    );
    i = 0;
    while i < max_synth_devs {
        snd_iprintf(buf, b"\nsynth %d: \0".as_ptr() as *const c_char, i);
        let rec: *mut seq_oss_synth = get_sdev(i);
        if rec.is_null() {
            snd_iprintf(buf, b"*empty*\n\0".as_ptr() as *const c_char);
            i += 1;
            continue;
        }
        snd_iprintf(
            buf,
            b"[%s]\n\0".as_ptr() as *const c_char,
            (*rec).name.as_ptr(),
        );
        snd_iprintf(
            buf,
            b"  type 0x%x : subtype 0x%x : voices %d\n\0".as_ptr() as *const c_char,
            (*rec).synth_type,
            (*rec).synth_subtype,
            (*rec).nr_voices,
        );
        snd_iprintf(
            buf,
            b"  capabilities : ioctl %s / load_patch %s\n\0".as_ptr() as *const c_char,
            str_enabled_disabled((*rec).oper.ioctl.map_or(0, |f| f as usize as c_long)),
            str_enabled_disabled((*rec).oper.load_patch.map_or(0, |f| f as usize as c_long)),
        );
        snd_use_lock_free(&mut (*rec).use_lock);
        i += 1;
    }
}
/* #endif CONFIG_SND_PROC_FS */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
