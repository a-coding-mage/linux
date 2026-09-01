// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * MIDI device handlers
 *
 * Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies:
// <sound/asoundef.h>, "seq_oss_midi.h", "seq_oss_readq.h",
// "seq_oss_timer.h", "seq_oss_event.h", <sound/seq_midi_event.h>,
// "../seq_lock.h", <linux/init.h>, <linux/slab.h>, <linux/nospec.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

/*
 * constants
 */
pub const SNDRV_SEQ_OSS_MAX_MIDI_NAME: usize = 30;

/*
 * definition of midi device record
 */
#[repr(C)]
pub struct seq_oss_midi {
    pub seq_device: c_int, /* device number */
    pub client: c_int, /* sequencer client number */
    pub port: c_int, /* sequencer port number */
    pub flags: c_uint, /* port capability */
    pub opened: c_int, /* flag for opening */
    pub name: [u8; SNDRV_SEQ_OSS_MAX_MIDI_NAME],
    pub coder: *mut snd_midi_event, /* MIDI event coder */
    pub devinfo: *mut seq_oss_devinfo, /* assigned OSSseq device */
    pub use_lock: snd_use_lock_t,
    pub open_mutex: mutex,
}

/*
 * midi device table
 */
static mut max_midi_devs: c_int = 0;
static mut midi_devs: [*mut seq_oss_midi; SNDRV_SEQ_OSS_MAX_MIDI_DEVS as usize] =
    [ptr::null_mut(); SNDRV_SEQ_OSS_MAX_MIDI_DEVS as usize];

static mut register_lock: spinlock_t = SPIN_LOCK_UNLOCKED;

/*
 * prototypes
 */
unsafe fn get_mdev(dev: c_int) -> *mut seq_oss_midi;
unsafe fn get_mididev(dp: *mut seq_oss_devinfo, dev: c_int) -> *mut seq_oss_midi;
unsafe fn send_synth_event(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    dev: c_int,
) -> c_int;
unsafe fn send_midi_event(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    mdev: *mut seq_oss_midi,
) -> c_int;

/*
 * look up the existing ports
 * this looks a very exhausting job.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_lookup_ports(client: c_int) -> c_int {
    let clinfo: *mut snd_seq_client_info =
        kzalloc(mem::size_of::<snd_seq_client_info>(), GFP_KERNEL) as *mut snd_seq_client_info;
    let pinfo: *mut snd_seq_port_info =
        kzalloc(mem::size_of::<snd_seq_port_info>(), GFP_KERNEL) as *mut snd_seq_port_info;

    if clinfo.is_null() || pinfo.is_null() {
        if !clinfo.is_null() {
            kfree(clinfo as *const c_void);
        }
        if !pinfo.is_null() {
            kfree(pinfo as *const c_void);
        }
        return -ENOMEM;
    }
    (*clinfo).client = -1;
    while snd_seq_kernel_client_ctl(client, SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT, clinfo as *mut c_void)
        == 0
    {
        if (*clinfo).client == client {
            continue; /* ignore myself */
        }
        (*pinfo).addr.client = (*clinfo).client;
        (*pinfo).addr.port = -1;
        while snd_seq_kernel_client_ctl(client, SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT, pinfo as *mut c_void)
            == 0
        {
            snd_seq_oss_midi_check_new_port(pinfo);
        }
    }
    kfree(pinfo as *const c_void);
    kfree(clinfo as *const c_void);
    0
}

/*
 */
unsafe fn get_mdev(dev: c_int) -> *mut seq_oss_midi {
    let flags = spin_lock_irqsave(&mut register_lock);
    let mdev = midi_devs[dev as usize];
    if !mdev.is_null() {
        snd_use_lock_use(&mut (*mdev).use_lock);
    }
    spin_unlock_irqrestore(&mut register_lock, flags);
    mdev
}

/*
 * look for the identical slot
 */
