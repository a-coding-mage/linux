/* SPDX-License-Identifier: GPL-2.0 */

// #include <linux/types.h>

// Forward declaration of struct net_device.
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

/* used to differentiate various protocols */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsr_version {
    HSR_V0 = 0,
    HSR_V1,
    PRP_V1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsr_port_type {
    HSR_PT_NONE = 0, /* Must be 0, used by framereg */
    HSR_PT_SLAVE_A,
    HSR_PT_SLAVE_B,
    HSR_PT_INTERLINK,
    HSR_PT_MASTER,
    HSR_PT_PORTS, /* This must be the last item in the enum */
}

/* HSR Tag.
 * As defined in IEC-62439-3:2010, the HSR tag is really { ethertype = 0x88FB,
 * path, LSDU_size, sequence Nr }. But we let eth_header() create { h_dest,
 * h_source, h_proto = 0x88FB }, and add { path, LSDU_size, sequence Nr,
 * encapsulated protocol } instead.
 *
 * Field names as defined in the IEC:2010 standard for HSR.
 */
#[repr(C, packed)]
pub struct hsr_tag {
    pub path_and_LSDU_size: __be16,
    pub sequence_nr: __be16,
    pub encap_proto: __be16,
}

pub const HSR_HLEN: u32 = 6;

// The following configuration condition corresponds to IS_ENABLED(CONFIG_HSR).
#[cfg(feature = "CONFIG_HSR")]
extern "C" {
    pub fn is_hsr_master(dev: *mut net_device) -> bool;
    pub fn hsr_get_version(dev: *mut net_device, ver: *mut hsr_version) -> ::core::ffi::c_int;
    pub fn hsr_get_port_ndev(
        ndev: *mut net_device,
        pt: hsr_port_type,
    ) -> *mut net_device;
    pub fn hsr_get_port_type(
        hsr_dev: *mut net_device,
        dev: *mut net_device,
        type_: *mut hsr_port_type,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_HSR"))]
pub unsafe fn is_hsr_master(_dev: *mut net_device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_HSR"))]
pub unsafe fn hsr_get_version(
    _dev: *mut net_device,
    _ver: *mut hsr_version,
) -> ::core::ffi::c_int {
    -EINVAL
}

#[cfg(not(feature = "CONFIG_HSR"))]
pub unsafe fn hsr_get_port_ndev(
    _ndev: *mut net_device,
    _pt: hsr_port_type,
) -> *mut net_device {
    ERR_PTR(-EINVAL)
}

#[cfg(not(feature = "CONFIG_HSR"))]
pub unsafe fn hsr_get_port_type(
    _hsr_dev: *mut net_device,
    _dev: *mut net_device,
    _type: *mut hsr_port_type,
) -> ::core::ffi::c_int {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
