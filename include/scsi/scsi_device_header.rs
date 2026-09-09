/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust translation of scsi_device.h. Types supplied by included kernel headers
// remain external dependencies.

pub type BlistFlagsT = u64;
pub const SCSI_SENSE_BUFFERSIZE: usize = 96;

#[repr(C)]
pub struct ScsiModeData { pub length: u32, pub block_descriptor_length: u16, pub medium_type: u8, pub device_specific: u8, pub header_length: u8, pub longlba: u8 }

#[repr(C)] pub struct ScsiEvent { pub evt_type: ScsiDeviceEvent, pub node: ListHead }
#[repr(C)] pub struct ScsiVpd { pub rcu: RcuHead, pub len: i32, pub data: [u8; 0] }

#[repr(C)]
pub struct ScsiDevice {
    pub host: *mut ScsiHost, pub request_queue: *mut RequestQueue,
    pub siblings: ListHead, pub same_target_siblings: ListHead,
    pub budget_map: Sbitmap, pub device_blocked: AtomicT, pub restarts: AtomicT,
    pub list_lock: SpinlockT, pub starved_entry: ListHead,
    pub queue_depth: u16, pub max_queue_depth: u16, pub last_queue_full_depth: u16,
    pub last_queue_full_count: u16, pub last_queue_full_time: CULong,
    pub queue_ramp_up_period: CULong, pub last_queue_ramp_up: CULong,
    pub id: u32, pub channel: u32, pub lun: u64, pub manufacturer: u32,
    pub sector_size: CUnsigned, pub hostdata: *mut CVoid, pub type_: u8,
    pub scsi_level: i8, pub inq_periph_qual: i8, pub inquiry_mutex: Mutex,
    pub inquiry_len: u8, pub inquiry: *mut u8,
    pub vendor: [c_char; INQUIRY_VENDOR_LEN + 1], pub model: [c_char; INQUIRY_MODEL_LEN + 1],
    pub rev: [c_char; INQUIRY_REVISION_LEN + 1],
    pub vpd_pg0: *mut ScsiVpd, pub vpd_pg83: *mut ScsiVpd, pub vpd_pg80: *mut ScsiVpd,
    pub vpd_pg89: *mut ScsiVpd, pub vpd_pgb0: *mut ScsiVpd, pub vpd_pgb1: *mut ScsiVpd,
    pub vpd_pgb2: *mut ScsiVpd, pub vpd_pgb7: *mut ScsiVpd,
    pub sdev_target: *mut ScsiTarget, pub sdev_bflags: BlistFlagsT, pub eh_timeout: u32,
    pub manage_system_start_stop: u8, pub manage_runtime_start_stop: u8, pub manage_shutdown: u8,
    pub manage_restart: u8, pub force_runtime_start_on_system_start: u8, pub is_ata: u8,
    pub removable: u8, pub changed: u8, pub busy: u8, pub lockable: u8, pub locked: u8,
    pub borken: u8, pub disconnect: u8, pub soft_reset: u8, pub sdtr: u8, pub wdtr: u8,
    pub ppr: u8, pub tagged_supported: u8, pub simple_tags: u8, pub was_reset: u8,
    pub expecting_cc_ua: u8, pub use_10_for_rw: u8, pub use_10_for_ms: u8, pub set_dbd_for_ms: u8,
    pub read_before_ms: u8, pub no_report_opcodes: u8, pub no_write_same: u8, pub use_16_for_rw: u8,
    pub use_16_for_sync: u8, pub skip_ms_page_8: u8, pub skip_ms_page_3f: u8, pub skip_vpd_pages: u8,
    pub try_vpd_pages: u8, pub use_192_bytes_for_3f: u8, pub no_start_on_add: u8, pub allow_restart: u8,
    pub start_stop_pwr_cond: u8, pub no_uld_attach: u8, pub select_no_atn: u8, pub fix_capacity: u8,
    pub guess_capacity: u8, pub retry_hwerror: u8, pub last_sector_bug: u8, pub no_read_disc_info: u8,
    pub no_read_capacity_16: u8, pub try_rc_10_first: u8, pub security_supported: u8, pub is_visible: u8,
    pub wce_default_on: u8, pub no_dif: u8, pub broken_fua: u8, pub lun_in_cdb: u8,
    pub unmap_limit_for_ws: u8, pub rpm_autosuspend: u8, pub ignore_media_change: u8,
    pub silence_suspend: u8, pub no_vpd_size: u8, pub cdl_supported: u8, pub cdl_enable: u8,
    pub queue_stopped: u32, pub offline_already: bool, pub ua_new_media_ctr: AtomicT,
    pub ua_por_ctr: AtomicT, pub disk_events_disable_depth: AtomicT,
    pub supported_events: [usize; 0], pub pending_events: [usize; 0], pub event_list: ListHead,
    pub event_work: WorkStruct, pub max_device_blocked: u32, pub iorequest_cnt: AtomicT,
    pub iodone_cnt: AtomicT, pub ioerr_cnt: AtomicT, pub iotmo_cnt: AtomicT,
    pub sdev_gendev: Device, pub sdev_dev: Device, pub requeue_work: WorkStruct,
    pub handler: *mut ScsiDeviceHandler, pub handler_data: *mut CVoid,
    pub dma_drain_len: usize, pub dma_drain_buf: *mut CVoid, pub sg_timeout: u32,
    pub sg_reserved_size: u32, pub bsg_dev: *mut BsgDevice, pub access_state: u8,
    pub state_mutex: Mutex, pub sdev_state: ScsiDeviceState, pub quiesced_by: *mut TaskStruct,
    pub sdev_data: [CULong; 0],
}

