/* C header translation: hdcp.h */

// Dependencies supplied by other translation units:
// mod_hdcp.h, hdcp_log.h, drm display helpers.

#[repr(C)]
pub enum mod_hdcp_trans_input_result { UNKNOWN = 0, PASS, FAIL }

#[repr(C)]
pub struct mod_hdcp_transition_input_hdcp1 {
    pub bksv_read: u8, pub bksv_validation: u8, pub create_session: u8,
    pub an_write: u8, pub aksv_write: u8, pub ainfo_write: u8,
    pub bcaps_read: u8, pub r0p_read: u8, pub rx_validation: u8,
    pub encryption: u8, pub link_maintenance: u8, pub ready_check: u8,
    pub bstatus_read: u8, pub max_cascade_check: u8, pub max_devs_check: u8,
    pub device_count_check: u8, pub ksvlist_read: u8, pub vp_read: u8,
    pub ksvlist_vp_validation: u8, pub hdcp_capable_dp: u8,
    pub binfo_read_dp: u8, pub r0p_available_dp: u8, pub link_integrity_check: u8,
    pub reauth_request_check: u8, pub stream_encryption_dp: u8,
}

#[repr(C)]
pub struct mod_hdcp_transition_input_hdcp2 {
    pub hdcp2version_read: u8, pub hdcp2_capable_check: u8, pub create_session: u8,
    pub ake_init_prepare: u8, pub ake_init_write: u8, pub rxstatus_read: u8,
    pub ake_cert_available: u8, pub ake_cert_read: u8, pub ake_cert_validation: u8,
    pub stored_km_write: u8, pub no_stored_km_write: u8, pub h_prime_available: u8,
    pub h_prime_read: u8, pub pairing_available: u8, pub pairing_info_read: u8,
    pub h_prime_validation: u8, pub lc_init_prepare: u8, pub lc_init_write: u8,
    pub l_prime_available_poll: u8, pub l_prime_read: u8, pub l_prime_combo_read: u8,
    pub l_prime_validation: u8, pub eks_prepare: u8, pub eks_write: u8,
    pub enable_encryption: u8, pub reauth_request_check: u8, pub rx_id_list_read: u8,
    pub device_count_check: u8, pub rx_id_list_validation: u8,
    pub repeater_auth_ack_write: u8, pub prepare_stream_manage: u8,
    pub stream_manage_write: u8, pub stream_ready_available: u8,
    pub stream_ready_read: u8, pub stream_ready_validation: u8,
    pub rx_caps_read_dp: u8, pub content_stream_type_write: u8,
    pub link_integrity_check_dp: u8, pub stream_encryption_dp: u8,
}

#[repr(C)] pub union mod_hdcp_transition_input {
    pub hdcp1: mod_hdcp_transition_input_hdcp1,
    pub hdcp2: mod_hdcp_transition_input_hdcp2,
}

#[repr(C)] pub struct mod_hdcp_message_hdcp1 {
    pub an: [u8;8], pub aksv: [u8;5], pub ainfo: u8, pub bksv: [u8;5],
    pub r0p: u16, pub bcaps: u8, pub bstatus: u16, pub ksvlist: [u8;635],
    pub ksvlist_size: u16, pub vp: [u8;20], pub binfo_dp: u16,
}
#[repr(C)] pub struct mod_hdcp_message_hdcp2 {
    pub hdcp2version_hdmi: u8, pub rxcaps_dp: [u8;3], pub rxstatus: [u8;2],
    pub ake_init: [u8;12], pub ake_cert: [u8;534], pub ake_no_stored_km: [u8;129],
    pub ake_stored_km: [u8;33], pub ake_h_prime: [u8;33], pub ake_pairing_info: [u8;17],
    pub lc_init: [u8;9], pub lc_l_prime: [u8;33], pub ske_eks: [u8;25],
    pub rx_id_list: [u8;177], pub rx_id_list_size: u16, pub repeater_auth_ack: [u8;17],
    pub repeater_auth_stream_manage: [u8;68], pub stream_manage_size: u16,
    pub repeater_auth_stream_ready: [u8;33], pub rxstatus_dp: u8,
    pub content_stream_type_dp: [u8;2],
}
#[repr(C)] pub union mod_hdcp_message { pub hdcp1: mod_hdcp_message_hdcp1, pub hdcp2: mod_hdcp_message_hdcp2 }
#[repr(C)] pub struct mod_hdcp_auth_counters { pub stream_management_retry_count: u8 }