unsafe fn find_slot(client: c_int, port: c_int) -> *mut seq_oss_midi {
    let mut i: c_int;
    let mut mdev: *mut seq_oss_midi;

    let flags = spin_lock_irqsave(&mut register_lock);
    i = 0;
    while i < max_midi_devs {
        mdev = midi_devs[i as usize];
        if !mdev.is_null() && (*mdev).client == client && (*mdev).port == port {
            /* found! */
            snd_use_lock_use(&mut (*mdev).use_lock);
            spin_unlock_irqrestore(&mut register_lock, flags);
            return mdev;
        }
        i += 1;
    }
    spin_unlock_irqrestore(&mut register_lock, flags);
    ptr::null_mut()
}

pub const PERM_WRITE: c_uint = SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
pub const PERM_READ: c_uint = SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;

/*
 * register a new port if it doesn't exist yet
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_check_new_port(
    pinfo: *mut snd_seq_port_info,
) -> c_int {
    let mut i: c_int;
    let mut mdev: *mut seq_oss_midi;

    /* the port must include generic midi */
    if ((*pinfo).type_ & SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC) == 0 {
        return 0;
    }
    /* either read or write subscribable */
    if ((*pinfo).capability & PERM_WRITE) != PERM_WRITE
        && ((*pinfo).capability & PERM_READ) != PERM_READ
    {
        return 0;
    }

    /*
     * look for the identical slot
     */
    mdev = find_slot((*pinfo).addr.client, (*pinfo).addr.port);
    if !mdev.is_null() {
        /* already exists */
        snd_use_lock_free(&mut (*mdev).use_lock);
        return 0;
    }

    /*
     * allocate midi info record
     */
    mdev = kzalloc(mem::size_of::<seq_oss_midi>(), GFP_KERNEL) as *mut seq_oss_midi;
    if mdev.is_null() {
        return -ENOMEM;
    }

    /* copy the port information */
    (*mdev).client = (*pinfo).addr.client;
    (*mdev).port = (*pinfo).addr.port;
    (*mdev).flags = (*pinfo).capability;
    (*mdev).opened = 0;
    snd_use_lock_init(&mut (*mdev).use_lock);
    mutex_init(&mut (*mdev).open_mutex);

    /* copy and truncate the name of synth device */
    strscpy(
        (*mdev).name.as_mut_ptr() as *mut c_char,
        (*pinfo).name.as_ptr() as *const c_char,
        mem::size_of_val(&(*mdev).name),
    );

    /* create MIDI coder */
    if snd_midi_event_new(MAX_MIDI_EVENT_BUF, &mut (*mdev).coder) < 0 {
        pr_err(c"ALSA: seq_oss: can't malloc midi coder\n".as_ptr());
        kfree(mdev as *const c_void);
        return -ENOMEM;
    }
    /* OSS sequencer adds running status to all sequences */
    snd_midi_event_no_status((*mdev).coder, 1);

    /*
     * look for en empty slot
     */
    let flags = spin_lock_irqsave(&mut register_lock);
    i = 0;
    while i < max_midi_devs {
        if midi_devs[i as usize].is_null() {
            break;
        }
        i += 1;
    }
    if i >= max_midi_devs {
        if max_midi_devs >= SNDRV_SEQ_OSS_MAX_MIDI_DEVS {
            spin_unlock_irqrestore(&mut register_lock, flags);
            snd_midi_event_free((*mdev).coder);
            kfree(mdev as *const c_void);
            return -ENOMEM;
        }
        max_midi_devs += 1;
    }
    (*mdev).seq_device = i;
    midi_devs[(*mdev).seq_device as usize] = mdev;
    spin_unlock_irqrestore(&mut register_lock, flags);

    0
}

