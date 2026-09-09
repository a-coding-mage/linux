/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2002,2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 */

pub const NTLMSSP_SIGNATURE: &[u8; 7] = b"NTLMSSP";

/* Message Types */
pub const NtLmNegotiate: u32 = 1;
pub const NtLmChallenge: u32 = 2;
pub const NtLmAuthenticate: u32 = 3;
pub const UnknownMessage: u32 = 8;

/* Negotiate Flags */
pub const NTLMSSP_NEGOTIATE_UNICODE: u32 = 0x01; /* Text strings are unicode */
pub const NTLMSSP_NEGOTIATE_OEM: u32 = 0x02; /* Text strings are in OEM */
pub const NTLMSSP_REQUEST_TARGET: u32 = 0x04; /* Srv returns its auth realm */
/* define reserved9                       0x08 */
pub const NTLMSSP_NEGOTIATE_SIGN: u32 = 0x0010; /* Request signing capability */
pub const NTLMSSP_NEGOTIATE_SEAL: u32 = 0x0020; /* Request confidentiality */
pub const NTLMSSP_NEGOTIATE_DGRAM: u32 = 0x0040;
pub const NTLMSSP_NEGOTIATE_LM_KEY: u32 = 0x0080; /* Use LM session key */
/* defined reserved 8                   0x0100 */
pub const NTLMSSP_NEGOTIATE_NTLM: u32 = 0x0200; /* NTLM authentication */
pub const NTLMSSP_NEGOTIATE_NT_ONLY: u32 = 0x0400; /* Lanman not allowed */
pub const NTLMSSP_ANONYMOUS: u32 = 0x0800;
pub const NTLMSSP_NEGOTIATE_DOMAIN_SUPPLIED: u32 = 0x1000; /* reserved6 */
pub const NTLMSSP_NEGOTIATE_WORKSTATION_SUPPLIED: u32 = 0x2000;
pub const NTLMSSP_NEGOTIATE_LOCAL_CALL: u32 = 0x4000; /* client/server same machine */
pub const NTLMSSP_NEGOTIATE_ALWAYS_SIGN: u32 = 0x8000; /* Sign. All security levels  */
pub const NTLMSSP_TARGET_TYPE_DOMAIN: u32 = 0x10000;
pub const NTLMSSP_TARGET_TYPE_SERVER: u32 = 0x20000;
pub const NTLMSSP_TARGET_TYPE_SHARE: u32 = 0x40000;
pub const NTLMSSP_NEGOTIATE_EXTENDED_SEC: u32 = 0x80000; /* NB:not related to NTLMv2 pwd*/
/* #define NTLMSSP_REQUEST_INIT_RESP     0x100000 */
pub const NTLMSSP_NEGOTIATE_IDENTIFY: u32 = 0x100000;
pub const NTLMSSP_REQUEST_ACCEPT_RESP: u32 = 0x200000; /* reserved5 */
pub const NTLMSSP_REQUEST_NON_NT_KEY: u32 = 0x400000;
pub const NTLMSSP_NEGOTIATE_TARGET_INFO: u32 = 0x800000;
/* #define reserved4                 0x1000000 */
pub const NTLMSSP_NEGOTIATE_VERSION: u32 = 0x2000000; /* we only set for SMB2+ */
/* #define reserved3                 0x4000000 */
/* #define reserved2                 0x8000000 */
/* #define reserved1                0x10000000 */
pub const NTLMSSP_NEGOTIATE_128: u32 = 0x20000000;
pub const NTLMSSP_NEGOTIATE_KEY_XCH: u32 = 0x40000000;
pub const NTLMSSP_NEGOTIATE_56: u32 = 0x80000000;

/* Define AV Pair Field IDs */
#[repr(C)]
pub enum av_field_type {
    NTLMSSP_AV_EOL = 0,
    NTLMSSP_AV_NB_COMPUTER_NAME,
    NTLMSSP_AV_NB_DOMAIN_NAME,
    NTLMSSP_AV_DNS_COMPUTER_NAME,
    NTLMSSP_AV_DNS_DOMAIN_NAME,
    NTLMSSP_AV_DNS_TREE_NAME,
    NTLMSSP_AV_FLAGS,
    NTLMSSP_AV_TIMESTAMP,
    NTLMSSP_AV_RESTRICTION,
    NTLMSSP_AV_TARGET_NAME,
    NTLMSSP_AV_CHANNEL_BINDINGS,
}

/* Although typedefs are not commonly used for structure definitions */
/* in the Linux kernel, in this particular case they are useful      */
/* to more closely match the standards document for NTLMSSP from     */
/* OpenGroup and to make the code more closely match the standard in */
/* appearance */

