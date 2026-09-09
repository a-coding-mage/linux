/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Isovalent */

// Dependency supplied by the Linux BPF bindings:
// use crate::{bpf_attr, bpf_prog, net_device, EINVAL};

#[cfg(CONFIG_NETKIT)]
extern "C" {
    pub fn netkit_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn netkit_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn netkit_prog_detach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32;
    pub fn netkit_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn netkit_peer_dev(dev: *mut net_device) -> *mut net_device;
}

#[cfg(not(CONFIG_NETKIT))]
#[inline]
pub unsafe fn netkit_prog_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(CONFIG_NETKIT))]
#[inline]
pub unsafe fn netkit_link_attach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(CONFIG_NETKIT))]
#[inline]
pub unsafe fn netkit_prog_detach(_attr: *const bpf_attr, _prog: *mut bpf_prog) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(CONFIG_NETKIT))]
#[inline]
pub unsafe fn netkit_prog_query(_attr: *const bpf_attr, _uattr: *mut bpf_attr) -> i32 {
    -(EINVAL as i32)
}

#[cfg(not(CONFIG_NETKIT))]
#[inline]
pub unsafe fn netkit_peer_dev(_dev: *mut net_device) -> *mut net_device {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
