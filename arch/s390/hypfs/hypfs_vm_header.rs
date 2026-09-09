/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Hypervisor filesystem for Linux on s390. z/VM implementation.
 *
 *    Copyright IBM Corp. 2006
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// C header guard: _S390_HYPFS_VM_H_

pub const DIAG2FC_NAME_LEN: usize = 8;

#[repr(C)]
pub struct diag2fc_data {
    pub version: u32,
    pub flags: u32,
    pub used_cpu: u64,
    pub el_time: u64,
    pub mem_min_kb: u64,
    pub mem_max_kb: u64,
    pub mem_share_kb: u64,
    pub mem_used_kb: u64,
    pub pcpus: u32,
    pub lcpus: u32,
    pub vcpus: u32,
    pub ocpus: u32,
    pub cpu_max: u32,
    pub cpu_shares: u32,
    pub cpu_use_samp: u32,
    pub cpu_delay_samp: u32,
    pub page_wait_samp: u32,
    pub idle_samp: u32,
    pub other_samp: u32,
    pub total_samp: u32,
    pub guest_name: [core::ffi::c_char; DIAG2FC_NAME_LEN],
}

#[repr(C)]
pub struct diag2fc_parm_list {
    pub userid: [core::ffi::c_char; DIAG2FC_NAME_LEN],
    pub aci_grp: [core::ffi::c_char; DIAG2FC_NAME_LEN],
    pub addr: u64,
    pub size: u32,
    pub fmt: u32,
}

extern "C" {
    pub fn diag2fc_store(
        query: *mut core::ffi::c_char,
        count: *mut core::ffi::c_uint,
        offset: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    pub fn diag2fc_free(data: *const core::ffi::c_void);
    pub static mut diag2fc_guest_query: *mut core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
