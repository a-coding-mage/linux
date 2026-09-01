/*
 * OPL4 sequencer functions
 *
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed and/or modified under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation; either version 2 of the License, or (at your option) any later
 * version.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

// C dependencies: "opl4_local.h", <linux/init.h>, <linux/moduleparam.h>,
// <linux/module.h>, <sound/initval.h>.
// Module metadata and parameter declarations from C:
// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_DESCRIPTION("OPL4 wavetable synth driver");
// MODULE_LICENSE("Dual BSD/GPL");
// module_param(volume_boost, int, 0644);
// MODULE_PARM_DESC(volume_boost, "Additional volume for OPL4 wavetable sounds.");

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EINVAL: c_int = 22;
pub const ENODEV: c_int = 19;
pub const ENOMEM: c_int = 12;

extern "C" {
    static THIS_MODULE: *mut module;
    static KBUILD_MODNAME: *const c_char;

    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    fn snd_opl4_synth_reset(opl4: *mut snd_opl4);
    fn snd_opl4_synth_shutdown(opl4: *mut snd_opl4);
    fn snd_opl4_note_on(chset: *mut snd_midi_channel_set, channel: c_int, note: c_int, vel: c_int);
    fn snd_opl4_note_off(chset: *mut snd_midi_channel_set, channel: c_int, note: c_int, vel: c_int);
    fn snd_opl4_terminate_note(chset: *mut snd_midi_channel_set, channel: c_int, note: c_int);
    fn snd_opl4_control(chset: *mut snd_midi_channel_set, channel: c_int, control: c_int, value: c_int);
    fn snd_opl4_sysex(chset: *mut snd_midi_channel_set, buf: *mut u8, len: c_int);

    fn snd_midi_process_event(
        ops: *const snd_midi_op,
        ev: *mut snd_seq_event,
        chset: *mut snd_midi_channel_set,
    );
    fn snd_midi_channel_free_set(chset: *mut snd_midi_channel_set);
    fn snd_midi_channel_alloc_set(channels: c_int) -> *mut snd_midi_channel_set;
    fn snd_yrw801_detect(opl4: *mut snd_opl4) -> c_int;
    fn snd_seq_create_kernel_client(card: *mut snd_card, device: c_int, name: *const c_char) -> c_int;
    fn snd_seq_event_port_attach(
        client: c_int,
        callbacks: *mut snd_seq_port_callback,
        cap: c_int,
        type_: c_int,
        midi_channels: c_int,
        midi_voices: c_int,
        name: *const c_char,
    ) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut c_void;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub module: *mut module,
}

#[repr(C)]
pub struct snd_opl4 {
    pub card: *mut snd_card,
    pub access_mutex: mutex,
    pub used: c_int,
    pub chset: *mut snd_midi_channel_set,
    pub seq_dev_num: c_int,
    pub seq_client: c_int,
}

#[repr(C)]
pub struct snd_seq_port_subscribe_addr {
    pub client: c_int,
}

#[repr(C)]
pub struct snd_seq_port_subscribe {
    pub sender: snd_seq_port_subscribe_addr,
}

#[repr(C)]
pub struct snd_seq_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_device {
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
    pub note_on: Option<unsafe extern "C" fn(*mut snd_midi_channel_set, c_int, c_int, c_int)>,
    pub note_off: Option<unsafe extern "C" fn(*mut snd_midi_channel_set, c_int, c_int, c_int)>,
    pub note_terminate: Option<unsafe extern "C" fn(*mut snd_midi_channel_set, c_int, c_int)>,
    pub control: Option<unsafe extern "C" fn(*mut snd_midi_channel_set, c_int, c_int, c_int)>,
    pub sysex: Option<unsafe extern "C" fn(*mut snd_midi_channel_set, *mut u8, c_int)>,
}

#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub use_: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub unuse: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub event_input:
        Option<unsafe extern "C" fn(*mut snd_seq_event, c_int, *mut c_void, c_int, c_int) -> c_int>,
    pub private_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_seq_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_seq_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_seq_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
    pub driver: snd_seq_driver_inner,
    pub id: *const c_char,
    pub argsize: usize,
}

pub const SNDRV_SEQ_CLIENT_SYSTEM: c_int = 0;
pub const SNDRV_SEQ_PORT_CAP_WRITE: c_int = 1 << 0;
pub const SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_int = 1 << 1;
pub const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: c_int = 1 << 1;
pub const SNDRV_SEQ_PORT_TYPE_MIDI_GM: c_int = 1 << 2;
pub const SNDRV_SEQ_PORT_TYPE_HARDWARE: c_int = 1 << 3;
pub const SNDRV_SEQ_PORT_TYPE_SYNTHESIZER: c_int = 1 << 4;

pub static SNDRV_SEQ_DEV_ID_OPL4: &[u8; 5] = b"opl4\0";

pub static mut volume_boost: c_int = 8;

unsafe extern "C" fn snd_opl4_seq_use_inc(opl4: *mut snd_opl4) -> c_int {
    if try_module_get((*(*opl4).card).module) == 0 {
        return -EFAULT;
    }
    0
}

unsafe extern "C" fn snd_opl4_seq_use_dec(opl4: *mut snd_opl4) {
    module_put((*(*opl4).card).module);
}

unsafe extern "C" fn snd_opl4_seq_use(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    let opl4 = private_data as *mut snd_opl4;
    let err: c_int;

    mutex_lock(&mut (*opl4).access_mutex);
    if (*opl4).used != 0 {
        mutex_unlock(&mut (*opl4).access_mutex);
        return -EBUSY;
    }
    (*opl4).used += 1;

    if (*info).sender.client != SNDRV_SEQ_CLIENT_SYSTEM {
        err = snd_opl4_seq_use_inc(opl4);
        if err < 0 {
            mutex_unlock(&mut (*opl4).access_mutex);
            return err;
        }
    }
    mutex_unlock(&mut (*opl4).access_mutex);

    snd_opl4_synth_reset(opl4);
    0
}

unsafe extern "C" fn snd_opl4_seq_unuse(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    let opl4 = private_data as *mut snd_opl4;

    snd_opl4_synth_shutdown(opl4);

    mutex_lock(&mut (*opl4).access_mutex);
    (*opl4).used -= 1;
    mutex_unlock(&mut (*opl4).access_mutex);

    if (*info).sender.client != SNDRV_SEQ_CLIENT_SYSTEM {
        snd_opl4_seq_use_dec(opl4);
    }
    0
}

static opl4_ops: snd_midi_op = snd_midi_op {
    note_on: Some(snd_opl4_note_on),
    note_off: Some(snd_opl4_note_off),
    note_terminate: Some(snd_opl4_terminate_note),
    control: Some(snd_opl4_control),
    sysex: Some(snd_opl4_sysex),
};

unsafe extern "C" fn snd_opl4_seq_event_input(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
    _atomic: c_int,
    _hop: c_int,
) -> c_int {
    let opl4 = private_data as *mut snd_opl4;

    snd_midi_process_event(&opl4_ops, ev, (*opl4).chset);
    0
}

unsafe extern "C" fn snd_opl4_seq_free_port(private_data: *mut c_void) {
    let opl4 = private_data as *mut snd_opl4;

    snd_midi_channel_free_set((*opl4).chset);
}

unsafe extern "C" fn snd_opl4_seq_probe(dev: *mut snd_seq_device) -> c_int {
    let opl4: *mut snd_opl4;
    let client: c_int;
    let mut pcallbacks: snd_seq_port_callback;

    opl4 = *(SNDRV_SEQ_DEVICE_ARGPTR(dev) as *mut *mut snd_opl4);
    if opl4.is_null() {
        return -EINVAL;
    }

    if snd_yrw801_detect(opl4) < 0 {
        return -ENODEV;
    }

    (*opl4).chset = snd_midi_channel_alloc_set(16);
    if (*opl4).chset.is_null() {
        return -ENOMEM;
    }
    (*(*opl4).chset).private_data = opl4 as *mut c_void;

    /* allocate new client */
    client = snd_seq_create_kernel_client((*opl4).card, (*opl4).seq_dev_num, b"OPL4 Wavetable\0".as_ptr() as *const c_char);
    if client < 0 {
        snd_midi_channel_free_set((*opl4).chset);
        return client;
    }
    (*opl4).seq_client = client;
    (*(*opl4).chset).client = client;

    /* create new port */
    pcallbacks = mem::zeroed();
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.use_ = Some(snd_opl4_seq_use);
    pcallbacks.unuse = Some(snd_opl4_seq_unuse);
    pcallbacks.event_input = Some(snd_opl4_seq_event_input);
    pcallbacks.private_free = Some(snd_opl4_seq_free_port);
    pcallbacks.private_data = opl4 as *mut c_void;

    (*(*opl4).chset).port = snd_seq_event_port_attach(
        client,
        &mut pcallbacks,
        SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE,
        SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
            | SNDRV_SEQ_PORT_TYPE_MIDI_GM
            | SNDRV_SEQ_PORT_TYPE_HARDWARE
            | SNDRV_SEQ_PORT_TYPE_SYNTHESIZER,
        16,
        24,
        b"OPL4 Wavetable Port\0".as_ptr() as *const c_char,
    );
    if (*(*opl4).chset).port < 0 {
        let err = (*(*opl4).chset).port;
        snd_midi_channel_free_set((*opl4).chset);
        snd_seq_delete_kernel_client(client);
        (*opl4).seq_client = -1;
        return err;
    }
    0
}

unsafe extern "C" fn snd_opl4_seq_remove(dev: *mut snd_seq_device) {
    let opl4: *mut snd_opl4;

    opl4 = *(SNDRV_SEQ_DEVICE_ARGPTR(dev) as *mut *mut snd_opl4);
    if opl4.is_null() {
        return;
    }

    if (*opl4).seq_client >= 0 {
        snd_seq_delete_kernel_client((*opl4).seq_client);
        (*opl4).seq_client = -1;
    }
}

static mut opl4_seq_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_opl4_seq_probe),
    remove: Some(snd_opl4_seq_remove),
    driver: snd_seq_driver_inner {
        name: ptr::null(),
    },
    id: SNDRV_SEQ_DEV_ID_OPL4.as_ptr() as *const c_char,
    argsize: mem::size_of::<*mut snd_opl4>(),
};

// C registration macro:
// module_snd_seq_driver(opl4_seq_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
