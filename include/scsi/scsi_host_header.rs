/* SPDX-License-Identifier: GPL-2.0 */
// Translated from scsi_host.h. C includes and build-time configuration are
// intentionally represented as external dependencies or conditional notes.

use core::ffi::{c_char, c_int, c_void};
pub type c_ulong = usize;

pub const SG_ALL: u32 = SG_CHUNK_SIZE;
pub const MODE_UNKNOWN: u32 = 0x00;
pub const MODE_INITIATOR: u32 = 0x01;
pub const MODE_TARGET: u32 = 0x02;

pub const SCSI_ADAPTER_RESET: c_int = 1;
pub const SCSI_FIRMWARE_RESET: c_int = 2;
pub const SCSI_DEFAULT_MAX_SECTORS: u32 = 1024;
pub const SCSI_DEFAULT_HOST_BLOCKED: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_timeout_action { SCSI_EH_DONE, SCSI_EH_RESET_TIMER, SCSI_EH_NOT_HANDLED }

#[repr(C)]
pub struct scsi_host_template {
    pub cmd_size: u32,
    pub queuecommand: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut scsi_cmnd) -> scsi_qc_status>,
    pub queue_reserved_command: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut scsi_cmnd) -> scsi_qc_status>,
    pub commit_rqs: Option<unsafe extern "C" fn(*mut Scsi_Host, u16)>,
    pub module: *mut module,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut Scsi_Host) -> *const c_char>,
    pub ioctl: Option<unsafe extern "C" fn(*mut scsi_device, u32, *mut c_void) -> c_int>,
    // CONFIG_COMPAT: compat_ioctl is present only in compatible builds.
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut scsi_device, u32, *mut c_void) -> c_int>,
    pub init_cmd_priv: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut scsi_cmnd) -> c_int>,
    pub exit_cmd_priv: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut scsi_cmnd) -> c_int>,
    pub eh_abort_handler: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub eh_device_reset_handler: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub eh_target_reset_handler: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub eh_bus_reset_handler: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub eh_host_reset_handler: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> c_int>,
    pub sdev_init: Option<unsafe extern "C" fn(*mut scsi_device) -> c_int>,
    pub sdev_configure: Option<unsafe extern "C" fn(*mut scsi_device, *mut queue_limits) -> c_int>,
    pub sdev_destroy: Option<unsafe extern "C" fn(*mut scsi_device)>,
    pub target_alloc: Option<unsafe extern "C" fn(*mut scsi_target) -> c_int>,
    pub target_destroy: Option<unsafe extern "C" fn(*mut scsi_target)>,
    pub scan_finished: Option<unsafe extern "C" fn(*mut Scsi_Host, c_ulong) -> c_int>,
    pub scan_start: Option<unsafe extern "C" fn(*mut Scsi_Host)>,
    pub change_queue_depth: Option<unsafe extern "C" fn(*mut scsi_device, c_int) -> c_int>,
    pub map_queues: Option<unsafe extern "C" fn(*mut Scsi_Host)>,
    pub mq_poll: Option<unsafe extern "C" fn(*mut Scsi_Host, u32) -> c_int>,
    pub dma_need_drain: Option<unsafe extern "C" fn(*mut request) -> bool>,
    pub bios_param: Option<unsafe extern "C" fn(*mut scsi_device, *mut gendisk, sector_t, *mut c_int) -> c_int>,
    pub unlock_native_capacity: Option<unsafe extern "C" fn(*mut scsi_device)>,
    pub show_info: Option<unsafe extern "C" fn(*mut seq_file, *mut Scsi_Host) -> c_int>,
    pub write_info: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut c_char, c_int) -> c_int>,
    pub eh_timed_out: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> scsi_timeout_action>,
    pub eh_should_retry_cmd: Option<unsafe extern "C" fn(*mut scsi_cmnd) -> bool>,
    pub host_reset: Option<unsafe extern "C" fn(*mut Scsi_Host, c_int) -> c_int>,
    pub proc_name: *const c_char,
    pub can_queue: c_int,
    pub nr_reserved_cmds: c_int,
    pub this_id: c_int,
    pub sg_tablesize: u16,
    pub sg_prot_tablesize: u16,
    pub max_sectors: u32,
    pub max_segment_size: u32,
    pub dma_alignment: u32,
    pub dma_boundary: c_ulong,
    pub virt_boundary_mask: c_ulong,
    pub cmd_per_lun: i16,
    pub tag_alloc_policy_rr: bool,
    pub track_queue_depth: u8,
    pub supported_mode: u8,
    pub emulated: u8,
    pub skip_settle_delay: u8,
    pub no_write_same: u8,
    pub host_tagset: u8,
    pub queuecommand_may_block: u8,
    pub max_host_blocked: u32,
    pub shost_groups: *const *const attribute_group,
    pub sdev_groups: *const *const attribute_group,
    pub vendor_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_host_state { SHOST_CREATED = 1, SHOST_RUNNING, SHOST_CANCEL, SHOST_DEL, SHOST_RECOVERY, SHOST_CANCEL_RECOVERY, SHOST_DEL_RECOVERY }

