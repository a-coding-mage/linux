/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const MRP_END_MARK: u8 = 0x0;

#[repr(C)]
pub struct mrp_pdu_hdr {
    pub version: u8,
}

#[repr(C)]
pub struct mrp_msg_hdr {
    pub attrtype: u8,
    pub attrlen: u8,
}

#[repr(C)]
pub struct mrp_vecattr_hdr {
    pub lenflags: __be16,
    pub firstattrvalue: [core::ffi::c_uchar; 0],
}

pub const MRP_VECATTR_HDR_LEN_MASK: __be16 = cpu_to_be16(0x1FFF);
pub const MRP_VECATTR_HDR_FLAG_LA: __be16 = cpu_to_be16(0x2000);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_vecattr_event {
    MRP_VECATTR_EVENT_NEW,
    MRP_VECATTR_EVENT_JOIN_IN,
    MRP_VECATTR_EVENT_IN,
    MRP_VECATTR_EVENT_JOIN_MT,
    MRP_VECATTR_EVENT_MT,
    MRP_VECATTR_EVENT_LV,
    __MRP_VECATTR_EVENT_MAX,
}

#[repr(C)]
pub struct mrp_skb_cb {
    pub mh: *mut mrp_msg_hdr,
    pub vah: *mut mrp_vecattr_hdr,
    pub attrvalue: [core::ffi::c_uchar; 0],
}

#[inline]
pub unsafe fn mrp_cb(skb: *mut sk_buff) -> *mut mrp_skb_cb {
    // BUILD_BUG_ON(sizeof(struct mrp_skb_cb) > sizeof_field(struct sk_buff, cb));
    (*skb).cb.as_mut_ptr() as *mut mrp_skb_cb
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_applicant_state {
    MRP_APPLICANT_INVALID,
    MRP_APPLICANT_VO,
    MRP_APPLICANT_VP,
    MRP_APPLICANT_VN,
    MRP_APPLICANT_AN,
    MRP_APPLICANT_AA,
    MRP_APPLICANT_QA,
    MRP_APPLICANT_LA,
    MRP_APPLICANT_AO,
    MRP_APPLICANT_QO,
    MRP_APPLICANT_AP,
    MRP_APPLICANT_QP,
    __MRP_APPLICANT_MAX,
}

pub const MRP_APPLICANT_MAX: mrp_applicant_state =
    __MRP_APPLICANT_MAX - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_event {
    MRP_EVENT_NEW,
    MRP_EVENT_JOIN,
    MRP_EVENT_LV,
    MRP_EVENT_TX,
    MRP_EVENT_R_NEW,
    MRP_EVENT_R_JOIN_IN,
    MRP_EVENT_R_IN,
    MRP_EVENT_R_JOIN_MT,
    MRP_EVENT_R_MT,
    MRP_EVENT_R_LV,
    MRP_EVENT_R_LA,
    MRP_EVENT_REDECLARE,
    MRP_EVENT_PERIODIC,
    __MRP_EVENT_MAX,
}

pub const MRP_EVENT_MAX: mrp_event = __MRP_EVENT_MAX - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_tx_action {
    MRP_TX_ACTION_NONE,
    MRP_TX_ACTION_S_NEW,
    MRP_TX_ACTION_S_JOIN_IN,
    MRP_TX_ACTION_S_JOIN_IN_OPTIONAL,
    MRP_TX_ACTION_S_IN_OPTIONAL,
    MRP_TX_ACTION_S_LV,
}

#[repr(C)]
pub struct mrp_attr {
    pub node: rb_node,
    pub state: mrp_applicant_state,
    pub type_: u8,
    pub len: u8,
    pub value: [core::ffi::c_uchar; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_applications {
    MRP_APPLICATION_MVRP,
    __MRP_APPLICATION_MAX,
}

pub const MRP_APPLICATION_MAX: mrp_applications = __MRP_APPLICATION_MAX - 1;

#[repr(C)]
pub struct mrp_application {
    pub type_: mrp_applications,
    pub maxattr: core::ffi::c_uint,
    pub pkttype: packet_type,
    pub group_address: [u8; ETH_ALEN],
    pub version: u8,
}

#[repr(C)]
pub struct mrp_applicant {
    pub app: *mut mrp_application,
    pub dev: *mut net_device,
    pub join_timer: timer_list,
    pub periodic_timer: timer_list,
    pub lock: spinlock_t,
    pub queue: sk_buff_head,
    pub pdu: *mut sk_buff,
    pub mad: rb_root,
    pub rcu: rcu_head,
    pub active: bool,
}

#[repr(C)]
pub struct mrp_port {
    pub applicants: [*mut mrp_applicant; (MRP_APPLICATION_MAX as usize) + 1],
    pub rcu: rcu_head,
}

unsafe extern "C" {
    pub fn mrp_register_application(app: *mut mrp_application) -> core::ffi::c_int;
    pub fn mrp_unregister_application(app: *mut mrp_application);
    pub fn mrp_init_applicant(dev: *mut net_device, app: *mut mrp_application) -> core::ffi::c_int;
    pub fn mrp_uninit_applicant(dev: *mut net_device, app: *mut mrp_application);
    pub fn mrp_request_join(dev: *const net_device, app: *const mrp_application, value: *const core::ffi::c_void, len: u8, type_: u8) -> core::ffi::c_int;
    pub fn mrp_request_leave(dev: *const net_device, app: *const mrp_application, value: *const core::ffi::c_void, len: u8, type_: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
