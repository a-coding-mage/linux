/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 */

pub const SMBD_DEFAULT_IOSIZE: usize = 8 * 1024 * 1024;
pub const SMBD_MIN_IOSIZE: usize = 512 * 1024;
pub const SMBD_MAX_IOSIZE: usize = 16 * 1024 * 1024;

// Opaque types supplied by the surrounding kernel translation.
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksmbd_transport {
    _private: [u8; 0],
}

// CONFIG_SMB_SERVER_SMBDIRECT selects the external implementations.
#[cfg(feature = "CONFIG_SMB_SERVER_SMBDIRECT")]
unsafe extern "C" {
    pub fn ksmbd_rdma_init() -> i32;
    pub fn ksmbd_rdma_stop_listening();
    pub fn ksmbd_rdma_enabled() -> bool;
    pub fn ksmbd_rdma_capable_netdev(netdev: *mut net_device) -> bool;
    pub fn init_smbd_max_io_size(sz: u32);
    pub fn get_smbd_max_read_write_size(kt: *mut ksmbd_transport) -> u32;
}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn ksmbd_rdma_init() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn ksmbd_rdma_stop_listening() {}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn ksmbd_rdma_enabled() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn ksmbd_rdma_capable_netdev(_netdev: *mut net_device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn init_smbd_max_io_size(_sz: u32) {}

#[cfg(not(feature = "CONFIG_SMB_SERVER_SMBDIRECT"))]
#[inline]
pub fn get_smbd_max_read_write_size(_kt: *mut ksmbd_transport) -> u32 {
    0
}

// Dependency intent preserved from: #include <linux/smbdirect.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
