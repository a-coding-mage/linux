/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by other headers/build units. */
#[cfg(CONFIG_PROC_FS)]
pub struct net;

#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn vlan_proc_init(net: *mut net) -> ::core::ffi::c_int;
    pub fn vlan_proc_rem_dev(vlandev: *mut net_device);
    pub fn vlan_proc_add_dev(vlandev: *mut net_device) -> ::core::ffi::c_int;
    pub fn vlan_proc_cleanup(net: *mut net);
}

/* `net_device` is declared by the surrounding networking dependencies. */

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn vlan_proc_init(_net: *mut net) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn vlan_proc_cleanup(_net: *mut net) {}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn vlan_proc_add_dev(_dev: *mut net_device) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub unsafe fn vlan_proc_rem_dev(_dev: *mut net_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
