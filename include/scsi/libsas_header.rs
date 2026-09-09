/* SPDX-License-Identifier: GPL-2.0-only */
/* SAS host prototypes and structures header file (translated from libsas.h). */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */
pub const ATA_RESP_FIS_SIZE: usize = 24;
pub const SAS_STATUS_BUF_SIZE: usize = 96;
pub const SAS_PHY_SHUTDOWN_THRES: i32 = 1024;
pub const SAS_TASK_STATE_PENDING: u32 = 1;
pub const SAS_TASK_STATE_DONE: u32 = 2;
pub const SAS_TASK_STATE_ABORTED: u32 = 4;
pub const SAS_TASK_NEED_DEV_RESET: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sas_phy_role { PHY_ROLE_NONE = 0, PHY_ROLE_TARGET = 0x40, PHY_ROLE_INITIATOR = 0x80 }
#[repr(C)] pub enum port_event { PORTE_BYTES_DMAED = 0, PORTE_BROADCAST_RCVD, PORTE_LINK_RESET_ERR, PORTE_TIMER_EVENT, PORTE_HARD_RESET, PORT_NUM_EVENTS }
#[repr(C)] pub enum phy_event { PHYE_LOSS_OF_SIGNAL = 0, PHYE_OOB_DONE, PHYE_OOB_ERROR, PHYE_SPINUP_HOLD, PHYE_RESUME_TIMEOUT, PHYE_SHUTDOWN, PHY_NUM_EVENTS }
#[repr(C)] pub enum discover_event { DISCE_DISCOVER_DOMAIN = 0, DISCE_REVALIDATE_DOMAIN, DISCE_SUSPEND, DISCE_RESUME, DISC_NUM_EVENTS }
#[repr(C)] pub enum routing_attribute { DIRECT_ROUTING, SUBTRACTIVE_ROUTING, TABLE_ROUTING }
#[repr(C)] pub enum ex_phy_state { PHY_EMPTY, PHY_VACANT, PHY_NOT_PRESENT, PHY_DEVICE_DISCOVERED }

