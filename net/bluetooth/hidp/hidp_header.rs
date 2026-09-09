/* SPDX-License-Identifier: GPL-2.0 */
/*
   HIDP implementation for Linux Bluetooth stack (BlueZ).
   Copyright (C) 2003-2004 Marcel Holtmann <marcel@holtmann.org>

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

// Dependencies supplied by the Linux and Bluetooth headers included by the C source:
// linux/types.h, linux/hid.h, linux/kref.h, net/bluetooth/bluetooth.h,
// net/bluetooth/l2cap.h

pub const HIDP_HEADER_TRANS_MASK: u8 = 0xf0;
pub const HIDP_HEADER_PARAM_MASK: u8 = 0x0f;

pub const HIDP_TRANS_HANDSHAKE: u8 = 0x00;
pub const HIDP_TRANS_HID_CONTROL: u8 = 0x10;
pub const HIDP_TRANS_GET_REPORT: u8 = 0x40;
pub const HIDP_TRANS_SET_REPORT: u8 = 0x50;
pub const HIDP_TRANS_GET_PROTOCOL: u8 = 0x60;
pub const HIDP_TRANS_SET_PROTOCOL: u8 = 0x70;
pub const HIDP_TRANS_GET_IDLE: u8 = 0x80;
pub const HIDP_TRANS_SET_IDLE: u8 = 0x90;
pub const HIDP_TRANS_DATA: u8 = 0xa0;
pub const HIDP_TRANS_DATC: u8 = 0xb0;

pub const HIDP_HSHK_SUCCESSFUL: u8 = 0x00;
pub const HIDP_HSHK_NOT_READY: u8 = 0x01;
pub const HIDP_HSHK_ERR_INVALID_REPORT_ID: u8 = 0x02;
pub const HIDP_HSHK_ERR_UNSUPPORTED_REQUEST: u8 = 0x03;
pub const HIDP_HSHK_ERR_INVALID_PARAMETER: u8 = 0x04;
pub const HIDP_HSHK_ERR_UNKNOWN: u8 = 0x0e;
pub const HIDP_HSHK_ERR_FATAL: u8 = 0x0f;

pub const HIDP_CTRL_NOP: u8 = 0x00;
pub const HIDP_CTRL_HARD_RESET: u8 = 0x01;
pub const HIDP_CTRL_SOFT_RESET: u8 = 0x02;
pub const HIDP_CTRL_SUSPEND: u8 = 0x03;
pub const HIDP_CTRL_EXIT_SUSPEND: u8 = 0x04;
pub const HIDP_CTRL_VIRTUAL_CABLE_UNPLUG: u8 = 0x05;

pub const HIDP_DATA_RTYPE_MASK: u8 = 0x03;
pub const HIDP_DATA_RSRVD_MASK: u8 = 0x0c;
pub const HIDP_DATA_RTYPE_OTHER: u8 = 0x00;
pub const HIDP_DATA_RTYPE_INPUT: u8 = 0x01;
pub const HIDP_DATA_RTYPE_OUPUT: u8 = 0x02;
pub const HIDP_DATA_RTYPE_FEATURE: u8 = 0x03;

pub const HIDP_PROTO_BOOT: u8 = 0x00;
pub const HIDP_PROTO_REPORT: u8 = 0x01;

// C ioctl macros preserved as source-level intent; their values depend on _IOW/_IOR.
// HIDPCONNADD    = _IOW('H', 200, int)
// HIDPCONNDEL    = _IOW('H', 201, int)
// HIDPGETCONNLIST = _IOR('H', 210, int)
// HIDPGETCONNINFO = _IOR('H', 211, int)

pub const HIDP_VIRTUAL_CABLE_UNPLUG: u32 = 0;
pub const HIDP_BOOT_PROTOCOL_MODE: u32 = 1;
pub const HIDP_BLUETOOTH_VENDOR_ID: u32 = 9;
pub const HIDP_WAITING_FOR_RETURN: u32 = 10;
pub const HIDP_WAITING_FOR_SEND_ACK: u32 = 11;

#[repr(C)]
pub struct hidp_connadd_req {
    pub ctrl_sock: i32,
    pub intr_sock: i32,
    pub parser: __u16,
    pub rd_size: __u16,
    pub rd_data: *mut __u8,
    pub country: __u8,
    pub subclass: __u8,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
    pub flags: __u32,
    pub idle_to: __u32,
    pub name: [core::ffi::c_char; 128],
}

#[repr(C)]
pub struct hidp_conndel_req {
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
}

#[repr(C)]
pub struct hidp_conninfo {
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
    pub state: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
    pub name: [core::ffi::c_char; 128],
}

#[repr(C)]
pub struct hidp_connlist_req {
    pub cnum: __u32,
    pub ci: *mut hidp_conninfo,
}

extern "C" {
    pub fn hidp_connection_add(req: *const hidp_connadd_req, ctrl_sock: *mut socket, intr_sock: *mut socket) -> i32;
    pub fn hidp_connection_del(req: *mut hidp_conndel_req) -> i32;
    pub fn hidp_get_connlist(req: *mut hidp_connlist_req) -> i32;
    pub fn hidp_get_conninfo(ci: *mut hidp_conninfo) -> i32;
}

#[repr(C)]
pub enum hidp_session_state {
    HIDP_SESSION_IDLING,
    HIDP_SESSION_PREPARING,
    HIDP_SESSION_RUNNING,
}

#[repr(C)]
pub struct hidp_session {
    pub list: list_head,
    pub ref_: kref,
    pub state: atomic_t,
    pub state_queue: wait_queue_head_t,
    pub terminate: atomic_t,
    pub task: *mut task_struct,
    pub flags: c_ulong,
    pub bdaddr: bdaddr_t,
    pub conn: *mut l2cap_conn,
    pub user: l2cap_user,
    pub ctrl_sock: *mut socket,
    pub intr_sock: *mut socket,
    pub ctrl_transmit: sk_buff_head,
    pub intr_transmit: sk_buff_head,
    pub ctrl_mtu: c_uint,
    pub intr_mtu: c_uint,
    pub idle_to: c_ulong,
    pub dev_init: work_struct,
    pub input: *mut input_dev,
    pub hid: *mut hid_device,
    pub timer: timer_list,
    pub rd_data: *mut __u8,
    pub rd_size: c_uint,
    pub keys: [u8; 8],
    pub leds: u8,
    pub waiting_report_type: i32,
    pub waiting_report_number: i32,
    pub report_mutex: mutex,
    pub report_return: *mut sk_buff,
    pub report_queue: wait_queue_head_t,
    pub output_report_success: i32,
    pub input_buf: [u8; HID_MAX_BUFFER_SIZE],
}

// __init and __exit are Linux section annotations in the C source.
pub unsafe extern "C" fn hidp_init_sockets() -> i32;
pub unsafe extern "C" fn hidp_cleanup_sockets();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
