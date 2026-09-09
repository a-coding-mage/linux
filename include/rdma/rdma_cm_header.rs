/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Translated from rdma_cm.h. External types and functions are supplied elsewhere. */

#[repr(C)]
pub enum rdma_cm_event_type {
    RDMA_CM_EVENT_ADDR_RESOLVED,
    RDMA_CM_EVENT_ADDR_ERROR,
    RDMA_CM_EVENT_ROUTE_RESOLVED,
    RDMA_CM_EVENT_ROUTE_ERROR,
    RDMA_CM_EVENT_CONNECT_REQUEST,
    RDMA_CM_EVENT_CONNECT_RESPONSE,
    RDMA_CM_EVENT_CONNECT_ERROR,
    RDMA_CM_EVENT_UNREACHABLE,
    RDMA_CM_EVENT_REJECTED,
    RDMA_CM_EVENT_ESTABLISHED,
    RDMA_CM_EVENT_DISCONNECTED,
    RDMA_CM_EVENT_DEVICE_REMOVAL,
    RDMA_CM_EVENT_MULTICAST_JOIN,
    RDMA_CM_EVENT_MULTICAST_ERROR,
    RDMA_CM_EVENT_ADDR_CHANGE,
    RDMA_CM_EVENT_TIMEWAIT_EXIT,
    RDMA_CM_EVENT_ADDRINFO_RESOLVED,
    RDMA_CM_EVENT_ADDRINFO_ERROR,
    RDMA_CM_EVENT_USER,
    RDMA_CM_EVENT_INTERNAL,
}

unsafe extern "C" {
    pub fn rdma_event_msg(event: rdma_cm_event_type) -> *const ::std::os::raw::c_char;
}

pub const RDMA_IB_IP_PS_MASK: u64 = 0xFFFFFFFFFFFF0000;
pub const RDMA_IB_IP_PS_TCP: u64 = 0x0000000001060000;
pub const RDMA_IB_IP_PS_UDP: u64 = 0x0000000001110000;
pub const RDMA_IB_IP_PS_IB: u64 = 0x00000000013F0000;

#[repr(C)]
pub struct rdma_addr {
    pub src_addr: sockaddr_storage,
    pub dst_addr: sockaddr_storage,
    pub dev_addr: rdma_dev_addr,
}

#[repr(C)]
pub struct rdma_route {
    pub addr: rdma_addr,
    pub path_rec: *mut sa_path_rec,
    pub path_rec_inbound: *mut sa_path_rec,
    pub path_rec_outbound: *mut sa_path_rec,
    pub num_pri_alt_paths: ::std::os::raw::c_int,
    pub num_service_recs: ::std::os::raw::c_uint,
    pub service_recs: *mut sa_service_rec,
}

#[repr(C)]
pub struct rdma_conn_param {
    pub private_data: *const ::std::ffi::c_void,
    pub private_data_len: u8,
    pub responder_resources: u8,
    pub initiator_depth: u8,
    pub flow_control: u8,
    pub retry_count: u8,
    pub rnr_retry_count: u8,
    pub srq: u8,
    pub qp_num: u32,
    pub qkey: u32,
}

#[repr(C)]
pub struct rdma_ud_param {
    pub private_data: *const ::std::ffi::c_void,
    pub private_data_len: u8,
    pub ah_attr: ib_ah_attr,
    pub qp_num: u32,
    pub qkey: u32,
}

#[repr(C)]
pub union rdma_cm_event_param {
    pub conn: rdma_conn_param,
    pub ud: rdma_ud_param,
    pub arg: u64,
}

#[repr(C)]
pub struct rdma_cm_event {
    pub event: rdma_cm_event_type,
    pub status: ::std::os::raw::c_int,
    pub param: rdma_cm_event_param,
    pub ece: rdma_ucm_ece,
}

#[repr(C)]
pub struct rdma_cm_id;

pub type rdma_cm_event_handler = unsafe extern "C" fn(
    id: *mut rdma_cm_id,
    event: *mut rdma_cm_event,
) -> ::std::os::raw::c_int;

#[repr(C)]
pub struct rdma_cm_id {
    pub device: *mut ib_device,
    pub context: *mut ::std::ffi::c_void,
    pub qp: *mut ib_qp,
    pub event_handler: rdma_cm_event_handler,
    pub route: rdma_route,
    pub ps: rdma_ucm_port_space,
    pub qp_type: ib_qp_type,
    pub port_num: u32,
    pub net_work: work_struct,
}

