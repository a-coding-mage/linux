/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 *  linux/include/linux/nfs4_mount.h
 *
 *  Copyright (C) 2002  Trond Myklebust
 *
 *  structure passed from user-space to kernel-space during an nfsv4 mount
 */

/*
 * WARNING!  Do not delete or change the order of these fields.  If
 * a new field is required then add it to the end.  The version field
 * tracks which fields are present.  This will ensure some measure of
 * mount-to-kernel version compatibility.  Some of these aren't used yet
 * but here they are anyway.
 */
pub const NFS4_MOUNT_VERSION: i32 = 1;

#[repr(C)]
pub struct nfs_string {
    pub len: u32,
    pub data: *const core::ffi::c_char,
}

#[repr(C)]
pub struct nfs4_mount_data {
    pub version: i32,       /* 1 */
    pub flags: i32,         /* 1 */
    pub rsize: i32,         /* 1 */
    pub wsize: i32,         /* 1 */
    pub timeo: i32,         /* 1 */
    pub retrans: i32,       /* 1 */
    pub acregmin: i32,      /* 1 */
    pub acregmax: i32,      /* 1 */
    pub acdirmin: i32,      /* 1 */
    pub acdirmax: i32,      /* 1 */

    /* see the definition of 'struct clientaddr4' in RFC3010 */
    pub client_addr: nfs_string, /* 1 */

    /* Mount path */
    pub mnt_path: nfs_string, /* 1 */

    /* Server details */
    pub hostname: nfs_string, /* 1 */
    /* Server IP address */
    pub host_addrlen: u32, /* 1 */
    pub host_addr: *mut sockaddr, /* 1 */

    /* Transport protocol to use */
    pub proto: i32, /* 1 */

    /* Pseudo-flavours to use for authentication. See RFC2623 */
    pub auth_flavourlen: i32, /* 1 */
    pub auth_flavours: *mut i32, /* 1 */
}

/* bits in the flags field */
/* Note: the fields that correspond to existing NFSv2/v3 mount options
 *      should mirror the values from include/linux/nfs_mount.h
 */

pub const NFS4_MOUNT_SOFT: i32 = 0x0001; /* 1 */
pub const NFS4_MOUNT_INTR: i32 = 0x0002; /* 1 */
pub const NFS4_MOUNT_NOCTO: i32 = 0x0010; /* 1 */
pub const NFS4_MOUNT_NOAC: i32 = 0x0020; /* 1 */
pub const NFS4_MOUNT_STRICTLOCK: i32 = 0x1000; /* 1 */
pub const NFS4_MOUNT_UNSHARED: i32 = 0x8000; /* 1 */
pub const NFS4_MOUNT_FLAGMASK: i32 = 0x9033;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
