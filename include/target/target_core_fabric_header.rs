/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/configfs.h, linux/types.h, target/target_core_base.h

#[repr(C)]
pub struct target_core_fabric_ops {
    pub module: *mut module,
    /*
     * XXX: Special case for iscsi/iSCSI...
     * If non-null, fabric_alias is used for matching target/$fabric
     * ConfigFS paths. If null, fabric_name is used for this (see below).
     */
    pub fabric_alias: *const c_char,
    /*
     * fabric_name is used for matching target/$fabric ConfigFS paths
     * without a fabric_alias (see above). It's also used for the ALUA state
     * path and is stored on disk with PR state.
     */
    pub fabric_name: *const c_char,
    pub node_acl_size: size_t,
    /* Limits number of scatterlist entries per SCF_SCSI_DATA_CDB payload. */
    pub max_data_sg_nents: u32,
    pub tpg_get_wwn: Option<unsafe extern "C" fn(*mut se_portal_group) -> *mut c_char>,
    pub tpg_get_tag: Option<unsafe extern "C" fn(*mut se_portal_group) -> u16>,
    pub tpg_get_default_depth: Option<unsafe extern "C" fn(*mut se_portal_group) -> u32>,
    pub tpg_check_demo_mode: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    pub tpg_check_demo_mode_cache: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    pub tpg_check_demo_mode_write_protect: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    pub tpg_check_prod_mode_write_protect: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    /* Optionally used by fabrics to allow demo-mode login only. */
    pub tpg_check_demo_mode_login_only: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    /* Optionally used to determine when target-core should signal PROTECT=1. */
    pub tpg_check_prot_fabric_only: Option<unsafe extern "C" fn(*mut se_portal_group) -> c_int>,
    pub tpg_get_inst_index: Option<unsafe extern "C" fn(*mut se_portal_group) -> u32>,
    /* Optional release of struct se_cmd and fabric-dependent I/O descriptor. */
    pub check_stop_free: Option<unsafe extern "C" fn(*mut se_cmd) -> c_int>,
    pub release_cmd: Option<unsafe extern "C" fn(*mut se_cmd)>,
    pub close_session: Option<unsafe extern "C" fn(*mut se_session)>,
    pub sess_get_index: Option<unsafe extern "C" fn(*mut se_session) -> u32>,
    /* Used only for SCSI fabrics that contain multi-value TransportIDs. */
    pub sess_get_initiator_sid: Option<unsafe extern "C" fn(*mut se_session, *mut u8, u32) -> u32>,
    pub write_pending: Option<unsafe extern "C" fn(*mut se_cmd) -> c_int>,
    pub set_default_node_attributes: Option<unsafe extern "C" fn(*mut se_node_acl)>,
    pub get_cmd_state: Option<unsafe extern "C" fn(*mut se_cmd) -> c_int>,
    pub queue_data_in: Option<unsafe extern "C" fn(*mut se_cmd) -> c_int>,
    pub queue_status: Option<unsafe extern "C" fn(*mut se_cmd) -> c_int>,
    pub queue_tm_rsp: Option<unsafe extern "C" fn(*mut se_cmd)>,
    pub aborted_task: Option<unsafe extern "C" fn(*mut se_cmd)>,
    /* fabric module calls for target_core_fabric_configfs.c */
    pub fabric_make_wwn: Option<unsafe extern "C" fn(*mut target_fabric_configfs, *mut config_group, *const c_char) -> *mut se_wwn>,
    pub fabric_drop_wwn: Option<unsafe extern "C" fn(*mut se_wwn)>,
    pub add_wwn_groups: Option<unsafe extern "C" fn(*mut se_wwn)>,
    pub fabric_make_tpg: Option<unsafe extern "C" fn(*mut se_wwn, *const c_char) -> *mut se_portal_group>,
    pub fabric_enable_tpg: Option<unsafe extern "C" fn(*mut se_portal_group, bool) -> c_int>,
    pub fabric_drop_tpg: Option<unsafe extern "C" fn(*mut se_portal_group)>,
    pub fabric_post_link: Option<unsafe extern "C" fn(*mut se_portal_group, *mut se_lun) -> c_int>,
    pub fabric_pre_unlink: Option<unsafe extern "C" fn(*mut se_portal_group, *mut se_lun)>,
    pub fabric_post_unlink: Option<unsafe extern "C" fn(*mut se_portal_group, *mut se_lun)>,
    pub fabric_make_np: Option<unsafe extern "C" fn(*mut se_portal_group, *mut config_group, *const c_char) -> *mut se_tpg_np>,
    pub fabric_drop_np: Option<unsafe extern "C" fn(*mut se_tpg_np)>,
    pub fabric_init_nodeacl: Option<unsafe extern "C" fn(*mut se_node_acl, *const c_char) -> c_int>,
    pub tfc_discovery_attrs: *mut *mut configfs_attribute,
    pub tfc_wwn_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_base_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_np_base_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_attrib_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_auth_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_param_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_nacl_base_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_nacl_attrib_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_nacl_auth_attrs: *mut *mut configfs_attribute,
    pub tfc_tpg_nacl_param_attrs: *mut *mut configfs_attribute,
    pub write_pending_must_be_called: u32,
    pub direct_compl_supp: u32,
    pub direct_submit_supp: u32,
    pub default_submit_type: u8,
    pub default_compl_type: u8,
}

