// Mechanical source-level Rust translation; endian aliases are represented by native-width integers.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* SPDX-License-Identifier: LGPL-2.1 */

pub const _COMMON_SMB2PDU_H: u64 = /*,
 * Note that, due to trying to use names similar to the protocol specifications,
 * there are many mixed case field names in the structures below.  Although
 * this does not match typical Linux kernel style, it is necessary to be
 * able to match against the protocol specification.
 *
 * SMB2 commands
 * Some commands have minimal (wct=0,bcc=0), or uninteresting, responses
 * (ie no useful data other than the SMB error code itself) and are marked such.
 * Knowing this helps avoid response buffer allocations and copy in some cases.
 */

/* List of commands in host endian */
pub const SMB2_NEGOTIATE_HE: u64 = 0x0000,
pub const SMB2_SESSION_SETUP_HE: u64 = 0x0001,
pub const SMB2_LOGOFF_HE: u64 = 0x0002 /* trivial request/resp */,
pub const SMB2_TREE_CONNECT_HE: u64 = 0x0003,
pub const SMB2_TREE_DISCONNECT_HE: u64 = 0x0004 /* trivial req/resp */,
pub const SMB2_CREATE_HE: u64 = 0x0005,
pub const SMB2_CLOSE_HE: u64 = 0x0006,
pub const SMB2_FLUSH_HE: u64 = 0x0007 /* trivial resp */,
pub const SMB2_READ_HE: u64 = 0x0008,
pub const SMB2_WRITE_HE: u64 = 0x0009,
pub const SMB2_LOCK_HE: u64 = 0x000A,
pub const SMB2_IOCTL_HE: u64 = 0x000B,
pub const SMB2_CANCEL_HE: u64 = 0x000C,
pub const SMB2_ECHO_HE: u64 = 0x000D,
pub const SMB2_QUERY_DIRECTORY_HE: u64 = 0x000E,
pub const SMB2_CHANGE_NOTIFY_HE: u64 = 0x000F,
pub const SMB2_QUERY_INFO_HE: u64 = 0x0010,
pub const SMB2_SET_INFO_HE: u64 = 0x0011,
pub const SMB2_OPLOCK_BREAK_HE: u64 = 0x0012,
pub const SMB2_SERVER_TO_CLIENT_NOTIFICATION: u64 = 0x0013,

/* The same list in little endian */
pub const SMB2_NEGOTIATE: u64 = (SMB2_NEGOTIATE_HE),
pub const SMB2_SESSION_SETUP: u64 = (SMB2_SESSION_SETUP_HE),
pub const SMB2_LOGOFF: u64 = (SMB2_LOGOFF_HE),
pub const SMB2_TREE_CONNECT: u64 = (SMB2_TREE_CONNECT_HE),
pub const SMB2_TREE_DISCONNECT: u64 = (SMB2_TREE_DISCONNECT_HE),
pub const SMB2_CREATE: u64 = (SMB2_CREATE_HE),
pub const SMB2_CLOSE: u64 = (SMB2_CLOSE_HE),
pub const SMB2_FLUSH: u64 = (SMB2_FLUSH_HE),
pub const SMB2_READ: u64 = (SMB2_READ_HE),
pub const SMB2_WRITE: u64 = (SMB2_WRITE_HE),
pub const SMB2_LOCK: u64 = (SMB2_LOCK_HE),
pub const SMB2_IOCTL: u64 = (SMB2_IOCTL_HE),
pub const SMB2_CANCEL: u64 = (SMB2_CANCEL_HE),
pub const SMB2_ECHO: u64 = (SMB2_ECHO_HE),
pub const SMB2_QUERY_DIRECTORY: u64 = (SMB2_QUERY_DIRECTORY_HE),
pub const SMB2_CHANGE_NOTIFY: u64 = (SMB2_CHANGE_NOTIFY_HE),
pub const SMB2_QUERY_INFO: u64 = (SMB2_QUERY_INFO_HE),
pub const SMB2_SET_INFO: u64 = (SMB2_SET_INFO_HE),
pub const SMB2_OPLOCK_BREAK: u64 = (SMB2_OPLOCK_BREAK_HE),

pub const SMB2_INTERNAL_CMD: u64 = (0xFFFF),

pub const NUMBER_OF_SMB2_COMMANDS: u64 = 0x0013,

/*
 * Size of the session key (crypto key encrypted with the password
 */
pub const SMB2_NTLMV2_SESSKEY_SIZE: u64 = 16,
pub const SMB2_SIGNATURE_SIZE: u64 = 16,
pub const SMB2_HMACSHA256_SIZE: u64 = 32,
pub const SMB2_CMACAES_SIZE: u64 = 16,
pub const SMB3_GCM128_CRYPTKEY_SIZE: u64 = 16,
pub const SMB3_GCM256_CRYPTKEY_SIZE: u64 = 32,

/*
 * Size of the smb3 encryption/decryption keys
 * This size is big enough to store any cipher key types.
 */
pub const SMB3_ENC_DEC_KEY_SIZE: u64 = 32,

/*
 * Size of the smb3 signing key
 */
pub const SMB3_SIGN_KEY_SIZE: u64 = 16,

pub const CIFS_CLIENT_CHALLENGE_SIZE: u64 = 8,

/* Maximum buffer size value we can send with 1 credit */
pub const SMB2_MAX_BUFFER_SIZE: u64 = 65536,

/*
 * The default wsize is 1M for SMB2 (and for some CIFS cases).
 * find_get_pages seems to return a maximum of 256
 * pages in a single call. With PAGE_SIZE == 4k, this means we can
 * fill a single wsize request with a single call.
 */
pub const SMB3_DEFAULT_IOSIZE: u64 = (4 * 1024 * 1024),

/* According to MS-SMB2 specification The minimum recommended value is 65536.*/
pub const CIFS_MIN_DEFAULT_IOSIZE: u64 = (65536),

/*
 * SMB2 Header Definition
 *
 * "MBZ" :  Must be Zero
 * "BB"  :  BugBug, Something to check/review/analyze later
 * "PDU" :  "Protocol Data Unit" (ie a network "frame")
 *
 */

pub const __SMB2_HEADER_STRUCTURE_SIZE: u64 = 64,
pub const SMB2_HEADER_STRUCTURE_SIZE: u64 = ,
	(__SMB2_HEADER_STRUCTURE_SIZE)

pub const SMB2_PROTO_NUMBER: u64 = (0x424d53fe),
pub const SMB2_TRANSFORM_PROTO_NUM: u64 = (0x424d53fd),
pub const SMB2_COMPRESSION_TRANSFORM_ID: u64 = (0x424d53fc),

/*
 *	SMB2 flag definitions
 */
pub const SMB2_FLAGS_SERVER_TO_REDIR: u64 = (0x00000001),
pub const SMB2_FLAGS_ASYNC_COMMAND: u64 = (0x00000002),
pub const SMB2_FLAGS_RELATED_OPERATIONS: u64 = (0x00000004),
pub const SMB2_FLAGS_SIGNED: u64 = (0x00000008),
pub const SMB2_FLAGS_PRIORITY_MASK: u64 = (0x00000070) /* SMB3.1.1 */,
pub const SMB2_FLAGS_DFS_OPERATIONS: u64 = (0x10000000),
pub const SMB2_FLAGS_REPLAY_OPERATION: u64 = (0x20000000) /* SMB3 & up */,

/*
 *	Definitions for SMB2 Protocol Data Units (network frames)
 *
 *  See MS-SMB2.PDF specification for protocol details.
 *  The Naming convention is the lower case version of the SMB2
 *  command code name for the struct. Note that structures must be packed.
 *
 */

/* See MS-SMB2 section 2.2.1 */

pub #[repr(C, packed)] pub struct smb2_hdr {
	pub ProtocolId: u32,	/* 0xFE 'S' 'M' 'B' */
	pub StructureSize: u16,	/* 64 */
	pub CreditCharge: u16,	/* MBZ */
	pub Status: u32,		/* Error from server */
	pub Command: u16,
	pub CreditRequest: u16,	/* CreditResponse */
	pub Flags: u32,
	pub NextCommand: u32,
	pub MessageId: u64,
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		#[repr(C)] pub #[repr(C, packed)] pub struct AnonymousStruct {
			pub ProcessId: u32,
			pub TreeId: u32
}  SyncId,
		pub AsyncId: u64
}  Id,
	pub SessionId: u64,
	u8   Signature[16]
} ,

pub #[repr(C, packed)] pub struct smb3_hdr_req {
	pub ProtocolId: u32,	/* 0xFE 'S' 'M' 'B' */
	pub StructureSize: u16,	/* 64 */
	pub CreditCharge: u16,	/* MBZ */
	pub ChannelSequence: u16, /* See MS-SMB2 3.2.4.1 and 3.2.7.1 */
	pub Reserved: u16,
	pub Command: u16,
	pub CreditRequest: u16,	/* CreditResponse */
	pub Flags: u32,
	pub NextCommand: u32,
	pub MessageId: u64,
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		#[repr(C)] pub #[repr(C, packed)] pub struct AnonymousStruct {
			pub ProcessId: u32,
			pub TreeId: u32
}  SyncId,
		pub AsyncId: u64
}  Id,
	pub SessionId: u64,
	u8   Signature[16]
} ,

pub #[repr(C, packed)] pub struct smb2_pdu {
	pub hdr: struct smb2_hdr,
	pub StructureSize2: u16, /* size of wct area (varies, request specific) */
} ,

pub const SMB2_ERROR_STRUCTURE_SIZE2: u64 = 9,
pub const SMB2_ERROR_STRUCTURE_SIZE2_LE: u64 = (SMB2_ERROR_STRUCTURE_SIZE2),

pub #[repr(C, packed)] pub struct smb2_err_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,
	pub ErrorContextCount: u8,
	pub Reserved: u8,
	pub ByteCount: u32,  /* even if zero, at least one byte follows */
	u8   ErrorData: [u8, 0],  /* variable length */
} ,

pub const SMB3_AES_CCM_NONCE: u64 = 11,
pub const SMB3_AES_GCM_NONCE: u64 = 12,

/* Transform flags (for 3.0 dialect this flag indicates CCM */
pub const TRANSFORM_FLAG_ENCRYPTED: u64 = 0x0001,

pub #[repr(C, packed)] pub struct smb2_transform_hdr {
	pub ProtocolId: u32,	/* 0xFD 'S' 'M' 'B' */
	u8   Signature[16],
	u8   Nonce[16],
	pub OriginalMessageSize: u32,
	pub Reserved1: u16,
	pub Flags: u16, /* EncryptionAlgorithm for 3.0, enc enabled for 3.1.1 */
	pub SessionId: u64
} ,

