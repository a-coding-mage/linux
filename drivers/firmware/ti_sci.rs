// SPDX-License-Identifier: GPL-2.0
// Texas Instruments System Control Interface Protocol Driver
//
// This is a source-level Rust translation.  Kernel and protocol types referenced
// here are supplied by the surrounding kernel translation unit.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::c_void;

// External Linux-kernel types and helpers are intentionally unresolved here;
// they are provided by the translated dependencies.
extern "C" {
    fn ti_sci_get_one_xfer(info: *mut ti_sci_info, msg_type: u16, msg_flags: u32,
                           tx_message_size: usize, rx_message_size: usize) -> *mut ti_sci_xfer;
    fn ti_sci_put_one_xfer(info: *mut ti_sci_xfers_info, xfer: *mut ti_sci_xfer);
    fn ti_sci_do_xfer(info: *mut ti_sci_info, xfer: *mut ti_sci_xfer) -> i32;
}

#[repr(C)]
pub struct ti_sci_xfer { pub tx_message: ti_msgmgr_message, pub rx_len: u8, pub xfer_buf: *mut u8, pub done: completion }
#[repr(C)]
pub struct ti_sci_xfers_info { pub sem_xfer_count: semaphore, pub xfer_block: *mut ti_sci_xfer, pub xfer_alloc_table: *mut c_ulong, pub xfer_lock: spinlock_t }
#[repr(C)]
pub struct ti_sci_desc { pub default_host_id: u8, pub max_rx_timeout_ms: i32, pub max_msgs: i32, pub max_msg_size: i32 }
#[repr(C)]
pub struct ti_sci_info {
    pub dev: *mut device, pub desc: *const ti_sci_desc, pub d: *mut dentry,
    pub debug_region: *mut c_void, pub debug_buffer: *mut i8, pub debug_region_size: usize,
    pub handle: ti_sci_handle, pub cl: mbox_client, pub chan_tx: *mut mbox_chan,
    pub chan_rx: *mut mbox_chan, pub minfo: ti_sci_xfers_info, pub node: list_head,
    pub irqs: [c_ulong; 32], pub irq_lock: mutex, pub host_id: u8, pub fw_caps: u64, pub users: i32,
}

#[repr(C)] pub struct ti_msgmgr_message { pub buf: *mut u8, pub len: usize, pub chan_rx: *mut mbox_chan, pub timeout_rx_ms: i32 }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct semaphore { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mbox_client { _private: [u8; 0] }
#[repr(C)] pub struct mbox_chan { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ti_sci_handle { pub version: ti_sci_version_info }
#[repr(C)] pub struct ti_sci_version_info { pub abi_major: u8, pub abi_minor: u8, pub firmware_revision: u16, pub firmware_description: [i8; 64] }
#[repr(C)] pub struct ti_sci_msg_hdr { pub type_: u16, pub host: u8, pub seq: u8, pub flags: u32 }
#[repr(C)] pub struct ti_sci_msg_resp_version { pub hdr: ti_sci_msg_hdr, pub abi_major: u8, pub abi_minor: u8, pub firmware_revision: u16, pub firmware_description: [i8; 64] }

type c_ulong = usize;

// Protocol constants are supplied by ti_sci_protocol.h in the complete build.
extern "Rust" {
    static TI_SCI_MSG_VERSION: u16;
    static TI_SCI_FLAG_REQ_ACK_ON_PROCESSED: u32;
}

unsafe fn ti_sci_is_response_ack(r: *const c_void) -> bool {
    let hdr = r as *const ti_sci_msg_hdr;
    ((*hdr).flags & 0x1) != 0
}

unsafe fn ti_sci_cmd_get_revision(info: *mut ti_sci_info) -> i32 {
    let xfer = ti_sci_get_one_xfer(info, TI_SCI_MSG_VERSION,
        TI_SCI_FLAG_REQ_ACK_ON_PROCESSED,
        core::mem::size_of::<ti_sci_msg_hdr>(), core::mem::size_of::<ti_sci_msg_resp_version>());
    if xfer.is_null() { return -12; }
    let rev = (*xfer).xfer_buf as *mut ti_sci_msg_resp_version;
    let ret = ti_sci_do_xfer(info, xfer);
    if ret == 0 {
        (*info).handle.version.abi_major = (*rev).abi_major;
        (*info).handle.version.abi_minor = (*rev).abi_minor;
        (*info).handle.version.firmware_revision = (*rev).firmware_revision;
        (*info).handle.version.firmware_description.copy_from_slice(&(*rev).firmware_description);
    }
    ti_sci_put_one_xfer(&mut (*info).minfo, xfer);
    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
