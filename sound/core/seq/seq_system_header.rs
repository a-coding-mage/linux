/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  ALSA sequencer System Client
 *  Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

// Dependency intent from C header: #include <sound/seq_kernel.h>

use core::ffi::c_int;

extern "C" {
    /* entry points for broadcasting system events */
    pub fn snd_seq_system_broadcast(client: c_int, port: c_int, type_: c_int, atomic: bool);

    pub fn snd_seq_system_notify(
        client: c_int,
        port: c_int,
        ev: *mut crate::snd_seq_event,
        atomic: bool,
    ) -> c_int;

    /* register our internal client */
    pub fn snd_seq_system_client_init() -> c_int;

    /* unregister our internal client */
    pub fn snd_seq_system_client_done();
}

/* normal system notification event broadcast */
#[inline]
pub unsafe fn notify_event(client: c_int, port: c_int, type_: c_int) {
    snd_seq_system_broadcast(client, port, type_, false);
}

/* notify UMP EP/FB change event */
#[inline]
pub unsafe fn snd_seq_system_ump_notify(client: c_int, block: c_int, type_: c_int, atomic: bool) {
    /*
     * reuse the existing snd_seq_system_broadcast():
     * struct snd_seq_ev_ump_notify is compatible with struct snd_seq_addr
     */
    snd_seq_system_broadcast(client, block, type_, atomic);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_client_start(client: c_int) {
    notify_event(client, 0, crate::SNDRV_SEQ_EVENT_CLIENT_START);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_client_exit(client: c_int) {
    notify_event(client, 0, crate::SNDRV_SEQ_EVENT_CLIENT_EXIT);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_client_change(client: c_int) {
    notify_event(client, 0, crate::SNDRV_SEQ_EVENT_CLIENT_CHANGE);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_port_start(client: c_int, port: c_int) {
    notify_event(client, port, crate::SNDRV_SEQ_EVENT_PORT_START);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_port_exit(client: c_int, port: c_int) {
    notify_event(client, port, crate::SNDRV_SEQ_EVENT_PORT_EXIT);
}

#[inline]
pub unsafe fn snd_seq_system_client_ev_port_change(client: c_int, port: c_int) {
    notify_event(client, port, crate::SNDRV_SEQ_EVENT_PORT_CHANGE);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
