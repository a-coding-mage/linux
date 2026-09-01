// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Generic MIDI synth driver for ALSA sequencer
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 *                         Jaroslav Kysela <perex@perex.cz>
 */

/*
Possible options for midisynth module:
	- automatic opening of midi ports on first received event or subscription
	  (close will be performed when client leaves)
*/

/* C dependencies:
 * <linux/init.h>, <linux/slab.h>, <linux/errno.h>, <linux/string.h>,
 * <linux/module.h>, <linux/mutex.h>, <sound/core.h>, <sound/rawmidi.h>,
 * <sound/seq_kernel.h>, <sound/seq_device.h>, <sound/seq_midi_event.h>,
 * <sound/initval.h>, "seq_lock.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

const PAGE_SIZE: c_int = 4096;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const MAX_MIDI_EVENT_BUF: c_int = 256;
const SNDRV_CARDS: usize = 32;
const SNDRV_RAWMIDI_DEVICES: usize = 8;
const SNDRV_RAWMIDI_LFLG_INPUT: c_int = 1;
const SNDRV_RAWMIDI_LFLG_OUTPUT: c_int = 2;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_uint = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_uint = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
const SNDRV_SEQ_ADDRESS_SUBSCRIBERS: c_int = 254;
const SNDRV_SEQ_EVENT_SYSEX: c_int = 130;
const SNDRV_SEQ_EVENT_LENGTH_MASK: c_uint = 0x0000000c;
const SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uint = 0x00000004;
const SNDRV_SEQ_PORT_FLG_GIVEN_PORT: c_uint = 1;
const SNDRV_SEQ_PORT_CAP_WRITE: c_uint = 1 << 0;
const SNDRV_SEQ_PORT_CAP_SYNC_WRITE: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_uint = 1 << 2;
const SNDRV_SEQ_PORT_CAP_READ: c_uint = 1 << 3;
const SNDRV_SEQ_PORT_CAP_SYNC_READ: c_uint = 1 << 4;
const SNDRV_SEQ_PORT_CAP_SUBS_READ: c_uint = 1 << 5;
const SNDRV_SEQ_PORT_CAP_DUPLEX: c_uint = 1 << 6;
const SNDRV_SEQ_PORT_DIR_INPUT: c_uint = 1;
const SNDRV_SEQ_PORT_DIR_OUTPUT: c_uint = 2;
const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: c_uint = 1 << 0;
const SNDRV_SEQ_PORT_TYPE_HARDWARE: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_TYPE_PORT: c_uint = 1 << 2;
const SNDRV_SEQ_IOCTL_CREATE_PORT: c_uint = 0;
const SNDRV_SEQ_DEV_ID_MIDISYNTH: c_int = 0;
const KBUILD_MODNAME: *const c_char = b"seq_midi\0".as_ptr() as *const c_char;
static mut output_buffer_size: c_int = PAGE_SIZE;
static mut input_buffer_size: c_int = PAGE_SIZE;

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub shortname: [c_char; 32],
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub get_port_info:
        Option<unsafe extern "C" fn(*mut snd_rawmidi, c_uint, *mut snd_seq_port_info)>,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub ops: *mut snd_rawmidi_ops,
}

#[repr(C)]
pub struct snd_rawmidi_runtime {
    pub avail: c_int,
    pub private_data: *mut c_void,
    pub event: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub runtime: *mut snd_rawmidi_runtime,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_file {
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
}

#[repr(C)]
pub struct snd_rawmidi_params {
    pub avail_min: c_int,
    pub buffer_size: c_int,
    pub no_active_sensing: c_int,
}

#[repr(C)]
pub struct snd_rawmidi_info {
    pub device: c_int,
    pub stream: c_uint,
    pub subdevice: c_uint,
    pub subdevices_count: c_uint,
    pub flags: c_uint,
    pub name: [c_char; 64],
    pub subname: [c_char; 64],
}

#[repr(C)]
pub struct snd_seq_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
pub struct snd_seq_event {
    pub type_: c_int,
    pub flags: c_uint,
    pub source: snd_seq_addr,
    pub dest: snd_seq_addr,
}

#[repr(C)]
pub struct snd_seq_port_subscribe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut c_void,
    pub private_data: *mut c_void,
    pub subscribe: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub unsubscribe:
        Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub use_: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub unuse: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub event_input:
        Option<unsafe extern "C" fn(*mut snd_seq_event, c_int, *mut c_void, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_seq_port_info {
    pub addr: snd_seq_addr,
    pub flags: c_uint,
    pub name: [c_char; 64],
    pub capability: c_uint,
    pub direction: c_uint,
    pub type_: c_uint,
    pub midi_channels: c_int,
    pub kernel: *mut snd_seq_port_callback,
}

#[repr(C)]
pub struct snd_seq_device {
    pub private_data: *mut c_void,
    pub card: *mut snd_card,
    pub device: c_int,
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
    pub id: c_int,
    pub argsize: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_use_lock_t {
    _private: c_int,
}

#[repr(C)]
pub struct mutex {
    _private: c_int,
}

#[repr(C)]
pub struct seq_midisynth {
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub device: c_int,
    pub subdevice: c_int,
    pub input_substream: *mut snd_rawmidi_substream,
    pub input_use_lock: snd_use_lock_t, /* in-flight event_input users */
    pub input_rfile: snd_rawmidi_file,
    pub output_use_lock: snd_use_lock_t, /* in-flight event_input users */
    pub output_substream: *mut snd_rawmidi_substream,
    pub output_rfile: snd_rawmidi_file,
    pub seq_client: c_int,
    pub seq_port: c_int,
    pub parser: *mut snd_midi_event,
}