#[repr(C)]
pub struct Scsi_Host {
    pub __devices: list_head, pub __targets: list_head, pub starved_list: list_head,
    pub default_lock: spinlock_t, pub host_lock: *mut spinlock_t, pub scan_mutex: mutex,
    pub eh_abort_list: list_head, pub eh_cmd_q: list_head, pub ehandler: *mut task_struct,
    pub eh_action: *mut completion, pub host_wait: wait_queue_head_t,
    pub hostt: *const scsi_host_template, pub transportt: *mut scsi_transport_template,
    pub tagset_refcnt: kref, pub tagset_freed: completion, pub tag_set: blk_mq_tag_set,
    pub host_blocked: atomic_t, pub host_failed: u32, pub host_eh_scheduled: u32, pub host_no: u32,
    pub eh_deadline: c_int, pub last_reset: c_ulong, pub max_channel: u32, pub max_id: u32, pub max_lun: u64,
    pub unique_id: u32, pub max_cmd_len: u16, pub this_id: c_int, pub can_queue: c_int,
    pub nr_reserved_cmds: u32, pub cmd_per_lun: i16, pub sg_tablesize: u16, pub sg_prot_tablesize: u16,
    pub max_sectors: u32, pub opt_sectors: u32, pub max_segment_size: u32, pub dma_alignment: u32,
    pub dma_boundary: c_ulong, pub virt_boundary_mask: c_ulong, pub nr_hw_queues: u32, pub nr_maps: u32,
    pub async_scan: bool, pub eh_noresume: bool, pub active_mode: u8, pub host_self_blocked: u8,
    pub reverse_ordering: u8, pub tmf_in_progress: u8, pub no_write_same: u8, pub host_tagset: u8,
    pub queuecommand_may_block: u8, pub short_inquiry: u8, pub no_scsi2_lun_in_cdb: u8,
    pub work_q: *mut workqueue_struct, pub tmf_work_q: *mut workqueue_struct, pub max_host_blocked: u32,
    pub prot_capabilities: u32, pub prot_guard_type: u8, pub base: c_ulong, pub io_port: c_ulong,
    pub n_io_port: u8, pub dma_channel: u8, pub irq: u32, pub shost_state: scsi_host_state,
    pub shost_gendev: device, pub shost_dev: device, pub pseudo_sdev: *mut scsi_device,
    pub shost_data: *mut c_void, pub dma_dev: *mut device, pub eh_work: work_struct,
    pub rpm_autosuspend_delay: c_int, pub hostdata: [c_ulong; 0],
}

#[inline]
pub unsafe fn shost_priv(shost: *mut Scsi_Host) -> *mut c_void { (*shost).hostdata.as_mut_ptr() as *mut c_void }

#[inline]
pub unsafe fn scsi_host_in_recovery(shost: *mut Scsi_Host) -> bool {
    let state = (*shost).shost_state;
    state == scsi_host_state::SHOST_RECOVERY || state == scsi_host_state::SHOST_CANCEL_RECOVERY || state == scsi_host_state::SHOST_DEL_RECOVERY || (*shost).tmf_in_progress != 0
}

