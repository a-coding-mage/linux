/* SPDX-License-Identifier: GPL-2.0-only */
// Rust translation of the C Hyper-V header. External kernel types and
// functions referenced below are supplied by other translation units.

pub const MAX_PAGE_BUFFER_COUNT: usize = 32;
pub const MAX_MULTIPAGE_BUFFER_COUNT: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_gpadl_type(pub u32);
pub const HV_GPADL_BUFFER: hv_gpadl_type = hv_gpadl_type(0);
pub const HV_GPADL_RING: hv_gpadl_type = hv_gpadl_type(1);
pub const HV_GPADL_BUFFER_DECRYPTED: hv_gpadl_type = hv_gpadl_type(2);

#[repr(C, packed)] pub struct hv_page_buffer { pub len: u32, pub offset: u32, pub pfn: u64 }
#[repr(C, packed)] pub struct hv_multipage_buffer { pub len: u32, pub offset: u32, pub pfn_array: [u64; MAX_MULTIPAGE_BUFFER_COUNT] }
#[repr(C, packed)] pub struct hv_mpb_array { pub len: u32, pub offset: u32, pub pfn_array: [u64; 0] }

#[repr(C, packed)] pub struct hv_ring_buffer {
    pub write_index: u32, pub read_index: u32, pub interrupt_mask: u32,
    pub pending_send_sz: u32, pub reserved1: [u32; 12], pub feature_bits: u32,
    pub reserved2: [u8; PAGE_SIZE - 68], pub buffer: [u8; 0],
}
#[repr(C)] pub struct hv_ring_buffer_info {
    pub ring_buffer: *mut hv_ring_buffer, pub ring_size: u32,
    pub ring_size_div10_reciprocal: reciprocal_value, pub ring_lock: spinlock_t,
    pub ring_datasize: u32, pub priv_read_index: u32, pub ring_buffer_mutex: mutex,
    pub pkt_buffer: *mut core::ffi::c_void, pub pkt_buffer_size: u32,
}

#[inline] pub unsafe fn hv_get_bytes_to_read(r: *const hv_ring_buffer_info) -> u32 { let d=(*r).ring_datasize; let a=(*(*r).ring_buffer).read_index; let b=core::ptr::read_volatile(&(*(*r).ring_buffer).write_index); if b>=a {b-a} else {(d-a)+b} }
#[inline] pub unsafe fn hv_get_bytes_to_write(r: *const hv_ring_buffer_info) -> u32 { let d=(*r).ring_datasize; let a=core::ptr::read_volatile(&(*(*r).ring_buffer).read_index); let b=(*(*r).ring_buffer).write_index; if b>=a {d-(b-a)} else {a-b} }
#[inline] pub unsafe fn hv_get_avail_to_write_percent(r: *const hv_ring_buffer_info) -> u32 { reciprocal_divide((hv_get_bytes_to_write(r)<<3)+(hv_get_bytes_to_write(r)<<1), (*r).ring_size_div10_reciprocal) }

pub const fn VMBUS_MAKE_VERSION(a:u32,b:u32)->u32 {(a<<16)|b}
pub const VERSION_WS2008:u32=VMBUS_MAKE_VERSION(0,13); pub const VERSION_WIN7:u32=VMBUS_MAKE_VERSION(1,1); pub const VERSION_WIN8:u32=VMBUS_MAKE_VERSION(2,4); pub const VERSION_WIN8_1:u32=VMBUS_MAKE_VERSION(3,0); pub const VERSION_WIN10:u32=VMBUS_MAKE_VERSION(4,0); pub const VERSION_WIN10_V4_1:u32=VMBUS_MAKE_VERSION(4,1); pub const VERSION_WIN10_V5:u32=VMBUS_MAKE_VERSION(5,0); pub const VERSION_WIN10_V5_1:u32=VMBUS_MAKE_VERSION(5,1); pub const VERSION_WIN10_V5_2:u32=VMBUS_MAKE_VERSION(5,2); pub const VERSION_WIN10_V5_3:u32=VMBUS_MAKE_VERSION(5,3); pub const VERSION_WIN10_V6_0:u32=VMBUS_MAKE_VERSION(6,0);
pub const MAX_PIPE_DATA_PAYLOAD:usize=16384; pub const VMBUS_PIPE_TYPE_BYTE:u32=0; pub const VMBUS_PIPE_TYPE_MESSAGE:u32=4; pub const MAX_USER_DEFINED_BYTES:usize=120; pub const MAX_PIPE_USER_DEFINED_BYTES:usize=116;

