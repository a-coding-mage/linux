// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA sequencer MIDI-through client
 * Copyright (c) 1999-2000 by Takashi Iwai <tiwai@suse.de>
 */

// C dependencies: <linux/init.h>, <linux/slab.h>, <linux/module.h>,
// <sound/core.h>, "seq_clientmgr.h", "seq_memory.h",
// <sound/initval.h>, <sound/asoundef.h>

/*

  Sequencer MIDI-through client

  This gives a simple midi-through client.  All the normal input events
  are redirected to output port immediately.
  The routing can be done via aconnect program in alsa-utils.

  Each client has a static client number 14 (= SNDRV_SEQ_CLIENT_DUMMY).
  If you want to auto-load this module, you may add the following alias
  in your /etc/conf.modules file.

	alias snd-seq-client-14  snd-seq-dummy

  The module is loaded on demand for client 14, or /proc/asound/seq/
  is accessed.  If you don't need this module to be loaded, alias
  snd-seq-client-14 as "off".  This will help modprobe.

  The number of ports to be created can be specified via the module
  parameter "ports".  For example, to create four ports, add the
  following option in a configuration file under /etc/modprobe.d/:

	option snd-seq-dummy ports=4

  The model option "duplex=1" enables duplex operation to the port.
  In duplex mode, a pair of ports are created instead of single port,
  and events are tunneled between pair-ports.  For example, input to
  port A is sent to output port of another port B and vice versa.
  In duplex mode, each port has DUPLEX capability.

 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
// MODULE_DESCRIPTION("ALSA sequencer MIDI-through client");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("snd-seq-client-" __stringify(SNDRV_SEQ_CLIENT_DUMMY));

static mut ports: c_int = 1;
static mut duplex: bool = false;

// module_param(ports, int, 0444);
// MODULE_PARM_DESC(ports, "number of ports to be created");
// module_param(duplex, bool, 0444);
// MODULE_PARM_DESC(duplex, "create DUPLEX ports");

// #if IS_ENABLED(CONFIG_SND_SEQ_UMP)
#[cfg(CONFIG_SND_SEQ_UMP)]
static mut ump: c_int = 0;
// module_param(ump, int, 0444);
// MODULE_PARM_DESC(ump, "UMP conversion (0: no convert, 1: MIDI 1.0, 2: MIDI 2.0)");
// #endif

#[repr(C)]
struct snd_seq_dummy_port {
    client: c_int,
    port: c_int,
    duplex: c_int,
    connect: c_int,
}

static mut my_client: c_int = -1;

#[repr(C)]
struct snd_seq_addr {
    client: c_int,
    port: c_int,
}

#[repr(C)]
struct snd_seq_event_legacy {
    source: snd_seq_addr,
    dest: snd_seq_addr,
}

#[repr(C)]
struct snd_seq_event {
    source: snd_seq_addr,
    type_: c_int,
}

#[repr(C)]
union __snd_seq_event {
    legacy: core::mem::ManuallyDrop<snd_seq_event_legacy>,
    event: core::mem::ManuallyDrop<snd_seq_event>,
}

#[repr(C)]
struct snd_seq_port_callback {
    owner: *mut c_void,
    event_input: Option<
        unsafe extern "C" fn(
            ev: *mut snd_seq_event,
            direct: c_int,
            private_data: *mut c_void,
            atomic: c_int,
            hop: c_int,
        ) -> c_int,
    >,
    private_free: Option<unsafe extern "C" fn(private_data: *mut c_void)>,
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_seq_port_info {
    addr: snd_seq_addr,
    name: [c_char; 64],
    capability: c_int,
    direction: c_int,
    type_: c_int,
    kernel: *mut snd_seq_port_callback,
}

#[cfg(CONFIG_SND_SEQ_UMP)]
#[repr(C)]
struct snd_seq_client {
    midi_version: c_int,
    filter: c_int,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    static SNDRV_SEQ_CLIENT_DUMMY: c_int;
    static SNDRV_SEQ_CLIENT_SYSTEM: c_int;
    static SNDRV_SEQ_EVENT_KERNEL_ERROR: c_int;
    static SNDRV_SEQ_ADDRESS_SUBSCRIBERS: c_int;
    static SNDRV_SEQ_PORT_CAP_READ: c_int;
    static SNDRV_SEQ_PORT_CAP_SUBS_READ: c_int;
    static SNDRV_SEQ_PORT_CAP_WRITE: c_int;
    static SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_int;
    static SNDRV_SEQ_PORT_CAP_DUPLEX: c_int;
    static SNDRV_SEQ_PORT_DIR_BIDIRECTION: c_int;
    static SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC: c_int;
    static SNDRV_SEQ_PORT_TYPE_SOFTWARE: c_int;
    static SNDRV_SEQ_PORT_TYPE_PORT: c_int;
    static SNDRV_SEQ_IOCTL_CREATE_PORT: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    #[cfg(CONFIG_SND_SEQ_UMP)]
    static SNDRV_SEQ_CLIENT_UMP_MIDI_1_0: c_int;
    #[cfg(CONFIG_SND_SEQ_UMP)]
    static SNDRV_SEQ_CLIENT_UMP_MIDI_2_0: c_int;
    #[cfg(CONFIG_SND_SEQ_UMP)]
    static SNDRV_SEQ_FILTER_NO_CONVERT: c_int;

    fn snd_seq_event_packet_size(ev: *mut snd_seq_event) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snd_seq_kernel_client_dispatch(
        client: c_int,
        ev: *mut snd_seq_event_legacy,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_seq_kernel_client_ctl(client: c_int, cmd: c_int, arg: *mut c_void) -> c_int;
    fn snd_seq_create_kernel_client(
        card: *mut c_void,
        client: c_int,
        name: *const c_char,
    ) -> c_int;
    fn snd_seq_delete_kernel_client(client: c_int) -> c_int;
    fn pr_err(format: *const c_char, ...) -> c_int;

    #[cfg(CONFIG_SND_SEQ_UMP)]
    fn snd_seq_kernel_client_get(client: c_int) -> *mut snd_seq_client;
    #[cfg(CONFIG_SND_SEQ_UMP)]
    fn snd_seq_kernel_client_put(client: *mut snd_seq_client);
}

/*
 * event input callback - just redirect events to subscribers
 */