#[repr(C)] pub struct mod_hdcp_connection {
    pub link: mod_hdcp_link, pub is_repeater: u8, pub is_km_stored: u8,
    pub is_hdcp1_revoked: u8, pub is_hdcp2_revoked: u8, pub trace: mod_hdcp_trace,
    pub hdcp1_retry_count: u8, pub hdcp2_retry_count: u8,
}
#[repr(C)] pub struct mod_hdcp_authentication { pub id: u32, pub msg: mod_hdcp_message, pub trans_input: mod_hdcp_transition_input, pub count: mod_hdcp_auth_counters }
#[repr(C)] pub struct mod_hdcp_state { pub id: u8, pub stay_count: u32 }
#[repr(C)] pub struct mod_hdcp_event_context { pub event: mod_hdcp_event, pub rx_id_list_ready: u8, pub unexpected_event: u8 }
#[repr(C)] pub struct mod_hdcp {
    pub config: mod_hdcp_config, pub connection: mod_hdcp_connection,
    pub displays: [mod_hdcp_display; MAX_NUM_OF_DISPLAYS as usize],
    pub auth: mod_hdcp_authentication, pub state: mod_hdcp_state, pub buf: [u8;2025],
}

#[repr(C)] pub enum mod_hdcp_initial_state_id { HDCP_UNINITIALIZED=0, HDCP_INITIALIZED, HDCP_CP_NOT_DESIRED }
pub const HDCP_INITIAL_STATE_START: mod_hdcp_initial_state_id = mod_hdcp_initial_state_id::HDCP_UNINITIALIZED;
pub const HDCP_INITIAL_STATE_END: mod_hdcp_initial_state_id = mod_hdcp_initial_state_id::HDCP_CP_NOT_DESIRED;
#[repr(C)] pub enum mod_hdcp_hdcp1_state_id { HDCP1_STATE_START=2, H1_A0_WAIT_FOR_ACTIVE_RX, H1_A1_EXCHANGE_KSVS, H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER, H1_A45_AUTHENTICATED, H1_A8_WAIT_FOR_READY, H1_A9_READ_KSV_LIST }
pub const HDCP1_STATE_END: mod_hdcp_hdcp1_state_id = mod_hdcp_hdcp1_state_id::H1_A9_READ_KSV_LIST;
#[repr(C)] pub enum mod_hdcp_hdcp1_dp_state_id { HDCP1_DP_STATE_START=8, D1_A0_DETERMINE_RX_HDCP_CAPABLE, D1_A1_EXCHANGE_KSVS, D1_A23_WAIT_FOR_R0_PRIME, D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER, D1_A4_AUTHENTICATED, D1_A6_WAIT_FOR_READY, D1_A7_READ_KSV_LIST }
pub const HDCP1_DP_STATE_END: mod_hdcp_hdcp1_dp_state_id = mod_hdcp_hdcp1_dp_state_id::D1_A7_READ_KSV_LIST;
#[repr(C)] pub enum mod_hdcp_hdcp2_state_id { HDCP2_STATE_START=16, H2_A0_KNOWN_HDCP2_CAPABLE_RX, H2_A1_SEND_AKE_INIT, H2_A1_VALIDATE_AKE_CERT, H2_A1_SEND_NO_STORED_KM, H2_A1_READ_H_PRIME, H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME, H2_A1_SEND_STORED_KM, H2_A1_VALIDATE_H_PRIME, H2_A2_LOCALITY_CHECK, H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER, H2_ENABLE_ENCRYPTION, H2_A5_AUTHENTICATED, H2_A6_WAIT_FOR_RX_ID_LIST, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK, H2_A9_SEND_STREAM_MANAGEMENT, H2_A9_VALIDATE_STREAM_READY }
pub const HDCP2_STATE_END: mod_hdcp_hdcp2_state_id = mod_hdcp_hdcp2_state_id::H2_A9_VALIDATE_STREAM_READY;
#[repr(C)] pub enum mod_hdcp_hdcp2_dp_state_id { HDCP2_DP_STATE_START=32, D2_A0_DETERMINE_RX_HDCP_CAPABLE, D2_A1_SEND_AKE_INIT, D2_A1_VALIDATE_AKE_CERT, D2_A1_SEND_NO_STORED_KM, D2_A1_READ_H_PRIME, D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME, D2_A1_SEND_STORED_KM, D2_A1_VALIDATE_H_PRIME, D2_A2_LOCALITY_CHECK, D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER, D2_SEND_CONTENT_STREAM_TYPE, D2_ENABLE_ENCRYPTION, D2_A5_AUTHENTICATED, D2_A6_WAIT_FOR_RX_ID_LIST, D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK, D2_A9_SEND_STREAM_MANAGEMENT, D2_A9_VALIDATE_STREAM_READY }
pub const HDCP2_DP_STATE_END: mod_hdcp_hdcp2_dp_state_id = mod_hdcp_hdcp2_dp_state_id::D2_A9_VALIDATE_STREAM_READY;