/*
 * These are simplified versions from the spec, as we don't need a fully fledged
 * form of both unchained and chained structs.
 *
 * For chained payloads, only the first 8 bytes belong to the transform header.
 * CompressionAlgorithm, Flags and Offset below overlay the first chained
 * payload header, where Offset represents Length.
 *
 * See MS-SMB2 2.2.42 for more details.
 */
pub const SMB2_COMPRESSION_FLAG_NONE: u64 = 0x0000,
pub const SMB2_COMPRESSION_FLAG_CHAINED: u64 = 0x0001,

pub #[repr(C, packed)] pub struct smb2_compression_hdr {
	pub ProtocolId: u32, /* 0xFC 'S' 'M' 'B' */
	pub OriginalCompressedSegmentSize: u32,
	pub CompressionAlgorithm: u16,
	pub Flags: u16,
	pub Offset: u32, /* this is the size of the uncompressed SMB2 header below */
	/* uncompressed SMB2 header (READ or WRITE) goes here */
	/* compressed data goes here */
} ,

/*
 * ... OTOH, set compression payload header to always have OriginalPayloadSize
 * as it's easier to pass the struct size minus sizeof(OriginalPayloadSize)
 * than to juggle around the header/data memory.
 */

pub #[repr(C, packed)] pub struct smb2_compression_payload_hdr {
	pub CompressionAlgorithm: u16,
	pub Flags: u16,
	pub Length: u32, /* length of compressed playload including field below if present */
	pub OriginalPayloadSize: u32, /* accounted when LZNT1, LZ77, LZ77+Huffman */
} ,

pub #[repr(C, packed)] pub struct smb2_compression_pattern_v1 {
	pub Pattern: u8,
	pub Reserved1: u8,
	pub Reserved2: u16,
	pub Repetitions: u32
} ,

/* See MS-SMB2 section 2.2.9.2 */
/* Context Types */
pub const SMB2_RESERVED_TREE_CONNECT_CONTEXT_ID: u64 = 0x0000,
pub const SMB2_REMOTED_IDENTITY_TREE_CONNECT_CONTEXT_ID: u64 = (0x0001),

pub #[repr(C, packed)] pub struct tree_connect_contexts {
	pub ContextType: u16,
	pub DataLength: u16,
	pub Reserved: u32,
	u8   Data: [u8, 0]
} ,

/* Remoted identity tree connect context structures - see MS-SMB2 2.2.9.2.1 */

pub #[repr(C, packed)] pub struct smb3_blob_data {
	pub BlobSize: u16,
	u8   BlobData: [u8, 0]
} ,

/* Valid values for Attr */
pub const SE_GROUP_MANDATORY: u64 = 0x00000001,
pub const SE_GROUP_ENABLED_BY_DEFAULT: u64 = 0x00000002,
pub const SE_GROUP_ENABLED: u64 = 0x00000004,
pub const SE_GROUP_OWNER: u64 = 0x00000008,
pub const SE_GROUP_USE_FOR_DENY_ONLY: u64 = 0x00000010,
pub const SE_GROUP_INTEGRITY: u64 = 0x00000020,
pub const SE_GROUP_INTEGRITY_ENABLED: u64 = 0x00000040,
pub const SE_GROUP_RESOURCE: u64 = 0x20000000,
pub const SE_GROUP_LOGON_ID: u64 = 0xC0000000,

pub #[repr(C, packed)] pub struct sid_array_data {
	pub SidAttrCount: u16,
	/* SidAttrList - array of sid_attr_data structs */
} ,

/* struct sid_attr_data is SidData array in BlobData format then le32 Attr */

pub #[repr(C, packed)] pub struct sid_attr_data {
	pub BlobSize: u16,
	u8 BlobData: [u8, 0],
	/* u32 Attr */
} ,

/*
 * struct privilege_data is the same as BLOB_DATA - see MS-SMB2 2.2.9.2.1.5
 * but with size of LUID_ATTR_DATA struct and BlobData set to LUID_ATTR DATA
 */

pub #[repr(C, packed)] pub struct privilege_array_data {
	pub PrivilegeCount: u16,
	/* array of privilege_data structs */
} ,

pub #[repr(C, packed)] pub struct remoted_identity_tcon_context {
	pub TicketType: u16, /* must be 0x0001 */
	pub TicketSize: u16, /* total size of this struct */
	pub User: u16, /* offset to SID_ATTR_DATA struct with user info */
	pub UserName: u16, /* offset to null terminated Unicode username string */
	pub Domain: u16, /* offset to null terminated Unicode domain name */
	pub Groups: u16, /* offset to SID_ARRAY_DATA struct with group info */
	pub RestrictedGroups: u16, /* similar to above */
	pub Privileges: u16, /* offset to PRIVILEGE_ARRAY_DATA struct */
	pub PrimaryGroup: u16, /* offset to SID_ARRAY_DATA struct */
	pub Owner: u16, /* offset to BLOB_DATA struct */
	pub DefaultDacl: u16, /* offset to BLOB_DATA struct */
	pub DeviceGroups: u16, /* offset to SID_ARRAY_DATA struct */
	pub UserClaims: u16, /* offset to BLOB_DATA struct */
	pub DeviceClaims: u16, /* offset to BLOB_DATA struct */
	u8   TicketInfo: [u8, 0], /* variable length buf - remoted identity data */
} ,

pub #[repr(C, packed)] pub struct smb2_tree_connect_req_extension {
	pub TreeConnectContextOffset: u32,
	pub TreeConnectContextCount: u16,
	u8  Reserved[10],
	u8  PathName: [u8, 0], /* variable sized array */
	/* followed by array of TreeConnectContexts */
} ,

/* Flags/Reserved for SMB3.1.1 */
pub const SMB2_TREE_CONNECT_FLAG_CLUSTER_RECONNECT: u64 = (0x0001),
pub const SMB2_TREE_CONNECT_FLAG_REDIRECT_TO_OWNER: u64 = (0x0002),
pub const SMB2_TREE_CONNECT_FLAG_EXTENSION_PRESENT: u64 = (0x0004),

pub #[repr(C, packed)] pub struct smb2_tree_connect_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 9 */
	pub Flags: u16,		/* Flags in SMB3.1.1 */
	pub PathOffset: u16,
	pub PathLength: u16,
	u8   Buffer: [u8, 0],	/* variable length */
} ,

/* Possible ShareType values */
pub const SMB2_SHARE_TYPE_DISK: u64 = 0x01,
pub const SMB2_SHARE_TYPE_PIPE: u64 = 0x02,
pub const SMB2_SHARE_TYPE_PRINT: u64 = 0x03,

/*
 * Possible ShareFlags - exactly one and only one of the first 4 caching flags
 * must be set (any of the remaining, SHI1005, flags may be set individually
 * or in combination.
 */
pub const SMB2_SHAREFLAG_MANUAL_CACHING: u64 = 0x00000000,
pub const SMB2_SHAREFLAG_AUTO_CACHING: u64 = 0x00000010,
pub const SMB2_SHAREFLAG_VDO_CACHING: u64 = 0x00000020,
pub const SMB2_SHAREFLAG_NO_CACHING: u64 = 0x00000030,
pub const SHI1005_FLAGS_DFS: u64 = 0x00000001,
pub const SHI1005_FLAGS_DFS_ROOT: u64 = 0x00000002,
pub const SMB2_SHAREFLAG_RESTRICT_EXCLUSIVE_OPENS: u64 = 0x00000100,
pub const SMB2_SHAREFLAG_FORCE_SHARED_DELETE: u64 = 0x00000200,
pub const SMB2_SHAREFLAG_ALLOW_NAMESPACE_CACHING: u64 = 0x00000400,
pub const SMB2_SHAREFLAG_ACCESS_BASED_DIRECTORY_ENUM: u64 = 0x00000800,
pub const SMB2_SHAREFLAG_FORCE_LEVELII_OPLOCK: u64 = 0x00001000,
pub const SMB2_SHAREFLAG_ENABLE_HASH_V1: u64 = 0x00002000,
pub const SMB2_SHAREFLAG_ENABLE_HASH_V2: u64 = 0x00004000,
pub const SMB2_SHAREFLAG_ENCRYPT_DATA: u64 = 0x00008000,
pub const SHI1005_FLAGS_ENCRYPT_DATA: u64 = SMB2_SHAREFLAG_ENCRYPT_DATA,
pub const SMB2_SHAREFLAG_IDENTITY_REMOTING: u64 = 0x00040000 /* 3.1.1 */,
pub const SMB2_SHAREFLAG_COMPRESS_DATA: u64 = 0x00100000 /* 3.1.1 */,
pub const SMB2_SHAREFLAG_ISOLATED_TRANSPORT: u64 = 0x00200000,
pub const SHI1005_FLAGS_ALL: u64 = 0x0034FF33,

/* Possible share capabilities */
pub const SMB2_SHARE_CAP_DFS: u64 = (0x00000008) /* all dialects */,
pub const SMB2_SHARE_CAP_CONTINUOUS_AVAILABILITY: u64 = (0x00000010) /* 3.0 */,
pub const SMB2_SHARE_CAP_SCALEOUT: u64 = (0x00000020) /* 3.0 */,
pub const SMB2_SHARE_CAP_CLUSTER: u64 = (0x00000040) /* 3.0 */,
pub const SMB2_SHARE_CAP_ASYMMETRIC: u64 = (0x00000080) /* 3.02 */,
pub const SMB2_SHARE_CAP_REDIRECT_TO_OWNER: u64 = (0x00000100) /* 3.1.1 */,

pub #[repr(C, packed)] pub struct smb2_tree_connect_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 16 */
	pub ShareType: u8,	/* see below */
	pub Reserved: u8,
	pub ShareFlags: u32,	/* see below */
	pub Capabilities: u32,	/* see below */
	pub MaximalAccess: u32
} ,

pub #[repr(C, packed)] pub struct smb2_tree_disconnect_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,

pub #[repr(C, packed)] pub struct smb2_tree_disconnect_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,


/*
 * SMB2_NEGOTIATE_PROTOCOL  See MS-SMB2 section 2.2.3
 */
/* SecurityMode flags */
pub const SMB2_NEGOTIATE_SIGNING_ENABLED: u64 = 0x0001,
pub const SMB2_NEGOTIATE_SIGNING_ENABLED_LE: u64 = (0x0001),
pub const SMB2_NEGOTIATE_SIGNING_REQUIRED: u64 = 0x0002,
pub const SMB2_NEGOTIATE_SIGNING_REQUIRED_LE: u64 = (0x0002),
pub const SMB2_SEC_MODE_FLAGS_ALL: u64 = 0x0003,

