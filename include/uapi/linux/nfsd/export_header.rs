/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/linux/nfsd/export.h
 *
 * Public declarations for NFS exports. The definitions for the
 * syscall interface are in nfsctl.h
 *
 * Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
 */

/* Important limits for the exports stuff. */
pub const NFSCLNT_IDMAX: u32 = 1024;
pub const NFSCLNT_ADDRMAX: u32 = 16;
pub const NFSCLNT_KEYMAX: u32 = 32;

/*
 * Export flags.
 *
 * Please update the expflags[] array in fs/nfsd/export.c when adding
 * a new flag.
 */
pub const NFSEXP_READONLY: u32 = 0x0001;
pub const NFSEXP_INSECURE_PORT: u32 = 0x0002;
pub const NFSEXP_ROOTSQUASH: u32 = 0x0004;
pub const NFSEXP_ALLSQUASH: u32 = 0x0008;
pub const NFSEXP_ASYNC: u32 = 0x0010;
pub const NFSEXP_GATHERED_WRITES: u32 = 0x0020;
pub const NFSEXP_NOREADDIRPLUS: u32 = 0x0040;
pub const NFSEXP_SECURITY_LABEL: u32 = 0x0080;
pub const NFSEXP_SIGN_FH: u32 = 0x0100;
pub const NFSEXP_NOHIDE: u32 = 0x0200;
pub const NFSEXP_NOSUBTREECHECK: u32 = 0x0400;
pub const NFSEXP_NOAUTHNLM: u32 = 0x0800; /* Don't authenticate NLM requests - just trust */
pub const NFSEXP_MSNFS: u32 = 0x1000; /* do silly things that MS clients expect; no longer supported */
pub const NFSEXP_FSID: u32 = 0x2000;
pub const NFSEXP_CROSSMOUNT: u32 = 0x4000;
pub const NFSEXP_NOACL: u32 = 0x8000; /* reserved for possible ACL related use */

/*
 * The NFSEXP_V4ROOT flag causes the kernel to give access only to NFSv4
 * clients, and only to the single directory that is the root of the
 * export; further lookup and readdir operations are treated as if every
 * subdirectory was a mountpoint, and ignored if they are not themselves
 * exported.  This is used by nfsd and mountd to construct the NFSv4
 * pseudofilesystem, which provides access only to paths leading to each
 * exported filesystem.
 */
pub const NFSEXP_V4ROOT: u32 = 0x10000;
pub const NFSEXP_PNFS: u32 = 0x20000;

/* All flags that we claim to support.  (Note we don't support NOACL.) */
pub const NFSEXP_ALLFLAGS: u32 = 0x3FFFF;

/* The flags that may vary depending on security flavor: */
pub const NFSEXP_SECINFO_FLAGS: u32 =
    NFSEXP_READONLY | NFSEXP_ROOTSQUASH | NFSEXP_ALLSQUASH | NFSEXP_INSECURE_PORT;

/*
 * Transport layer security policies that are permitted to access
 * an export
 */
pub const NFSEXP_XPRTSEC_NONE: u32 = 0x0001;
pub const NFSEXP_XPRTSEC_TLS: u32 = 0x0002;
pub const NFSEXP_XPRTSEC_MTLS: u32 = 0x0004;

pub const NFSEXP_XPRTSEC_NUM: u32 = 3;

pub const NFSEXP_XPRTSEC_ALL: u32 =
    NFSEXP_XPRTSEC_NONE | NFSEXP_XPRTSEC_TLS | NFSEXP_XPRTSEC_MTLS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
