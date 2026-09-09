/* SPDX-License-Identifier: LGPL-2.1 */
/* Rust translation of smb1pdu.h.  Types supplied by ../common/smb1pdu.h remain external. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64;
pub type __le16 = u16; pub type __le32 = u32; pub type __le64 = u64;

macro_rules! c { ($($n:ident = $v:expr;)+) => { $(pub const $n: u32 = $v;)+ }; }
c! {
 CIFS_PROT=0; POSIX_PROT=CIFS_PROT+1; BAD_PROT=0xffff;
 SMB_COM_CREATE_DIRECTORY=0x00; SMB_COM_DELETE_DIRECTORY=0x01; SMB_COM_CLOSE=0x04; SMB_COM_FLUSH=0x05; SMB_COM_DELETE=0x06; SMB_COM_RENAME=0x07; SMB_COM_QUERY_INFORMATION=0x08; SMB_COM_SETATTR=0x09; SMB_COM_LOCKING_ANDX=0x24; SMB_COM_COPY=0x29; SMB_COM_ECHO=0x2b; SMB_COM_OPEN_ANDX=0x2d; SMB_COM_READ_ANDX=0x2e; SMB_COM_WRITE_ANDX=0x2f; SMB_COM_TRANSACTION2=0x32; SMB_COM_TRANSACTION2_SECONDARY=0x33; SMB_COM_FIND_CLOSE2=0x34; SMB_COM_TREE_DISCONNECT=0x71; SMB_COM_NEGOTIATE=0x72; SMB_COM_SESSION_SETUP_ANDX=0x73; SMB_COM_LOGOFF_ANDX=0x74; SMB_COM_TREE_CONNECT_ANDX=0x75; SMB_COM_NT_TRANSACT=0xa0; SMB_COM_NT_TRANSACT_SECONDARY=0xa1; SMB_COM_NT_CREATE_ANDX=0xa2; SMB_COM_NT_CANCEL=0xa4; SMB_COM_NT_RENAME=0xa5;
 MAX_CIFS_HDR_SIZE=0x54; CIFS_SMALL_PATH=120; CIFS_MAX_MSGSIZE=4*4096; CIFS_ENCPWD_SIZE=16; CIFS_CRYPTO_KEY_SIZE=8; CIFS_AUTH_RESP_SIZE=24; CIFS_SESS_KEY_SIZE=16; CIFS_SERVER_CHALLENGE_SIZE=8; CIFS_HMAC_MD5_HASH_SIZE=16; CIFS_CPHTXT_SIZE=16; CIFS_NTHASH_SIZE=16; CIFS_UNLEN=20; CIFS_NO_HANDLE=0xffff; ASCII_NULL=0;
 SMBOPEN_WRITE_THROUGH=0x4000; SMBOPEN_DENY_ALL=0x10; SMBOPEN_DENY_WRITE=0x20; SMBOPEN_DENY_READ=0x30; SMBOPEN_DENY_NONE=0x40; SMBOPEN_READ=0; SMBOPEN_WRITE=1; SMBOPEN_READWRITE=2; SMBOPEN_EXECUTE=3; SMBOPEN_OCREATE=0x10; SMBOPEN_OTRUNC=2; SMBOPEN_OAPPEND=1;
 SMBFLG_EXTD_LOCK=1; SMBFLG_RCV_POSTED=2; SMBFLG_RSVD=4; SMBFLG_CASELESS=8; SMBFLG_CANONICAL_PATH_FORMAT=0x10; SMBFLG_OLD_OPLOCK=0x20; SMBFLG_OLD_OPLOCK_NOTIFY=0x40; SMBFLG_RESPONSE=0x80;
 SMBFLG2_KNOWS_LONG_NAMES=1; SMBFLG2_KNOWS_EAS=2; SMBFLG2_SECURITY_SIGNATURE=4; SMBFLG2_COMPRESSED=8; SMBFLG2_SECURITY_SIGNATURE_REQUIRED=0x10; SMBFLG2_IS_LONG_NAME=0x40; SMBFLG2_REPARSE_PATH=0x400; SMBFLG2_EXT_SEC=0x800; SMBFLG2_DFS=0x1000; SMBFLG2_PAGING_IO=0x2000; SMBFLG2_ERR_STATUS=0x4000; SMBFLG2_UNICODE=0x8000;
 FILE_NO_SHARE=0; FILE_SHARE_READ=1; FILE_SHARE_WRITE=2; FILE_SHARE_DELETE=4; FILE_SHARE_ALL=7; FILE_SUPERSEDE=0; FILE_OPEN=1; FILE_CREATE=2; FILE_OPEN_IF=3; FILE_OVERWRITE=4; FILE_OVERWRITE_IF=5;
 CREATE_NOT_FILE=1; CREATE_WRITE_THROUGH=2; CREATE_SEQUENTIAL=4; CREATE_NO_BUFFER=8; CREATE_SYNC_ALERT=0x10; CREATE_ASYNC_ALERT=0x20; CREATE_NOT_DIR=0x40; CREATE_TREE_CONNECTION=0x80; CREATE_COMPLETE_IF_OPLK=0x100; CREATE_NO_EA_KNOWLEDGE=0x200; CREATE_EIGHT_DOT_THREE=0x400; CREATE_OPEN_FOR_RECOVERY=0x400; CREATE_RANDOM_ACCESS=0x800; CREATE_DELETE_ON_CLOSE=0x1000; CREATE_OPEN_BY_ID=0x2000; CREATE_OPEN_BACKUP_INTENT=0x4000; CREATE_NO_COMPRESSION=0x8000; CREATE_RESERVE_OPFILTER=0x100000; OPEN_REPARSE_POINT=0x200000; OPEN_NO_RECALL=0x400000; OPEN_FREE_SPACE_QUERY=0x800000; CREATE_OPTIONS_MASK=0x7fffff;
 SECURITY_ANONYMOUS=0; SECURITY_IDENTIFICATION=1; SECURITY_IMPERSONATION=2; SECURITY_DELEGATION=3; SECURITY_CONTEXT_TRACKING=1; SECURITY_EFFECTIVE_ONLY=2; CIFS_DFT_PID=0x1234; CIFS_COPY_OP=1; CIFS_RENAME_OP=2; CNLEN=15; MAXCOMMENTLEN=40; MAX_PATHCONF=256; MIN_TZ_ADJ=15*60; READ_RAW_ENABLE=1; WRITE_RAW_ENABLE=2; RAW_ENABLE=3; SMB1_CLIENT_GUID_SIZE=16;
}

#[repr(C, packed)] pub struct smb_negotiate_rsp { pub hdr: smb_hdr, pub DialectIndex: __le16, pub SecurityMode: __u8, pub MaxMpxCount: __le16, pub MaxNumberVcs: __le16, pub MaxBufferSize: __le32, pub MaxRawSize: __le32, pub SessionKey: __le32, pub Capabilities: __le32, pub SystemTimeLow: __le32, pub SystemTimeHigh: __le32, pub ServerTimeZone: __le16, pub EncryptionKeyLength: __u8, pub ByteCount: __u16, pub EncryptionKey: [__u8;0] }
pub type SMB_NEGOTIATE_RSP = smb_negotiate_rsp;
#[repr(C, packed)] pub struct ntlmssp2_name { pub r#type: __le16, pub length: __le16, pub data: [__u8;0] }
#[repr(C, packed)] pub struct ntlmv2_resp { pub ntlmv2_hash: [__u8;16], pub blob_signature: __le32, pub reserved: __u32, pub time: __le64, pub client_chal: __u64, pub reserved2: __u32 }
#[repr(C, packed)] pub struct smb_com_echo_req { pub hdr:smb_hdr, pub EchoCount:__le16, pub ByteCount:__le16, pub Data:[u8;0] }
pub type ECHO_REQ=smb_com_echo_req;
#[repr(C, packed)] pub struct smb_com_echo_rsp { pub hdr:smb_hdr, pub SequenceNumber:__le16, pub ByteCount:__le16, pub Data:[u8;0] }
pub type ECHO_RSP=smb_com_echo_rsp;
#[repr(C, packed)] pub struct trans2_req { pub TotalParameterCount:__le16, pub TotalDataCount:__le16, pub MaxParameterCount:__le16, pub MaxDataCount:__le16, pub MaxSetupCount:u8, pub Reserved:u8, pub Flags:__le16, pub Timeout:__le32, pub Reserved2:u16, pub ParameterCount:__le16, pub ParameterOffset:__le16, pub DataCount:__le16, pub DataOffset:__le16, pub SetupCount:u8, pub Reserved3:u8, pub SubCommand:__le16, pub ByteCount:__le16 }
#[repr(C, packed)] pub struct trans2_resp { pub TotalParameterCount:__le16, pub TotalDataCount:__le16, pub Reserved:u16, pub ParameterCount:__le16, pub ParameterOffset:__le16, pub ParameterDisplacement:__le16, pub DataCount:__le16, pub DataOffset:__le16, pub DataDisplacement:__le16, pub SetupCount:u8, pub Reserved1:u8 }
#[repr(C, packed)] pub struct smb_t2_req { pub hdr:smb_hdr, pub t2_req:trans2_req }
#[repr(C, packed)] pub struct smb_t2_rsp { pub hdr:smb_hdr, pub t2_rsp:trans2_resp }
#[repr(C, packed)] pub struct cifs_posix_lock { pub lock_type:__le16, pub lock_flags:__le16, pub pid:__le32, pub start:__le64, pub length:__le64 }
#[repr(C, packed)] pub struct cifs_quota_data { pub rsrvd1:__u32, pub sid_size:__u32, pub rsrvd2:__u64, pub space_used:__u64, pub soft_limit:__u64, pub hard_limit:__u64, pub sid:[u8;0] }
#[repr(C, packed)] pub struct data_blob { pub data:*mut __u8, pub length:usize, pub free:Option<unsafe extern "C" fn(*mut data_blob)> }
#[repr(C, packed)] pub struct xsymlink { pub signature:[u8;4], pub cr0:i8, pub length:[u8;4], pub cr1:i8, pub md5:[u8;32], pub cr2:i8, pub path:[u8;1024] }
/* Remaining packet declarations use the common SMB header and preserve C flexible-array tails. */
#[repr(C, packed)] pub struct smb_hdr { pub protocol:[u8;4], pub command:u8, pub status:__le32, pub flags:u8, pub flags2:__le16, pub pid_high:__le16, pub signature:[u8;8], pub reserved:__le16, pub tid:__le16, pub pid:__le16, pub uid:__le16, pub mid:__le16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
