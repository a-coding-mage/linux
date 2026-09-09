/* Translated from mod_hdcp.h. */

// External types supplied by os_types.h and signal_types.h.
use core::ffi::c_void;

pub const MAX_NUM_OF_DISPLAYS: usize = 6;
pub const MAX_NUM_OF_ATTEMPTS: usize = 4;
pub const MAX_NUM_OF_ERROR_TRACE: usize = 10;

#[repr(C)]
pub struct mod_hdcp;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mod_hdcp_status {
    MOD_HDCP_STATUS_SUCCESS,
    MOD_HDCP_STATUS_FAILURE,
    MOD_HDCP_STATUS_RESET_NEEDED,
    MOD_HDCP_STATUS_DISPLAY_OUT_OF_BOUND,
    MOD_HDCP_STATUS_DISPLAY_NOT_FOUND,
    MOD_HDCP_STATUS_INVALID_STATE,
    MOD_HDCP_STATUS_NOT_IMPLEMENTED,
    MOD_HDCP_STATUS_INTERNAL_POLICY_FAILURE,
    MOD_HDCP_STATUS_UPDATE_TOPOLOGY_FAILURE,
    MOD_HDCP_STATUS_CREATE_PSP_SERVICE_FAILURE,
    MOD_HDCP_STATUS_DESTROY_PSP_SERVICE_FAILURE,
    MOD_HDCP_STATUS_HDCP1_CREATE_SESSION_FAILURE,
    MOD_HDCP_STATUS_HDCP1_DESTROY_SESSION_FAILURE,
    MOD_HDCP_STATUS_HDCP1_VALIDATE_ENCRYPTION_FAILURE,
    MOD_HDCP_STATUS_HDCP1_NOT_HDCP_REPEATER,
    MOD_HDCP_STATUS_HDCP1_NOT_CAPABLE,
    MOD_HDCP_STATUS_HDCP1_R0_PRIME_PENDING,
    MOD_HDCP_STATUS_HDCP1_VALIDATE_RX_FAILURE,
    MOD_HDCP_STATUS_HDCP1_BKSV_REVOKED,
    MOD_HDCP_STATUS_HDCP1_KSV_LIST_NOT_READY,
    MOD_HDCP_STATUS_HDCP1_VALIDATE_KSV_LIST_FAILURE,
    MOD_HDCP_STATUS_HDCP1_KSV_LIST_REVOKED,
    MOD_HDCP_STATUS_HDCP1_ENABLE_ENCRYPTION_FAILURE,
    MOD_HDCP_STATUS_HDCP1_ENABLE_STREAM_ENCRYPTION_FAILURE,
    MOD_HDCP_STATUS_HDCP1_MAX_CASCADE_EXCEEDED_FAILURE,
    MOD_HDCP_STATUS_HDCP1_MAX_DEVS_EXCEEDED_FAILURE,
    MOD_HDCP_STATUS_HDCP1_DEVICE_COUNT_MISMATCH_FAILURE,
    MOD_HDCP_STATUS_HDCP1_LINK_INTEGRITY_FAILURE,
    MOD_HDCP_STATUS_HDCP1_REAUTH_REQUEST_ISSUED,
    MOD_HDCP_STATUS_HDCP1_LINK_MAINTENANCE_FAILURE,
    MOD_HDCP_STATUS_HDCP1_INVALID_BKSV,
    MOD_HDCP_STATUS_DDC_FAILURE,
    MOD_HDCP_STATUS_INVALID_OPERATION,
    MOD_HDCP_STATUS_HDCP2_NOT_CAPABLE,
    MOD_HDCP_STATUS_HDCP2_CREATE_SESSION_FAILURE,
    MOD_HDCP_STATUS_HDCP2_DESTROY_SESSION_FAILURE,
    MOD_HDCP_STATUS_HDCP2_PREP_AKE_INIT_FAILURE,
    MOD_HDCP_STATUS_HDCP2_AKE_CERT_PENDING,
    MOD_HDCP_STATUS_HDCP2_H_PRIME_PENDING,
    MOD_HDCP_STATUS_HDCP2_PAIRING_INFO_PENDING,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_AKE_CERT_FAILURE,
    MOD_HDCP_STATUS_HDCP2_AKE_CERT_REVOKED,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_H_PRIME_FAILURE,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_PAIRING_INFO_FAILURE,
    MOD_HDCP_STATUS_HDCP2_PREP_LC_INIT_FAILURE,
    MOD_HDCP_STATUS_HDCP2_L_PRIME_PENDING,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_L_PRIME_FAILURE,
    MOD_HDCP_STATUS_HDCP2_PREP_EKS_FAILURE,
    MOD_HDCP_STATUS_HDCP2_ENABLE_ENCRYPTION_FAILURE,
    MOD_HDCP_STATUS_HDCP2_RX_ID_LIST_NOT_READY,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_RX_ID_LIST_FAILURE,
    MOD_HDCP_STATUS_HDCP2_RX_ID_LIST_REVOKED,
    MOD_HDCP_STATUS_HDCP2_ENABLE_STREAM_ENCRYPTION_FAILURE,
    MOD_HDCP_STATUS_HDCP2_STREAM_READY_PENDING,
    MOD_HDCP_STATUS_HDCP2_VALIDATE_STREAM_READY_FAILURE,
    MOD_HDCP_STATUS_HDCP2_PREPARE_STREAM_MANAGEMENT_FAILURE,
    MOD_HDCP_STATUS_HDCP2_REAUTH_REQUEST,
    MOD_HDCP_STATUS_HDCP2_REAUTH_LINK_INTEGRITY_FAILURE,
    MOD_HDCP_STATUS_HDCP2_DEVICE_COUNT_MISMATCH_FAILURE,
    MOD_HDCP_STATUS_UNSUPPORTED_PSP_VER_FAILURE,
    MOD_HDCP_STATUS_HDCP2_LOCALITY_COMBO_READ_FAILURE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_displayport { pub rev: u8, pub assr_enabled: u8, pub mst_enabled: u8, pub dp2_enabled: u8, pub usb4_enabled: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_hdmi { pub frl_enabled: u8, pub reserved: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mod_hdcp_operation_mode { MOD_HDCP_MODE_OFF, MOD_HDCP_MODE_DEFAULT, MOD_HDCP_MODE_DP }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mod_hdcp_display_state { MOD_HDCP_DISPLAY_INACTIVE = 0, MOD_HDCP_DISPLAY_ACTIVE, MOD_HDCP_DISPLAY_ENCRYPTION_ENABLED }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_psp_caps { pub dtm_v3_supported: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum mod_hdcp_display_disable_option { MOD_HDCP_DISPLAY_NOT_DISABLE = 0, MOD_HDCP_DISPLAY_DISABLE_AUTHENTICATION, MOD_HDCP_DISPLAY_DISABLE_ENCRYPTION }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_atomic_op_i2c { pub address: u8, pub offset: u8, pub data: *mut u8, pub size: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_atomic_op_aux { pub address: u32, pub data: *mut u8, pub size: u32 }

pub type ReadI2c = unsafe extern "C" fn(*mut c_void, u32, u8, *mut u8, u32) -> bool;
pub type WriteI2c = unsafe extern "C" fn(*mut c_void, u32, u8, *const u8, u32) -> bool;
pub type ReadDpcd = unsafe extern "C" fn(*mut c_void, u32, *mut u8, u32) -> bool;
pub type WriteDpcd = unsafe extern "C" fn(*mut c_void, u32, *const u8, u32) -> bool;
pub type AtomicI2c = unsafe extern "C" fn(*mut c_void, *const mod_hdcp_atomic_op_i2c, *const mod_hdcp_atomic_op_i2c, *mut mod_hdcp_atomic_op_i2c, u32, u8) -> bool;
pub type AtomicAux = unsafe extern "C" fn(*mut c_void, *const mod_hdcp_atomic_op_aux, *const mod_hdcp_atomic_op_aux, *mut mod_hdcp_atomic_op_aux, u32, u8) -> bool;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_ddc_funcs { pub read_i2c: Option<ReadI2c>, pub write_i2c: Option<WriteI2c>, pub read_dpcd: Option<ReadDpcd>, pub write_dpcd: Option<WriteDpcd>, pub atomic_write_poll_read_i2c: Option<AtomicI2c>, pub atomic_write_poll_read_aux: Option<AtomicAux> }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_ddc { pub handle: *mut c_void, pub funcs: mod_hdcp_ddc_funcs }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_hdcp_psp { pub handle: *mut c_void, pub funcs: *mut c_void, pub caps: mod_hdcp_psp_caps }

// C bit-fields are represented by their containing byte; masks are dependency-facing.
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_display_adjustment { pub bits: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_link_adjustment_hdcp1 { pub bits: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub enum mod_hdcp_force_hdcp_type { MOD_HDCP_FORCE_TYPE_MAX = 0, MOD_HDCP_FORCE_TYPE_0, MOD_HDCP_FORCE_TYPE_1 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_link_adjustment_hdcp2 { pub bits: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_link_adjustment { pub auth_delay: u8, pub retry_limit: u8, pub hdcp1: mod_hdcp_link_adjustment_hdcp1, pub hdcp2: mod_hdcp_link_adjustment_hdcp2 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_error { pub status: mod_hdcp_status, pub state_id: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp1_trace { pub attempt_count: u8, pub downstream_device_count: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp2_trace { pub attempt_count: u8, pub downstream_device_count: u8, pub hdcp1_device_downstream: u8, pub hdcp2_legacy_device_downstream: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_trace { pub errors: [mod_hdcp_error; MAX_NUM_OF_ERROR_TRACE], pub error_count: u8, pub hdcp1: mod_hdcp1_trace, pub hdcp2: mod_hdcp2_trace }

#[repr(C)] #[derive(Copy, Clone)] pub enum mod_hdcp_encryption_status { MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF = 0, MOD_HDCP_ENCRYPTION_STATUS_HDCP1_ON, MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE0_ON, MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE1_ON, MOD_HDCP_ENCRYPTION_STATUS_HDCP2_ON }
#[repr(C)] #[derive(Copy, Clone)] pub enum mod_hdcp_event { MOD_HDCP_EVENT_CALLBACK = 0, MOD_HDCP_EVENT_WATCHDOG_TIMEOUT, MOD_HDCP_EVENT_CPIRQ }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_output { pub callback_needed: u8, pub callback_stop: u8, pub watchdog_timer_needed: u8, pub watchdog_timer_stop: u8, pub callback_delay: u16, pub watchdog_timer_delay: u16, pub auth_complete: u8 }

#[repr(C)] #[derive(Copy, Clone)] pub union mod_hdcp_display_union { pub vc_id: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_display { pub state: mod_hdcp_display_state, pub index: u8, pub controller: u8, pub dig_fe: u8, pub stream_enc_idx: u8, pub vc: mod_hdcp_display_union, pub adjust: mod_hdcp_display_adjustment }
#[repr(C)] #[derive(Copy, Clone)] pub union mod_hdcp_link_union { pub dp: mod_hdcp_displayport, pub hdmi: mod_hdcp_hdmi }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_link { pub mode: mod_hdcp_operation_mode, pub dig_be: u8, pub ddc_line: u8, pub link_enc_idx: u8, pub phy_idx: u8, pub dio_output_id: u8, pub hdcp_supported_informational: u8, pub signal: mod_hdcp_link_union, pub adjust: mod_hdcp_link_adjustment }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_display_query { pub display: *const mod_hdcp_display, pub link: *const mod_hdcp_link, pub trace: *const mod_hdcp_trace, pub encryption_status: mod_hdcp_encryption_status }
#[repr(C)] #[derive(Copy, Clone)] pub struct mod_hdcp_config { pub psp: mod_hdcp_psp, pub ddc: mod_hdcp_ddc, pub index: u8 }

extern "C" {
    pub fn mod_hdcp_get_memory_size() -> usize;
    pub fn mod_hdcp_setup(hdcp: *mut mod_hdcp, config: *mut mod_hdcp_config) -> mod_hdcp_status;
    pub fn mod_hdcp_teardown(hdcp: *mut mod_hdcp) -> mod_hdcp_status;
    pub fn mod_hdcp_add_display(hdcp: *mut mod_hdcp, link: *mut mod_hdcp_link, display: *mut mod_hdcp_display, output: *mut mod_hdcp_output) -> mod_hdcp_status;
    pub fn mod_hdcp_remove_display(hdcp: *mut mod_hdcp, index: u8, output: *mut mod_hdcp_output) -> mod_hdcp_status;
    pub fn mod_hdcp_update_display(hdcp: *mut mod_hdcp, index: u8, link_adjust: *mut mod_hdcp_link_adjustment, display_adjust: *mut mod_hdcp_display_adjustment, output: *mut mod_hdcp_output) -> mod_hdcp_status;
    pub fn mod_hdcp_query_display(hdcp: *mut mod_hdcp, index: u8, query: *mut mod_hdcp_display_query) -> mod_hdcp_status;
    pub fn mod_hdcp_reset_connection(hdcp: *mut mod_hdcp, output: *mut mod_hdcp_output) -> mod_hdcp_status;
    pub fn mod_hdcp_process_event(hdcp: *mut mod_hdcp, event: mod_hdcp_event, output: *mut mod_hdcp_output) -> mod_hdcp_status;
    pub fn mod_hdcp_status_to_str(status: i32) -> *mut u8;
    pub fn mod_hdcp_state_id_to_str(id: i32) -> *mut u8;
    pub fn mod_hdcp_signal_type_to_operation_mode(signal: signal_type) -> mod_hdcp_operation_mode;
}

// Supplied by signal_types.h.
#[repr(C)] pub enum signal_type {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