#[repr(C, packed)] pub struct vmbus_channel_offer { pub if_type: guid_t, pub if_instance: guid_t, pub reserved1:u64,pub reserved2:u64,pub chn_flags:u16,pub mmio_megabytes:u16,pub u: vmbus_channel_offer_u,pub sub_channel_index:u16,pub reserved3:u16 }
#[repr(C)] pub union vmbus_channel_offer_u { pub std:[u8;120], pub pipe:vmbus_pipe_offer }
#[repr(C)] pub struct vmbus_pipe_offer { pub pipe_mode:u32,pub user_def:[u8;116] }
pub const VMBUS_CHANNEL_ENUMERATE_DEVICE_INTERFACE:u16=1; pub const VMBUS_CHANNEL_CONFIDENTIAL_RING_BUFFER:u16=2; pub const VMBUS_CHANNEL_CONFIDENTIAL_EXTERNAL_MEMORY:u16=4; pub const VMBUS_CHANNEL_NAMED_PIPE_MODE:u16=0x10; pub const VMBUS_CHANNEL_LOOPBACK_OFFER:u16=0x100; pub const VMBUS_CHANNEL_PARENT_OFFER:u16=0x200; pub const VMBUS_CHANNEL_REQUEST_MONITORED_NOTIFICATION:u16=0x400; pub const VMBUS_CHANNEL_TLNPI_PROVIDER_OFFER:u16=0x2000;
#[repr(C, packed)] pub struct vmpacket_descriptor { pub r#type:u16,pub offset8_:u16,pub len8:u16,pub flags:u16,pub trans_id:u64 }
#[repr(C, packed)] pub struct vmpacket_header { pub prev_pkt_start_offset:u32,pub descriptor:vmpacket_descriptor }
#[repr(C, packed)] pub struct vmtransfer_page_range { pub byte_count:u32,pub byte_offset:u32 }
#[repr(C, packed)] pub struct vmtransfer_page_packet_header { pub d:vmpacket_descriptor,pub xfer_pageset_id:u16,pub sender_owns_set:u8,pub reserved:u8,pub range_cnt:u32,pub ranges:[vmtransfer_page_range;0] }
#[repr(C)] pub struct gpa_range { pub byte_count:u32,pub byte_offset:u32,pub pfn_array:[u64;0] }
#[repr(C, packed)] pub struct vmdata_gpa_direct { pub d:vmpacket_descriptor,pub reserved:u32,pub range_cnt:u32,pub range:[gpa_range;1] }

#[repr(u16)] pub enum vmbus_packet_type { VM_PKT_INVALID=0,VM_PKT_SYNCH=1,VM_PKT_ADD_XFER_PAGESET=2,VM_PKT_RM_XFER_PAGESET=3,VM_PKT_ESTABLISH_GPADL=4,VM_PKT_TEARDOWN_GPADL=5,VM_PKT_DATA_INBAND=6,VM_PKT_DATA_USING_XFER_PAGES=7,VM_PKT_DATA_USING_GPADL=8,VM_PKT_DATA_USING_GPA_DIRECT=9,VM_PKT_CANCEL_REQUEST=10,VM_PKT_COMP=11,VM_PKT_DATA_USING_ADDITIONAL_PKT=12,VM_PKT_ADDITIONAL_DATA=13 }
pub const VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED:u32=1;

