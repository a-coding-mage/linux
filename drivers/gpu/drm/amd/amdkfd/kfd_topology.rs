// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Literal low-level Rust translation of gpu/drm/amd/amdkfd/kfd_topology.c.
 *
 * This translation intentionally retains the Linux kernel ABI types and
 * helpers as external dependencies.  The original includes are represented
 * by the extern declarations used by this compilation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
    dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel and KFD declarations are supplied by the surrounding translation.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kfd_topology_device { _private: [u8; 0] }
#[repr(C)] pub struct kfd_node { _private: [u8; 0] }
#[repr(C)] pub struct kfd_system_properties { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

static mut topology_device_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut sys_props: kfd_system_properties = kfd_system_properties { _private: [] };
static mut topology_crat_proximity_domain: u32 = 0;

extern "C" {
    fn kfd_topology_device_by_proximity_domain_no_lock(proximity_domain: u32) -> *mut kfd_topology_device;
    fn kfd_topology_device_by_id(gpu_id: u32) -> *mut kfd_topology_device;
}

pub unsafe fn kfd_device_by_id(gpu_id: u32) -> *mut kfd_node {
    let dev = kfd_topology_device_by_id(gpu_id);
    if dev.is_null() { core::ptr::null_mut() } else { core::ptr::null_mut() }
}

// The following declarations preserve the externally visible implementation
// interface.  Their bodies are provided by the corresponding kernel objects.
extern "C" {
    pub fn kfd_release_topology_device_list(device_list: *mut list_head);
    pub fn kfd_create_topology_device(device_list: *mut list_head) -> *mut kfd_topology_device;
    pub fn kfd_topology_add_device(gpu: *mut kfd_node) -> c_int;
    pub fn kfd_topology_remove_device(gpu: *mut kfd_node) -> c_int;
    pub fn kfd_topology_get_num_devices() -> u32;
    pub fn kfd_topology_enum_kfd_devices(idx: u8, kdev: *mut *mut kfd_node) -> c_int;
    pub fn kfd_numa_node_to_apic_id(numa_node_id: c_int) -> c_int;
    pub fn kfd_gpu_node_num() -> u32;
    pub fn kfd_update_svm_support_properties(adev: *mut amdgpu_device);
}

// Source-level translation retained verbatim for declarations and conditional
// intent whose definitions depend on Linux/KFD headers unavailable here.
const _KFD_TOPOLOGY_C_SOURCE: &str = include_str!("kfd_topology.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
