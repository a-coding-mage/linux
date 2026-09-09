/* SPDX-License-Identifier: LGPL-2.1+ */
/*
 *   Copyright (c) International Business Machines  Corp., 2002,2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 */

/* C header guard: __KSMBD_NTLMSSP_H */

pub const NTLMSSP_SIGNATURE: &str = "NTLMSSP";

/* Security blob target info data */
pub const TGT_Name: &str = "KSMBD";

/* Size of the crypto key returned on the negotiate SMB in bytes */
pub const CIFS_CRYPTO_KEY_SIZE: usize = 8;
pub const CIFS_KEY_SIZE: usize = 40;

/* Size of encrypted user password in bytes */
pub const CIFS_ENCPWD_SIZE: usize = 16;
pub const CIFS_CPHTXT_SIZE: usize = 16;

/* Message Types; cpu_to_le32 values */
pub const NtLmNegotiate: u32 = 1u32.to_le();
pub const NtLmChallenge: u32 = 2u32.to_le();
pub const NtLmAuthenticate: u32 = 3u32.to_le();
pub const UnknownMessage: u32 = 8u32.to_le();

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
pub const NTLMSSP_NEGOTIATE_VERSION: u32 = 0x2000000; /* we do not set */
/* #define reserved3                 0x4000000 */
/* #define reserved2                 0x8000000 */
/* #define reserved1                0x10000000 */
pub const NTLMSSP_NEGOTIATE_128: u32 = 0x20000000;
pub const NTLMSSP_NEGOTIATE_KEY_XCH: u32 = 0x40000000;
pub const NTLMSSP_NEGOTIATE_56: u32 = 0x80000000;

/* Define AV Pair Field IDs */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

/* Although typedefs are not commonly used for structure definitions...
 * These packed structures closely match the NTLMSSP standards document. */
#[repr(C, packed)]
pub struct security_buffer {
    pub Length: u16,
    pub MaximumLength: u16,
    pub BufferOffset: u32, /* offset to buffer */
}

#[repr(C, packed)]
pub struct target_info {
    pub Type: u16,
    pub Length: u16,
    pub Content: [u8; 0],
}

#[repr(C, packed)]
pub struct negotiate_message {
    pub Signature: [u8; 8],
    pub MessageType: u32, /* NtLmNegotiate = 1 */
    pub NegotiateFlags: u32,
    pub DomainName: security_buffer, /* RFC 1001 style and ASCII */
    pub WorkstationName: security_buffer, /* RFC 1001 and ASCII */
    /* version info is not present since we do not set the version flag */
    pub DomainString: [i8; 0],
    /* followed by WorkstationString */
}

#[repr(C, packed)]
pub struct challenge_message {
    pub Signature: [u8; 8],
    pub MessageType: u32, /* NtLmChallenge = 2 */
    pub TargetName: security_buffer,
    pub NegotiateFlags: u32,
    pub Challenge: [u8; CIFS_CRYPTO_KEY_SIZE],
    pub Reserved: [u8; 8],
    pub TargetInfoArray: security_buffer,
    /* version info is not present since we do not set the version flag */
}

#[repr(C, packed)]
pub struct authenticate_message {
    pub Signature: [u8; 8],
    pub MessageType: u32, /* NtLmsAuthenticate = 3 */
    pub LmChallengeResponse: security_buffer,
    pub NtChallengeResponse: security_buffer,
    pub DomainName: security_buffer,
    pub UserName: security_buffer,
    pub WorkstationName: security_buffer,
    pub SessionKey: security_buffer,
    pub NegotiateFlags: u32,
    /* version info is not present since we do not set the version flag */
    pub UserString: [i8; 0],
}

#[repr(C, packed)]
pub struct ntlmv2_resp {
    pub ntlmv2_hash: [i8; CIFS_ENCPWD_SIZE],
    pub blob_signature: u32,
    pub reserved: u32,
    pub time: u64,
    pub client_chal: u64, /* random */
    pub reserved2: u32,
    /* array of name entries could follow ending in minimum 4 byte struct */
}

/* per smb session structure/fields */
#[repr(C)]
pub struct ntlmssp_auth {
    /* whether session key is per smb session */
    pub sesskey_per_smbsess: bool,
    /* sent by client in type 1 ntlmsssp exchange */
    pub client_flags: u32,
    /* sent by server in type 2 ntlmssp exchange */
    pub conn_flags: u32,
    /* sent to server */
    pub ciphertext: [u8; CIFS_CPHTXT_SIZE],
    /* used by ntlmssp */
    pub cryptkey: [i8; CIFS_CRYPTO_KEY_SIZE],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
