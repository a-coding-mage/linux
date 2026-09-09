/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for the compaq Micro MFD
 */

// C header dependencies: linux/spinlock.h, linux/completion.h, linux/list.h

pub const TX_BUF_SIZE: usize = 32;
pub const RX_BUF_SIZE: usize = 16;
pub const CHAR_SOF: u8 = 0x02;

/*
 * These are the different messages that can be sent to the microcontroller
 * to control various aspects.
 */
pub const MSG_VERSION: u8 = 0x0;
pub const MSG_KEYBOARD: u8 = 0x2;
pub const MSG_TOUCHSCREEN: u8 = 0x3;
pub const MSG_EEPROM_READ: u8 = 0x4;
pub const MSG_EEPROM_WRITE: u8 = 0x5;
pub const MSG_THERMAL_SENSOR: u8 = 0x6;
pub const MSG_NOTIFY_LED: u8 = 0x8;
pub const MSG_BATTERY: u8 = 0x9;
pub const MSG_SPI_READ: u8 = 0xb;
pub const MSG_SPI_WRITE: u8 = 0xc;
pub const MSG_BACKLIGHT: u8 = 0xd; // H3600 only
pub const MSG_CODEC_CTRL: u8 = 0xe; // H3100 only
pub const MSG_DISPLAY_CTRL: u8 = 0xf; // H3100 only

/* state of receiver parser */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RxState {
    StateSof = 0, // Next byte should be start of frame
    StateId,      // Next byte is ID & message length
    StateData,    // Next byte is a data byte
    StateChksum,  // Next byte should be checksum
}

/* TX state */
#[repr(C)]
pub struct IpaqMicroTxdev {
    pub len: u8,
    pub index: u8,
    pub buf: [u8; TX_BUF_SIZE],
}

/* RX state */
#[repr(C)]
pub struct IpaqMicroRxdev {
    pub state: RxState,
    pub chksum: u8,
    pub id: u8,
    pub len: core::ffi::c_uint,
    pub index: core::ffi::c_uint,
    pub buf: [u8; RX_BUF_SIZE],
}

/* message to the iPAQ microcontroller */
#[repr(C)]
pub struct IpaqMicroMsg {
    pub id: u8,
    pub tx_len: u8,
    pub tx_data: [u8; TX_BUF_SIZE],
    pub rx_len: u8,
    pub rx_data: [u8; RX_BUF_SIZE],
    pub ack: Completion,
    pub node: ListHead,
}

/* iPAQ microcontroller state */
#[repr(C)]
pub struct IpaqMicro {
    pub dev: *mut Device,
    pub base: *mut core::ffi::c_void,
    pub sdlc: *mut core::ffi::c_void,
    pub version: [core::ffi::c_char; 5],
    pub tx: IpaqMicroTxdev, /* transmit ISR state */
    pub rx: IpaqMicroRxdev, /* receive ISR state */
    pub lock: Spinlock,
    pub msg: *mut IpaqMicroMsg,
    pub queue: ListHead,
    pub key: Option<unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int, *mut u8)>,
    pub key_data: *mut core::ffi::c_void,
    pub ts: Option<unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int, *mut u8)>,
    pub ts_data: *mut core::ffi::c_void,
}

extern "C" {
    pub fn ipaq_micro_tx_msg(micro: *mut IpaqMicro, msg: *mut IpaqMicroMsg) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn ipaq_micro_tx_msg_sync(
    micro: *mut IpaqMicro,
    msg: *mut IpaqMicroMsg,
) -> core::ffi::c_int {
    init_completion(core::ptr::addr_of_mut!((*msg).ack));
    let ret = ipaq_micro_tx_msg(micro, msg);
    wait_for_completion(core::ptr::addr_of_mut!((*msg).ack));
    ret
}

#[inline]
pub unsafe fn ipaq_micro_tx_msg_async(
    micro: *mut IpaqMicro,
    msg: *mut IpaqMicroMsg,
) -> core::ffi::c_int {
    init_completion(core::ptr::addr_of_mut!((*msg).ack));
    ipaq_micro_tx_msg(micro, msg)
}

// Types and functions supplied by the Linux dependencies included by the C header.
extern "C" {
    pub type Device;
    pub type Completion;
    pub type ListHead;
    pub type Spinlock;
    pub fn init_completion(completion: *mut Completion);
    pub fn wait_for_completion(completion: *mut Completion);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
