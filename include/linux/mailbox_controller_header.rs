/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_void;
type c_ulong = usize;

/* Types supplied by the corresponding kernel headers. */
pub enum device {}
pub enum completion {}
pub enum hrtimer {}
pub enum fwnode_reference_args {}
pub enum of_phandle_args {}
pub enum spinlock_t {}
pub enum list_head {}

pub struct mbox_chan;
pub struct mbox_client;

/* Sentinel value distinguishing "no active request" from "NULL message data" */
pub const MBOX_NO_MSG: *mut c_void = (-1isize) as *mut c_void;

pub const MBOX_TXDONE_BY_IRQ: u32 = 1u32 << 0; /* controller has remote RTR irq */
pub const MBOX_TXDONE_BY_POLL: u32 = 1u32 << 1; /* controller can read status of last TX */
pub const MBOX_TXDONE_BY_ACK: u32 = 1u32 << 2; /* S/W ACK received by Client ticks the TX */

#[repr(C)]
pub struct mbox_chan_ops {
    pub send_data: Option<unsafe extern "C" fn(chan: *mut mbox_chan, data: *mut c_void) -> i32>,
    pub flush: Option<unsafe extern "C" fn(chan: *mut mbox_chan, timeout: c_ulong) -> i32>,
    pub startup: Option<unsafe extern "C" fn(chan: *mut mbox_chan) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(chan: *mut mbox_chan)>,
    pub last_tx_done: Option<unsafe extern "C" fn(chan: *mut mbox_chan) -> bool>,
    pub peek_data: Option<unsafe extern "C" fn(chan: *mut mbox_chan) -> bool>,
}

#[repr(C)]
pub struct mbox_controller {
    pub dev: *mut device,
    pub ops: *const mbox_chan_ops,
    pub chans: *mut mbox_chan,
    pub num_chans: i32,
    pub txdone_irq: bool,
    pub txdone_poll: bool,
    pub txpoll_period: u32,
    pub fw_xlate: Option<unsafe extern "C" fn(
        mbox: *mut mbox_controller,
        sp: *const fwnode_reference_args,
    ) -> *mut mbox_chan>,
    pub of_xlate: Option<unsafe extern "C" fn(
        mbox: *mut mbox_controller,
        sp: *const of_phandle_args,
    ) -> *mut mbox_chan>,
    /* Internal to API */
    pub poll_hrt: hrtimer,
    pub poll_hrt_lock: spinlock_t,
    pub node: list_head,
}

pub const MBOX_TX_QUEUE_LEN: usize = 20;

#[repr(C)]
pub struct mbox_chan {
    pub mbox: *mut mbox_controller,
    pub txdone_method: u32,
    pub cl: *mut mbox_client,
    pub tx_complete: completion,
    pub tx_status: i32,
    pub active_req: *mut c_void,
    pub msg_count: u32,
    pub msg_free: u32,
    pub msg_data: [*mut c_void; MBOX_TX_QUEUE_LEN],
    pub lock: spinlock_t, /* Serialise access to the channel */
    pub con_priv: *mut c_void,
}

unsafe extern "C" {
    pub fn mbox_controller_register(mbox: *mut mbox_controller) -> i32; /* can sleep */
    pub fn mbox_controller_unregister(mbox: *mut mbox_controller); /* can sleep */
    pub fn mbox_chan_received_data(chan: *mut mbox_chan, data: *mut c_void); /* atomic */
    pub fn mbox_chan_txdone(chan: *mut mbox_chan, r: i32); /* atomic */
    pub fn devm_mbox_controller_register(
        dev: *mut device,
        mbox: *mut mbox_controller,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
