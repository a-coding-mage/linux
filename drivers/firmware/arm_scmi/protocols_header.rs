/* SPDX-License-Identifier: GPL-2.0 */
/* System Control and Management Interface (SCMI) common protocol header. */

pub const PROTOCOL_REV_MINOR_MASK: u32 = 0x0000_ffff;
pub const PROTOCOL_REV_MAJOR_MASK: u32 = 0xffff_0000;
#[inline]
pub const fn protocol_rev_major(x: u32) -> u16 {
    ((x & PROTOCOL_REV_MAJOR_MASK) >> 16) as u16
}
#[inline]
pub const fn protocol_rev_minor(x: u32) -> u16 {
    (x & PROTOCOL_REV_MINOR_MASK) as u16
}

pub const SCMI_PROTOCOL_VENDOR_BASE: u32 = 0x80;
#[inline]
pub const fn msg_supports_fastchannel(x: u32) -> u32 { x & (1u32 << 0) }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScmiCommonCmd {
    ProtocolVersion = 0x0,
    ProtocolAttributes = 0x1,
    ProtocolMessageAttributes = 0x2,
    NegotiateProtocolVersion = 0x10,
}

#[repr(C)]
pub struct ScmiMsgRespProtVersion {
    pub minor_version: u16,
    pub major_version: u16,
}

#[repr(C)]
pub struct ScmiMsg {
    pub buf: *mut core::ffi::c_void,
    pub len: usize,
}

#[repr(C)]
pub struct ScmiMsgHdr {
    pub id: u8,
    pub protocol_id: u8,
    pub type_: u8,
    pub seq: u16,
    pub status: u32,
    pub poll_completion: bool,
}

#[repr(C)]
pub struct ScmiXfer {
    pub transfer_id: i32,
    pub hdr: ScmiMsgHdr,
    pub tx: ScmiMsg,
    pub rx: ScmiMsg,
    pub done: Completion,
    pub async_done: *mut Completion,
    pub pending: bool,
    pub node: HlistNode,
    pub users: Refcount,
    pub busy: Atomic,
    pub state: i32,
    pub flags: i32,
    pub lock: Spinlock,
    pub priv_: *mut core::ffi::c_void,
}

pub const SCMI_XFER_FREE: i32 = 0;
pub const SCMI_XFER_BUSY: i32 = 1;
pub const SCMI_XFER_SENT_OK: i32 = 0;
pub const SCMI_XFER_RESP_OK: i32 = 1;
pub const SCMI_XFER_DRESP_OK: i32 = 2;
pub const SCMI_XFER_FLAG_IS_RAW: i32 = 1 << 0;
pub const SCMI_XFER_FLAG_CHAN_SET: i32 = 1 << 1;
#[inline]
pub unsafe fn scmi_xfer_is_raw(x: *const ScmiXfer) -> bool { ((*x).flags & SCMI_XFER_FLAG_IS_RAW) != 0 }
#[inline]
pub unsafe fn scmi_xfer_is_chan_set(x: *const ScmiXfer) -> bool { ((*x).flags & SCMI_XFER_FLAG_CHAN_SET) != 0 }

#[repr(C)]
pub struct ScmiProtocolHandle {
    pub dev: *mut Device,
    pub version: u32,
    pub xops: *const ScmiXferOps,
    pub hops: *const ScmiProtoHelpersOps,
    pub set_priv: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut core::ffi::c_void) -> i32>,
    pub get_priv: Option<unsafe extern "C" fn(*const ScmiProtocolHandle) -> *mut core::ffi::c_void>,
}

#[repr(C)]
pub struct ScmiIteratorState {
    pub desc_index: u32,
    pub num_returned: u32,
    pub num_remaining: u32,
    pub max_resources: u32,
    pub loop_idx: u32,
    pub rx_len: usize,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ScmiIteratorOps {
    pub prepare_message: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void)>,
    pub update_state: Option<unsafe extern "C" fn(*mut ScmiIteratorState, *const core::ffi::c_void, *mut core::ffi::c_void) -> i32>,
    pub process_response: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *const core::ffi::c_void, *mut ScmiIteratorState, *mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct ScmiFcDbInfo { pub width: i32, pub set: u64, pub mask: u64, pub addr: *mut core::ffi::c_void }
#[repr(C)]
pub struct ScmiFcInfo { pub set_addr: *mut core::ffi::c_void, pub get_addr: *mut core::ffi::c_void, pub set_db: *mut ScmiFcDbInfo, pub rate_limit: u32 }

#[repr(C)]
pub struct ScmiProtoHelpersOps {
    pub extended_name_get: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, u8, u32, *mut u32, *mut i8, usize) -> i32>,
    pub iter_response_init: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut ScmiIteratorOps, u32, u8, usize, *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub iter_response_run: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub iter_response_run_bound: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> i32>,
    pub iter_response_bound_cleanup: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub protocol_msg_check: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, u32, *mut u32) -> i32>,
    pub fastchannel_init: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, u8, u32, u32, u32, *mut *mut core::ffi::c_void, *mut *mut ScmiFcDbInfo, *mut u32)>,
    pub fastchannel_db_ring: Option<unsafe extern "C" fn(*mut ScmiFcDbInfo)>,
    pub get_max_msg_size: Option<unsafe extern "C" fn(*const ScmiProtocolHandle) -> i32>,
}

#[repr(C)]
pub struct ScmiXferOps {
    pub xfer_get_init: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, u8, usize, usize, *mut *mut ScmiXfer) -> i32>,
    pub reset_rx_to_maxsz: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut ScmiXfer)>,
    pub do_xfer: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut ScmiXfer) -> i32>,
    pub do_xfer_with_response: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut ScmiXfer) -> i32>,
    pub xfer_put: Option<unsafe extern "C" fn(*const ScmiProtocolHandle, *mut ScmiXfer)>,
}

pub type ScmiProtInitPhFnT = unsafe extern "C" fn(*const ScmiProtocolHandle) -> i32;

#[repr(C)]
pub struct ScmiProtocol {
    pub id: u8,
    pub owner: *mut Module,
    pub instance_init: Option<ScmiProtInitPhFnT>,
    pub instance_deinit: Option<ScmiProtInitPhFnT>,
    pub ops: *const core::ffi::c_void,
    pub events: *const ScmiProtocolEvents,
    pub supported_version: u32,
    pub vendor_id: *mut i8,
    pub sub_vendor_id: *mut i8,
    pub impl_ver: u32,
}

/* External kernel types and protocol registration functions are supplied by dependencies. */
extern "C" {
    pub fn scmi_protocol_register(proto: *const ScmiProtocol) -> i32;
    pub fn scmi_protocol_unregister(proto: *const ScmiProtocol);
    pub fn scmi_base_register() -> i32;
    pub fn scmi_base_unregister();
    pub fn scmi_clock_register() -> i32;
    pub fn scmi_clock_unregister();
    pub fn scmi_perf_register() -> i32;
    pub fn scmi_perf_unregister();
    pub fn scmi_pinctrl_register() -> i32;
    pub fn scmi_pinctrl_unregister();
    pub fn scmi_power_register() -> i32;
    pub fn scmi_power_unregister();
    pub fn scmi_reset_register() -> i32;
    pub fn scmi_reset_unregister();
    pub fn scmi_sensors_register() -> i32;
    pub fn scmi_sensors_unregister();
    pub fn scmi_voltage_register() -> i32;
    pub fn scmi_voltage_unregister();
    pub fn scmi_system_register() -> i32;
    pub fn scmi_system_unregister();
    pub fn scmi_powercap_register() -> i32;
    pub fn scmi_powercap_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