#[repr(C, packed)] pub struct vmbus_channel_message_header { pub msgtype:u32,pub padding:u32 }
#[repr(C, packed)] pub struct vmbus_channel_query_vmbus_version { pub header:vmbus_channel_message_header,pub version:u32 }
#[repr(C, packed)] pub struct vmbus_channel_version_supported { pub header:vmbus_channel_message_header,pub version_supported:u8 }
#[repr(C, packed)] pub struct vmbus_channel_open_channel { pub header:vmbus_channel_message_header,pub child_relid:u32,pub openid:u32,pub ringbuffer_gpadlhandle:u32,pub target_vp:u32,pub downstream_ringbuffer_pageoffset:u32,pub userdata:[u8;120] }
#[repr(C, packed)] pub struct vmbus_channel_open_result { pub header:vmbus_channel_message_header,pub child_relid:u32,pub openid:u32,pub status:u32 }
#[repr(C, packed)] pub struct vmbus_channel_modifychannel_response { pub header:vmbus_channel_message_header,pub child_relid:u32,pub status:u32 }
#[repr(C, packed)] pub struct vmbus_channel_close_channel { pub header:vmbus_channel_message_header,pub child_relid:u32 }
#[repr(C, packed)] pub struct vmbus_channel_gpadl_header { pub header:vmbus_channel_message_header,pub child_relid:u32,pub gpadl:u32,pub range_buflen:u16,pub rangecount:u16,pub range:[gpa_range;0] }
#[repr(C, packed)] pub struct vmbus_channel_gpadl_body { pub header:vmbus_channel_message_header,pub msgnumber:u32,pub gpadl:u32,pub pfn:[u64;0] }
#[repr(C, packed)] pub struct vmbus_channel_gpadl_created { pub header:vmbus_channel_message_header,pub child_relid:u32,pub gpadl:u32,pub creation_status:u32 }
#[repr(C, packed)] pub struct vmbus_channel_gpadl_teardown { pub header:vmbus_channel_message_header,pub child_relid:u32,pub gpadl:u32 }
#[repr(C, packed)] pub struct vmbus_channel_gpadl_torndown { pub header:vmbus_channel_message_header,pub gpadl:u32 }
#[repr(C, packed)] pub struct vmbus_channel_relid_released { pub header:vmbus_channel_message_header,pub child_relid:u32 }