/*
 * release the midi device if it was registered
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_check_exit_port(
    client: c_int,
    port: c_int,
) -> c_int {
    let mut mdev: *mut seq_oss_midi;
    let mut index: c_int;

    mdev = find_slot(client, port);
    if !mdev.is_null() {
        let flags = spin_lock_irqsave(&mut register_lock);
        midi_devs[(*mdev).seq_device as usize] = ptr::null_mut();
        spin_unlock_irqrestore(&mut register_lock, flags);
        snd_use_lock_free(&mut (*mdev).use_lock);
        snd_use_lock_sync(&mut (*mdev).use_lock);
        snd_midi_event_free((*mdev).coder);
        kfree(mdev as *const c_void);
    }
    let flags = spin_lock_irqsave(&mut register_lock);
    index = max_midi_devs - 1;
    while index >= 0 {
        if !midi_devs[index as usize].is_null() {
            break;
        }
        index -= 1;
    }
    max_midi_devs = index + 1;
    spin_unlock_irqrestore(&mut register_lock, flags);
    0
}

/*
 * release the midi device if it was registered
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_clear_all() {
    let mut i: c_int;
    let mut mdev: *mut seq_oss_midi;

    let flags = spin_lock_irqsave(&mut register_lock);
    i = 0;
    while i < max_midi_devs {
        mdev = midi_devs[i as usize];
        if !mdev.is_null() {
            snd_midi_event_free((*mdev).coder);
            kfree(mdev as *const c_void);
            midi_devs[i as usize] = ptr::null_mut();
        }
        i += 1;
    }
    max_midi_devs = 0;
    spin_unlock_irqrestore(&mut register_lock, flags);
}

/*
 * set up midi tables
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_setup(dp: *mut seq_oss_devinfo) {
    spin_lock_irq(&mut register_lock);
    (*dp).max_mididev = max_midi_devs;
    spin_unlock_irq(&mut register_lock);
}

/*
 * clean up midi tables
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_cleanup(dp: *mut seq_oss_devinfo) {
    let mut i: c_int = 0;
    while i < (*dp).max_mididev {
        snd_seq_oss_midi_close(dp, i);
        i += 1;
    }
    (*dp).max_mididev = 0;
}

/*
 * open all midi devices.  ignore errors.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_open_all(
    dp: *mut seq_oss_devinfo,
    file_mode: c_int,
) {
    let mut i: c_int = 0;
    while i < (*dp).max_mididev {
        snd_seq_oss_midi_open(dp, i, file_mode);
        i += 1;
    }
}

/*
 * get the midi device information
 */
unsafe fn get_mididev(dp: *mut seq_oss_devinfo, mut dev: c_int) -> *mut seq_oss_midi {
    if dev < 0 || dev >= (*dp).max_mididev {
        return ptr::null_mut();
    }
    dev = array_index_nospec(dev, (*dp).max_mididev);
    get_mdev(dev)
}

/*
 * open the midi device if not opened yet
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_open(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    fmode: c_int,
) -> c_int {
    let mut perm: c_uint;
    let mut subs: snd_seq_port_subscribe = mem::zeroed();
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return -ENODEV;
    }

    mutex_lock(&mut (*mdev).open_mutex);
    /* already used? */
    if (*mdev).opened != 0 && (*mdev).devinfo != dp {
        mutex_unlock(&mut (*mdev).open_mutex);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return -EBUSY;
    }

    perm = 0;
    if is_write_mode(fmode) {
        perm |= PERM_WRITE;
    }
    if is_read_mode(fmode) {
        perm |= PERM_READ;
    }
    perm &= (*mdev).flags;
    if perm == 0 {
        mutex_unlock(&mut (*mdev).open_mutex);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return -ENXIO;
    }

    /* already opened? */
    if (((*mdev).opened as c_uint) & perm) == perm {
        mutex_unlock(&mut (*mdev).open_mutex);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return 0;
    }

    perm &= !((*mdev).opened as c_uint);

    if (perm & PERM_WRITE) != 0 {
        subs.sender = (*dp).addr;
        subs.dest.client = (*mdev).client;
        subs.dest.port = (*mdev).port;
        if snd_seq_kernel_client_ctl(
            (*dp).cseq,
            SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT,
            &mut subs as *mut _ as *mut c_void,
        ) >= 0
        {
            (*mdev).opened |= PERM_WRITE as c_int;
        }
    }
    if (perm & PERM_READ) != 0 {
        subs.sender.client = (*mdev).client;
        subs.sender.port = (*mdev).port;
        subs.dest = (*dp).addr;
        subs.flags = SNDRV_SEQ_PORT_SUBS_TIMESTAMP;
        subs.queue = (*dp).queue; /* queue for timestamps */
        if snd_seq_kernel_client_ctl(
            (*dp).cseq,
            SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT,
            &mut subs as *mut _ as *mut c_void,
        ) >= 0
        {
            (*mdev).opened |= PERM_READ as c_int;
        }
    }

    if (*mdev).opened == 0 {
        mutex_unlock(&mut (*mdev).open_mutex);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return -ENXIO;
    }

    (*mdev).devinfo = dp;
    mutex_unlock(&mut (*mdev).open_mutex);
    snd_use_lock_free(&mut (*mdev).use_lock);
    0
}

