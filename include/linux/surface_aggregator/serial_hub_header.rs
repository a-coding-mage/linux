/* SPDX-License-Identifier: GPL-2.0+ */
/* Surface Serial Hub (SSH) protocol and communication interface. */

// External Linux/kernel types and helpers are supplied by other translated files.

#[repr(u8)]
pub enum ssh_frame_type {
    SSH_FRAME_TYPE_DATA_SEQ = 0x80,
    SSH_FRAME_TYPE_DATA_NSQ = 0x00,
    SSH_FRAME_TYPE_ACK = 0x40,
    SSH_FRAME_TYPE_NAK = 0x04,
}

#[repr(C, packed)]
pub struct ssh_frame {
    pub type_: u8,
    pub len: __le16,
    pub seq: u8,
}

pub const SSH_FRAME_MAX_PAYLOAD_SIZE: u16 = u16::MAX;

#[repr(u8)]
pub enum ssh_payload_type {
    SSH_PLD_TYPE_CMD = 0x80,
}

#[repr(C, packed)]
pub struct ssh_command {
    pub type_: u8,
    pub tc: u8,
    pub tid: u8,
    pub sid: u8,
    pub iid: u8,
    pub rqid: __le16,
    pub cid: u8,
}

pub const SSH_COMMAND_MAX_PAYLOAD_SIZE: usize = SSH_FRAME_MAX_PAYLOAD_SIZE as usize - core::mem::size_of::<ssh_command>();
pub const SSH_MSG_LEN_BASE: usize = core::mem::size_of::<ssh_frame>() + 3 * core::mem::size_of::<u16>();
pub const SSH_MSG_LEN_CTRL: usize = SSH_MSG_LEN_BASE;

#[macro_export]
macro_rules! SSH_MESSAGE_LENGTH { ($payload_size:expr) => { $crate::SSH_MSG_LEN_BASE + ($payload_size) }; }
#[macro_export]
macro_rules! SSH_COMMAND_MESSAGE_LENGTH { ($payload_size:expr) => { $crate::SSH_MESSAGE_LENGTH!(core::mem::size_of::<$crate::ssh_command>() + ($payload_size)) }; }
#[macro_export]
macro_rules! SSH_MSGOFFSET_FRAME { ($field:ident) => { core::mem::size_of::<u16>() + core::mem::offset_of!($crate::ssh_frame, $field) }; }
#[macro_export]
macro_rules! SSH_MSGOFFSET_COMMAND { ($field:ident) => { 2 * core::mem::size_of::<u16>() + core::mem::size_of::<$crate::ssh_frame>() + core::mem::offset_of!($crate::ssh_command, $field) }; }

pub const SSH_MSG_SYN: u16 = 0x55aa;

pub unsafe fn ssh_crc(buf: *const u8, len: usize) -> u16 { crc_itu_t(0xffff, buf, len) }

pub const SSH_NUM_EVENTS: u16 = 38;
pub const SSH_NUM_TARGETS: u8 = 2;

pub const fn ssh_rqid_next_valid(rqid: u16) -> u16 { if rqid > 0 { rqid.wrapping_add(1) } else { rqid.wrapping_add(SSH_NUM_EVENTS + 1) } }
pub const fn ssh_rqid_to_event(rqid: u16) -> u16 { rqid.wrapping_sub(1) }
pub const fn ssh_rqid_is_event(rqid: u16) -> bool { ssh_rqid_to_event(rqid) < SSH_NUM_EVENTS }
pub const fn ssh_tc_to_rqid(tc: u8) -> u16 { tc as u16 }
pub const fn ssh_tid_to_index(tid: u8) -> u8 { tid.wrapping_sub(1) }
pub const fn ssh_tid_is_valid(tid: u8) -> bool { ssh_tid_to_index(tid) < SSH_NUM_TARGETS }

#[repr(C)]
pub struct ssam_span { pub ptr: *mut u8, pub len: usize }

#[repr(u8)]
pub enum ssam_ssh_tid { SSAM_SSH_TID_HOST=0x00, SSAM_SSH_TID_SAM=0x01, SSAM_SSH_TID_KIP=0x02, SSAM_SSH_TID_DEBUG=0x03, SSAM_SSH_TID_SURFLINK=0x04 }

#[repr(u8)]
pub enum ssam_ssh_tc {
    SSAM_SSH_TC_SAM=0x01, SSAM_SSH_TC_BAT=0x02, SSAM_SSH_TC_TMP=0x03, SSAM_SSH_TC_PMC=0x04, SSAM_SSH_TC_FAN=0x05, SSAM_SSH_TC_PoM=0x06, SSAM_SSH_TC_DBG=0x07, SSAM_SSH_TC_KBD=0x08, SSAM_SSH_TC_FWU=0x09, SSAM_SSH_TC_UNI=0x0a, SSAM_SSH_TC_LPC=0x0b, SSAM_SSH_TC_TCL=0x0c, SSAM_SSH_TC_SFL=0x0d, SSAM_SSH_TC_KIP=0x0e, SSAM_SSH_TC_EXT=0x0f, SSAM_SSH_TC_BLD=0x10, SSAM_SSH_TC_BAS=0x11, SSAM_SSH_TC_SEN=0x12, SSAM_SSH_TC_SRQ=0x13, SSAM_SSH_TC_MCU=0x14, SSAM_SSH_TC_HID=0x15, SSAM_SSH_TC_TCH=0x16, SSAM_SSH_TC_BKL=0x17, SSAM_SSH_TC_TAM=0x18, SSAM_SSH_TC_ACC0=0x19, SSAM_SSH_TC_UFI=0x1a, SSAM_SSH_TC_USC=0x1b, SSAM_SSH_TC_PEN=0x1c, SSAM_SSH_TC_VID=0x1d, SSAM_SSH_TC_AUD=0x1e, SSAM_SSH_TC_SMC=0x1f, SSAM_SSH_TC_KPD=0x20, SSAM_SSH_TC_REG=0x21, SSAM_SSH_TC_SPT=0x22, SSAM_SSH_TC_SYS=0x23, SSAM_SSH_TC_ACC1=0x24, SSAM_SSH_TC_SHB=0x25, SSAM_SSH_TC_POS=0x26,
}

