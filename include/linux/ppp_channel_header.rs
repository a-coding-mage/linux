/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions for the interface between the generic PPP code
 * and a PPP channel.
 *
 * A PPP channel provides a way for the generic PPP code to send
 * and receive packets over some sort of communications medium.
 * Packets are stored in sk_buffs and have the 2-byte PPP protocol
 * number at the start, but not the address and control bytes.
 *
 * Copyright 1999 Paul Mackerras.
 *
 * ==FILEVERSION 20000322==
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original header's include directives.

#[repr(C)]
pub struct net_device_path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device_path_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ppp_channel {
    pub private: *mut core::ffi::c_void,
    pub ops: *const ppp_channel_ops,
    pub mtu: core::ffi::c_int,
    pub hdrlen: core::ffi::c_int,
    pub ppp: *mut core::ffi::c_void,
    pub speed: core::ffi::c_int,
    pub direct_xmit: bool,
}

#[repr(C)]
pub struct ppp_channel_ops {
    /* Send a packet (or multilink fragment) on this channel.
       Returns 1 if it was accepted, 0 if not. */
    pub start_xmit:
        Option<unsafe extern "C" fn(*mut ppp_channel, *mut sk_buff) -> core::ffi::c_int>,
    /* Handle an ioctl call that has come in via /dev/ppp. */
    pub ioctl: Option<
        unsafe extern "C" fn(
            *mut ppp_channel,
            core::ffi::c_uint,
            core::ffi::c_ulong,
        ) -> core::ffi::c_int,
    >,
    pub fill_forward_path: Option<
        unsafe extern "C" fn(
            *mut net_device_path_ctx,
            *mut net_device_path,
            *const ppp_channel,
        ) -> core::ffi::c_int,
    >,
}

/* Opaque type supplied by the kernel skbuff definitions. */
pub struct sk_buff;

/* Called by the channel when it can send some more data. */
extern "C" {
    pub fn ppp_output_wakeup(channel: *mut ppp_channel);

    /* Called by the channel to process a received PPP packet.
       The packet should have just the 2-byte PPP protocol header. */
    pub fn ppp_input(channel: *mut ppp_channel, skb: *mut sk_buff);

    /* Called by the channel when an input error occurs, indicating
       that we may have missed a packet. */
    pub fn ppp_input_error(channel: *mut ppp_channel);

    /* Attach a channel to a given PPP unit in specified net. */
    pub fn ppp_register_net_channel(
        net: *mut net,
        channel: *mut ppp_channel,
    ) -> core::ffi::c_int;

    /* Attach a channel to a given PPP unit. */
    pub fn ppp_register_channel(channel: *mut ppp_channel) -> core::ffi::c_int;

    /* Detach a channel from its PPP unit (e.g. on hangup). */
    pub fn ppp_unregister_channel(channel: *mut ppp_channel);

    /* Get the channel number for a channel */
    pub fn ppp_channel_index(channel: *mut ppp_channel) -> core::ffi::c_int;

    /* Get the unit number associated with a channel, or -1 if none */
    pub fn ppp_unit_number(channel: *mut ppp_channel) -> core::ffi::c_int;

    /* Get the device name associated with a channel, or NULL if none.
     * Caller must hold RCU read lock.
     */
    pub fn ppp_dev_name(channel: *mut ppp_channel) -> *mut core::ffi::c_char;
}

/*
 * SMP locking notes:
 * The channel code must ensure that when it calls ppp_unregister_channel,
 * nothing is executing in any of the procedures above, for that
 * channel.  The generic layer will ensure that nothing is executing
 * in the start_xmit and ioctl routines for the channel by the time
 * that ppp_unregister_channel returns.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
