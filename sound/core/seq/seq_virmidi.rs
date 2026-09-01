// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Virtual Raw MIDI client on Sequencer
 *
 *  Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>,
 *                        Jaroslav Kysela <perex@perex.cz>
 */

/*
 * Virtual Raw MIDI client
 *
 * The virtual rawmidi client is a sequencer client which associate
 * a rawmidi device file.  The created rawmidi device file can be
 * accessed as a normal raw midi, but its MIDI source and destination
 * are arbitrary.  For example, a user-client software synth connected
 * to this port can be used as a normal midi device as well.
 *
 * The virtual rawmidi device accepts also multiple opens.  Each file
 * has its own input buffer, so that no conflict would occur.  The drain
 * of input/output buffer acts only to the local buffer.
 *
 */

// C includes translated as external dependency intent:
// linux/init.h, linux/wait.h, linux/module.h, linux/slab.h,
// sound/core.h, sound/rawmidi.h, sound/info.h, sound/control.h,
// sound/minors.h, sound/seq_kernel.h, sound/seq_midi_event.h,
// sound/seq_virmidi.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

extern "C" {
    static system_highpri_wq: *mut workqueue_struct;
    static THIS_MODULE: *mut module;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn pr_err(format: *const c_char, ...);

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn init_rwsem(sem: *mut rw_semaphore);

    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn snd_rawmidi_receive(substream: *mut c_void, buf: *mut c_void, count: c_int) -> c_int;
    fn snd_rawmidi_proceed(substream: *mut snd_rawmidi_substream);
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut c_void,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        output_count: c_int,
        input_count: c_int,
        rrawmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: c_int,
        ops: *const snd_rawmidi_ops,
    );

    fn snd_midi_event_new(bufsize: c_int, rdev: *mut *mut snd_midi_event) -> c_int;
    fn snd_midi_event_free(dev: *mut snd_midi_event);
    fn snd_midi_event_reset_decode(dev: *mut snd_midi_event);
    fn snd_midi_event_decode(
        dev: *mut snd_midi_event,
        buf: *mut u8,
        count: usize,
        ev: *mut snd_seq_event,
    ) -> c_int;
    fn snd_midi_event_encode_byte(
        dev: *mut snd_midi_event,
        c: u8,
        ev: *mut snd_seq_event,
    ) -> c_int;
    fn snd_seq_dump_var_event(
        ev: *mut snd_seq_event,
        func: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, c_int) -> c_int>,
        private_data: *mut c_void,
    ) -> c_int;
    fn snd_seq_create_kernel_client(
        card: *mut snd_card,
        device: c_int,
        name_fmt: *const c_char,
        ...
    ) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn snd_seq_kernel_client_ctl(client: c_int, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn snd_seq_kernel_client_dispatch(
        client: c_int,
        ev: *mut snd_seq_event,
        atomic: bool,
        hop: c_int,
    ) -> c_int;

    fn try_module_get(module: *mut module) -> bool;
    fn module_put(module: *mut module);
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;

    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct);
    fn cond_resched();
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;

const SNDRV_VIRMIDI_SEQ_DISPATCH: c_int = 0;
const SNDRV_VIRMIDI_SEQ_ATTACH: c_int = 1;
const SNDRV_VIRMIDI_SUBSCRIBE: c_uint = 1 << 0;
const SNDRV_VIRMIDI_USE: c_uint = 1 << 1;

const SNDRV_SEQ_ADDRESS_SUBSCRIBERS: c_int = 253;
const SNDRV_SEQ_EVENT_NONE: c_int = 0;
const SNDRV_SEQ_EVENT_SYSEX: c_int = 130;
const SNDRV_SEQ_EVENT_LENGTH_MASK: c_uint = 3 << 2;
const SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_uint = 1 << 2;
const MAX_MIDI_EVENT_BUF: c_int = 256;

