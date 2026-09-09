/* SPDX-License-Identifier: GPL-2.0-only */

// C header dependencies are supplied by the surrounding kernel translation.

pub const PSP_DEFAULT_UDP_PORT: u32 = 1000;

#[repr(C)]
pub struct psphdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub crypt_offset: u8,
    pub verfl: u8,
    pub spi: __be32,
    pub iv: __be64,
    pub vc: [__be64; 0], // optional
}

pub const PSP_ENCAP_HLEN: usize = core::mem::size_of::<udphdr>() + core::mem::size_of::<psphdr>();
pub const PSP_SPI_KEY_ID: u32 = 0x7fff_ffff;
pub const PSP_SPI_KEY_PHASE: u32 = 1u32 << 31;
pub const PSPHDR_CRYPT_OFFSET: u8 = 0x3f;
pub const PSPHDR_VERFL_SAMPLE: u8 = 1u8 << 7;
pub const PSPHDR_VERFL_DROP: u8 = 1u8 << 6;
pub const PSPHDR_VERFL_VERSION: u8 = 0x3c;
pub const PSPHDR_VERFL_VIRT: u8 = 1u8 << 1;
pub const PSPHDR_VERFL_ONE: u8 = 1;
pub const PSP_HDRLEN_NOOPT: usize = (core::mem::size_of::<psphdr>() - 8) / 8;

#[repr(C)]
pub struct psp_dev_config {
    pub versions: u32,
}

pub const PSP_ASSOC_DEV_MAX: usize = 128;

#[repr(C)]
pub struct psp_assoc_dev {
    pub dev_list: list_head,
    pub assoc_dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
}

#[repr(C)]
pub struct psp_dev {
    pub main_netdev: *mut net_device,
    pub assoc_dev_list: list_head,
    pub assoc_dev_cnt: core::ffi::c_int,
    pub ops: *mut psp_dev_ops,
    pub caps: *mut psp_dev_caps,
    pub drv_priv: *mut core::ffi::c_void,
    pub lock: mutex,
    pub refcnt: refcount_t,
    pub id: u32,
    pub generation: u8,
    pub config: psp_dev_config,
    pub active_assocs: list_head,
    pub prev_assocs: list_head,
    pub stale_assocs: list_head,
    pub stats: psp_dev_stats_core,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct psp_dev_stats_core {
    pub rotations: c_ulong,
    pub stales: c_ulong,
}

pub const PSP_GEN_VALID_MASK: u8 = 0x7f;

#[repr(C)]
pub struct psp_dev_caps {
    pub versions: u32,
    pub assoc_drv_spc: u32,
}

pub const PSP_MAX_KEY: usize = 32;
pub const PSP_HDR_SIZE: usize = 16;
pub const PSP_TRL_SIZE: usize = 16;

#[repr(C)]
pub struct psp_skb_ext {
    pub spi: __be32,
    pub dev_id: u16,
    pub generation: u8,
    pub version: u8,
}

#[repr(C)]
pub struct psp_key_parsed {
    pub spi: __be32,
    pub key: [u8; PSP_MAX_KEY],
}

#[repr(C)]
pub struct psp_assoc {
    pub psd: *mut psp_dev,
    pub dev_id: u16,
    pub generation: u8,
    pub version: u8,
    pub peer_tx: u8,
    pub upgrade_seq: u32,
    pub tx: psp_key_parsed,
    pub rx: psp_key_parsed,
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
    pub work: work_struct,
    pub assocs_list: list_head,
    pub drv_data: [u8; 0], // __aligned(8)
}

#[repr(C)]
pub struct psp_dev_stats_values {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_auth_fail: u64,
    pub rx_error: u64,
    pub rx_bad: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_error: u64,
}

#[repr(C)]
pub union psp_dev_stats_union {
    pub values: psp_dev_stats_values,
    pub required: [u64; 0],
}

#[repr(C)]
pub struct psp_dev_stats {
    pub stats: psp_dev_stats_union,
}

#[repr(C)]
pub struct psp_dev_ops {
    pub set_config: Option<unsafe extern "C" fn(*mut psp_dev, *mut psp_dev_config, *mut netlink_ext_ack) -> core::ffi::c_int>,
    pub key_rotate: Option<unsafe extern "C" fn(*mut psp_dev, *mut netlink_ext_ack) -> core::ffi::c_int>,
    pub rx_spi_alloc: Option<unsafe extern "C" fn(*mut psp_dev, u32, *mut psp_key_parsed, *mut netlink_ext_ack) -> core::ffi::c_int>,
    pub tx_key_add: Option<unsafe extern "C" fn(*mut psp_dev, *mut psp_assoc, *mut netlink_ext_ack) -> core::ffi::c_int>,
    pub tx_key_del: Option<unsafe extern "C" fn(*mut psp_dev, *mut psp_assoc)>,
    pub get_stats: Option<unsafe extern "C" fn(*mut psp_dev, *mut psp_dev_stats)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
