/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer Client Manager
 *   Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Dependencies in the original C header:
 * <sound/seq_kernel.h>, <linux/bitops.h>, "seq_fifo.h", "seq_ports.h",
 * and "seq_lock.h".
 */

/* client manager */

pub const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 {
        c_uint::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

pub const SND_SEQ_GROUP_FILTER_MASK: c_uint = GENMASK(SNDRV_UMP_MAX_GROUPS, 0);
pub const SND_SEQ_GROUP_FILTER_GROUPS: c_uint = GENMASK(SNDRV_UMP_MAX_GROUPS, 1);

#[repr(C)]
pub struct snd_seq_user_client {
    pub file: *mut file, /* file struct of client */
    /* ... */
    pub owner: *mut pid,

    /* fifo */
    pub fifo: *mut snd_seq_fifo, /* queue for incoming events */
    pub fifo_pool_size: c_int,
}

#[repr(C)]
pub struct snd_seq_kernel_client {
    /* ... */
    pub card: *mut snd_card,
}

#[repr(C)]
pub union snd_seq_client_data {
    pub user: core::mem::ManuallyDrop<snd_seq_user_client>,
    pub kernel: core::mem::ManuallyDrop<snd_seq_kernel_client>,
}

#[repr(C)]
pub struct snd_seq_client {
    pub type_: snd_seq_client_type_t,
    /* C bitfields:
     * unsigned int accept_input: 1, accept_output: 1;
     */
    pub accept_input_accept_output: c_uint,
    pub midi_version: c_uint,
    pub user_pversion: c_uint,
    pub name: [c_char; 64], /* client name */
    pub number: c_int,      /* client number */
    pub filter: c_uint,     /* filter flags */
    /* C declaration: DECLARE_BITMAP(event_filter, 256); */
    pub event_filter: [core::ffi::c_ulong; __BITS_TO_LONGS(256)],
    pub group_filter: c_uint,
    pub use_lock: snd_use_lock_t,
    pub event_lost: c_int,
    /* ports */
    pub num_ports: c_int, /* number of ports */
    pub ports_list_head: list_head,
    pub ports_mutex: mutex,
    pub ioctl_mutex: mutex,
    pub convert32: c_int, /* convert 32->64bit */
    pub ump_endpoint_port: c_int,

    /* output pool */
    pub pool: *mut snd_seq_pool, /* memory pool for this client */

    pub data: snd_seq_client_data,

    /* for UMP */
    pub ump_info: *mut *mut c_void,
}

/* usage statistics */
#[repr(C)]
pub struct snd_seq_usage {
    pub cur: c_int,
    pub peak: c_int,
}

unsafe extern "C" {
    pub fn client_init_data() -> c_int;
    pub fn snd_sequencer_device_init() -> c_int;
    pub fn snd_sequencer_device_done();

    /* get locked pointer to client */
    pub fn snd_seq_client_use_ptr(clientid: c_int) -> *mut snd_seq_client;
}

#[inline]
pub unsafe fn snd_seq_client_ref(client: *mut snd_seq_client) -> *mut snd_seq_client {
    unsafe {
        snd_use_lock_use(&mut (*client).use_lock);
    }
    client
}

/* unlock pointer to client */
#[inline]
pub unsafe fn snd_seq_client_unref(client: *mut snd_seq_client) {
    unsafe {
        snd_use_lock_free(&mut (*client).use_lock);
    }
}

/* C cleanup helper:
 * DEFINE_FREE(snd_seq_client, struct snd_seq_client *,
 *             if (!IS_ERR_OR_NULL(_T)) snd_seq_client_unref(_T))
 */

unsafe extern "C" {
    /* dispatch event to client(s) */
    pub fn snd_seq_dispatch_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int)
        -> c_int;

    pub fn snd_seq_kernel_client_write_poll(
        clientid: c_int,
        file: *mut file,
        wait: *mut poll_table,
    ) -> c_int;
    pub fn snd_seq_client_notify_subscription(
        client: c_int,
        port: c_int,
        info: *mut snd_seq_port_subscribe,
        evtype: c_int,
    ) -> c_int;

    pub fn __snd_seq_deliver_single_event(
        dest: *mut snd_seq_client,
        dest_port: *mut snd_seq_client_port,
        event: *mut snd_seq_event,
        atomic: c_int,
        hop: c_int,
    ) -> c_int;

    /* only for OSS sequencer */
    pub fn snd_seq_kernel_client_ioctl(
        clientid: c_int,
        cmd: c_uint,
        arg: *mut c_void,
    ) -> c_int;

    pub static mut seq_client_load: [c_int; 15];

    /* for internal use between kernel sequencer clients */
    pub fn snd_seq_kernel_client_get(client: c_int) -> *mut snd_seq_client;
    pub fn snd_seq_kernel_client_put(cptr: *mut snd_seq_client);
}

#[inline]
pub unsafe fn snd_seq_client_is_ump(c: *mut snd_seq_client) -> bool {
    unsafe { (*c).midi_version != SNDRV_SEQ_CLIENT_LEGACY_MIDI }
}

#[inline]
pub unsafe fn snd_seq_client_is_midi2(c: *mut snd_seq_client) -> bool {
    unsafe { (*c).midi_version == SNDRV_SEQ_CLIENT_UMP_MIDI_2_0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