const SNDRV_SEQ_PORT_CAP_WRITE: c_uint = 1 << 0;
const SNDRV_SEQ_PORT_CAP_SYNC_WRITE: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_uint = 1 << 2;
const SNDRV_SEQ_PORT_CAP_READ: c_uint = 1 << 3;
const SNDRV_SEQ_PORT_CAP_SYNC_READ: c_uint = 1 << 4;
const SNDRV_SEQ_PORT_CAP_SUBS_READ: c_uint = 1 << 5;
const SNDRV_SEQ_PORT_CAP_DUPLEX: c_uint = 1 << 6;
const SNDRV_SEQ_PORT_DIR_BIDIRECTION: c_uint = 3;
const SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: c_uint = 1 << 1;
const SNDRV_SEQ_PORT_TYPE_SOFTWARE: c_uint = 1 << 2;
const SNDRV_SEQ_PORT_TYPE_PORT: c_uint = 1 << 3;
const SNDRV_SEQ_IOCTL_CREATE_PORT: c_uint = 0;

const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 1 << 0;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 1 << 2;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_midi_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub module: *mut module,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_rawmidi_runtime {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub runtime: *mut snd_rawmidi_runtime,
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    pub ops: *const snd_rawmidi_global_ops,
    pub name: [c_char; 80],
    pub id: [c_char; 80],
    pub info_flags: c_uint,
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
pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub private_data: *mut c_void,
    pub subscribe:
        Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
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
    pub name: [c_char; 64],
    pub capability: c_uint,
    pub direction: c_uint,
    pub type_: c_uint,
    pub midi_channels: c_int,
    pub kernel: *mut snd_seq_port_callback,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
    pub drain: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream)>,
}

#[repr(C)]
pub struct snd_rawmidi_global_ops {
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
    pub dev_unregister: Option<unsafe extern "C" fn(*mut snd_rawmidi) -> c_int>,
}

#[repr(C)]
pub struct snd_virmidi {
    pub list: list_head,
    pub substream: *mut snd_rawmidi_substream,
    pub parser: *mut snd_midi_event,
    pub seq_mode: c_int,
    pub client: c_int,
    pub port: c_int,
    pub event: snd_seq_event,
    pub output_work: work_struct,
    pub trigger: bool,
    pub rdev: *mut snd_virmidi_dev,
}

#[repr(C)]
pub struct snd_virmidi_dev {
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub device: c_int,
    pub client: c_int,
    pub port: c_int,
    pub flags: c_uint,
    pub filelist_sem: rw_semaphore,
    pub filelist: list_head,
    pub seq_mode: c_int,
}

unsafe fn read_once_bool(p: *const bool) -> bool {
    ptr::read_volatile(p)
}

unsafe fn write_once_bool(p: *mut bool, v: bool) {
    ptr::write_volatile(p, v);
}

unsafe fn list_entry_snd_virmidi(ptr: *mut list_head) -> *mut snd_virmidi {
    (ptr as *mut u8).sub(0) as *mut snd_virmidi
}

unsafe fn container_of_output_work(ptr: *mut work_struct) -> *mut snd_virmidi {
    let base = ptr::null::<snd_virmidi>();
    let offset = (&(*base).output_work as *const work_struct as usize) - (base as usize);
    (ptr as *mut u8).sub(offset) as *mut snd_virmidi
}

/*
 * initialize an event record
 */
unsafe extern "C" fn snd_virmidi_init_event(vmidi: *mut snd_virmidi, ev: *mut snd_seq_event) {
    memset(ev as *mut c_void, 0, size_of::<snd_seq_event>());
    (*ev).source.port = (*vmidi).port;
    match (*vmidi).seq_mode {
        SNDRV_VIRMIDI_SEQ_DISPATCH => {
            (*ev).dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
        }
        SNDRV_VIRMIDI_SEQ_ATTACH => {
            /* FIXME: source and destination are same - not good.. */
            (*ev).dest.client = (*vmidi).client;
            (*ev).dest.port = (*vmidi).port;
        }
        _ => {}
    }
    (*ev).type_ = SNDRV_SEQ_EVENT_NONE;
}

/*
 * decode input event and put to read buffer of each opened file
 */

/* callback for snd_seq_dump_var_event(), bridging to snd_rawmidi_receive() */
unsafe extern "C" fn dump_to_rawmidi(ptr: *mut c_void, buf: *mut c_void, count: c_int) -> c_int {
    snd_rawmidi_receive(ptr, buf, count)
}

