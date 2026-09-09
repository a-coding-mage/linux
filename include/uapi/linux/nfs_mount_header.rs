/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 *  linux/include/linux/nfs_mount.h
 *
 *  Copyright (C) 1992  Rick Sladkey
 *
 *  structure passed from user-space to kernel-space during an nfs mount
 */
// C dependencies: linux/in.h, linux/nfs.h, linux/nfs2.h, linux/nfs3.h.

/*
 * WARNING!  Do not delete or change the order of these fields.  If
 * a new field is required then add it to the end.  The version field
 * tracks which fields are present.  This will ensure some measure of
 * mount-to-kernel version compatibility.  Some of these aren't used yet
 * but here they are anyway.
 */
pub const NFS_MOUNT_VERSION: i32 = 6;
pub const NFS_MAX_CONTEXT_LEN: usize = 256;

#[repr(C)]
pub struct nfs_mount_data {
    pub version: i32,       /* 1 */
    pub fd: i32,            /* 1 */
    pub old_root: nfs2_fh,  /* 1 */
    pub flags: i32,         /* 1 */
    pub rsize: i32,         /* 1 */
    pub wsize: i32,         /* 1 */
    pub timeo: i32,         /* 1 */
    pub retrans: i32,       /* 1 */
    pub acregmin: i32,      /* 1 */
    pub acregmax: i32,      /* 1 */
    pub acdirmin: i32,      /* 1 */
    pub acdirmax: i32,      /* 1 */
    pub addr: sockaddr_in,  /* 1 */
    pub hostname: [i8; NFS_MAXNAMLEN + 1], /* 1 */
    pub namlen: i32,        /* 2 */
    pub bsize: u32,         /* 3 */
    pub root: nfs3_fh,      /* 4 */
    pub pseudoflavor: i32,  /* 5 */
    pub context: [i8; NFS_MAX_CONTEXT_LEN + 1], /* 6 */
}

/* bits in the flags field visible to user space */

pub const NFS_MOUNT_SOFT: i32 = 0x0001;       /* 1 */
pub const NFS_MOUNT_INTR: i32 = 0x0002;       /* 1, now unused, but ABI */
pub const NFS_MOUNT_SECURE: i32 = 0x0004;     /* 1 */
pub const NFS_MOUNT_POSIX: i32 = 0x0008;      /* 1 */
pub const NFS_MOUNT_NOCTO: i32 = 0x0010;      /* 1 */
pub const NFS_MOUNT_NOAC: i32 = 0x0020;       /* 1 */
pub const NFS_MOUNT_TCP: i32 = 0x0040;        /* 2 */
pub const NFS_MOUNT_VER3: i32 = 0x0080;       /* 3 */
pub const NFS_MOUNT_KERBEROS: i32 = 0x0100;   /* 3 */
pub const NFS_MOUNT_NONLM: i32 = 0x0200;      /* 3 */
pub const NFS_MOUNT_BROKEN_SUID: i32 = 0x0400; /* 4 */
pub const NFS_MOUNT_NOACL: i32 = 0x0800;      /* 4 */
pub const NFS_MOUNT_STRICTLOCK: i32 = 0x1000; /* reserved for NFSv4 */
pub const NFS_MOUNT_SECFLAVOUR: i32 = 0x2000; /* 5 non-text parsed mount data only */
pub const NFS_MOUNT_NORDIRPLUS: i32 = 0x4000; /* 5 */
pub const NFS_MOUNT_UNSHARED: i32 = 0x8000;   /* 5 */
pub const NFS_MOUNT_FLAGMASK: i32 = 0xFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