/*
 * close the midi device if already opened
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_close(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
) -> c_int {
    let mut subs: snd_seq_port_subscribe = mem::zeroed();
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return -ENODEV;
    }
    mutex_lock(&mut (*mdev).open_mutex);
    if (*mdev).opened == 0 || (*mdev).devinfo != dp {
        mutex_unlock(&mut (*mdev).open_mutex);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return 0;
    }

    if ((*mdev).opened & PERM_WRITE as c_int) != 0 {
        subs.sender = (*dp).addr;
        subs.dest.client = (*mdev).client;
        subs.dest.port = (*mdev).port;
        snd_seq_kernel_client_ctl(
            (*dp).cseq,
            SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT,
            &mut subs as *mut _ as *mut c_void,
        );
    }
    if ((*mdev).opened & PERM_READ as c_int) != 0 {
        subs.sender.client = (*mdev).client;
        subs.sender.port = (*mdev).port;
        subs.dest = (*dp).addr;
        snd_seq_kernel_client_ctl(
            (*dp).cseq,
            SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT,
            &mut subs as *mut _ as *mut c_void,
        );
    }

    (*mdev).opened = 0;
    (*mdev).devinfo = ptr::null_mut();
    mutex_unlock(&mut (*mdev).open_mutex);
    snd_use_lock_free(&mut (*mdev).use_lock);
    0
}

/*
 * change seq capability flags to file mode flags
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_filemode(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
) -> c_int {
    let mut mode: c_int;
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return 0;
    }

    mode = 0;
    if ((*mdev).opened & PERM_WRITE as c_int) != 0 {
        mode |= SNDRV_SEQ_OSS_FILE_WRITE;
    }
    if ((*mdev).opened & PERM_READ as c_int) != 0 {
        mode |= SNDRV_SEQ_OSS_FILE_READ;
    }

    snd_use_lock_free(&mut (*mdev).use_lock);
    mode
}

/*
 * reset the midi device and close it:
 * so far, only close the device.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_reset(dp: *mut seq_oss_devinfo, dev: c_int) {
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return;
    }
    if (*mdev).opened == 0 {
        snd_use_lock_free(&mut (*mdev).use_lock);
        return;
    }

    if ((*mdev).opened & PERM_WRITE as c_int) != 0 {
        let mut ev: snd_seq_event = mem::zeroed();
        let mut c: c_int;

        ev.dest.client = (*mdev).client;
        ev.dest.port = (*mdev).port;
        ev.queue = (*dp).queue;
        ev.source.port = (*dp).port;
        if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_SYNTH {
            ev.type_ = SNDRV_SEQ_EVENT_SENSING;
            snd_seq_oss_dispatch(dp, &mut ev, 0, 0);
        }
        c = 0;
        while c < 16 {
            ev.type_ = SNDRV_SEQ_EVENT_CONTROLLER;
            ev.data.control.channel = c;
            ev.data.control.param = MIDI_CTL_ALL_NOTES_OFF;
            snd_seq_oss_dispatch(dp, &mut ev, 0, 0);
            if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
                ev.data.control.param = MIDI_CTL_RESET_CONTROLLERS;
                snd_seq_oss_dispatch(dp, &mut ev, 0, 0);
                ev.type_ = SNDRV_SEQ_EVENT_PITCHBEND;
                ev.data.control.value = 0;
                snd_seq_oss_dispatch(dp, &mut ev, 0, 0);
            }
            c += 1;
        }
    }
    snd_use_lock_free(&mut (*mdev).use_lock);
    // snd_seq_oss_midi_close(dp, dev);
}

/*
 * get client/port of the specified MIDI device
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_get_addr(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    addr: *mut snd_seq_addr,
) {
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return;
    }
    (*addr).client = (*mdev).client;
    (*addr).port = (*mdev).port;
    snd_use_lock_free(&mut (*mdev).use_lock);
}

/*
 * input callback - this can be atomic
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_input(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
) -> c_int {
    let dp = private_data as *mut seq_oss_devinfo;

    if (*dp).readq.is_null() {
        return 0;
    }
    let mdev = find_slot((*ev).source.client, (*ev).source.port);
    if mdev.is_null() {
        return 0;
    }
    if ((*mdev).opened & PERM_READ as c_int) == 0 {
        snd_use_lock_free(&mut (*mdev).use_lock);
        return 0;
    }

    let ret = if (*dp).seq_mode == SNDRV_SEQ_OSS_MODE_MUSIC {
        send_synth_event(dp, ev, (*mdev).seq_device)
    } else {
        send_midi_event(dp, ev, mdev)
    };
    snd_use_lock_free(&mut (*mdev).use_lock);
    ret
}

/*
 * convert ALSA sequencer event to OSS synth event
 */
