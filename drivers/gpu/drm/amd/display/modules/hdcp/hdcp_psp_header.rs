/* Translated from hdcp_psp.h. */

/* These parameters are a one-to-one copy of the parameters required by PSP. */
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum bgd_security_hdcp_encryption_level { HDCP_ENCRYPTION_LEVEL__INVALID = 0, HDCP_ENCRYPTION_LEVEL__OFF, HDCP_ENCRYPTION_LEVEL__ON }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum bgd_security_hdcp2_content_type { HDCP2_CONTENT_TYPE__INVALID = 0, HDCP2_CONTENT_TYPE__TYPE0, HDCP2_CONTENT_TYPE__TYPE1 }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ta_dtm_command { TA_DTM_COMMAND__UNUSED_1 = 1, TA_DTM_COMMAND__TOPOLOGY_UPDATE_V2, TA_DTM_COMMAND__TOPOLOGY_ASSR_ENABLE, TA_DTM_COMMAND__TOPOLOGY_UPDATE_V3 }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ta_dtm_status { TA_DTM_STATUS__SUCCESS = 0, TA_DTM_STATUS__GENERIC_FAILURE = 1, TA_DTM_STATUS__INVALID_PARAMETER = 2, TA_DTM_STATUS__NULL_POINTER = 3 }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ta_dtm_hdcp_version_max_supported { TA_DTM_HDCP_VERSION_MAX_SUPPORTED__NONE=0, TA_DTM_HDCP_VERSION_MAX_SUPPORTED__1_x=10, TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_0=20, TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_1=21, TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_2=22, TA_DTM_HDCP_VERSION_MAX_SUPPORTED__2_3=23 }
#[repr(C)]
pub struct ta_dtm_topology_update_input_v2 { pub display_handle:u32,pub is_active:u32,pub is_miracast:u32,pub controller:u32,pub ddc_line:u32,pub dig_be:u32,pub dig_fe:u32,pub dp_mst_vcid:u32,pub is_assr:u32,pub max_hdcp_supported_version:u32 }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ta_dtm_encoder_type { TA_DTM_ENCODER_TYPE__INVALID=0, TA_DTM_ENCODER_TYPE__FRL=0x20, TA_DTM_ENCODER_TYPE__DIG=0x10 }
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ta_dtm_dio_output_type { TA_DTM_DIO_OUTPUT_TYPE__INVALID, TA_DTM_DIO_OUTPUT_TYPE__DIRECT, TA_DTM_DIO_OUTPUT_TYPE__DPIA }
#[repr(C)]
pub struct ta_dtm_topology_update_input_v3 { pub display_handle:u32,pub is_active:u32,pub is_miracast:u32,pub controller:u32,pub ddc_line:u32,pub link_enc:u32,pub stream_enc:u32,pub dp_mst_vcid:u32,pub is_assr:u32,pub max_hdcp_supported_version:u32,pub encoder_type:ta_dtm_encoder_type,pub phy_id:u32,pub link_hdcp_cap:u32,pub dio_output_type:ta_dtm_dio_output_type,pub dio_output_id:u32 }
#[repr(C)] pub struct ta_dtm_topology_assr_enable { pub display_topology_dig_be_index:u32 }
#[repr(C)] pub union ta_dtm_cmd_input { pub topology_update_v2:ta_dtm_topology_update_input_v2, pub topology_assr_enable:ta_dtm_topology_assr_enable, pub topology_update_v3:ta_dtm_topology_update_input_v3 }
#[repr(C)] pub union ta_dtm_cmd_output { pub reserved:u32 }
#[repr(C)] pub struct ta_dtm_shared_memory { pub cmd_id:u32,pub resp_id:u32,pub dtm_status:ta_dtm_status,pub reserved:u32,pub dtm_in_message:ta_dtm_cmd_input,pub dtm_out_message:ta_dtm_cmd_output }

extern "C" { pub fn psp_cmd_submit_buf(psp:*mut psp_context, ucode:*mut amdgpu_firmware_info, cmd:*mut psp_gfx_cmd_resp, fence_mc_addr:u64) -> i32; }
pub const PSP_HDCP_SRM_FIRST_GEN_MAX_SIZE:usize = 5120;
pub enum psp_context {} pub enum amdgpu_firmware_info {} pub enum psp_gfx_cmd_resp {}