#[repr(C)] pub struct ex_phy { pub phy_id: i32, pub phy_state: ex_phy_state, pub attached_dev_type: sas_device_type, pub linkrate: sas_linkrate, pub attached_sata_host: u8, pub attached_sata_dev: u8, pub attached_sata_ps: u8, pub attached_tproto: sas_protocol, pub attached_iproto: sas_protocol, pub attached_sas_addr: [u8; SAS_ADDR_SIZE], pub attached_phy_id: u8, pub phy_change_count: i32, pub routing_attr: routing_attribute, pub virtual_: u8, pub last_da_index: i32, pub phy: *mut sas_phy, pub port: *mut sas_port }
#[repr(C)] pub struct expander_device { pub children: list_head, pub ex_change_count: i32, pub max_route_indexes: u16, pub num_phys: u8, pub t2t_supp: u8, pub configuring: u8, pub conf_route_table: u8, pub enclosure_logical_id: [u8; 8], pub ex_phy: *mut ex_phy, pub parent_port: *mut sas_port, pub cmd_mutex: mutex }
#[repr(C)] pub struct sata_device { pub class: u32, pub port_no: u8, pub ap: *mut ata_port, pub ata_host: *mut ata_host, pub rps_resp: smp_rps_resp, pub fis: [u8; ATA_RESP_FIS_SIZE] }
#[repr(C)] pub struct ssp_device { pub eh_list_node: list_head, pub reset_lun: scsi_lun }
#[repr(C)] pub struct domain_device { pub done_lock: spinlock_t, pub dev_type: sas_device_type, pub linkrate: sas_linkrate, pub min_linkrate: sas_linkrate, pub max_linkrate: sas_linkrate, pub pathways: i32, pub parent: *mut domain_device, pub siblings: list_head, pub port: *mut asd_sas_port, pub phy: *mut sas_phy, pub dev_list_node: list_head, pub disco_list_node: list_head, pub iproto: sas_protocol, pub tproto: sas_protocol, pub rphy: *mut sas_rphy, pub sas_addr: [u8; SAS_ADDR_SIZE], pub hashed_sas_addr: [u8; HASHED_SAS_ADDR_SIZE], pub frame_rcvd: [u8; 32], pub devices: domain_device_union, pub lldd_dev: *mut c_void, pub state: usize, pub kref: kref }
#[repr(C)] pub union domain_device_union { pub ex_dev: expander_device, pub sata_dev: sata_device, pub ssp_dev: ssp_device }
#[repr(C)] pub struct sas_work { pub drain_node: list_head, pub work: work_struct }
#[repr(C)] pub struct sas_discovery_event { pub work: sas_work, pub port: *mut asd_sas_port }
#[repr(C)] pub struct sas_discovery { pub disc_work: [sas_discovery_event; DISC_NUM_EVENTS as usize], pub pending: usize, pub fanout_sas_addr: [u8; SAS_ADDR_SIZE], pub eeds_a: [u8; SAS_ADDR_SIZE], pub eeds_b: [u8; SAS_ADDR_SIZE], pub max_level: i32 }
#[repr(C)] pub struct asd_sas_port { pub disc: sas_discovery, pub port_dev: *mut domain_device, pub dev_list_lock: spinlock_t, pub dev_list: list_head, pub disco_list: list_head, pub destroy_list: list_head, pub sas_port_del_list: list_head, pub linkrate: sas_linkrate, pub work: sas_work, pub suspended: i32, pub id: i32, pub sas_addr: [u8; SAS_ADDR_SIZE], pub attached_sas_addr: [u8; SAS_ADDR_SIZE], pub iproto: sas_protocol, pub tproto: sas_protocol, pub oob_mode: sas_oob_mode, pub phy_list_lock: spinlock_t, pub phy_list: list_head, pub num_phys: i32, pub phy_mask: u32, pub ha: *mut sas_ha_struct, pub port: *mut sas_port, pub lldd_port: *mut c_void }
#[repr(C)] pub struct asd_sas_event { pub work: sas_work, pub phy: *mut asd_sas_phy, pub event: i32 }
#[repr(C)] pub struct asd_sas_phy { pub event_nr: atomic_t, pub in_shutdown: i32, pub error: i32, pub suspended: i32, pub phy: *mut sas_phy, pub enabled: i32, pub id: i32, pub iproto: sas_protocol, pub tproto: sas_protocol, pub role: sas_phy_role, pub oob_mode: sas_oob_mode, pub linkrate: sas_linkrate, pub sas_addr: *mut u8, pub attached_sas_addr: [u8; SAS_ADDR_SIZE], pub frame_rcvd_lock: spinlock_t, pub frame_rcvd: *mut u8, pub frame_rcvd_size: i32, pub sas_prim_lock: spinlock_t, pub sas_prim: u32, pub port_phy_el: list_head, pub port: *mut asd_sas_port, pub ha: *mut sas_ha_struct, pub lldd_phy: *mut c_void }
#[repr(C)] pub enum sas_ha_state { SAS_HA_REGISTERED, SAS_HA_DRAINING, SAS_HA_ATA_EH_ACTIVE, SAS_HA_FROZEN, SAS_HA_RESUMING }
#[repr(C)] pub struct sas_ha_struct { pub defer_q: list_head, pub drain_mutex: mutex, pub state: usize, pub lock: spinlock_t, pub eh_active: i32, pub eh_wait_q: wait_queue_head_t, pub eh_dev_q: list_head, pub disco_mutex: mutex, pub shost: *mut Scsi_Host, pub sas_ha_name: *mut i8, pub dev: *mut device, pub event_q: *mut workqueue_struct, pub disco_q: *mut workqueue_struct, pub sas_addr: *mut u8, pub hashed_sas_addr: [u8; HASHED_SAS_ADDR_SIZE], pub phy_port_lock: spinlock_t, pub sas_phy: *mut *mut asd_sas_phy, pub sas_port: *mut *mut asd_sas_port, pub num_phys: i32, pub strict_wide_ports: i32, pub lldd_ha: *mut c_void, pub eh_done_q: list_head, pub eh_ata_q: list_head, pub event_thres: i32 }

#[inline] pub unsafe fn dev_is_expander(t: sas_device_type) -> bool { t == SAS_EDGE_EXPANDER_DEVICE || t == SAS_FANOUT_EXPANDER_DEVICE }
#[inline] pub unsafe fn dev_parent_is_expander(d: *mut domain_device) -> bool { !(*d).parent.is_null() && dev_is_expander((*(*d).parent).dev_type) }
#[inline] pub unsafe fn sas_phy_disconnected(p: *mut asd_sas_phy) { (*p).oob_mode = OOB_NOT_CONNECTED; (*p).linkrate = SAS_LINK_RATE_UNKNOWN; }
#[inline] pub const fn to_sas_gpio_od(device: i32, bit: i32) -> u32 { (3 * device + bit) as u32 }