// External kernel declarations and the remaining large channel/device graph.
// Flexible arrays, callback signatures, and opaque kernel objects retain their
// C ABI through these declarations.
#[repr(C)] pub struct vmbus_channel_initiate_contact { pub header:vmbus_channel_message_header,pub vmbus_version_requested:u32,pub target_vcpu:u32,pub interrupt_page:u64,pub monitor_page1:u64,pub monitor_page2:u64 }
#[repr(C, packed)] pub struct vmbus_channel_tl_connect_request { pub header:vmbus_channel_message_header,pub guest_endpoint_id:guid_t,pub host_service_id:guid_t }
#[repr(C, packed)] pub struct vmbus_channel_modifychannel { pub header:vmbus_channel_message_header,pub child_relid:u32,pub target_vp:u32 }
#[repr(C, packed)] pub struct vmbus_channel_version_response { pub header:vmbus_channel_message_header,pub version_supported:u8,pub connection_state:u8,pub padding:u16,pub msg_conn_id:u32 }
#[repr(u32)] pub enum vmbus_channel_state { CHANNEL_OFFER_STATE,CHANNEL_OPENING_STATE,CHANNEL_OPEN_STATE,CHANNEL_OPENED_STATE }
#[repr(C)] pub struct vmbus_gpadl { pub gpadl_handle:u32,pub size:u32,pub buffer:*mut core::ffi::c_void,pub decrypted:bool }
#[repr(C)] pub struct vmbus_requestor { pub req_arr:*mut u64,pub req_bitmap:*mut usize,pub size:u32,pub next_request_id:u64,pub req_lock:spinlock_t }
pub const VMBUS_DEFAULT_MAX_PKT_SIZE:u32=4096; pub const VMBUS_NO_RQSTOR:u64=u64::MAX; pub const VMBUS_RQST_ERROR:u64=u64::MAX-1; pub const VMBUS_RQST_ADDR_ANY:u64=u64::MAX; pub const VMBUS_RQST_ID_NO_RESPONSE:u64=u64::MAX-2; pub const VMBUS_RQST_INIT:u64=u64::MAX-2; pub const VMBUS_RQST_RESET:u64=u64::MAX-3;
#[repr(C)] pub struct vmbus_device { pub pref_ring_size:usize,pub dev_type:u16,pub guid:guid_t,pub perf_device:bool,pub allowed_in_isolated:bool }
#[repr(C)] pub struct vmbus_channel { pub listentry:list_head,pub device_obj:*mut hv_device,pub state:vmbus_channel_state,pub offermsg:vmbus_channel_offer,pub monitor_grp:u8,pub monitor_bit:u8,pub rescind:bool,pub rescind_ref:bool,pub rescind_event:completion,pub ringbuffer_gpadlhandle:vmbus_gpadl }
#[repr(C)] pub struct hv_device { pub dev_type:guid_t,pub dev_instance:guid_t,pub vendor_id:u16,pub device_id:u16 }
#[repr(C, packed)] pub struct vmbuspipe_hdr { pub flags:u32,pub msgsize:u32 }
#[repr(C, packed)] pub struct ic_version { pub major:u16,pub minor:u16 }
#[repr(C, packed)] pub struct icmsg_hdr { pub icverframe:ic_version,pub icmsgtype:u16,pub icvermsg:ic_version,pub icmsgsize:u16,pub status:u32,pub ictransaction_id:u8,pub icflags:u8,pub reserved:[u8;2] }
#[repr(C, packed)] pub struct icmsg_negotiate { pub icframe_vercnt:u16,pub icmsg_vercnt:u16,pub reserved:u32,pub icversion_data:[ic_version;0] }
#[repr(C, packed)] pub struct shutdown_msg_data { pub reason_code:u32,pub timeout_seconds:u32,pub flags:u32,pub display_message:[u8;2048] }
#[repr(C, packed)] pub struct heartbeat_msg_data { pub seq_num:u64,pub reserved:[u32;8] }
#[repr(C, packed)] pub struct ictimesync_data { pub parenttime:u64,pub childtime:u64,pub roundtriptime:u64,pub flags:u8 }
#[repr(C, packed)] pub struct ictimesync_ref_data { pub parenttime:u64,pub vmreferencetime:u64,pub flags:u8,pub leapflags:i8,pub stratum:i8,pub reserved:[u8;3] }
pub const ICMSGTYPE_NEGOTIATE:u32=0; pub const ICMSGTYPE_HEARTBEAT:u32=1; pub const ICMSGTYPE_KVPEXCHANGE:u32=2; pub const ICMSGTYPE_SHUTDOWN:u32=3; pub const ICMSGTYPE_TIMESYNC:u32=4; pub const ICMSGTYPE_VSS:u32=5; pub const ICMSGTYPE_FCOPY:u32=7; pub const ICMSGHDRFLAG_TRANSACTION:u8=1; pub const ICMSGHDRFLAG_REQUEST:u8=2; pub const ICMSGHDRFLAG_RESPONSE:u8=4;

pub const GPADL_TYPE_RING_BUFFER:u32=1; pub const GPADL_TYPE_SERVER_SAVE_AREA:u32=2; pub const GPADL_TYPE_TRANSACTION:u32=8;
pub const VMBUS_FEATURE_FLAG_CONFIDENTIAL_CHANNELS:u32=0x10; pub const INVALID_RELID:u32=u32::MAX;
pub const IC_VERSION_NEGOTIATION_MAX_VER_COUNT:u32=100; pub const ICTIMESYNCFLAG_PROBE:u8=0; pub const ICTIMESYNCFLAG_SYNC:u8=1; pub const ICTIMESYNCFLAG_SAMPLE:u8=2; pub const WLTIMEDELTA:u64=116444736000000000;
pub const HV_CONFIG_BLOCK_SIZE_MAX:u32=128; pub const MAX_SRV_VER:u32=0x7ffffff;
pub const NR_HV_HYP_PAGES_IN_PAGE:usize=PAGE_SIZE/HV_HYP_PAGE_SIZE;