unsafe fn send_synth_event(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    dev: c_int,
) -> c_int {
    let mut ossev: evrec = mem::zeroed();

    match (*ev).type_ {
        SNDRV_SEQ_EVENT_NOTEON => {
            ossev.v.cmd = MIDI_NOTEON;
        }
        SNDRV_SEQ_EVENT_NOTEOFF => {
            ossev.v.cmd = MIDI_NOTEOFF;
        }
        SNDRV_SEQ_EVENT_KEYPRESS => {
            ossev.v.cmd = MIDI_KEY_PRESSURE;
        }
        SNDRV_SEQ_EVENT_CONTROLLER => {
            ossev.l.cmd = MIDI_CTL_CHANGE;
        }
        SNDRV_SEQ_EVENT_PGMCHANGE => {
            ossev.l.cmd = MIDI_PGM_CHANGE;
        }
        SNDRV_SEQ_EVENT_CHANPRESS => {
            ossev.l.cmd = MIDI_CHN_PRESSURE;
        }
        SNDRV_SEQ_EVENT_PITCHBEND => {
            ossev.l.cmd = MIDI_PITCH_BEND;
        }
        _ => {
            return 0; /* not supported */
        }
    }

    ossev.v.dev = dev;

    match (*ev).type_ {
        SNDRV_SEQ_EVENT_NOTEON | SNDRV_SEQ_EVENT_NOTEOFF | SNDRV_SEQ_EVENT_KEYPRESS => {
            ossev.v.code = EV_CHN_VOICE;
            ossev.v.note = (*ev).data.note.note;
            ossev.v.parm = (*ev).data.note.velocity;
            ossev.v.chn = (*ev).data.note.channel;
        }
        SNDRV_SEQ_EVENT_CONTROLLER | SNDRV_SEQ_EVENT_PGMCHANGE | SNDRV_SEQ_EVENT_CHANPRESS => {
            ossev.l.code = EV_CHN_COMMON;
            ossev.l.p1 = (*ev).data.control.param;
            ossev.l.val = (*ev).data.control.value;
            ossev.l.chn = (*ev).data.control.channel;
        }
        SNDRV_SEQ_EVENT_PITCHBEND => {
            ossev.l.code = EV_CHN_COMMON;
            ossev.l.val = (*ev).data.control.value + 8192;
            ossev.l.chn = (*ev).data.control.channel;
        }
        _ => {}
    }

    snd_seq_oss_readq_put_timestamp((*dp).readq, (*ev).time.tick, (*dp).seq_mode);
    snd_seq_oss_readq_put_event((*dp).readq, &mut ossev);

    0
}

/*
 * decode event and send MIDI bytes to read queue
 */
