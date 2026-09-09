/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright © 2016 Intel Corporation */

// Dependency supplied by the surrounding kernel translation: __be16, __be32, __be64.

pub const TCG_SECP_00: i32 = 0;
pub const TCG_SECP_01: i32 = 1;
pub const TCG_SECP_02: i32 = 2;

pub const OPAL_DTA_TOKENID_BYTESTRING: u32 = 0xe0;
pub const OPAL_DTA_TOKENID_SINT: u32 = 0xe1;
pub const OPAL_DTA_TOKENID_UINT: u32 = 0xe2;
pub const OPAL_DTA_TOKENID_TOKEN: u32 = 0xe3;
pub const OPAL_DTA_TOKENID_INVALID: u32 = 0x0;

pub const DTAERROR_NO_METHOD_STATUS: u32 = 0x89;
pub const GENERIC_HOST_SESSION_NUM: u32 = 0x41;
pub const FIRST_TPER_SESSION_NUM: u32 = 4096;
pub const TPER_SYNC_SUPPORTED: u32 = 0x01;
pub const LOCKING_SUPPORTED_MASK: u32 = 0x01;
pub const LOCKING_ENABLED_MASK: u32 = 0x02;
pub const LOCKED_MASK: u32 = 0x04;
pub const MBR_ENABLED_MASK: u32 = 0x10;
pub const MBR_DONE_MASK: u32 = 0x20;
pub const TINY_ATOM_DATA_MASK: u32 = 0x3f;
pub const TINY_ATOM_SIGNED: u32 = 0x40;
pub const SHORT_ATOM_ID: u32 = 0x80;
pub const SHORT_ATOM_BYTESTRING: u32 = 0x20;
pub const SHORT_ATOM_SIGNED: u32 = 0x10;
pub const SHORT_ATOM_LEN_MASK: u32 = 0xf;
pub const MEDIUM_ATOM_ID: u32 = 0xc0;
pub const MEDIUM_ATOM_BYTESTRING: u32 = 0x10;
pub const MEDIUM_ATOM_SIGNED: u32 = 0x8;
pub const MEDIUM_ATOM_LEN_MASK: u32 = 0x7;
pub const LONG_ATOM_ID: u32 = 0xe0;
pub const LONG_ATOM_BYTESTRING: u32 = 0x2;
pub const LONG_ATOM_SIGNED: u32 = 0x1;
pub const TINY_ATOM_BYTE: u32 = 0x7f;
pub const SHORT_ATOM_BYTE: u32 = 0xbf;
pub const MEDIUM_ATOM_BYTE: u32 = 0xdf;
pub const LONG_ATOM_BYTE: u32 = 0xe3;
pub const EMPTY_ATOM_BYTE: u32 = 0xff;
pub const OPAL_INVAL_PARAM: u32 = 12;
pub const OPAL_MANUFACTURED_INACTIVE: u32 = 0x08;
pub const OPAL_DISCOVERY_COMID: u32 = 0x0001;
pub const LOCKING_RANGE_NON_GLOBAL: u32 = 0x03;
pub const OPAL_METHOD_LENGTH: u32 = 8;
pub const OPAL_MSID_KEYLEN: u32 = 15;
pub const OPAL_UID_LENGTH_HALF: u32 = 4;
pub const OPAL_BOOLEAN_AND: u32 = 0;
pub const OPAL_BOOLEAN_OR: u32 = 1;
pub const OPAL_BOOLEAN_NOT: u32 = 2;

#[repr(i32)]
pub enum opal_uid {
    OPAL_SMUID_UID, OPAL_THISSP_UID, OPAL_ADMINSP_UID, OPAL_LOCKINGSP_UID,
    OPAL_ENTERPRISE_LOCKINGSP_UID, OPAL_ANYBODY_UID, OPAL_SID_UID, OPAL_ADMIN1_UID,
    OPAL_USER1_UID, OPAL_USER2_UID, OPAL_PSID_UID, OPAL_ENTERPRISE_BANDMASTER0_UID,
    OPAL_ENTERPRISE_ERASEMASTER_UID, OPAL_TABLE_TABLE, OPAL_LOCKINGRANGE_GLOBAL,
    OPAL_LOCKINGRANGE_ACE_START_TO_KEY, OPAL_LOCKINGRANGE_ACE_RDLOCKED,
    OPAL_LOCKINGRANGE_ACE_WRLOCKED, OPAL_MBRCONTROL, OPAL_MBR, OPAL_AUTHORITY_TABLE,
    OPAL_C_PIN_TABLE, OPAL_LOCKING_INFO_TABLE, OPAL_ENTERPRISE_LOCKING_INFO_TABLE,
    OPAL_DATASTORE, OPAL_LOCKING_TABLE, OPAL_C_PIN_MSID, OPAL_C_PIN_SID,
    OPAL_C_PIN_ADMIN1, OPAL_HALF_UID_AUTHORITY_OBJ_REF, OPAL_HALF_UID_BOOLEAN_ACE,
    OPAL_UID_HEXFF,
}