unsafe extern "C" fn dummy_input(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
    atomic: c_int,
    hop: c_int,
) -> c_int {
    let p: *mut snd_seq_dummy_port = private_data.cast();
    let mut tmpev = MaybeUninit::<__snd_seq_event>::uninit();
    let size: usize;

    if (*ev).source.client == SNDRV_SEQ_CLIENT_SYSTEM
        || (*ev).type_ == SNDRV_SEQ_EVENT_KERNEL_ERROR
    {
        return 0;
    }
    size = snd_seq_event_packet_size(ev);
    memcpy(tmpev.as_mut_ptr().cast(), ev.cast(), size);
    let tmpev = tmpev.as_mut_ptr();
    if (*p).duplex != 0 {
        (*(*tmpev).legacy).source.port = (*p).connect;
    } else {
        (*(*tmpev).legacy).source.port = (*p).port;
    }
    (*(*tmpev).legacy).dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
    snd_seq_kernel_client_dispatch((*p).client, (&mut (*(*tmpev).legacy)) as *mut _, atomic, hop)
}

/*
 * free_private callback
 */
unsafe extern "C" fn dummy_free(private_data: *mut c_void) {
    kfree(private_data);
}

/*
 * create a port
 */
unsafe fn create_port(idx: c_int, type_: c_int) -> *mut snd_seq_dummy_port {
    let mut pinfo = MaybeUninit::<snd_seq_port_info>::uninit();
    let mut pcb = MaybeUninit::<snd_seq_port_callback>::uninit();
    let rec: *mut snd_seq_dummy_port;

    rec = kzalloc(core::mem::size_of::<snd_seq_dummy_port>()).cast();
    if rec.is_null() {
        return ptr::null_mut();
    }

    (*rec).client = my_client;
    (*rec).duplex = duplex as c_int;
    (*rec).connect = 0;
    memset(pinfo.as_mut_ptr().cast(), 0, core::mem::size_of::<snd_seq_port_info>());
    let pinfo = pinfo.as_mut_ptr();
    (*pinfo).addr.client = my_client;
    if duplex {
        sprintf(
            (*pinfo).name.as_mut_ptr(),
            b"Midi Through Port-%d:%c\0".as_ptr().cast(),
            idx,
            if type_ != 0 { b'B' as c_int } else { b'A' as c_int },
        );
    } else {
        sprintf(
            (*pinfo).name.as_mut_ptr(),
            b"Midi Through Port-%d\0".as_ptr().cast(),
            idx,
        );
    }
    (*pinfo).capability = SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;
    (*pinfo).capability |= SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
    if duplex {
        (*pinfo).capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
    }
    (*pinfo).direction = SNDRV_SEQ_PORT_DIR_BIDIRECTION;
    (*pinfo).type_ =
        SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC | SNDRV_SEQ_PORT_TYPE_SOFTWARE | SNDRV_SEQ_PORT_TYPE_PORT;
    memset(
        pcb.as_mut_ptr().cast(),
        0,
        core::mem::size_of::<snd_seq_port_callback>(),
    );
    let pcb = pcb.as_mut_ptr();
    (*pcb).owner = THIS_MODULE;
    (*pcb).event_input = Some(dummy_input);
    (*pcb).private_free = Some(dummy_free);
    (*pcb).private_data = rec.cast();
    (*pinfo).kernel = pcb;
    if snd_seq_kernel_client_ctl(
        my_client,
        SNDRV_SEQ_IOCTL_CREATE_PORT,
        pinfo.cast(),
    ) < 0
    {
        kfree(rec.cast());
        return ptr::null_mut();
    }
    (*rec).port = (*pinfo).addr.port;
    rec
}

