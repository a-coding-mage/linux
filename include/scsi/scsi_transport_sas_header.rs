/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from scsi_transport_sas.h. Kernel-provided dependencies remain external. */

use core::ffi::c_void;

pub enum device {}
pub enum list_head {}
pub enum mutex {}
pub enum request_queue {}
pub enum request {}
pub enum scsi_transport_template {}
pub enum scsi_device {}
pub enum Scsi_Host {}
pub enum bsg_job {}

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify {
    pub device_type: sas_device_type,
    pub initiator_port_protocols: sas_protocol,
    pub target_port_protocols: sas_protocol,
    pub sas_address: u64,
    pub phy_identifier: u8,
}

pub type sas_device_type = u32;
pub type sas_protocol = u32;
pub const SAS_PROTOCOL_SATA: sas_protocol = 1 << 0;
pub const SAS_PROTOCOL_STP: sas_protocol = 1 << 1;
pub const SAS_FANOUT_EXPANDER_DEVICE: sas_device_type = 0x02;
pub const SAS_EDGE_EXPANDER_DEVICE: sas_device_type = 0x03;

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATTRS"))]
#[inline]
pub unsafe fn scsi_is_sas_rphy(_sdev: *const device) -> i32 { 0 }

#[cfg(feature = "CONFIG_SCSI_SAS_ATTRS")]
unsafe extern "C" {
    pub fn scsi_is_sas_rphy(sdev: *const device) -> i32;
}

