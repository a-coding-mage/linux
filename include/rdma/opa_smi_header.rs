/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2014 Intel Corporation.  All rights reserved.
 */

// Dependencies supplied by the corresponding RDMA headers are intentionally
// referenced here rather than reimplemented.

pub const OPA_SMP_LID_DATA_SIZE: usize = 2016;
pub const OPA_SMP_DR_DATA_SIZE: usize = 1872;
pub const OPA_SMP_MAX_PATH_HOPS: usize = 64;

pub const OPA_MAX_VLS: u8 = 32;
pub const OPA_MAX_SLS: u8 = 32;
pub const OPA_MAX_SCS: u8 = 32;

pub const OPA_LID_PERMISSIVE: u32 = 0xffff_ffff;

#[repr(C, packed)]
pub struct opa_smp {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: __be16,
    pub hop_ptr: u8,
    pub hop_cnt: u8,
    pub tid: __be64,
    pub attr_id: __be16,
    pub resv: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub route: opa_smp_route,
}

#[repr(C)]
pub union opa_smp_route {
    pub lid: opa_smp_lid_route,
    pub dr: opa_smp_dr_route,
}

#[repr(C)]
pub struct opa_smp_lid_route {
    pub data: [u8; OPA_SMP_LID_DATA_SIZE],
}

#[repr(C)]
pub struct opa_smp_dr_route {
    pub dr_slid: __be32,
    pub dr_dlid: __be32,
    pub initial_path: [u8; OPA_SMP_MAX_PATH_HOPS],
    pub return_path: [u8; OPA_SMP_MAX_PATH_HOPS],
    pub reserved: [u8; 8],
    pub data: [u8; OPA_SMP_DR_DATA_SIZE],
}

/* Subnet management attributes */
pub const OPA_ATTRIB_ID_NODE_DESCRIPTION: __be16 = u16::from_be(0x0010);
pub const OPA_ATTRIB_ID_NODE_INFO: __be16 = u16::from_be(0x0011);
pub const OPA_ATTRIB_ID_PORT_INFO: __be16 = u16::from_be(0x0015);
pub const OPA_ATTRIB_ID_PARTITION_TABLE: __be16 = u16::from_be(0x0016);
pub const OPA_ATTRIB_ID_SL_TO_SC_MAP: __be16 = u16::from_be(0x0017);
pub const OPA_ATTRIB_ID_VL_ARBITRATION: __be16 = u16::from_be(0x0018);
pub const OPA_ATTRIB_ID_SM_INFO: __be16 = u16::from_be(0x0020);
pub const OPA_ATTRIB_ID_CABLE_INFO: __be16 = u16::from_be(0x0032);
pub const OPA_ATTRIB_ID_AGGREGATE: __be16 = u16::from_be(0x0080);
pub const OPA_ATTRIB_ID_SC_TO_SL_MAP: __be16 = u16::from_be(0x0082);
pub const OPA_ATTRIB_ID_SC_TO_VLR_MAP: __be16 = u16::from_be(0x0083);
pub const OPA_ATTRIB_ID_SC_TO_VLT_MAP: __be16 = u16::from_be(0x0084);
pub const OPA_ATTRIB_ID_SC_TO_VLNT_MAP: __be16 = u16::from_be(0x0085);
pub const OPA_ATTRIB_ID_PORT_STATE_INFO: __be16 = u16::from_be(0x0087);
pub const OPA_ATTRIB_ID_BUFFER_CONTROL_TABLE: __be16 = u16::from_be(0x008a);

#[repr(C, packed)]
pub struct opa_node_description {
    pub data: [u8; 64],
}

#[repr(C, packed)]
pub struct opa_node_info {
    pub base_version: u8,
    pub class_version: u8,
    pub node_type: u8,
    pub num_ports: u8,
    pub reserved: __be32,
    pub system_image_guid: __be64,
    pub node_guid: __be64,
    pub port_guid: __be64,
    pub partition_cap: __be16,
    pub device_id: __be16,
    pub revision: __be32,
    pub local_port_num: u8,
    pub vendor_id: [u8; 3],
}

pub const OPA_PARTITION_TABLE_BLK_SIZE: usize = 32;

#[inline]
pub unsafe fn opa_get_smp_direction(smp: *mut opa_smp) -> u8 {
    ib_get_smp_direction(smp as *mut ib_smp)
}

#[inline]
pub unsafe fn opa_get_smp_data(smp: *mut opa_smp) -> *mut u8 {
    if (*smp).mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE {
        (*smp).route.dr.data.as_mut_ptr()
    } else {
        (*smp).route.lid.data.as_mut_ptr()
    }
}

#[inline]
pub unsafe fn opa_get_smp_data_size(smp: *mut opa_smp) -> usize {
    if (*smp).mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE {
        core::mem::size_of::<[u8; OPA_SMP_DR_DATA_SIZE]>()
    } else {
        core::mem::size_of::<[u8; OPA_SMP_LID_DATA_SIZE]>()
    }
}

#[inline]
pub unsafe fn opa_get_smp_header_size(smp: *mut opa_smp) -> usize {
    if (*smp).mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE {
        core::mem::size_of::<opa_smp>() - core::mem::size_of::<[u8; OPA_SMP_DR_DATA_SIZE]>()
    } else {
        core::mem::size_of::<opa_smp>() - core::mem::size_of::<[u8; OPA_SMP_LID_DATA_SIZE]>()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