pub const SCSI_DEFAULT_RAMP_UP_PERIOD: u32 = 120 * HZ;
pub const SCSI_DEFAULT_VPD_LEN: u32 = 255;
pub const SCSI_DEFAULT_DEVICE_BLOCKED: u32 = 3;

#[repr(C)] pub struct ScsiTarget { pub starget_sdev_user: *mut ScsiDevice, pub siblings: ListHead, pub devices: ListHead, pub dev: Device, pub reap_ref: Kref, pub channel: u32, pub id: u32, pub create: u8, pub single_lun: u8, pub pdt_1f_for_no_lun: u8, pub no_report_luns: u8, pub expecting_lun_change: u8, pub target_busy: AtomicT, pub target_blocked: AtomicT, pub can_queue: u32, pub max_target_blocked: u32, pub scsi_level: c_char, pub state: ScsiTargetState, pub hostdata: *mut CVoid, pub starget_data: [CULong; 0] }
pub const SCSI_DEFAULT_TARGET_BLOCKED: u32 = 3;

#[repr(i32)] pub enum ScsiDeviceState { SDEV_CREATED = 1, SDEV_RUNNING, SDEV_CANCEL, SDEV_DEL, SDEV_QUIESCE, SDEV_OFFLINE, SDEV_TRANSPORT_OFFLINE, SDEV_BLOCK, SDEV_CREATED_BLOCK }
#[repr(i32)] pub enum ScsiScanMode { SCSI_SCAN_INITIAL = 0, SCSI_SCAN_RESCAN, SCSI_SCAN_MANUAL }
#[repr(i32)] pub enum ScsiDeviceEvent { SDEV_EVT_MEDIA_CHANGE = 1, SDEV_EVT_INQUIRY_CHANGE_REPORTED, SDEV_EVT_CAPACITY_CHANGE_REPORTED, SDEV_EVT_SOFT_THRESHOLD_REACHED_REPORTED, SDEV_EVT_MODE_PARAMETER_CHANGE_REPORTED, SDEV_EVT_LUN_CHANGE_REPORTED, SDEV_EVT_ALUA_STATE_CHANGE_REPORTED, SDEV_EVT_POWER_ON_RESET_OCCURRED }
pub const SDEV_EVT_FIRST: ScsiDeviceEvent = ScsiDeviceEvent::SDEV_EVT_MEDIA_CHANGE;
pub const SDEV_EVT_LAST: ScsiDeviceEvent = ScsiDeviceEvent::SDEV_EVT_POWER_ON_RESET_OCCURRED;
pub const SDEV_EVT_MAXBITS: usize = 9;
#[repr(i32)] pub enum ScsiTargetState { STARGET_CREATED = 1, STARGET_RUNNING, STARGET_REMOVE, STARGET_CREATED_REMOVE, STARGET_DEL }

