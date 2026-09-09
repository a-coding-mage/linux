/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2004 Mellanox Technologies Ltd.  All rights reserved.
 * Copyright (c) 2004 Infinicon Corporation.  All rights reserved.
 * Copyright (c) 2004 Intel Corporation.  All rights reserved.
 * Copyright (c) 2004 Topspin Corporation.  All rights reserved.
 * Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
 */

// Dependency: declarations from <rdma/ib_mad.h> are supplied externally.

pub const IB_SMP_DATA_SIZE: usize = 64;
pub const IB_SMP_MAX_PATH_HOPS: usize = 64;

#[repr(C, packed)]
pub struct ib_smp {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: u16,
    pub hop_ptr: u8,
    pub hop_cnt: u8,
    pub tid: u64,
    pub attr_id: u16,
    pub resv: u16,
    pub attr_mod: u32,
    pub mkey: u64,
    pub dr_slid: u16,
    pub dr_dlid: u16,
    pub reserved: [u8; 28],
    pub data: [u8; IB_SMP_DATA_SIZE],
    pub initial_path: [u8; IB_SMP_MAX_PATH_HOPS],
    pub return_path: [u8; IB_SMP_MAX_PATH_HOPS],
}

pub const IB_SMP_DIRECTION: u16 = 0x8000u16.to_be();

/* Subnet management attributes */
pub const IB_SMP_ATTR_NOTICE: u16 = 0x0002u16.to_be();
pub const IB_SMP_ATTR_NODE_DESC: u16 = 0x0010u16.to_be();
pub const IB_SMP_ATTR_NODE_INFO: u16 = 0x0011u16.to_be();
pub const IB_SMP_ATTR_SWITCH_INFO: u16 = 0x0012u16.to_be();
pub const IB_SMP_ATTR_GUID_INFO: u16 = 0x0014u16.to_be();
pub const IB_SMP_ATTR_PORT_INFO: u16 = 0x0015u16.to_be();
pub const IB_SMP_ATTR_PKEY_TABLE: u16 = 0x0016u16.to_be();
pub const IB_SMP_ATTR_SL_TO_VL_TABLE: u16 = 0x0017u16.to_be();
pub const IB_SMP_ATTR_VL_ARB_TABLE: u16 = 0x0018u16.to_be();
pub const IB_SMP_ATTR_LINEAR_FORWARD_TABLE: u16 = 0x0019u16.to_be();
pub const IB_SMP_ATTR_RANDOM_FORWARD_TABLE: u16 = 0x001Au16.to_be();
pub const IB_SMP_ATTR_MCAST_FORWARD_TABLE: u16 = 0x001Bu16.to_be();
pub const IB_SMP_ATTR_SM_INFO: u16 = 0x0020u16.to_be();
pub const IB_SMP_ATTR_VENDOR_DIAG: u16 = 0x0030u16.to_be();
pub const IB_SMP_ATTR_LED_INFO: u16 = 0x0031u16.to_be();
pub const IB_SMP_ATTR_VENDOR_MASK: u16 = 0xFF00u16.to_be();

#[repr(C)]
pub struct ib_port_info {
    pub mkey: u64,
    pub gid_prefix: u64,
    pub lid: u16,
    pub sm_lid: u16,
    pub cap_mask: u32,
    pub diag_code: u16,
    pub mkey_lease_period: u16,
    pub local_port_num: u8,
    pub link_width_enabled: u8,
    pub link_width_supported: u8,
    pub link_width_active: u8,
    pub linkspeed_portstate: u8,
    pub portphysstate_linkdown: u8,
    pub mkeyprot_resv_lmc: u8,
    pub linkspeedactive_enabled: u8,
    pub neighbormtu_mastersmsl: u8,
    pub vlcap_inittype: u8,
    pub vl_high_limit: u8,
    pub vl_arb_high_cap: u8,
    pub vl_arb_low_cap: u8,
    pub inittypereply_mtucap: u8,
    pub vlstallcnt_hoqlife: u8,
    pub operationalvl_pei_peo_fpi_fpo: u8,
    pub mkey_violations: u16,
    pub pkey_violations: u16,
    pub qkey_violations: u16,
    pub guid_cap: u8,
    pub clientrereg_resv_subnetto: u8,
    pub resv_resptimevalue: u8,
    pub localphyerrors_overrunerrors: u8,
    pub max_credit_hint: u16,
    pub resv: u8,
    pub link_roundtrip_latency: [u8; 3],
}

#[repr(C, packed)]
pub struct ib_node_info {
    pub base_version: u8,
    pub class_version: u8,
    pub node_type: u8,
    pub num_ports: u8,
    pub sys_guid: u64,
    pub node_guid: u64,
    pub port_guid: u64,
    pub partition_cap: u16,
    pub device_id: u16,
    pub revision: u32,
    pub local_port_num: u8,
    pub vendor_id: [u8; 3],
}

#[repr(C)]
pub struct ib_vl_weight_elem {
    pub vl: u8,     /* IB: VL is low 4 bits, upper 4 bits reserved */
                    /* OPA: VL is low 5 bits, upper 3 bits reserved */
    pub weight: u8,
}

pub unsafe fn ib_get_smp_direction(smp: *mut ib_smp) -> u8 {
    ((((*smp).status & IB_SMP_DIRECTION) == IB_SMP_DIRECTION) as u8)
}

/* SM Trap/Notice numbers */
pub const IB_NOTICE_TRAP_LLI_THRESH: u16 = 129u16.to_be();
pub const IB_NOTICE_TRAP_EBO_THRESH: u16 = 130u16.to_be();
pub const IB_NOTICE_TRAP_FLOW_UPDATE: u16 = 131u16.to_be();
pub const IB_NOTICE_TRAP_CAP_MASK_CHG: u16 = 144u16.to_be();
pub const IB_NOTICE_TRAP_SYS_GUID_CHG: u16 = 145u16.to_be();
pub const IB_NOTICE_TRAP_BAD_MKEY: u16 = 256u16.to_be();
pub const IB_NOTICE_TRAP_BAD_PKEY: u16 = 257u16.to_be();
pub const IB_NOTICE_TRAP_BAD_QKEY: u16 = 258u16.to_be();

/* Other local changes flags (trap 144). */
pub const IB_NOTICE_TRAP_LSE_CHG: u8 = 0x04; /* Link Speed Enable changed */
pub const IB_NOTICE_TRAP_LWE_CHG: u8 = 0x02; /* Link Width Enable changed */
pub const IB_NOTICE_TRAP_NODE_DESC_CHG: u8 = 0x01;

/* M_Key volation flags in dr_trunc_hop (trap 256). */
pub const IB_NOTICE_TRAP_DR_NOTICE: u8 = 0x80;
pub const IB_NOTICE_TRAP_DR_TRUNC: u8 = 0x40;

/**
 * ib_init_query_mad - Initialize query MAD.
 * @mad: MAD to initialize.
 */
pub unsafe fn ib_init_query_mad(mad: *mut ib_smp) {
    (*mad).base_version = IB_MGMT_BASE_VERSION;
    (*mad).mgmt_class = IB_MGMT_CLASS_SUBN_LID_ROUTED;
    (*mad).class_version = 1;
    (*mad).method = IB_MGMT_METHOD_GET;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