/* Capabilities flags */
pub const SMB2_GLOBAL_CAP_DFS: u64 = 0x00000001,
pub const SMB2_GLOBAL_CAP_LEASING: u64 = 0x00000002 /* Resp only New to SMB2.1 */,
pub const SMB2_GLOBAL_CAP_LARGE_MTU: u64 = 0x00000004 /* Resp only New to SMB2.1 */,
pub const SMB2_GLOBAL_CAP_MULTI_CHANNEL: u64 = 0x00000008 /* New to SMB3 */,
pub const SMB2_GLOBAL_CAP_PERSISTENT_HANDLES: u64 = 0x00000010 /* New to SMB3 */,
pub const SMB2_GLOBAL_CAP_DIRECTORY_LEASING: u64 = 0x00000020 /* New to SMB3 */,
pub const SMB2_GLOBAL_CAP_ENCRYPTION: u64 = 0x00000040 /* New to SMB3 */,
pub const SMB2_GLOBAL_CAP_NOTIFICATIONS: u64 = 0x00000080 /* New to SMB3.1.1 */,
/* Internal types */
pub const SMB2_NT_FIND: u64 = 0x00100000,
pub const SMB2_LARGE_FILES: u64 = 0x00200000,

pub const SMB2_CLIENT_GUID_SIZE: u64 = 16,
pub const SMB2_CREATE_GUID_SIZE: u64 = 16,

/* Dialects */
pub const SMB10_PROT_ID: u64 = 0x0000 /* local only, not sent on wire w/CIFS negprot */,
pub const SMB20_PROT_ID: u64 = 0x0202,
pub const SMB21_PROT_ID: u64 = 0x0210,
pub const SMB2X_PROT_ID: u64 = 0x02FF,
pub const SMB30_PROT_ID: u64 = 0x0300,
pub const SMB302_PROT_ID: u64 = 0x0302,
pub const SMB311_PROT_ID: u64 = 0x0311,
pub const BAD_PROT_ID: u64 = 0xFFFF,

pub const SMB311_SALT_SIZE: u64 = 32,
/* Hash Algorithm Types */
pub const SMB2_PREAUTH_INTEGRITY_SHA512: u64 = (0x0001),
pub const SMB2_PREAUTH_HASH_SIZE: u64 = 64,

/* Negotiate Contexts - ContextTypes. See MS-SMB2 section 2.2.3.1 for details */
pub const SMB2_PREAUTH_INTEGRITY_CAPABILITIES: u64 = (1),
pub const SMB2_ENCRYPTION_CAPABILITIES: u64 = (2),
pub const SMB2_COMPRESSION_CAPABILITIES: u64 = (3),
pub const SMB2_NETNAME_NEGOTIATE_CONTEXT_ID: u64 = (5),
pub const SMB2_TRANSPORT_CAPABILITIES: u64 = (6),
pub const SMB2_RDMA_TRANSFORM_CAPABILITIES: u64 = (7),
pub const SMB2_SIGNING_CAPABILITIES: u64 = (8),
pub const SMB2_POSIX_EXTENSIONS_AVAILABLE: u64 = (0x100),

pub #[repr(C, packed)] pub struct smb2_neg_context {
	pub ContextType: u16,
	pub DataLength: u16,
	pub Reserved: u32,
	/* Followed by array of data. NOTE: some servers require padding to 8 byte boundary */
} ,

/*
 * SaltLength that the server send can be zero, so the only three required
 * fields (all u16) end up six bytes total, so the minimum context data len
 * in the response is six bytes which accounts for
 *
 *      HashAlgorithmCount, SaltLength, and 1 HashAlgorithm.
 */
pub const MIN_PREAUTH_CTXT_DATA_LEN: u64 = 6,

pub #[repr(C, packed)] pub struct smb2_preauth_neg_context {
	pub ContextType: u16, /* 1 */
	pub DataLength: u16,
	pub Reserved: u32,
	pub HashAlgorithmCount: u16, /* 1 */
	pub SaltLength: u16,
	pub HashAlgorithms: u16, /* HashAlgorithms[0] since only one defined */
	u8	Salt[SMB311_SALT_SIZE]
} ,

/* Encryption Algorithms Ciphers */
pub const SMB2_ENCRYPTION_AES128_CCM: u64 = (0x0001),
pub const SMB2_ENCRYPTION_AES128_GCM: u64 = (0x0002),
pub const SMB2_ENCRYPTION_AES256_CCM: u64 = (0x0003),
pub const SMB2_ENCRYPTION_AES256_GCM: u64 = (0x0004),

/* Min encrypt context data is one cipher so 2 bytes + 2 byte count field */
pub const MIN_ENCRYPT_CTXT_DATA_LEN: u64 = 4,

pub #[repr(C, packed)] pub struct smb2_encryption_neg_context {
	pub ContextType: u16, /* 2 */
	pub DataLength: u16,
	pub Reserved: u32,
	/* CipherCount usually 2, but can be 3 when AES256-GCM enabled */
	pub CipherCount: u16, /* AES128-GCM and AES128-CCM by default */
	Ciphers: [u16, 0]
} ,

/* See MS-SMB2 2.2.3.1.3 */
pub const SMB3_COMPRESS_NONE: u64 = (0x0000),
pub const SMB3_COMPRESS_LZNT1: u64 = (0x0001),
pub const SMB3_COMPRESS_LZ77: u64 = (0x0002),
pub const SMB3_COMPRESS_LZ77_HUFF: u64 = (0x0003),
/* Pattern scanning algorithm See MS-SMB2 3.1.4.4.1 */
pub const SMB3_COMPRESS_PATTERN: u64 = (0x0004) /* Pattern_V1 */,
pub const SMB3_COMPRESS_LZ4: u64 = (0x0005),
/* Account for NONE for easier array indexing */
pub const SMB3_COMPRESS_MAX_ALGS: u64 = 6,

/* Compression Flags */
pub const SMB2_COMPRESSION_CAPABILITIES_FLAG_NONE: u64 = (0x00000000),
pub const SMB2_COMPRESSION_CAPABILITIES_FLAG_CHAINED: u64 = (0x00000001),

pub #[repr(C, packed)] pub struct smb2_compression_capabilities_context {
	pub ContextType: u16, /* 3 */
	pub DataLength: u16,
	pub Reserved: u32,
	pub CompressionAlgorithmCount: u16,
	pub Padding: u16,
	pub Flags: u32,
	u16	CompressionAlgorithms[4]
} ,

/*
 * For smb2_netname_negotiate_context_id See MS-SMB2 2.2.3.1.4.
 * Its struct simply contains NetName, an array of Unicode characters
 */

pub #[repr(C, packed)] pub struct smb2_netname_neg_context {
	pub ContextType: u16, /* 5 */
	pub DataLength: u16,
	pub Reserved: u32,
	NetName: [u16, 0], /* hostname of target converted to UCS-2 */
} ,

/*
 * For smb2_transport_capabilities context see MS-SMB2 2.2.3.1.5
 * and 2.2.4.1.5
 */

/* Flags */
pub const SMB2_ACCEPT_TRANSPORT_LEVEL_SECURITY: u64 = 0x00000001,

pub #[repr(C, packed)] pub struct smb2_transport_capabilities_context {
	pub ContextType: u16, /* 6 */
	pub DataLength: u16,
	pub Reserved: u32,
	pub Flags: u32,
	pub Pad: u32
} ,

/*
 * For rdma transform capabilities context see MS-SMB2 2.2.3.1.6
 * and 2.2.4.1.6
 */

/* RDMA Transform IDs */
pub const SMB2_RDMA_TRANSFORM_NONE: u64 = 0x0000,
pub const SMB2_RDMA_TRANSFORM_ENCRYPTION: u64 = 0x0001,
pub const SMB2_RDMA_TRANSFORM_SIGNING: u64 = 0x0002,

pub #[repr(C, packed)] pub struct smb2_rdma_transform_capabilities_context {
	pub ContextType: u16, /* 7 */
	pub DataLength: u16,
	pub Reserved: u32,
	pub TransformCount: u16,
	pub Reserved1: u16,
	pub Reserved2: u32,
	RDMATransformIds: [u16, 0]
} ,

/*
 * For signing capabilities context see MS-SMB2 2.2.3.1.7
 * and 2.2.4.1.7
 */

/* Signing algorithms */
pub const SIGNING_ALG_HMAC_SHA256: u64 = 0,
pub const SIGNING_ALG_HMAC_SHA256_LE: u64 = (0),
pub const SIGNING_ALG_AES_CMAC: u64 = 1,
pub const SIGNING_ALG_AES_CMAC_LE: u64 = (1),
pub const SIGNING_ALG_AES_GMAC: u64 = 2,
pub const SIGNING_ALG_AES_GMAC_LE: u64 = (2),

pub #[repr(C, packed)] pub struct smb2_signing_capabilities {
	pub ContextType: u16, /* 8 */
	pub DataLength: u16,
	pub Reserved: u32,
	pub SigningAlgorithmCount: u16,
	SigningAlgorithms: [u16, 0],
	/*  Followed by padding to 8 byte boundary (required by some servers) */
} ,

pub const POSIX_CTXT_DATA_LEN: u64 = 16,

pub #[repr(C, packed)] pub struct smb2_posix_neg_context {
	pub ContextType: u16, /* 0x100 */
	pub DataLength: u16,
	pub Reserved: u32,
	u8	Name[16], /* POSIX ctxt GUID 93AD25509CB411E7B42383DE968BCD7C */
} ,

pub #[repr(C, packed)] pub struct smb2_negotiate_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 36 */
	pub DialectCount: u16,
	pub SecurityMode: u16,
	pub Reserved: u16,	/* MBZ */
	pub Capabilities: u32,
	u8   ClientGUID[SMB2_CLIENT_GUID_SIZE],
	/* In SMB3.02 and earlier next three were MBZ le64 ClientStartTime */
	pub NegotiateContextOffset: u32, /* SMB3.1.1 only. MBZ earlier */
	pub NegotiateContextCount: u16,  /* SMB3.1.1 only. MBZ earlier */
	pub Reserved2: u16,
	u16 Dialects: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_negotiate_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 65 */
	pub SecurityMode: u16,
	pub DialectRevision: u16,
	pub NegotiateContextCount: u16,	/* Prior to SMB3.1.1 was Reserved & MBZ */
	u8   ServerGUID[16],
	pub Capabilities: u32,
	pub MaxTransactSize: u32,
	pub MaxReadSize: u32,
	pub MaxWriteSize: u32,
	pub SystemTime: u64,	/* MBZ */
	pub ServerStartTime: u64,
	pub SecurityBufferOffset: u16,
	pub SecurityBufferLength: u16,
	pub NegotiateContextOffset: u32,	/* Pre:SMB3.1.1 was reserved/ignored */
	u8   Buffer: [u8, 0],	/* variable length GSS security buffer */
} ,


