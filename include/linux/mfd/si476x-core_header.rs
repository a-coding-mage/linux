/* SPDX-License-Identifier: GPL-2.0-only */
/* Common definitions for si476x core device. */

// Linux headers and si476x-platform/reports dependencies are supplied by the
// surrounding translation unit.

pub const SI476X_DEFAULT_TIMEOUT: u32 = 100000;
pub const SI476X_TIMEOUT_TUNE: u32 = 700000;
pub const SI476X_TIMEOUT_POWER_UP: u32 = 330000;
pub const SI476X_STATUS_POLL_US: u32 = 0;

#[repr(C)]
pub struct si476x_core {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub chip_id: i32,
    pub cells: [mfd_cell; SI476X_MFD_CELLS as usize],
    pub cmd_lock: mutex,
    pub users: atomic_t,
    pub rds_read_queue: wait_queue_head_t,
    pub rds_fifo: kfifo,
    pub rds_fifo_drainer: work_struct,
    pub rds_drainer_is_working: bool,
    pub rds_drainer_status_lock: mutex,
    pub command: wait_queue_head_t,
    pub cts: atomic_t,
    pub tuning: wait_queue_head_t,
    pub stc: atomic_t,
    pub power_up_parameters: si476x_power_up_args,
    pub power_state: si476x_power_state,
    pub supplies: [regulator_bulk_data; 4],
    pub reset: *mut gpio_desc,
    pub pinmux: si476x_pinmux,
    pub diversity_mode: si476x_phase_diversity_mode,
    pub is_alive: atomic_t,
    pub status_monitor: delayed_work,
    pub revision: i32,
    pub rds_fifo_depth: i32,
}

pub const SI476X_WORK_TO_CORE: Option<unsafe fn(*mut work_struct) -> *mut si476x_core> = None;

#[repr(i32)] pub enum si476x_freq_supported_chips { SI476X_CHIP_SI4761 = 1, SI476X_CHIP_SI4764, SI476X_CHIP_SI4768 }
#[repr(i32)] pub enum si476x_part_revisions { SI476X_REVISION_A10 = 0, SI476X_REVISION_A20 = 1, SI476X_REVISION_A30 = 2 }
pub const SI476X_RADIO_CELL: i32 = 0;
pub const SI476X_CODEC_CELL: i32 = 1;
pub const SI476X_MFD_CELLS: i32 = 2;
#[repr(i32)] pub enum si476x_power_state { SI476X_POWER_DOWN = 0, SI476X_POWER_UP_FULL = 1, SI476X_POWER_INCONSISTENT = 2 }

#[inline] pub unsafe fn i2c_mfd_cell_to_core(dev: *mut device) -> *mut si476x_core { i2c_get_clientdata(to_i2c_client((*dev).parent)) as *mut si476x_core }
#[inline] pub unsafe fn si476x_core_lock(core: *mut si476x_core) { mutex_lock(&mut (*core).cmd_lock); }
#[inline] pub unsafe fn si476x_core_unlock(core: *mut si476x_core) { mutex_unlock(&mut (*core).cmd_lock); }

#[inline]
pub unsafe fn hz_to_si476x(core: *mut si476x_core, freq: i32) -> u16 {
    match (*core).power_up_parameters.func { SI476X_FUNC_AM_RECEIVER => (freq / 1000) as u16, _ => (freq / 10000) as u16 }
}
#[inline]
pub unsafe fn si476x_to_hz(core: *mut si476x_core, freq: u16) -> i32 {
    match (*core).power_up_parameters.func { SI476X_FUNC_AM_RECEIVER => freq as i32 * 1000, _ => freq as i32 * 10000 }
}
#[inline] pub fn hz_to_v4l2(freq: i32) -> i32 { (freq * 10) / 625 }
#[inline] pub fn v4l2_to_hz(freq: i32) -> i32 { (freq * 625) / 10 }
#[inline] pub unsafe fn v4l2_to_si476x(core: *mut si476x_core, freq: i32) -> u16 { hz_to_si476x(core, v4l2_to_hz(freq)) }
#[inline] pub unsafe fn si476x_to_v4l2(core: *mut si476x_core, freq: u16) -> i32 { hz_to_v4l2(si476x_to_hz(core, freq)) }

