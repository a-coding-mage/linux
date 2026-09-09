/* SPDX-License-Identifier: GPL-2.0 */
// Dependency: iscsi_target_core.h (struct iscsit_cmd)

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct sockaddr_storage;

#[repr(C)]
pub struct iscsit_transport {
    pub name: [c_char; ISCSIT_TRANSPORT_NAME],
    pub transport_type: ::core::ffi::c_int,
    pub rdma_shutdown: bool,
    pub priv_size: ::core::ffi::c_int,
    pub owner: *mut module,
    pub t_node: list_head,
    pub iscsit_setup_np: Option<unsafe extern "C" fn(*mut iscsi_np, *mut sockaddr_storage) -> ::core::ffi::c_int>,
    pub iscsit_accept_np: Option<unsafe extern "C" fn(*mut iscsi_np, *mut iscsit_conn) -> ::core::ffi::c_int>,
    pub iscsit_free_np: Option<unsafe extern "C" fn(*mut iscsi_np)>,
    pub iscsit_wait_conn: Option<unsafe extern "C" fn(*mut iscsit_conn)>,
    pub iscsit_free_conn: Option<unsafe extern "C" fn(*mut iscsit_conn)>,
    pub iscsit_get_login_rx: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsi_login) -> ::core::ffi::c_int>,
    pub iscsit_put_login_tx: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsi_login, u32) -> ::core::ffi::c_int>,
    pub iscsit_immediate_queue: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub iscsit_response_queue: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub iscsit_get_dataout: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd, bool) -> ::core::ffi::c_int>,
    pub iscsit_queue_data_in: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd) -> ::core::ffi::c_int>,
    pub iscsit_queue_status: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd) -> ::core::ffi::c_int>,
    pub iscsit_aborted_task: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd)>,
    pub iscsit_xmit_pdu: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd, *mut iscsi_datain_req, *const c_void, u32) -> ::core::ffi::c_int>,
    pub iscsit_unmap_cmd: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd)>,
    pub iscsit_get_rx_pdu: Option<unsafe extern "C" fn(*mut iscsit_conn)>,
    pub iscsit_validate_params: Option<unsafe extern "C" fn(*mut iscsit_conn) -> ::core::ffi::c_int>,
    pub iscsit_get_r2t_ttt: Option<unsafe extern "C" fn(*mut iscsit_conn, *mut iscsit_cmd, *mut iscsi_r2t)>,
    pub iscsit_get_sup_prot_ops: Option<unsafe extern "C" fn(*mut iscsit_conn) -> target_prot_op>,
}

pub const ISCSIT_TRANSPORT_NAME: usize = 16;

pub unsafe fn iscsit_priv_cmd(cmd: *mut iscsit_cmd) -> *mut c_void {
    cmd.add(1) as *mut c_void
}

/*
 * From iscsi_target_transport.c
 */