#[repr(C)] pub struct hv_util_service { pub recv_buffer:*mut u8,pub channel:*mut core::ffi::c_void,pub util_cb:Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,pub util_init:Option<unsafe extern "C" fn(*mut hv_util_service)->i32>,pub util_init_transport:Option<unsafe extern "C" fn()->i32>,pub util_deinit:Option<unsafe extern "C" fn()>,pub util_pre_suspend:Option<unsafe extern "C" fn()->i32>,pub util_pre_resume:Option<unsafe extern "C" fn()->i32> }
#[repr(C)] pub struct hv_dma_range { pub dma:dma_addr_t,pub mapping_size:u32 }
#[repr(C)] pub struct hyperv_service_callback { pub msg_type:u8,pub log_msg:*mut i8,pub data:guid_t,pub channel:*mut vmbus_channel,pub callback:Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)] pub struct hyperv_pci_block_ops { pub read_block:*mut core::ffi::c_void,pub write_block:*mut core::ffi::c_void,pub reg_blk_invalidate:*mut core::ffi::c_void }
extern "C" { pub static mut vmbus_proto_version:u32; pub static mut hvpci_block_ops:hyperv_pci_block_ops; }

#[inline] pub unsafe fn is_co_ring_buffer(o:*const vmbus_channel_offer)->bool { ((*o).chn_flags & VMBUS_CHANNEL_CONFIDENTIAL_RING_BUFFER)!=0 }
#[inline] pub unsafe fn is_co_external_memory(o:*const vmbus_channel_offer)->bool { ((*o).chn_flags & VMBUS_CHANNEL_CONFIDENTIAL_EXTERNAL_MEMORY)!=0 }
#[inline] pub unsafe fn is_sub_channel(c:*const vmbus_channel)->bool { (*c).offermsg.sub_channel_index!=0 }
#[inline] pub unsafe fn hv_get_ring_buffer(r:*const hv_ring_buffer_info)->*mut u8 { (*(*r).ring_buffer).buffer.as_mut_ptr() }
#[inline] pub unsafe fn hv_begin_read(r:*mut hv_ring_buffer_info) { (*(*r).ring_buffer).interrupt_mask=1; virt_mb(); }
#[inline] pub unsafe fn hv_end_read(r:*mut hv_ring_buffer_info)->u32 { (*(*r).ring_buffer).interrupt_mask=0; virt_mb(); hv_get_bytes_to_read(r) }

// Function declarations preserve the header's externally visible interfaces.
extern "C" {
 pub fn vmbus_next_request_id(channel:*mut vmbus_channel,rqst_addr:u64)->u64;
 pub fn vmbus_request_addr(channel:*mut vmbus_channel,trans_id:u64)->u64;
 pub fn vmbus_onmessage(hdr:*mut vmbus_channel_message_header);
 pub fn vmbus_request_offers()->i32;
 pub fn vmbus_alloc_ring(channel:*mut vmbus_channel,send_size:u32,recv_size:u32)->i32;
 pub fn vmbus_free_ring(channel:*mut vmbus_channel);
 pub fn vmbus_open(channel:*mut vmbus_channel,send:u32,recv:u32,userdata:*mut core::ffi::c_void,len:u32,cb:*mut core::ffi::c_void,context:*mut core::ffi::c_void)->i32;
 pub fn vmbus_close(channel:*mut vmbus_channel);
 pub fn vmbus_recvpacket(channel:*mut vmbus_channel,buffer:*mut core::ffi::c_void,len:u32,actual:*mut u32,requestid:*mut u64)->i32;
 pub fn vmbus_setevent(channel:*mut vmbus_channel);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
