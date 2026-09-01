// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer System services Client
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

// C dependencies:
// <linux/init.h>, <linux/export.h>, <linux/slab.h>, <sound/core.h>
// "seq_system.h", "seq_timer.h", "seq_queue.h"

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

/* internal client that provide system services, access to timer etc. */

/*
 * Port "Timer"
 *      - send tempo /start/stop etc. events to this port to manipulate the
 *        queue's timer. The queue address is specified in
 *        data.queue.queue.
 *      - this port supports subscription. The received timer events are
 *        broadcasted to all subscribed clients. The modified tempo
 *        value is stored on data.queue.value.
 *        The modifier client/port is not send.
 *
 * Port "Announce"
 *      - does not receive message
 *      - supports supscription. For each client or port attaching to or
 *        detaching from the system an announcement is send to the subscribed
 *        clients.
 *
 * Idea: the subscription mechanism might also work handy for distributing
 * synchronisation and timing information. In this case we would ideally have
 * a list of subscribers for each type of sync (time, tick), for each timing
 * queue.
 *
 * NOTE: the queue to be started, stopped, etc. must be specified
 *       in data.queue.addr.queue field.  queue is used only for
 *       scheduling, and no longer referred as affected queue.
 *       They are used only for timer broadcast (see above).
 *                                                      -- iwai
 */

use crate::*;

/* client id of our system client */
static mut sysclient: i32 = -1;

/* port id numbers for this client */
static mut announce_port: i32 = -1;

/* number of subscriptions to announce port */
static mut announce_subscribed: i32 = 0;

/* fill standard header data, source port & channel are filled in */
unsafe fn setheader(ev: *mut snd_seq_event, client: i32, port: i32) -> i32 {
    if announce_port < 0 || announce_subscribed == 0 {
        return -ENODEV;
    }

    ptr::write_bytes(ev as *mut u8, 0, size_of::<snd_seq_event>());

    (*ev).flags &= !SNDRV_SEQ_EVENT_LENGTH_MASK;
    (*ev).flags |= SNDRV_SEQ_EVENT_LENGTH_FIXED;

    (*ev).source.client = sysclient;
    (*ev).source.port = announce_port;
    (*ev).dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;

    /* fill data */
    /*ev->data.addr.queue = SNDRV_SEQ_ADDRESS_UNKNOWN;*/
    (*ev).data.addr.client = client;
    (*ev).data.addr.port = port;

    0
}

/* entry points for broadcasting system events */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_system_broadcast(
    client: i32,
    port: i32,
    type_: i32,
    atomic: bool,
) {
    let mut ev: snd_seq_event = core::mem::zeroed();

    if setheader(&mut ev, client, port) < 0 {
        return;
    }
    ev.type_ = type_;
    snd_seq_kernel_client_dispatch(sysclient, &mut ev, atomic, 0);
}
// EXPORT_SYMBOL_GPL(snd_seq_system_broadcast);

/* entry points for broadcasting system events */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_system_notify(
    client: i32,
    port: i32,
    ev: *mut snd_seq_event,
    atomic: bool,
) -> i32 {
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_FIXED;
    (*ev).source.client = sysclient;
    (*ev).source.port = announce_port;
    (*ev).dest.client = client;
    (*ev).dest.port = port;
    snd_seq_kernel_client_dispatch(sysclient, ev, atomic, 0)
}

/* call-back handler for timer events */
unsafe extern "C" fn event_input_timer(
    ev: *mut snd_seq_event,
    direct: i32,
    private_data: *mut c_void,
    atomic: i32,
    hop: i32,
) -> i32 {
    let _ = direct;
    let _ = private_data;
    snd_seq_control_queue(ev, atomic, hop)
}

unsafe extern "C" fn sys_announce_subscribe(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> i32 {
    let _ = private_data;
    let _ = info;
    announce_subscribed += 1;
    0
}

unsafe extern "C" fn sys_announce_unsubscribe(
    private_data: *mut c_void,
    info: *mut snd_seq_port_subscribe,
) -> i32 {
    let _ = private_data;
    let _ = info;
    if snd_BUG_ON(announce_subscribed == 0) != 0 {
        return 0;
    }
    announce_subscribed -= 1;
    0
}

/* register our internal client */
// __init
#[no_mangle]
pub unsafe extern "C" fn snd_seq_system_client_init() -> i32 {
    let mut pcallbacks: snd_seq_port_callback = core::mem::zeroed();
    let port: *mut snd_seq_port_info;
    let mut err: i32;

    port = kzalloc_obj::<snd_seq_port_info>();
    if port.is_null() {
        return -ENOMEM;
    }

    ptr::write_bytes(
        &mut pcallbacks as *mut snd_seq_port_callback as *mut u8,
        0,
        size_of::<snd_seq_port_callback>(),
    );
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.event_input = Some(event_input_timer);

    /* register client */
    sysclient = snd_seq_create_kernel_client(ptr::null_mut(), 0, c"System".as_ptr());
    if sysclient < 0 {
        kfree(port as *mut c_void);
        return sysclient;
    }

    /* register timer */
    strscpy((*port).name.as_mut_ptr(), c"Timer".as_ptr());
    (*port).capability = SNDRV_SEQ_PORT_CAP_WRITE; /* accept queue control */
    (*port).capability |= SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ; /* for broadcast */
    (*port).kernel = &mut pcallbacks;
    (*port).type_ = 0;
    (*port).flags = SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
    (*port).addr.client = sysclient;
    (*port).addr.port = SNDRV_SEQ_PORT_SYSTEM_TIMER;
    err = snd_seq_kernel_client_ctl(sysclient, SNDRV_SEQ_IOCTL_CREATE_PORT, port as *mut c_void);
    if err < 0 {
        goto_error_port(port, err);
        return err;
    }

    /* register announcement port */
    strscpy((*port).name.as_mut_ptr(), c"Announce".as_ptr());
    (*port).capability = SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ; /* for broadcast only */
    pcallbacks.event_input = None;
    pcallbacks.subscribe = Some(sys_announce_subscribe);
    pcallbacks.unsubscribe = Some(sys_announce_unsubscribe);
    (*port).kernel = &mut pcallbacks;
    (*port).type_ = 0;
    (*port).flags = SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
    (*port).addr.client = sysclient;
    (*port).addr.port = SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE;
    err = snd_seq_kernel_client_ctl(sysclient, SNDRV_SEQ_IOCTL_CREATE_PORT, port as *mut c_void);
    if err < 0 {
        goto_error_port(port, err);
        return err;
    }
    announce_port = (*port).addr.port;

    kfree(port as *mut c_void);
    0
}

unsafe fn goto_error_port(port: *mut snd_seq_port_info, err: i32) {
    let _ = err;
    snd_seq_system_client_done();
    kfree(port as *mut c_void);
}

/* unregister our internal client */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_system_client_done() {
    let oldsysclient: i32 = sysclient;

    if oldsysclient >= 0 {
        sysclient = -1;
        announce_port = -1;
        snd_seq_delete_kernel_client(oldsysclient);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
