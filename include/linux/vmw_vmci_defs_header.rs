/* SPDX-License-Identifier: GPL-2.0-only */
/* VMware VMCI Driver; translated from vmw_vmci_defs.h. */

// Linux dependencies supplied by the surrounding translation unit: BIT, PAGE_SIZE,
// _IO, and the kernel integer aliases/volatile access conventions.

pub const VMCI_STATUS_ADDR: u32 = 0x00;
pub const VMCI_CONTROL_ADDR: u32 = 0x04;
pub const VMCI_ICR_ADDR: u32 = 0x08;
pub const VMCI_IMR_ADDR: u32 = 0x0c;
pub const VMCI_DATA_OUT_ADDR: u32 = 0x10;
pub const VMCI_DATA_IN_ADDR: u32 = 0x14;
pub const VMCI_CAPS_ADDR: u32 = 0x18;
pub const VMCI_RESULT_LOW_ADDR: u32 = 0x1c;
pub const VMCI_RESULT_HIGH_ADDR: u32 = 0x20;
pub const VMCI_DATA_OUT_LOW_ADDR: u32 = 0x24;
pub const VMCI_DATA_OUT_HIGH_ADDR: u32 = 0x28;
pub const VMCI_DATA_IN_LOW_ADDR: u32 = 0x2c;
pub const VMCI_DATA_IN_HIGH_ADDR: u32 = 0x30;
pub const VMCI_GUEST_PAGE_SHIFT: u32 = 0x34;
pub const VMCI_MAX_DEVICES: usize = 1;

pub const VMCI_STATUS_INT_ON: u32 = 1 << 0;
pub const VMCI_CONTROL_RESET: u32 = 1 << 0;
pub const VMCI_CONTROL_INT_ENABLE: u32 = 1 << 1;
pub const VMCI_CONTROL_INT_DISABLE: u32 = 1 << 2;
pub const VMCI_CAPS_HYPERCALL: u32 = 1 << 0;
pub const VMCI_CAPS_GUESTCALL: u32 = 1 << 1;
pub const VMCI_CAPS_DATAGRAM: u32 = 1 << 2;
pub const VMCI_CAPS_NOTIFICATIONS: u32 = 1 << 3;
pub const VMCI_CAPS_PPN64: u32 = 1 << 4;
pub const VMCI_CAPS_DMA_DATAGRAM: u32 = 1 << 5;
pub const VMCI_ICR_DATAGRAM: u32 = 1 << 0;
pub const VMCI_ICR_NOTIFICATION: u32 = 1 << 1;
pub const VMCI_ICR_DMA_DATAGRAM: u32 = 1 << 2;
pub const VMCI_IMR_DATAGRAM: u32 = 1 << 0;
pub const VMCI_IMR_NOTIFICATION: u32 = 1 << 1;
pub const VMCI_IMR_DMA_DATAGRAM: u32 = 1 << 2;
pub const VMCI_MAX_INTRS_NOTIFICATION: usize = 2;
pub const VMCI_MAX_INTRS_DMA_DATAGRAM: usize = 3;
pub const VMCI_MAX_INTRS: usize = VMCI_MAX_INTRS_DMA_DATAGRAM;

pub const VMCI_INTR_DATAGRAM: i32 = 0;
pub const VMCI_INTR_NOTIFICATION: i32 = 1;
pub const VMCI_INTR_DMA_DATAGRAM: i32 = 2;

