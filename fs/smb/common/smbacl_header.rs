/* SPDX-License-Identifier: LGPL-2.1+ */
/*
 *   Copyright (c) International Business Machines  Corp., 2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *   Modified by Namjae Jeon (linkinjeon@kernel.org)
 */

// Translated from smbacl.h. C header guards and includes are omitted.

pub const NUM_AUTHS: usize = 6; // number of authority fields
pub const SID_MAX_SUB_AUTHORITIES: usize = 15; // max number of sub authority fields

/* ACE types - see MS-DTYP 2.4.4.1 */
pub const ACCESS_ALLOWED_ACE_TYPE: u8 = 0x00;
pub const ACCESS_DENIED_ACE_TYPE: u8 = 0x01;
pub const SYSTEM_AUDIT_ACE_TYPE: u8 = 0x02;
pub const SYSTEM_ALARM_ACE_TYPE: u8 = 0x03;
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: u8 = 0x04;
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: u8 = 0x05;
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: u8 = 0x06;
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: u8 = 0x07;
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: u8 = 0x08;
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: u8 = 0x09;
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: u8 = 0x0A;
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: u8 = 0x0B;
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: u8 = 0x0C;
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: u8 = 0x0D;
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: u8 = 0x0E; // Reserved
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: u8 = 0x0F;
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: u8 = 0x10; // reserved
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: u8 = 0x11;
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: u8 = 0x12;
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: u8 = 0x13;

/* ACE flags */
pub const OBJECT_INHERIT_ACE: u8 = 0x01;
pub const CONTAINER_INHERIT_ACE: u8 = 0x02;
pub const NO_PROPAGATE_INHERIT_ACE: u8 = 0x04;
pub const INHERIT_ONLY_ACE: u8 = 0x08;
pub const INHERITED_ACE: u8 = 0x10;
pub const SUCCESSFUL_ACCESS_ACE_FLAG: u8 = 0x40;
pub const FAILED_ACCESS_ACE_FLAG: u8 = 0x80;

/*
 * Maximum size of a string representation of a SID:
 *
 * The fields are unsigned values in decimal. So:
 *
 * u8:  max 3 bytes in decimal
 * u32: max 10 bytes in decimal
 *
 * "S-" + 3 bytes for version field + 15 for authority field + NULL terminator
 *
 * For authority field, max is when all 6 values are non-zero and it must be
 * represented in hex. So "-0x" + 12 hex digits.
 *
 * Add 11 bytes for each subauthority field (10 bytes each + 1 for '-')
 */
pub const SID_STRING_BASE_SIZE: usize = 2 + 3 + 15 + 1;
pub const SID_STRING_SUBAUTH_SIZE: usize = 11; // size of a single subauth string

// Requires the external C-equivalent little-endian conversion supplied by the consumer.
pub const DOMAIN_USER_RID_LE: u32 = cpu_to_le32(513);

/* ACE types - see MS-DTYP 2.4.4.1 */
pub const ACCESS_ALLOWED: i32 = 0;
pub const ACCESS_DENIED: i32 = 1;

/* Security ID types */
pub const SIDOWNER: i32 = 1;
pub const SIDGROUP: i32 = 2;
pub const SIDCREATOR_OWNER: i32 = 3;
pub const SIDCREATOR_GROUP: i32 = 4;
pub const SIDUNIX_USER: i32 = 5;
pub const SIDUNIX_GROUP: i32 = 6;
pub const SIDNFS_USER: i32 = 7;
pub const SIDNFS_GROUP: i32 = 8;
pub const SIDNFS_MODE: i32 = 9;

#[repr(C, packed)]
pub struct smb_ntsd {
    pub revision: u16, // revision level
    pub type_: u16,
    pub osidoffset: u32,
    pub gsidoffset: u32,
    pub sacloffset: u32,
    pub dacloffset: u32,
}

#[repr(C, packed)]
pub struct smb_sid {
    pub revision: u8, // revision level
    pub num_subauth: u8,
    pub authority: [u8; NUM_AUTHS],
    pub sub_auth: [u32; SID_MAX_SUB_AUTHORITIES], // sub_auth[num_subauth]
}

/* size of a struct smb_sid, sans sub_auth array */
pub const CIFS_SID_BASE_SIZE: usize = 1 + 1 + NUM_AUTHS;

#[repr(C, packed)]
pub struct smb_acl {
    pub revision: u16, // revision level
    pub size: u16,
    pub num_aces: u16,
    pub reserved: u16,
}

#[repr(C, packed)]
pub struct smb_ace {
    pub type_: u8, // see above and MS-DTYP 2.4.4.1
    pub flags: u8,
    pub size: u16,
    pub access_req: u32,
    pub sid: smb_sid, // ie UUID of user or group who gets these perms
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
