/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of common.h; external kernel types and functions are supplied by dependencies. */

pub const SCMI_MAX_CHANNELS: usize = 256;
pub const SCMI_MAX_RESPONSE_TIMEOUT: u32 = 2 * MSEC_PER_SEC;
pub const SCMI_SHMEM_MAX_PAYLOAD_SIZE: usize = 104;
pub const SCMI_TRANSPORT_DEVNAME_PREFIX: &str = "__scmi_transport_device";

#[repr(i32)]
pub enum scmi_error_codes {
    SCMI_SUCCESS = 0,
    SCMI_ERR_SUPPORT = -1,
    SCMI_ERR_PARAMS = -2,
    SCMI_ERR_ACCESS = -3,
    SCMI_ERR_ENTRY = -4,
    SCMI_ERR_RANGE = -5,
    SCMI_ERR_BUSY = -6,
    SCMI_ERR_COMMS = -7,
    SCMI_ERR_GENERIC = -8,
    SCMI_ERR_HARDWARE = -9,
    SCMI_ERR_PROTOCOL = -10,
}

pub static scmi_linux_errmap: [i32; 11] = [0, -EOPNOTSUPP, -EINVAL, -EACCES, -ENOENT,
    -ERANGE, -EBUSY, -ECOMM, -EIO, -EREMOTEIO, -EPROTO];

pub unsafe fn scmi_to_linux_errno(errno: i32) -> i32 {
    let err_idx = errno.wrapping_neg();
    if err_idx >= 0 && (err_idx as usize) < scmi_linux_errmap.len() {
        scmi_linux_errmap[err_idx as usize]
    } else { -EIO }
}

pub const MSG_ID_MASK: u32 = GENMASK(7, 0);
pub const MSG_TYPE_MASK: u32 = GENMASK(9, 8);
pub const MSG_TYPE_COMMAND: u32 = 0;
pub const MSG_TYPE_DELAYED_RESP: u32 = 2;
pub const MSG_TYPE_NOTIFICATION: u32 = 3;
pub const MSG_PROTOCOL_ID_MASK: u32 = GENMASK(17, 10);
pub const MSG_TOKEN_ID_MASK: u32 = GENMASK(27, 18);
pub const MSG_TOKEN_MAX: u32 = ((MSG_TOKEN_ID_MASK >> 18) + 1);
pub const SCMI_PENDING_XFERS_HT_ORDER_SZ: usize = 9;

#[inline]
pub unsafe fn pack_scmi_header(hdr: *const scmi_msg_hdr) -> u32 {
    FIELD_PREP(MSG_ID_MASK, (*hdr).id) | FIELD_PREP(MSG_TYPE_MASK, (*hdr).type_) |
        FIELD_PREP(MSG_TOKEN_ID_MASK, (*hdr).seq) | FIELD_PREP(MSG_PROTOCOL_ID_MASK, (*hdr).protocol_id)
}

#[inline]
pub unsafe fn unpack_scmi_header(msg_hdr: u32, hdr: *mut scmi_msg_hdr) {
    (*hdr).id = FIELD_GET(MSG_ID_MASK, msg_hdr);
    (*hdr).protocol_id = FIELD_GET(MSG_PROTOCOL_ID_MASK, msg_hdr);
    (*hdr).type_ = FIELD_GET(MSG_TYPE_MASK, msg_hdr);
}

pub const SCMI_BUS_NOTIFY_DEVICE_REQUEST: u32 = 0;
pub const SCMI_BUS_NOTIFY_DEVICE_UNREQUEST: u32 = 1;

extern "C" {
    pub fn scmi_revision_area_get(ph: *const scmi_protocol_handle) -> *mut scmi_base_info;
    pub fn scmi_setup_protocol_implemented(ph: *const scmi_protocol_handle, prot_imp: *mut u8);
    pub static scmi_bus_type: bus_type;
    pub static mut scmi_requested_devices_nh: blocking_notifier_head;
    pub fn scmi_device_create(np: *mut device_node, parent: *mut device, protocol: i32, name: *const c_char) -> *mut scmi_device;
    pub fn scmi_device_destroy(parent: *mut device, protocol: i32, name: *const c_char);
    pub fn scmi_protocol_acquire(handle: *const scmi_handle, protocol_id: u8) -> i32;
    pub fn scmi_protocol_release(handle: *const scmi_handle, protocol_id: u8);
}

#[repr(C)]
pub struct scmi_chan_info {
    pub id: i32, pub dev: *mut device, pub is_p2a: bool, pub rx_timeout_ms: u32,
    pub max_msg_size: u32, pub handle: *mut scmi_handle, pub no_completion_irq: bool,
    pub transport_info: *mut c_void,
}

