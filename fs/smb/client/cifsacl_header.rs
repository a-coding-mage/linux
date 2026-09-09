/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */

// Dependency: ../common/smbacl.h

pub const READ_BIT: u32 = 0x4;
pub const WRITE_BIT: u32 = 0x2;
pub const EXEC_BIT: u32 = 0x1;

pub const ACL_OWNER_MASK: u32 = 0o700;
pub const ACL_GROUP_MASK: u32 = 0o070;
pub const ACL_EVERYONE_MASK: u32 = 0o007;

pub const UBITSHIFT: u32 = 6;
pub const GBITSHIFT: u32 = 3;

/*
 * Security Descriptor length containing DACL with 3 ACEs (one each for
 * owner, group and world).
 */
pub const DEFAULT_SEC_DESC_LEN: usize = core::mem::size_of::<smb_ntsd>()
    + core::mem::size_of::<smb_acl>()
    + (core::mem::size_of::<smb_ace>() * 4);

/*
 * The current SMB3 form of security descriptor is similar to what was used for
 * cifs (see above) but some fields are split, and fields in the struct below
 * matches names of fields to the spec, MS-DTYP (see sections 2.4.5 and
 * 2.4.6). Note that "CamelCase" fields are used in this struct in order to
 * match the MS-DTYP and MS-SMB2 specs which define the wire format.
 */
#[repr(C, packed)]
pub struct smb3_sd {
    pub Revision: u8, /* revision level, MUST be one */
    pub Sbz1: u8, /* only meaningful if 'RM' flag set below */
    pub Control: u16,
    pub OffsetOwner: u32,
    pub OffsetGroup: u32,
    pub OffsetSacl: u32,
    pub OffsetDacl: u32,
}

/* Meaning of 'Control' field flags */
pub const ACL_CONTROL_SR: u32 = 0x8000; /* Self relative */
pub const ACL_CONTROL_RM: u32 = 0x4000; /* Resource manager control bits */
pub const ACL_CONTROL_PS: u32 = 0x2000; /* SACL protected from inherits */
pub const ACL_CONTROL_PD: u32 = 0x1000; /* DACL protected from inherits */
pub const ACL_CONTROL_SI: u32 = 0x0800; /* SACL Auto-Inherited */
pub const ACL_CONTROL_DI: u32 = 0x0400; /* DACL Auto-Inherited */
pub const ACL_CONTROL_SC: u32 = 0x0200; /* SACL computed through inheritance */
pub const ACL_CONTROL_DC: u32 = 0x0100; /* DACL computed through inheritance */
pub const ACL_CONTROL_SS: u32 = 0x0080; /* Create server ACL */
pub const ACL_CONTROL_DT: u32 = 0x0040; /* DACL provided by trusted source */
pub const ACL_CONTROL_SD: u32 = 0x0020; /* SACL defaulted */
pub const ACL_CONTROL_SP: u32 = 0x0010; /* SACL is present on object */
pub const ACL_CONTROL_DD: u32 = 0x0008; /* DACL defaulted */
pub const ACL_CONTROL_DP: u32 = 0x0004; /* DACL is present on object */
pub const ACL_CONTROL_GD: u32 = 0x0002; /* Group was defaulted */
pub const ACL_CONTROL_OD: u32 = 0x0001; /* User was defaulted */

/* Meaning of AclRevision flags */
pub const ACL_REVISION: u8 = 0x02; /* See section 2.4.4.1 of MS-DTYP */
pub const ACL_REVISION_DS: u8 = 0x04; /* Additional AceTypes allowed */

#[repr(C, packed)]
pub struct smb3_acl {
    pub AclRevision: u8, /* revision level */
    pub Sbz1: u8, /* MBZ */
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16, /* MBZ */
}

/*
 * Used to store the special 'NFS SIDs' used to persist the POSIX uid and gid
 * See http://technet.microsoft.com/en-us/library/hh509017(v=ws.10).aspx
 */
#[repr(C, packed)]
pub struct owner_sid {
    pub Revision: u8,
    pub NumAuth: u8,
    pub Authority: [u8; 6],
    pub SubAuthorities: [u32; 3],
}

#[repr(C, packed)]
pub struct owner_group_sids {
    pub owner: owner_sid,
    pub group: owner_sid,
}

/*
 * Minimum security identifier can be one for system defined Users
 * and Groups such as NULL SID and World or Built-in accounts such
 * as Administrator and Guest and consists of
 * Revision + Num (Sub)Auths + Authority + Domain (one Subauthority)
 */
pub const MIN_SID_LEN: usize = 1 + 1 + 6 + 4; /* in bytes */

/*
 * Minimum security descriptor can be one without any SACL and DACL and can
 * consist of revision, type, and two sids of minimum size for owner and group
 */
pub const MIN_SEC_DESC_LEN: usize = core::mem::size_of::<smb_ntsd>() + (2 * MIN_SID_LEN);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