/*
 * register client and create ports
 */
unsafe fn register_client() -> c_int {
    let mut rec1: *mut snd_seq_dummy_port;
    let mut rec2: *mut snd_seq_dummy_port;
    #[cfg(CONFIG_SND_SEQ_UMP)]
    let client: *mut snd_seq_client;
    let mut i: c_int;

    if ports < 1 {
        pr_err(
            b"ALSA: seq_dummy: invalid number of ports %d\n\0"
                .as_ptr()
                .cast(),
            ports,
        );
        return -EINVAL;
    }

    /* create client */
    my_client = snd_seq_create_kernel_client(
        ptr::null_mut(),
        SNDRV_SEQ_CLIENT_DUMMY,
        b"Midi Through\0".as_ptr().cast(),
    );
    if my_client < 0 {
        return my_client;
    }

    // #if IS_ENABLED(CONFIG_SND_SEQ_UMP)
    #[cfg(CONFIG_SND_SEQ_UMP)]
    {
        client = snd_seq_kernel_client_get(my_client);
        if client.is_null() {
            return -EINVAL;
        }
        match ump {
            1 => {
                (*client).midi_version = SNDRV_SEQ_CLIENT_UMP_MIDI_1_0;
            }
            2 => {
                (*client).midi_version = SNDRV_SEQ_CLIENT_UMP_MIDI_2_0;
            }
            _ => {
                /* don't convert events but just pass-through */
                (*client).filter = SNDRV_SEQ_FILTER_NO_CONVERT;
            }
        }
        snd_seq_kernel_client_put(client);
    }
    // #endif

    /* create ports */
    i = 0;
    while i < ports {
        rec1 = create_port(i, 0);
        if rec1.is_null() {
            snd_seq_delete_kernel_client(my_client);
            return -ENOMEM;
        }
        if duplex {
            rec2 = create_port(i, 1);
            if rec2.is_null() {
                snd_seq_delete_kernel_client(my_client);
                return -ENOMEM;
            }
            (*rec1).connect = (*rec2).port;
            (*rec2).connect = (*rec1).port;
        }
        i += 1;
    }

    0
}

/*
 * delete client if exists
 */
unsafe fn delete_client() {
    if my_client >= 0 {
        snd_seq_delete_kernel_client(my_client);
    }
}

/*
 *  Init part
 */

unsafe fn alsa_seq_dummy_init() -> c_int {
    register_client()
}

unsafe fn alsa_seq_dummy_exit() {
    delete_client();
}

// module_init(alsa_seq_dummy_init)
// module_exit(alsa_seq_dummy_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
