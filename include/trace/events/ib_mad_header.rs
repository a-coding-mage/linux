/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Rust translation of trace/events/ib_mad.h. */

// The following types are supplied by the RDMA and tracepoint headers.
#[repr(C)] pub struct ib_mad_send_wr_private { _private: [u8; 0] }
#[repr(C)] pub struct ib_mad_qp_info { _private: [u8; 0] }
#[repr(C)] pub struct ib_wc { _private: [u8; 0] }
#[repr(C)] pub struct ib_mad_hdr { _private: [u8; 0] }
#[repr(C)] pub struct ib_mad_agent_private { _private: [u8; 0] }
#[repr(C)] pub struct opa_smp { _private: [u8; 0] }
#[repr(C)] pub struct ib_smp { _private: [u8; 0] }

#[cfg(feature = "tracepoints")]
extern "C" {
    fn create_mad_addr_info(
        mad_send_wr: *mut ib_mad_send_wr_private,
        qp_info: *mut ib_mad_qp_info,
        entry: *mut ib_mad_send_template_entry,
    );
}

#[repr(C)]
pub struct ib_mad_send_template_entry {
    pub base_version: u8, pub mgmt_class: u8, pub class_version: u8, pub port_num: u8,
    pub qp_num: u32, pub method: u8, pub sl: u8, pub attr_id: u16, pub attr_mod: u32,
    pub wrtid: u64, pub tid: u64, pub status: u16, pub class_specific: u16,
    pub length: u32, pub dlid: u32, pub rqpn: u32, pub rqkey: u32, pub dev_index: u32,
    pub agent_priv: *mut core::ffi::c_void, pub timeout: usize,
    pub retries_left: i32, pub max_retries: i32, pub retry: i32,
}

#[repr(C)]
pub struct ib_mad_send_done_handler_entry {
    pub port_num: u8, pub base_version: u8, pub mgmt_class: u8, pub class_version: u8,
    pub qp_num: u32, pub wrtid: u64, pub status: u16, pub wc_status: u16,
    pub length: u32, pub agent_priv: *mut core::ffi::c_void, pub timeout: usize,
    pub dev_index: u32, pub retries_left: i32, pub max_retries: i32, pub retry: i32,
    pub method: u8,
}

#[repr(C)]
pub struct ib_mad_recv_done_handler_entry {
    pub base_version: u8, pub mgmt_class: u8, pub class_version: u8, pub port_num: u8,
    pub qp_num: u32, pub status: u16, pub class_specific: u16, pub length: u32,
    pub tid: u64, pub method: u8, pub sl: u8, pub attr_id: u16, pub attr_mod: u32,
    pub src_qp: u16, pub wc_status: u16, pub slid: u32, pub dev_index: u32,
}

#[repr(C)]
pub struct ib_mad_agent_template_entry {
    pub dev_index: u32, pub hi_tid: u32, pub port_num: u8, pub mgmt_class: u8,
    pub mgmt_class_version: u8,
}

#[repr(C)]
pub struct ib_mad_opa_smi_template_entry<const N: usize> {
    pub mkey: u64, pub dr_slid: u32, pub dr_dlid: u32, pub hop_ptr: u8, pub hop_cnt: u8,
    pub initial_path: [u8; N], pub return_path: [u8; N],
}

// DECLARE_EVENT_CLASS/TRACE_EVENT and DEFINE_EVENT expand to tracepoint
// declarations and registration metadata in the kernel.  These declarations
// preserve each externally visible event name and its original arguments.
extern "C" {
    pub fn ib_mad_error_handler(wr: *mut ib_mad_send_wr_private, qp_info: *mut ib_mad_qp_info);
    pub fn ib_mad_ib_send_mad(wr: *mut ib_mad_send_wr_private, qp_info: *mut ib_mad_qp_info);
    pub fn ib_mad_send_done_resend(wr: *mut ib_mad_send_wr_private, qp_info: *mut ib_mad_qp_info);
    pub fn ib_mad_send_done_handler(wr: *mut ib_mad_send_wr_private, wc: *mut ib_wc);
    pub fn ib_mad_recv_done_handler(qp_info: *mut ib_mad_qp_info, wc: *mut ib_wc, mad_hdr: *mut ib_mad_hdr);
    pub fn ib_mad_recv_done_agent(agent: *mut ib_mad_agent_private);
    pub fn ib_mad_send_done_agent(agent: *mut ib_mad_agent_private);
    pub fn ib_mad_create_agent(agent: *mut ib_mad_agent_private);
    pub fn ib_mad_unregister_agent(agent: *mut ib_mad_agent_private);
    pub fn ib_mad_handle_opa_smi(smp: *mut opa_smp);
    pub fn ib_mad_handle_out_opa_smi(smp: *mut opa_smp);
    pub fn ib_mad_handle_ib_smi(smp: *mut ib_smp);
    pub fn ib_mad_handle_out_ib_smi(smp: *mut ib_smp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
