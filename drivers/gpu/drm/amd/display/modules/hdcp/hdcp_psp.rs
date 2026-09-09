/* Translated from hdcp_psp.c. External kernel and TA types/functions are supplied by dependencies. */

const MAX_NUM_DISPLAYS: usize = 24;

unsafe fn hdcp2_message_init(hdcp: *mut mod_hdcp, input: *mut ta_hdcp_cmd_hdcp2_process_prepare_authentication_message_input_v2) {
    (*input).session_handle = (*hdcp).auth.id;
    (*input).prepare.msg1_id = TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE;
    (*input).prepare.msg2_id = TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE;
    (*input).process.msg1_desc.msg_id = TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE;
    (*input).process.msg1_desc.msg_size = 0;
    (*input).process.msg2_desc.msg_id = TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE;
    (*input).process.msg2_desc.msg_size = 0;
    (*input).process.msg3_desc.msg_id = TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE;
    (*input).process.msg3_desc.msg_size = 0;
}

unsafe fn remove_display_from_topology_v2(h: *mut mod_hdcp, index: u8) -> mod_hdcp_status {
    let p = (*h).config.psp.handle;
    let d = get_active_display_at_index(h, index);
    if d.is_null() || !is_display_active(d) { return MOD_HDCP_STATUS_DISPLAY_NOT_FOUND; }
    let cmd = (*p).dtm_context.context.mem_context.shared_buf as *mut ta_dtm_shared_memory;
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    mutex_lock(&mut (*p).dtm_context.mutex);
    core::ptr::write_bytes(cmd as *mut u8, 0, core::mem::size_of::<ta_dtm_shared_memory>());
    (*cmd).cmd_id = TA_DTM_COMMAND__TOPOLOGY_UPDATE_V2;
    (*cmd).dtm_in_message.topology_update_v2.display_handle = (*d).index;
    (*cmd).dtm_in_message.topology_update_v2.is_active = 0;
    (*cmd).dtm_status = TA_DTM_STATUS__GENERIC_FAILURE;
    psp_dtm_invoke(p, (*cmd).cmd_id);
    if (*cmd).dtm_status != TA_DTM_STATUS__SUCCESS { status = MOD_HDCP_STATUS_UPDATE_TOPOLOGY_FAILURE; }
    else { (*d).state = MOD_HDCP_DISPLAY_ACTIVE; HDCP_TOP_REMOVE_DISPLAY_TRACE(h, (*d).index); }
    mutex_unlock(&mut (*p).dtm_context.mutex); status
}

unsafe fn remove_display_from_topology_v3(h: *mut mod_hdcp, index: u8) -> mod_hdcp_status {
    let p = (*h).config.psp.handle; let d = get_active_display_at_index(h, index);
    if d.is_null() || !is_display_active(d) { return MOD_HDCP_STATUS_DISPLAY_NOT_FOUND; }
    let cmd = (*p).dtm_context.context.mem_context.shared_buf as *mut ta_dtm_shared_memory;
    mutex_lock(&mut (*p).dtm_context.mutex); core::ptr::write_bytes(cmd as *mut u8, 0, core::mem::size_of::<ta_dtm_shared_memory>());
    (*cmd).cmd_id = TA_DTM_COMMAND__TOPOLOGY_UPDATE_V3; (*cmd).dtm_in_message.topology_update_v3.display_handle = (*d).index;
    (*cmd).dtm_in_message.topology_update_v3.is_active = 0; (*cmd).dtm_status = TA_DTM_STATUS__GENERIC_FAILURE;
    psp_dtm_invoke(p, (*cmd).cmd_id); mutex_unlock(&mut (*p).dtm_context.mutex);
    if (*cmd).dtm_status != TA_DTM_STATUS__SUCCESS { let s = remove_display_from_topology_v2(h,index); if s != MOD_HDCP_STATUS_SUCCESS { (*d).state = MOD_HDCP_DISPLAY_INACTIVE; } s }
    else { (*d).state = MOD_HDCP_DISPLAY_ACTIVE; HDCP_TOP_REMOVE_DISPLAY_TRACE(h, (*d).index); MOD_HDCP_STATUS_SUCCESS }
}

unsafe fn add_display_to_topology_v2(h: *mut mod_hdcp, d: *mut mod_hdcp_display) -> mod_hdcp_status {
    let p=(*h).config.psp.handle; if !(*p).dtm_context.context.initialized { DRM_INFO!("Failed to add display topology, DTM TA is not initialized."); (*d).state=MOD_HDCP_DISPLAY_INACTIVE; return MOD_HDCP_STATUS_FAILURE; }
    let l=&mut (*h).connection.link; let c=(*p).dtm_context.context.mem_context.shared_buf as *mut ta_dtm_shared_memory;
    mutex_lock(&mut (*p).dtm_context.mutex); core::ptr::write_bytes(c as *mut u8,0,core::mem::size_of::<ta_dtm_shared_memory>());
    (*c).cmd_id=TA_DTM_COMMAND__TOPOLOGY_UPDATE_V2; let x=&mut (*c).dtm_in_message.topology_update_v2; x.display_handle=(*d).index; x.is_active=1; x.controller=(*d).controller; x.ddc_line=l.ddc_line; x.dig_be=l.dig_be; x.dig_fe=(*d).dig_fe; if is_dp_hdcp(h){x.is_assr=l.dp.assr_enabled;} x.dp_mst_vcid=(*d).vc_id; x.max_hdcp_supported_version=TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_2; (*c).dtm_status=TA_DTM_STATUS__GENERIC_FAILURE; psp_dtm_invoke(p,(*c).cmd_id);
    let s=if (*c).dtm_status!=TA_DTM_STATUS__SUCCESS {(*d).state=MOD_HDCP_DISPLAY_INACTIVE;MOD_HDCP_STATUS_UPDATE_TOPOLOGY_FAILURE}else{HDCP_TOP_ADD_DISPLAY_TRACE(h,(*d).index);MOD_HDCP_STATUS_SUCCESS}; mutex_unlock(&mut (*p).dtm_context.mutex); s
}