/*
 * SMB2_SESSION_SETUP  See MS-SMB2 section 2.2.5
 */
/* Flags */
pub const SMB2_SESSION_REQ_FLAG_BINDING: u64 = 0x01,
pub const SMB2_SESSION_REQ_FLAG_ENCRYPT_DATA: u64 = 0x04,

pub #[repr(C, packed)] pub struct smb2_sess_setup_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 25 */
	pub Flags: u8,
	pub SecurityMode: u8,
	pub Capabilities: u32,
	pub Channel: u32,
	pub SecurityBufferOffset: u16,
	pub SecurityBufferLength: u16,
	pub PreviousSessionId: u64,
	u8   Buffer: [u8, 0],	/* variable length GSS security buffer */
} ,

/* Currently defined SessionFlags */
pub const SMB2_SESSION_FLAG_IS_GUEST: u64 = 0x0001,
pub const SMB2_SESSION_FLAG_IS_GUEST_LE: u64 = (0x0001),
pub const SMB2_SESSION_FLAG_IS_NULL: u64 = 0x0002,
pub const SMB2_SESSION_FLAG_IS_NULL_LE: u64 = (0x0002),
pub const SMB2_SESSION_FLAG_ENCRYPT_DATA: u64 = 0x0004,
pub const SMB2_SESSION_FLAG_ENCRYPT_DATA_LE: u64 = (0x0004),

pub #[repr(C, packed)] pub struct smb2_sess_setup_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 9 */
	pub SessionFlags: u16,
	pub SecurityBufferOffset: u16,
	pub SecurityBufferLength: u16,
	u8   Buffer: [u8, 0],	/* variable length GSS security buffer */
} ,


/*
 * SMB2_LOGOFF  See MS-SMB2 section 2.2.7
 */

pub #[repr(C, packed)] pub struct smb2_logoff_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,

pub #[repr(C, packed)] pub struct smb2_logoff_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,


/*
 * SMB2_CLOSE  See MS-SMB2 section 2.2.15
 */
/* Currently defined values for close flags */
pub const SMB2_CLOSE_FLAG_POSTQUERY_ATTRIB: u64 = (0x0001),

pub #[repr(C, packed)] pub struct smb2_close_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 24 */
	pub Flags: u16,
	pub Reserved: u32,
	pub PersistentFileId: u64, /* opaque endianness */
	pub VolatileFileId: u64, /* opaque endianness */
} ,

/*
 * Maximum size of a SMB2_CLOSE response is 64 (smb2 header) + 60 (data)
 */
pub const MAX_SMB2_CLOSE_RESPONSE_SIZE: u64 = 124,

pub #[repr(C, packed)] pub struct smb2_close_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* 60 */
	pub Flags: u16,
	pub Reserved: u32,
	struct_group_attr(network_open_info, ,
		pub CreationTime: u64,
		pub LastAccessTime: u64,
		pub LastWriteTime: u64,
		pub ChangeTime: u64,
		/* Beginning of FILE_STANDARD_INFO equivalent */
		pub AllocationSize: u64,
		pub EndOfFile: u64,
		pub Attributes: u32,
	)
} ,


/*
 * SMB2_READ  See MS-SMB2 section 2.2.19
 */
/* For read request Flags field below, following flag is defined for SMB3.02 */
pub const SMB2_READFLAG_READ_UNBUFFERED: u64 = 0x01,
pub const SMB2_READFLAG_REQUEST_COMPRESSED: u64 = 0x02 /* See MS-SMB2 2.2.19 */,

/* Channel field for read and write: exactly one of following flags can be set*/
pub const SMB2_CHANNEL_NONE: u64 = (0x00000000),
pub const SMB2_CHANNEL_RDMA_V1: u64 = (0x00000001),
pub const SMB2_CHANNEL_RDMA_V1_INVALIDATE: u64 = (0x00000002),
pub const SMB2_CHANNEL_RDMA_TRANSFORM: u64 = (0x00000003),

/* See MS-SMB2 2.2.43. */

pub #[repr(C, packed)] pub struct smb2_rdma_transform {
	pub RdmaDescriptorOffset: u16,
	pub RdmaDescriptorLength: u16,
	pub Channel: u32,
	pub TransformCount: u16,
	pub Reserved1: u16,
	pub Reserved2: u32
} ,

pub const SMB2_RDMA_TRANSFORM_TYPE_ENCRYPTION: u64 = 0x0001,
pub const SMB2_RDMA_TRANSFORM_TYPE_SIGNING: u64 = 0x0002,

pub #[repr(C, packed)] pub struct smb2_rdma_crypto_transform {
	pub TransformType: u16,
	pub SignatureLength: u16,
	pub NonceLength: u16,
	pub Reserved: u16,
	u8 Signature: [u8, 0],
	/* Followed by Nonce[] and optional alignment padding. */
} ,

/* SMB2 read request without RFC1001 length at the beginning */

pub #[repr(C, packed)] pub struct smb2_read_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 49 */
	pub Padding: u8, /* offset from start of SMB2 header to place read */
	pub Flags: u8, /* MBZ unless SMB3.02 or later */
	pub Length: u32,
	pub Offset: u64,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	pub MinimumCount: u32,
	pub Channel: u32, /* MBZ except for SMB3 or later */
	pub RemainingBytes: u32,
	pub ReadChannelInfoOffset: u16,
	pub ReadChannelInfoLength: u16,
	u8   Buffer: [u8, 0]
} ,

/* Read flags */
pub const SMB2_READFLAG_RESPONSE_NONE: u64 = (0x00000000),
pub const SMB2_READFLAG_RESPONSE_RDMA_TRANSFORM: u64 = (0x00000001),

pub #[repr(C, packed)] pub struct smb2_read_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 17 */
	pub DataOffset: u8,
	pub Reserved: u8,
	pub DataLength: u32,
	pub DataRemaining: u32,
	pub Flags: u32,
	u8   Buffer: [u8, 0]
} ,


/*
 * SMB2_WRITE  See MS-SMB2 section 2.2.21
 */
/* For write request Flags field below the following flags are defined: */
pub const SMB2_WRITEFLAG_WRITE_THROUGH: u64 = 0x00000001	/* SMB2.1 or later */,
pub const SMB2_WRITEFLAG_WRITE_UNBUFFERED: u64 = 0x00000002	/* SMB3.02 or later */,

pub #[repr(C, packed)] pub struct smb2_write_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 49 */
	pub DataOffset: u16, /* offset from start of SMB2 header to write data */
	pub Length: u32,
	pub Offset: u64,
	pub PersistentFileId: u64, /* opaque endianness */
	pub VolatileFileId: u64, /* opaque endianness */
	pub Channel: u32, /* MBZ unless SMB3.02 or later */
	pub RemainingBytes: u32,
	pub WriteChannelInfoOffset: u16,
	pub WriteChannelInfoLength: u16,
	pub Flags: u32,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_write_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 17 */
	pub DataOffset: u8,
	pub Reserved: u8,
	pub DataLength: u32,
	pub DataRemaining: u32,
	pub Reserved2: u32,
	u8   Buffer: [u8, 0]
} ,


/*
 * SMB2_FLUSH  See MS-SMB2 section 2.2.17
 */

pub #[repr(C, packed)] pub struct smb2_flush_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 24 */
	pub Reserved1: u16,
	pub Reserved2: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64
} ,

pub #[repr(C, packed)] pub struct smb2_flush_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,
	pub Reserved: u16
} ,

pub const SMB2_LOCKFLAG_SHARED: u64 = 0x0001,
pub const SMB2_LOCKFLAG_EXCLUSIVE: u64 = 0x0002,
pub const SMB2_LOCKFLAG_UNLOCK: u64 = 0x0004,
pub const SMB2_LOCKFLAG_FAIL_IMMEDIATELY: u64 = 0x0010,
pub const SMB2_LOCKFLAG_MASK: u64 = 0x0007,

pub #[repr(C, packed)] pub struct smb2_lock_element {
	pub Offset: u64,
	pub Length: u64,
	pub Flags: u32,
	pub Reserved: u32
} ,

pub #[repr(C, packed)] pub struct smb2_lock_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 48 */
	pub LockCount: u16,
	/*
	 * The least significant four bits are the lock sequence number. The
	 * other 28 bits are the index (0 to 64). See MS-SMB2 2.2.26.
	 */
	pub LockSequenceNumber: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	/* Followed by at least one */
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		pub lock: struct smb2_lock_element,
		locks: [u8, 0]
}
} ,

pub #[repr(C, packed)] pub struct smb2_lock_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 4 */
	pub Reserved: u16
} ,

pub #[repr(C, packed)] pub struct smb2_echo_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,

pub #[repr(C, packed)] pub struct smb2_echo_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 4 */
	pub Reserved: u16
} ,

/*
 * Valid FileInformation classes for query directory
 *
 * Note that these are a subset of the (file) QUERY_INFO levels defined
 * later in this file (but since QUERY_DIRECTORY uses equivalent numbers
 * we do not redefine them here)
 *
 * FileDirectoryInfomation		0x01
 * FileFullDirectoryInformation		0x02
 * FileIdFullDirectoryInformation	0x26
 * FileBothDirectoryInformation		0x03
 * FileIdBothDirectoryInformation	0x25
 * FileNamesInformation			0x0C
 * FileIdExtdDirectoryInformation	0x3C
 */

/* search (query_directory) Flags field */
pub const SMB2_RESTART_SCANS: u64 = 0x01,
pub const SMB2_RETURN_SINGLE_ENTRY: u64 = 0x02,
pub const SMB2_INDEX_SPECIFIED: u64 = 0x04,
pub const SMB2_REOPEN: u64 = 0x10,

pub #[repr(C, packed)] pub struct smb2_query_directory_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 33 */
	pub FileInformationClass: u8,
	pub Flags: u8,
	pub FileIndex: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	pub FileNameOffset: u16,
	pub FileNameLength: u16,
	pub OutputBufferLength: u32,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_query_directory_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 9 */
	pub OutputBufferOffset: u16,
	pub OutputBufferLength: u32,
	u8   Buffer: [u8, 0]
} ,

