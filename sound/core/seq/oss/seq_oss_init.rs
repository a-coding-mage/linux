// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OSS compatible sequencer driver
 *
 * open/close and reset interface
 *
 * Copyright (C) 1998-1999 Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from:
 * seq_oss_device.h, seq_oss_synth.h, seq_oss_midi.h, seq_oss_writeq.h,
 * seq_oss_readq.h, seq_oss_timer.h, seq_oss_event.h, linux/init.h,
 * linux/export.h, linux/moduleparam.h, linux/slab.h, linux/workqueue.h
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * common variables
 */
static mut maxqlen: c_int = SNDRV_SEQ_OSS_MAX_QLEN;
/* module_param(maxqlen, int, 0444); */
/* MODULE_PARM_DESC(maxqlen, "maximum queue length"); */

static mut system_client: c_int = -1; /* ALSA sequencer client number */
static mut system_port: c_int = -1;

static mut num_clients: c_int = 0;
static mut client_table: [*mut seq_oss_devinfo; SNDRV_SEQ_OSS_MAX_CLIENTS as usize] =
    [ptr::null_mut(); SNDRV_SEQ_OSS_MAX_CLIENTS as usize];

/*
 * prototypes
 */
unsafe extern "C" {
    fn snd_seq_kernel_client_ctl(client: c_int, typ: c_int, rec: *mut c_void) -> c_int;
    fn snd_seq_oss_midi_lookup_ports(client: c_int);
    fn snd_seq_create_kernel_client(
        card: *mut c_void,
        client_index: c_int,
        name: *const c_char,
    ) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn snd_seq_oss_midi_clear_all();
    fn snd_seq_oss_midi_check_new_port(pinfo: *mut snd_seq_port_info);
    fn snd_seq_oss_midi_check_exit_port(client: c_int, port: c_int);
    fn snd_seq_oss_synth_setup(dp: *mut seq_oss_devinfo);
    fn snd_seq_oss_midi_setup(dp: *mut seq_oss_devinfo);
    fn snd_seq_oss_synth_cleanup(dp: *mut seq_oss_devinfo);
    fn snd_seq_oss_midi_cleanup(dp: *mut seq_oss_devinfo);
    fn snd_seq_oss_readq_new(dp: *mut seq_oss_devinfo, maxqlen: c_int) -> *mut seq_oss_readq;
    fn snd_seq_oss_writeq_new(dp: *mut seq_oss_devinfo, maxqlen: c_int) -> *mut seq_oss_writeq;
    fn snd_seq_oss_timer_new(dp: *mut seq_oss_devinfo) -> *mut seq_oss_timer;
    fn snd_seq_oss_synth_setup_midi(dp: *mut seq_oss_devinfo);
    fn snd_seq_oss_midi_open_all(dp: *mut seq_oss_devinfo, file_mode: c_int);
    fn snd_seq_event_port_detach(client: c_int, port: c_int) -> c_int;
    fn snd_seq_oss_event_input(
        ev: *mut snd_seq_event,
        direct: c_int,
        private: *mut c_void,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;
    fn snd_seq_oss_timer_delete(timer: *mut seq_oss_timer);
    fn snd_seq_oss_writeq_delete(writeq: *mut seq_oss_writeq);
    fn snd_seq_oss_readq_delete(readq: *mut seq_oss_readq);
    fn snd_seq_oss_synth_reset(dp: *mut seq_oss_devinfo, dev: c_int);
    fn snd_seq_oss_midi_reset(dp: *mut seq_oss_devinfo, dev: c_int);
    fn snd_seq_oss_readq_clear(readq: *mut seq_oss_readq);
    fn snd_seq_oss_writeq_clear(writeq: *mut seq_oss_writeq);
    fn snd_seq_oss_timer_stop(timer: *mut seq_oss_timer);
    fn snd_seq_oss_readq_info_read(readq: *mut seq_oss_readq, buf: *mut snd_info_buffer);
    fn cancel_work_sync(work: *mut work_struct) -> c_int;
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_iprintf(buf: *mut snd_info_buffer, fmt: *const c_char, ...);
}

unsafe fn call_ctl<T>(typ: c_int, rec: *mut T) -> c_int {
    snd_seq_kernel_client_ctl(system_client, typ, rec as *mut c_void)
}

/* call snd_seq_oss_midi_lookup_ports() asynchronously */
unsafe extern "C" fn async_call_lookup_ports(_work: *mut work_struct) {
    snd_seq_oss_midi_lookup_ports(system_client);
}

static mut async_lookup_work: work_struct = work_struct {
    func: Some(async_call_lookup_ports),
};

/*
 * create sequencer client for OSS sequencer
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_create_client() -> c_int {
    let mut port_callback: snd_seq_port_callback;
    let port: *mut snd_seq_port_info = kzalloc(size_of::<snd_seq_port_info>(), GFP_KERNEL)
        as *mut snd_seq_port_info;

    if port.is_null() {
        return -ENOMEM;
    }

    /* create ALSA client */
    let rc = snd_seq_create_kernel_client(
        ptr::null_mut(),
        SNDRV_SEQ_CLIENT_OSS,
        c"OSS sequencer".as_ptr(),
    );
    if rc < 0 {
        kfree(port as *mut c_void);
        return rc;
    }

    system_client = rc;

    /* create announcement receiver port */
    strscpy((*port).name.as_mut_ptr(), c"Receiver".as_ptr());
    (*port).addr.client = system_client;
    (*port).capability = SNDRV_SEQ_PORT_CAP_WRITE; /* receive only */
    (*port).type_ = 0;

    memset(
        &mut port_callback as *mut _ as *mut c_void,
        0,
        size_of::<snd_seq_port_callback>(),
    );
    /* don't set port_callback.owner here. otherwise the module counter
     * is incremented and we can no longer release the module..
     */
    port_callback.event_input = Some(receive_announce);
    (*port).kernel = &mut port_callback;

    if call_ctl(SNDRV_SEQ_IOCTL_CREATE_PORT, port) >= 0 {
        let mut subs: snd_seq_port_subscribe;

        system_port = (*port).addr.port;
        memset(
            &mut subs as *mut _ as *mut c_void,
            0,
            size_of::<snd_seq_port_subscribe>(),
        );
        subs.sender.client = SNDRV_SEQ_CLIENT_SYSTEM;
        subs.sender.port = SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE;
        subs.dest.client = system_client;
        subs.dest.port = system_port;
        call_ctl(SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT, &mut subs);
    }

    kfree(port as *mut c_void);

    /* look up midi devices */
    schedule_work(&mut async_lookup_work);

    0
}

