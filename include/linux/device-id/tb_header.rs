/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard omitted; this Rust file is included through the module
 * system. The __KERNEL__ typedef maps kernel_ulong_t to an unsigned long.
 */
pub type kernel_ulong_t = usize;

pub const TBSVC_MATCH_PROTOCOL_KEY: u32 = 0x0001;
pub const TBSVC_MATCH_PROTOCOL_ID: u32 = 0x0002;
pub const TBSVC_MATCH_PROTOCOL_VERSION: u32 = 0x0004;
pub const TBSVC_MATCH_PROTOCOL_REVISION: u32 = 0x0008;

/**
 * struct tb_service_id - Thunderbolt service identifiers
 * @match_flags: Flags used to match the structure
 * @protocol_key: Protocol key the service supports
 * @protocol_id: Protocol id the service supports
 * @protocol_version: Version of the protocol
 * @protocol_revision: Revision of the protocol software
 * @driver_data: Driver specific data
 *
 * Thunderbolt XDomain services are exposed as devices where each device
 * carries the protocol information the service supports. Thunderbolt
 * XDomain service drivers match against that information.
 */
#[repr(C)]
pub struct tb_service_id {
    pub match_flags: u32,
    pub protocol_key: [core::ffi::c_char; 8 + 1],
    pub protocol_id: u32,
    pub protocol_version: u32,
    pub protocol_revision: u32,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
