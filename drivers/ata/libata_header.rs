/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of libata.h. */

pub const DRV_NAME: &str = "libata";
pub const DRV_VERSION: &str = "3.00";

pub const ATA_READID_POSTRESET: u32 = 1 << 0;
pub const ATA_DNXFER_PIO: u32 = 0;
pub const ATA_DNXFER_DMA: u32 = 1;
pub const ATA_DNXFER_40C: u32 = 2;
pub const ATA_DNXFER_FORCE_PIO: u32 = 3;
pub const ATA_DNXFER_FORCE_PIO0: u32 = 4;
pub const ATA_DNXFER_QUIET: u32 = 1 << 31;
pub const ATA_PORT_TYPE_NAME: &str = "ata_port";

extern "C" {
    pub static mut atapi_passthru16: ::core::ffi::c_int;
    pub static mut libata_fua: ::core::ffi::c_int;
    pub static mut libata_noacpi: ::core::ffi::c_int;
    pub static mut libata_allow_tpm: ::core::ffi::c_int;
    pub static ata_port_type: device_type;
    pub fn ata_dev_phys_link(dev: *mut ata_device) -> *mut ata_link;
    pub fn ata_id_zoned_cap(id: *const u16) -> u16;
    pub fn container_of<T, U>(ptr: *mut T, member: *mut U) -> *mut ata_port;
}

#[repr(C)] pub struct device_type { _private: [u8; 0] }
#[repr(C)] pub struct ata_device { pub class: u32, pub id: *mut u16, pub zpodd: *mut ::core::ffi::c_void }
#[repr(C)] pub struct ata_port { pub pflags: u32, _private: [u8; 0] }
#[repr(C)] pub struct ata_link { _private: [u8; 0] }
#[repr(C)] pub struct ata_taskfile { _private: [u8; 0] }
#[repr(C)] pub struct ata_queued_cmd { _private: [u8; 0] }
#[repr(C)] pub struct ata_host { _private: [u8; 0] }
#[repr(C)] pub struct scsi_device { _private: [u8; 0] }
#[repr(C)] pub struct scsi_cmnd { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct Scsi_Host { _private: [u8; 0] }
#[repr(C)] pub struct scsi_host_template { _private: [u8; 0] }
#[repr(C)] pub struct queue_limits { _private: [u8; 0] }
#[repr(C)] pub struct ata_reset_operations { _private: [u8; 0] }
#[repr(C)] pub struct ata_ering { _private: [u8; 0] }
#[repr(C)] pub struct ata_ering_entry { _private: [u8; 0] }
#[repr(C)] pub struct pm_message_t { _private: [u8; 0] }

pub type dma_data_direction = ::core::ffi::c_int;
pub type ata_lpm_policy = ::core::ffi::c_int;
pub type scsi_qc_status = ::core::ffi::c_int;
pub type scsi_timeout_action = ::core::ffi::c_int;
pub type scsi_disposition = ::core::ffi::c_int;
pub type acpi_handle = *mut ::core::ffi::c_void;

pub const ATA_SCSI_RBUF_SIZE: usize = 2048;
pub const ATA_DEV_MAX_CPR: usize = 255.min((ATA_SCSI_RBUF_SIZE - 64) / 32);