#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp_command { TA_HDCP_COMMAND__INITIALIZE,TA_HDCP_COMMAND__HDCP1_CREATE_SESSION,TA_HDCP_COMMAND__HDCP1_DESTROY_SESSION,TA_HDCP_COMMAND__HDCP1_FIRST_PART_AUTHENTICATION,TA_HDCP_COMMAND__HDCP1_SECOND_PART_AUTHENTICATION,TA_HDCP_COMMAND__HDCP1_ENABLE_ENCRYPTION,TA_HDCP_COMMAND__HDCP1_ENABLE_DP_STREAM_ENCRYPTION,TA_HDCP_COMMAND__HDCP1_GET_ENCRYPTION_STATUS,TA_HDCP_COMMAND__UNUSED_1,TA_HDCP_COMMAND__HDCP2_DESTROY_SESSION,TA_HDCP_COMMAND__UNUSED_2,TA_HDCP_COMMAND__HDCP2_SET_ENCRYPTION,TA_HDCP_COMMAND__HDCP2_GET_ENCRYPTION_STATUS,TA_HDCP_COMMAND__UNUSED_3,TA_HDCP_COMMAND__HDCP2_CREATE_SESSION_V2,TA_HDCP_COMMAND__HDCP2_PREPARE_PROCESS_AUTHENTICATION_MSG_V2,TA_HDCP_COMMAND__HDCP2_ENABLE_DP_STREAM_ENCRYPTION,TA_HDCP_COMMAND__HDCP_DESTROY_ALL_SESSIONS,TA_HDCP_COMMAND__HDCP_SET_SRM,TA_HDCP_COMMAND__HDCP_GET_SRM }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp2_msg_id { TA_HDCP_HDCP2_MSG_ID__NULL_MESSAGE=1,TA_HDCP_HDCP2_MSG_ID__AKE_INIT=2,TA_HDCP_HDCP2_MSG_ID__AKE_SEND_CERT=3,TA_HDCP_HDCP2_MSG_ID__AKE_NO_STORED_KM=4,TA_HDCP_HDCP2_MSG_ID__AKE_STORED_KM=5,TA_HDCP_HDCP2_MSG_ID__AKE_SEND_RRX=6,TA_HDCP_HDCP2_MSG_ID__AKE_SEND_H_PRIME=7,TA_HDCP_HDCP2_MSG_ID__AKE_SEND_PAIRING_INFO=8,TA_HDCP_HDCP2_MSG_ID__LC_INIT=9,TA_HDCP_HDCP2_MSG_ID__LC_SEND_L_PRIME=10,TA_HDCP_HDCP2_MSG_ID__SKE_SEND_EKS=11,TA_HDCP_HDCP2_MSG_ID__REPEATERAUTH_SEND_RECEIVERID_LIST=12,TA_HDCP_HDCP2_MSG_ID__RTT_READY=13,TA_HDCP_HDCP2_MSG_ID__RTT_CHALLENGE=14,TA_HDCP_HDCP2_MSG_ID__REPEATERAUTH_SEND_ACK=15,TA_HDCP_HDCP2_MSG_ID__REPEATERAUTH_STREAM_MANAGE=16,TA_HDCP_HDCP2_MSG_ID__REPEATERAUTH_STREAM_READY=17,TA_HDCP_HDCP2_MSG_ID__RECEIVER_AUTH_STATUS=18,TA_HDCP_HDCP2_MSG_ID__AKE_TRANSMITTER_INFO=19,TA_HDCP_HDCP2_MSG_ID__AKE_RECEIVER_INFO=20,TA_HDCP_HDCP2_MSG_ID__SIGNAL_CONTENT_STREAM_TYPE_DP=129 }
pub const TA_HDCP__INVALID_SESSION:u16=0xffff; pub const TA_HDCP__HDCP1_AN_SIZE:usize=8; pub const TA_HDCP__HDCP1_KSV_SIZE:usize=5; pub const TA_HDCP__HDCP1_KSV_LIST_MAX_ENTRIES:usize=127; pub const TA_HDCP__HDCP1_V_PRIME_SIZE:usize=20;
pub const TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_NO_STORED_KM:usize=129; pub const TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_STORED_KM:usize=33; pub const TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_SEND_CERT:usize=534; pub const TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_RECEIVER_INFO:usize=6; pub const TA_HDCP__HDCP2_TX_BUF_MAX_SIZE:usize=168; pub const TA_HDCP__HDCP2_RX_BUF_MAX_SIZE:usize=544;
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp2_hdcp2_msg_id_max_size { TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__NULL_MESSAGE=0,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_INIT=12,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_SEND_CERT=534,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_NO_STORED_KM=129,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_STORED_KM=33,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_SEND_RRX=9,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_SEND_H_PRIME=33,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_SEND_PAIRING_INFO=17,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__LC_INIT=9,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__LC_SEND_L_PRIME=33,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__SKE_SEND_EKS=25,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__REPEATERAUTH_SEND_RECEIVERID_LIST=181,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__RTT_READY=1,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__RTT_CHALLENGE=17,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__REPEATERAUTH_SEND_RACK=17,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__REPEATERAUTH_STREAM_MANAGE=13,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__REPEATERAUTH_STREAM_READY=33,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__RECEIVER_AUTH_STATUS=4,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_TRANSMITTER_INFO=6,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__AKE_RECEIVER_INFO=6,TA_HDCP_HDCP2_MSG_ID_MAX_SIZE__SIGNAL_CONTENT_STREAM_TYPE_DP=1 }