unsafe fn send_midi_event(
    dp: *mut seq_oss_devinfo,
    ev: *mut snd_seq_event,
    mdev: *mut seq_oss_midi,
) -> c_int {
    let mut msg: [c_char; 32] = [0; 32];
    let mut len: c_int;

    snd_seq_oss_readq_put_timestamp((*dp).readq, (*ev).time.tick, (*dp).seq_mode);
    if !(*(*dp).timer).running {
        len = snd_seq_oss_timer_start((*dp).timer);
    }
    if (*ev).type_ == SNDRV_SEQ_EVENT_SYSEX {
        snd_seq_oss_readq_sysex((*dp).readq, (*mdev).seq_device, ev);
        snd_midi_event_reset_decode((*mdev).coder);
    } else {
        len = snd_midi_event_decode((*mdev).coder, msg.as_mut_ptr(), msg.len(), ev);
        if len > 0 {
            snd_seq_oss_readq_puts((*dp).readq, (*mdev).seq_device, msg.as_mut_ptr(), len);
        }
    }

    0
}

/*
 * dump midi data
 * return 0 : enqueued
 *        non-zero : invalid - ignored
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_putc(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    c: u8,
    ev: *mut snd_seq_event,
    lockp: *mut *mut snd_use_lock_t,
) -> c_int {
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return -ENODEV;
    }
    if snd_midi_event_encode_byte((*mdev).coder, c, ev) != 0 {
        snd_seq_oss_fill_addr(dp, ev, (*mdev).client, (*mdev).port);
        /* the caller must release this later */
        *lockp = &mut (*mdev).use_lock;
        snd_use_lock_use(*lockp);
        snd_use_lock_free(&mut (*mdev).use_lock);
        return 0;
    }
    snd_use_lock_free(&mut (*mdev).use_lock);
    -EINVAL
}

/*
 * create OSS compatible midi_info record
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_make_info(
    dp: *mut seq_oss_devinfo,
    dev: c_int,
    inf: *mut midi_info,
) -> c_int {
    let mdev = get_mididev(dp, dev);

    if mdev.is_null() {
        return -ENXIO;
    }
    (*inf).device = dev;
    (*inf).dev_type = 0; /* FIXME: ?? */
    (*inf).capabilities = 0; /* FIXME: ?? */
    strscpy(
        (*inf).name.as_mut_ptr() as *mut c_char,
        (*mdev).name.as_ptr() as *const c_char,
        mem::size_of_val(&(*inf).name),
    );
    snd_use_lock_free(&mut (*mdev).use_lock);
    0
}

// Original C condition: #ifdef CONFIG_SND_PROC_FS
#[cfg(CONFIG_SND_PROC_FS)]
/*
 * proc interface
 */
unsafe fn capmode_str(val: c_int) -> *mut c_char {
    let val = val & (PERM_READ | PERM_WRITE) as c_int;
    if val == (PERM_READ | PERM_WRITE) as c_int {
        c"read/write".as_ptr() as *mut c_char
    } else if val == PERM_READ as c_int {
        c"read".as_ptr() as *mut c_char
    } else if val == PERM_WRITE as c_int {
        c"write".as_ptr() as *mut c_char
    } else {
        c"none".as_ptr() as *mut c_char
    }
}

#[cfg(CONFIG_SND_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_midi_info_read(buf: *mut snd_info_buffer) {
    let mut i: c_int;

    snd_iprintf(buf, c"\nNumber of MIDI devices: %d\n".as_ptr(), max_midi_devs);
    i = 0;
    while i < max_midi_devs {
        snd_iprintf(buf, c"\nmidi %d: ".as_ptr(), i);
        let mdev = get_mdev(i);
        if mdev.is_null() {
            snd_iprintf(buf, c"*empty*\n".as_ptr());
            i += 1;
            continue;
        }
        snd_iprintf(
            buf,
            c"[%s] ALSA port %d:%d\n".as_ptr(),
            (*mdev).name.as_ptr(),
            (*mdev).client,
            (*mdev).port,
        );
        snd_iprintf(
            buf,
            c"  capability %s / opened %s\n".as_ptr(),
            capmode_str((*mdev).flags as c_int),
            capmode_str((*mdev).opened),
        );
        snd_use_lock_free(&mut (*mdev).use_lock);
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
