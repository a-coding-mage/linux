/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on policycap.h for __POLICYDB_CAP_MAX. */

/* clang-format off */
/* Policy capability names */
pub static selinux_policycap_names: [*const core::ffi::c_char; __POLICYDB_CAP_MAX as usize] = [
    b"network_peer_controls\0".as_ptr() as *const core::ffi::c_char,
    b"open_perms\0".as_ptr() as *const core::ffi::c_char,
    b"extended_socket_class\0".as_ptr() as *const core::ffi::c_char,
    b"always_check_network\0".as_ptr() as *const core::ffi::c_char,
    b"cgroup_seclabel\0".as_ptr() as *const core::ffi::c_char,
    b"nnp_nosuid_transition\0".as_ptr() as *const core::ffi::c_char,
    b"genfs_seclabel_symlinks\0".as_ptr() as *const core::ffi::c_char,
    b"ioctl_skip_cloexec\0".as_ptr() as *const core::ffi::c_char,
    b"userspace_initial_context\0".as_ptr() as *const core::ffi::c_char,
    b"netlink_xperm\0".as_ptr() as *const core::ffi::c_char,
    b"netif_wildcard\0".as_ptr() as *const core::ffi::c_char,
    b"genfs_seclabel_wildcard\0".as_ptr() as *const core::ffi::c_char,
    b"functionfs_seclabel\0".as_ptr() as *const core::ffi::c_char,
    b"memfd_class\0".as_ptr() as *const core::ffi::c_char,
    b"bpf_token_perms\0".as_ptr() as *const core::ffi::c_char,
];
/* clang-format on */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