#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp_status { TA_HDCP_STATUS__SUCCESS=0,TA_HDCP_STATUS__GENERIC_FAILURE,TA_HDCP_STATUS__NULL_POINTER,TA_HDCP_STATUS__FAILED_ALLOCATING_SESSION,TA_HDCP_STATUS__FAILED_SETUP_TX,TA_HDCP_STATUS__INVALID_PARAMETER,TA_HDCP_STATUS__VHX_ERROR,TA_HDCP_STATUS__SESSION_NOT_CLOSED_PROPERLY,TA_HDCP_STATUS__SRM_FAILURE,TA_HDCP_STATUS__MST_AUTHENTICATED_ALREADY_STARTED,TA_HDCP_STATUS__AKE_SEND_CERT_FAILURE,TA_HDCP_STATUS__AKE_NO_STORED_KM_FAILURE,TA_HDCP_STATUS__AKE_SEND_HPRIME_FAILURE,TA_HDCP_STATUS__LC_SEND_LPRIME_FAILURE,TA_HDCP_STATUS__SKE_SEND_EKS_FAILURE,TA_HDCP_STATUS__REPAUTH_SEND_RXIDLIST_FAILURE,TA_HDCP_STATUS__REPAUTH_STREAM_READY_FAILURE,TA_HDCP_STATUS__ASD_GENERIC_FAILURE,TA_HDCP_STATUS__UNWRAP_SECRET_FAILURE,TA_HDCP_STATUS__ENABLE_ENCR_FAILURE,TA_HDCP_STATUS__DISABLE_ENCR_FAILURE,TA_HDCP_STATUS__NOT_ENOUGH_MEMORY_FAILURE,TA_HDCP_STATUS__UNKNOWN_MESSAGE,TA_HDCP_STATUS__TOO_MANY_STREAM }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp2_msg_authentication_status { TA_HDCP2_MSG_AUTHENTICATION_STATUS__SUCCESS=0,TA_HDCP2_MSG_AUTHENTICATION_STATUS__KM_NOT_AVAILABLE,TA_HDCP2_MSG_AUTHENTICATION_STATUS__UNUSED,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID=100,TA_HDCP2_MSG_AUTHENTICATION_STATUS__NOT_ENOUGH_MEMORY,TA_HDCP2_MSG_AUTHENTICATION_STATUS__NOT_EXPECTED_MSG,TA_HDCP2_MSG_AUTHENTICATION_STATUS__SIGNATURE_CERTIFICAT_ERROR,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INCORRECT_HDCP_VERSION,TA_HDCP2_MSG_AUTHENTICATION_STATUS__UNKNOWN_MESSAGE,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID_HMAC,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID_TOPOLOGY,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID_SEQ_NUM,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID_SIZE,TA_HDCP2_MSG_AUTHENTICATION_STATUS__INVALID_LENGTH,TA_HDCP2_MSG_AUTHENTICATION_STATUS__REAUTH_REQUEST,TA_HDCP2_MSG_AUTHENTICATION_STATUS__RECEIVERID_REVOKED }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp_authentication_status { TA_HDCP_AUTHENTICATION_STATUS__NOT_STARTED=0,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_FIRST_PART_FAILED,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_FIRST_PART_COMPLETE,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_SECOND_PART_FAILED,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_AUTHENTICATED,TA_HDCP_AUTHENTICATION_STATUS__HDCP22_AUTHENTICATION_PENDING=6,TA_HDCP_AUTHENTICATION_STATUS__HDCP22_AUTHENTICATION_FAILED,TA_HDCP_AUTHENTICATION_STATUS__HDCP22_AUTHENTICATED,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_KSV_VALIDATION_FAILED,TA_HDCP_AUTHENTICATION_STATUS__HDCP1_KSV_REVOKED }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp_content_type { TA_HDCP2_CONTENT_TYPE__TYPE0=1,TA_HDCP2_CONTENT_TYPE__TYPE1 }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp_content_type_negotiation_type { TA_HDCP2_CONTENT_TYPE_NEGOTIATION_TYPE__FORCE_TYPE0=1,TA_HDCP2_CONTENT_TYPE_NEGOTIATION_TYPE__FORCE_TYPE1,TA_HDCP2_CONTENT_TYPE_NEGOTIATION_TYPE__MAX_SUPPORTED }
#[repr(u32)] #[derive(Clone,Copy)] pub enum ta_hdcp2_version { TA_HDCP2_VERSION_UNKNOWN=0,TA_HDCP2_VERSION_2_0=20,TA_HDCP2_VERSION_2_1=21,TA_HDCP2_VERSION_2_2=22,TA_HDCP2_VERSION_2_3=23 }

