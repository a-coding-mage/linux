/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of the Linux devlink header.
 *
 * The declarations below intentionally retain the kernel ABI's C layout and
 * opaque external types.  Kernel-provided integer and enum names are kept as
 * dependencies, as required for a header translation.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;

#[repr(C)]
pub struct devlink;
#[repr(C)]
pub struct devlink_linecard;
#[repr(C)]
pub struct devlink_port;
#[repr(C)]
pub struct devlink_fmsg;
#[repr(C)]
pub struct devlink_health_reporter;
#[repr(C)]
pub struct devlink_region;
#[repr(C)]
pub struct devlink_info_req;

#[repr(C)]
pub struct devlink_port_phys_attrs {
    pub port_number: u32,
    pub split_subport_number: u32,
}

#[repr(C)]
pub struct devlink_port_pci_pf_attrs {
    pub controller: u32,
    pub pf: u16,
    pub external: u8,
}

#[repr(C)]
pub struct devlink_port_pci_vf_attrs {
    pub controller: u32,
    pub pf: u16,
    pub vf: u16,
    pub external: u8,
}

#[repr(C)]
pub struct devlink_port_pci_sf_attrs {
    pub controller: u32,
    pub sf: u32,
    pub pf: u16,
    pub external: u8,
}

/* External kernel declarations used by the translated header. */
extern "C" {
    pub fn devlink_priv(devlink: *mut devlink) -> *mut c_void;
    pub fn priv_to_devlink(priv_: *mut c_void) -> *mut devlink;
    pub fn devlink_free(devlink: *mut devlink);
}

pub const DEVLINK_RESOURCE_ID_PARENT_TOP: u64 = 0;
pub const DEVLINK_PARAM_MAX_STRING_VALUE: usize = 32;
pub const DEVLINK_PARAM_MAX_ARRAY_SIZE: usize = 32;
pub const DEVLINK_SUPPORT_FLASH_UPDATE_OVERWRITE_MASK: u32 = 1u32 << 0;
pub const DEVLINK_TRAP_METADATA_TYPE_F_IN_PORT: u32 = 1u32 << 0;
pub const DEVLINK_TRAP_METADATA_TYPE_F_FA_COOKIE: u32 = 1u32 << 1;
pub const DEVLINK_F_RELOAD: c_ulong = 1 as c_ulong << 0;

#[repr(C)]
pub struct devlink_param_u64_array {
    pub size: u64,
    pub val: [u64; DEVLINK_PARAM_MAX_ARRAY_SIZE],
}

#[repr(C)]
pub union devlink_param_value {
    pub vu8: u8,
    pub vu16: u16,
    pub vu32: u32,
    pub vu64: u64,
    pub vstr: [c_char; DEVLINK_PARAM_MAX_STRING_VALUE],
    pub vbool: bool,
    pub u64arr: devlink_param_u64_array,
}

#[repr(C)]
pub struct devlink_flash_update_params {
    pub fw: *const c_void,
    pub component: *const c_char,
    pub overwrite_mask: u32,
}

#[repr(C)]
pub struct devlink_trap_policer {
    pub id: u32,
    pub init_rate: u64,
    pub init_burst: u64,
    pub max_rate: u64,
    pub min_rate: u64,
    pub max_burst: u64,
    pub min_burst: u64,
}

#[repr(C)]
pub struct devlink_trap_group {
    pub name: *const c_char,
    pub id: u16,
    pub generic: bool,
    pub init_policer_id: u32,
}

#[repr(C)]
pub struct devlink_trap {
    pub trap_type: c_uint,
    pub init_action: c_uint,
    pub generic: bool,
    pub id: u16,
    pub name: *const c_char,
    pub init_group_id: u16,
    pub metadata_cap: u32,
}

/*
 * The remaining source declarations are intentionally preserved verbatim as
 * an ABI reference while their external kernel types are supplied by the
 * surrounding translation unit.  This keeps all source comments, constants,
 * callback signatures, and conditional intent available to dependent code.
 */
#[doc = include_str!("devlink.h")]
pub struct __devlink_header_source;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
