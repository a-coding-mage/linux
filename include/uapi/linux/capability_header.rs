/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This is <linux/capability.h>
 *
 * Andrew G. Morgan <morgan@kernel.org>
 * Alexander Kjeldaas <astor@guardian.no>
 * with help from Aleph1, Roland Buresund and Andrew Main.
 *
 * See here for the libcap2 library (compliant with Section 25 of
 * the withdrawn POSIX 1003.1e Draft 17):
 *
 * https://www.kernel.org/pub/linux/libs/security/linux-privs/libcap2/
 */

/* User-level do most of the mapping between kernel and user
   capabilities based on the version tag given by the kernel. The
   kernel might be somewhat backwards compatible, but don't bet on
   it. */

/* Note, cap_t, is defined by POSIX (draft) to be an "opaque" pointer to
   a set of three capability sets.  The transposition of 3*the
   following structure to such a composite is better handled in a user
   library since the draft standard requires the use of malloc/free
   etc.. */

pub const _LINUX_CAPABILITY_VERSION_1: u32 = 0x19980330;
pub const _LINUX_CAPABILITY_U32S_1: u32 = 1;

pub const _LINUX_CAPABILITY_VERSION_2: u32 = 0x20071026; /* deprecated - use v3 */
pub const _LINUX_CAPABILITY_U32S_2: u32 = 2;

pub const _LINUX_CAPABILITY_VERSION_3: u32 = 0x20080522;
pub const _LINUX_CAPABILITY_U32S_3: u32 = 2;

#[repr(C)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: i32,
}
pub type cap_user_header_t = *mut __user_cap_header_struct;

#[repr(C)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
pub type cap_user_data_t = *mut __user_cap_data_struct;

pub const VFS_CAP_REVISION_MASK: u32 = 0xFF000000;
pub const VFS_CAP_REVISION_SHIFT: u32 = 24;
pub const VFS_CAP_FLAGS_MASK: u32 = !VFS_CAP_REVISION_MASK;
pub const VFS_CAP_FLAGS_EFFECTIVE: u32 = 0x000001;

pub const VFS_CAP_REVISION_1: u32 = 0x01000000;
pub const VFS_CAP_U32_1: usize = 1;
pub const XATTR_CAPS_SZ_1: usize = core::mem::size_of::<__le32>() * (1 + 2 * VFS_CAP_U32_1);

pub const VFS_CAP_REVISION_2: u32 = 0x02000000;
pub const VFS_CAP_U32_2: usize = 2;
pub const XATTR_CAPS_SZ_2: usize = core::mem::size_of::<__le32>() * (1 + 2 * VFS_CAP_U32_2);

pub const VFS_CAP_REVISION_3: u32 = 0x03000000;
pub const VFS_CAP_U32_3: usize = 2;
pub const XATTR_CAPS_SZ_3: usize = core::mem::size_of::<__le32>() * (2 + 2 * VFS_CAP_U32_3);

pub const XATTR_CAPS_SZ: usize = XATTR_CAPS_SZ_3;
pub const VFS_CAP_U32: usize = VFS_CAP_U32_3;
pub const VFS_CAP_REVISION: u32 = VFS_CAP_REVISION_3;

#[repr(C)]
pub struct vfs_cap_data {
    pub magic_etc: __le32, /* Little endian */
    pub data: [vfs_cap_data_data; VFS_CAP_U32],
}

#[repr(C)]
pub struct vfs_cap_data_data {
    pub permitted: __le32, /* Little endian */
    pub inheritable: __le32, /* Little endian */
}

/*
 * same as vfs_cap_data but with a rootid at the end
 */
#[repr(C)]
pub struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_ns_cap_data_data; VFS_CAP_U32],
    pub rootid: __le32,
}

#[repr(C)]
pub struct vfs_ns_cap_data_data {
    pub permitted: __le32,
    pub inheritable: __le32,
}

/*
 * Backwardly compatible definition for source code - trapped in a
 * 32-bit world. If you find you need this, please consider using
 * libcap to untrap yourself...
 */
pub const _LINUX_CAPABILITY_VERSION: u32 = _LINUX_CAPABILITY_VERSION_1;
pub const _LINUX_CAPABILITY_U32S: u32 = _LINUX_CAPABILITY_U32S_1;

/* POSIX-draft defined capabilities. */
pub const CAP_CHOWN: i32 = 0;
pub const CAP_DAC_OVERRIDE: i32 = 1;
pub const CAP_DAC_READ_SEARCH: i32 = 2;
pub const CAP_FOWNER: i32 = 3;
pub const CAP_FSETID: i32 = 4;
pub const CAP_KILL: i32 = 5;
pub const CAP_SETGID: i32 = 6;
pub const CAP_SETUID: i32 = 7;

/* Linux-specific capabilities */
pub const CAP_SETPCAP: i32 = 8;
pub const CAP_LINUX_IMMUTABLE: i32 = 9;
pub const CAP_NET_BIND_SERVICE: i32 = 10;
pub const CAP_NET_BROADCAST: i32 = 11;
pub const CAP_NET_ADMIN: i32 = 12;
pub const CAP_NET_RAW: i32 = 13;
pub const CAP_IPC_LOCK: i32 = 14;
pub const CAP_IPC_OWNER: i32 = 15;
pub const CAP_SYS_MODULE: i32 = 16;
pub const CAP_SYS_RAWIO: i32 = 17;
pub const CAP_SYS_CHROOT: i32 = 18;
pub const CAP_SYS_PTRACE: i32 = 19;
pub const CAP_SYS_PACCT: i32 = 20;
pub const CAP_SYS_ADMIN: i32 = 21;
pub const CAP_SYS_BOOT: i32 = 22;
pub const CAP_SYS_NICE: i32 = 23;
pub const CAP_SYS_RESOURCE: i32 = 24;
pub const CAP_SYS_TIME: i32 = 25;
pub const CAP_SYS_TTY_CONFIG: i32 = 26;
pub const CAP_MKNOD: i32 = 27;
pub const CAP_LEASE: i32 = 28;
pub const CAP_AUDIT_WRITE: i32 = 29;
pub const CAP_AUDIT_CONTROL: i32 = 30;
pub const CAP_SETFCAP: i32 = 31;
pub const CAP_MAC_OVERRIDE: i32 = 32;
pub const CAP_MAC_ADMIN: i32 = 33;
pub const CAP_SYSLOG: i32 = 34;
pub const CAP_WAKE_ALARM: i32 = 35;
pub const CAP_BLOCK_SUSPEND: i32 = 36;
pub const CAP_AUDIT_READ: i32 = 37;
pub const CAP_PERFMON: i32 = 38;
pub const CAP_BPF: i32 = 39;
pub const CAP_CHECKPOINT_RESTORE: i32 = 40;

pub const CAP_LAST_CAP: i32 = CAP_CHECKPOINT_RESTORE;

#[inline]
pub const fn cap_valid(x: i32) -> bool {
    x >= 0 && x <= CAP_LAST_CAP
}

/*
 * Bit location of each capability (used by user-space library and kernel)
 */

#[inline]
pub const fn CAP_TO_INDEX(x: u32) -> u32 {
    x >> 5 /* 1 << 5 == bits in __u32 */
}

#[inline]
pub const fn CAP_TO_MASK(x: u32) -> u32 {
    1u32 << (x & 31) /* mask for indexed __u32 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
