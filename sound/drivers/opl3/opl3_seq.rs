// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Uros Bizjak <uros@kss-loka.si>
 *
 *  Midi Sequencer interface routines for OPL2/OPL3/OPL4 FM
 *
 *  OPL2/3 FM instrument loader:
 *   alsa-tools/seq/sbiload/
 */

// Original C dependencies:
// #include "opl3_voice.h"
// #include <linux/init.h>
// #include <linux/moduleparam.h>
// #include <linux/module.h>
// #include <sound/initval.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

// MODULE_AUTHOR("Uros Bizjak <uros@kss-loka.si>");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ALSA driver for OPL3 FM synth");

pub static mut use_internal_drums: bool = false;
// module_param(use_internal_drums, bool, 0444);
// MODULE_PARM_DESC(use_internal_drums, "Enable internal OPL2/3 drums.");

const EFAULT: c_int = 14;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static THIS_MODULE: *mut module;
    static KBUILD_MODNAME: *const c_char;

    static mut opl3_ops: snd_midi_op;

    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn timer_delete(timer: *mut timer_list);
    fn timer_setup(timer: *mut timer_list, func: timer_func_t, flags: c_uint);
    fn wake_up(wait: *mut wait_queue_head_t);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn snd_opl3_reset(opl3: *mut snd_opl3);
    fn snd_opl3_load_drums(opl3: *mut snd_opl3);
    fn snd_opl3_note_on();
    fn snd_opl3_note_off();
    fn snd_opl3_key_press();
    fn snd_opl3_terminate_note();
    fn snd_opl3_control();
    fn snd_opl3_nrpn();
    fn snd_opl3_sysex();
    fn snd_midi_process_event(ops: *const snd_midi_op, ev: *mut snd_seq_event, chset: *mut snd_midi_channel_set);
    fn snd_midi_channel_alloc_set(n: c_int) -> *mut snd_midi_channel_set;
    fn snd_midi_channel_free_set(chset: *mut snd_midi_channel_set);
    fn snd_seq_event_port_attach(
        client: c_int,
        callbacks: *mut snd_seq_port_callback,
        cap: c_uint,
        typ: c_uint,
        midi_channels: c_int,
        midi_voices: c_int,
        name: *mut c_char,
    ) -> c_int;
    fn snd_seq_create_kernel_client(card: *mut snd_card, device: c_int, name: *mut c_char) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn snd_opl3_timer_func(timer: *mut timer_list);
    fn snd_opl3_init_seq_oss(opl3: *mut snd_opl3, name: *mut c_char);
    fn snd_opl3_free_seq_oss(opl3: *mut snd_opl3);
    fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut c_void;
}

type timer_func_t = Option<unsafe extern "C" fn(*mut timer_list)>;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub module: *mut module,
}