unsafe extern "C" fn snd_virmidi_dev_receive_event(
    rdev: *mut snd_virmidi_dev,
    ev: *mut snd_seq_event,
    atomic: bool,
) -> c_int {
    let mut msg: [u8; 4] = [0; 4];
    let mut len: c_int;

    if atomic {
        rcu_read_lock();
    } else {
        down_read(&mut (*rdev).filelist_sem);
    }

    let mut pos = (*rdev).filelist.next;
    while pos != &mut (*rdev).filelist {
        let vmidi = list_entry_snd_virmidi(pos);
        if !read_once_bool(&(*vmidi).trigger) {
            pos = (*pos).next;
            continue;
        }
        if (*ev).type_ == SNDRV_SEQ_EVENT_SYSEX {
            if ((*ev).flags & SNDRV_SEQ_EVENT_LENGTH_MASK) != SNDRV_SEQ_EVENT_LENGTH_VARIABLE {
                pos = (*pos).next;
                continue;
            }
            snd_seq_dump_var_event(
                ev,
                Some(dump_to_rawmidi),
                (*vmidi).substream as *mut c_void,
            );
            snd_midi_event_reset_decode((*vmidi).parser);
        } else {
            len = snd_midi_event_decode((*vmidi).parser, msg.as_mut_ptr(), msg.len(), ev);
            if len > 0 {
                snd_rawmidi_receive((*vmidi).substream as *mut c_void, msg.as_mut_ptr() as *mut c_void, len);
            }
        }
        pos = (*pos).next;
    }

    if atomic {
        rcu_read_unlock();
    } else {
        up_read(&mut (*rdev).filelist_sem);
    }

    0
}

/*
 * event handler of virmidi port
 */
unsafe extern "C" fn snd_virmidi_event_input(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
    atomic: c_int,
    _hop: c_int,
) -> c_int {
    let rdev = private_data as *mut snd_virmidi_dev;

    if ((*rdev).flags & SNDRV_VIRMIDI_USE) == 0 {
        return 0; /* ignored */
    }
    snd_virmidi_dev_receive_event(rdev, ev, atomic != 0)
}

/*
 * trigger rawmidi stream for input
 */
unsafe extern "C" fn snd_virmidi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let vmidi = (*(*substream).runtime).private_data as *mut snd_virmidi;

    write_once_bool(&mut (*vmidi).trigger, up != 0);
}

/* process rawmidi bytes and send events;
 * we need no lock here for vmidi->event since it's handled only in this work
 */
unsafe extern "C" fn snd_vmidi_output_work(work: *mut work_struct) {
    let vmidi = container_of_output_work(work);
    let substream = (*vmidi).substream;
    let mut input: u8 = 0;
    let mut ret: c_int;

    /* discard the outputs in dispatch mode unless subscribed */
    if (*vmidi).seq_mode == SNDRV_VIRMIDI_SEQ_DISPATCH
        && ((*(*vmidi).rdev).flags & SNDRV_VIRMIDI_SUBSCRIBE) == 0
    {
        snd_rawmidi_proceed(substream);
        return;
    }

    while read_once_bool(&(*vmidi).trigger) {
        if snd_rawmidi_transmit(substream, &mut input as *mut u8 as *mut c_void, 1) != 1 {
            break;
        }
        if snd_midi_event_encode_byte((*vmidi).parser, input, &mut (*vmidi).event) == 0 {
            continue;
        }
        if (*vmidi).event.type_ != SNDRV_SEQ_EVENT_NONE {
            ret = snd_seq_kernel_client_dispatch((*vmidi).client, &mut (*vmidi).event, false, 0);
            (*vmidi).event.type_ = SNDRV_SEQ_EVENT_NONE;
            if ret < 0 {
                break;
            }
        }
        /* rawmidi input might be huge, allow to have a break */
        cond_resched();
    }
}

/*
 * trigger rawmidi stream for output
 */
unsafe extern "C" fn snd_virmidi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let vmidi = (*(*substream).runtime).private_data as *mut snd_virmidi;

    write_once_bool(&mut (*vmidi).trigger, up != 0);
    if up != 0 {
        queue_work(system_highpri_wq, &mut (*vmidi).output_work);
    }
}

/*
 * open rawmidi handle for input
 */