/* DeviceType Flags */
pub const FILE_DEVICE_CD_ROM: u64 = 0x00000002,
pub const FILE_DEVICE_CD_ROM_FILE_SYSTEM: u64 = 0x00000003,
pub const FILE_DEVICE_DFS: u64 = 0x00000006,
pub const FILE_DEVICE_DISK: u64 = 0x00000007,
pub const FILE_DEVICE_DISK_FILE_SYSTEM: u64 = 0x00000008,
pub const FILE_DEVICE_FILE_SYSTEM: u64 = 0x00000009,
pub const FILE_DEVICE_NAMED_PIPE: u64 = 0x00000011,
pub const FILE_DEVICE_NETWORK: u64 = 0x00000012,
pub const FILE_DEVICE_NETWORK_FILE_SYSTEM: u64 = 0x00000014,
pub const FILE_DEVICE_NULL: u64 = 0x00000015,
pub const FILE_DEVICE_PARALLEL_PORT: u64 = 0x00000016,
pub const FILE_DEVICE_PRINTER: u64 = 0x00000018,
pub const FILE_DEVICE_SERIAL_PORT: u64 = 0x0000001b,
pub const FILE_DEVICE_STREAMS: u64 = 0x0000001e,
pub const FILE_DEVICE_TAPE: u64 = 0x0000001f,
pub const FILE_DEVICE_TAPE_FILE_SYSTEM: u64 = 0x00000020,
pub const FILE_DEVICE_VIRTUAL_DISK: u64 = 0x00000024,
pub const FILE_DEVICE_NETWORK_REDIRECTOR: u64 = 0x00000028,

/* Device Characteristics */
pub const FILE_REMOVABLE_MEDIA: u64 = 0x00000001,
pub const FILE_READ_ONLY_DEVICE: u64 = 0x00000002,
pub const FILE_FLOPPY_DISKETTE: u64 = 0x00000004,
pub const FILE_WRITE_ONCE_MEDIA: u64 = 0x00000008,
pub const FILE_REMOTE_DEVICE: u64 = 0x00000010,
pub const FILE_DEVICE_IS_MOUNTED: u64 = 0x00000020,
pub const FILE_VIRTUAL_VOLUME: u64 = 0x00000040,
pub const FILE_DEVICE_SECURE_OPEN: u64 = 0x00000100,
pub const FILE_CHARACTERISTIC_TS_DEVICE: u64 = 0x00001000,
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: u64 = 0x00002000,
pub const FILE_PORTABLE_DEVICE: u64 = 0x00004000,
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: u64 = 0x00020000,

/*
 * Maximum number of iovs we need for a set-info request.
 * The largest one is rename/hardlink
 * [0] : struct smb2_set_info_req + smb2_file_[rename|link]_info
 * [1] : path
 * [2] : compound padding
 */
pub const SMB2_SET_INFO_IOV_SIZE: u64 = 3,

pub #[repr(C, packed)] pub struct smb2_set_info_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 33 */
	pub InfoType: u8,
	pub FileInfoClass: u8,
	pub BufferLength: u32,
	pub BufferOffset: u16,
	pub Reserved: u16,
	pub AdditionalInformation: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_set_info_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 2 */
} ,

/*
 * SMB2_NOTIFY  See MS-SMB2 section 2.2.35
 */
/* notify flags */
pub const SMB2_WATCH_TREE: u64 = 0x0001,

/* notify completion filter flags. See MS-FSCC 2.6 and MS-SMB2 2.2.35 */
pub const FILE_NOTIFY_CHANGE_FILE_NAME: u64 = 0x00000001,
pub const FILE_NOTIFY_CHANGE_DIR_NAME: u64 = 0x00000002,
pub const FILE_NOTIFY_CHANGE_NAME: u64 = 0x00000003,
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: u64 = 0x00000004,
pub const FILE_NOTIFY_CHANGE_SIZE: u64 = 0x00000008,
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: u64 = 0x00000010,
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: u64 = 0x00000020,
pub const FILE_NOTIFY_CHANGE_CREATION: u64 = 0x00000040,
pub const FILE_NOTIFY_CHANGE_EA: u64 = 0x00000080,
pub const FILE_NOTIFY_CHANGE_SECURITY: u64 = 0x00000100,
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: u64 = 0x00000200,
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: u64 = 0x00000400,
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: u64 = 0x00000800,

/* See MS-SMB2 2.2.35 */

pub #[repr(C, packed)] pub struct smb2_change_notify_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,
	pub Flags: u16,
	pub OutputBufferLength: u32,
	pub PersistentFileId: u64, /* opaque endianness */
	pub VolatileFileId: u64, /* opaque endianness */
	pub CompletionFilter: u32,
	pub Reserved: u32
} ,

/* See MS-SMB2 2.2.36 */

pub #[repr(C, packed)] pub struct smb2_change_notify_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,  /* Must be 9 */
	pub OutputBufferOffset: u16,
	pub OutputBufferLength: u32,
	Buffer: [u8, 0], /* array of file notify structs */
} ,

/*
 * SMB2_SERVER_TO_CLIENT_NOTIFICATION: See MS-SMB2 section 2.2.44
 */

pub const SMB2_NOTIFY_SESSION_CLOSED: u64 = 0x0000,

pub #[repr(C, packed)] pub struct smb2_server_client_notification {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,
	pub Reserved: u16, /* MBZ */
	pub NotificationType: u32,
	u8	NotificationBuffer[4], /* MBZ */
} ,

/*
 * SMB2_CREATE  See MS-SMB2 section 2.2.13
 */
/* Oplock levels */
pub const SMB2_OPLOCK_LEVEL_NONE: u64 = 0x00,
pub const SMB2_OPLOCK_LEVEL_II: u64 = 0x01,
pub const SMB2_OPLOCK_LEVEL_EXCLUSIVE: u64 = 0x08,
pub const SMB2_OPLOCK_LEVEL_BATCH: u64 = 0x09,
pub const SMB2_OPLOCK_LEVEL_LEASE: u64 = 0xFF,
/* Non-spec internal type */
pub const SMB2_OPLOCK_LEVEL_NOCHANGE: u64 = 0x99,

/* Impersonation Levels. See MS-WPO section 9.7 and MSDN-IMPERS */
pub const IL_ANONYMOUS: u64 = (0x00000000),
pub const IL_IDENTIFICATION: u64 = (0x00000001),
pub const IL_IMPERSONATION: u64 = (0x00000002),
pub const IL_DELEGATE: u64 = (0x00000003),

/* Desired Access Flags */
pub const FILE_READ_DATA_LE: u64 = (0x00000001),
pub const FILE_LIST_DIRECTORY_LE: u64 = (0x00000001),
pub const FILE_WRITE_DATA_LE: u64 = (0x00000002),
pub const FILE_APPEND_DATA_LE: u64 = (0x00000004),
pub const FILE_ADD_SUBDIRECTORY_LE: u64 = (0x00000004),
pub const FILE_READ_EA_LE: u64 = (0x00000008),
pub const FILE_WRITE_EA_LE: u64 = (0x00000010),
pub const FILE_EXECUTE_LE: u64 = (0x00000020),
pub const FILE_DELETE_CHILD_LE: u64 = (0x00000040),
pub const FILE_READ_ATTRIBUTES_LE: u64 = (0x00000080),
pub const FILE_WRITE_ATTRIBUTES_LE: u64 = (0x00000100),
pub const FILE_DELETE_LE: u64 = (0x00010000),
pub const FILE_READ_CONTROL_LE: u64 = (0x00020000),
pub const FILE_WRITE_DAC_LE: u64 = (0x00040000),
pub const FILE_WRITE_OWNER_LE: u64 = (0x00080000),
pub const FILE_SYNCHRONIZE_LE: u64 = (0x00100000),
pub const FILE_ACCESS_SYSTEM_SECURITY_LE: u64 = (0x01000000),
pub const FILE_MAXIMAL_ACCESS_LE: u64 = (0x02000000),
pub const FILE_GENERIC_ALL_LE: u64 = (0x10000000),
pub const FILE_GENERIC_EXECUTE_LE: u64 = (0x20000000),
pub const FILE_GENERIC_WRITE_LE: u64 = (0x40000000),
pub const FILE_GENERIC_READ_LE: u64 = (0x80000000),
pub const DESIRED_ACCESS_MASK: u64 = (0xF21F01FF),


pub const FILE_READ_DESIRED_ACCESS_LE: u64 = (FILE_READ_DATA_LE        |	,
					 FILE_READ_EA_LE          |     \
					 FILE_GENERIC_READ_LE)
pub const FILE_WRITE_DESIRE_ACCESS_LE: u64 = (FILE_WRITE_DATA_LE       |	,
					 FILE_APPEND_DATA_LE      |	\
					 FILE_WRITE_EA_LE         |	\
					 FILE_WRITE_ATTRIBUTES_LE |	\
					 FILE_GENERIC_WRITE_LE)

/* ShareAccess Flags */
pub const FILE_SHARE_READ_LE: u64 = (0x00000001),
pub const FILE_SHARE_WRITE_LE: u64 = (0x00000002),
pub const FILE_SHARE_DELETE_LE: u64 = (0x00000004),
pub const FILE_SHARE_ALL_LE: u64 = (0x00000007),

/* CreateDisposition Flags */
pub const FILE_SUPERSEDE_LE: u64 = (0x00000000),
pub const FILE_OPEN_LE: u64 = (0x00000001),
pub const FILE_CREATE_LE: u64 = (0x00000002),
pub const FILE_OPEN_IF_LE: u64 = (0x00000003),
pub const FILE_OVERWRITE_LE: u64 = (0x00000004),
pub const FILE_OVERWRITE_IF_LE: u64 = (0x00000005),
pub const FILE_CREATE_MASK_LE: u64 = (0x00000007),

/* CreateOptions Flags */
pub const FILE_DIRECTORY_FILE_LE: u64 = (0x00000001),
/* same as pub const CREATE_NOT_FILE_LE: u64 = (0x00000001) */,
pub const FILE_WRITE_THROUGH_LE: u64 = (0x00000002),
pub const FILE_SEQUENTIAL_ONLY_LE: u64 = (0x00000004),
pub const FILE_NO_INTERMEDIATE_BUFFERING_LE: u64 = (0x00000008),
/* FILE_SYNCHRONOUS_IO_ALERT_LE		(0x00000010) should be zero, ignored */
/* FILE_SYNCHRONOUS_IO_NONALERT		(0x00000020) should be zero, ignored */
pub const FILE_NON_DIRECTORY_FILE_LE: u64 = (0x00000040),
pub const FILE_COMPLETE_IF_OPLOCKED_LE: u64 = (0x00000100),
pub const FILE_NO_EA_KNOWLEDGE_LE: u64 = (0x00000200),
/* FILE_OPEN_REMOTE_INSTANCE		(0x00000400) should be zero, ignored */
pub const FILE_RANDOM_ACCESS_LE: u64 = (0x00000800),
pub const FILE_DELETE_ON_CLOSE_LE: u64 = (0x00001000) /* MBZ */,
pub const FILE_OPEN_BY_FILE_ID_LE: u64 = (0x00002000),
pub const FILE_OPEN_FOR_BACKUP_INTENT_LE: u64 = (0x00004000),
pub const FILE_NO_COMPRESSION_LE: u64 = (0x00008000),
/* FILE_OPEN_REQUIRING_OPLOCK		(0x00010000) should be zero, ignored */
/* FILE_DISALLOW_EXCLUSIVE		(0x00020000) should be zero, ignored */
/* FILE_RESERVE_OPFILTER		(0x00100000) MBZ */
pub const FILE_OPEN_REPARSE_POINT_LE: u64 = (0x00200000),
pub const FILE_OPEN_NO_RECALL_LE: u64 = (0x00400000),
/* pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u64 = (0x00800000) should be zero, ignored */,
pub const CREATE_OPTIONS_MASK_LE: u64 = (0x00FFFFFF),