extern "C" {
    pub fn target_register_template(fo: *const target_core_fabric_ops) -> c_int;
    pub fn target_unregister_template(fo: *const target_core_fabric_ops);
    pub fn target_depend_item(item: *mut config_item) -> c_int;
    pub fn target_undepend_item(item: *mut config_item);
    pub fn target_setup_session(tpg: *mut se_portal_group, tag_num: c_uint, cmd_num: c_uint, prot_op: target_prot_op, name: *const c_char, priv_: *mut c_void, callback: Option<unsafe extern "C" fn(*mut se_portal_group, *mut se_session, *mut c_void) -> c_int>) -> *mut se_session;
    pub fn target_remove_session(sess: *mut se_session);
    pub fn target_stop_cmd_counter(cmd_cnt: *mut target_cmd_counter);
    pub fn target_wait_for_cmds(cmd_cnt: *mut target_cmd_counter);
    pub fn target_alloc_cmd_counter() -> *mut target_cmd_counter;
    pub fn target_free_cmd_counter(cmd_cnt: *mut target_cmd_counter);
    pub fn transport_init_session(se_sess: *mut se_session);
    pub fn transport_alloc_session(prot_op: target_prot_op) -> *mut se_session;
    pub fn transport_alloc_session_tags(se_sess: *mut se_session, tag_num: c_uint, cmd_num: c_uint) -> c_int;
    pub fn __transport_register_session(tpg: *mut se_portal_group, nacl: *mut se_node_acl, sess: *mut se_session, priv_: *mut c_void);
    pub fn transport_register_session(tpg: *mut se_portal_group, nacl: *mut se_node_acl, sess: *mut se_session, priv_: *mut c_void);
    pub fn target_show_dynamic_sessions(tpg: *mut se_portal_group, page: *mut c_char) -> ssize_t;
    pub fn transport_free_session(sess: *mut se_session);
    pub fn target_spc2_release(nacl: *mut se_node_acl);
    pub fn target_put_nacl(nacl: *mut se_node_acl);
    pub fn transport_deregister_session_configfs(sess: *mut se_session);
    pub fn transport_deregister_session(sess: *mut se_session);
}

