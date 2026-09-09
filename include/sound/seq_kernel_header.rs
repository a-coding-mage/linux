/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Main kernel header file for the ALSA sequencer
 *  Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 */
// Dependencies supplied by the surrounding kernel and ALSA translation units.

pub type snd_seq_real_time_t = snd_seq_real_time;
pub type snd_seq_timestamp_t = snd_seq_timestamp;

/* maximum number of queues */
pub const SNDRV_SEQ_MAX_QUEUES: i32 = 32;

/* max number of concurrent clients */
pub const SNDRV_SEQ_MAX_CLIENTS: i32 = 192;

/* max number of concurrent ports */
pub const SNDRV_SEQ_MAX_PORTS: i32 = 254;

/* max number of events in memory pool */
pub const SNDRV_SEQ_MAX_EVENTS: i32 = 2000;

/* default number of events in memory pool */
pub const SNDRV_SEQ_DEFAULT_EVENTS: i32 = 500;

/* max number of events in memory pool for one client (outqueue) */
pub const SNDRV_SEQ_MAX_CLIENT_EVENTS: i32 = 2000;

/* default number of events in memory pool for one client (outqueue) */
pub const SNDRV_SEQ_DEFAULT_CLIENT_EVENTS: i32 = 200;

/* max delivery path length */
/* NOTE: this shouldn't be greater than MAX_LOCKDEP_SUBCLASSES */
pub const SNDRV_SEQ_MAX_HOPS: i32 = 8;

/* max size of event size */
pub const SNDRV_SEQ_MAX_EVENT_LEN: i32 = 0x3fffffff;

/* call-backs for kernel port */
#[repr(C)]
pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub private_data: *mut core::ffi::c_void,
    pub subscribe: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub unsubscribe: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub use_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub unuse: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut snd_seq_port_subscribe) -> i32>,
    pub event_input: Option<unsafe extern "C" fn(*mut snd_seq_event, i32, *mut core::ffi::c_void, i32, i32) -> i32>,
    pub private_free: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    /*...*/
}

/* interface for kernel client */
// C attribute: __printf(3, 4)
unsafe extern "C" {
    pub fn snd_seq_create_kernel_client(card: *mut snd_card, client_index: i32, name_fmt: *const core::ffi::c_char, ...) -> i32;
    pub fn snd_seq_delete_kernel_client(client: i32) -> i32;
    pub fn snd_seq_kernel_client_enqueue(client: i32, ev: *mut snd_seq_event, file: *mut file, blocking: bool) -> i32;
    pub fn snd_seq_kernel_client_dispatch(client: i32, ev: *mut snd_seq_event, atomic: i32, hop: i32) -> i32;
    pub fn snd_seq_kernel_client_ctl(client: i32, cmd: u32, arg: *mut core::ffi::c_void) -> i32;
}

pub const SNDRV_SEQ_EXT_MASK: u32 = 0xc0000000;
pub const SNDRV_SEQ_EXT_USRPTR: u32 = 0x80000000;
pub const SNDRV_SEQ_EXT_CHAINED: u32 = 0x40000000;

pub type snd_seq_dump_func_t = unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> i32;

unsafe extern "C" {
    pub fn snd_seq_expand_var_event(event: *const snd_seq_event, count: i32, buf: *mut core::ffi::c_char, in_kernel: i32, size_aligned: i32) -> i32;
    pub fn snd_seq_expand_var_event_at(event: *const snd_seq_event, count: i32, buf: *mut core::ffi::c_char, offset: i32) -> i32;
    pub fn snd_seq_dump_var_event(event: *const snd_seq_event, func: snd_seq_dump_func_t, private_data: *mut core::ffi::c_void) -> i32;
}

/* size of the event packet; it can be greater than snd_seq_event size */
#[inline]
pub unsafe fn snd_seq_event_packet_size(ev: *mut snd_seq_event) -> usize {
    if snd_seq_ev_is_ump(ev) {
        core::mem::size_of::<snd_seq_ump_event>()
    } else {
        core::mem::size_of::<snd_seq_event>()
    }
}

/* interface for OSS emulation */
unsafe extern "C" {
    pub fn snd_seq_set_queue_tempo(client: i32, tempo: *mut snd_seq_queue_tempo) -> i32;

    /* port attach/detach */
    pub fn snd_seq_event_port_attach(client: i32, pcbp: *mut snd_seq_port_callback, cap: i32, type_: i32, midi_channels: i32, midi_voices: i32, portname: *mut core::ffi::c_char) -> i32;
    pub fn snd_seq_event_port_detach(client: i32, port: i32) -> i32;
}

// CONFIG_MODULES controls whether these kernel module hooks are provided.
#[cfg(CONFIG_MODULES)]
unsafe extern "C" {
    pub fn snd_seq_autoload_init();
    pub fn snd_seq_autoload_exit();
}

#[cfg(not(CONFIG_MODULES))]
#[inline]
pub const fn snd_seq_autoload_init() {}

#[cfg(not(CONFIG_MODULES))]
#[inline]
pub const fn snd_seq_autoload_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
