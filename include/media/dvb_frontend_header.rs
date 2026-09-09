/* Translated from media/dvb_frontend.h. External kernel and DVB types are
 * intentionally referenced rather than reimplemented here. */

pub const MAX_DELSYS: usize = 8;
pub const KHZ: usize = 1000;
pub const MHZ: usize = 1_000_000;

#[repr(C)]
pub struct dvb_frontend_tune_settings { pub min_delay_ms: i32, pub step_size: i32, pub max_drift: i32 }

#[repr(C)]
pub struct dvb_tuner_info {
    pub name: [u8; 128],
    pub frequency_min_hz: u32, pub frequency_max_hz: u32, pub frequency_step_hz: u32,
    pub bandwidth_min: u32, pub bandwidth_max: u32, pub bandwidth_step: u32,
}

#[repr(C)]
pub struct analog_parameters { pub frequency: u32, pub mode: u32, pub audmode: u32, pub std: u64 }

#[repr(i32)]
pub enum dvbfe_algo {
    DVBFE_ALGO_HW = 1 << 0, DVBFE_ALGO_SW = 1 << 1,
    DVBFE_ALGO_CUSTOM = 1 << 2, DVBFE_ALGO_RECOVERY = 1 << 31,
}

#[repr(i32)]
pub enum dvbfe_search {
    DVBFE_ALGO_SEARCH_SUCCESS = 1 << 0, DVBFE_ALGO_SEARCH_ASLEEP = 1 << 1,
    DVBFE_ALGO_SEARCH_FAILED = 1 << 2, DVBFE_ALGO_SEARCH_INVALID = 1 << 3,
    DVBFE_ALGO_SEARCH_AGAIN = 1 << 4, DVBFE_ALGO_SEARCH_ERROR = 1 << 31,
}

#[repr(C)]
pub struct dvb_tuner_ops {
    pub info: dvb_tuner_info,
    pub release: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub init: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub sleep: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub set_params: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub set_analog_params: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut analog_parameters) -> i32>,
    pub set_config: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut core::ffi::c_void) -> i32>,
    pub get_frequency: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub get_bandwidth: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub get_if_frequency: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub get_status: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub get_rf_strength: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u16) -> i32>,
    pub get_afc: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut i32) -> i32>,
    pub calc_regs: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u8, i32) -> i32>,
    pub set_frequency: Option<unsafe extern "C" fn(*mut dvb_frontend, u32) -> i32>,
    pub set_bandwidth: Option<unsafe extern "C" fn(*mut dvb_frontend, u32) -> i32>,
}
pub const TUNER_STATUS_LOCKED: u32 = 1;
pub const TUNER_STATUS_STEREO: u32 = 2;

#[repr(C)]
pub struct analog_demod_info { pub name: *mut u8 }
#[repr(C)]
pub struct analog_demod_ops {
    pub info: analog_demod_info,
    pub set_params: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut analog_parameters)>,
    pub has_signal: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u16) -> i32>,
    pub get_afc: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut i32) -> i32>,
    pub tuner_status: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub standby: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub release: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub i2c_gate_ctrl: Option<unsafe extern "C" fn(*mut dvb_frontend, i32) -> i32>,
    pub set_config: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct dvb_frontend_internal_info {
    pub name: [u8; 128], pub frequency_min_hz: u32, pub frequency_max_hz: u32,
    pub frequency_stepsize_hz: u32, pub frequency_tolerance_hz: u32,
    pub symbol_rate_min: u32, pub symbol_rate_max: u32, pub symbol_rate_tolerance: u32,
    pub caps: fe_caps,
}

#[repr(C)]
pub struct dvb_frontend_ops {
    pub info: dvb_frontend_internal_info, pub delsys: [u8; MAX_DELSYS],
    pub detach: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub release: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub release_sec: Option<unsafe extern "C" fn(*mut dvb_frontend)>,
    pub init: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub sleep: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut dvb_frontend, *const u8, i32) -> i32>,
    pub tune: Option<unsafe extern "C" fn(*mut dvb_frontend, bool, u32, *mut u32, *mut fe_status) -> i32>,
    pub get_frontend_algo: Option<unsafe extern "C" fn(*mut dvb_frontend) -> dvbfe_algo>,
    pub set_frontend: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub get_tune_settings: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut dvb_frontend_tune_settings) -> i32>,
    pub get_frontend: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut dtv_frontend_properties) -> i32>,
    pub read_status: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut fe_status) -> i32>,
    pub read_ber: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub read_signal_strength: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u16) -> i32>,
    pub read_snr: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u16) -> i32>,
    pub read_ucblocks: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut u32) -> i32>,
    pub diseqc_reset_overload: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub diseqc_send_master_cmd: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut dvb_diseqc_master_cmd) -> i32>,
    pub diseqc_recv_slave_reply: Option<unsafe extern "C" fn(*mut dvb_frontend, *mut dvb_diseqc_slave_reply) -> i32>,
    pub diseqc_send_burst: Option<unsafe extern "C" fn(*mut dvb_frontend, fe_sec_mini_cmd) -> i32>,
    pub set_tone: Option<unsafe extern "C" fn(*mut dvb_frontend, fe_sec_tone_mode) -> i32>,
    pub set_voltage: Option<unsafe extern "C" fn(*mut dvb_frontend, fe_sec_voltage) -> i32>,
    pub enable_high_lnb_voltage: Option<unsafe extern "C" fn(*mut dvb_frontend, i64) -> i32>,
    pub dishnetwork_send_legacy_command: Option<unsafe extern "C" fn(*mut dvb_frontend, u64) -> i32>,
    pub i2c_gate_ctrl: Option<unsafe extern "C" fn(*mut dvb_frontend, i32) -> i32>,
    pub ts_bus_ctrl: Option<unsafe extern "C" fn(*mut dvb_frontend, i32) -> i32>,
    pub set_lna: Option<unsafe extern "C" fn(*mut dvb_frontend) -> i32>,
    pub search: Option<unsafe extern "C" fn(*mut dvb_frontend) -> dvbfe_search>,
    pub tuner_ops: dvb_tuner_ops, pub analog_ops: analog_demod_ops,
}