#[repr(C)] pub enum service_response { SAS_TASK_COMPLETE, SAS_TASK_UNDELIVERED = -1 }
#[repr(C)] pub enum exec_status { SAS_SAM_STAT_GOOD = SAM_STAT_GOOD, SAS_SAM_STAT_BUSY = SAM_STAT_BUSY, SAS_SAM_STAT_TASK_ABORTED = SAM_STAT_TASK_ABORTED, SAS_SAM_STAT_CHECK_CONDITION = SAM_STAT_CHECK_CONDITION, SAS_DEV_NO_RESPONSE = 0x80, SAS_DATA_UNDERRUN, SAS_DATA_OVERRUN, SAS_INTERRUPTED, SAS_QUEUE_FULL, SAS_DEVICE_UNKNOWN, SAS_OPEN_REJECT, SAS_OPEN_TO, SAS_PROTO_RESPONSE, SAS_PHY_DOWN, SAS_NAK_R_ERR, SAS_PENDING, SAS_ABORTED_TASK }
#[repr(C)] pub struct ata_task_resp { pub frame_len: u16, pub ending_fis: [u8; ATA_RESP_FIS_SIZE] }
#[repr(C)] pub struct task_status_struct { pub resp: service_response, pub stat: exec_status, pub buf_valid_size: i32, pub buf: [u8; SAS_STATUS_BUF_SIZE], pub residual: u32, pub open_rej_reason: sas_open_rej_reason }
#[repr(C)] pub struct sas_ata_task { pub fis: host_to_dev_fis, pub atapi_packet: [u8; 16], pub dma_xfer: u8, pub use_ncq: u8, pub return_fis_on_success: u8, pub device_control_reg_update: u8, pub force_phy: bool, pub force_phy_id: i32 }
#[repr(C)] pub enum sas_internal_abort { SAS_INTERNAL_ABORT_SINGLE = 0, SAS_INTERNAL_ABORT_DEV = 1 }
#[repr(C)] pub struct sas_internal_abort_task { pub type_: sas_internal_abort, pub qid: u32, pub tag: u16 }
#[repr(C)] pub struct sas_smp_task { pub smp_req: scatterlist, pub smp_resp: scatterlist }
#[repr(C)] pub enum task_attribute { TASK_ATTR_SIMPLE = 0, TASK_ATTR_HOQ = 1, TASK_ATTR_ORDERED = 2, TASK_ATTR_ACA = 4 }
#[repr(C)] pub struct sas_ssp_task { pub LUN: [u8; 8], pub task_attr: task_attribute, pub cmd: *mut scsi_cmnd }
#[repr(C)] pub struct sas_tmf_task { pub tmf: u8, pub tag_of_task_to_be_managed: u16 }
#[repr(C)] pub union sas_task_union { pub ata_task: sas_ata_task, pub smp_task: sas_smp_task, pub ssp_task: sas_ssp_task, pub abort_task: sas_internal_abort_task }
#[repr(C)] pub struct sas_task { pub dev: *mut domain_device, pub task_state_lock: spinlock_t, pub task_state_flags: u32, pub task_proto: sas_protocol, pub task: sas_task_union, pub scatter: *mut scatterlist, pub num_scatter: i32, pub total_xfer_len: u32, pub data_dir: u8, pub task_status: task_status_struct, pub task_done: Option<unsafe extern "C" fn(*mut sas_task)>, pub lldd_task: *mut c_void, pub uldd_task: *mut c_void, pub slow_task: *mut sas_task_slow, pub tmf: *mut sas_tmf_task }
#[repr(C)] pub struct sas_task_slow { pub timer: timer_list, pub completion: completion, pub task: *mut sas_task }

#[inline] pub unsafe fn sas_is_internal_abort(t: *mut sas_task) -> bool { (*t).task_proto == SAS_PROTOCOL_INTERNAL_ABORT }

#[repr(C)] pub struct sas_domain_function_template { pub lldd_port_formed: Option<unsafe extern "C" fn(*mut asd_sas_phy)>, pub lldd_port_deformed: Option<unsafe extern "C" fn(*mut asd_sas_phy)>, pub lldd_dev_found: Option<unsafe extern "C" fn(*mut domain_device) -> i32>, pub lldd_dev_gone: Option<unsafe extern "C" fn(*mut domain_device)>, pub lldd_execute_task: Option<unsafe extern "C" fn(*mut sas_task, gfp_t) -> i32>, pub lldd_abort_task: Option<unsafe extern "C" fn(*mut sas_task) -> i32>, pub lldd_abort_task_set: Option<unsafe extern "C" fn(*mut domain_device, *mut u8) -> i32>, pub lldd_clear_task_set: Option<unsafe extern "C" fn(*mut domain_device, *mut u8) -> i32>, pub lldd_I_T_nexus_reset: Option<unsafe extern "C" fn(*mut domain_device) -> i32>, pub lldd_ata_check_ready: Option<unsafe extern "C" fn(*mut domain_device) -> i32>, pub lldd_ata_set_dmamode: Option<unsafe extern "C" fn(*mut domain_device)>, pub lldd_lu_reset: Option<unsafe extern "C" fn(*mut domain_device, *mut u8) -> i32>, pub lldd_query_task: Option<unsafe extern "C" fn(*mut sas_task) -> i32>, pub lldd_tmf_exec_complete: Option<unsafe extern "C" fn(*mut domain_device)>, pub lldd_tmf_aborted: Option<unsafe extern "C" fn(*mut sas_task)>, pub lldd_abort_timeout: Option<unsafe extern "C" fn(*mut sas_task, *mut c_void) -> bool>, pub lldd_clear_nexus_port: Option<unsafe extern "C" fn(*mut asd_sas_port) -> i32>, pub lldd_clear_nexus_ha: Option<unsafe extern "C" fn(*mut sas_ha_struct) -> i32>, pub lldd_control_phy: Option<unsafe extern "C" fn(*mut asd_sas_phy, phy_func, *mut c_void) -> i32>, pub lldd_write_gpio: Option<unsafe extern "C" fn(*mut sas_ha_struct, u8, u8, u8, *mut u8) -> i32> }

