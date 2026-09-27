/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   ALSA sequencer Ports
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/* C dependencies:
 * <sound/seq_kernel.h>
 * <sound/ump_convert.h>
 * "seq_lock.h"
 */

/* list of 'exported' ports */

/* Client ports that are not exported are still accessible, but are
 anonymous ports.

 If a port supports SUBSCRIPTION, that port can send events to all
 subscribersto a special address, with address
 (queue==SNDRV_SEQ_ADDRESS_SUBSCRIBERS). The message is then send to all
 recipients that are registered in the subscription list. A typical
 application for these SUBSCRIPTION events is handling of incoming MIDI
 data. The port doesn't 'know' what other clients are interested in this
 message. If for instance a MIDI recording application would like to receive
 the events from that port, it will first have to subscribe with that port.

*/

#[repr(C)]
pub struct snd_seq_subscribers {
    pub info: snd_seq_port_subscribe, /* additional info */
    pub src_list: hlist_node,         /* link of sources */
    pub dest_list: hlist_node,        /* link of destinations */
    pub ref_count: atomic_t,
    pub rcu: rcu_head,                /* for deferred free */
}

#[repr(C)]
pub struct snd_seq_port_subs_info {
    pub list_head: hlist_head, /* list of subscribed ports */
    pub count: kernel::ffi::c_uint, /* count of subscribers */
    pub exclusive: kernel::ffi::c_uint, /* C bitfield: unsigned int exclusive: 1; exclusive mode */
    pub list_mutex: rw_semaphore,
    pub open: Option<
        unsafe extern "C" fn(
            private_data: *mut kernel::ffi::c_void,
            info: *mut snd_seq_port_subscribe,
        ) -> kernel::ffi::c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(
            private_data: *mut kernel::ffi::c_void,
            info: *mut snd_seq_port_subscribe,
        ) -> kernel::ffi::c_int,
    >,
}

#[repr(C)]
pub struct snd_seq_client_port {
    pub addr: snd_seq_addr, /* client/port number */
    pub owner: *mut module, /* owner of this port */
    pub name: [kernel::ffi::c_char; 64], /* port name */
    pub list: list_head, /* port list */
    pub use_lock: snd_use_lock_t,

    /* subscribers */
    pub c_src: snd_seq_port_subs_info,  /* read (sender) list */
    pub c_dest: snd_seq_port_subs_info, /* write (dest) list */

    pub event_input: Option<
        unsafe extern "C" fn(
            ev: *mut snd_seq_event,
            direct: kernel::ffi::c_int,
            private_data: *mut kernel::ffi::c_void,
            atomic: kernel::ffi::c_int,
            hop: kernel::ffi::c_int,
        ) -> kernel::ffi::c_int,
    >,
    pub private_free:
        Option<unsafe extern "C" fn(private_data: *mut kernel::ffi::c_void)>,
    pub private_data: *mut kernel::ffi::c_void,
    pub closing: kernel::ffi::c_uint, /* C bitfield: unsigned int closing : 1; */
    pub timestamping: kernel::ffi::c_uint, /* C bitfield: unsigned int timestamping: 1; */
    pub time_real: kernel::ffi::c_uint, /* C bitfield: unsigned int time_real: 1; */
    pub time_queue: kernel::ffi::c_int,

    /* capability, inport, output, sync */
    pub capability: kernel::ffi::c_uint, /* port capability bits */
    pub type_: kernel::ffi::c_uint,      /* port type bits */

    /* supported channels */
    pub midi_channels: kernel::ffi::c_int,
    pub midi_voices: kernel::ffi::c_int,
    pub synth_voices: kernel::ffi::c_int,

    /* UMP direction and group */
    pub direction: kernel::ffi::c_uchar,
    pub ump_group: kernel::ffi::c_uchar,

    pub is_midi1: bool, /* keep MIDI 1.0 protocol */

    /* Present in C when IS_ENABLED(CONFIG_SND_SEQ_UMP). */
    #[cfg(CONFIG_SND_SEQ_UMP)]
    pub midi2_bank: [ump_cvt_to_ump_bank; 16], /* per channel */
}

#[repr(C)]
pub struct snd_seq_client {
    _unused: [u8; 0],
}

unsafe extern "C" {
    /* return pointer to port structure and lock port */
    pub fn snd_seq_port_use_ptr(
        client: *mut snd_seq_client,
        num: kernel::ffi::c_int,
    ) -> *mut snd_seq_client_port;

    /* search for next port - port is locked if found */
    pub fn snd_seq_port_query_nearest(
        client: *mut snd_seq_client,
        pinfo: *mut snd_seq_port_info,
    ) -> *mut snd_seq_client_port;
}

/* unlock the port */
#[inline]
pub unsafe fn snd_seq_port_unlock(port: *mut snd_seq_client_port) {
    unsafe {
        snd_use_lock_free(&mut (*port).use_lock);
    }
}

/* C DEFINE_FREE(snd_seq_port, struct snd_seq_client_port *,
 *     if (!IS_ERR_OR_NULL(_T)) snd_seq_port_unlock(_T))
 */

unsafe extern "C" {
    /* create a port, 0 on success or a negative error code is returned */
    pub fn snd_seq_create_port(
        client: *mut snd_seq_client,
        port_ret: *mut *mut snd_seq_client_port,
    ) -> kernel::ffi::c_int;

    /* insert the port; return the port address or a negative error code */
    pub fn snd_seq_insert_port(
        client: *mut snd_seq_client,
        port: kernel::ffi::c_int,
        new_port: *mut snd_seq_client_port,
    ) -> kernel::ffi::c_int;

    /* delete a port */
    pub fn snd_seq_delete_port(
        client: *mut snd_seq_client,
        port: kernel::ffi::c_int,
    ) -> kernel::ffi::c_int;

    /* delete all ports */
    pub fn snd_seq_delete_all_ports(client: *mut snd_seq_client) -> kernel::ffi::c_int;

    /* set port info fields */
    pub fn snd_seq_set_port_info(
        port: *mut snd_seq_client_port,
        info: *mut snd_seq_port_info,
    ) -> kernel::ffi::c_int;

    /* get port info fields */
    pub fn snd_seq_get_port_info(
        port: *mut snd_seq_client_port,
        info: *mut snd_seq_port_info,
    ) -> kernel::ffi::c_int;

    /* add subscriber to subscription list */
    pub fn snd_seq_port_connect(
        caller: *mut snd_seq_client,
        s: *mut snd_seq_client,
        sp: *mut snd_seq_client_port,
        d: *mut snd_seq_client,
        dp: *mut snd_seq_client_port,
        info: *mut snd_seq_port_subscribe,
    ) -> kernel::ffi::c_int;

    /* remove subscriber from subscription list */
    pub fn snd_seq_port_disconnect(
        caller: *mut snd_seq_client,
        s: *mut snd_seq_client,
        sp: *mut snd_seq_client_port,
        d: *mut snd_seq_client,
        dp: *mut snd_seq_client_port,
        info: *mut snd_seq_port_subscribe,
    ) -> kernel::ffi::c_int;

    /* subscribe port */
    pub fn snd_seq_port_subscribe(
        port: *mut snd_seq_client_port,
        info: *mut snd_seq_port_subscribe,
    ) -> kernel::ffi::c_int;

    /* get matched subscriber */
    pub fn snd_seq_port_get_subscription(
        src_grp: *mut snd_seq_port_subs_info,
        dest_addr: *mut snd_seq_addr,
        subs: *mut snd_seq_port_subscribe,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