#[repr(i32)]
pub enum opal_method {
    OPAL_PROPERTIES, OPAL_STARTSESSION, OPAL_REVERT, OPAL_ACTIVATE, OPAL_EGET,
    OPAL_ESET, OPAL_NEXT, OPAL_EAUTHENTICATE, OPAL_GETACL, OPAL_GENKEY,
    OPAL_REVERTSP, OPAL_GET, OPAL_SET, OPAL_AUTHENTICATE, OPAL_RANDOM, OPAL_ERASE,
    OPAL_REACTIVATE,
}

// C enum values are constants; duplicate values require constants rather than a Rust enum.
pub const OPAL_TRUE: u32 = 0x01; pub const OPAL_FALSE: u32 = 0x00;
pub const OPAL_BOOLEAN_EXPR: u32 = 0x03; pub const OPAL_TABLE: u32 = 0x00;
pub const OPAL_STARTROW: u32 = 0x01; pub const OPAL_ENDROW: u32 = 0x02;
pub const OPAL_STARTCOLUMN: u32 = 0x03; pub const OPAL_ENDCOLUMN: u32 = 0x04;
pub const OPAL_VALUES: u32 = 0x01; pub const OPAL_TABLE_UID: u32 = 0x00;
pub const OPAL_TABLE_NAME: u32 = 0x01; pub const OPAL_TABLE_COMMON: u32 = 0x02;
pub const OPAL_TABLE_TEMPLATE: u32 = 0x03; pub const OPAL_TABLE_KIND: u32 = 0x04;
pub const OPAL_TABLE_COLUMN: u32 = 0x05; pub const OPAL_TABLE_COLUMNS: u32 = 0x06;
pub const OPAL_TABLE_ROWS: u32 = 0x07; pub const OPAL_TABLE_ROWS_FREE: u32 = 0x08;
pub const OPAL_TABLE_ROW_BYTES: u32 = 0x09; pub const OPAL_TABLE_LASTID: u32 = 0x0a;
pub const OPAL_TABLE_MIN: u32 = 0x0b; pub const OPAL_TABLE_MAX: u32 = 0x0c;
pub const OPAL_PIN: u32 = 0x03; pub const OPAL_RANGESTART: u32 = 0x03;
pub const OPAL_RANGELENGTH: u32 = 0x04; pub const OPAL_READLOCKENABLED: u32 = 0x05;
pub const OPAL_WRITELOCKENABLED: u32 = 0x06; pub const OPAL_READLOCKED: u32 = 0x07;
pub const OPAL_WRITELOCKED: u32 = 0x08; pub const OPAL_ACTIVEKEY: u32 = 0x0a;
pub const OPAL_LIFECYCLE: u32 = 0x06; pub const OPAL_MAXRANGES: u32 = 0x04;
pub const OPAL_MBRENABLE: u32 = 0x01; pub const OPAL_MBRDONE: u32 = 0x02;
pub const OPAL_HOSTPROPERTIES: u32 = 0x00; pub const OPAL_STARTLIST: u32 = 0xf0;
pub const OPAL_ENDLIST: u32 = 0xf1; pub const OPAL_STARTNAME: u32 = 0xf2;
pub const OPAL_ENDNAME: u32 = 0xf3; pub const OPAL_CALL: u32 = 0xf8;
pub const OPAL_ENDOFDATA: u32 = 0xf9; pub const OPAL_ENDOFSESSION: u32 = 0xfa;
pub const OPAL_STARTTRANSACTON: u32 = 0xfb; pub const OPAL_ENDTRANSACTON: u32 = 0xfc;
pub const OPAL_EMPTYATOM: u32 = 0xff; pub const OPAL_WHERE: u32 = 0x00;