// External declarations from the included headers and companion source files.
pub type mod_hdcp_action = unsafe extern "C" fn(*mut mod_hdcp) -> mod_hdcp_status;
extern "C" {
    pub fn mod_hdcp_execute_and_set(func: mod_hdcp_action, flag: *mut u8, status: *mut mod_hdcp_status, hdcp: *mut mod_hdcp, str_: *mut i8) -> u8;
    pub fn mod_hdcp_dump_binary_message(msg:*mut u8,msg_size:u32,buf:*mut u8,buf_size:u32);
    pub fn mod_hdcp_log_ddc_trace(hdcp:*mut mod_hdcp);
    pub fn mod_hdcp_hdcp1_execution(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_dp_execution(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_transition(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp1,output:*mut mod_hdcp_output)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_dp_transition(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp1,output:*mut mod_hdcp_output)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_execution(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_dp_execution(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_transition(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp2,output:*mut mod_hdcp_output)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_dp_transition(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp2,output:*mut mod_hdcp_output)->mod_hdcp_status;
}

// The remaining function declarations are intentionally represented as external symbols.
extern "C" {
    pub fn mod_hdcp_add_display_to_topology(hdcp:*mut mod_hdcp, display:*mut mod_hdcp_display)->mod_hdcp_status;
    pub fn mod_hdcp_remove_display_from_topology(hdcp:*mut mod_hdcp,index:u8)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_create_session(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_destroy_session(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_validate_rx(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_enable_encryption(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_validate_ksvlist_vp(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_enable_dp_stream_encryption(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp1_link_maintenance(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_create_session(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_destroy_session(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_ake_init(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_ake_cert(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_h_prime(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_lc_init(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_l_prime(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_eks(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_enable_encryption(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_rx_id_list(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_enable_dp_stream_encryption(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_prepare_stream_management(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_hdcp2_validate_stream_ready(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_bksv(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_bcaps(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_bstatus(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_r0p(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_ksvlist(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_vp(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_binfo(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_aksv(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_ainfo(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_an(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_hdcp2version(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_rxcaps(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_rxstatus(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_ake_cert(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_h_prime(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_pairing_info(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_l_prime(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_read_rx_id_list(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_read_stream_ready(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_ake_init(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_no_stored_km(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_stored_km(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_lc_init(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_eks(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_repeater_auth_ack(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_write_stream_manage(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_content_type(hdcp:*mut mod_hdcp)->mod_hdcp_status; pub fn mod_hdcp_clear_cp_irq_status(hdcp:*mut mod_hdcp)->mod_hdcp_status;
    pub fn mod_hdcp_write_poll_read_lc_fw(hdcp:*mut mod_hdcp)->mod_hdcp_status;
}

#[inline] pub unsafe fn is_dp_hdcp(h:*mut mod_hdcp)->u8 { ((*h).connection.link.mode == MOD_HDCP_MODE_DP) as u8 }
#[inline] pub unsafe fn is_dp_mst_hdcp(h:*mut mod_hdcp)->u8 { ((*h).connection.link.mode == MOD_HDCP_MODE_DP && (*h).connection.link.dp.mst_enabled != 0) as u8 }
#[inline] pub unsafe fn is_hdmi_dvi_sl_hdcp(h:*mut mod_hdcp)->u8 { ((*h).connection.link.mode == MOD_HDCP_MODE_DEFAULT) as u8 }
#[inline] pub unsafe fn is_frl_hdcp(h:*mut mod_hdcp)->u8 { ((*h).connection.link.mode == MOD_HDCP_MODE_DEFAULT && (*h).connection.link.hdmi.frl_enabled != 0) as u8 }
#[inline] pub unsafe fn current_state(h:*mut mod_hdcp)->u8 { (*h).state.id }
#[inline] pub unsafe fn increment_stay_counter(h:*mut mod_hdcp) { (*h).state.stay_count = (*h).state.stay_count.wrapping_add(1); }
#[inline] pub unsafe fn fail_and_restart_in_ms(time:u16,status:*mut mod_hdcp_status,output:*mut mod_hdcp_output) { (*output).callback_needed=1; (*output).callback_delay=time; (*output).watchdog_timer_needed=0; (*output).watchdog_timer_delay=0; *status=MOD_HDCP_STATUS_RESET_NEEDED; }
#[inline] pub unsafe fn callback_in_ms(time:u16,output:*mut mod_hdcp_output) { (*output).callback_needed=1; (*output).callback_delay=time; }
#[inline] pub unsafe fn set_watchdog_in_ms(_h:*mut mod_hdcp,time:u16,output:*mut mod_hdcp_output) { (*output).watchdog_timer_needed=1; (*output).watchdog_timer_delay=time; }
#[inline] pub unsafe fn set_auth_complete(h:*mut mod_hdcp,output:*mut mod_hdcp_output) { (*output).auth_complete=1; HDCP_AUTH_COMPLETE_TRACE(h); }
#[inline] pub unsafe fn is_display_active(d:*mut mod_hdcp_display)->u8 { ((*d).state >= MOD_HDCP_DISPLAY_ACTIVE) as u8 }
#[inline] pub unsafe fn is_display_encryption_enabled(d:*mut mod_hdcp_display)->u8 { ((*d).state >= MOD_HDCP_DISPLAY_ENCRYPTION_ENABLED) as u8 }
#[inline] pub unsafe fn get_active_display_count(h:*mut mod_hdcp)->u8 { let mut n=0; for i in 0..MAX_NUM_OF_DISPLAYS as usize { if is_display_active((*h).displays.as_mut_ptr().add(i)) != 0 { n+=1; } } n }
#[inline] pub unsafe fn get_first_active_display(h:*mut mod_hdcp)->*mut mod_hdcp_display { for i in 0..MAX_NUM_OF_DISPLAYS as usize { let d=(*h).displays.as_mut_ptr().add(i); if is_display_active(d)!=0 { return d; } } core::ptr::null_mut() }
#[inline] pub unsafe fn get_active_display_at_index(h:*mut mod_hdcp,index:u8)->*mut mod_hdcp_display { for i in 0..MAX_NUM_OF_DISPLAYS as usize { let d=(*h).displays.as_mut_ptr().add(i); if (*d).index==index && is_display_active(d)!=0 { return d; } } core::ptr::null_mut() }
#[inline] pub unsafe fn get_empty_display_container(h:*mut mod_hdcp)->*mut mod_hdcp_display { for i in 0..MAX_NUM_OF_DISPLAYS as usize { let d=(*h).displays.as_mut_ptr().add(i); if is_display_active(d)==0 { return d; } } core::ptr::null_mut() }
#[inline] pub unsafe fn reset_retry_counts(h:*mut mod_hdcp) { (*h).connection.hdcp1_retry_count=0; (*h).connection.hdcp2_retry_count=0; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