/*
 * receive announcement from system port, and check the midi device
 */
unsafe extern "C" fn receive_announce(
    ev: *mut snd_seq_event,
    _direct: c_int,
    _private: *mut c_void,
    atomic: c_int,
    _hop: c_int,
) -> c_int {
    let mut pinfo: snd_seq_port_info;

    if atomic != 0 {
        return 0; /* it must not happen */
    }

    match (*ev).type_ {
        SNDRV_SEQ_EVENT_PORT_START | SNDRV_SEQ_EVENT_PORT_CHANGE => {
            if (*ev).data.addr.client == system_client {
                return 0; /* ignore myself */
            }
            memset(
                &mut pinfo as *mut _ as *mut c_void,
                0,
                size_of::<snd_seq_port_info>(),
            );
            pinfo.addr = (*ev).data.addr;
            if call_ctl(SNDRV_SEQ_IOCTL_GET_PORT_INFO, &mut pinfo) >= 0 {
                snd_seq_oss_midi_check_new_port(&mut pinfo);
            }
        }
        SNDRV_SEQ_EVENT_PORT_EXIT => {
            if (*ev).data.addr.client == system_client {
                return 0; /* ignore myself */
            }
            snd_seq_oss_midi_check_exit_port((*ev).data.addr.client, (*ev).data.addr.port);
        }
        _ => {}
    }
    0
}