#[repr(i32)] pub enum opal_lockingstate { OPAL_LOCKING_READWRITE = 1, OPAL_LOCKING_READONLY, OPAL_LOCKING_LOCKED }
pub const OPAL_SUM_SET_LIST: u32 = 0x060000;
pub const OPAL_SUM_RANGE_POLICY: u32 = 0x060001;
pub const OPAL_SUM_ADMIN1_PIN: u32 = 0x060002;
pub const OPAL_KEEP_GLOBAL_RANGE_KEY: u32 = 0x060000;

#[repr(C)] pub struct opal_compacket { pub reserved0: __be32, pub extendedComID: [u8; 4], pub outstandingData: __be32, pub minTransfer: __be32, pub length: __be32 }
#[repr(C)] pub struct opal_packet { pub tsn: __be32, pub hsn: __be32, pub seq_number: __be32, pub reserved0: __be16, pub ack_type: __be16, pub acknowledgment: __be32, pub length: __be32 }
#[repr(C)] pub struct opal_data_subpacket { pub reserved0: [u8; 6], pub kind: __be16, pub length: __be32 }
#[repr(C)] pub struct opal_header { pub cp: opal_compacket, pub pkt: opal_packet, pub subpkt: opal_data_subpacket }

pub const OPAL_STACK_RESET: u32 = 0x0002;
#[repr(C)] pub struct opal_stack_reset { pub extendedComID: [u8; 4], pub request_code: __be32 }
#[repr(C)] pub struct opal_stack_reset_response { pub extendedComID: [u8; 4], pub request_code: __be32, pub reserved0: [u8; 2], pub data_length: __be16, pub response: __be32 }

pub const FC_TPER: u32 = 0x0001; pub const FC_LOCKING: u32 = 0x0002; pub const FC_GEOMETRY: u32 = 0x0003;
pub const FC_ENTERPRISE: u32 = 0x0100; pub const FC_DATASTORE: u32 = 0x0202; pub const FC_SINGLEUSER: u32 = 0x0201;
pub const FC_OPALV100: u32 = 0x0200; pub const FC_OPALV200: u32 = 0x0203;

#[repr(C)] pub struct d0_header { pub length: __be32, pub revision: __be32, pub reserved01: __be32, pub reserved02: __be32, pub ignored: [u8; 32] }
#[repr(C)] pub struct d0_tper_features { pub supported_features: u8, pub reserved01: [u8; 3], pub reserved02: __be32, pub reserved03: __be32 }
#[repr(C)] pub struct d0_locking_features { pub supported_features: u8, pub reserved01: [u8; 3], pub reserved02: __be32, pub reserved03: __be32 }
#[repr(C)] pub struct d0_geometry_features { pub header: [u8; 4], pub reserved01: u8, pub reserved02: [u8; 7], pub logical_block_size: __be32, pub alignment_granularity: __be64, pub lowest_aligned_lba: __be64 }
#[repr(C)] pub struct d0_enterprise_ssc { pub baseComID: __be16, pub numComIDs: __be16, pub range_crossing: u8, pub reserved01: u8, pub reserved02: __be16, pub reserved03: __be32, pub reserved04: __be32 }
#[repr(C)] pub struct d0_opal_v100 { pub baseComID: __be16, pub numComIDs: __be16 }
#[repr(C)] pub struct d0_single_user_mode { pub num_locking_objects: __be32, pub reserved01: u8, pub reserved02: u8, pub reserved03: __be16, pub reserved04: __be32 }
#[repr(C)] pub struct d0_datastore_table { pub reserved01: __be16, pub max_tables: __be16, pub max_size_tables: __be32, pub table_size_alignment: __be32 }
#[repr(C)] pub struct d0_opal_v200 { pub baseComID: __be16, pub numComIDs: __be16, pub range_crossing: u8, pub num_locking_admin_auth: [u8; 2], pub num_locking_user_auth: [u8; 2], pub initialPIN: u8, pub revertedPIN: u8, pub reserved01: u8, pub reserved02: __be32 }
#[repr(C)] pub struct d0_features { pub code: __be16, pub r_version: u8, pub length: u8, pub features: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
