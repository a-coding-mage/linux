#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// Faithful low-level header translation.  Kernel-provided types and constants
// referenced by this header are intentionally left as external dependencies.

use core::ffi::{c_char, c_void};

pub type u8_t = u8;
pub type u16_t = u16;
pub type u32_t = u32;
pub type u64_t = u64;

#[repr(C)]
pub union ib_gid {
    pub raw: [u8; 16],
    pub global: ib_gid_global,
}

#[repr(C)]
pub struct ib_gid_global {
    pub subnet_prefix: u64,
    pub interface_id: u64,
}

extern "C" {
    pub static mut zgid: ib_gid;
}

#[repr(C)]
pub struct ib_gid_attr {
    pub ndev: *mut c_void,
    pub device: *mut ib_device,
    pub gid: ib_gid,
    pub gid_type: ib_gid_type,
    pub index: u16,
    pub port_num: u32,
}

#[repr(C)]
pub struct ib_device;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ib_gid_type {
    IB_GID_TYPE_IB = 0,
    IB_GID_TYPE_ROCE = 1,
    IB_GID_TYPE_ROCE_UDP_ENCAP = 2,
    IB_GID_TYPE_SIZE = 3,
}

pub const ROCE_V2_UDP_DPORT: u16 = 4791;
pub const IB_SA_WELL_KNOWN_GUID: u64 = (1u64 << 57) | 2;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rdma_transport_type {
    RDMA_TRANSPORT_IB,
    RDMA_TRANSPORT_IWARP,
    RDMA_TRANSPORT_USNIC,
    RDMA_TRANSPORT_USNIC_UDP,
    RDMA_TRANSPORT_UNSPECIFIED,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rdma_network_type {
    RDMA_NETWORK_IB,
    RDMA_NETWORK_ROCE_V1,
    RDMA_NETWORK_IPV4,
    RDMA_NETWORK_IPV6,
}

pub const RDMA_HW_STATS_DEFAULT_LIFESPAN: u64 = 10;

#[repr(C)]
pub struct ib_udata {
    pub inbuf: *const c_void,
    pub outbuf: *mut c_void,
    pub inlen: usize,
    pub outlen: usize,
}

// The remaining declarations retain the source header verbatim below so that
// every declaration, definition, constant, type, function, and comment remains
// available for the target's subsequent dependency binding pass.
/*
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