extern "C" {
    pub fn scsi_is_host_device(dev: *const device) -> c_int;
    pub fn scsi_queue_work(host: *mut Scsi_Host, work: *mut work_struct) -> c_int;
    pub fn scsi_flush_work(host: *mut Scsi_Host);
    pub fn scsi_host_alloc(t: *const scsi_host_template, privsize: c_int) -> *mut Scsi_Host;
    pub fn scsi_add_host_with_dma(host: *mut Scsi_Host, dev: *mut device, dma_dev: *mut device) -> c_int;
    pub fn scsi_scan_host(host: *mut Scsi_Host);
    pub fn scsi_resume_device(sdev: *mut scsi_device) -> c_int;
    pub fn scsi_rescan_device(sdev: *mut scsi_device) -> c_int;
    pub fn scsi_remove_host(host: *mut Scsi_Host);
    pub fn scsi_host_get(host: *mut Scsi_Host) -> *mut Scsi_Host;
    pub fn scsi_host_busy(host: *mut Scsi_Host) -> c_int;
    pub fn scsi_host_put(host: *mut Scsi_Host);
    pub fn scsi_host_lookup(hostnum: u32) -> *mut Scsi_Host;
    pub fn scsi_host_state_name(state: scsi_host_state) -> *const c_char;
    pub fn scsi_unblock_requests(host: *mut Scsi_Host);
    pub fn scsi_block_requests(host: *mut Scsi_Host);
    pub fn scsi_host_block(host: *mut Scsi_Host) -> c_int;
    pub fn scsi_host_unblock(host: *mut Scsi_Host, new_state: c_int) -> c_int;
    pub fn scsi_host_set_state(host: *mut Scsi_Host, state: scsi_host_state) -> c_int;
    pub fn scsi_host_complete_all_commands(host: *mut Scsi_Host, status: scsi_host_status);
}

#[inline]
pub unsafe fn scsi_add_host(host: *mut Scsi_Host, dev: *mut device) -> c_int {
    scsi_add_host_with_dma(host, dev, dev)
}

#[inline]
pub unsafe fn scsi_get_device(host: *mut Scsi_Host) -> *mut device { (*host).shost_gendev.parent }

#[inline]
pub unsafe fn scsi_host_scan_allowed(host: *mut Scsi_Host) -> bool {
    let state = (*host).shost_state;
    state == scsi_host_state::SHOST_RUNNING || state == scsi_host_state::SHOST_RECOVERY
}

pub type scsi_host_busy_fn = unsafe extern "C" fn(*mut scsi_cmnd, *mut c_void) -> bool;
extern "C" {
    pub fn scsi_host_busy_iter(host: *mut Scsi_Host, f: Option<scsi_host_busy_fn>, priv_: *mut c_void);
}

#[repr(C)]
pub enum scsi_host_prot_capabilities { SHOST_DIF_TYPE1_PROTECTION = 1, SHOST_DIF_TYPE2_PROTECTION = 2, SHOST_DIF_TYPE3_PROTECTION = 4, SHOST_DIX_TYPE0_PROTECTION = 8, SHOST_DIX_TYPE1_PROTECTION = 16, SHOST_DIX_TYPE2_PROTECTION = 32, SHOST_DIX_TYPE3_PROTECTION = 64 }
#[repr(C)]
pub enum scsi_host_guard_type { SHOST_DIX_GUARD_CRC = 1, SHOST_DIX_GUARD_IP = 2 }

#[inline] pub unsafe fn scsi_host_set_prot(h: *mut Scsi_Host, mask: u32) { (*h).prot_capabilities = mask; }
#[inline] pub unsafe fn scsi_host_get_prot(h: *mut Scsi_Host) -> u32 { (*h).prot_capabilities }
#[inline] pub unsafe fn scsi_host_prot_dma(h: *mut Scsi_Host) -> c_int { ((*h).prot_capabilities >= 8) as c_int }
#[inline] pub unsafe fn scsi_host_set_guard(h: *mut Scsi_Host, ty: u8) { (*h).prot_guard_type = ty; }
#[inline] pub unsafe fn scsi_host_get_guard(h: *mut Scsi_Host) -> u8 { (*h).prot_guard_type }

#[inline]
pub unsafe fn scsi_host_dif_capable(h: *mut Scsi_Host, target_type: usize) -> u32 {
    let cap = [0, 1, 2, 4];
    if target_type >= cap.len() { return 0; }
    if (*h).prot_capabilities & cap[target_type] != 0 { target_type as u32 } else { 0 }
}

#[inline]
pub unsafe fn scsi_host_dix_capable(h: *mut Scsi_Host, target_type: usize) -> u32 {
    let cap = [8, 16, 32, 64];
    if target_type >= cap.len() { return 0; }
    (*h).prot_capabilities & cap[target_type]
}

// External C types and constants supplied by the included kernel headers.
extern "C" { static SG_CHUNK_SIZE: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
