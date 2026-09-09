/* SPDX-License-Identifier: GPL-2.0 */
/* Author: Dan Scally <djrscally@gmail.com> */

// C dependencies: linux/mod_devicetable.h, linux/property.h,
// linux/types.h, and media/v4l2-fwnode.h.

pub const IPU_HID: &str = "INT343E";
pub const IPU_MAX_LANES: usize = 4;
pub const IPU_MAX_PORTS: usize = 4;
pub const MAX_NUM_LINK_FREQS: usize = 3;

/* Values are educated guesses as we don't have a spec */
pub const IPU_SENSOR_ROTATION_NORMAL: u32 = 0;
pub const IPU_SENSOR_ROTATION_INVERTED: u32 = 1;

#[macro_export]
macro_rules! IPU_SENSOR_CONFIG {
    ($hid:expr, $nr:expr $(, $freq:expr)*) => {
        $crate::ipu_sensor_config {
            hid: $hid,
            nr_link_freqs: $nr,
            link_freqs: [$($freq,)* 0 as _],
        }
    };
}

#[macro_export]
macro_rules! NODE_SENSOR {
    ($hid:expr, $props:expr) => {
        $crate::software_node { name: $hid, properties: $props, ..::core::default::Default::default() }
    };
}

#[macro_export]
macro_rules! NODE_PORT {
    ($port:expr, $sensor_node:expr) => {
        $crate::software_node { name: $port, parent: $sensor_node, ..::core::default::Default::default() }
    };
}

#[macro_export]
macro_rules! NODE_ENDPOINT {
    ($ep:expr, $port:expr, $props:expr) => {
        $crate::software_node { name: $ep, parent: $port, properties: $props, ..::core::default::Default::default() }
    };
}

#[macro_export]
macro_rules! NODE_VCM {
    ($type_:expr) => {
        $crate::software_node { name: $type_, ..::core::default::Default::default() }
    };
}

#[repr(C)]
pub enum ipu_sensor_swnodes {
    SWNODE_SENSOR_HID,
    SWNODE_SENSOR_PORT,
    SWNODE_SENSOR_ENDPOINT,
    SWNODE_IPU_PORT,
    SWNODE_IPU_ENDPOINT,
    /* below are optional / maybe empty */
    SWNODE_IVSC_HID,
    SWNODE_IVSC_SENSOR_PORT,
    SWNODE_IVSC_SENSOR_ENDPOINT,
    SWNODE_IVSC_IPU_PORT,
    SWNODE_IVSC_IPU_ENDPOINT,
    SWNODE_VCM,
    SWNODE_COUNT,
}

/* Data representation as it is in ACPI SSDB buffer */
#[repr(C, packed)]
pub struct ipu_sensor_ssdb {
    pub version: u8,
    pub sku: u8,
    pub guid_csi2: [u8; 16],
    pub devfunction: u8,
    pub bus: u8,
    pub dphylinkenfuses: u32,
    pub clockdiv: u32,
    pub link: u8,
    pub lanes: u8,
    pub csiparams: [u32; 10],
    pub maxlanespeed: u32,
    pub sensorcalibfileidx: u8,
    pub sensorcalibfileidxInMBZ: [u8; 3],
    pub romtype: u8,
    pub vcmtype: u8,
    pub platforminfo: u8,
    pub platformsubinfo: u8,
    pub flash: u8,
    pub privacyled: u8,
    pub degree: u8,
    pub mipilinkdefined: u8,
    pub mclkspeed: u32,
    pub controllogicid: u8,
    pub reserved1: [u8; 3],
    pub mclkport: u8,
    pub reserved2: [u8; 13],
}

#[repr(C)]
pub struct ipu_property_names {
    pub clock_frequency: [::core::ffi::c_char; 16],
    pub rotation: [::core::ffi::c_char; 9],
    pub orientation: [::core::ffi::c_char; 12],
    pub bus_type: [::core::ffi::c_char; 9],
    pub data_lanes: [::core::ffi::c_char; 11],
    pub remote_endpoint: [::core::ffi::c_char; 16],
    pub link_frequencies: [::core::ffi::c_char; 17],
}