/*
 * delete OSS sequencer client
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_delete_client() -> c_int {
    cancel_work_sync(&mut async_lookup_work);
    if system_client >= 0 {
        snd_seq_delete_kernel_client(system_client);
    }

    snd_seq_oss_midi_clear_all();

    0
}

/*
 * open sequencer device
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_open(file: *mut file, level: c_int) -> c_int {
    let mut i: c_int;
    let mut rc: c_int;
    let dp: *mut seq_oss_devinfo;

    dp = kzalloc(size_of::<seq_oss_devinfo>(), GFP_KERNEL) as *mut seq_oss_devinfo;
    if dp.is_null() {
        return -ENOMEM;
    }

    (*dp).cseq = system_client;
    (*dp).port = -1;
    (*dp).queue = -1;

    i = 0;
    while i < SNDRV_SEQ_OSS_MAX_CLIENTS {
        if client_table[i as usize].is_null() {
            break;
        }
        i += 1;
    }

    (*dp).index = i;
    if i >= SNDRV_SEQ_OSS_MAX_CLIENTS {
        pr_debug(c"ALSA: seq_oss: too many applications\n".as_ptr());
        rc = -ENOMEM;
        snd_seq_oss_synth_cleanup(dp);
        snd_seq_oss_midi_cleanup(dp);
        delete_seq_queue((*dp).queue);
        delete_port(dp);
        return rc;
    }

    /* look up synth and midi devices */
    snd_seq_oss_synth_setup(dp);
    snd_seq_oss_midi_setup(dp);

    if (*dp).synth_opened == 0 && (*dp).max_mididev == 0 {
        /* pr_err("ALSA: seq_oss: no device found\n"); */
        rc = -ENODEV;
        snd_seq_oss_synth_cleanup(dp);
        snd_seq_oss_midi_cleanup(dp);
        delete_seq_queue((*dp).queue);
        delete_port(dp);
        return rc;
    }

    /* create port */
    rc = create_port(dp);
    if rc < 0 {
        pr_err(c"ALSA: seq_oss: can't create port\n".as_ptr());
        snd_seq_oss_synth_cleanup(dp);
        snd_seq_oss_midi_cleanup(dp);
        delete_seq_queue((*dp).queue);
        delete_port(dp);
        return rc;
    }

    /* allocate queue */
    rc = alloc_seq_queue(dp);
    if rc < 0 {
        snd_seq_oss_synth_cleanup(dp);
        snd_seq_oss_midi_cleanup(dp);
        delete_seq_queue((*dp).queue);
        delete_port(dp);
        return rc;
    }

    /* set address */
    (*dp).addr.client = (*dp).cseq;
    (*dp).addr.port = (*dp).port;
    /*dp->addr.queue = dp->queue;*/
    /*dp->addr.channel = 0;*/

    (*dp).seq_mode = level;

    /* set up file mode */
    (*dp).file_mode = translate_mode(file);

    /* initialize read queue */
    if is_read_mode((*dp).file_mode) != 0 {
        (*dp).readq = snd_seq_oss_readq_new(dp, maxqlen);
        if (*dp).readq.is_null() {
            rc = -ENOMEM;
            snd_seq_oss_synth_cleanup(dp);
            snd_seq_oss_midi_cleanup(dp);
            delete_seq_queue((*dp).queue);
            delete_port(dp);
            return rc;
        }
    }

    /* initialize write queue */
    if is_write_mode((*dp).file_mode) != 0 {
        (*dp).writeq = snd_seq_oss_writeq_new(dp, maxqlen);
        if (*dp).writeq.is_null() {
            rc = -ENOMEM;
            snd_seq_oss_synth_cleanup(dp);
            snd_seq_oss_midi_cleanup(dp);
            delete_seq_queue((*dp).queue);
            delete_port(dp);
            return rc;
        }
    }

    /* initialize timer */
    (*dp).timer = snd_seq_oss_timer_new(dp);
    if (*dp).timer.is_null() {
        pr_err(c"ALSA: seq_oss: can't alloc timer\n".as_ptr());
        rc = -ENOMEM;
        snd_seq_oss_synth_cleanup(dp);
        snd_seq_oss_midi_cleanup(dp);
        delete_seq_queue((*dp).queue);
        delete_port(dp);
        return rc;
    }

    /* set private data pointer */
    (*file).private_data = dp as *mut c_void;

    /* set up for mode2 */
    if level == SNDRV_SEQ_OSS_MODE_MUSIC {
        snd_seq_oss_synth_setup_midi(dp);
    } else if is_read_mode((*dp).file_mode) != 0 {
        snd_seq_oss_midi_open_all(dp, SNDRV_SEQ_OSS_FILE_READ);
    }

    client_table[(*dp).index as usize] = dp;
    num_clients += 1;

    0
}