unsafe extern "C" {
    pub fn __rdma_create_kernel_id(
        net: *mut net,
        event_handler: rdma_cm_event_handler,
        context: *mut ::std::ffi::c_void,
        ps: rdma_ucm_port_space,
        qp_type: ib_qp_type,
        caller: *const ::std::os::raw::c_char,
    ) -> *mut rdma_cm_id;
    pub fn rdma_create_user_id(event_handler: rdma_cm_event_handler, context: *mut ::std::ffi::c_void, ps: rdma_ucm_port_space, qp_type: ib_qp_type) -> *mut rdma_cm_id;
    pub fn rdma_destroy_id(id: *mut rdma_cm_id);
    pub fn rdma_restrict_node_type(id: *mut rdma_cm_id, node_type: u8) -> ::std::os::raw::c_int;
    pub fn rdma_bind_addr(id: *mut rdma_cm_id, addr: *mut sockaddr) -> ::std::os::raw::c_int;
    pub fn rdma_resolve_addr(id: *mut rdma_cm_id, src_addr: *mut sockaddr, dst_addr: *const sockaddr, timeout_ms: ::std::os::raw::c_ulong) -> ::std::os::raw::c_int;
    pub fn rdma_resolve_route(id: *mut rdma_cm_id, timeout_ms: ::std::os::raw::c_ulong) -> ::std::os::raw::c_int;
    pub fn rdma_resolve_ib_service(id: *mut rdma_cm_id, ibs: *mut rdma_ucm_ib_service) -> ::std::os::raw::c_int;
    pub fn rdma_create_qp(id: *mut rdma_cm_id, pd: *mut ib_pd, qp_init_attr: *mut ib_qp_init_attr) -> ::std::os::raw::c_int;
    pub fn rdma_destroy_qp(id: *mut rdma_cm_id);
    pub fn rdma_init_qp_attr(id: *mut rdma_cm_id, qp_attr: *mut ib_qp_attr, qp_attr_mask: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn rdma_connect(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> ::std::os::raw::c_int;
    pub fn rdma_connect_locked(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> ::std::os::raw::c_int;
    pub fn rdma_connect_ece(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param, ece: *mut rdma_ucm_ece) -> ::std::os::raw::c_int;
    pub fn rdma_listen(id: *mut rdma_cm_id, backlog: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn rdma_accept(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> ::std::os::raw::c_int;
    pub fn rdma_lock_handler(id: *mut rdma_cm_id);
    pub fn rdma_unlock_handler(id: *mut rdma_cm_id);
    pub fn rdma_accept_ece(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param, ece: *mut rdma_ucm_ece) -> ::std::os::raw::c_int;
    pub fn rdma_notify(id: *mut rdma_cm_id, event: ib_event_type) -> ::std::os::raw::c_int;
    pub fn rdma_reject(id: *mut rdma_cm_id, private_data: *const ::std::ffi::c_void, private_data_len: u8, reason: u8) -> ::std::os::raw::c_int;
    pub fn rdma_disconnect(id: *mut rdma_cm_id) -> ::std::os::raw::c_int;
    pub fn rdma_join_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr, join_state: u8, context: *mut ::std::ffi::c_void) -> ::std::os::raw::c_int;
    pub fn rdma_leave_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr);
    pub fn rdma_set_service_type(id: *mut rdma_cm_id, tos: ::std::os::raw::c_int);
    pub fn rdma_set_reuseaddr(id: *mut rdma_cm_id, reuse: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn rdma_set_afonly(id: *mut rdma_cm_id, afonly: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn rdma_set_ack_timeout(id: *mut rdma_cm_id, timeout: u8) -> ::std::os::raw::c_int;
    pub fn rdma_set_min_rnr_timer(id: *mut rdma_cm_id, min_rnr_timer: u8) -> ::std::os::raw::c_int;
    pub fn rdma_get_service_id(id: *mut rdma_cm_id, addr: *mut sockaddr) -> __be64;
    pub fn rdma_reject_msg(id: *mut rdma_cm_id, reason: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn rdma_consumer_reject_data(id: *mut rdma_cm_id, ev: *mut rdma_cm_event, data_len: *mut u8) -> *const ::std::ffi::c_void;
    pub fn rdma_read_gids(cm_id: *mut rdma_cm_id, sgid: *mut ib_gid, dgid: *mut ib_gid);
    pub fn rdma_iw_cm_id(cm_id: *mut rdma_cm_id) -> *mut iw_cm_id;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