#[repr(C)] pub struct ScsiFailure { pub result: i32, pub sense: u8, pub asc: u8, pub ascq: u8, pub allowed: i8, pub retries: i8 }
#[repr(C)] pub struct ScsiFailures { pub total_allowed: i32, pub total_retries: i32, pub failure_definitions: *mut ScsiFailure }
#[repr(C)] pub struct ScsiExecArgs { pub sense: *mut u8, pub sense_len: u32, pub sshdr: *mut ScsiSenseHdr, pub req_flags: BlkMqReqFlagsT, pub scmd_flags: i32, pub resid: *mut i32, pub failures: *mut ScsiFailures }

pub const SCMD_FAILURE_RESULT_ANY: i32 = 0x7fffffff;
pub const SCMD_FAILURE_STAT_ANY: u8 = 0xff;
pub const SCMD_FAILURE_SENSE_ANY: u8 = 0xff;
pub const SCMD_FAILURE_ASC_ANY: u8 = 0xff;
pub const SCMD_FAILURE_ASCQ_ANY: u8 = 0xff;
pub const SCMD_FAILURE_NO_LIMIT: i32 = -1;

pub unsafe fn scsi_device_reprobe(sdev: *mut ScsiDevice) -> i32 { device_reprobe(&mut (*sdev).sdev_gendev) }
pub unsafe fn sdev_channel(sdev: *mut ScsiDevice) -> u32 { (*sdev).channel }
pub unsafe fn sdev_id(sdev: *mut ScsiDevice) -> u32 { (*sdev).id }
pub unsafe fn scsi_device_is_pseudo_dev(sdev: *mut ScsiDevice) -> bool { (*sdev).lun == u64::MAX }
pub unsafe fn scsi_device_online(sdev: *mut ScsiDevice) -> i32 { !matches!((*sdev).sdev_state, ScsiDeviceState::SDEV_OFFLINE | ScsiDeviceState::SDEV_TRANSPORT_OFFLINE | ScsiDeviceState::SDEV_DEL) as i32 }
pub unsafe fn scsi_device_blocked(sdev: *mut ScsiDevice) -> i32 { matches!((*sdev).sdev_state, ScsiDeviceState::SDEV_BLOCK | ScsiDeviceState::SDEV_CREATED_BLOCK) as i32 }
pub unsafe fn scsi_device_created(sdev: *mut ScsiDevice) -> i32 { matches!((*sdev).sdev_state, ScsiDeviceState::SDEV_CREATED | ScsiDeviceState::SDEV_CREATED_BLOCK) as i32 }
pub unsafe fn scsi_device_sync(sdev: *mut ScsiDevice) -> i32 { (*sdev).sdtr as i32 }
pub unsafe fn scsi_device_wide(sdev: *mut ScsiDevice) -> i32 { (*sdev).wdtr as i32 }
pub unsafe fn scsi_device_dt(sdev: *mut ScsiDevice) -> i32 { (*sdev).ppr as i32 }
pub unsafe fn scsi_device_dt_only(sdev: *mut ScsiDevice) -> i32 { if (*sdev).inquiry_len < 57 { 0 } else { ((*(*sdev).inquiry.add(56) & 0x0c) == 0x04) as i32 } }
pub unsafe fn scsi_device_ius(sdev: *mut ScsiDevice) -> i32 { if (*sdev).inquiry_len < 57 { 0 } else { (*(*sdev).inquiry.add(56) & 1) as i32 } }
pub unsafe fn scsi_device_qas(sdev: *mut ScsiDevice) -> i32 { if (*sdev).inquiry_len < 57 { 0 } else { (*(*sdev).inquiry.add(56) & 2) as i32 } }
pub unsafe fn scsi_device_enclosure(sdev: *mut ScsiDevice) -> i32 { if (*sdev).inquiry.is_null() { 1 } else { ((*(*sdev).inquiry.add(6) & (1 << 6)) != 0) as i32 } }
pub unsafe fn scsi_device_protection(sdev: *mut ScsiDevice) -> i32 { if (*sdev).no_dif != 0 { 0 } else { (((*sdev).scsi_level as i32) > SCSI_2 && (*(*sdev).inquiry.add(5) & 1) != 0) as i32 } }
pub unsafe fn scsi_device_tpgs(sdev: *mut ScsiDevice) -> i32 { if (*sdev).inquiry.is_null() { 0 } else { ((*(*sdev).inquiry.add(5) >> 4) & 3) as i32 } }
pub unsafe fn scsi_device_supports_vpd(sdev: *mut ScsiDevice) -> i32 { if (*sdev).try_vpd_pages != 0 || (((*sdev).scsi_level as i32) >= SCSI_SPC_2 && (*sdev).skip_vpd_pages == 0) { 1 } else { 0 } }
pub unsafe fn scsi_device_busy(sdev: *mut ScsiDevice) -> i32 { sbitmap_weight(&(*sdev).budget_map) }