#[repr(C)] pub struct si476x_func_info { pub firmware: si476x_firmware, pub patch_id: u16, pub func: si476x_func }
#[repr(C)] pub struct si476x_firmware { pub major: u8, pub minor: [u8; 2] }
#[repr(C)] pub struct si476x_power_down_args { pub xosc: bool }
#[repr(i32)] pub enum si476x_tunemode { SI476X_TM_VALIDATED_NORMAL_TUNE = 0, SI476X_TM_INVALIDATED_FAST_TUNE = 1, SI476X_TM_VALIDATED_AF_TUNE = 2, SI476X_TM_VALIDATED_AF_CHECK = 3 }
#[repr(i32)] pub enum si476x_smoothmetrics { SI476X_SM_INITIALIZE_AUDIO = 0, SI476X_SM_TRANSITION_AUDIO = 1 }
#[repr(C)] pub struct si476x_rds_status_report { pub rdstpptyint: bool, pub rdspiint: bool, pub rdssyncint: bool, pub rdsfifoint: bool, pub tpptyvalid: bool, pub pivalid: bool, pub rdssync: bool, pub rdsfifolost: bool, pub tp: bool, pub pty: u8, pub pi: u16, pub rdsfifoused: u8, pub ble: [u8; 4], pub rds: [v4l2_rds_data; 4] }
#[repr(C)] pub struct si476x_rsq_status_args { pub primary: bool, pub rsqack: bool, pub attune: bool, pub cancel: bool, pub stcack: bool }
#[repr(i32)] pub enum si476x_injside { SI476X_INJSIDE_AUTO = 0, SI476X_INJSIDE_LOW = 1, SI476X_INJSIDE_HIGH = 2 }
#[repr(C)] pub struct si476x_tune_freq_args { pub zifsr: bool, pub hd: bool, pub injside: si476x_injside, pub freq: i32, pub tunemode: si476x_tunemode, pub smoothmetrics: si476x_smoothmetrics, pub antcap: i32 }

#[repr(i32)] pub enum si476x_i2c_type { SI476X_I2C_SEND, SI476X_I2C_RECV }

#[repr(i32)] pub enum si476x_power_grid_type { SI476X_POWER_GRID_50HZ = 0, SI476X_POWER_GRID_60HZ }
#[repr(i32)] pub enum si476x_interrupt_flags { SI476X_STCIEN = 1, SI476X_ACFIEN = 2, SI476X_RDSIEN = 4, SI476X_RSQIEN = 8, SI476X_ERRIEN = 64, SI476X_CTSIEN = 128, SI476X_STCREP = 256, SI476X_ACFREP = 512, SI476X_RDSREP = 1024, SI476X_RSQREP = 2048 }
#[repr(i32)] pub enum si476x_rdsint_sources { SI476X_RDSTPPTY = 16, SI476X_RDSPI = 8, SI476X_RDSSYNC = 2, SI476X_RDSRECV = 1 }
#[repr(i32)] pub enum si476x_status_response_bits { SI476X_CTS = 128, SI476X_ERR = 64, SI476X_WB_ASQ_INT = 16, SI476X_RSQ_INT = 8, SI476X_FM_RDS_INT = 4, SI476X_ACF_INT = 2, SI476X_STC_INT = 1 }