#[inline]
pub fn sas_protocol_ata(proto: sas_protocol) -> i32 {
    if (proto & SAS_PROTOCOL_SATA) != 0 || (proto & SAS_PROTOCOL_STP) != 0 { 1 } else { 0 }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sas_linkrate {
    SAS_LINK_RATE_UNKNOWN = 0,
    SAS_PHY_DISABLED = 1,
    SAS_PHY_RESET_PROBLEM = 2,
    SAS_SATA_SPINUP_HOLD = 3,
    SAS_SATA_PORT_SELECTOR = 4,
    SAS_PHY_RESET_IN_PROGRESS = 5,
    SAS_LINK_RATE_1_5_GBPS = 8,
    SAS_LINK_RATE_G1 = 8,
    SAS_LINK_RATE_3_0_GBPS = 9,
    SAS_LINK_RATE_G2 = 9,
    SAS_LINK_RATE_6_0_GBPS = 10,
    SAS_LINK_RATE_12_0_GBPS = 11,
    SAS_LINK_RATE_22_5_GBPS = 12,
    SAS_LINK_RATE_FAILED = 0x10,
    SAS_PHY_VIRTUAL = 0x11,
}

#[repr(C)]
pub struct sas_phy {
    pub dev: device,
    pub number: i32,
    pub enabled: i32,
    pub identify: sas_identify,
    pub negotiated_linkrate: sas_linkrate,
    pub minimum_linkrate_hw: sas_linkrate,
    pub minimum_linkrate: sas_linkrate,
    pub maximum_linkrate_hw: sas_linkrate,
    pub maximum_linkrate: sas_linkrate,
    pub invalid_dword_count: u32,
    pub running_disparity_error_count: u32,
    pub loss_of_dword_sync_count: u32,
    pub phy_reset_problem_count: u32,
    pub port_siblings: list_head,
    pub hostdata: *mut c_void,
}

#[repr(C)]
pub struct sas_rphy {
    pub dev: device,
    pub identify: sas_identify,
    pub list: list_head,
    pub q: *mut request_queue,
    pub scsi_target_id: u32,
}

#[repr(C)]
pub struct sas_end_device {
    pub rphy: sas_rphy,
    /* C bit-fields, represented as individual one-byte flags. */
    pub ready_led_meaning: u8,
    pub tlr_supported: u8,
    pub tlr_enabled: u8,
    pub I_T_nexus_loss_timeout: u16,
    pub initiator_response_timeout: u16,
}

pub const SAS_EXPANDER_VENDOR_ID_LEN: usize = 8;
pub const SAS_EXPANDER_PRODUCT_ID_LEN: usize = 16;
pub const SAS_EXPANDER_PRODUCT_REV_LEN: usize = 4;
pub const SAS_EXPANDER_COMPONENT_VENDOR_ID_LEN: usize = 8;

#[repr(C)]
pub struct sas_expander_device {
    pub level: i32,
    pub next_port_id: i32,
    pub vendor_id: [i8; SAS_EXPANDER_VENDOR_ID_LEN + 1],
    pub product_id: [i8; SAS_EXPANDER_PRODUCT_ID_LEN + 1],
    pub product_rev: [i8; SAS_EXPANDER_PRODUCT_REV_LEN + 1],
    pub component_vendor_id: [i8; SAS_EXPANDER_COMPONENT_VENDOR_ID_LEN + 1],
    pub component_id: u16,
    pub component_revision_id: u8,
    pub rphy: sas_rphy,
}

#[repr(C)]
pub struct sas_port {
    pub dev: device,
    pub port_identifier: i32,
    pub num_phys: i32,
    pub is_backlink: u8,
    pub rphy: *mut sas_rphy,
    pub phy_list_mutex: mutex,
    pub phy_list: list_head,
    pub del_list: list_head,
}

#[repr(C)]
pub struct sas_phy_linkrates {
    pub maximum_linkrate: sas_linkrate,
    pub minimum_linkrate: sas_linkrate,
}

#[repr(C)]
pub struct sas_function_template {
    pub get_linkerrors: Option<unsafe extern "C" fn(*mut sas_phy) -> i32>,
    pub get_enclosure_identifier: Option<unsafe extern "C" fn(*mut sas_rphy, *mut u64) -> i32>,
    pub get_bay_identifier: Option<unsafe extern "C" fn(*mut sas_rphy) -> i32>,
    pub phy_reset: Option<unsafe extern "C" fn(*mut sas_phy, i32) -> i32>,
    pub phy_enable: Option<unsafe extern "C" fn(*mut sas_phy, i32) -> i32>,
    pub phy_setup: Option<unsafe extern "C" fn(*mut sas_phy) -> i32>,
    pub phy_release: Option<unsafe extern "C" fn(*mut sas_phy)>,
    pub set_phy_speed: Option<unsafe extern "C" fn(*mut sas_phy, *mut sas_phy_linkrates) -> i32>,
    pub smp_handler: Option<unsafe extern "C" fn(*mut bsg_job, *mut Scsi_Host, *mut sas_rphy)>,
}

unsafe extern "C" {
    pub fn sas_remove_children(dev: *mut device);
    pub fn sas_remove_host(shost: *mut Scsi_Host);
    pub fn sas_phy_alloc(dev: *mut device, number: i32) -> *mut sas_phy;
    pub fn sas_phy_free(phy: *mut sas_phy);
    pub fn sas_phy_add(phy: *mut sas_phy) -> i32;
    pub fn sas_phy_delete(phy: *mut sas_phy);
    pub fn scsi_is_sas_phy(dev: *const device) -> i32;
    pub fn sas_get_address(sdev: *mut scsi_device) -> u64;
    pub fn sas_tlr_supported(sdev: *mut scsi_device) -> u32;
    pub fn sas_is_tlr_enabled(sdev: *mut scsi_device) -> u32;
    pub fn sas_disable_tlr(sdev: *mut scsi_device);
    pub fn sas_enable_tlr(sdev: *mut scsi_device);
    pub fn sas_ata_ncq_prio_supported(sdev: *mut scsi_device) -> bool;
    pub fn sas_end_device_alloc(port: *mut sas_port) -> *mut sas_rphy;
    pub fn sas_expander_alloc(port: *mut sas_port, dtype: sas_device_type) -> *mut sas_rphy;
    pub fn sas_rphy_free(rphy: *mut sas_rphy);
    pub fn sas_rphy_add(rphy: *mut sas_rphy) -> i32;
    pub fn sas_rphy_remove(rphy: *mut sas_rphy);
    pub fn sas_rphy_delete(rphy: *mut sas_rphy);
    pub fn sas_rphy_unlink(rphy: *mut sas_rphy);
    pub fn sas_port_alloc(dev: *mut device, id: i32) -> *mut sas_port;
    pub fn sas_port_alloc_num(dev: *mut device) -> *mut sas_port;
    pub fn sas_port_add(port: *mut sas_port) -> i32;
    pub fn sas_port_free(port: *mut sas_port);
    pub fn sas_port_delete(port: *mut sas_port);
    pub fn sas_port_add_phy(port: *mut sas_port, phy: *mut sas_phy);
    pub fn sas_port_delete_phy(port: *mut sas_port, phy: *mut sas_phy);
    pub fn sas_port_mark_backlink(port: *mut sas_port);
    pub fn scsi_is_sas_port(dev: *const device) -> i32;
    pub fn sas_port_get_phy(port: *mut sas_port) -> *mut sas_phy;
    pub fn sas_attach_transport(t: *mut sas_function_template) -> *mut scsi_transport_template;
    pub fn sas_release_transport(t: *mut scsi_transport_template);
    pub fn sas_read_port_mode_page(sdev: *mut scsi_device) -> i32;
}

#[inline]
pub unsafe fn sas_port_put_phy(phy: *mut sas_phy) {
    if !phy.is_null() { put_device(&mut (*phy).dev); }
}

#[inline]
pub unsafe fn scsi_is_sas_expander_device(dev: *mut device) -> i32 {
    if scsi_is_sas_rphy(dev) == 0 { return 0; }
    let rphy = dev_to_rphy(dev);
    ((*rphy).identify.device_type == SAS_FANOUT_EXPANDER_DEVICE
        || (*rphy).identify.device_type == SAS_EDGE_EXPANDER_DEVICE) as i32
}

unsafe extern "C" { pub fn put_device(dev: *mut device); }

#[inline] pub unsafe fn dev_to_rphy(d: *mut device) -> *mut sas_rphy { d as *mut sas_rphy }
#[inline] pub unsafe fn dev_to_phy(d: *mut device) -> *mut sas_phy { d as *mut sas_phy }
#[inline] pub unsafe fn dev_to_sas_port(d: *mut device) -> *mut sas_port { d as *mut sas_port }
#[inline] pub unsafe fn rphy_to_end_device(r: *mut sas_rphy) -> *mut sas_end_device { r as *mut sas_end_device }
#[inline] pub unsafe fn rphy_to_expander_device(r: *mut sas_rphy) -> *mut sas_expander_device { r as *mut sas_expander_device }
#[inline] pub unsafe fn transport_class_to_phy(d: *mut device) -> *mut sas_phy { dev_to_phy(d) }
#[inline] pub unsafe fn transport_class_to_rphy(d: *mut device) -> *mut sas_rphy { dev_to_rphy(d) }
#[inline] pub unsafe fn transport_class_to_sas_port(d: *mut device) -> *mut sas_port { dev_to_sas_port(d) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