pub const FILE_READ_RIGHTS_LE: u64 = (FILE_READ_DATA_LE | FILE_READ_EA_LE ,
			| FILE_READ_ATTRIBUTES_LE)
pub const FILE_WRITE_RIGHTS_LE: u64 = (FILE_WRITE_DATA_LE | FILE_APPEND_DATA_LE ,
			| FILE_WRITE_EA_LE | FILE_WRITE_ATTRIBUTES_LE)
pub const FILE_EXEC_RIGHTS_LE: u64 = (FILE_EXECUTE_LE),

/* Create Context Values */
pub const SMB2_CREATE_EA_BUFFER: u64 = "ExtA" /* extended attributes */,
pub const SMB2_CREATE_SD_BUFFER: u64 = "SecD" /* security descriptor */,
pub const SMB2_CREATE_DURABLE_HANDLE_REQUEST: u64 = "DHnQ",
pub const SMB2_CREATE_DURABLE_HANDLE_RECONNECT: u64 = "DHnC",
pub const SMB2_CREATE_ALLOCATION_SIZE: u64 = "AlSi",
pub const SMB2_CREATE_QUERY_MAXIMAL_ACCESS_REQUEST: u64 = "MxAc",
pub const SMB2_CREATE_TIMEWARP_REQUEST: u64 = "TWrp",
pub const SMB2_CREATE_QUERY_ON_DISK_ID: u64 = "QFid",
pub const SMB2_CREATE_REQUEST_LEASE: u64 = "RqLs",
pub const SMB2_CREATE_DURABLE_HANDLE_REQUEST_V2: u64 = "DH2Q",
pub const SMB2_CREATE_DURABLE_HANDLE_RECONNECT_V2: u64 = "DH2C",
pub const SMB2_CREATE_TAG_POSIX: u64 = "x93xADx25x50x9CxB4x11xE7xB4x23x83xDEx96x8BxCDx7C",
pub const SMB2_CREATE_APP_INSTANCE_ID: u64 = "x45xBCxA6x6AxEFxA7xF7x4Ax90x08xFAx46x2Ex14x4Dx74",
pub const SMB2_CREATE_APP_INSTANCE_VERSION: u64 = "xB9x82xD0xB7x3Bx56x07x4FxA0x7Bx52x4Ax81x16xA0x10",
pub const SVHDX_OPEN_DEVICE_CONTEXT: u64 = "x9CxCBxCFx9Ex04xC1xE6x43x98x0Ex15x8DxA1xF6xECx83",
pub const SMB2_CREATE_TAG_AAPL: u64 = "AAPL",

/* Flag (SMB3 open response) values */
pub const SMB2_CREATE_FLAG_REPARSEPOINT: u64 = 0x01,

pub #[repr(C, packed)] pub struct create_context {
	/* New members must be added within the struct_group() macro below. */
	__struct_group(create_context_hdr, hdr, ,
		pub Next: u32,
		pub NameOffset: u16,
		pub NameLength: u16,
		pub Reserved: u16,
		pub DataOffset: u16,
		pub DataLength: u32,
	),
	u8 Buffer: [u8, 0]
} ,


pub #[repr(C, packed)] pub struct smb2_create_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 57 */
	pub SecurityFlags: u8,
	pub RequestedOplockLevel: u8,
	pub ImpersonationLevel: u32,
	pub SmbCreateFlags: u64,
	pub Reserved: u64,
	pub DesiredAccess: u32,
	pub FileAttributes: u32,
	pub ShareAccess: u32,
	pub CreateDisposition: u32,
	pub CreateOptions: u32,
	pub NameOffset: u16,
	pub NameLength: u16,
	pub CreateContextsOffset: u32,
	pub CreateContextsLength: u32,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_create_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16,	/* Must be 89 */
	pub OplockLevel: u8,
	pub Flags: u8,  /* 0x01 if reparse point */
	pub CreateAction: u32,
	pub CreationTime: u64,
	pub LastAccessTime: u64,
	pub LastWriteTime: u64,
	pub ChangeTime: u64,
	pub AllocationSize: u64,
	pub EndofFile: u64,
	pub FileAttributes: u32,
	pub Reserved2: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	pub CreateContextsOffset: u32,
	pub CreateContextsLength: u32,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct create_posix {
	pub ccontext: struct create_context_hdr,
	u8    Name[16],
	pub Mode: u32,
	pub Reserved: u32
} ,

/* See MS-SMB2 2.2.13.2.3 and MS-SMB2 2.2.13.2.4 */
pub #[repr(C, packed)] pub struct TmpTypedef {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		u8  Reserved[16],
		#[repr(C)] pub #[repr(C, packed)] pub struct AnonymousStruct {
			pub PersistentFileId: u64,
			pub VolatileFileId: u64
} Fid
} Data
}  create_durable_req_t, create_durable_reconn_t,

/* See MS-SMB2 2.2.13.2.5 */

pub #[repr(C, packed)] pub struct create_mxac_req {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub Timestamp: u64
} ,

/*
 * AAPL flags. See Samba libcli/smb/smb2_create_ctx.h
 */

/* "AAPL" Context Command Codes */
pub const SMB2_CRTCTX_AAPL_SERVER_QUERY: u64 = 1,
pub const SMB2_CRTCTX_AAPL_RESOLVE_ID: u64 = 2,

/* "AAPL" Server Query request/response bitmap */
pub const SMB2_CRTCTX_AAPL_SERVER_CAPS: u64 = 1,
pub const SMB2_CRTCTX_AAPL_VOLUME_CAPS: u64 = 2,
pub const SMB2_CRTCTX_AAPL_MODEL_INFO: u64 = 4,

/* "AAPL" Client/Server Capabilities bitmap */
pub const SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR: u64 = 1,
pub const SMB2_CRTCTX_AAPL_SUPPORTS_OSX_COPYFILE: u64 = 2,
pub const SMB2_CRTCTX_AAPL_UNIX_BASED: u64 = 4,
pub const SMB2_CRTCTX_AAPL_SUPPORTS_NFS_ACE: u64 = 8,
/*
 * V2 extends the same inline-FinderInfo mechanism as
 * SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR with an added flags field,
 * confirmed byte-identical to V1 otherwise against AAPL's actual
 * public client behavior.  Mutually exclusive with the V1 bit on
 * the wire, not both set together.
 */
pub const SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR_V2: u64 = 16,

/* "AAPL" Volume Capabilities bitmap */
pub const SMB2_CRTCTX_AAPL_SUPPORT_RESOLVE_ID: u64 = 1,
pub const SMB2_CRTCTX_AAPL_CASE_SENSITIVE: u64 = 2,
pub const SMB2_CRTCTX_AAPL_FULL_SYNC: u64 = 4,

/*
 * Flags
 * See MS-SMB2 2.2.13.2.11
 *     MS-SMB2 2.2.13.2.12
 *     MS-SMB2 2.2.14.2.12
 */
pub const SMB2_DHANDLE_FLAG_PERSISTENT: u64 = 0x00000002,

/* See MS-SMB2 2.2.13.2.11 */

pub #[repr(C, packed)] pub struct durable_context_v2_req {
	pub Timeout: u32,
	pub Flags: u32, /* see SMB2_DHANDLE_FLAG_PERSISTENT */
	pub Reserved: u64,
	u8 CreateGuid[16]
} ,

pub #[repr(C, packed)] pub struct create_durable_req_v2 {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub dcontext: struct durable_context_v2_req
} ,

/* See MS-SMB2 2.2.13.2.12 */

pub #[repr(C, packed)] pub struct durable_reconnect_context_v2 {
	#[repr(C)] pub #[repr(C, packed)] pub struct AnonymousStruct {
		pub PersistentFileId: u64,
		pub VolatileFileId: u64
} Fid,
	u8 CreateGuid[16],
	pub Flags: u32, /* see SMB2_DHANDLE_FLAG_PERSISTENT */
} ,

pub #[repr(C, packed)] pub struct create_durable_handle_reconnect_v2 {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub dcontext: struct durable_reconnect_context_v2,
	u8 Pad[4]
} ,

/* See MS-SMB2 2.2.14.2.12 */

pub #[repr(C, packed)] pub struct durable_context_v2_rsp {
	pub Timeout: u32,
	pub Flags: u32, /* see SMB2_DHANDLE_FLAG_PERSISTENT */
} ,

pub #[repr(C, packed)] pub struct create_durable_rsp_v2 {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub dcontext: struct durable_context_v2_rsp
} ,

/* See MS-SMB2 2.2.14.2.5 */

pub #[repr(C, packed)] pub struct create_mxac_rsp {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub QueryStatus: u32,
	pub MaximalAccess: u32
} ,

pub const SMB2_LEASE_NONE_LE: u64 = (0x00),
pub const SMB2_LEASE_READ_CACHING_LE: u64 = (0x01),
pub const SMB2_LEASE_HANDLE_CACHING_LE: u64 = (0x02),
pub const SMB2_LEASE_WRITE_CACHING_LE: u64 = (0x04),

pub const SMB2_LEASE_FLAG_BREAK_IN_PROGRESS_LE: u64 = (0x02),
pub const SMB2_LEASE_FLAG_PARENT_LEASE_KEY_SET_LE: u64 = (0x04),

pub const SMB2_LEASE_KEY_SIZE: u64 = 16,

/* See MS-SMB2 2.2.13.2.8 */

pub #[repr(C, packed)] pub struct lease_context {
	u8 LeaseKey[SMB2_LEASE_KEY_SIZE],
	pub LeaseState: u32,
	pub LeaseFlags: u32,
	pub LeaseDuration: u64
} ,

/* See MS-SMB2 2.2.13.2.10 */