#[repr(C)]
pub struct snd_hwdep {
    pub open_mutex: mutex,
    pub used: c_int,
    pub open_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct snd_opl3_voice {
    pub state: c_int,
    pub time: c_int,
    pub keyon_reg: u8,
}

#[repr(C)]
pub struct snd_opl3 {
    pub card: *mut snd_card,
    pub hwdep: *mut snd_hwdep,
    pub voices: [snd_opl3_voice; MAX_OPL3_VOICES],
    pub use_time: c_int,
    pub connection_reg: u8,
    pub hardware: c_int,
    pub command: Option<unsafe extern "C" fn(*mut snd_opl3, c_int, u8)>,
    pub max_voices: c_int,
    pub sys_timer_lock: spinlock_t,
    pub sys_timer_status: c_int,
    pub tlist: timer_list,
    pub drum_reg: u8,
    pub synth_mode: c_int,
    pub chset: *mut snd_midi_channel_set,
    pub voice_lock: spinlock_t,
    pub seq_client: c_int,
    pub seq_dev_num: c_int,
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_int,
}

#[repr(C)]
pub struct snd_seq_port_subscribe {
    pub sender: snd_seq_addr,
}

#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_channel_set {
    pub private_data: *mut c_void,
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct snd_midi_op {
    pub note_on: Option<unsafe extern "C" fn()>,
    pub note_off: Option<unsafe extern "C" fn()>,
    pub key_press: Option<unsafe extern "C" fn()>,
    pub note_terminate: Option<unsafe extern "C" fn()>,
    pub control: Option<unsafe extern "C" fn()>,
    pub nrpn: Option<unsafe extern "C" fn()>,
    pub sysex: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub use_: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub unuse: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub event_input: Option<unsafe extern "C" fn(*mut snd_seq_event, c_int, *mut c_void, c_int, c_int) -> c_int>,
    pub private_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_seq_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_seq_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_seq_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
    pub driver: device_driver,
    pub id: c_int,
    pub argsize: usize,
}

const MAX_OPL3_VOICES: usize = 18;
const MAX_OPL2_VOICES: c_int = 9;
const SNDRV_OPL3_ST_OFF: c_int = 0;
const SNDRV_OPL3_ST_NOT_AVAIL: c_int = 1;
const OPL3_HW_OPL3: c_int = 0x0300;
const OPL3_HW_MASK: c_int = 0xff00;
const OPL3_RIGHT: c_int = 0x100;
const OPL3_LEFT: c_int = 0x000;
const OPL3_REG_CONNECTION_SELECT: c_int = 0x04;
const OPL3_REG_PERCUSSION: c_int = 0xbd;
const OPL3_PERCUSSION_ENABLE: u8 = 0x20;
const SNDRV_SEQ_CLIENT_SYSTEM: c_int = 0;
const SNDRV_OPL3_MODE_SEQ: c_int = 1;
const SNDRV_SEQ_PORT_CAP_WRITE: c_uint = 1 << 0;
const SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: c_uint = 1 << 0;
const SNDRV_SEQ_PORT_TYPE_MIDI_GM: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_TYPE_DIRECT_SAMPLE: c_uint = 1 << 2;
const SNDRV_SEQ_PORT_TYPE_HARDWARE: c_uint = 1 << 3;
const SNDRV_SEQ_PORT_TYPE_SYNTHESIZER: c_uint = 1 << 4;
const SNDRV_SEQ_DEV_ID_OPL3: c_int = 0;

pub unsafe extern "C" fn snd_opl3_synth_use_inc(opl3: *mut snd_opl3) -> c_int {
    if !try_module_get((*(*opl3).card).module) {
        return -EFAULT;
    }
    return 0;
}

pub unsafe extern "C" fn snd_opl3_synth_use_dec(opl3: *mut snd_opl3) {
    module_put((*(*opl3).card).module);
}

pub unsafe extern "C" fn snd_opl3_synth_setup(opl3: *mut snd_opl3) -> c_int {
    let mut idx: c_int;
    let hwdep: *mut snd_hwdep = (*opl3).hwdep;

    mutex_lock(&mut (*hwdep).open_mutex);
    if (*hwdep).used != 0 {
        mutex_unlock(&mut (*hwdep).open_mutex);
        return -EBUSY;
    }
    (*hwdep).used += 1;
    mutex_unlock(&mut (*hwdep).open_mutex);

    snd_opl3_reset(opl3);

    idx = 0;
    while idx < MAX_OPL3_VOICES as c_int {
        (*opl3).voices[idx as usize].state = SNDRV_OPL3_ST_OFF;
        (*opl3).voices[idx as usize].time = 0;
        (*opl3).voices[idx as usize].keyon_reg = 0x00;
        idx += 1;
    }
    (*opl3).use_time = 0;
    (*opl3).connection_reg = 0x00;
    if (*opl3).hardware >= OPL3_HW_OPL3 {
        /* Clear 4-op connections */
        if let Some(command) = (*opl3).command {
            command(
                opl3,
                OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT,
                (*opl3).connection_reg,
            );
        }
        (*opl3).max_voices = MAX_OPL3_VOICES as c_int;
    }
    return 0;
}

pub unsafe extern "C" fn snd_opl3_synth_cleanup(opl3: *mut snd_opl3) {
    let hwdep: *mut snd_hwdep;

    /* Stop system timer */
    spin_lock_irq(&mut (*opl3).sys_timer_lock);
    if (*opl3).sys_timer_status != 0 {
        timer_delete(&mut (*opl3).tlist);
        (*opl3).sys_timer_status = 0;
    }
    spin_unlock_irq(&mut (*opl3).sys_timer_lock);

    snd_opl3_reset(opl3);
    hwdep = (*opl3).hwdep;
    mutex_lock(&mut (*hwdep).open_mutex);
    (*hwdep).used -= 1;
    mutex_unlock(&mut (*hwdep).open_mutex);
    wake_up(&mut (*hwdep).open_wait);
}

unsafe extern "C" fn snd_opl3_synth_use(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    let opl3: *mut snd_opl3 = private_data as *mut snd_opl3;
    let mut err: c_int;

    err = snd_opl3_synth_setup(opl3);
    if err < 0 {
        return err;
    }

    if use_internal_drums {
        /* Percussion mode */
        (*opl3).voices[8].state = SNDRV_OPL3_ST_NOT_AVAIL;
        (*opl3).voices[7].state = (*opl3).voices[8].state;
        (*opl3).voices[6].state = (*opl3).voices[7].state;
        snd_opl3_load_drums(opl3);
        (*opl3).drum_reg = OPL3_PERCUSSION_ENABLE;
        if let Some(command) = (*opl3).command {
            command(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, (*opl3).drum_reg);
        }
    } else {
        (*opl3).drum_reg = 0x00;
    }

    if (*info).sender.client != SNDRV_SEQ_CLIENT_SYSTEM {
        err = snd_opl3_synth_use_inc(opl3);
        if err < 0 {
            return err;
        }
    }
    (*opl3).synth_mode = SNDRV_OPL3_MODE_SEQ;
    return 0;
}

unsafe extern "C" fn snd_opl3_synth_unuse(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    let opl3: *mut snd_opl3 = private_data as *mut snd_opl3;

    snd_opl3_synth_cleanup(opl3);

    if (*info).sender.client != SNDRV_SEQ_CLIENT_SYSTEM {
        snd_opl3_synth_use_dec(opl3);
    }
    return 0;
}

/*
 * MIDI emulation operators
 */
#[no_mangle]
pub static opl3_ops: snd_midi_op = snd_midi_op {
    note_on: Some(snd_opl3_note_on),
    note_off: Some(snd_opl3_note_off),
    key_press: Some(snd_opl3_key_press),
    note_terminate: Some(snd_opl3_terminate_note),
    control: Some(snd_opl3_control),
    nrpn: Some(snd_opl3_nrpn),
    sysex: Some(snd_opl3_sysex),
};

unsafe extern "C" fn snd_opl3_synth_event_input(
    ev: *mut snd_seq_event,
    direct: c_int,
    private_data: *mut c_void,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    let opl3: *mut snd_opl3 = private_data as *mut snd_opl3;

    let _ = direct;
    let _ = atomic;
    let _ = hop;
    snd_midi_process_event(&opl3_ops, ev, (*opl3).chset);
    return 0;
}

/* ------------------------------ */

unsafe extern "C" fn snd_opl3_synth_free_port(private_data: *mut c_void) {
    let opl3: *mut snd_opl3 = private_data as *mut snd_opl3;

    snd_midi_channel_free_set((*opl3).chset);
}

unsafe extern "C" fn snd_opl3_synth_create_port(opl3: *mut snd_opl3) -> c_int {
    let mut callbacks: snd_seq_port_callback = core::mem::zeroed();
    let mut name: [c_char; 32] = [0; 32];
    let voices: c_int;
    let opl_ver: c_int;

    voices = if (*opl3).hardware < OPL3_HW_OPL3 {
        MAX_OPL2_VOICES
    } else {
        MAX_OPL3_VOICES as c_int
    };
    (*opl3).chset = snd_midi_channel_alloc_set(16);
    if (*opl3).chset.is_null() {
        return -ENOMEM;
    }
    (*(*opl3).chset).private_data = opl3 as *mut c_void;

    memset(
        &mut callbacks as *mut snd_seq_port_callback as *mut c_void,
        0,
        size_of::<snd_seq_port_callback>(),
    );
    callbacks.owner = THIS_MODULE;
    callbacks.use_ = Some(snd_opl3_synth_use);
    callbacks.unuse = Some(snd_opl3_synth_unuse);
    callbacks.event_input = Some(snd_opl3_synth_event_input);
    callbacks.private_free = Some(snd_opl3_synth_free_port);
    callbacks.private_data = opl3 as *mut c_void;

    opl_ver = ((*opl3).hardware & OPL3_HW_MASK) >> 8;
    sprintf(name.as_mut_ptr(), b"OPL%i FM Port\0".as_ptr() as *const c_char, opl_ver);

    (*(*opl3).chset).client = (*opl3).seq_client;
    (*(*opl3).chset).port = snd_seq_event_port_attach(
        (*opl3).seq_client,
        &mut callbacks,
        SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE,
        SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
            | SNDRV_SEQ_PORT_TYPE_MIDI_GM
            | SNDRV_SEQ_PORT_TYPE_DIRECT_SAMPLE
            | SNDRV_SEQ_PORT_TYPE_HARDWARE
            | SNDRV_SEQ_PORT_TYPE_SYNTHESIZER,
        16,
        voices,
        name.as_mut_ptr(),
    );
    if (*(*opl3).chset).port < 0 {
        let port: c_int;
        port = (*(*opl3).chset).port;
        snd_midi_channel_free_set((*opl3).chset);
        return port;
    }
    return 0;
}

/* ------------------------------ */

unsafe extern "C" fn snd_opl3_seq_probe(dev: *mut snd_seq_device) -> c_int {
    let opl3: *mut snd_opl3;
    let client: c_int;
    let mut err: c_int;
    let mut name: [c_char; 32] = [0; 32];
    let opl_ver: c_int;

    opl3 = *(SNDRV_SEQ_DEVICE_ARGPTR(dev) as *mut *mut snd_opl3);
    if opl3.is_null() {
        return -EINVAL;
    }

    spin_lock_init(&mut (*opl3).voice_lock);

    (*opl3).seq_client = -1;

    /* allocate new client */
    opl_ver = ((*opl3).hardware & OPL3_HW_MASK) >> 8;
    sprintf(name.as_mut_ptr(), b"OPL%i FM synth\0".as_ptr() as *const c_char, opl_ver);
    (*opl3).seq_client = snd_seq_create_kernel_client((*opl3).card, (*opl3).seq_dev_num, name.as_mut_ptr());
    client = (*opl3).seq_client;
    if client < 0 {
        return client;
    }

    err = snd_opl3_synth_create_port(opl3);
    if err < 0 {
        snd_seq_delete_kernel_client(client);
        (*opl3).seq_client = -1;
        return err;
    }

    /* setup system timer */
    timer_setup(&mut (*opl3).tlist, Some(snd_opl3_timer_func), 0);
    spin_lock_init(&mut (*opl3).sys_timer_lock);
    (*opl3).sys_timer_status = 0;

    // #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
    snd_opl3_init_seq_oss(opl3, name.as_mut_ptr());
    // #endif
    return 0;
}

unsafe extern "C" fn snd_opl3_seq_remove(dev: *mut snd_seq_device) {
    let opl3: *mut snd_opl3;

    opl3 = *(SNDRV_SEQ_DEVICE_ARGPTR(dev) as *mut *mut snd_opl3);
    if opl3.is_null() {
        return;
    }

    // #if IS_ENABLED(CONFIG_SND_SEQUENCER_OSS)
    snd_opl3_free_seq_oss(opl3);
    // #endif
    if (*opl3).seq_client >= 0 {
        snd_seq_delete_kernel_client((*opl3).seq_client);
        (*opl3).seq_client = -1;
    }
}

static mut opl3_seq_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_opl3_seq_probe),
    remove: Some(snd_opl3_seq_remove),
    driver: device_driver {
        name: unsafe { KBUILD_MODNAME },
    },
    id: SNDRV_SEQ_DEV_ID_OPL3,
    argsize: size_of::<*mut snd_opl3>(),
};

// module_snd_seq_driver(opl3_seq_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
