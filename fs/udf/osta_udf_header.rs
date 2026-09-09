/*
 * osta_udf.h
 *
 * This file is based on OSTA UDF(tm) 2.60 (March 1, 2005)
 * http://www.osta.org
 *
 * Copyright (c) 2001-2004  Ben Fennema
 * Copyright (c) 2017-2019  Pali Rohár <pali@kernel.org>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU Public License ("GPL").
 */

/* Dependency types are supplied by the translated ECMA-167 header. */

pub const UDF_CHAR_SET_TYPE: u8 = 0;
pub const UDF_CHAR_SET_INFO: &str = "OSTA Compressed Unicode";

pub const UDF_ID_DEVELOPER: &str = "*Linux UDFFS";
pub const UDF_ID_COMPLIANT: &str = "*OSTA UDF Compliant";
pub const UDF_ID_LV_INFO: &str = "*UDF LV Info";
pub const UDF_ID_FREE_EA: &str = "*UDF FreeEASpace";
pub const UDF_ID_FREE_APP_EA: &str = "*UDF FreeAppEASpace";
pub const UDF_ID_DVD_CGMS: &str = "*UDF DVD CGMS Info";
pub const UDF_ID_VAT_LVEXTENSION: &str = "*UDF VAT LVExtension";
pub const UDF_ID_OS2_EA: &str = "*UDF OS/2 EA";
pub const UDF_ID_OS2_EA_LENGTH: &str = "*UDF OS/2 EALength";
pub const UDF_ID_MAC_VOLUME: &str = "*UDF Mac VolumeInfo";
pub const UDF_ID_MAC_FINDER: &str = "*UDF Mac FinderInfo";
pub const UDF_ID_MAC_UNIQUE: &str = "*UDF Mac UniqueIDTable";
pub const UDF_ID_MAC_RESOURCE: &str = "*UDF Mac ResourceFork";
pub const UDF_ID_OS400_DIRINFO: &str = "*UDF OS/400 DirInfo";
pub const UDF_ID_VIRTUAL: &str = "*UDF Virtual Partition";
pub const UDF_ID_SPARABLE: &str = "*UDF Sparable Partition";
pub const UDF_ID_ALLOC: &str = "*UDF Virtual Alloc Tbl";
pub const UDF_ID_SPARING: &str = "*UDF Sparing Table";
pub const UDF_ID_METADATA: &str = "*UDF Metadata Partition";

pub const DOMAIN_FLAGS_HARD_WRITE_PROTECT: u8 = 0x01;
pub const DOMAIN_FLAGS_SOFT_WRITE_PROTECT: u8 = 0x02;

#[repr(C, packed)]
pub struct domainIdentSuffix { pub UDFRevision: __le16, pub domainFlags: u8, pub reserved: [u8; 5] }
#[repr(C, packed)]
pub struct UDFIdentSuffix { pub UDFRevision: __le16, pub OSClass: u8, pub OSIdentifier: u8, pub reserved: [u8; 4] }
#[repr(C, packed)]
pub struct impIdentSuffix { pub OSClass: u8, pub OSIdentifier: u8, pub impUse: [u8; 6] }
#[repr(C, packed)]
pub struct appIdentSuffix { pub impUse: [u8; 8] }

#[repr(C, packed)]
pub struct logicalVolIntegrityDescImpUse { pub impIdent: regid, pub numFiles: __le32, pub numDirs: __le32, pub minUDFReadRev: __le16, pub minUDFWriteRev: __le16, pub maxUDFWriteRev: __le16, pub impUse: [u8; 0] }
#[repr(C, packed)]
pub struct impUseVolDescImpUse { pub LVICharset: charspec, pub logicalVolIdent: [dstring; 128], pub LVInfo1: [dstring; 36], pub LVInfo2: [dstring; 36], pub LVInfo3: [dstring; 36], pub impIdent: regid, pub impUse: [u8; 128] }
#[repr(C, packed)]
pub struct udfPartitionMap2 { pub partitionMapType: u8, pub partitionMapLength: u8, pub reserved1: [u8; 2], pub partIdent: regid, pub volSeqNum: __le16, pub partitionNum: __le16 }
#[repr(C, packed)]
pub struct virtualPartitionMap { pub partitionMapType: u8, pub partitionMapLength: u8, pub reserved1: [u8; 2], pub partIdent: regid, pub volSeqNum: __le16, pub partitionNum: __le16, pub reserved2: [u8; 24] }
#[repr(C, packed)]
pub struct sparablePartitionMap { pub partitionMapType: u8, pub partitionMapLength: u8, pub reserved1: [u8; 2], pub partIdent: regid, pub volSeqNum: __le16, pub partitionNum: __le16, pub packetLength: __le16, pub numSparingTables: u8, pub reserved2: [u8; 1], pub sizeSparingTable: __le32, pub locSparingTable: [__le32; 4] }
#[repr(C, packed)]
pub struct metadataPartitionMap { pub partitionMapType: u8, pub partitionMapLength: u8, pub reserved1: [u8; 2], pub partIdent: regid, pub volSeqNum: __le16, pub partitionNum: __le16, pub metadataFileLoc: __le32, pub metadataMirrorFileLoc: __le32, pub metadataBitmapFileLoc: __le32, pub allocUnitSize: __le32, pub alignUnitSize: __le16, pub flags: u8, pub reserved2: [u8; 5] }
#[repr(C, packed)]
pub struct virtualAllocationTable20 { pub lengthHeader: __le16, pub lengthImpUse: __le16, pub logicalVolIdent: [dstring; 128], pub previousVATICBLoc: __le32, pub numFiles: __le32, pub numDirs: __le32, pub minUDFReadRev: __le16, pub minUDFWriteRev: __le16, pub maxUDFWriteRev: __le16, pub reserved: __le16, pub impUse: [u8; 0] }

