/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013-2014 Linaro Ltd.
 * Author: Jassi Brar <jassisinghbrar@gmail.com>
 */

// Dependencies supplied by other headers in the original source:
// <linux/device.h>, <linux/of.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbox_chan {
    _private: [u8; 0],
}

/**
 * struct mbox_client - User of a mailbox
 * @dev:            The client device
 * @tx_block:       If the mbox_send_message should block until data is
 *                  transmitted.
 * @tx_tout:        Max block period in ms before TX is assumed failure
 * @knows_txdone:   If the client could run the TX state machine. Usually
 *                  if the client receives some ACK packet for transmission.
 *                  Unused if the controller already has TX_Done/RTR IRQ.
 * @rx_callback:    Atomic callback to provide client the data received
 * @tx_prepare:     Atomic callback to ask client to prepare the payload
 *                  before initiating the transmission if required.
 * @tx_done:        Atomic callback to tell client of data transmission
 */
#[repr(C)]
pub struct mbox_client {
    pub dev: *mut device,
    pub tx_block: bool,
    pub tx_tout: c_ulong,
    pub knows_txdone: bool,

    pub rx_callback: Option<unsafe extern "C" fn(cl: *mut mbox_client, mssg: *mut c_void)>,
    pub tx_prepare: Option<unsafe extern "C" fn(cl: *mut mbox_client, mssg: *mut c_void)>,
    pub tx_done:
        Option<unsafe extern "C" fn(cl: *mut mbox_client, mssg: *mut c_void, r: c_int)>,
}

extern "C" {
    pub fn mbox_bind_client(chan: *mut mbox_chan, cl: *mut mbox_client) -> c_int;
    pub fn mbox_request_channel_byname(
        cl: *mut mbox_client,
        name: *const c_char,
    ) -> *mut mbox_chan;
    pub fn mbox_request_channel(cl: *mut mbox_client, index: c_int) -> *mut mbox_chan;
    pub fn mbox_send_message(chan: *mut mbox_chan, mssg: *mut c_void) -> c_int;
    pub fn mbox_flush(chan: *mut mbox_chan, timeout: c_ulong) -> c_int;
    pub fn mbox_client_txdone(chan: *mut mbox_chan, r: c_int); /* atomic */
    pub fn mbox_client_peek_data(chan: *mut mbox_chan) -> bool; /* atomic */
    pub fn mbox_chan_tx_slots_available(chan: *mut mbox_chan) -> c_uint; /* atomic */
    pub fn mbox_free_channel(chan: *mut mbox_chan); /* may sleep */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
