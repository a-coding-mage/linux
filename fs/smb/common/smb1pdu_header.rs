/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2002,2009
 *                 2018 Samsung Electronics Co., Ltd.
 *   Author(s): Steve French <sfrench@us.ibm.com>
 *              Namjae Jeon <linkinjeon@kernel.org>
 *
 */

#![allow(non_snake_case, non_camel_case_types, dead_code)]

// The C header includes dependencies supplying __u8, __u16, __u32, __le16,
// and __le32.  Those names remain external dependencies of this translation.

// Equivalent to cpu_to_le32(0x424d53ff).
pub const SMB1_PROTO_NUMBER: u32 = u32::to_le(0x424d53ff);

/*
 * See MS-CIFS 2.2.3.1
 *     MS-SMB 2.2.3.1
 */
#[repr(C, packed)]
pub struct smb_hdr {
    pub Protocol: [__u8; 4],
    pub Command: __u8,
    pub Status: smb_hdr_Status,
    pub Flags: __u8,
    pub Flags2: __le16, // note: le
    pub PidHigh: __le16,
    pub Signature: smb_hdr_Signature,
    pub pad: [__u8; 2],
    pub Tid: __u16,
    pub Pid: __le16,
    pub Uid: __u16,
    pub Mid: __le16,
    pub WordCount: __u8,
}

#[repr(C, packed)]
pub union smb_hdr_Status {
    pub DosError: smb_hdr_Status_DosError,
    pub CifsError: __le32,
}

#[repr(C, packed)]
pub struct smb_hdr_Status_DosError {
    pub ErrorClass: __u8,
    pub Reserved: __u8,
    pub Error: __le16,
}

#[repr(C, packed)]
pub union smb_hdr_Signature {
    pub Sequence: smb_hdr_Signature_Sequence,
    pub SecuritySignature: [__u8; 8],
}

#[repr(C, packed)]
pub struct smb_hdr_Signature_Sequence {
    pub SequenceNumber: __le32, // le
    pub Reserved: __u32,         // zero
}

/* See MS-CIFS 2.2.4.52.1 */
#[repr(C, packed)]
pub struct smb_negotiate_req {
    pub hdr: smb_hdr, // wct = 0
    pub ByteCount: __le16,
    pub DialectsArray: [core::ffi::c_uchar; 0],
}

pub type SMB_NEGOTIATE_REQ = smb_negotiate_req;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