unsafe extern "C" fn snd_virmidi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let rdev = (*(*substream).rmidi).private_data as *mut snd_virmidi_dev;
    let runtime = (*substream).runtime;
    let vmidi = kzalloc(size_of::<snd_virmidi>(), GFP_KERNEL) as *mut snd_virmidi;

    if vmidi.is_null() {
        return -ENOMEM;
    }
    (*vmidi).substream = substream;
    if snd_midi_event_new(0, &mut (*vmidi).parser) < 0 {
        kfree(vmidi as *mut c_void);
        return -ENOMEM;
    }
    (*vmidi).seq_mode = (*rdev).seq_mode;
    (*vmidi).client = (*rdev).client;
    (*vmidi).port = (*rdev).port;
    (*runtime).private_data = vmidi as *mut c_void;
    down_write(&mut (*rdev).filelist_sem);
    list_add_tail_rcu(&mut (*vmidi).list, &mut (*rdev).filelist);
    up_write(&mut (*rdev).filelist_sem);
    (*vmidi).rdev = rdev;
    0
}

/*
 * open rawmidi handle for output
 */
unsafe extern "C" fn snd_virmidi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let rdev = (*(*substream).rmidi).private_data as *mut snd_virmidi_dev;
    let runtime = (*substream).runtime;
    let vmidi = kzalloc(size_of::<snd_virmidi>(), GFP_KERNEL) as *mut snd_virmidi;

    if vmidi.is_null() {
        return -ENOMEM;
    }
    (*vmidi).substream = substream;
    if snd_midi_event_new(MAX_MIDI_EVENT_BUF, &mut (*vmidi).parser) < 0 {
        kfree(vmidi as *mut c_void);
        return -ENOMEM;
    }
    (*vmidi).seq_mode = (*rdev).seq_mode;
    (*vmidi).client = (*rdev).client;
    (*vmidi).port = (*rdev).port;
    snd_virmidi_init_event(vmidi, &mut (*vmidi).event);
    (*vmidi).rdev = rdev;
    INIT_WORK(&mut (*vmidi).output_work, snd_vmidi_output_work);
    (*runtime).private_data = vmidi as *mut c_void;
    0
}

/*
 * close rawmidi handle for input
 */
unsafe extern "C" fn snd_virmidi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let rdev = (*(*substream).rmidi).private_data as *mut snd_virmidi_dev;
    let vmidi = (*(*substream).runtime).private_data as *mut snd_virmidi;

    down_write(&mut (*rdev).filelist_sem);
    list_del_rcu(&mut (*vmidi).list);
    up_write(&mut (*rdev).filelist_sem);
    /* wait for a grace period so that lockless readers in the atomic
     * delivery path (snd_virmidi_dev_receive_event()) are no longer
     * traversing this entry before its parser and memory are freed
     */
    synchronize_rcu();
    snd_midi_event_free((*vmidi).parser);
    (*(*substream).runtime).private_data = ptr::null_mut();
    kfree(vmidi as *mut c_void);
    0
}

/*
 * close rawmidi handle for output
 */
unsafe extern "C" fn snd_virmidi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let vmidi = (*(*substream).runtime).private_data as *mut snd_virmidi;

    write_once_bool(&mut (*vmidi).trigger, false); /* to be sure */
    cancel_work_sync(&mut (*vmidi).output_work);
    snd_midi_event_free((*vmidi).parser);
    (*(*substream).runtime).private_data = ptr::null_mut();
    kfree(vmidi as *mut c_void);
    0
}

/*
 * drain output work queue
 */
unsafe extern "C" fn snd_virmidi_output_drain(substream: *mut snd_rawmidi_substream) {
    let vmidi = (*(*substream).runtime).private_data as *mut snd_virmidi;

    flush_work(&mut (*vmidi).output_work);
}

/*
 * subscribe callback - allow output to rawmidi device
 */
unsafe extern "C" fn snd_virmidi_subscribe(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let rdev = private_data as *mut snd_virmidi_dev;

    if !try_module_get((*(*rdev).card).module) {
        return -EFAULT;
    }
    (*rdev).flags |= SNDRV_VIRMIDI_SUBSCRIBE;
    0
}