extern "C" {
    pub fn iscsit_register_transport(transport: *mut iscsit_transport);
    pub fn iscsit_unregister_transport(transport: *mut iscsit_transport);
    pub fn iscsit_get_transport(transport_type: ::core::ffi::c_int) -> *mut iscsit_transport;
    pub fn iscsit_put_transport(transport: *mut iscsit_transport);

    /* From iscsi_target.c */
    pub fn iscsit_setup_scsi_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_set_unsolicited_dataout(cmd: *mut iscsit_cmd);
    pub fn iscsit_process_scsi_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, req: *mut iscsi_scsi_req) -> ::core::ffi::c_int;
    pub fn __iscsit_check_dataout_hdr(conn: *mut iscsit_conn, buf: *mut c_void, cmd: *mut iscsit_cmd, length: u32, is_immediate: *mut bool) -> ::core::ffi::c_int;
    pub fn iscsit_check_dataout_hdr(conn: *mut iscsit_conn, buf: *mut c_void, out_cmd: *mut *mut iscsit_cmd) -> ::core::ffi::c_int;
    pub fn iscsit_check_dataout_payload(cmd: *mut iscsit_cmd, data: *mut iscsi_data, immediate: bool) -> ::core::ffi::c_int;
    pub fn iscsit_setup_nop_out(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, nop: *mut iscsi_nopout) -> ::core::ffi::c_int;
    pub fn iscsit_process_nop_out(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, nop: *mut iscsi_nopout) -> ::core::ffi::c_int;
    pub fn iscsit_handle_logout_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_handle_task_mgt_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_setup_text_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, text: *mut iscsi_text) -> ::core::ffi::c_int;
    pub fn iscsit_process_text_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, text: *mut iscsi_text) -> ::core::ffi::c_int;
    pub fn iscsit_build_rsp_pdu(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, flag: bool, rsp: *mut iscsi_scsi_rsp);
    pub fn iscsit_build_nopin_rsp(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, nop: *mut iscsi_nopin, flag: bool);
    pub fn iscsit_build_task_mgt_rsp(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, rsp: *mut iscsi_tm_rsp);
    pub fn iscsit_build_text_rsp(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, rsp: *mut iscsi_text_rsp, transport_type: iscsit_transport_type) -> ::core::ffi::c_int;
    pub fn iscsit_build_reject(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, reject: *mut iscsi_reject);
    pub fn iscsit_build_logout_rsp(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, rsp: *mut iscsi_logout_rsp) -> ::core::ffi::c_int;
    pub fn iscsit_logout_post_handler(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn) -> ::core::ffi::c_int;
    pub fn iscsit_queue_rsp(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd) -> ::core::ffi::c_int;
    pub fn iscsit_aborted_task(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd);
    pub fn iscsit_add_reject(conn: *mut iscsit_conn, reason: u8, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_reject_cmd(cmd: *mut iscsit_cmd, reason: u8, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_handle_snack(conn: *mut iscsit_conn, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn iscsit_build_datain_pdu(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, data: *mut iscsi_datain, rsp: *mut iscsi_data_rsp, flag: bool);
    pub fn iscsit_build_r2ts_for_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, flag: bool) -> ::core::ffi::c_int;
    pub fn iscsit_immediate_queue(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, value: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn iscsit_response_queue(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, value: ::core::ffi::c_int) -> ::core::ffi::c_int;

    /* From iscsi_target_device.c */
    pub fn iscsit_increment_maxcmdsn(cmd: *mut iscsit_cmd, session: *mut iscsit_session);
    /* From iscsi_target_erl0.c */
    pub fn iscsit_cause_connection_reinstatement(conn: *mut iscsit_conn, value: ::core::ffi::c_int);
    /* From iscsi_target_erl1.c */
    pub fn iscsit_stop_dataout_timer(cmd: *mut iscsit_cmd);
    /* From iscsi_target_tmr.c */
    pub fn iscsit_tmr_post_handler(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn) -> ::core::ffi::c_int;
    /* From iscsi_target_util.c */
    pub fn iscsit_allocate_cmd(conn: *mut iscsit_conn, value: ::core::ffi::c_int) -> *mut iscsit_cmd;
    pub fn iscsit_sequence_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd, buf: *mut u8, tag: __be32) -> ::core::ffi::c_int;
    pub fn iscsit_release_cmd(cmd: *mut iscsit_cmd);
    pub fn iscsit_free_cmd(cmd: *mut iscsit_cmd, flag: bool);
    pub fn iscsit_add_cmd_to_immediate_queue(cmd: *mut iscsit_cmd, conn: *mut iscsit_conn, value: u8);
    pub fn iscsit_find_cmd_from_itt_or_dump(conn: *mut iscsit_conn, init_task_tag: itt_t, length: u32) -> *mut iscsit_cmd;
    /* From iscsi_target_nego.c */
    pub fn iscsi_target_check_login_request(conn: *mut iscsit_conn, login: *mut iscsi_login) -> ::core::ffi::c_int;
    /* From iscsi_target_login.c: __printf(2, 3) */
    pub fn iscsi_change_param_sprintf(conn: *mut iscsit_conn, fmt: *const c_char, ...) -> ::core::ffi::c_int;
    /* From iscsi_target_parameters.c */
    pub fn iscsi_find_param_from_key(key: *mut c_char, list: *mut iscsi_param_list) -> *mut iscsi_param;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
