/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/ref_tracker.h.
pub enum ref_tracker {}

#[cfg(CONFIG_NET_DEV_REFCNT_TRACKER)]
pub type netdevice_tracker = *mut ref_tracker;

#[cfg(not(CONFIG_NET_DEV_REFCNT_TRACKER))]
#[repr(C)]
pub struct netdevice_tracker;

#[cfg(CONFIG_NET_NS_REFCNT_TRACKER)]
pub type netns_tracker = *mut ref_tracker;

#[cfg(not(CONFIG_NET_NS_REFCNT_TRACKER))]
#[repr(C)]
pub struct netns_tracker;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