#[repr(C)]
pub struct seq_midisynth_client {
    pub seq_client: c_int,
    pub num_ports: c_int,
    pub ports_per_device: [c_int; SNDRV_RAWMIDI_DEVICES],
    pub ports: [*mut seq_midisynth; SNDRV_RAWMIDI_DEVICES],
}

static mut synths: [*mut seq_midisynth_client; SNDRV_CARDS] = [null_mut(); SNDRV_CARDS];
static mut register_mutex: mutex = mutex { _private: 0 };
static mut THIS_MODULE: *mut c_void = null_mut();

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn printk_ratelimit() -> c_int;
    fn pr_err(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn snd_use_lock_init(lock: *mut snd_use_lock_t);
    fn snd_use_lock_use(lock: *mut snd_use_lock_t);
    fn snd_use_lock_free(lock: *mut snd_use_lock_t);
    fn snd_use_lock_sync(lock: *mut snd_use_lock_t);
    fn synchronize_rcu();
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn snd_rawmidi_kernel_read(
        substream: *mut snd_rawmidi_substream,
        buf: *mut c_char,
        count: c_int,
    ) -> c_long;
    fn snd_rawmidi_kernel_write(
        substream: *mut snd_rawmidi_substream,
        buf: *const c_char,
        count: c_int,
    ) -> c_int;
    fn snd_seq_kernel_client_dispatch(
        client: c_int,
        ev: *mut snd_seq_event,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;
    fn snd_midi_event_encode_byte(
        dev: *mut snd_midi_event,
        c: c_int,
        event: *mut snd_seq_event,
    ) -> c_int;
    fn snd_seq_dump_var_event(
        ev: *mut snd_seq_event,
        func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int>,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_midi_event_reset_decode(dev: *mut snd_midi_event);
    fn snd_midi_event_decode(
        dev: *mut snd_midi_event,
        buf: *mut u8,
        count: usize,
        ev: *mut snd_seq_event,
    ) -> c_int;
    fn snd_midi_event_new(bufsize: c_int, rdev: *mut *mut snd_midi_event) -> c_int;
    fn snd_midi_event_reset_encode(dev: *mut snd_midi_event);
    fn snd_midi_event_free(dev: *mut snd_midi_event);
    fn snd_rawmidi_kernel_open(
        rmidi: *mut snd_rawmidi,
        subdevice: c_int,
        mode: c_int,
        rfile: *mut snd_rawmidi_file,
    ) -> c_int;
    fn snd_rawmidi_input_params(
        substream: *mut snd_rawmidi_substream,
        params: *mut snd_rawmidi_params,
    ) -> c_int;
    fn snd_rawmidi_output_params(
        substream: *mut snd_rawmidi_substream,
        params: *mut snd_rawmidi_params,
    ) -> c_int;
    fn snd_rawmidi_kernel_release(rfile: *mut snd_rawmidi_file) -> c_int;
    fn snd_rawmidi_drain_output(substream: *mut snd_rawmidi_substream);
    fn snd_seq_event_port_detach(client: c_int, port: c_int) -> c_int;
    fn snd_rawmidi_info_select(card: *mut snd_card, info: *mut snd_rawmidi_info) -> c_int;
    fn snd_seq_create_kernel_client(
        card: *mut snd_card,
        device: c_int,
        format: *const c_char,
        ...
    ) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn snd_seq_kernel_client_ctl(client: c_int, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T {
    p
}

unsafe fn rcu_assign_pointer<T>(slot: *mut *mut T, val: *mut T) {
    *slot = val;
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), 0) as *mut T
}

unsafe fn kzalloc_objs<T>(count: usize) -> *mut T {
    kzalloc(size_of::<T>() * count, 0) as *mut T
}

/* handle rawmidi input event (MIDI v1.0 stream) */
unsafe extern "C" fn snd_midi_input_event(substream: *mut snd_rawmidi_substream) {
    let runtime: *mut snd_rawmidi_runtime;
    let msynth: *mut seq_midisynth;
    let mut ev: snd_seq_event = zeroed();
    let mut buf: [c_char; 16] = [0; 16];
    let mut pbuf: *mut c_char;
    let mut res: c_long;

    if substream.is_null() {
        return;
    }
    runtime = (*substream).runtime;
    msynth = (*runtime).private_data as *mut seq_midisynth;
    if msynth.is_null() {
        return;
    }

    if rcu_dereference((*msynth).input_substream) != substream {
        return;
    }
    snd_use_lock_use(addr_of_mut!((*msynth).input_use_lock));

    memset((&mut ev as *mut snd_seq_event).cast(), 0, size_of::<snd_seq_event>());
    while (*runtime).avail > 0 {
        res = snd_rawmidi_kernel_read(substream, buf.as_mut_ptr(), size_of::<[c_char; 16]>() as c_int);
        if res <= 0 {
            continue;
        }
        if (*msynth).parser.is_null() {
            continue;
        }
        pbuf = buf.as_mut_ptr();
        while {
            let old = res;
            res -= 1;
            old > 0
        } {
            let ch = *pbuf;
            pbuf = pbuf.add(1);
            if snd_midi_event_encode_byte((*msynth).parser, ch as c_int, &mut ev) == 0 {
                continue;
            }
            ev.source.port = (*msynth).seq_port;
            ev.dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
            snd_seq_kernel_client_dispatch((*msynth).seq_client, &mut ev, 1, 0);
            /* clear event and reset header */
            memset((&mut ev as *mut snd_seq_event).cast(), 0, size_of::<snd_seq_event>());
        }
    }

    snd_use_lock_free(addr_of_mut!((*msynth).input_use_lock));
}

unsafe fn dump_midi(
    substream: *mut snd_rawmidi_substream,
    buf: *const c_char,
    count: c_int,
) -> c_int {
    let runtime: *mut snd_rawmidi_runtime;
    let tmp: c_int;

    if snd_BUG_ON(substream.is_null() || buf.is_null()) != 0 {
        return -EINVAL;
    }
    runtime = (*substream).runtime;
    tmp = (*runtime).avail;
    if tmp < count {
        if printk_ratelimit() != 0 {
            pr_err(c"ALSA: seq_midi: MIDI output buffer overrun\n".as_ptr());
        }
        return -ENOMEM;
    }
    if snd_rawmidi_kernel_write(substream, buf, count) < count {
        return -EINVAL;
    }
    0
}

/* callback for snd_seq_dump_var_event(), bridging to dump_midi() */
unsafe extern "C" fn __dump_midi(ptr: *mut c_void, buf: *mut c_void, count: c_int) -> c_int {
    dump_midi(ptr as *mut snd_rawmidi_substream, buf as *const c_char, count)
}

unsafe extern "C" fn event_process_midi(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
    _atomic: c_int,
    _hop: c_int,
) -> c_int {
    let msynth = private_data as *mut seq_midisynth;
    let mut msg: [u8; 10] = [0; 10]; /* buffer for constructing midi messages */
    let substream: *mut snd_rawmidi_substream;
    let mut err: c_int = 0;
    let len: c_int;

    if snd_BUG_ON(msynth.is_null()) != 0 {
        return -EINVAL;
    }

    substream = rcu_dereference((*msynth).output_substream);
    if substream.is_null() {
        return -ENODEV;
    }
    snd_use_lock_use(addr_of_mut!((*msynth).output_use_lock));

    if (*ev).type_ == SNDRV_SEQ_EVENT_SYSEX {
        /* special case, to save space */
        if ((*ev).flags & SNDRV_SEQ_EVENT_LENGTH_MASK) != SNDRV_SEQ_EVENT_LENGTH_VARIABLE {
            /* invalid event */
            pr_debug(c"ALSA: seq_midi: invalid sysex event flags = 0x%x\n".as_ptr(), (*ev).flags);
            snd_use_lock_free(addr_of_mut!((*msynth).output_use_lock));
            return err;
        }
        snd_seq_dump_var_event(ev, Some(__dump_midi), substream as *mut c_void);
        snd_midi_event_reset_decode((*msynth).parser);
    } else {
        if (*msynth).parser.is_null() {
            err = -EIO;
            snd_use_lock_free(addr_of_mut!((*msynth).output_use_lock));
            return err;
        }
        len = snd_midi_event_decode((*msynth).parser, msg.as_mut_ptr(), size_of::<[u8; 10]>(), ev);
        if len < 0 {
            snd_use_lock_free(addr_of_mut!((*msynth).output_use_lock));
            return err;
        }
        if dump_midi(substream, msg.as_ptr() as *const c_char, len) < 0 {
            snd_midi_event_reset_decode((*msynth).parser);
        }
    }

    snd_use_lock_free(addr_of_mut!((*msynth).output_use_lock));
    err
}

unsafe fn snd_seq_midisynth_new(
    msynth: *mut seq_midisynth,
    card: *mut snd_card,
    device: c_int,
    subdevice: c_int,
) -> c_int {
    if snd_midi_event_new(MAX_MIDI_EVENT_BUF, addr_of_mut!((*msynth).parser)) < 0 {
        return -ENOMEM;
    }
    (*msynth).card = card;
    (*msynth).device = device;
    (*msynth).subdevice = subdevice;
    snd_use_lock_init(addr_of_mut!((*msynth).input_use_lock));
    snd_use_lock_init(addr_of_mut!((*msynth).output_use_lock));
    0
}

/* open associated midi device for input */
unsafe extern "C" fn midisynth_subscribe(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let mut err: c_int;
    let msynth = private_data as *mut seq_midisynth;
    let runtime: *mut snd_rawmidi_runtime;
    let mut rfile: snd_rawmidi_file = zeroed();
    let mut params: snd_rawmidi_params = zeroed();

    /* open midi port */
    err = snd_rawmidi_kernel_open(
        (*msynth).rmidi,
        (*msynth).subdevice,
        SNDRV_RAWMIDI_LFLG_INPUT,
        &mut rfile,
    );
    if err < 0 {
        pr_debug(c"ALSA: seq_midi: midi input open failed!!!\n".as_ptr());
        return err;
    }
    runtime = (*rfile.input).runtime;
    memset((&mut params as *mut snd_rawmidi_params).cast(), 0, size_of::<snd_rawmidi_params>());
    params.avail_min = 1;
    params.buffer_size = input_buffer_size;
    err = snd_rawmidi_input_params(rfile.input, &mut params);
    if err < 0 {
        snd_rawmidi_kernel_release(&mut rfile);
        return err;
    }
    snd_midi_event_reset_encode((*msynth).parser);
    (*runtime).event = Some(snd_midi_input_event);
    (*runtime).private_data = msynth as *mut c_void;
    (*msynth).input_rfile = rfile;
    rcu_assign_pointer(addr_of_mut!((*msynth).input_substream), rfile.input);
    snd_rawmidi_kernel_read((*msynth).input_rfile.input, null_mut(), 0);
    0
}

/* close associated midi device for input */
unsafe extern "C" fn midisynth_unsubscribe(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let err: c_int;
    let msynth = private_data as *mut seq_midisynth;
    let mut rfile: snd_rawmidi_file;

    rcu_assign_pointer(addr_of_mut!((*msynth).input_substream), null_mut());
    synchronize_rcu();
    snd_use_lock_sync(addr_of_mut!((*msynth).input_use_lock));

    rfile = (*msynth).input_rfile;
    (*msynth).input_rfile = zeroed();

    if snd_BUG_ON(rfile.input.is_null()) != 0 {
        return -EINVAL;
    }

    err = snd_rawmidi_kernel_release(&mut rfile);
    err
}

/* open associated midi device for output */
unsafe extern "C" fn midisynth_use(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let mut err: c_int;
    let msynth = private_data as *mut seq_midisynth;
    let mut rfile: snd_rawmidi_file = zeroed();
    let mut params: snd_rawmidi_params = zeroed();

    /* open midi port */
    err = snd_rawmidi_kernel_open(
        (*msynth).rmidi,
        (*msynth).subdevice,
        SNDRV_RAWMIDI_LFLG_OUTPUT,
        &mut rfile,
    );
    if err < 0 {
        pr_debug(c"ALSA: seq_midi: midi output open failed!!!\n".as_ptr());
        return err;
    }
    memset((&mut params as *mut snd_rawmidi_params).cast(), 0, size_of::<snd_rawmidi_params>());
    params.avail_min = 1;
    params.buffer_size = output_buffer_size;
    params.no_active_sensing = 1;
    err = snd_rawmidi_output_params(rfile.output, &mut params);
    if err < 0 {
        snd_rawmidi_kernel_release(&mut rfile);
        return err;
    }
    snd_midi_event_reset_decode((*msynth).parser);
    (*msynth).output_rfile = rfile;
    rcu_assign_pointer(addr_of_mut!((*msynth).output_substream), rfile.output);
    0
}

/* close associated midi device for output */
unsafe extern "C" fn midisynth_unuse(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let msynth = private_data as *mut seq_midisynth;
    let mut rfile: snd_rawmidi_file;

    rcu_assign_pointer(addr_of_mut!((*msynth).output_substream), null_mut());
    synchronize_rcu();
    snd_use_lock_sync(addr_of_mut!((*msynth).output_use_lock));
    rfile = (*msynth).output_rfile;
    (*msynth).output_rfile = zeroed();

    if snd_BUG_ON(rfile.output.is_null()) != 0 {
        return -EINVAL;
    }
    snd_rawmidi_drain_output(rfile.output);
    snd_rawmidi_kernel_release(&mut rfile)
}

/* delete given midi synth port */
unsafe fn snd_seq_midisynth_delete(msynth: *mut seq_midisynth) {
    if msynth.is_null() {
        return;
    }

    if (*msynth).seq_client > 0 {
        /* delete port */
        snd_seq_event_port_detach((*msynth).seq_client, (*msynth).seq_port);
    }

    snd_midi_event_free((*msynth).parser);
}

/* register new midi synth port */
unsafe extern "C" fn snd_seq_midisynth_probe(dev: *mut snd_seq_device) -> c_int {
    let mut client: *mut seq_midisynth_client;
    let mut msynth: *mut seq_midisynth;
    let mut ms: *mut seq_midisynth;
    let rmidi = (*dev).private_data as *mut snd_rawmidi;
    let mut newclient: c_int = 0;
    let mut p: c_uint;
    let mut ports: c_uint;
    let mut pcallbacks: snd_seq_port_callback = zeroed();
    let card = (*dev).card;
    let device = (*dev).device;
    let mut input_count: c_uint = 0;
    let mut output_count: c_uint = 0;
    let mut info: *mut snd_rawmidi_info;
    let mut port: *mut snd_seq_port_info;

    if snd_BUG_ON(card.is_null() || device < 0 || device >= SNDRV_RAWMIDI_DEVICES as c_int) != 0 {
        return -EINVAL;
    }

    info = kzalloc_obj::<snd_rawmidi_info>();
    if info.is_null() {
        return -ENOMEM;
    }
    (*info).device = device;
    (*info).stream = SNDRV_RAWMIDI_STREAM_OUTPUT;
    (*info).subdevice = 0;
    if snd_rawmidi_info_select(card, info) >= 0 {
        output_count = (*info).subdevices_count;
    }
    (*info).stream = SNDRV_RAWMIDI_STREAM_INPUT;
    if snd_rawmidi_info_select(card, info) >= 0 {
        input_count = (*info).subdevices_count;
    }
    ports = output_count;
    if ports < input_count {
        ports = input_count;
    }
    if ports == 0 {
        kfree(info.cast());
        return -ENODEV;
    }
    if ports > (256 / SNDRV_RAWMIDI_DEVICES) as c_uint {
        ports = (256 / SNDRV_RAWMIDI_DEVICES) as c_uint;
    }

    mutex_lock(addr_of_mut!(register_mutex));
    client = synths[(*card).number as usize];
    if client.is_null() {
        newclient = 1;
        client = kzalloc_obj::<seq_midisynth_client>();
        if client.is_null() {
            mutex_unlock(addr_of_mut!(register_mutex));
            kfree(info.cast());
            return -ENOMEM;
        }
        (*client).seq_client = snd_seq_create_kernel_client(
            card,
            0,
            c"%s".as_ptr(),
            if (*card).shortname[0] != 0 {
                (*card).shortname.as_ptr()
            } else {
                c"External MIDI".as_ptr()
            },
        );
        if (*client).seq_client < 0 {
            kfree(client.cast());
            mutex_unlock(addr_of_mut!(register_mutex));
            kfree(info.cast());
            return -ENOMEM;
        }
    }

    msynth = kzalloc_objs::<seq_midisynth>(ports as usize);

    port = kzalloc_obj::<snd_seq_port_info>();
    if msynth.is_null() || port.is_null() {
        goto_nomem(msynth, ports, newclient, client, info, port);
        mutex_unlock(addr_of_mut!(register_mutex));
        return -ENOMEM;
    }

    p = 0;
    while p < ports {
        ms = msynth.add(p as usize);
        (*ms).rmidi = rmidi;

        if snd_seq_midisynth_new(ms, card, device, p as c_int) < 0 {
            goto_nomem(msynth, ports, newclient, client, info, port);
            mutex_unlock(addr_of_mut!(register_mutex));
            return -ENOMEM;
        }

        /* declare port */
        memset(port.cast(), 0, size_of::<snd_seq_port_info>());
        (*port).addr.client = (*client).seq_client;
        (*port).addr.port = device * (256 / SNDRV_RAWMIDI_DEVICES) as c_int + p as c_int;
        (*port).flags = SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
        memset(info.cast(), 0, size_of::<snd_rawmidi_info>());
        (*info).device = device;
        if p < output_count {
            (*info).stream = SNDRV_RAWMIDI_STREAM_OUTPUT;
        } else {
            (*info).stream = SNDRV_RAWMIDI_STREAM_INPUT;
        }
        (*info).subdevice = p;
        if snd_rawmidi_info_select(card, info) >= 0 {
            strscpy((*port).name.as_mut_ptr(), (*info).subname.as_ptr());
        }
        if (*port).name[0] == 0 {
            if (*info).name[0] != 0 {
                if ports > 1 {
                    scnprintf(
                        (*port).name.as_mut_ptr(),
                        size_of::<[c_char; 64]>(),
                        c"%s-%u".as_ptr(),
                        (*info).name.as_ptr(),
                        p,
                    );
                } else {
                    scnprintf(
                        (*port).name.as_mut_ptr(),
                        size_of::<[c_char; 64]>(),
                        c"%s".as_ptr(),
                        (*info).name.as_ptr(),
                    );
                }
            } else {
                /* last resort */
                if ports > 1 {
                    sprintf(
                        (*port).name.as_mut_ptr(),
                        c"MIDI %d-%d-%u".as_ptr(),
                        (*card).number,
                        device,
                        p,
                    );
                } else {
                    sprintf(
                        (*port).name.as_mut_ptr(),
                        c"MIDI %d-%d".as_ptr(),
                        (*card).number,
                        device,
                    );
                }
            }
        }
        if ((*info).flags & SNDRV_RAWMIDI_INFO_OUTPUT) != 0 && p < output_count {
            (*port).capability |=
                SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SYNC_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
        }
        if ((*info).flags & SNDRV_RAWMIDI_INFO_INPUT) != 0 && p < input_count {
            (*port).capability |=
                SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SYNC_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;
        }
        if ((*port).capability & (SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_READ))
            == (SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_READ)
            && ((*info).flags & SNDRV_RAWMIDI_INFO_DUPLEX) != 0
        {
            (*port).capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
        }
        if ((*port).capability & SNDRV_SEQ_PORT_CAP_READ) != 0 {
            (*port).direction |= SNDRV_SEQ_PORT_DIR_INPUT;
        }
        if ((*port).capability & SNDRV_SEQ_PORT_CAP_WRITE) != 0 {
            (*port).direction |= SNDRV_SEQ_PORT_DIR_OUTPUT;
        }
        (*port).type_ =
            SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC | SNDRV_SEQ_PORT_TYPE_HARDWARE | SNDRV_SEQ_PORT_TYPE_PORT;
        (*port).midi_channels = 16;
        memset((&mut pcallbacks as *mut snd_seq_port_callback).cast(), 0, size_of::<snd_seq_port_callback>());
        pcallbacks.owner = THIS_MODULE;
        pcallbacks.private_data = ms as *mut c_void;
        pcallbacks.subscribe = Some(midisynth_subscribe);
        pcallbacks.unsubscribe = Some(midisynth_unsubscribe);
        pcallbacks.use_ = Some(midisynth_use);
        pcallbacks.unuse = Some(midisynth_unuse);
        pcallbacks.event_input = Some(event_process_midi);
        (*port).kernel = &mut pcallbacks;
        if !(*rmidi).ops.is_null() && (*(*rmidi).ops).get_port_info.is_some() {
            ((*(*rmidi).ops).get_port_info.unwrap())(rmidi, p, port);
        }
        if snd_seq_kernel_client_ctl(
            (*client).seq_client,
            SNDRV_SEQ_IOCTL_CREATE_PORT,
            port.cast(),
        ) < 0
        {
            goto_nomem(msynth, ports, newclient, client, info, port);
            mutex_unlock(addr_of_mut!(register_mutex));
            return -ENOMEM;
        }
        (*ms).seq_client = (*client).seq_client;
        (*ms).seq_port = (*port).addr.port;
        p += 1;
    }
    (*client).ports_per_device[device as usize] = ports as c_int;
    (*client).ports[device as usize] = msynth;
    (*client).num_ports += 1;
    if newclient != 0 {
        synths[(*card).number as usize] = client;
    }
    kfree(port.cast());
    kfree(info.cast());
    mutex_unlock(addr_of_mut!(register_mutex));
    0 /* success */
}

unsafe fn goto_nomem(
    msynth: *mut seq_midisynth,
    ports: c_uint,
    newclient: c_int,
    client: *mut seq_midisynth_client,
    info: *mut snd_rawmidi_info,
    port: *mut snd_seq_port_info,
) {
    let mut p: c_uint;
    if !msynth.is_null() {
        p = 0;
        while p < ports {
            snd_seq_midisynth_delete(msynth.add(p as usize));
            p += 1;
        }
        kfree(msynth.cast());
    }
    if newclient != 0 {
        snd_seq_delete_kernel_client((*client).seq_client);
        kfree(client.cast());
    }
    if !port.is_null() {
        kfree(port.cast());
    }
    if !info.is_null() {
        kfree(info.cast());
    }
}

/* release midi synth port */
unsafe extern "C" fn snd_seq_midisynth_remove(dev: *mut snd_seq_device) {
    let mut client: *mut seq_midisynth_client;
    let msynth: *mut seq_midisynth;
    let card = (*dev).card;
    let device = (*dev).device;
    let mut p: c_int;
    let ports: c_int;

    mutex_lock(addr_of_mut!(register_mutex));
    client = synths[(*card).number as usize];
    if client.is_null() || (*client).ports[device as usize].is_null() {
        mutex_unlock(addr_of_mut!(register_mutex));
        return;
    }
    ports = (*client).ports_per_device[device as usize];
    (*client).ports_per_device[device as usize] = 0;
    msynth = (*client).ports[device as usize];
    (*client).ports[device as usize] = null_mut();
    p = 0;
    while p < ports {
        snd_seq_midisynth_delete(msynth.add(p as usize));
        p += 1;
    }
    kfree(msynth.cast());
    (*client).num_ports -= 1;
    if (*client).num_ports <= 0 {
        snd_seq_delete_kernel_client((*client).seq_client);
        synths[(*card).number as usize] = null_mut();
        kfree(client.cast());
    }
    mutex_unlock(addr_of_mut!(register_mutex));
}

static mut seq_midisynth_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_seq_midisynth_probe),
    remove: Some(snd_seq_midisynth_remove),
    driver: snd_seq_driver_inner {
        name: KBUILD_MODNAME,
    },
    id: SNDRV_SEQ_DEV_ID_MIDISYNTH,
    argsize: 0,
};

/* module_snd_seq_driver(seq_midisynth_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