extern "C" {
    pub fn __scsi_add_device(host: *mut ScsiHost, channel: u32, target: u32, lun: u64, hostdata: *mut CVoid) -> *mut ScsiDevice;
    pub fn scsi_add_device(host: *mut ScsiHost, channel: u32, target: u32, lun: u64) -> i32;
    pub fn scsi_register_device_handler(h: *mut ScsiDeviceHandler) -> i32;
    pub fn scsi_remove_device(sdev: *mut ScsiDevice);
    pub fn scsi_unregister_device_handler(h: *mut ScsiDeviceHandler) -> i32;
    pub fn scsi_attach_vpd(sdev: *mut ScsiDevice); pub fn scsi_cdl_check(sdev: *mut ScsiDevice);
    pub fn scsi_cdl_enable(sdev: *mut ScsiDevice, enable: bool) -> i32;
    pub fn scsi_device_get(sdev: *mut ScsiDevice) -> i32; pub fn scsi_device_put(sdev: *mut ScsiDevice);
    pub fn scsi_device_lookup(h: *mut ScsiHost, channel: u32, target: u32, lun: u64) -> *mut ScsiDevice;
    pub fn __scsi_device_lookup(h: *mut ScsiHost, channel: u32, target: u32, lun: u64) -> *mut ScsiDevice;
    pub fn scsi_device_lookup_by_target(t: *mut ScsiTarget, lun: u64) -> *mut ScsiDevice;
    pub fn __scsi_device_lookup_by_target(t: *mut ScsiTarget, lun: u64) -> *mut ScsiDevice;
    pub fn __scsi_iterate_devices(h: *mut ScsiHost, sdev: *mut ScsiDevice) -> *mut ScsiDevice;
    pub fn scsi_change_queue_depth(sdev: *mut ScsiDevice, depth: i32) -> i32;
    pub fn scsi_track_queue_full(sdev: *mut ScsiDevice, depth: i32) -> i32;
    pub fn scsi_set_medium_removal(sdev: *mut ScsiDevice, state: c_char) -> i32;
    pub fn scsi_device_set_state(sdev: *mut ScsiDevice, state: ScsiDeviceState) -> i32;
    pub fn scsi_execute_cmd(sdev: *mut ScsiDevice, cmd: *const u8, opf: BlkOpfT, buffer: *mut CVoid, bufflen: u32, timeout: i32, retries: i32, args: *const ScsiExecArgs) -> i32;
    pub fn scsi_failures_reset_retries(f: *mut ScsiFailures);
    pub fn scsi_device_quiesce(sdev: *mut ScsiDevice) -> i32; pub fn scsi_device_resume(sdev: *mut ScsiDevice);
    pub fn scsi_target_quiesce(t: *mut ScsiTarget); pub fn scsi_target_resume(t: *mut ScsiTarget);
    pub fn scsi_target_reap(t: *mut ScsiTarget); pub fn scsi_remove_target(dev: *mut Device);
    pub fn scsi_internal_device_block_nowait(sdev: *mut ScsiDevice) -> i32;
    pub fn scsi_internal_device_unblock_nowait(sdev: *mut ScsiDevice, state: ScsiDeviceState) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