pub const SI476X_PROP_INT_CTL_ENABLE: u16 = 0x0000;
pub const SI476X_PROP_DIGITAL_IO_INPUT_SAMPLE_RATE: u16 = 0x0200;
pub const SI476X_PROP_DIGITAL_IO_INPUT_FORMAT: u16 = 0x0201;
pub const SI476X_PROP_DIGITAL_IO_OUTPUT_SAMPLE_RATE: u16 = 0x0202;
pub const SI476X_PROP_DIGITAL_IO_OUTPUT_FORMAT: u16 = 0x0203;
pub const SI476X_PROP_SEEK_BAND_BOTTOM: u16 = 0x1100;
pub const SI476X_PROP_SEEK_BAND_TOP: u16 = 0x1101;
pub const SI476X_PROP_SEEK_FREQUENCY_SPACING: u16 = 0x1102;
pub const SI476X_PROP_VALID_MAX_TUNE_ERROR: u16 = 0x2000;
pub const SI476X_PROP_VALID_SNR_THRESHOLD: u16 = 0x2003;
pub const SI476X_PROP_VALID_RSSI_THRESHOLD: u16 = 0x2004;
pub const SI476X_PROP_AUDIO_PWR_LINE_FILTER: u16 = 0x0303;
pub const SI476X_PROP_AUDIO_DEEMPHASIS: u16 = 0x0302;
pub const SI476X_PROP_FM_RDS_INTERRUPT_SOURCE: u16 = 0x4000;
pub const SI476X_PROP_FM_RDS_INTERRUPT_FIFO_COUNT: u16 = 0x4001;
pub const SI476X_PROP_FM_RDS_CONFIG: u16 = 0x4002;
pub const SI476X_PROP_PWR_HARMONICS_MASK: u16 = 0x001f;
pub const SI476X_PROP_PWR_GRID_MASK: u16 = 0x0100;
pub const SI476X_PROP_PWR_ENABLE_MASK: u16 = 0x0200;
pub const SI476X_PROP_PWR_GRID_50HZ: u16 = 0;
pub const SI476X_PROP_PWR_GRID_60HZ: u16 = 0x0100;
pub const SI476X_PROP_RDSEN_MASK: u16 = 1;
pub const SI476X_PROP_RDSEN: u16 = 1;

extern "C" {
    pub fn si476x_core_stop(core: *mut si476x_core, soft: bool) -> i32;
    pub fn si476x_core_start(core: *mut si476x_core, soft: bool) -> i32;
    pub fn si476x_core_set_power_state(core: *mut si476x_core, state: si476x_power_state) -> i32;
    pub fn si476x_core_has_am(core: *mut si476x_core) -> bool;
    pub fn si476x_core_has_diversity(core: *mut si476x_core) -> bool;
    pub fn si476x_core_is_a_secondary_tuner(core: *mut si476x_core) -> bool;
    pub fn si476x_core_is_a_primary_tuner(core: *mut si476x_core) -> bool;
    pub fn si476x_core_is_in_am_receiver_mode(core: *mut si476x_core) -> bool;
    pub fn si476x_core_is_powered_up(core: *mut si476x_core) -> bool;
    pub fn si476x_core_i2c_xfer(core: *mut si476x_core, ty: si476x_i2c_type, buf: *mut i8, len: i32) -> i32;
    pub fn si476x_core_cmd_func_info(core: *mut si476x_core, info: *mut si476x_func_info) -> i32;
    pub fn si476x_core_cmd_set_property(core: *mut si476x_core, prop: u16, value: u16) -> i32;
    pub fn si476x_core_cmd_get_property(core: *mut si476x_core, prop: u16) -> i32;
    pub fn si476x_core_cmd_fm_seek_start(core: *mut si476x_core, up: bool, wrap: bool) -> i32;
    pub fn si476x_core_cmd_am_seek_start(core: *mut si476x_core, up: bool, wrap: bool) -> i32;
    pub fn si476x_core_cmd_fm_rds_status(core: *mut si476x_core, intack: bool, mt: bool, si: bool, report: *mut si476x_rds_status_report) -> i32;
    pub fn si476x_core_cmd_fm_tune_freq(core: *mut si476x_core, args: *mut si476x_tune_freq_args) -> i32;
    pub fn si476x_core_cmd_am_tune_freq(core: *mut si476x_core, args: *mut si476x_tune_freq_args) -> i32;
    pub fn si476x_core_cmd_power_up(core: *mut si476x_core, args: *mut si476x_power_up_args) -> i32;
    pub fn si476x_core_cmd_power_down(core: *mut si476x_core, args: *mut si476x_power_down_args) -> i32;
    pub fn si476x_core_cmd_fm_phase_div_status(core: *mut si476x_core) -> i32;
    pub fn si476x_core_cmd_fm_phase_diversity(core: *mut si476x_core, mode: si476x_phase_diversity_mode) -> i32;
    pub fn devm_regmap_init_si476x(core: *mut si476x_core) -> *mut regmap;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