pub #[repr(C, packed)] pub struct lease_context_v2 {
	u8 LeaseKey[SMB2_LEASE_KEY_SIZE],
	pub LeaseState: u32,
	pub LeaseFlags: u32,
	pub LeaseDuration: u64,
	u8 ParentLeaseKey[SMB2_LEASE_KEY_SIZE],
	pub Epoch: u16,
	pub Reserved: u16
} ,

pub #[repr(C, packed)] pub struct create_lease {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub lcontext: struct lease_context
} ,

pub #[repr(C, packed)] pub struct create_lease_v2 {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub lcontext: struct lease_context_v2,
	u8   Pad[4]
} ,

/* See MS-SMB2 2.2.14.2.9 */

pub #[repr(C, packed)] pub struct create_disk_id_rsp {
	pub ccontext: struct create_context_hdr,
	u8   Name[8],
	pub DiskFileId: u64,
	pub VolumeId: u64,
	u8  Reserved[16]
} ,

/* See MS-SMB2 2.2.13.2.13 */

pub #[repr(C, packed)] pub struct create_app_inst_id {
	pub ccontext: struct create_context_hdr,
	u8 Name[16],
	pub StructureSize: u32, /* Must be 20 */
	pub Reserved: u16,
	u8 AppInstanceId[16]
} ,

/* See MS-SMB2 2.2.13.2.15 */

pub #[repr(C, packed)] pub struct create_app_inst_id_vers {
	pub ccontext: struct create_context_hdr,
	u8 Name[16],
	pub StructureSize: u32, /* Must be 24 */
	pub Reserved: u16,
	pub Padding: u32,
	pub AppInstanceVersionHigh: u64,
	pub AppInstanceVersionLow: u64
} ,

/* See MS-SMB2 2.2.31 and 2.2.32 */

pub #[repr(C, packed)] pub struct smb2_ioctl_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 57 */
	pub Reserved: u16, /* offset from start of SMB2 header to write data */
	pub CtlCode: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	pub InputOffset: u32, /* Reserved MBZ */
	pub InputCount: u32,
	pub MaxInputResponse: u32,
	pub OutputOffset: u32,
	pub OutputCount: u32,
	pub MaxOutputResponse: u32,
	pub Flags: u32,
	pub Reserved2: u32,
	u8   Buffer: [u8, 0]
} ,

/* See MS-SMB2 2.2.31.1.1 */

pub #[repr(C, packed)] pub struct srv_copychunk {
	pub SourceOffset: u64,
	pub TargetOffset: u64,
	pub Length: u32,
	pub Reserved: u32
} ,

pub const COPY_CHUNK_RES_KEY_SIZE: u64 = 24,

/* See MS-SMB2 2.2.31.1 */
/* this goes in the ioctl buffer when doing a copychunk request */

pub #[repr(C, packed)] pub struct copychunk_ioctl_req {
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		u8 SourceKey[COPY_CHUNK_RES_KEY_SIZE],
		u64 SourceKeyU64[3]
},
	pub ChunkCount: u32,
	pub Reserved: u32,
	struct srv_copychunk Chunks[] 
} ,

/* See MS-SMB2 2.2.32.1 */

pub #[repr(C, packed)] pub struct copychunk_ioctl_rsp {
	pub ChunksWritten: u32,
	pub ChunkBytesWritten: u32,
	pub TotalBytesWritten: u32
} ,

/* See MS-SMB2 2.2.32.3 */

pub #[repr(C, packed)] pub struct resume_key_ioctl_rsp {
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		u8 ResumeKey[COPY_CHUNK_RES_KEY_SIZE],
		u64 ResumeKeyU64[3]
},
	pub ContextLength: u32,	/* MBZ */
	u8	Context[4],	/* ignored, Windows sets to 4 bytes of zero */
} ,

pub #[repr(C, packed)] pub struct smb2_ioctl_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 49 */
	pub Reserved: u16,
	pub CtlCode: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	pub InputOffset: u32, /* Reserved MBZ */
	pub InputCount: u32,
	pub OutputOffset: u32,
	pub OutputCount: u32,
	pub Flags: u32,
	pub Reserved2: u32,
	u8   Buffer: [u8, 0]
} ,

/* See MS-SMB2 2.2.32.5.1.1 */

pub #[repr(C, packed)] pub struct smb_sockaddr_in {
	pub Port: u16,
	pub IPv4Address: u32,
	u8   Reserved[8]
} ,

/* See MS-SMB2 2.2.32.5.1.2 */

pub #[repr(C, packed)] pub struct smb_sockaddr_in6 {
	pub Port: u16,
	pub FlowInfo: u32,
	u8   IPv6Address[16],
	pub ScopeId: u32
} ,

/* See MS-SMB2 2.2.32.5 and MS-SMB2 2.2.32.5.1 */
pub const RSS_CAPABLE: u64 = (0x00000001),
pub const RDMA_CAPABLE: u64 = (0x00000002),
pub const INTERNETWORK: u64 = (0x0002),
pub const INTERNETWORKV6: u64 = (0x0017),

pub #[repr(C, packed)] pub struct network_interface_info_ioctl_rsp {
	pub Next: u32, /* next interface. zero if this is last one */
	pub IfIndex: u32,
	pub Capability: u32, /* RSS or RDMA Capable */
	pub Reserved: u32,
	pub LinkSpeed: u64,
	#[repr(C)] pub #[repr(C)] pub union AnonymousUnion {
		u8	SockAddr_Storage[128],
		#[repr(C)] pub #[repr(C, packed)] pub struct AnonymousStruct {
			pub Family: u16,
			u8 Buffer[126]
}
}
} ,

/* Integrity ChecksumAlgorithm choices for above */
pub const CHECKSUM_TYPE_NONE: u64 = 0x0000,
pub const CHECKSUM_TYPE_CRC64: u64 = 0x0002,
pub const CHECKSUM_TYPE_UNCHANGED: u64 = 0xFFFF	/* set only */,

/* Integrity flags for above */
pub const FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF: u64 = 0x00000001,

pub #[repr(C, packed)] pub struct validate_negotiate_info_req {
	pub Capabilities: u32,
	u8   Guid[SMB2_CLIENT_GUID_SIZE],
	pub SecurityMode: u16,
	pub DialectCount: u16,
	u16 Dialects[4], /* BB expand this if autonegotiate > 4 dialects */
} ,

pub #[repr(C, packed)] pub struct validate_negotiate_info_rsp {
	pub Capabilities: u32,
	u8   Guid[SMB2_CLIENT_GUID_SIZE],
	pub SecurityMode: u16,
	pub Dialect: u16, /* Dialect in use for the connection */
} ,


/* Possible InfoType values */
pub const SMB2_O_INFO_FILE: u64 = 0x01,
pub const SMB2_O_INFO_FILESYSTEM: u64 = 0x02,
pub const SMB2_O_INFO_SECURITY: u64 = 0x03,
pub const SMB2_O_INFO_QUOTA: u64 = 0x04,

/* SMB2 Query Info see MS-SMB2 (2.2.37) or MS-DTYP */

/* List of QUERY INFO levels (those also valid for QUERY_DIR are noted below */
pub const FILE_DIRECTORY_INFORMATION: u64 = 1	/* also for QUERY_DIR */,
pub const FILE_FULL_DIRECTORY_INFORMATION: u64 = 2	/* also for QUERY_DIR */,
pub const FILE_BOTH_DIRECTORY_INFORMATION: u64 = 3	/* also for QUERY_DIR */,
pub const FILE_BASIC_INFORMATION: u64 = 4,
pub const FILE_STANDARD_INFORMATION: u64 = 5,
pub const FILE_INTERNAL_INFORMATION: u64 = 6,
pub const FILE_EA_INFORMATION: u64 = 7,
pub const FILE_ACCESS_INFORMATION: u64 = 8,
pub const FILE_NAME_INFORMATION: u64 = 9,
pub const FILE_RENAME_INFORMATION: u64 = 10,
pub const FILE_LINK_INFORMATION: u64 = 11,
pub const FILE_NAMES_INFORMATION: u64 = 12	/* also for QUERY_DIR */,
pub const FILE_DISPOSITION_INFORMATION: u64 = 13,
pub const FILE_POSITION_INFORMATION: u64 = 14,
pub const FILE_FULL_EA_INFORMATION: u64 = 15,
pub const FILE_MODE_INFORMATION: u64 = 16,
pub const FILE_ALIGNMENT_INFORMATION: u64 = 17,
pub const FILE_ALL_INFORMATION: u64 = 18,
pub const FILE_ALLOCATION_INFORMATION: u64 = 19,
pub const FILE_END_OF_FILE_INFORMATION: u64 = 20,
pub const FILE_ALTERNATE_NAME_INFORMATION: u64 = 21,
pub const FILE_STREAM_INFORMATION: u64 = 22,
pub const FILE_PIPE_INFORMATION: u64 = 23,
pub const FILE_PIPE_LOCAL_INFORMATION: u64 = 24,
pub const FILE_PIPE_REMOTE_INFORMATION: u64 = 25,
pub const FILE_MAILSLOT_QUERY_INFORMATION: u64 = 26,
pub const FILE_MAILSLOT_SET_INFORMATION: u64 = 27,
pub const FILE_COMPRESSION_INFORMATION: u64 = 28,
pub const FILE_OBJECT_ID_INFORMATION: u64 = 29,
/* Number 30 not defined in documents */
pub const FILE_MOVE_CLUSTER_INFORMATION: u64 = 31,
pub const FILE_QUOTA_INFORMATION: u64 = 32,
pub const FILE_REPARSE_POINT_INFORMATION: u64 = 33,
pub const FILE_NETWORK_OPEN_INFORMATION: u64 = 34,
pub const FILE_ATTRIBUTE_TAG_INFORMATION: u64 = 35,
pub const FILE_TRACKING_INFORMATION: u64 = 36,
pub const FILEID_BOTH_DIRECTORY_INFORMATION: u64 = 37	/* also for QUERY_DIR */,
pub const FILEID_FULL_DIRECTORY_INFORMATION: u64 = 38	/* also for QUERY_DIR */,
pub const FILE_VALID_DATA_LENGTH_INFORMATION: u64 = 39,
pub const FILE_SHORT_NAME_INFORMATION: u64 = 40,
pub const FILE_SFIO_RESERVE_INFORMATION: u64 = 44,
pub const FILE_SFIO_VOLUME_INFORMATION: u64 = 45,
pub const FILE_HARD_LINK_INFORMATION: u64 = 46,
pub const FILE_NORMALIZED_NAME_INFORMATION: u64 = 48,
pub const FILEID_GLOBAL_TX_DIRECTORY_INFORMATION: u64 = 50,
pub const FILE_STANDARD_LINK_INFORMATION: u64 = 54,
pub const FILE_ID_INFORMATION: u64 = 59,
pub const FILE_ID_EXTD_DIRECTORY_INFORMATION: u64 = 60	/* also for QUERY_DIR */,
pub const FileId64ExtdDirectoryInformation: u64 = 78	/* also for QUERY_DIR */,
pub const FileId64ExtdBothDirectoryInformation: u64 = 79 /* also for QUERY_DIR */,
pub const FileIdAllExtdDirectoryInformation: u64 = 80	/* also for QUERY_DIR */,
pub const FileIdAllExtdBothDirectoryInformation: u64 = 81 /* also for QUERY_DIR */,
/* Used for Query Info and Find File POSIX Info for SMB3.1.1 and SMB1 */
pub const SMB_FIND_FILE_POSIX_INFO: u64 = 0x064,