#[repr(C)]
pub struct scmi_transport_ops {
    pub chan_available: Option<unsafe extern "C" fn(*mut device_node, i32) -> bool>,
    pub chan_setup: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut device, bool) -> i32>,
    pub chan_free: Option<unsafe extern "C" fn(i32, *mut c_void, *mut c_void) -> i32>,
    pub get_max_msg: Option<unsafe extern "C" fn(*mut scmi_chan_info) -> u32>,
    pub send_message: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut scmi_xfer) -> i32>,
    pub mark_txdone: Option<unsafe extern "C" fn(*mut scmi_chan_info, i32, *mut scmi_xfer)>,
    pub fetch_response: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut scmi_xfer)>,
    pub fetch_notification: Option<unsafe extern "C" fn(*mut scmi_chan_info, usize, *mut scmi_xfer)>,
    pub clear_channel: Option<unsafe extern "C" fn(*mut scmi_chan_info)>,
    pub poll_done: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut scmi_xfer) -> bool>,
}

#[repr(C)]
pub struct scmi_desc {
    pub ops: *const scmi_transport_ops, pub max_rx_timeout_ms: i32, pub max_msg: i32,
    pub max_msg_size: i32, pub atomic_threshold: u32, pub no_completion_irq: bool,
    pub force_polling: bool, pub sync_cmds_completed_on_ret: bool, pub atomic_enabled: bool,
}

#[inline] pub unsafe fn is_polling_required(cinfo: *mut scmi_chan_info, desc: *const scmi_desc) -> bool { (*cinfo).no_completion_irq || (*desc).force_polling }
#[inline] pub unsafe fn is_transport_polling_capable(desc: *const scmi_desc) -> bool { (*desc).ops.as_ref().unwrap().poll_done.is_some() || (*desc).sync_cmds_completed_on_ret }
#[inline] pub unsafe fn is_polling_enabled(cinfo: *mut scmi_chan_info, desc: *const scmi_desc) -> bool { is_polling_required(cinfo, desc) && is_transport_polling_capable(desc) }

extern "C" {
    pub fn scmi_xfer_raw_put(handle: *const scmi_handle, xfer: *mut scmi_xfer);
    pub fn scmi_xfer_raw_get(handle: *const scmi_handle) -> *mut scmi_xfer;
    pub fn scmi_xfer_raw_channel_get(handle: *const scmi_handle, protocol_id: u8) -> *mut scmi_chan_info;
    pub fn scmi_xfer_raw_inflight_register(handle: *const scmi_handle, xfer: *mut scmi_xfer) -> i32;
    pub fn scmi_xfer_raw_wait_for_message_response(cinfo: *mut scmi_chan_info, xfer: *mut scmi_xfer, timeout_ms: u32) -> i32;
}

#[repr(i32)] pub enum debug_counters { SENT_OK, SENT_FAIL, SENT_FAIL_POLLING_UNSUPPORTED, SENT_FAIL_CHANNEL_NOT_FOUND, RESPONSE_OK, NOTIFICATION_OK, DELAYED_RESPONSE_OK, XFERS_RESPONSE_TIMEOUT, XFERS_RESPONSE_POLLED_TIMEOUT, RESPONSE_POLLED_OK, ERR_MSG_UNEXPECTED, ERR_MSG_INVALID, ERR_MSG_NOMEM, ERR_PROTOCOL, XFERS_INFLIGHT, SCMI_DEBUG_COUNTERS_LAST }
#[repr(C)] pub struct scmi_debug_info { pub top_dentry: *mut dentry, pub name: *const c_char, pub type_: *const c_char, pub is_atomic: bool, pub counters: [atomic_t; SCMI_DEBUG_COUNTERS_LAST as usize] }
#[inline] pub unsafe fn scmi_inc_count(dbg: *mut scmi_debug_info, stat: i32) { if IS_ENABLED(CONFIG_ARM_SCMI_DEBUG_COUNTERS) && !dbg.is_null() { atomic_inc(&mut (*dbg).counters[stat as usize]); } }
#[inline] pub unsafe fn scmi_dec_count(dbg: *mut scmi_debug_info, stat: i32) { if IS_ENABLED(CONFIG_ARM_SCMI_DEBUG_COUNTERS) && !dbg.is_null() { atomic_dec(&mut (*dbg).counters[stat as usize]); } }

#[repr(i32)] pub enum scmi_bad_msg { MSG_UNEXPECTED = -1, MSG_INVALID = -2, MSG_UNKNOWN = -3, MSG_NOMEM = -4, MSG_MBOX_SPURIOUS = -5 }
pub type shmem_copy_toio_t = unsafe extern "C" fn(*mut c_void, *const c_void, usize);
pub type shmem_copy_fromio_t = unsafe extern "C" fn(*mut c_void, *const c_void, usize);
#[repr(C)] pub struct scmi_shmem_io_ops { pub fromio: shmem_copy_fromio_t, pub toio: shmem_copy_toio_t }
pub struct scmi_shared_mem;
pub struct scmi_msg_payld;
pub const SCMI_MSG_MAX_PROT_OVERHEAD: usize = 2 * core::mem::size_of::<__le32>();