/*
 * translate file flags to private mode
 */
unsafe fn translate_mode(file: *mut file) -> c_int {
    let mut file_mode: c_int = 0;
    if ((*file).f_flags & O_ACCMODE) != O_RDONLY {
        file_mode |= SNDRV_SEQ_OSS_FILE_WRITE;
    }
    if ((*file).f_flags & O_ACCMODE) != O_WRONLY {
        file_mode |= SNDRV_SEQ_OSS_FILE_READ;
    }
    if ((*file).f_flags & O_NONBLOCK) != 0 {
        file_mode |= SNDRV_SEQ_OSS_FILE_NONBLOCK;
    }
    file_mode
}

/*
 * create sequencer port
 */
unsafe fn create_port(dp: *mut seq_oss_devinfo) -> c_int {
    let mut port: snd_seq_port_info;
    let mut callback: snd_seq_port_callback;

    memset(
        &mut port as *mut _ as *mut c_void,
        0,
        size_of::<snd_seq_port_info>(),
    );
    port.addr.client = (*dp).cseq;
    sprintf(port.name.as_mut_ptr(), c"Sequencer-%d".as_ptr(), (*dp).index);
    port.capability = SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_WRITE; /* no subscription */
    port.type_ = SNDRV_SEQ_PORT_TYPE_SPECIFIC;
    port.midi_channels = 128;
    port.synth_voices = 128;

    memset(
        &mut callback as *mut _ as *mut c_void,
        0,
        size_of::<snd_seq_port_callback>(),
    );
    callback.owner = THIS_MODULE;
    callback.private_data = dp as *mut c_void;
    callback.event_input = Some(snd_seq_oss_event_input);
    callback.private_free = Some(free_devinfo);
    port.kernel = &mut callback;

    let rc = call_ctl(SNDRV_SEQ_IOCTL_CREATE_PORT, &mut port);
    if rc < 0 {
        return rc;
    }

    (*dp).port = port.addr.port;

    0
}

/*
 * delete ALSA port
 */
unsafe fn delete_port(dp: *mut seq_oss_devinfo) -> c_int {
    if (*dp).port < 0 {
        kfree(dp as *mut c_void);
        return 0;
    }

    snd_seq_event_port_detach((*dp).cseq, (*dp).port)
}

/*
 * allocate a queue
 */
unsafe fn alloc_seq_queue(dp: *mut seq_oss_devinfo) -> c_int {
    let mut qinfo: snd_seq_queue_info;

    memset(
        &mut qinfo as *mut _ as *mut c_void,
        0,
        size_of::<snd_seq_queue_info>(),
    );
    qinfo.owner = system_client;
    qinfo.locked = 1;
    strscpy(qinfo.name.as_mut_ptr(), c"OSS Sequencer Emulation".as_ptr());
    let rc = call_ctl(SNDRV_SEQ_IOCTL_CREATE_QUEUE, &mut qinfo);
    if rc < 0 {
        return rc;
    }
    (*dp).queue = qinfo.queue;
    0
}

/*
 * release queue
 */
unsafe fn delete_seq_queue(queue: c_int) -> c_int {
    let mut qinfo: snd_seq_queue_info;

    if queue < 0 {
        return 0;
    }
    memset(
        &mut qinfo as *mut _ as *mut c_void,
        0,
        size_of::<snd_seq_queue_info>(),
    );
    qinfo.queue = queue;
    let rc = call_ctl(SNDRV_SEQ_IOCTL_DELETE_QUEUE, &mut qinfo);
    if rc < 0 {
        pr_err(
            c"ALSA: seq_oss: unable to delete queue %d (%d)\n".as_ptr(),
            queue,
            rc,
        );
    }
    rc
}

/*
 * free device informations - private_free callback of port
 */