/*
 * unsubscribe callback - disallow output to rawmidi device
 */
unsafe extern "C" fn snd_virmidi_unsubscribe(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let rdev = private_data as *mut snd_virmidi_dev;

    (*rdev).flags &= !SNDRV_VIRMIDI_SUBSCRIBE;
    module_put((*(*rdev).card).module);
    0
}

/*
 * use callback - allow input to rawmidi device
 */
unsafe extern "C" fn snd_virmidi_use(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let rdev = private_data as *mut snd_virmidi_dev;

    if !try_module_get((*(*rdev).card).module) {
        return -EFAULT;
    }
    (*rdev).flags |= SNDRV_VIRMIDI_USE;
    0
}

/*
 * unuse callback - disallow input to rawmidi device
 */
unsafe extern "C" fn snd_virmidi_unuse(
    private_data: *mut c_void,
    _info: *mut snd_seq_port_subscribe,
) -> c_int {
    let rdev = private_data as *mut snd_virmidi_dev;

    (*rdev).flags &= !SNDRV_VIRMIDI_USE;
    module_put((*(*rdev).card).module);
    0
}

/*
 *  Register functions
 */

static snd_virmidi_input_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_virmidi_input_open),
    close: Some(snd_virmidi_input_close),
    trigger: Some(snd_virmidi_input_trigger),
    drain: None,
};

static snd_virmidi_output_ops: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_virmidi_output_open),
    close: Some(snd_virmidi_output_close),
    trigger: Some(snd_virmidi_output_trigger),
    drain: Some(snd_virmidi_output_drain),
};

/*
 * create a sequencer client and a port
 */
unsafe extern "C" fn snd_virmidi_dev_attach_seq(rdev: *mut snd_virmidi_dev) -> c_int {
    let mut client: c_int;
    let mut pcallbacks: snd_seq_port_callback = zeroed();
    let mut err: c_int;

    if (*rdev).client >= 0 {
        return 0;
    }

    let pinfo = kzalloc(size_of::<snd_seq_port_info>(), GFP_KERNEL) as *mut snd_seq_port_info;
    if pinfo.is_null() {
        return -ENOMEM;
    }

    client = snd_seq_create_kernel_client(
        (*rdev).card,
        (*rdev).device,
        b"%s %d-%d\0".as_ptr() as *const c_char,
        (*(*rdev).rmidi).name.as_ptr(),
        (*(*rdev).card).number,
        (*rdev).device,
    );
    if client < 0 {
        kfree(pinfo as *mut c_void);
        return client;
    }
    (*rdev).client = client;

    /* create a port */
    (*pinfo).addr.client = client;
    sprintf(
        (*pinfo).name.as_mut_ptr(),
        b"VirMIDI %d-%d\0".as_ptr() as *const c_char,
        (*(*rdev).card).number,
        (*rdev).device,
    );
    /* set all capabilities */
    (*pinfo).capability |=
        SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SYNC_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
    (*pinfo).capability |=
        SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SYNC_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;
    (*pinfo).capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
    (*pinfo).direction = SNDRV_SEQ_PORT_DIR_BIDIRECTION;
    (*pinfo).type_ = SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
        | SNDRV_SEQ_PORT_TYPE_SOFTWARE
        | SNDRV_SEQ_PORT_TYPE_PORT;
    (*pinfo).midi_channels = 16;
    memset(
        &mut pcallbacks as *mut snd_seq_port_callback as *mut c_void,
        0,
        size_of::<snd_seq_port_callback>(),
    );
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.private_data = rdev as *mut c_void;
    pcallbacks.subscribe = Some(snd_virmidi_subscribe);
    pcallbacks.unsubscribe = Some(snd_virmidi_unsubscribe);
    pcallbacks.use_ = Some(snd_virmidi_use);
    pcallbacks.unuse = Some(snd_virmidi_unuse);
    pcallbacks.event_input = Some(snd_virmidi_event_input);
    (*pinfo).kernel = &mut pcallbacks;
    err = snd_seq_kernel_client_ctl(client, SNDRV_SEQ_IOCTL_CREATE_PORT, pinfo as *mut c_void);
    if err < 0 {
        snd_seq_delete_kernel_client(client);
        (*rdev).client = -1;
        kfree(pinfo as *mut c_void);
        return err;
    }

    (*rdev).port = (*pinfo).addr.port;
    kfree(pinfo as *mut c_void);
    0 /* success */
}