/* Additional declarations from the target core fabric interface. */
extern "C" {
    pub fn __target_init_cmd(cmd: *mut se_cmd, tfo: *const target_core_fabric_ops, sess: *mut se_session, data_length: u32, data_direction: c_int, task_attr: c_int, sense_buffer: *mut u8, unpacked_lun: u64, cmd_cnt: *mut target_cmd_counter);
    pub fn target_init_cmd(se_cmd: *mut se_cmd, se_sess: *mut se_session, sense: *mut u8, unpacked_lun: u64, data_length: u32, task_attr: c_int, data_dir: c_int, flags: c_int) -> c_int;
    pub fn target_submit_prep(se_cmd: *mut se_cmd, cdb: *mut u8, sgl: *mut scatterlist, sgl_count: u32, sgl_bidi: *mut scatterlist, sgl_bidi_count: u32, sgl_prot: *mut scatterlist, sgl_prot_count: u32, gfp: gfp_t) -> c_int;
    pub fn target_submit(se_cmd: *mut se_cmd) -> c_int;
    pub fn transport_lookup_cmd_lun(se_cmd: *mut se_cmd) -> sense_reason_t;
    pub fn target_cmd_init_cdb(se_cmd: *mut se_cmd, cdb: *mut u8, gfp: gfp_t) -> sense_reason_t;
    pub fn target_cmd_parse_cdb(se_cmd: *mut se_cmd) -> sense_reason_t;
    pub fn target_submit_cmd(se_cmd: *mut se_cmd, se_sess: *mut se_session, cdb: *mut u8, sense: *mut u8, unpacked_lun: u64, data_length: u32, task_attr: c_int, data_dir: c_int, flags: c_int);
    pub fn target_submit_tmr(se_cmd: *mut se_cmd, se_sess: *mut se_session, sense: *mut u8, unpacked_lun: u64, fabric_tmr_ptr: *mut c_void, tm_type: u8, gfp: gfp_t, arg: u64, fabric_tm_ptr: c_int) -> c_int;
    pub fn transport_generic_new_cmd(se_cmd: *mut se_cmd) -> sense_reason_t;
    pub fn target_put_cmd_and_wait(cmd: *mut se_cmd);
    pub fn target_execute_cmd(cmd: *mut se_cmd);
    pub fn transport_generic_free_cmd(cmd: *mut se_cmd, wait_for_tasks: c_int) -> c_int;
    pub fn transport_wait_for_tasks(cmd: *mut se_cmd) -> bool;
    pub fn transport_send_check_condition_and_sense(cmd: *mut se_cmd, reason: sense_reason_t, from_transport: c_int) -> c_int;
    pub fn target_send_busy(cmd: *mut se_cmd) -> c_int;
    pub fn target_get_sess_cmd(cmd: *mut se_cmd, ack_kref: bool) -> c_int;
    pub fn target_put_sess_cmd(cmd: *mut se_cmd) -> c_int;
    pub fn target_stop_session(se_sess: *mut se_session);
    pub fn target_wait_for_sess_cmds(se_sess: *mut se_session);
    pub fn target_show_cmd(pfx: *const c_char, cmd: *mut se_cmd);
    pub fn core_tmr_alloc_req(cmd: *mut se_cmd, fabric_tmr_ptr: *mut c_void, tm_type: u8, gfp: gfp_t) -> c_int;
    pub fn core_tmr_release_req(tmr: *mut se_tmr_req);
    pub fn transport_generic_handle_tmr(cmd: *mut se_cmd) -> c_int;
    pub fn transport_generic_request_failure(cmd: *mut se_cmd, reason: sense_reason_t);
    pub fn transport_lookup_tmr_lun(cmd: *mut se_cmd) -> c_int;
    pub fn core_allocate_nexus_loss_ua(acl: *mut se_node_acl);
    pub fn core_tpg_get_initiator_node_acl(tpg: *mut se_portal_group, initiatorname: *mut u8) -> *mut se_node_acl;
    pub fn target_tpg_has_node_acl(tpg: *mut se_portal_group, initiatorname: *const c_char) -> bool;
    pub fn core_tpg_check_initiator_node_acl(tpg: *mut se_portal_group, initiatorname: *mut u8) -> *mut se_node_acl;
    pub fn core_tpg_set_initiator_node_queue_depth(nacl: *mut se_node_acl, depth: u32) -> c_int;
    pub fn core_tpg_set_initiator_node_tag(tpg: *mut se_portal_group, nacl: *mut se_node_acl, tag: *const c_char) -> c_int;
    pub fn core_tpg_register(wwn: *mut se_wwn, tpg: *mut se_portal_group, proto_id: c_int) -> c_int;
    pub fn core_tpg_deregister(tpg: *mut se_portal_group) -> c_int;
    pub fn target_alloc_sgl(sgl: *mut *mut scatterlist, nents: *mut c_uint, length: u32, zero_page: bool, chainable: bool) -> c_int;
    pub fn target_free_sgl(sgl: *mut scatterlist, nents: c_int);
}

/* The LIO target core uses DMA_TO_DEVICE for data going to the target and
 * DMA_FROM_DEVICE for data coming from the target, opposite to DMA mapping. */
pub unsafe fn target_reverse_dma_direction(se_cmd: *mut se_cmd) -> dma_data_direction {
    if ((*se_cmd).se_cmd_flags & SCF_BIDI) != 0 {
        return DMA_BIDIRECTIONAL;
    }

    match (*se_cmd).data_direction {
        DMA_TO_DEVICE => DMA_FROM_DEVICE,
        DMA_FROM_DEVICE => DMA_TO_DEVICE,
        DMA_NONE => DMA_NONE,
        _ => DMA_NONE,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
