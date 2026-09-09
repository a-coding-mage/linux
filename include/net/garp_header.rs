/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced by name here; the C header includes are not executable Rust.

pub const GARP_PROTOCOL_ID: u16 = 0x1;
pub const GARP_END_MARK: u8 = 0x0;

#[repr(C)]
pub struct garp_pdu_hdr {
    pub protocol: u16,
}

#[repr(C)]
pub struct garp_msg_hdr {
    pub attrtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_attr_event {
    GARP_LEAVE_ALL,
    GARP_JOIN_EMPTY,
    GARP_JOIN_IN,
    GARP_LEAVE_EMPTY,
    GARP_LEAVE_IN,
    GARP_EMPTY,
}

#[repr(C)]
pub struct garp_attr_hdr {
    pub len: u8,
    pub event: u8,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct garp_skb_cb {
    pub cur_type: u8,
}

#[inline]
pub unsafe fn garp_cb(skb: *mut sk_buff) -> *mut garp_skb_cb {
    // BUILD_BUG_ON(sizeof(struct garp_skb_cb) > sizeof_field(struct sk_buff, cb));
    (*skb).cb.as_mut_ptr() as *mut garp_skb_cb
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_applicant_state {
    GARP_APPLICANT_INVALID,
    GARP_APPLICANT_VA,
    GARP_APPLICANT_AA,
    GARP_APPLICANT_QA,
    GARP_APPLICANT_LA,
    GARP_APPLICANT_VP,
    GARP_APPLICANT_AP,
    GARP_APPLICANT_QP,
    GARP_APPLICANT_VO,
    GARP_APPLICANT_AO,
    GARP_APPLICANT_QO,
    __GARP_APPLICANT_MAX,
}

pub const GARP_APPLICANT_MAX: u32 = __GARP_APPLICANT_MAX as u32 - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_event {
    GARP_EVENT_REQ_JOIN,
    GARP_EVENT_REQ_LEAVE,
    GARP_EVENT_R_JOIN_IN,
    GARP_EVENT_R_JOIN_EMPTY,
    GARP_EVENT_R_EMPTY,
    GARP_EVENT_R_LEAVE_IN,
    GARP_EVENT_R_LEAVE_EMPTY,
    GARP_EVENT_TRANSMIT_PDU,
    __GARP_EVENT_MAX,
}

pub const GARP_EVENT_MAX: u32 = __GARP_EVENT_MAX as u32 - 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_action {
    GARP_ACTION_NONE,
    GARP_ACTION_S_JOIN_IN,
    GARP_ACTION_S_LEAVE_EMPTY,
}

#[repr(C)]
pub struct garp_attr {
    pub node: rb_node,
    pub state: garp_applicant_state,
    pub type_: u8,
    pub dlen: u8,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_applications {
    GARP_APPLICATION_GVRP,
    __GARP_APPLICATION_MAX,
}

pub const GARP_APPLICATION_MAX: u32 = __GARP_APPLICATION_MAX as u32 - 1;

#[repr(C)]
pub struct garp_application {
    pub type_: garp_applications,
    pub maxattr: u32,
    pub proto: stp_proto,
}

#[repr(C)]
pub struct garp_applicant {
    pub app: *mut garp_application,
    pub dev: *mut net_device,
    pub join_timer: timer_list,
    pub lock: spinlock_t,
    pub queue: sk_buff_head,
    pub pdu: *mut sk_buff,
    pub gid: rb_root,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct garp_port {
    pub applicants: [*mut garp_applicant; (GARP_APPLICATION_MAX + 1) as usize],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn garp_register_application(app: *mut garp_application) -> i32;
    pub fn garp_unregister_application(app: *mut garp_application);

    pub fn garp_init_applicant(dev: *mut net_device, app: *mut garp_application) -> i32;
    pub fn garp_uninit_applicant(dev: *mut net_device, app: *mut garp_application);

    pub fn garp_request_join(
        dev: *const net_device,
        app: *const garp_application,
        data: *const core::ffi::c_void,
        len: u8,
        type_: u8,
    ) -> i32;
    pub fn garp_request_leave(
        dev: *const net_device,
        app: *const garp_application,
        data: *const core::ffi::c_void,
        len: u8,
        type_: u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