/*
 * release the sequencer client
 */
unsafe extern "C" fn snd_virmidi_dev_detach_seq(rdev: *mut snd_virmidi_dev) {
    if (*rdev).client >= 0 {
        snd_seq_delete_kernel_client((*rdev).client);
        (*rdev).client = -1;
    }
}

/*
 * register the device
 */
unsafe extern "C" fn snd_virmidi_dev_register(rmidi: *mut snd_rawmidi) -> c_int {
    let rdev = (*rmidi).private_data as *mut snd_virmidi_dev;
    let err: c_int;

    match (*rdev).seq_mode {
        SNDRV_VIRMIDI_SEQ_DISPATCH => {
            err = snd_virmidi_dev_attach_seq(rdev);
            if err < 0 {
                return err;
            }
        }
        SNDRV_VIRMIDI_SEQ_ATTACH => {
            if (*rdev).client == 0 {
                return -EINVAL;
            }
            /* should check presence of port more strictly.. */
        }
        _ => {
            pr_err(
                b"ALSA: seq_virmidi: seq_mode is not set: %d\n\0".as_ptr() as *const c_char,
                (*rdev).seq_mode,
            );
            return -EINVAL;
        }
    }
    0
}

/*
 * unregister the device
 */
unsafe extern "C" fn snd_virmidi_dev_unregister(rmidi: *mut snd_rawmidi) -> c_int {
    let rdev = (*rmidi).private_data as *mut snd_virmidi_dev;

    if (*rdev).seq_mode == SNDRV_VIRMIDI_SEQ_DISPATCH {
        snd_virmidi_dev_detach_seq(rdev);
    }
    0
}

/*
 *
 */
static snd_virmidi_global_ops: snd_rawmidi_global_ops = snd_rawmidi_global_ops {
    dev_register: Some(snd_virmidi_dev_register),
    dev_unregister: Some(snd_virmidi_dev_unregister),
};

/*
 * free device
 */
unsafe extern "C" fn snd_virmidi_free(rmidi: *mut snd_rawmidi) {
    let rdev = (*rmidi).private_data as *mut snd_virmidi_dev;
    kfree(rdev as *mut c_void);
}

/*
 * create a new device
 *
 */
/* exported */
#[no_mangle]
pub unsafe extern "C" fn snd_virmidi_new(
    card: *mut snd_card,
    device: c_int,
    rrmidi: *mut *mut snd_rawmidi,
) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let rdev: *mut snd_virmidi_dev;
    let mut err: c_int;

    *rrmidi = ptr::null_mut();
    err = snd_rawmidi_new(
        card,
        b"VirMidi\0".as_ptr() as *const c_char,
        device,
        16, /* may be configurable */
        16, /* may be configurable */
        &mut rmidi,
    );
    if err < 0 {
        return err;
    }
    strscpy((*rmidi).name.as_mut_ptr(), (*rmidi).id.as_ptr());
    rdev = kzalloc(size_of::<snd_virmidi_dev>(), GFP_KERNEL) as *mut snd_virmidi_dev;
    if rdev.is_null() {
        snd_device_free(card, rmidi as *mut c_void);
        return -ENOMEM;
    }
    (*rdev).card = card;
    (*rdev).rmidi = rmidi;
    (*rdev).device = device;
    (*rdev).client = -1;
    init_rwsem(&mut (*rdev).filelist_sem);
    INIT_LIST_HEAD(&mut (*rdev).filelist);
    (*rdev).seq_mode = SNDRV_VIRMIDI_SEQ_DISPATCH;
    (*rmidi).private_data = rdev as *mut c_void;
    (*rmidi).private_free = Some(snd_virmidi_free);
    (*rmidi).ops = &snd_virmidi_global_ops;
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_virmidi_input_ops);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_virmidi_output_ops);
    (*rmidi).info_flags =
        SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    *rrmidi = rmidi;
    0
}

// EXPORT_SYMBOL(snd_virmidi_new);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