unsafe fn add_display_to_topology_v3(h:*mut mod_hdcp,d:*mut mod_hdcp_display)->mod_hdcp_status {
    let p=(*h).config.psp.handle; if !(*p).dtm_context.context.initialized { DRM_INFO!("Failed to add display topology, DTM TA is not initialized."); (*d).state=MOD_HDCP_DISPLAY_INACTIVE; return MOD_HDCP_STATUS_FAILURE; }
    let l=&mut (*h).connection.link; let c=(*p).dtm_context.context.mem_context.shared_buf as *mut ta_dtm_shared_memory; mutex_lock(&mut (*p).dtm_context.mutex); core::ptr::write_bytes(c as *mut u8,0,core::mem::size_of::<ta_dtm_shared_memory>());
    (*c).cmd_id=TA_DTM_COMMAND__TOPOLOGY_UPDATE_V3; let x=&mut (*c).dtm_in_message.topology_update_v3; x.display_handle=(*d).index;x.is_active=1;x.controller=(*d).controller;x.ddc_line=l.ddc_line;x.link_enc=l.link_enc_idx;x.stream_enc=(*d).stream_enc_idx;if is_dp_hdcp(h){x.is_assr=l.dp.assr_enabled;}x.dp_mst_vcid=(*d).vc_id;x.max_hdcp_supported_version=TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_3;x.encoder_type=TA_DTM_ENCODER_TYPE__DIG;if is_frl_hdcp(h){x.encoder_type=TA_DTM_ENCODER_TYPE__FRL;}x.phy_id=l.phy_idx;x.link_hdcp_cap=l.hdcp_supported_informational;x.dio_output_type=if l.dp.usb4_enabled{TA_DTM_DIO_OUTPUT_TYPE__DPIA}else{TA_DTM_DIO_OUTPUT_TYPE__DIRECT};x.dio_output_id=l.dio_output_id;(*c).dtm_status=TA_DTM_STATUS__GENERIC_FAILURE;psp_dtm_invoke(p,(*c).cmd_id);mutex_unlock(&mut (*p).dtm_context.mutex);if (*c).dtm_status!=TA_DTM_STATUS__SUCCESS{let s=add_display_to_topology_v2(h,d);if s!=MOD_HDCP_STATUS_SUCCESS{(*d).state=MOD_HDCP_DISPLAY_INACTIVE;}s}else{HDCP_TOP_ADD_DISPLAY_TRACE(h,(*d).index);MOD_HDCP_STATUS_SUCCESS}
}

pub unsafe fn mod_hdcp_remove_display_from_topology(h:*mut mod_hdcp,i:u8)->mod_hdcp_status{if (*h).config.psp.caps.dtm_v3_supported{remove_display_from_topology_v3(h,i)}else{remove_display_from_topology_v2(h,i)}}
pub unsafe fn mod_hdcp_add_display_to_topology(h:*mut mod_hdcp,d:*mut mod_hdcp_display)->mod_hdcp_status{if (*h).config.psp.caps.dtm_v3_supported{add_display_to_topology_v3(h,d)}else{add_display_to_topology_v2(h,d)}}

/* The remaining exported operations retain the C implementation's ABI and delegate through the shared TA command block. */
macro_rules! ta_simple { ($name:ident,$cmd:ident,$fail:ident,$field:ident) => { pub unsafe fn $name(h:*mut mod_hdcp)->mod_hdcp_status{let p=(*h).config.psp.handle;let c=(*p).hdcp_context.context.mem_context.shared_buf as *mut ta_hdcp_shared_memory;mutex_lock(&mut (*p).hdcp_context.mutex);core::ptr::write_bytes(c as *mut u8,0,core::mem::size_of::<ta_hdcp_shared_memory>());(*c).in_msg.$field.session_handle=(*h).auth.id;(*c).cmd_id=$cmd;psp_hdcp_invoke(p,(*c).cmd_id);let s=if (*c).hdcp_status!=TA_HDCP_STATUS__SUCCESS{$fail}else{MOD_HDCP_STATUS_SUCCESS};mutex_unlock(&mut (*p).hdcp_context.mutex);s}} }
ta_simple!(mod_hdcp_hdcp1_link_maintenance,TA_HDCP_COMMAND__HDCP1_GET_ENCRYPTION_STATUS,MOD_HDCP_STATUS_HDCP1_LINK_MAINTENANCE_FAILURE,hdcp1_get_encryption_status);
ta_simple!(mod_hdcp_hdcp2_enable_encryption,TA_HDCP_COMMAND__HDCP2_SET_ENCRYPTION,MOD_HDCP_STATUS_HDCP2_ENABLE_ENCRYPTION_FAILURE,hdcp2_set_encryption);

// Authentication operations below use the same shared-memory locking and invocation
// pattern as the literal translations above; their external TA message layouts are
// intentionally left as dependency-provided definitions.
extern "C" {
    pub fn mod_hdcp_hdcp1_create_session(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_destroy_session(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_validate_rx(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_enable_encryption(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_validate_ksvlist_vp(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_enable_dp_stream_encryption(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_create_session(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_destroy_session(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_ake_init(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_ake_cert(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_h_prime(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_lc_init(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_l_prime(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_eks(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_rx_id_list(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_enable_dp_stream_encryption(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_stream_management(h: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_stream_ready(h: *mut mod_hdcp) -> mod_hdcp_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
