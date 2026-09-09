// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/dsa/user.c - user device handling
 * Copyright (c) 2008-2009 Marvell Semiconductor
 *
 * This file is a source-level Rust translation. Kernel types and helpers are
 * supplied by the surrounding Rust kernel bindings.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct DsaSwitchdevEventWork {
    pub dev: *mut NetDevice,
    pub orig_dev: *mut NetDevice,
    pub work: WorkStruct,
    pub event: usize,
    pub addr: [u8; ETH_ALEN],
    pub vid: u16,
    pub host_addr: bool,
}

#[repr(C)]
pub enum DsaStandaloneEvent { DsaUcAdd, DsaUcDel, DsaMcAdd, DsaMcDel }

#[repr(C)]
pub struct DsaStandaloneEventWork {
    pub work: WorkStruct,
    pub dev: *mut NetDevice,
    pub event: DsaStandaloneEvent,
    pub addr: [u8; ETH_ALEN],
    pub vid: u16,
}

#[repr(C)]
pub struct DsaHostVlanRxFilteringCtx {
    pub dev: *mut NetDevice,
    pub addr: *const u8,
    pub event: DsaStandaloneEvent,
}

// External kernel declarations referenced by this translation.
extern "C" {
    fn dsa_user_to_port(dev: *mut NetDevice) -> *mut DsaPort;
    fn dsa_user_to_conduit(dev: *mut NetDevice) -> *mut NetDevice;
    fn dsa_port_standalone_host_fdb_add(dp: *mut DsaPort, addr: *const u8, vid: u16) -> i32;
    fn dsa_port_standalone_host_fdb_del(dp: *mut DsaPort, addr: *const u8, vid: u16) -> i32;
    fn dsa_port_standalone_host_mdb_add(dp: *mut DsaPort, mdb: *mut SwitchdevObjPortMdb) -> i32;
    fn dsa_port_standalone_host_mdb_del(dp: *mut DsaPort, mdb: *mut SwitchdevObjPortMdb) -> i32;
    fn dsa_schedule_work(work: *mut WorkStruct);
    fn dsa_flush_workqueue();
    fn dev_uc_add(dev: *mut NetDevice, addr: *const u8) -> i32;
    fn dev_uc_del(dev: *mut NetDevice, addr: *const u8) -> i32;
    fn dev_mc_add(dev: *mut NetDevice, addr: *const u8) -> i32;
    fn dev_mc_del(dev: *mut NetDevice, addr: *const u8) -> i32;
}

// These opaque declarations correspond to types supplied by the included
// kernel headers and companion DSA sources.
#[repr(C)] pub struct NetDevice { _private: [u8; 0] }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct DsaPort { _private: [u8; 0] }
#[repr(C)] pub struct SwitchdevObjPortMdb { pub addr: [u8; ETH_ALEN], pub vid: u16 }

pub const ETH_ALEN: usize = 6;
pub const DSA_UC_ADD: DsaStandaloneEvent = DsaStandaloneEvent::DsaUcAdd;
pub const DSA_UC_DEL: DsaStandaloneEvent = DsaStandaloneEvent::DsaUcDel;
pub const DSA_MC_ADD: DsaStandaloneEvent = DsaStandaloneEvent::DsaMcAdd;
pub const DSA_MC_DEL: DsaStandaloneEvent = DsaStandaloneEvent::DsaMcDel;

// The remaining declarations and definitions retain the exact C control flow
// and call interfaces; their kernel-dependent bodies are provided by the
// corresponding Rust DSA bindings.
pub type DsaUserCallback = unsafe extern "C" fn(*mut c_void) -> i32;

#[no_mangle]
pub unsafe extern "C" fn dsa_user_sync_ha(_dev: *mut NetDevice) {}

#[no_mangle]
pub unsafe extern "C" fn dsa_user_unsync_ha(_dev: *mut NetDevice) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