extern "C" {
    pub fn ata_sstatus_online(sstatus: u32) -> bool;
    pub fn ata_dev_is_zoned(dev: *mut ata_device) -> bool;
    pub fn ata_port_eh_scheduled(ap: *mut ata_port) -> bool;
    pub fn ata_force_cbl(ap: *mut ata_port);
    pub fn ata_tf_to_lba(tf: *const ata_taskfile) -> u64;
    pub fn ata_tf_to_lba48(tf: *const ata_taskfile) -> u64;
    pub fn ata_build_rw_tf(qc: *mut ata_queued_cmd, block: u64, n_block: u32, tf_flags: u32, dld: ::core::ffi::c_int, class: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ata_tf_read_block(tf: *const ata_taskfile, dev: *mut ata_device) -> u64;
    pub fn ata_exec_internal(dev: *mut ata_device, tf: *mut ata_taskfile, cdb: *const u8, dma_dir: dma_data_direction, buf: *mut ::core::ffi::c_void, buflen: u32, timeout: u32) -> u32;
    pub fn ata_wait_ready(link: *mut ata_link, deadline: ::core::ffi::c_ulong, check_ready: Option<unsafe extern "C" fn(*mut ata_link) -> ::core::ffi::c_int>) -> ::core::ffi::c_int;
    pub fn ata_dev_read_id(dev: *mut ata_device, p_class: *mut u32, flags: u32, id: *mut u16) -> ::core::ffi::c_int;
    pub fn ata_dev_reread_id(dev: *mut ata_device, readid_flags: u32) -> ::core::ffi::c_int;
    pub fn ata_dev_revalidate(dev: *mut ata_device, new_class: u32, readid_flags: u32) -> ::core::ffi::c_int;
    pub fn ata_dev_configure(dev: *mut ata_device) -> ::core::ffi::c_int;
    pub fn ata_dev_power_init_tf(dev: *mut ata_device, tf: *mut ata_taskfile, set_active: bool) -> bool;
    pub fn ata_dev_power_set_standby(dev: *mut ata_device);
    pub fn ata_dev_power_set_active(dev: *mut ata_device);
    pub fn ata_dev_free_resources(dev: *mut ata_device);
    pub fn ata_down_xfermask_limit(dev: *mut ata_device, sel: u32) -> ::core::ffi::c_int;
    pub fn ata_dev_set_feature(dev: *mut ata_device, subcmd: u8, action: u8) -> u32;
    pub fn ata_qc_free(qc: *mut ata_queued_cmd);
    pub fn ata_qc_issue(ap: *mut ata_port, qc: *mut ata_queued_cmd);
    pub fn __ata_qc_complete(qc: *mut ata_queued_cmd);
    pub fn atapi_check_dma(qc: *mut ata_queued_cmd) -> ::core::ffi::c_int;
    pub fn swap_buf_le16(buf: *mut u16, buf_words: u32);
    pub fn ata_phys_link_online(link: *mut ata_link) -> bool;
    pub fn ata_phys_link_offline(link: *mut ata_link) -> bool;
    pub fn ata_adapter_is_online(ap: *mut ata_port) -> bool;
    pub fn ata_dev_init(dev: *mut ata_device);
    pub fn ata_link_init(ap: *mut ata_port, link: *mut ata_link, pmp: ::core::ffi::c_int);
    pub fn sata_link_init_spd(link: *mut ata_link) -> ::core::ffi::c_int;
    pub fn ata_task_ioctl(scsidev: *mut scsi_device, arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn ata_cmd_ioctl(scsidev: *mut scsi_device, arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn sata_spd_string(spd: u32) -> *const ::core::ffi::c_char;
    pub fn ata_read_log_page(dev: *mut ata_device, log: u8, page: u8, buf: *mut ::core::ffi::c_void, sectors: u32) -> u32;
}

#[inline] pub unsafe fn ata_sstatus_online_inline(sstatus: u32) -> bool { (sstatus & 0xf) == 0x3 }
#[inline] pub unsafe fn ata_dev_is_zoned_inline(dev: *mut ata_device) -> bool { (*dev).class == 0x21 || ata_id_zoned_cap((*dev).id) == 1 }
#[inline] pub unsafe fn ata_port_eh_scheduled_inline(ap: *mut ata_port) -> bool { (*ap).pflags & (1 << 6 | 1 << 7) != 0 }

// Configuration-dependent declarations and fallback implementations.
#[cfg(not(feature = "config_ata_force"))] #[inline] pub unsafe fn ata_force_cbl_fallback(_: *mut ata_port) {}

extern "C" {
    pub fn sata_down_spd_limit(link: *mut ata_link, spd_limit: u32) -> ::core::ffi::c_int;
    pub fn ata_eh_get_ncq_success_sense(link: *mut ata_link) -> ::core::ffi::c_int;
    pub fn ata_scsi_find_dev(ap: *mut ata_port, scsidev: *const scsi_device) -> *mut ata_device;
    pub fn ata_scsi_add_hosts(host: *mut ata_host, sht: *const scsi_host_template) -> ::core::ffi::c_int;
    pub fn ata_scsi_scan_host(ap: *mut ata_port, sync: ::core::ffi::c_int);
    pub fn ata_scsi_offline_dev(dev: *mut ata_device) -> bool;
    pub fn ata_scsi_sense_is_valid(sk: u8, asc: u8, ascq: u8) -> bool;
    pub fn ata_scsi_set_sense(dev: *mut ata_device, cmd: *mut scsi_cmnd, sk: u8, asc: u8, ascq: u8);
    pub fn ata_scsi_media_change_notify(dev: *mut ata_device);
    pub fn ata_scsi_hotplug(work: *mut work_struct);
    pub fn ata_scsi_dev_rescan(work: *mut work_struct);
    pub fn ata_scsi_user_scan(shost: *mut Scsi_Host, channel: u32, id: u32, lun: u64) -> ::core::ffi::c_int;
    pub fn ata_scsi_sdev_config(sdev: *mut scsi_device);
    pub fn ata_scsi_dev_config(sdev: *mut scsi_device, lim: *mut queue_limits, dev: *mut ata_device) -> ::core::ffi::c_int;
    pub fn __ata_scsi_queuecmd(scmd: *mut scsi_cmnd, dev: *mut ata_device, ap: *mut ata_port) -> scsi_qc_status;
    pub fn ata_scsi_deferred_qc_work(work: *mut work_struct);
    pub fn ata_scsi_requeue_deferred_qc(ap: *mut ata_port, scmd: *mut scsi_cmnd) -> scsi_timeout_action;
    pub fn ata_internal_cmd_timeout(dev: *mut ata_device, cmd: u8) -> u32;
    pub fn ata_internal_cmd_timed_out(dev: *mut ata_device, cmd: u8);
    pub fn ata_eh_acquire(ap: *mut ata_port);
    pub fn ata_eh_release(ap: *mut ata_port);
    pub fn ata_scsi_error(host: *mut Scsi_Host);
    pub fn ata_eh_fastdrain_timerfn(t: *mut timer_list);
    pub fn ata_qc_schedule_eh(qc: *mut ata_queued_cmd);
    pub fn ata_dev_disable(dev: *mut ata_device);
    pub fn ata_eh_detach_dev(dev: *mut ata_device);
    pub fn ata_eh_about_to_do(link: *mut ata_link, dev: *mut ata_device, action: u32);
    pub fn ata_eh_done(link: *mut ata_link, dev: *mut ata_device, action: u32);
    pub fn ata_eh_autopsy(ap: *mut ata_port);
    pub fn ata_get_cmd_name(command: u8) -> *const ::core::ffi::c_char;
    pub fn ata_eh_report(ap: *mut ata_port);
    pub fn ata_eh_reset(ap: *mut ata_port, link: *mut ata_link, classify: ::core::ffi::c_int, reset_ops: *mut ata_reset_operations) -> ::core::ffi::c_int;
    pub fn ata_eh_recover(ap: *mut ata_port, reset_ops: *mut ata_reset_operations, r_failed_disk: *mut *mut ata_link) -> ::core::ffi::c_int;
    pub fn ata_eh_finish(ap: *mut ata_port);
    pub fn ata_ering_map(ering: *mut ata_ering, map_fn: Option<unsafe extern "C" fn(*mut ata_ering_entry, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn ata_eh_decide_disposition(qc: *mut ata_queued_cmd) -> scsi_disposition;
    pub fn atapi_eh_tur(dev: *mut ata_device, r_sense_key: *mut u8) -> u32;
    pub fn atapi_eh_request_sense(dev: *mut ata_device, sense_buf: *mut u8, dfl_sense_key: u8) -> u32;
    pub fn sata_pmp_scr_read(link: *mut ata_link, reg: ::core::ffi::c_int, val: *mut u32) -> ::core::ffi::c_int;
    pub fn sata_pmp_scr_write(link: *mut ata_link, reg: ::core::ffi::c_int, val: u32) -> ::core::ffi::c_int;
    pub fn sata_pmp_set_lpm(link: *mut ata_link, policy: ata_lpm_policy, hints: u32) -> ::core::ffi::c_int;
    pub fn sata_pmp_attach(dev: *mut ata_device) -> ::core::ffi::c_int;
    pub fn ata_sff_flush_pio_task(ap: *mut ata_port);
    pub fn ata_sff_port_init(ap: *mut ata_port);
    pub fn ata_sff_init() -> ::core::ffi::c_int;
    pub fn ata_sff_exit();
    pub fn zpodd_init(dev: *mut ata_device);
    pub fn zpodd_exit(dev: *mut ata_device);
    pub fn zpodd_on_suspend(dev: *mut ata_device);
    pub fn zpodd_zpready(dev: *mut ata_device) -> bool;
    pub fn zpodd_enable_run_wake(dev: *mut ata_device);
    pub fn zpodd_disable_run_wake(dev: *mut ata_device);
    pub fn zpodd_post_poweron(dev: *mut ata_device);
}

#[inline] pub unsafe fn zpodd_dev_enabled(dev: *mut ata_device) -> bool { !(*dev).zpodd.is_null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