#[repr(C, packed)]
pub struct SECURITY_BUFFER {
    pub Length: u16,
    pub MaximumLength: u16,
    pub BufferOffset: u32, /* offset to buffer */
}

#[repr(C, packed)]
pub struct NEGOTIATE_MESSAGE {
    pub Signature: [u8; 7],
    pub MessageType: u32, /* NtLmNegotiate = 1 */
    pub NegotiateFlags: u32,
    pub DomainName: SECURITY_BUFFER, /* RFC 1001 style and ASCII */
    pub WorkstationName: SECURITY_BUFFER, /* RFC 1001 and ASCII */
    /* SECURITY_BUFFER for version info not present since we
       do not set the version is present flag */
    pub DomainString: [u8; 0],
    /* followed by WorkstationString */
}

pub const NTLMSSP_REVISION_W2K3: u8 = 0x0F;

/* See MS-NLMP section 2.2.2.10 */
#[repr(C, packed)]
pub struct ntlmssp_version {
    pub ProductMajorVersion: u8,
    pub ProductMinorVersion: u8,
    pub ProductBuild: u16, /* we send the cifs.ko module version here */
    pub Reserved: [u8; 3],
    pub NTLMRevisionCurrent: u8, /* currently 0x0F */
}

/* see MS-NLMP section 2.2.1.1 */
#[repr(C, packed)]
pub struct negotiate_message {
    pub Signature: [u8; 7],
    pub MessageType: u32, /* NtLmNegotiate = 1 */
    pub NegotiateFlags: u32,
    pub DomainName: SECURITY_BUFFER, /* RFC 1001 style and ASCII */
    pub WorkstationName: SECURITY_BUFFER, /* RFC 1001 and ASCII */
    pub Version: ntlmssp_version,
    /* SECURITY_BUFFER */
    pub DomainString: [u8; 0],
    /* followed by WorkstationString */
}

#[repr(C, packed)]
pub struct CHALLENGE_MESSAGE {
    pub Signature: [u8; 7],
    pub MessageType: u32, /* NtLmChallenge = 2 */
    pub TargetName: SECURITY_BUFFER,
    pub NegotiateFlags: u32,
    pub Challenge: [u8; CIFS_CRYPTO_KEY_SIZE],
    pub Reserved: [u8; 8],
    pub TargetInfoArray: SECURITY_BUFFER,
    /* SECURITY_BUFFER for version info not present since we
       do not set the version is present flag */
}

#[repr(C, packed)]
pub struct AUTHENTICATE_MESSAGE {
    pub Signature: [u8; 7],
    pub MessageType: u32, /* NtLmsAuthenticate = 3 */
    pub LmChallengeResponse: SECURITY_BUFFER,
    pub NtChallengeResponse: SECURITY_BUFFER,
    pub DomainName: SECURITY_BUFFER,
    pub UserName: SECURITY_BUFFER,
    pub WorkstationName: SECURITY_BUFFER,
    pub SessionKey: SECURITY_BUFFER,
    pub NegotiateFlags: u32,
    pub Version: ntlmssp_version,
    /* SECURITY_BUFFER */
    pub UserString: [u8; 0],
}

/*
 * Size of the session key (crypto key encrypted with the password
 */

extern "C" {
    pub fn decode_ntlmssp_challenge(
        bcc_ptr: *mut std::os::raw::c_char,
        blob_len: std::os::raw::c_int,
        ses: *mut cifs_ses,
    ) -> std::os::raw::c_int;
    pub fn build_ntlmssp_negotiate_blob(
        pbuffer: *mut *mut u8,
        buflen: *mut u16,
        ses: *mut cifs_ses,
        server: *mut TCP_Server_Info,
        nls_cp: *const nls_table,
    ) -> std::os::raw::c_int;
    pub fn build_ntlmssp_smb3_negotiate_blob(
        pbuffer: *mut *mut u8,
        buflen: *mut u16,
        ses: *mut cifs_ses,
        server: *mut TCP_Server_Info,
        nls_cp: *const nls_table,
    ) -> std::os::raw::c_int;
    pub fn build_ntlmssp_auth_blob(
        pbuffer: *mut *mut u8,
        buflen: *mut u16,
        ses: *mut cifs_ses,
        server: *mut TCP_Server_Info,
        nls_cp: *const nls_table,
    ) -> std::os::raw::c_int;
}

extern "C" {
    pub type cifs_ses;
    pub type TCP_Server_Info;
    pub type nls_table;
}

extern "C" {
    pub const CIFS_CRYPTO_KEY_SIZE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
