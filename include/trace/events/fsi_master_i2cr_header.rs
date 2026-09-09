/* SPDX-License-Identifier: GPL-2.0-or-later */

//! Rust translation of `trace/events/fsi_master_i2cr.h`.
//!
//! The Linux tracepoint registration and formatting machinery is supplied by
//! the surrounding kernel environment.  The declarations below preserve the
//! event payload layouts and the assignments performed by each trace event.

use core::ffi::c_void;

/// Opaque kernel I2C client.  The concrete definition is supplied externally.
#[repr(C)]
pub struct I2cClient {
    _private: [u8; 0],
}

/// Opaque trace event payload marker for external tracepoint integration.
#[repr(C)]
pub struct TraceEventContext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct I2crI2cErrorEntry {
    pub bus: i32,
    pub rc: i32,
    pub command: [u8; core::mem::size_of::<u32>()],
    pub addr: u16,
}

#[repr(C)]
pub struct I2crReadEntry {
    pub bus: i32,
    pub data: [u8; core::mem::size_of::<u64>()],
    pub command: [u8; core::mem::size_of::<u32>()],
    pub addr: u16,
}

#[repr(C)]
pub struct I2crStatusEntry {
    pub status: u64,
    pub bus: i32,
    pub addr: u16,
}

#[repr(C)]
pub struct I2crStatusErrorEntry {
    pub error: u64,
    pub log: u64,
    pub status: u64,
    pub bus: i32,
    pub addr: u16,
}

#[repr(C)]
pub struct I2crWriteEntry {
    pub bus: i32,
    pub data: [u8; core::mem::size_of::<u64>()],
    pub command: [u8; core::mem::size_of::<u32>()],
    pub addr: u16,
}

/// The following accessors correspond to `client->adapter->nr` and
/// `client->addr` in the kernel-provided `struct i2c_client`.
extern "C" {
    pub fn fsi_master_i2cr_client_bus(client: *const I2cClient) -> i32;
    pub fn fsi_master_i2cr_client_addr(client: *const I2cClient) -> u16;
}

#[inline]
pub unsafe fn i2cr_i2c_error(
    client: *const I2cClient,
    command: u32,
    rc: i32,
) -> I2crI2cErrorEntry {
    let mut entry = I2crI2cErrorEntry {
        bus: fsi_master_i2cr_client_bus(client),
        rc,
        command: [0; core::mem::size_of::<u32>()],
        addr: fsi_master_i2cr_client_addr(client),
    };
    entry.command.copy_from_slice(&command.to_ne_bytes());
    entry
}

#[inline]
pub unsafe fn i2cr_read(
    client: *const I2cClient,
    command: u32,
    data: *const u64,
) -> I2crReadEntry {
    let mut entry = I2crReadEntry {
        bus: fsi_master_i2cr_client_bus(client),
        data: [0; core::mem::size_of::<u64>()],
        command: [0; core::mem::size_of::<u32>()],
        addr: fsi_master_i2cr_client_addr(client),
    };
    core::ptr::copy_nonoverlapping(data as *const u8, entry.data.as_mut_ptr(), core::mem::size_of::<u64>());
    entry.command.copy_from_slice(&command.to_ne_bytes());
    entry
}

#[inline]
pub unsafe fn i2cr_status(client: *const I2cClient, status: u64) -> I2crStatusEntry {
    I2crStatusEntry {
        status,
        bus: fsi_master_i2cr_client_bus(client),
        addr: fsi_master_i2cr_client_addr(client),
    }
}

#[inline]
pub unsafe fn i2cr_status_error(
    client: *const I2cClient,
    status: u64,
    error: u64,
    log: u64,
) -> I2crStatusErrorEntry {
    I2crStatusErrorEntry {
        error,
        log,
        status,
        bus: fsi_master_i2cr_client_bus(client),
        addr: fsi_master_i2cr_client_addr(client),
    }
}

#[inline]
pub unsafe fn i2cr_write(
    client: *const I2cClient,
    command: u32,
    data: u64,
) -> I2crWriteEntry {
    let mut entry = I2crWriteEntry {
        bus: fsi_master_i2cr_client_bus(client),
        data: [0; core::mem::size_of::<u64>()],
        command: [0; core::mem::size_of::<u32>()],
        addr: fsi_master_i2cr_client_addr(client),
    };
    entry.data.copy_from_slice(&data.to_ne_bytes());
    entry.command.copy_from_slice(&command.to_ne_bytes());
    entry
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
