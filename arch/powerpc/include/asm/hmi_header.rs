/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Hypervisor Maintenance Interrupt header file.
 *
 * Copyright 2015 IBM Corporation
 * Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
 */

/* Translated from the C header. */

#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
pub const CORE_TB_RESYNC_REQ_BIT: u32 = 63;

#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
pub const MAX_SUBCORE_PER_CORE: usize = 4;

/*
 * sibling_subcore_state structure is used to co-ordinate all threads
 * during HMI to avoid TB corruption. This structure is allocated once
 * per each core and shared by all threads on that core.
 */
#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
#[repr(C)]
pub struct sibling_subcore_state {
    pub flags: core::ffi::c_ulong,
    pub in_guest: [u8; MAX_SUBCORE_PER_CORE],
}

#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
unsafe extern "C" {
    pub fn wait_for_subcore_guest_exit();
    pub fn wait_for_tb_resync();
}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_POSSIBLE))]
#[inline]
pub fn wait_for_subcore_guest_exit() {}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_POSSIBLE))]
#[inline]
pub fn wait_for_tb_resync() {}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn hmi_handle_debugtrig(regs: *mut pt_regs) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