pub const VMCI_MAX_GUEST_QP_MEMORY: usize = 128 * 1024 * 1024;
pub const VMCI_MAX_GUEST_QP_COUNT: usize = VMCI_MAX_GUEST_QP_MEMORY / PAGE_SIZE / 2;
pub const VMCI_MAX_GUEST_DOORBELL_COUNT: usize = PAGE_SIZE;
pub const VMCI_MAX_PINNED_QP_MEMORY: usize = 32 * 1024;
pub const VMCI_WITH_MMIO_ACCESS_BAR_SIZE: usize = 256 * 1024;
pub const VMCI_MMIO_ACCESS_OFFSET: usize = 128 * 1024;
pub const VMCI_MMIO_ACCESS_SIZE: usize = 64 * 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_data_in_out_header { pub busy: u32, pub opcode: u32, pub size: u32, pub rsvd: u32, pub result: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_sg_elem { pub addr: u64, pub size: u64 }

pub const VMCI_RESOURCES_QUERY: i32 = 0;
pub const VMCI_GET_CONTEXT_ID: i32 = 1;
pub const VMCI_SET_NOTIFY_BITMAP: i32 = 2;
pub const VMCI_DOORBELL_LINK: i32 = 3;
pub const VMCI_DOORBELL_UNLINK: i32 = 4;
pub const VMCI_DOORBELL_NOTIFY: i32 = 5;
pub const VMCI_DATAGRAM_REQUEST_MAP: i32 = 6;
pub const VMCI_DATAGRAM_REMOVE_MAP: i32 = 7;
pub const VMCI_EVENT_SUBSCRIBE: i32 = 8;
pub const VMCI_EVENT_UNSUBSCRIBE: i32 = 9;
pub const VMCI_QUEUEPAIR_ALLOC: i32 = 10;
pub const VMCI_QUEUEPAIR_DETACH: i32 = 11;
pub const VMCI_HGFS_TRANSPORT: i32 = 13;
pub const VMCI_UNITY_PBRPC_REGISTER: i32 = 14;
pub const VMCI_RPC_PRIVILEGED: i32 = 15;
pub const VMCI_RPC_UNPRIVILEGED: i32 = 16;
pub const VMCI_RESOURCE_MAX: i32 = 17;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct vmci_handle { pub context: u32, pub resource: u32 }
#[inline] pub const fn vmci_make_handle(cid: u32, rid: u32) -> vmci_handle { vmci_handle { context: cid, resource: rid } }
#[inline] pub const fn vmci_handle_is_equal(h1: vmci_handle, h2: vmci_handle) -> bool { h1.context == h2.context && h1.resource == h2.resource }
pub const VMCI_INVALID_ID: u32 = !0;
pub const VMCI_INVALID_HANDLE: vmci_handle = vmci_handle { context: VMCI_INVALID_ID, resource: VMCI_INVALID_ID };
#[inline] pub const fn vmci_handle_is_invalid(h: vmci_handle) -> bool { vmci_handle_is_equal(h, VMCI_INVALID_HANDLE) }
pub const VMCI_ANON_SRC_CONTEXT_ID: u32 = VMCI_INVALID_ID;
pub const VMCI_ANON_SRC_RESOURCE_ID: u32 = VMCI_INVALID_ID;
pub const VMCI_ANON_SRC_HANDLE: vmci_handle = vmci_handle { context: VMCI_ANON_SRC_CONTEXT_ID, resource: VMCI_ANON_SRC_RESOURCE_ID };
pub const VMCI_RESERVED_CID_LIMIT: u32 = 16;
pub const VMCI_HYPERVISOR_CONTEXT_ID: u32 = 0;
pub const VMCI_WELL_KNOWN_CONTEXT_ID: u32 = 1;
pub const VMCI_HOST_CONTEXT_ID: u32 = 2;
#[inline] pub const fn VMCI_CONTEXT_IS_VM(cid: u32) -> bool { VMCI_INVALID_ID != cid && cid > VMCI_HOST_CONTEXT_ID }
pub const VMCI_CONTEXT_RESOURCE_ID: u32 = 0;

pub const VMCI_SUCCESS_QUEUEPAIR_ATTACH: i32 = 5; pub const VMCI_SUCCESS_QUEUEPAIR_CREATE: i32 = 4;
pub const VMCI_SUCCESS_LAST_DETACH: i32 = 3; pub const VMCI_SUCCESS_ACCESS_GRANTED: i32 = 2;
pub const VMCI_SUCCESS_ENTRY_DEAD: i32 = 1; pub const VMCI_SUCCESS: i32 = 0;
pub const VMCI_ERROR_INVALID_RESOURCE: i32 = -1; pub const VMCI_ERROR_INVALID_ARGS: i32 = -2;
pub const VMCI_ERROR_NO_MEM: i32 = -3; pub const VMCI_ERROR_DATAGRAM_FAILED: i32 = -4;
pub const VMCI_ERROR_MORE_DATA: i32 = -5; pub const VMCI_ERROR_NO_MORE_DATAGRAMS: i32 = -6;
pub const VMCI_ERROR_NO_ACCESS: i32 = -7; pub const VMCI_ERROR_NO_HANDLE: i32 = -8;
pub const VMCI_ERROR_DUPLICATE_ENTRY: i32 = -9; pub const VMCI_ERROR_DST_UNREACHABLE: i32 = -10;
pub const VMCI_ERROR_PAYLOAD_TOO_LARGE: i32 = -11; pub const VMCI_ERROR_INVALID_PRIV: i32 = -12;
pub const VMCI_ERROR_GENERIC: i32 = -13; pub const VMCI_ERROR_PAGE_ALREADY_SHARED: i32 = -14;
pub const VMCI_ERROR_CANNOT_SHARE_PAGE: i32 = -15; pub const VMCI_ERROR_CANNOT_UNSHARE_PAGE: i32 = -16;
pub const VMCI_ERROR_NO_PROCESS: i32 = -17; pub const VMCI_ERROR_NO_DATAGRAM: i32 = -18;
pub const VMCI_ERROR_NO_RESOURCES: i32 = -19; pub const VMCI_ERROR_UNAVAILABLE: i32 = -20;
pub const VMCI_ERROR_NOT_FOUND: i32 = -21; pub const VMCI_ERROR_ALREADY_EXISTS: i32 = -22;
pub const VMCI_ERROR_NOT_PAGE_ALIGNED: i32 = -23; pub const VMCI_ERROR_INVALID_SIZE: i32 = -24;
pub const VMCI_ERROR_REGION_ALREADY_SHARED: i32 = -25; pub const VMCI_ERROR_TIMEOUT: i32 = -26;
pub const VMCI_ERROR_DATAGRAM_INCOMPLETE: i32 = -27; pub const VMCI_ERROR_INCORRECT_IRQL: i32 = -28;
pub const VMCI_ERROR_EVENT_UNKNOWN: i32 = -29; pub const VMCI_ERROR_OBSOLETE: i32 = -30;
pub const VMCI_ERROR_QUEUEPAIR_MISMATCH: i32 = -31; pub const VMCI_ERROR_QUEUEPAIR_NOTSET: i32 = -32;
pub const VMCI_ERROR_QUEUEPAIR_NOTOWNER: i32 = -33; pub const VMCI_ERROR_QUEUEPAIR_NOTATTACHED: i32 = -34;
pub const VMCI_ERROR_QUEUEPAIR_NOSPACE: i32 = -35; pub const VMCI_ERROR_QUEUEPAIR_NODATA: i32 = -36;
pub const VMCI_ERROR_BUSMEM_INVALIDATION: i32 = -37; pub const VMCI_ERROR_MODULE_NOT_LOADED: i32 = -38;
pub const VMCI_ERROR_DEVICE_NOT_FOUND: i32 = -39; pub const VMCI_ERROR_QUEUEPAIR_NOT_READY: i32 = -40;
pub const VMCI_ERROR_WOULD_BLOCK: i32 = -41; pub const VMCI_ERROR_CLIENT_MIN: i32 = -500;
pub const VMCI_ERROR_CLIENT_MAX: i32 = -550; pub const VMCI_SHAREDMEM_ERROR_BAD_CONTEXT: i32 = -1000;

pub const VMCI_EVENT_CTX_ID_UPDATE: i32 = 0; pub const VMCI_EVENT_CTX_REMOVED: i32 = 1;
pub const VMCI_EVENT_QP_RESUMED: i32 = 2; pub const VMCI_EVENT_QP_PEER_ATTACH: i32 = 3;
pub const VMCI_EVENT_QP_PEER_DETACH: i32 = 4; pub const VMCI_EVENT_MEM_ACCESS_ON: i32 = 5;
pub const VMCI_EVENT_MEM_ACCESS_OFF: i32 = 6; pub const VMCI_EVENT_MAX: i32 = 7;
#[inline] pub const fn VMCI_EVENT_VALID_VMX(e: i32) -> bool { e == VMCI_EVENT_MEM_ACCESS_ON || e == VMCI_EVENT_MEM_ACCESS_OFF }
#[inline] pub const fn VMCI_EVENT_VALID(e: i32) -> bool { e < VMCI_EVENT_MAX && !VMCI_EVENT_VALID_VMX(e) }
pub const VMCI_EVENT_HANDLER: u32 = 0;
pub const VMCI_NO_PRIVILEGE_FLAGS: u32 = 0; pub const VMCI_PRIVILEGE_FLAG_RESTRICTED: u32 = 1;
pub const VMCI_PRIVILEGE_FLAG_TRUSTED: u32 = 2; pub const VMCI_PRIVILEGE_ALL_FLAGS: u32 = 3;
pub const VMCI_DEFAULT_PROC_PRIVILEGE_FLAGS: u32 = 0; pub const VMCI_LEAST_PRIVILEGE_FLAGS: u32 = 1;
pub const VMCI_MAX_PRIVILEGE_FLAGS: u32 = 2; pub const VMCI_RESERVED_RESOURCE_ID_MAX: u32 = 1023;

pub const VMCI_VERSION_SHIFT_WIDTH: u32 = 16;
#[inline] pub const fn VMCI_MAKE_VERSION(major: u32, minor: u16) -> u32 { (major << 16) | minor as u32 }
#[inline] pub const fn VMCI_VERSION_MAJOR(v: u32) -> u32 { v >> 16 }
#[inline] pub const fn VMCI_VERSION_MINOR(v: u32) -> u16 { v as u16 }
pub const VMCI_VERSION_NOVMVM: u32 = VMCI_MAKE_VERSION(11, 0);
pub const VMCI_VERSION_NOTIFY: u32 = VMCI_MAKE_VERSION(10, 0);
pub const VMCI_VERSION_HOSTQP: u32 = VMCI_MAKE_VERSION(9, 0);
pub const VMCI_VERSION_PREHOSTQP: u32 = VMCI_MAKE_VERSION(8, 0);
pub const VMCI_VERSION_PREVERS2: u32 = VMCI_MAKE_VERSION(1, 0);
pub const VMCI_VERSION: u32 = VMCI_VERSION_NOVMVM;
#[inline] pub fn VMCI_SOCKETS_MAKE_VERSION(p: &[u32; 3]) -> u32 { ((p[0] & 0xff) << 24) | ((p[1] & 0xff) << 16) | p[2] }
// IOCTL values retain the Linux _IO(7, number) dependency from the source header.
pub const IOCTL_VMCI_VERSION: u32 = _IO(7, 0x9f);
pub const IOCTL_VMCI_INIT_CONTEXT: u32 = _IO(7, 0xa0);
pub const IOCTL_VMCI_QUEUEPAIR_SETVA: u32 = _IO(7, 0xa4);
pub const IOCTL_VMCI_NOTIFY_RESOURCE: u32 = _IO(7, 0xa5);
pub const IOCTL_VMCI_NOTIFICATIONS_RECEIVE: u32 = _IO(7, 0xa6);
pub const IOCTL_VMCI_VERSION2: u32 = _IO(7, 0xa7);
pub const IOCTL_VMCI_QUEUEPAIR_ALLOC: u32 = _IO(7, 0xa8);
pub const IOCTL_VMCI_QUEUEPAIR_SETPAGEFILE: u32 = _IO(7, 0xa9);
pub const IOCTL_VMCI_QUEUEPAIR_DETACH: u32 = _IO(7, 0xaa);
pub const IOCTL_VMCI_DATAGRAM_SEND: u32 = _IO(7, 0xab);
pub const IOCTL_VMCI_DATAGRAM_RECEIVE: u32 = _IO(7, 0xac);
pub const IOCTL_VMCI_CTX_ADD_NOTIFICATION: u32 = _IO(7, 0xaf);
pub const IOCTL_VMCI_CTX_REMOVE_NOTIFICATION: u32 = _IO(7, 0xb0);
pub const IOCTL_VMCI_CTX_GET_CPT_STATE: u32 = _IO(7, 0xb1);
pub const IOCTL_VMCI_CTX_SET_CPT_STATE: u32 = _IO(7, 0xb2);
pub const IOCTL_VMCI_GET_CONTEXT_ID: u32 = _IO(7, 0xb3);
pub const IOCTL_VMCI_SET_NOTIFY: u32 = _IO(7, 0xcb);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_queue_header { pub handle: vmci_handle, pub producer_tail: u64, pub consumer_head: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_datagram { pub dst: vmci_handle, pub src: vmci_handle, pub payload_size: u64 }
pub const VMCI_FLAG_DG_NONE: u32 = 0; pub const VMCI_FLAG_WELLKNOWN_DG_HND: u32 = 1;
pub const VMCI_FLAG_ANYCID_DG_HND: u32 = 2; pub const VMCI_FLAG_DG_DELAYED_CB: u32 = 4;
pub const VMCI_MAX_DG_SIZE: usize = 17 * 4096;
pub const VMCI_MAX_DG_PAYLOAD_SIZE: usize = VMCI_MAX_DG_SIZE - core::mem::size_of::<vmci_datagram>();
pub const VMCI_DG_HEADERSIZE: usize = core::mem::size_of::<vmci_datagram>();
pub const VMCI_MAX_DATAGRAM_QUEUE_SIZE: usize = VMCI_MAX_DG_SIZE * 2;
#[inline] pub unsafe fn VMCI_DG_PAYLOAD(dg: *mut vmci_datagram) -> *mut core::ffi::c_void { (dg as *mut u8).add(core::mem::size_of::<vmci_datagram>()) as *mut _ }
#[inline] pub unsafe fn VMCI_DG_SIZE(dg: *const vmci_datagram) -> usize { VMCI_DG_HEADERSIZE + (*dg).payload_size as usize }
#[inline] pub unsafe fn VMCI_DG_SIZE_ALIGNED(dg: *const vmci_datagram) -> usize { (VMCI_DG_SIZE(dg) + 7) & !7 }

#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_payload_qp { pub handle: vmci_handle, pub peer_id: u32, pub _pad: u32 }
pub const VMCI_QPFLAG_ATTACH_ONLY: u32 = 1; pub const VMCI_QPFLAG_LOCAL: u32 = 2;
pub const VMCI_QPFLAG_NONBLOCK: u32 = 4; pub const VMCI_QPFLAG_PINNED: u32 = 8;
pub const VMCI_QP_ALL_FLAGS: u32 = 15; pub const VMCI_QP_ASYMM: u32 = 12; pub const VMCI_QP_ASYMM_PEER: u32 = 13;
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_resource_query_hdr { pub hdr: vmci_datagram, pub num_resources: u32, pub _padding: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_resource_query_msg { pub num_resources: u32, pub _padding: u32, pub resources: [u32; 1] }
pub const VMCI_RESOURCE_QUERY_MAX_NUM: usize = 31;
#[repr(C)] #[derive(Copy, Clone)] pub union vmci_notify_bm_set_union { pub bitmap_ppn32: u32, pub bitmap_ppn64: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_notify_bm_set_msg { pub hdr: vmci_datagram, pub bitmap: vmci_notify_bm_set_union }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_doorbell_link_msg { pub hdr: vmci_datagram, pub handle: vmci_handle, pub notify_idx: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_doorbell_unlink_msg { pub hdr: vmci_datagram, pub handle: vmci_handle }
pub type vmci_doorbell_notify_msg = vmci_doorbell_unlink_msg;
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_data { pub event: u32, pub _pad: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_payld_ctx { pub context_id: u32, pub _pad: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_payld_qp { pub handle: vmci_handle, pub peer_id: u32, pub _pad: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub union vmci_event_payload_union { pub context_payload: vmci_event_payld_ctx, pub qp_payload: vmci_event_payld_qp }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_data_max { pub event_data: vmci_event_data, pub ev_data_payload: vmci_event_payload_union }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_msg { pub hdr: vmci_datagram, pub event_data: vmci_event_data }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_ctx { pub msg: vmci_event_msg, pub payload: vmci_event_payld_ctx }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_event_qp { pub msg: vmci_event_msg, pub payload: vmci_event_payld_qp }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_qp_alloc_msg { pub hdr: vmci_datagram, pub handle: vmci_handle, pub peer: u32, pub flags: u32, pub produce_size: u64, pub consume_size: u64, pub num_ppns: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct vmci_qp_detach_msg { pub hdr: vmci_datagram, pub handle: vmci_handle }
pub const VMCI_FLAG_DELAYED_CB: u32 = 1;
pub type vmci_callback = unsafe extern "C" fn(*mut core::ffi::c_void);
#[repr(C)] pub struct vmci_qp { _private: [u8; 0] }
pub type vmci_datagram_recv_cb = unsafe extern "C" fn(*mut core::ffi::c_void, *mut vmci_datagram) -> i32;
pub type vmci_event_cb = unsafe extern "C" fn(u32, *const vmci_event_data, *mut core::ffi::c_void);

#[inline] pub unsafe fn vmci_event_data_const_payload(ev_data: *const vmci_event_data) -> *const core::ffi::c_void { (ev_data as *const u8).add(core::mem::size_of::<vmci_event_data>()) as *const _ }
#[inline] pub unsafe fn vmci_event_data_payload(ev_data: *mut vmci_event_data) -> *mut core::ffi::c_void { vmci_event_data_const_payload(ev_data).cast_mut() }
#[inline] pub unsafe fn vmci_q_read_pointer(var: *const u64) -> u64 { core::ptr::read_volatile(var as *const usize) as u64 }
#[inline] pub unsafe fn vmci_q_set_pointer(var: *mut u64, new_val: u64) { core::ptr::write_volatile(var as *mut usize, new_val as usize); }
#[inline] pub unsafe fn vmci_qp_add_pointer(var: *mut u64, add: usize, size: u64) { let mut n = vmci_q_read_pointer(var); if n >= size - add as u64 { n -= size; } n += add as u64; vmci_q_set_pointer(var, n); }
#[inline] pub unsafe fn vmci_q_header_producer_tail(q: *const vmci_queue_header) -> u64 { vmci_q_read_pointer(core::ptr::addr_of!((*q).producer_tail)) }
#[inline] pub unsafe fn vmci_q_header_consumer_head(q: *const vmci_queue_header) -> u64 { vmci_q_read_pointer(core::ptr::addr_of!((*q).consumer_head)) }
#[inline] pub unsafe fn vmci_q_header_add_producer_tail(q: *mut vmci_queue_header, add: usize, size: u64) { vmci_qp_add_pointer(core::ptr::addr_of_mut!((*q).producer_tail), add, size) }
#[inline] pub unsafe fn vmci_q_header_add_consumer_head(q: *mut vmci_queue_header, add: usize, size: u64) { vmci_qp_add_pointer(core::ptr::addr_of_mut!((*q).consumer_head), add, size) }
#[inline] pub unsafe fn vmci_q_header_get_pointers(p: *const vmci_queue_header, c: *const vmci_queue_header, pt: *mut u64, ch: *mut u64) { if !pt.is_null() { *pt = vmci_q_header_producer_tail(p); } if !ch.is_null() { *ch = vmci_q_header_consumer_head(c); } }
#[inline] pub unsafe fn vmci_q_header_init(q: *mut vmci_queue_header, h: vmci_handle) { (*q).handle = h; (*q).producer_tail = 0; (*q).consumer_head = 0; }
pub unsafe fn vmci_q_header_free_space(p: *const vmci_queue_header, c: *const vmci_queue_header, size: u64) -> i64 { let tail = vmci_q_header_producer_tail(p); let head = vmci_q_header_consumer_head(c); if tail >= size || head >= size { return VMCI_ERROR_INVALID_SIZE as i64; } if tail >= head { (size - (tail - head) - 1) as i64 } else { (head - tail - 1) as i64 } }
#[inline] pub unsafe fn vmci_q_header_buf_ready(c: *const vmci_queue_header, p: *const vmci_queue_header, size: u64) -> i64 { let free = vmci_q_header_free_space(c, p, size); if free < VMCI_SUCCESS as i64 { free } else { size as i64 - free - 1 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