#[repr(u8)]
pub enum ssh_packet_base_priority { SSH_PACKET_PRIORITY_FLUSH=0, SSH_PACKET_PRIORITY_DATA=0, SSH_PACKET_PRIORITY_NAK=1, SSH_PACKET_PRIORITY_ACK=2 }
pub const fn __ssh_packet_priority(base: u8, try_: u8) -> u8 { (base << 4) | (try_ & 0x0f) }
pub const fn ssh_packet_priority_get_try(priority: u8) -> u8 { priority & 0x0f }
pub const fn ssh_packet_priority_get_base(priority: u8) -> u8 { (priority & 0xf0) >> 4 }

#[repr(u32)]
pub enum ssh_packet_flags {
    SSH_PACKET_SF_LOCKED_BIT, SSH_PACKET_SF_QUEUED_BIT, SSH_PACKET_SF_PENDING_BIT, SSH_PACKET_SF_TRANSMITTING_BIT, SSH_PACKET_SF_TRANSMITTED_BIT, SSH_PACKET_SF_ACKED_BIT, SSH_PACKET_SF_CANCELED_BIT, SSH_PACKET_SF_COMPLETED_BIT,
    SSH_PACKET_TY_FLUSH_BIT, SSH_PACKET_TY_SEQUENCED_BIT, SSH_PACKET_TY_BLOCKING_BIT,
    SSH_PACKET_FLAGS_SF_MASK = (1<<0)|(1<<1)|(1<<2)|(1<<3)|(1<<4)|(1<<5)|(1<<6)|(1<<7),
    SSH_PACKET_FLAGS_TY_MASK = (1<<8)|(1<<9)|(1<<10),
}

#[repr(C)] pub struct ssh_ptl;
#[repr(C)] pub struct ssh_packet;
#[repr(C)] pub struct ssh_packet_ops { pub release: Option<unsafe extern "C" fn(*mut ssh_packet)>, pub complete: Option<unsafe extern "C" fn(*mut ssh_packet, i32)> }
#[repr(C)] pub struct ssh_packet { pub ptl: *mut ssh_ptl, pub refcnt: kref, pub priority: u8, pub data: ssam_span, pub state: c_ulong, pub timestamp: ktime_t, pub queue_node: list_head, pub pending_node: list_head, pub ops: *const ssh_packet_ops }
extern "C" { pub fn ssh_packet_get(p: *mut ssh_packet) -> *mut ssh_packet; pub fn ssh_packet_put(p: *mut ssh_packet); }
pub unsafe fn ssh_packet_set_data(p: *mut ssh_packet, ptr: *mut u8, len: usize) { (*p).data.ptr=ptr; (*p).data.len=len; }

#[repr(u32)] pub enum ssh_request_flags { SSH_REQUEST_SF_LOCKED_BIT, SSH_REQUEST_SF_QUEUED_BIT, SSH_REQUEST_SF_PENDING_BIT, SSH_REQUEST_SF_TRANSMITTING_BIT, SSH_REQUEST_SF_TRANSMITTED_BIT, SSH_REQUEST_SF_RSPRCVD_BIT, SSH_REQUEST_SF_CANCELED_BIT, SSH_REQUEST_SF_COMPLETED_BIT, SSH_REQUEST_TY_FLUSH_BIT, SSH_REQUEST_TY_HAS_RESPONSE_BIT, SSH_REQUEST_FLAGS_SF_MASK=(1<<0)|(1<<1)|(1<<2)|(1<<3)|(1<<4)|(1<<5)|(1<<6)|(1<<7), SSH_REQUEST_FLAGS_TY_MASK=(1<<8)|(1<<9) }
#[repr(C)] pub struct ssh_rtl;
#[repr(C)] pub struct ssh_request;
#[repr(C)] pub struct ssh_request_ops { pub release: Option<unsafe extern "C" fn(*mut ssh_request)>, pub complete: Option<unsafe extern "C" fn(*mut ssh_request, *const ssh_command, *const ssam_span, i32)> }
#[repr(C)] pub struct ssh_request { pub packet: ssh_packet, pub node: list_head, pub state: c_ulong, pub timestamp: ktime_t, pub ops: *const ssh_request_ops }
pub unsafe fn to_ssh_request(p: *mut ssh_packet) -> *mut ssh_request { (p as *mut u8).sub(core::mem::offset_of!(ssh_request, packet)) as *mut ssh_request }
pub unsafe fn ssh_request_get(r: *mut ssh_request) -> *mut ssh_request { if r.is_null() { core::ptr::null_mut() } else { to_ssh_request(ssh_packet_get(&mut (*r).packet)) } }
pub unsafe fn ssh_request_put(r: *mut ssh_request) { if !r.is_null() { ssh_packet_put(&mut (*r).packet); } }
pub unsafe fn ssh_request_set_data(r: *mut ssh_request, ptr: *mut u8, len: usize) { ssh_packet_set_data(&mut (*r).packet, ptr, len); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