/* HDCP command structures. */
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_create_session_input{pub display_handle:u8}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_create_session_output{pub session_handle:u32,pub an_primary:[u8;8],pub aksv_primary:[u8;5],pub ainfo_primary:u8,pub an_secondary:[u8;8],pub aksv_secondary:[u8;5],pub ainfo_secondary:u8}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_destroy_session_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_first_part_authentication_input{pub session_handle:u32,pub bksv_primary:[u8;5],pub bksv_secondary:[u8;5],pub bcaps:u8,pub r0_prime_primary:u16,pub r0_prime_secondary:u16}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_first_part_authentication_output{pub authentication_status:ta_hdcp_authentication_status}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_second_part_authentication_input{pub session_handle:u32,pub bstatus_binfo:u16,pub ksv_list:[[u8;5];127],pub ksv_list_size:u32,pub pj_prime:u8,pub v_prime:[u8;20]}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_second_part_authentication_output{pub authentication_status:ta_hdcp_authentication_status}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_enable_encryption_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_enable_dp_stream_encryption_input{pub session_handle:u32,pub display_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_get_encryption_status_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp1_get_encryption_status_output{pub protection_level:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_create_session_input_v2{pub display_handle:u32,pub negotiate_content_type:ta_hdcp_content_type_negotiation_type}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_create_session_output_v2{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_destroy_session_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_authentication_message_v2{pub msg_id:ta_hdcp2_msg_id,pub msg_size:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_process_authentication_message_input_v2{pub msg1_desc:ta_hdcp_cmd_hdcp2_authentication_message_v2,pub msg2_desc:ta_hdcp_cmd_hdcp2_authentication_message_v2,pub msg3_desc:ta_hdcp_cmd_hdcp2_authentication_message_v2,pub receiver_message:[u8;544]}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_process_authentication_message_output_v2{pub hdcp_version:u32,pub is_km_stored:u32,pub is_locality_precompute_support:u32,pub is_repeater:u32,pub msg1_status:u32,pub msg2_status:u32,pub msg3_status:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_prepare_authentication_message_input_v2{pub msg1_id:ta_hdcp2_msg_id,pub msg2_id:ta_hdcp2_msg_id}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_prepare_authentication_message_output_v2{pub msg1_status:u32,pub msg2_status:u32,pub msg1_desc:ta_hdcp_cmd_hdcp2_authentication_message_v2,pub msg2_desc:ta_hdcp_cmd_hdcp2_authentication_message_v2,pub transmitter_message:[u8;168]}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_process_prepare_authentication_message_input_v2{pub session_handle:u32,pub process:ta_hdcp_cmd_hdcp2_process_authentication_message_input_v2,pub prepare:ta_hdcp_cmd_hdcp2_prepare_authentication_message_input_v2}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_process_prepare_authentication_message_output_v2{pub authentication_status:u32,pub process:ta_hdcp_cmd_hdcp2_process_authentication_message_output_v2,pub prepare:ta_hdcp_cmd_hdcp2_prepare_authentication_message_output_v2}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_set_encryption_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_get_encryption_status_input{pub session_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_get_encryption_status_output{pub hdcp2_type:ta_hdcp_content_type,pub protection_level:u32}
#[repr(C)] pub struct ta_hdcp_cmd_hdcp2_enable_dp_stream_encryption_input{pub session_handle:u32,pub display_handle:u32}
#[repr(C)] pub struct ta_hdcp_cmd_set_srm_input{pub srm_buf_size:u32,pub srm_buf:[u8;5120]}
#[repr(C)] pub struct ta_hdcp_cmd_set_srm_output{pub valid_signature:u8,pub srm_version:u32}
#[repr(C)] pub struct ta_hdcp_cmd_get_srm_output{pub srm_version:u32,pub srm_buf_size:u32,pub srm_buf:[u8;5120]}

#[repr(C)] pub union ta_hdcp_cmd_input { pub hdcp1_create_session:ta_hdcp_cmd_hdcp1_create_session_input,pub hdcp1_destroy_session:ta_hdcp_cmd_hdcp1_destroy_session_input,pub hdcp1_first_part_authentication:ta_hdcp_cmd_hdcp1_first_part_authentication_input,pub hdcp1_second_part_authentication:ta_hdcp_cmd_hdcp1_second_part_authentication_input,pub hdcp1_enable_encryption:ta_hdcp_cmd_hdcp1_enable_encryption_input,pub hdcp1_enable_dp_stream_encryption:ta_hdcp_cmd_hdcp1_enable_dp_stream_encryption_input,pub hdcp1_get_encryption_status:ta_hdcp_cmd_hdcp1_get_encryption_status_input,pub hdcp2_destroy_session:ta_hdcp_cmd_hdcp2_destroy_session_input,pub hdcp2_set_encryption:ta_hdcp_cmd_hdcp2_set_encryption_input,pub hdcp2_get_encryption_status:ta_hdcp_cmd_hdcp2_get_encryption_status_input,pub hdcp2_create_session_v2:ta_hdcp_cmd_hdcp2_create_session_input_v2,pub hdcp2_prepare_process_authentication_message_v2:ta_hdcp_cmd_hdcp2_process_prepare_authentication_message_input_v2,pub hdcp2_enable_dp_stream_encryption:ta_hdcp_cmd_hdcp2_enable_dp_stream_encryption_input,pub hdcp_set_srm:ta_hdcp_cmd_set_srm_input }
#[repr(C)] pub union ta_hdcp_cmd_output { pub hdcp1_create_session:ta_hdcp_cmd_hdcp1_create_session_output,pub hdcp1_first_part_authentication:ta_hdcp_cmd_hdcp1_first_part_authentication_output,pub hdcp1_second_part_authentication:ta_hdcp_cmd_hdcp1_second_part_authentication_output,pub hdcp1_get_encryption_status:ta_hdcp_cmd_hdcp1_get_encryption_status_output,pub hdcp2_get_encryption_status:ta_hdcp_cmd_hdcp2_get_encryption_status_output,pub hdcp2_create_session_v2:ta_hdcp_cmd_hdcp2_create_session_output_v2,pub hdcp2_prepare_process_authentication_message_v2:ta_hdcp_cmd_hdcp2_process_prepare_authentication_message_output_v2,pub hdcp_set_srm:ta_hdcp_cmd_set_srm_output,pub hdcp_get_srm:ta_hdcp_cmd_get_srm_output }
#[repr(C)] pub struct ta_hdcp_shared_memory{pub cmd_id:u32,pub hdcp_status:ta_hdcp_status,pub reserved:u32,pub in_msg:ta_hdcp_cmd_input,pub out_msg:ta_hdcp_cmd_output}
#[repr(u32)] #[derive(Clone,Copy)] pub enum psp_status { PSP_STATUS__SUCCESS=0,PSP_STATUS__ERROR_INVALID_PARAMS,PSP_STATUS__ERROR_GENERIC,PSP_STATUS__ERROR_OUT_OF_MEMORY,PSP_STATUS__ERROR_UNSUPPORTED_FEATURE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
