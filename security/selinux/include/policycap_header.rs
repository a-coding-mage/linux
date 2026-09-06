/* SPDX-License-Identifier: GPL-2.0 */

/* Policy capabilities */
pub const POLICYDB_CAP_NETPEER: u32 = 0;
pub const POLICYDB_CAP_OPENPERM: u32 = 1;
pub const POLICYDB_CAP_EXTSOCKCLASS: u32 = 2;
pub const POLICYDB_CAP_ALWAYSNETWORK: u32 = 3;
pub const POLICYDB_CAP_CGROUPSECLABEL: u32 = 4;
pub const POLICYDB_CAP_NNP_NOSUID_TRANSITION: u32 = 5;
pub const POLICYDB_CAP_GENFS_SECLABEL_SYMLINKS: u32 = 6;
pub const POLICYDB_CAP_IOCTL_SKIP_CLOEXEC: u32 = 7;
pub const POLICYDB_CAP_USERSPACE_INITIAL_CONTEXT: u32 = 8;
pub const POLICYDB_CAP_NETLINK_XPERM: u32 = 9;
pub const POLICYDB_CAP_NETIF_WILDCARD: u32 = 10;
pub const POLICYDB_CAP_GENFS_SECLABEL_WILDCARD: u32 = 11;
pub const POLICYDB_CAP_FUNCTIONFS_SECLABEL: u32 = 12;
pub const POLICYDB_CAP_MEMFD_CLASS: u32 = 13;
pub const POLICYDB_CAP_BPF_TOKEN_PERMS: u32 = 14;
pub const __POLICYDB_CAP_MAX: u32 = 15;

pub const POLICYDB_CAP_MAX: u32 = __POLICYDB_CAP_MAX - 1;

unsafe extern "C" {
    pub static selinux_policycap_names: [*const core::ffi::c_char; __POLICYDB_CAP_MAX as usize];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