/* Security info type additionalinfo flags. */
pub const OWNER_SECINFO: u64 = 0x00000001,
pub const GROUP_SECINFO: u64 = 0x00000002,
pub const DACL_SECINFO: u64 = 0x00000004,
pub const SACL_SECINFO: u64 = 0x00000008,
pub const LABEL_SECINFO: u64 = 0x00000010,
pub const ATTRIBUTE_SECINFO: u64 = 0x00000020,
pub const SCOPE_SECINFO: u64 = 0x00000040,
pub const BACKUP_SECINFO: u64 = 0x00010000,
pub const UNPROTECTED_SACL_SECINFO: u64 = 0x10000000,
pub const UNPROTECTED_DACL_SECINFO: u64 = 0x20000000,
pub const PROTECTED_SACL_SECINFO: u64 = 0x40000000,
pub const PROTECTED_DACL_SECINFO: u64 = 0x80000000,

/* Flags used for FileFullEAinfo */
pub const SL_RESTART_SCAN: u64 = 0x00000001,
pub const SL_RETURN_SINGLE_ENTRY: u64 = 0x00000002,
pub const SL_INDEX_SPECIFIED: u64 = 0x00000004,

pub #[repr(C, packed)] pub struct smb2_query_info_req {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 41 */
	pub InfoType: u8,
	pub FileInfoClass: u8,
	pub OutputBufferLength: u32,
	pub InputBufferOffset: u16,
	pub Reserved: u16,
	pub InputBufferLength: u32,
	pub AdditionalInformation: u32,
	pub Flags: u32,
	pub PersistentFileId: u64,
	pub VolatileFileId: u64,
	u8   Buffer: [u8, 0]
} ,

pub #[repr(C, packed)] pub struct smb2_query_info_rsp {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 9 */
	pub OutputBufferOffset: u16,
	pub OutputBufferLength: u32,
	u8   Buffer: [u8, 0]
} ,

/* Level 100 query info */

pub #[repr(C, packed)] pub struct smb311_posix_qinfo {
	pub CreationTime: u64,
	pub LastAccessTime: u64,
	pub LastWriteTime: u64,
	pub ChangeTime: u64,
	pub EndOfFile: u64,
	pub AllocationSize: u64,
	pub DosAttributes: u32,
	pub Inode: u64,
	pub DeviceId: u32,
	pub Zero: u32,
	/* beginning of POSIX Create Context Response */
	pub HardLinks: u32,
	pub ReparseTag: u32,
	pub Mode: u32,
	u8     Sids: [u8, 0],
	/*
	 * var sized owner SID
	 * var sized group SID
	 * le32 filenamelength
	 * u8  filename[]
	 */
} ,

/* See MS-SMB2 2.2.23 through 2.2.25 */

pub #[repr(C, packed)] pub struct smb2_oplock_break {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 24 */
	pub OplockLevel: u8,
	pub Reserved: u8,
	pub Reserved2: u32,
	pub PersistentFid: u64,
	pub VolatileFid: u64
} ,

pub const SMB2_NOTIFY_BREAK_LEASE_FLAG_ACK_REQUIRED: u64 = (0x01),

pub #[repr(C, packed)] pub struct smb2_lease_break {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 44 */
	pub Epoch: u16,
	pub Flags: u32,
	u8   LeaseKey[16],
	pub CurrentLeaseState: u32,
	pub NewLeaseState: u32,
	pub BreakReason: u32,
	pub AccessMaskHint: u32,
	pub ShareMaskHint: u32
} ,

pub #[repr(C, packed)] pub struct smb2_lease_ack {
	pub hdr: struct smb2_hdr,
	pub StructureSize: u16, /* Must be 36 */
	pub Reserved: u16,
	pub Flags: u32,
	u8   LeaseKey[16],
	pub LeaseState: u32,
	pub LeaseDuration: u64
} ,

pub const OP_BREAK_STRUCT_SIZE_20: u64 = 24,
pub const OP_BREAK_STRUCT_SIZE_21: u64 = 36,

/*
 * See MS-SMB2 2.2.13.1.1
 *     MS-SMB 2.2.1.4.1
 * These are the file access permission bits defined in CIFS for the
 * NTCreateAndX as well as the level 0x107
 * TRANS2_QUERY_PATH_INFORMATION API.  The level 0x107, SMB_QUERY_FILE_ALL_INFO
 * responds with the AccessFlags.
 * The AccessFlags specifies the access permissions a caller has to the
 * file and can have any suitable combination of the following values:
 */
pub const FILE_READ_DATA: u64 = 0x00000001  /* Data can be read from the file   */,
					  /* or directory child entries can   */
					  /* be listed together with the      */
					  /* associated child attributes      */
					  /* (so the FILE_READ_ATTRIBUTES on  */
					  /* the child entry is not needed)   */
pub const FILE_WRITE_DATA: u64 = 0x00000002  /* Data can be written to the file  */,
					  /* or new file can be created in    */
					  /* the directory                    */
pub const FILE_APPEND_DATA: u64 = 0x00000004  /* Data can be appended to the file */,
					  /* (for non-local files over SMB it */
					  /* is same as FILE_WRITE_DATA)      */
					  /* or new subdirectory can be       */
					  /* created in the directory         */
pub const FILE_READ_EA: u64 = 0x00000008  /* Extended attributes associated   */,
					  /* with the file can be read        */
pub const FILE_WRITE_EA: u64 = 0x00000010  /* Extended attributes associated   */,
					  /* with the file can be written     */
pub const FILE_EXECUTE: u64 = 0x00000020  /*Data can be read into memory from */,
					  /* the file using system paging I/O */
					  /* for executing the file / script  */
					  /* or right to traverse directory   */
					  /* (but by default all users have   */
					  /* directory bypass traverse        */
					  /* privilege and do not need this   */
					  /* permission on directories at all)*/
pub const FILE_DELETE_CHILD: u64 = 0x00000040  /* Child entry can be deleted from  */,
					  /* the directory (so the DELETE on  */
					  /* the child entry is not needed)   */
pub const FILE_READ_ATTRIBUTES: u64 = 0x00000080  /* Attributes associated with the   */,
					  /* file or directory can be read    */
pub const FILE_WRITE_ATTRIBUTES: u64 = 0x00000100  /* Attributes associated with the   */,
					  /* file or directory can be written */
pub const DELETE: u64 = 0x00010000  /* The file or dir can be deleted   */,
pub const READ_CONTROL: u64 = 0x00020000  /* The discretionary access control */,
					  /* list and ownership associated    */
					  /* with the file or dir can be read */
pub const WRITE_DAC: u64 = 0x00040000  /* The discretionary access control */,
					  /* list associated with the file or */
					  /* directory can be written         */
pub const WRITE_OWNER: u64 = 0x00080000  /* Ownership information associated */,
					  /* with the file/dir can be written */
pub const SYNCHRONIZE: u64 = 0x00100000  /* The file handle can waited on to */,
					  /* synchronize with the completion  */
					  /* of an input/output request       */
pub const SYSTEM_SECURITY: u64 = 0x01000000  /* The system access control list   */,
					  /* associated with the file or      */
					  /* directory can be read or written */
					  /* (cannot be in DACL, can in SACL) */
pub const MAXIMUM_ALLOWED: u64 = 0x02000000  /* Maximal subset of GENERIC_ALL    */,
					  /* permissions which can be granted */
					  /* (cannot be in DACL nor SACL)     */
pub const GENERIC_ALL: u64 = 0x10000000  /* Same as: GENERIC_EXECUTE |       */,
					  /*          GENERIC_WRITE |         */
					  /*          GENERIC_READ |          */
					  /*          FILE_DELETE_CHILD |     */
					  /*          DELETE |                */
					  /*          WRITE_DAC |             */
					  /*          WRITE_OWNER             */
					  /* So GENERIC_ALL contains all bits */
					  /* mentioned above except these two */
					  /* SYSTEM_SECURITY  MAXIMUM_ALLOWED */
pub const GENERIC_EXECUTE: u64 = 0x20000000  /* Same as: FILE_EXECUTE |          */,
					  /*          FILE_READ_ATTRIBUTES |  */
					  /*          READ_CONTROL |          */
					  /*          SYNCHRONIZE             */
pub const GENERIC_WRITE: u64 = 0x40000000  /* Same as: FILE_WRITE_DATA |       */,
					  /*          FILE_APPEND_DATA |      */
					  /*          FILE_WRITE_EA |         */
					  /*          FILE_WRITE_ATTRIBUTES | */
					  /*          READ_CONTROL |          */
					  /*          SYNCHRONIZE             */
pub const GENERIC_READ: u64 = 0x80000000  /* Same as: FILE_READ_DATA |        */,
					  /*          FILE_READ_EA |          */
					  /*          FILE_READ_ATTRIBUTES |  */
					  /*          READ_CONTROL |          */
					  /*          SYNCHRONIZE             */

/* Combinations of file access permission bits */
pub const FILE_READ_RIGHTS: u64 = (FILE_READ_DATA | FILE_READ_EA | FILE_READ_ATTRIBUTES),
pub const FILE_WRITE_RIGHTS: u64 = (FILE_WRITE_DATA | FILE_APPEND_DATA ,
			| FILE_WRITE_EA | FILE_WRITE_ATTRIBUTES)
pub const FILE_EXEC_RIGHTS: u64 = (FILE_EXECUTE),
pub const SET_FILE_EXEC_RIGHTS: u64 = (FILE_READ_EA | FILE_WRITE_EA | FILE_EXECUTE ,
				| FILE_READ_ATTRIBUTES \
				| FILE_WRITE_ATTRIBUTES \
				| DELETE | READ_CONTROL | WRITE_DAC \
				| WRITE_OWNER | SYNCHRONIZE)
pub const SET_MINIMUM_RIGHTS: u64 = (FILE_READ_EA | FILE_READ_ATTRIBUTES ,
				| READ_CONTROL | SYNCHRONIZE)



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