#[repr(C)]
pub struct ipu_node_names {
    pub port: [::core::ffi::c_char; 7],
    pub ivsc_sensor_port: [::core::ffi::c_char; 7],
    pub ivsc_ipu_port: [::core::ffi::c_char; 7],
    pub endpoint: [::core::ffi::c_char; 11],
    pub remote_port: [::core::ffi::c_char; 9],
    pub vcm: [::core::ffi::c_char; 16],
}

#[repr(C)]
pub struct ipu_sensor_config {
    pub hid: *const ::core::ffi::c_char,
    pub nr_link_freqs: u8,
    pub link_freqs: [u64; MAX_NUM_LINK_FREQS],
}

#[repr(C)]
pub struct ipu_sensor {
    /* append ssdb.link(u8) in "-%u" format as suffix of HID */
    pub name: [::core::ffi::c_char; ACPI_ID_LEN + 4],
    pub adev: *mut acpi_device,
    pub csi_dev: *mut device,
    pub ivsc_adev: *mut acpi_device,
    pub ivsc_name: [::core::ffi::c_char; ACPI_ID_LEN + 4],
    /* SWNODE_COUNT + 1 for terminating NULL */
    pub group: [*const software_node; SWNODE_COUNT as usize + 1],
    pub swnodes: [software_node; SWNODE_COUNT as usize],
    pub node_names: ipu_node_names,
    pub link: u8,
    pub lanes: u8,
    pub mclkspeed: u32,
    pub rotation: u32,
    pub orientation: v4l2_fwnode_orientation,
    pub vcm_type: *const ::core::ffi::c_char,
    pub prop_names: ipu_property_names,
    pub ep_properties: [property_entry; 5],
    pub dev_properties: [property_entry; 5],
    pub ipu_properties: [property_entry; 3],
    pub ivsc_properties: [property_entry; 1],
    pub ivsc_sensor_ep_properties: [property_entry; 4],
    pub ivsc_ipu_ep_properties: [property_entry; 4],
    pub local_ref: [software_node_ref_args; 1],
    pub remote_ref: [software_node_ref_args; 1],
    pub vcm_ref: [software_node_ref_args; 1],
    pub ivsc_sensor_ref: [software_node_ref_args; 1],
    pub ivsc_ipu_ref: [software_node_ref_args; 1],
}

pub type ipu_parse_sensor_fwnode_t = unsafe extern "C" fn(*mut acpi_device, *mut ipu_sensor) -> ::core::ffi::c_int;

#[repr(C)]
pub struct ipu_bridge {
    pub dev: *mut device,
    pub parse_sensor_fwnode: Option<ipu_parse_sensor_fwnode_t>,
    pub ipu_node_name: [::core::ffi::c_char; ACPI_ID_LEN],
    pub ipu_hid_node: software_node,
    pub data_lanes: [u32; 4],
    pub n_sensors: ::core::ffi::c_uint,
    pub sensors: [ipu_sensor; IPU_MAX_PORTS],
}

// External types and ACPI_ID_LEN are supplied by the corresponding kernel headers.

#[cfg(feature = "CONFIG_IPU_BRIDGE")]
unsafe extern "C" {
    pub fn ipu_bridge_init(dev: *mut device, parse_sensor_fwnode: ipu_parse_sensor_fwnode_t) -> ::core::ffi::c_int;
    pub fn ipu_bridge_parse_ssdb(adev: *mut acpi_device, sensor: *mut ipu_sensor) -> ::core::ffi::c_int;
    pub fn ipu_bridge_instantiate_vcm(sensor: *mut device) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_IPU_BRIDGE"))]
#[macro_export]
macro_rules! ipu_bridge_init {
    ($dev:expr, $parse_sensor_fwnode:expr) => { 0 };
}

#[cfg(not(feature = "CONFIG_IPU_BRIDGE"))]
#[inline]
pub unsafe fn ipu_bridge_instantiate_vcm(_s: *mut device) -> ::core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