pub const ICBTAG_FILE_TYPE_VAT20: u32 = 0xF8;

#[repr(C, packed)]
pub struct sparingEntry { pub origLocation: __le32, pub mappedLocation: __le32 }
#[repr(C, packed)]
pub struct sparingTable { pub descTag: tag, pub sparingIdent: regid, pub reallocationTableLen: __le16, pub reserved: __le16, pub sequenceNum: __le32, pub mapEntry: [sparingEntry; 0] }

pub const ICBTAG_FILE_TYPE_MAIN: u32 = 0xFA;
pub const ICBTAG_FILE_TYPE_MIRROR: u32 = 0xFB;
pub const ICBTAG_FILE_TYPE_BITMAP: u32 = 0xFC;

#[repr(C, packed)]
pub struct allocDescImpUse { pub flags: __le16, pub impUse: [u8; 4] }
pub const AD_IU_EXT_ERASED: u16 = 0x0001;
pub const ICBTAG_FILE_TYPE_REALTIME: u32 = 0xF9;
#[repr(C, packed)]
pub struct freeEaSpace { pub headerChecksum: __le16, pub freeEASpace: [u8; 0] }
#[repr(C, packed)]
pub struct DVDCopyrightImpUse { pub headerChecksum: __le16, pub CGMSInfo: u8, pub dataType: u8, pub protectionSystemInfo: [u8; 4] }
#[repr(C, packed)]
pub struct LVExtensionEA { pub headerChecksum: __le16, pub verificationID: __le64, pub numFiles: __le32, pub numDirs: __le32, pub logicalVolIdent: [dstring; 128] }
#[repr(C, packed)]
pub struct freeAppEASpace { pub headerChecksum: __le16, pub freeEASpace: [u8; 0] }

pub const UDF_ID_UNIQUE_ID: &str = "*UDF Unique ID Mapping Data";
pub const UDF_ID_NON_ALLOC: &str = "*UDF Non-Allocatable Space";
pub const UDF_ID_POWER_CAL: &str = "*UDF Power Cal Table";
pub const UDF_ID_BACKUP: &str = "*UDF Backup";
pub const UDF_ID_MAC_RESOURCE_FORK_STREAM: &str = "*UDF Macintosh Resource Fork";
pub const UDF_ID_NT_ACL: &str = "*UDF NT ACL";
pub const UDF_ID_UNIX_ACL: &str = "*UDF UNIX ACL";

pub const UDF_OS_CLASS_UNDEF: u32 = 0x00; pub const UDF_OS_CLASS_DOS: u32 = 0x01; pub const UDF_OS_CLASS_OS2: u32 = 0x02; pub const UDF_OS_CLASS_MAC: u32 = 0x03; pub const UDF_OS_CLASS_UNIX: u32 = 0x04; pub const UDF_OS_CLASS_WIN9X: u32 = 0x05; pub const UDF_OS_CLASS_WINNT: u32 = 0x06; pub const UDF_OS_CLASS_OS400: u32 = 0x07; pub const UDF_OS_CLASS_BEOS: u32 = 0x08; pub const UDF_OS_CLASS_WINCE: u32 = 0x09;
pub const UDF_OS_ID_UNDEF: u32 = 0x00; pub const UDF_OS_ID_DOS: u32 = 0x00; pub const UDF_OS_ID_OS2: u32 = 0x00; pub const UDF_OS_ID_MAC: u32 = 0x00; pub const UDF_OS_ID_MAX_OSX: u32 = 0x01; pub const UDF_OS_ID_UNIX: u32 = 0x00; pub const UDF_OS_ID_AIX: u32 = 0x01; pub const UDF_OS_ID_SOLARIS: u32 = 0x02; pub const UDF_OS_ID_HPUX: u32 = 0x03; pub const UDF_OS_ID_IRIX: u32 = 0x04; pub const UDF_OS_ID_LINUX: u32 = 0x05; pub const UDF_OS_ID_MKLINUX: u32 = 0x06; pub const UDF_OS_ID_FREEBSD: u32 = 0x07; pub const UDF_OS_ID_NETBSD: u32 = 0x08; pub const UDF_OS_ID_WIN9X: u32 = 0x00; pub const UDF_OS_ID_WINNT: u32 = 0x00; pub const UDF_OS_ID_OS400: u32 = 0x00; pub const UDF_OS_ID_BEOS: u32 = 0x00; pub const UDF_OS_ID_WINCE: u32 = 0x00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