extern "C" {
    pub fn sas_register_ha(ha: *mut sas_ha_struct) -> i32;
    pub fn sas_unregister_ha(ha: *mut sas_ha_struct) -> i32;
    pub fn sas_prep_resume_ha(ha: *mut sas_ha_struct);
    pub fn sas_resume_ha(ha: *mut sas_ha_struct);
    pub fn sas_suspend_ha(ha: *mut sas_ha_struct);
    pub fn sas_phy_reset(phy: *mut sas_phy, hard_reset: i32) -> i32;
    pub fn sas_phy_enable(phy: *mut sas_phy, enable: i32) -> i32;
    pub fn sas_queuecommand(host: *mut Scsi_Host, cmd: *mut scsi_cmnd) -> scsi_qc_status;
    pub fn sas_target_alloc(starget: *mut scsi_target) -> i32;
    pub fn sas_sdev_configure(dev: *mut scsi_device, lim: *mut queue_limits) -> i32;
    pub fn sas_change_queue_depth(dev: *mut scsi_device, new_depth: i32) -> i32;
    pub fn sas_bios_param(dev: *mut scsi_device, disk: *mut gendisk, capacity: sector_t, hsc: *mut i32) -> i32;
    pub fn sas_execute_internal_abort_single(device: *mut domain_device, tag: u16, qid: u32, data: *mut c_void) -> i32;
    pub fn sas_execute_internal_abort_dev(device: *mut domain_device, qid: u32, data: *mut c_void) -> i32;
    pub fn sas_task_abort(task: *mut sas_task);
    pub fn sas_eh_abort_handler(cmd: *mut scsi_cmnd) -> i32;
    pub fn sas_eh_device_reset_handler(cmd: *mut scsi_cmnd) -> i32;
    pub fn sas_eh_target_reset_handler(cmd: *mut scsi_cmnd) -> i32;
    pub fn sas_target_destroy(starget: *mut scsi_target);
    pub fn sas_sdev_init(dev: *mut scsi_device) -> i32;
    pub fn sas_drain_work(ha: *mut sas_ha_struct) -> i32;
    pub fn sas_get_local_phy(dev: *mut domain_device) -> *mut sas_phy;
    pub fn sas_request_addr(shost: *mut Scsi_Host, addr: *mut u8) -> i32;
    pub fn sas_abort_task_set(dev: *mut domain_device, lun: *mut u8) -> i32;
    pub fn sas_clear_task_set(dev: *mut domain_device, lun: *mut u8) -> i32;
    pub fn sas_lu_reset(dev: *mut domain_device, lun: *mut u8) -> i32;
    pub fn sas_query_task(task: *mut sas_task, tag: u16) -> i32;
    pub fn sas_abort_task(task: *mut sas_task, tag: u16) -> i32;
    pub fn sas_find_attached_phy_id(ex_dev: *mut expander_device, dev: *mut domain_device) -> i32;
    pub fn sas_notify_port_event(phy: *mut asd_sas_phy, event: port_event, gfp_flags: gfp_t);
    pub fn sas_notify_phy_event(phy: *mut asd_sas_phy, event: phy_event, gfp_flags: gfp_t);
}

/* CONFIG_SCSI_SAS_HOST_SMP controls whether this symbol is supplied externally. */
#[cfg(feature = "CONFIG_SCSI_SAS_HOST_SMP")]
extern "C" { pub fn try_test_sas_gpio_gp_bit(od: u32, data: *mut u8, index: u8, count: u8) -> i32; }
#[cfg(not(feature = "CONFIG_SCSI_SAS_HOST_SMP"))]
#[inline] pub unsafe fn try_test_sas_gpio_gp_bit(_: u32, _: *mut u8, _: u8, _: u8) -> i32 { -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