#[repr(C)]
pub struct dtv_frontend_properties {
    pub frequency: u32, pub modulation: fe_modulation, pub voltage: fe_sec_voltage,
    pub sectone: fe_sec_tone_mode, pub inversion: fe_spectral_inversion, pub fec_inner: fe_code_rate,
    pub transmission_mode: fe_transmit_mode, pub bandwidth_hz: u32, pub guard_interval: fe_guard_interval,
    pub hierarchy: fe_hierarchy, pub symbol_rate: u32, pub code_rate_HP: fe_code_rate, pub code_rate_LP: fe_code_rate,
    pub pilot: fe_pilot, pub rolloff: fe_rolloff, pub delivery_system: fe_delivery_system, pub interleaving: fe_interleaving,
    pub isdbt_partial_reception: u8, pub isdbt_sb_mode: u8, pub isdbt_sb_subchannel: u8,
    pub isdbt_sb_segment_idx: u32, pub isdbt_sb_segment_count: u32, pub isdbt_layer_enabled: u8,
    pub layer: [dtv_frontend_layer; 3], pub stream_id: u32, pub scrambling_sequence_index: u32,
    pub atscmh_fic_ver: u8, pub atscmh_parade_id: u8, pub atscmh_nog: u8, pub atscmh_tnog: u8,
    pub atscmh_sgn: u8, pub atscmh_prc: u8, pub atscmh_rs_frame_mode: u8, pub atscmh_rs_frame_ensemble: u8,
    pub atscmh_rs_code_mode_pri: u8, pub atscmh_rs_code_mode_sec: u8, pub atscmh_sccc_block_mode: u8,
    pub atscmh_sccc_code_mode_a: u8, pub atscmh_sccc_code_mode_b: u8, pub atscmh_sccc_code_mode_c: u8,
    pub atscmh_sccc_code_mode_d: u8, pub lna: u32,
    pub strength: dtv_fe_stats, pub cnr: dtv_fe_stats, pub pre_bit_error: dtv_fe_stats,
    pub pre_bit_count: dtv_fe_stats, pub post_bit_error: dtv_fe_stats, pub post_bit_count: dtv_fe_stats,
    pub block_error: dtv_fe_stats, pub block_count: dtv_fe_stats,
}
#[repr(C)]
pub struct dtv_frontend_layer { pub segment_count: u8, pub fec: fe_code_rate, pub modulation: fe_modulation, pub interleaving: u8 }

pub const DVB_FE_NO_EXIT: u32 = 0;
pub const DVB_FE_NORMAL_EXIT: u32 = 1;
pub const DVB_FE_DEVICE_REMOVED: u32 = 2;
pub const DVB_FE_DEVICE_RESUME: u32 = 3;
pub const DVB_FRONTEND_COMPONENT_TUNER: i32 = 0;
pub const DVB_FRONTEND_COMPONENT_DEMOD: i32 = 1;

#[repr(C)]
pub struct dvb_frontend {
    pub refcount: kref, pub ops: dvb_frontend_ops, pub dvb: *mut dvb_adapter,
    pub demodulator_priv: *mut core::ffi::c_void, pub tuner_priv: *mut core::ffi::c_void,
    pub frontend_priv: *mut core::ffi::c_void, pub sec_priv: *mut core::ffi::c_void,
    pub analog_demod_priv: *mut core::ffi::c_void, pub dtv_property_cache: dtv_frontend_properties,
    pub callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32, i32, i32) -> i32>,
    pub id: i32, pub exit: u32,
}

unsafe extern "C" {
    pub fn dvb_register_frontend(dvb: *mut dvb_adapter, fe: *mut dvb_frontend) -> i32;
    pub fn dvb_unregister_frontend(fe: *mut dvb_frontend) -> i32;
    pub fn dvb_frontend_detach(fe: *mut dvb_frontend);
    pub fn dvb_frontend_suspend(fe: *mut dvb_frontend) -> i32;
    pub fn dvb_frontend_resume(fe: *mut dvb_frontend) -> i32;
    pub fn dvb_frontend_reinitialise(fe: *mut dvb_frontend);
    pub fn dvb_frontend_sleep_until(waketime: *mut ktime_t, add_usec: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
