/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Upcall description for nfsdcld communication
 *
 * Copyright (c) 2012 Red Hat, Inc.
 * Author(s): Jeff Layton <jlayton@redhat.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */

/* latest upcall version available */
pub const CLD_UPCALL_VERSION: i32 = 2;

/* defined by RFC3530 */
pub const NFS4_OPAQUE_LIMIT: i32 = 1024;

/* SHA256_DIGEST_SIZE is supplied by the surrounding environment when defined. */
pub const SHA256_DIGEST_SIZE: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cld_command {
	Cld_Create,    /* create a record for this cm_id */
	Cld_Remove,    /* remove record of this cm_id */
	Cld_Check,     /* is this cm_id allowed? */
	Cld_GraceDone, /* grace period is complete */
	Cld_GraceStart, /* grace start (upload client records) */
	Cld_GetVersion, /* query max supported upcall version */
}

/* representation of long-form NFSv4 client ID */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_name {
	pub cn_len: u16,                         /* length of cm_id */
	pub cn_id: [u8; NFS4_OPAQUE_LIMIT as usize], /* client-provided */
}

/* sha256 hash of the kerberos principal */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_princhash {
	pub cp_len: u8,                         /* length of cp_data */
	pub cp_data: [u8; SHA256_DIGEST_SIZE],  /* hash of principal */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_clntinfo {
	pub cc_name: cld_name,
	pub cc_princhash: cld_princhash,
}

/* message struct for communication with userspace */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_msg {
	pub cm_vers: u8,   /* upcall version */
	pub cm_cmd: u8,    /* upcall command */
	pub cm_status: i16, /* return code */
	pub cm_xid: u32,   /* transaction id */
	pub cm_u: cld_msg__bindgen_ty_1,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union cld_msg__bindgen_ty_1 {
	pub cm_gracetime: i64, /* grace period start time */
	pub cm_name: cld_name,
	pub cm_version: u8,    /* for getting max version */
}

/* version 2 message can include hash of kerberos principal */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_msg_v2 {
	pub cm_vers: u8,   /* upcall version */
	pub cm_cmd: u8,    /* upcall command */
	pub cm_status: i16, /* return code */
	pub cm_xid: u32,   /* transaction id */
	pub cm_u: cld_msg_v2__bindgen_ty_1,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union cld_msg_v2__bindgen_ty_1 {
	pub cm_name: cld_name,
	pub cm_version: u8,       /* for getting max version */
	pub cm_clntinfo: cld_clntinfo, /* name & princ hash */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct cld_msg_hdr {
	pub cm_vers: u8,   /* upcall version */
	pub cm_cmd: u8,    /* upcall command */
	pub cm_status: i16, /* return code */
	pub cm_xid: u32,   /* transaction id */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