extern "C" {
    pub fn scmi_shared_mem_operations_get() -> *const scmi_shared_mem_operations;
    pub fn scmi_message_operations_get() -> *const scmi_message_operations;
    pub fn scmi_notification_instance_data_set(handle: *const scmi_handle, priv_: *mut c_void);
    pub fn scmi_notification_instance_data_get(handle: *const scmi_handle) -> *mut c_void;
    pub fn scmi_inflight_count(handle: *const scmi_handle) -> i32;
}

#[repr(C)] pub struct scmi_shared_mem_operations { pub tx_prepare: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer, *mut scmi_chan_info, shmem_copy_toio_t)>, pub read_header: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> u32>, pub fetch_response: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer, shmem_copy_fromio_t)>, pub fetch_notification: Option<unsafe extern "C" fn(*mut scmi_shared_mem, usize, *mut scmi_xfer, shmem_copy_fromio_t)>, pub clear_channel: Option<unsafe extern "C" fn(*mut scmi_shared_mem)>, pub poll_done: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer) -> bool>, pub channel_free: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> bool>, pub channel_intr_enabled: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> bool>, pub setup_iomap: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut device, bool, *mut resource, *mut *mut scmi_shmem_io_ops) -> *mut c_void> }
#[repr(C)] pub struct scmi_message_operations { pub response_size: Option<unsafe extern "C" fn(*mut scmi_xfer) -> usize>, pub command_size: Option<unsafe extern "C" fn(*mut scmi_xfer) -> usize>, pub tx_prepare: Option<unsafe extern "C" fn(*mut scmi_msg_payld, *mut scmi_xfer)>, pub read_header: Option<unsafe extern "C" fn(*mut scmi_msg_payld) -> u32>, pub fetch_response: Option<unsafe extern "C" fn(*mut scmi_msg_payld, usize, *mut scmi_xfer)>, pub fetch_notification: Option<unsafe extern "C" fn(*mut scmi_msg_payld, usize, usize, *mut scmi_xfer)> }
#[repr(C)] pub struct scmi_transport_core_operations { pub bad_message_trace: Option<unsafe extern "C" fn(*mut scmi_chan_info, u32, scmi_bad_msg)>, pub rx_callback: Option<unsafe extern "C" fn(*mut scmi_chan_info, u32, *mut c_void)>, pub shmem: *const scmi_shared_mem_operations, pub msg: *const scmi_message_operations }
#[repr(C)] pub struct scmi_transport_handle { pub supplier_get: Option<unsafe extern "C" fn(*const scmi_transport_handle) -> *mut device>, pub supplier_put: Option<unsafe extern "C" fn(*const scmi_transport_handle, *mut device) -> i32> }
#[repr(C)] pub struct scmi_transport { pub supplier: *mut device, pub desc: scmi_desc, pub core_ops: *mut *mut scmi_transport_core_operations, pub th: *const scmi_transport_handle }
#[repr(C)] pub struct scmi_transport_supplier { pub mtx: mutex, pub available: *mut device, pub th: scmi_transport_handle }

#[inline]
pub unsafe fn scmi_transport_supplier_put(th: *const scmi_transport_handle, supplier: *mut device) -> i32 {
    if supplier.is_null() || IS_ERR(supplier) { return 0; }
    let sup = container_of!(th, scmi_transport_supplier, th);
    guard_mutex!(&mut (*sup).mtx);
    match PTR_ERR_OR_ZERO((*sup).available) {
        -EPROBE_DEFER | -EBUSY => { (*sup).available = supplier; 0 }
        0 => { if supplier != (*sup).available { -EINVAL } else { (*sup).available = ERR_PTR(-EPROBE_DEFER); 0 } }
        _ => -EINVAL,
    }
}

#[inline]
pub unsafe fn scmi_transport_supplier_get(th: *const scmi_transport_handle) -> *mut device {
    let sup = container_of!(th, scmi_transport_supplier, th);
    guard_mutex!(&mut (*sup).mtx);
    let supplier = (*sup).available;
    if !IS_ERR(supplier) { (*sup).available = ERR_PTR(-EBUSY); }
    supplier
}

/* Source-level equivalents of the C integration macros. */
#[macro_export] macro_rules! XFER_FIND { ($ht:expr, $k:expr) => {{ let key_ = $k; let mut xfer_: *mut scmi_xfer = core::ptr::null_mut(); hash_for_each_possible!($ht, xfer_, node, key_); xfer_ }} }
#[macro_export] macro_rules! to_sup { ($t:expr) => { container_of!($t, scmi_transport_supplier, th) } }
#[macro_export] macro_rules! DEFINE_SCMI_TRANSPORT_SUPPLIER { ($supplier:ident) => { static mut $supplier: scmi_transport_supplier = scmi_transport_supplier { mtx: __MUTEX_INITIALIZER!(), available: INIT_ERR_PTR(-EPROBE_DEFER), th: scmi_transport_handle { supplier_get: Some(scmi_transport_supplier_get), supplier_put: Some(scmi_transport_supplier_put) } }; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
