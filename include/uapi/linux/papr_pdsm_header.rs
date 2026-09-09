/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * PAPR nvDimm Specific Methods (PDSM) and structs for libndctl
 *
 * (C) Copyright IBM 2020
 *
 * Author: Vaibhav Jain <vaibhav at linux.ibm.com>
 */

/* Dependencies: linux/types.h and linux/ndctl.h. */

/* Max payload size that we can handle */
pub const ND_PDSM_PAYLOAD_MAX_SIZE: usize = 184;

/* Max payload size that we can handle */
pub const ND_PDSM_HDR_SIZE: usize = core::mem::size_of::<nd_pkg_pdsm>() - ND_PDSM_PAYLOAD_MAX_SIZE;

/* Various nvdimm health indicators */
pub const PAPR_PDSM_DIMM_HEALTHY: u32 = 0;
pub const PAPR_PDSM_DIMM_UNHEALTHY: u32 = 1;
pub const PAPR_PDSM_DIMM_CRITICAL: u32 = 2;
pub const PAPR_PDSM_DIMM_FATAL: u32 = 3;

/* struct nd_papr_pdsm_health.extension_flags field flags */

/* Indicate that the 'dimm_fuel_gauge' field is valid */
pub const PDSM_DIMM_HEALTH_RUN_GAUGE_VALID: u32 = 1;

/* Indicate that the 'dimm_dsc' field is valid */
pub const PDSM_DIMM_DSC_VALID: u32 = 2;

#[repr(C)]
pub struct nd_papr_pdsm_health_fields {
    pub extension_flags: u32,
    pub dimm_unarmed: u8,
    pub dimm_bad_shutdown: u8,
    pub dimm_bad_restore: u8,
    pub dimm_scrubbed: u8,
    pub dimm_locked: u8,
    pub dimm_encrypted: u8,
    pub dimm_health: u16,
    /* Extension flag PDSM_DIMM_HEALTH_RUN_GAUGE_VALID */
    pub dimm_fuel_gauge: u16,
    /* Extension flag PDSM_DIMM_DSC_VALID */
    pub dimm_dsc: u64,
}

#[repr(C)]
pub union nd_papr_pdsm_health {
    pub fields: nd_papr_pdsm_health_fields,
    pub buf: [u8; ND_PDSM_PAYLOAD_MAX_SIZE],
}

/* Flags for injecting specific smart errors */
pub const PDSM_SMART_INJECT_HEALTH_FATAL: u32 = 1 << 0;
pub const PDSM_SMART_INJECT_BAD_SHUTDOWN: u32 = 1 << 1;

#[repr(C)]
pub struct nd_papr_pdsm_smart_inject_fields {
    /* One or more of PDSM_SMART_INJECT_ */
    pub flags: u32,
    pub fatal_enable: u8,
    pub unsafe_shutdown_enable: u8,
}

#[repr(C)]
pub union nd_papr_pdsm_smart_inject {
    pub fields: nd_papr_pdsm_smart_inject_fields,
    pub buf: [u8; ND_PDSM_PAYLOAD_MAX_SIZE],
}

/* Methods to be embedded in ND_CMD_CALL request. */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum papr_pdsm {
    PAPR_PDSM_MIN = 0x0,
    PAPR_PDSM_HEALTH,
    PAPR_PDSM_SMART_INJECT,
    PAPR_PDSM_MAX,
}

/* Maximal union that can hold all possible payload types */
#[repr(C, packed)]
pub union nd_pdsm_payload {
    pub health: nd_papr_pdsm_health,
    pub smart_inject: nd_papr_pdsm_smart_inject,
    pub buf: [u8; ND_PDSM_PAYLOAD_MAX_SIZE],
}

/* PDSM-header + payload expected with ND_CMD_CALL ioctl from libnvdimm. */
#[repr(C, packed)]
pub struct nd_pkg_pdsm {
    pub cmd_status: i32,
    pub reserved: [u16; 2],
    pub payload: nd_pdsm_payload,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
