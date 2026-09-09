/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
  File: linux/xattr.h

  Extended attributes handling.

  Copyright (C) 2001 by Andreas Gruenbacher <a.gruenbacher@computer.org>
  Copyright (c) 2001-2002 Silicon Graphics, Inc.  All Rights Reserved.
  Copyright (c) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
  Copyright (c) 2020 Jan (janneke) Nieuwenhuizen <janneke@gnu.org>
*/

// The contents guarded by __UAPI_DEF_XATTR are enabled when that build-time
// condition is true in the C header.
// __USE_KERNEL_XATTR_DEFS
pub const XATTR_CREATE: u32 = 0x1; // set value, fail if attr already exists
pub const XATTR_REPLACE: u32 = 0x2; // set value, fail if attr does not exist

#[repr(C)]
pub struct xattr_args {
    pub value: __aligned_u64,
    pub size: __u32,
    pub flags: __u32,
}

/* Namespaces */
pub const XATTR_OS2_PREFIX: &str = "os2.";
pub const XATTR_OS2_PREFIX_LEN: usize = XATTR_OS2_PREFIX.len();

pub const XATTR_MAC_OSX_PREFIX: &str = "osx.";
pub const XATTR_MAC_OSX_PREFIX_LEN: usize = XATTR_MAC_OSX_PREFIX.len();

pub const XATTR_BTRFS_PREFIX: &str = "btrfs.";
pub const XATTR_BTRFS_PREFIX_LEN: usize = XATTR_BTRFS_PREFIX.len();

pub const XATTR_HURD_PREFIX: &str = "gnu.";
pub const XATTR_HURD_PREFIX_LEN: usize = XATTR_HURD_PREFIX.len();

pub const XATTR_SECURITY_PREFIX: &str = "security.";
pub const XATTR_SECURITY_PREFIX_LEN: usize = XATTR_SECURITY_PREFIX.len();

pub const XATTR_SYSTEM_PREFIX: &str = "system.";
pub const XATTR_SYSTEM_PREFIX_LEN: usize = XATTR_SYSTEM_PREFIX.len();

pub const XATTR_TRUSTED_PREFIX: &str = "trusted.";
pub const XATTR_TRUSTED_PREFIX_LEN: usize = XATTR_TRUSTED_PREFIX.len();

pub const XATTR_USER_PREFIX: &str = "user.";
pub const XATTR_USER_PREFIX_LEN: usize = XATTR_USER_PREFIX.len();

/* Security namespace */
pub const XATTR_EVM_SUFFIX: &str = "evm";
pub const XATTR_NAME_EVM: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_EVM_SUFFIX);

pub const XATTR_IMA_SUFFIX: &str = "ima";
pub const XATTR_NAME_IMA: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_IMA_SUFFIX);

pub const XATTR_SELINUX_SUFFIX: &str = "selinux";
pub const XATTR_NAME_SELINUX: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SELINUX_SUFFIX);

pub const XATTR_SMACK_SUFFIX: &str = "SMACK64";
pub const XATTR_SMACK_IPIN: &str = "SMACK64IPIN";
pub const XATTR_SMACK_IPOUT: &str = "SMACK64IPOUT";
pub const XATTR_SMACK_EXEC: &str = "SMACK64EXEC";
pub const XATTR_SMACK_TRANSMUTE: &str = "SMACK64TRANSMUTE";
pub const XATTR_SMACK_MMAP: &str = "SMACK64MMAP";
pub const XATTR_NAME_SMACK: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_SUFFIX);
pub const XATTR_NAME_SMACKIPIN: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_IPIN);
pub const XATTR_NAME_SMACKIPOUT: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_IPOUT);
pub const XATTR_NAME_SMACKEXEC: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_EXEC);
pub const XATTR_NAME_SMACKTRANSMUTE: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_TRANSMUTE);
pub const XATTR_NAME_SMACKMMAP: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_SMACK_MMAP);

pub const XATTR_APPARMOR_SUFFIX: &str = "apparmor";
pub const XATTR_NAME_APPARMOR: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_APPARMOR_SUFFIX);

pub const XATTR_CAPS_SUFFIX: &str = "capability";
pub const XATTR_NAME_CAPS: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_CAPS_SUFFIX);

pub const XATTR_BPF_LSM_SUFFIX: &str = "bpf.";
pub const XATTR_NAME_BPF_LSM: &str = concat!(XATTR_SECURITY_PREFIX, XATTR_BPF_LSM_SUFFIX);
pub const XATTR_NAME_BPF_LSM_LEN: usize = XATTR_NAME_BPF_LSM.len();

pub const XATTR_POSIX_ACL_ACCESS: &str = "posix_acl_access";
pub const XATTR_NAME_POSIX_ACL_ACCESS: &str = concat!(XATTR_SYSTEM_PREFIX, XATTR_POSIX_ACL_ACCESS);
pub const XATTR_POSIX_ACL_DEFAULT: &str = "posix_acl_default";
pub const XATTR_NAME_POSIX_ACL_DEFAULT: &str = concat!(XATTR_SYSTEM_PREFIX, XATTR_POSIX_ACL_DEFAULT);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