unsafe extern "C" fn free_devinfo(private: *mut c_void) {
    let dp = private as *mut seq_oss_devinfo;

    snd_seq_oss_timer_delete((*dp).timer);

    snd_seq_oss_writeq_delete((*dp).writeq);

    snd_seq_oss_readq_delete((*dp).readq);

    kfree(dp as *mut c_void);
}

/*
 * close sequencer device
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_release(dp: *mut seq_oss_devinfo) {
    let queue: c_int;

    client_table[(*dp).index as usize] = ptr::null_mut();
    num_clients -= 1;

    snd_seq_oss_reset(dp);

    snd_seq_oss_synth_cleanup(dp);
    snd_seq_oss_midi_cleanup(dp);

    /* clear slot */
    queue = (*dp).queue;
    if (*dp).port >= 0 {
        delete_port(dp);
    }
    delete_seq_queue(queue);
}

/*
 * reset sequencer devices
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_reset(dp: *mut seq_oss_devinfo) {
    let mut i: c_int;

    /* reset all synth devices */
    i = 0;
    while i < (*dp).max_synthdev {
        snd_seq_oss_synth_reset(dp, i);
        i += 1;
    }

    /* reset all midi devices */
    if (*dp).seq_mode != SNDRV_SEQ_OSS_MODE_MUSIC {
        i = 0;
        while i < (*dp).max_mididev {
            snd_seq_oss_midi_reset(dp, i);
            i += 1;
        }
    }

    /* remove queues */
    if !(*dp).readq.is_null() {
        snd_seq_oss_readq_clear((*dp).readq);
    }
    if !(*dp).writeq.is_null() {
        snd_seq_oss_writeq_clear((*dp).writeq);
    }

    /* reset timer */
    snd_seq_oss_timer_stop((*dp).timer);
}

/* CONFIG_SND_PROC_FS */
/*
 * misc. functions for proc interface
 */
unsafe fn filemode_str(val: c_int) -> *const c_char {
    static str_: [*const c_char; 4] = [
        c"none".as_ptr(),
        c"read".as_ptr(),
        c"write".as_ptr(),
        c"read/write".as_ptr(),
    ];
    str_[(val & SNDRV_SEQ_OSS_FILE_ACMODE) as usize]
}

/*
 * proc interface
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_oss_system_info_read(buf: *mut snd_info_buffer) {
    let mut i: c_int;
    let mut dp: *mut seq_oss_devinfo;

    snd_iprintf(buf, c"ALSA client number %d\n".as_ptr(), system_client);
    snd_iprintf(buf, c"ALSA receiver port %d\n".as_ptr(), system_port);

    snd_iprintf(
        buf,
        c"\nNumber of applications: %d\n".as_ptr(),
        num_clients,
    );
    i = 0;
    while i < num_clients {
        snd_iprintf(buf, c"\nApplication %d: ".as_ptr(), i);
        dp = client_table[i as usize];
        if dp.is_null() {
            snd_iprintf(buf, c"*empty*\n".as_ptr());
            i += 1;
            continue;
        }
        snd_iprintf(
            buf,
            c"port %d : queue %d\n".as_ptr(),
            (*dp).port,
            (*dp).queue,
        );
        snd_iprintf(
            buf,
            c"  sequencer mode = %s : file open mode = %s\n".as_ptr(),
            if (*dp).seq_mode != 0 {
                c"music".as_ptr()
            } else {
                c"synth".as_ptr()
            },
            filemode_str((*dp).file_mode),
        );
        if (*dp).seq_mode != 0 {
            snd_iprintf(
                buf,
                c"  timer tempo = %d, timebase = %d\n".as_ptr(),
                (*(*dp).timer).oss_tempo,
                (*(*dp).timer).oss_timebase,
            );
        }
        snd_iprintf(buf, c"  max queue length %d\n".as_ptr(), maxqlen);
        if is_read_mode((*dp).file_mode) != 0 && !(*dp).readq.is_null() {
            snd_seq_oss_readq_info_read((*dp).readq, buf);
        }
        i += 1;
    }
}
/* CONFIG_SND_PROC_FS */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
