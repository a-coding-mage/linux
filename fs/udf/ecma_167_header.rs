/* Rust translation of ecma_167.h. */

pub type dchars = u8;
pub type dstring = u8;
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;

#[repr(C, packed)] pub struct charspec { pub charSetType:u8, pub charSetInfo:[u8;63] }
pub const CHARSPEC_TYPE_CS0:u8=0x00; pub const CHARSPEC_TYPE_CS1:u8=0x01; pub const CHARSPEC_TYPE_CS2:u8=0x02; pub const CHARSPEC_TYPE_CS3:u8=0x03; pub const CHARSPEC_TYPE_CS4:u8=0x04; pub const CHARSPEC_TYPE_CS5:u8=0x05; pub const CHARSPEC_TYPE_CS6:u8=0x06; pub const CHARSPEC_TYPE_CS7:u8=0x07; pub const CHARSPEC_TYPE_CS8:u8=0x08;
#[repr(C, packed)] pub struct timestamp { pub typeAndTimezone:__le16,pub year:__le16,pub month:u8,pub day:u8,pub hour:u8,pub minute:u8,pub second:u8,pub centiseconds:u8,pub hundredsOfMicroseconds:u8,pub microseconds:u8 }
pub const TIMESTAMP_TYPE_MASK:u16=0xF000; pub const TIMESTAMP_TYPE_CUT:u16=0; pub const TIMESTAMP_TYPE_LOCAL:u16=0x1000; pub const TIMESTAMP_TYPE_AGREEMENT:u16=0x2000; pub const TIMESTAMP_TIMEZONE_MASK:u16=0x0FFF;
#[repr(C, packed)] pub struct regid { pub flags:u8,pub ident:[u8;23],pub identSuffix:[u8;8] }
pub const ENTITYID_FLAGS_DIRTY:u8=1; pub const ENTITYID_FLAGS_PROTECTED:u8=2;
pub const VSD_STD_ID_LEN:usize=5;
#[repr(C, packed)] pub struct volStructDesc { pub structType:u8,pub stdIdent:[u8;5],pub structVersion:u8,pub structData:[u8;2041] }
pub const VSD_STD_ID_NSR02:&[u8;5]=b"NSR02"; pub const VSD_STD_ID_BEA01:&[u8;5]=b"BEA01"; pub const VSD_STD_ID_BOOT2:&[u8;5]=b"BOOT2"; pub const VSD_STD_ID_CD001:&[u8;5]=b"CD001"; pub const VSD_STD_ID_CDW02:&[u8;5]=b"CDW02"; pub const VSD_STD_ID_NSR03:&[u8;5]=b"NSR03"; pub const VSD_STD_ID_TEA01:&[u8;5]=b"TEA01";
#[repr(C, packed)] pub struct beginningExtendedAreaDesc { pub structType:u8,pub stdIdent:[u8;5],pub structVersion:u8,pub structData:[u8;2041] }
#[repr(C, packed)] pub struct terminatingExtendedAreaDesc { pub structType:u8,pub stdIdent:[u8;5],pub structVersion:u8,pub structData:[u8;2041] }
#[repr(C, packed)] pub struct bootDesc { pub structType:u8,pub stdIdent:[u8;5],pub structVersion:u8,pub reserved1:u8,pub archType:regid,pub bootIdent:regid,pub bootExtLocation:__le32,pub bootExtLength:__le32,pub loadAddress:__le64,pub startAddress:__le64,pub descCreationDateAndTime:timestamp,pub flags:__le16,pub reserved2:[u8;32],pub bootUse:[u8;1906] }
pub const BOOT_FLAGS_ERASE:u16=1;
#[repr(C, packed)] pub struct extent_ad { pub extLength:__le32,pub extLocation:__le32 }
#[repr(C)] pub struct kernel_extent_ad { pub extLength:u32,pub extLocation:u32 }
#[repr(C, packed)] pub struct tag { pub tagIdent:__le16,pub descVersion:__le16,pub tagChecksum:u8,pub reserved:u8,pub tagSerialNum:__le16,pub descCRC:__le16,pub descCRCLength:__le16,pub tagLocation:__le32 }
pub const TAG_IDENT_PVD:u16=1; pub const TAG_IDENT_AVDP:u16=2; pub const TAG_IDENT_VDP:u16=3; pub const TAG_IDENT_IUVD:u16=4; pub const TAG_IDENT_PD:u16=5; pub const TAG_IDENT_LVD:u16=6; pub const TAG_IDENT_USD:u16=7; pub const TAG_IDENT_TD:u16=8; pub const TAG_IDENT_LVID:u16=9;
#[repr(C, packed)] pub struct NSRDesc { pub structType:u8,pub stdIdent:[u8;5],pub structVersion:u8,pub reserved:u8,pub structData:[u8;2040] }
#[repr(C, packed)] pub struct genericDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub reserved:[u8;492] }
#[repr(C, packed)] pub struct primaryVolDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub primaryVolDescNum:__le32,pub volIdent:[dstring;32],pub volSeqNum:__le16,pub maxVolSeqNum:__le16,pub interchangeLvl:__le16,pub maxInterchangeLvl:__le16,pub charSetList:__le32,pub maxCharSetList:__le32,pub volSetIdent:[dstring;128],pub descCharSet:charspec,pub explanatoryCharSet:charspec,pub volAbstract:extent_ad,pub volCopyright:extent_ad,pub appIdent:regid,pub recordingDateAndTime:timestamp,pub impIdent:regid,pub impUse:[u8;64],pub predecessorVolDescSeqLocation:__le32,pub flags:__le16,pub reserved:[u8;22] }
pub const PVD_FLAGS_VSID_COMMON:u16=1;
#[repr(C, packed)] pub struct anchorVolDescPtr { pub descTag:tag,pub mainVolDescSeqExt:extent_ad,pub reserveVolDescSeqExt:extent_ad,pub reserved:[u8;480] }
#[repr(C, packed)] pub struct volDescPtr { pub descTag:tag,pub volDescSeqNum:__le32,pub nextVolDescSeqExt:extent_ad,pub reserved:[u8;484] }
#[repr(C, packed)] pub struct impUseVolDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub impIdent:regid,pub impUse:[u8;460] }
#[repr(C, packed)] pub struct partitionDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub partitionFlags:__le16,pub partitionNumber:__le16,pub partitionContents:regid,pub partitionContentsUse:[u8;128],pub accessType:__le32,pub partitionStartingLocation:__le32,pub partitionLength:__le32,pub impIdent:regid,pub impUse:[u8;128],pub reserved:[u8;156] }
pub const PD_PARTITION_FLAGS_ALLOC:u16=1; pub const PD_PARTITION_CONTENTS_NSR02:&[u8;6]=b"+NSR02"; pub const PD_PARTITION_CONTENTS_FDC01:&[u8;6]=b"+FDC01"; pub const PD_PARTITION_CONTENTS_CD001:&[u8;6]=b"+CD001"; pub const PD_PARTITION_CONTENTS_CDW02:&[u8;6]=b"+CDW02"; pub const PD_PARTITION_CONTENTS_NSR03:&[u8;6]=b"+NSR03";
pub const PD_ACCESS_TYPE_NONE:u32=0; pub const PD_ACCESS_TYPE_READ_ONLY:u32=1; pub const PD_ACCESS_TYPE_WRITE_ONCE:u32=2; pub const PD_ACCESS_TYPE_REWRITABLE:u32=3; pub const PD_ACCESS_TYPE_OVERWRITABLE:u32=4;
#[repr(C, packed)] pub struct logicalVolDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub descCharSet:charspec,pub logicalVolIdent:[dstring;128],pub logicalBlockSize:__le32,pub domainIdent:regid,pub logicalVolContentsUse:[u8;16],pub mapTableLength:__le32,pub numPartitionMaps:__le32,pub impIdent:regid,pub impUse:[u8;128],pub integritySeqExt:extent_ad,pub partitionMaps:[u8;0] }
#[repr(C, packed)] pub struct genericPartitionMap { pub partitionMapType:u8,pub partitionMapLength:u8,pub partitionMapping:[u8;0] }
pub const GP_PARTITION_MAP_TYPE_UNDEF:u8=0; pub const GP_PARTITION_MAP_TYPE_1:u8=1; pub const GP_PARTITION_MAP_TYPE_2:u8=2;
#[repr(C, packed)] pub struct genericPartitionMap1 { pub partitionMapType:u8,pub partitionMapLength:u8,pub volSeqNum:__le16,pub partitionNum:__le16 }
#[repr(C, packed)] pub struct genericPartitionMap2 { pub partitionMapType:u8,pub partitionMapLength:u8,pub partitionIdent:[u8;62] }
#[repr(C, packed)] pub struct unallocSpaceDesc { pub descTag:tag,pub volDescSeqNum:__le32,pub numAllocDescs:__le32,pub allocDescs:[extent_ad;0] }
#[repr(C, packed)] pub struct terminatingDesc { pub descTag:tag,pub reserved:[u8;496] }
#[repr(C, packed)] pub struct logicalVolIntegrityDesc { pub descTag:tag,pub recordingDateAndTime:timestamp,pub integrityType:__le32,pub nextIntegrityExt:extent_ad,pub logicalVolContentsUse:[u8;32],pub numOfPartitions:__le32,pub lengthOfImpUse:__le32,pub freeSpaceTable:[__le32;0] }
pub const LVID_INTEGRITY_TYPE_OPEN:u32=0; pub const LVID_INTEGRITY_TYPE_CLOSE:u32=1;
#[repr(C, packed)] pub struct lb_addr { pub logicalBlockNum:__le32,pub partitionReferenceNum:__le16 }
#[repr(C)] pub struct kernel_lb_addr { pub logicalBlockNum:u32,pub partitionReferenceNum:u16 }
#[repr(C, packed)] pub struct short_ad { pub extLength:__le32,pub extPosition:__le32 }
#[repr(C, packed)] pub struct long_ad { pub extLength:__le32,pub extLocation:lb_addr,pub impUse:[u8;6] }
#[repr(C)] pub struct kernel_long_ad { pub extLength:u32,pub extLocation:kernel_lb_addr,pub impUse:[u8;6] }
#[repr(C, packed)] pub struct ext_ad { pub extLength:__le32,pub recordedLength:__le32,pub informationLength:__le32,pub extLocation:lb_addr }
#[repr(C)] pub struct kernel_ext_ad { pub extLength:u32,pub recordedLength:u32,pub informationLength:u32,pub extLocation:kernel_lb_addr }
#[repr(C, packed)] pub struct fileSetDesc { pub descTag:tag,pub recordingDateAndTime:timestamp,pub interchangeLvl:__le16,pub maxInterchangeLvl:__le16,pub charSetList:__le32,pub maxCharSetList:__le32,pub fileSetNum:__le32,pub fileSetDescNum:__le32,pub logicalVolIdentCharSet:charspec,pub logicalVolIdent:[dstring;128],pub fileSetCharSet:charspec,pub fileSetIdent:[dstring;32],pub copyrightFileIdent:[dstring;32],pub abstractFileIdent:[dstring;32],pub rootDirectoryICB:long_ad,pub domainIdent:regid,pub nextExt:long_ad,pub streamDirectoryICB:long_ad,pub reserved:[u8;32] }
#[repr(C, packed)] pub struct partitionHeaderDesc { pub unallocSpaceTable:short_ad,pub unallocSpaceBitmap:short_ad,pub partitionIntegrityTable:short_ad,pub freedSpaceTable:short_ad,pub freedSpaceBitmap:short_ad,pub reserved:[u8;88] }
#[repr(C, packed)] pub struct fileIdentDesc { pub descTag:tag,pub fileVersionNum:__le16,pub fileCharacteristics:u8,pub lengthFileIdent:u8,pub icb:long_ad,pub lengthOfImpUse:__le16 }
pub const FID_FILE_CHAR_HIDDEN:u8=1; pub const FID_FILE_CHAR_DIRECTORY:u8=2; pub const FID_FILE_CHAR_DELETED:u8=4; pub const FID_FILE_CHAR_PARENT:u8=8; pub const FID_FILE_CHAR_METADATA:u8=0x10;
#[repr(C, packed)] pub struct allocExtDesc { pub descTag:tag,pub previousAllocExtLocation:__le32,pub lengthAllocDescs:__le32 }
#[repr(C, packed)] pub struct icbtag { pub priorRecordedNumDirectEntries:__le32,pub strategyType:__le16,pub strategyParameter:__le16,pub numEntries:__le16,pub reserved:u8,pub fileType:u8,pub parentICBLocation:lb_addr,pub flags:__le16 }
#[repr(C, packed)] pub struct indirectEntry { pub descTag:tag,pub icbTag:icbtag,pub indirectICB:long_ad }
#[repr(C, packed)] pub struct terminalEntry { pub descTag:tag,pub icbTag:icbtag }
#[repr(C, packed)] pub struct fileEntry { pub descTag:tag,pub icbTag:icbtag,pub uid:__le32,pub gid:__le32,pub permissions:__le32,pub fileLinkCount:__le16,pub recordFormat:u8,pub recordDisplayAttr:u8,pub recordLength:__le32,pub informationLength:__le64,pub logicalBlocksRecorded:__le64,pub accessTime:timestamp,pub modificationTime:timestamp,pub attrTime:timestamp,pub checkpoint:__le32,pub extendedAttrICB:long_ad,pub impIdent:regid,pub uniqueID:__le64,pub lengthExtendedAttr:__le32,pub lengthAllocDescs:__le32,pub extendedAttr:[u8;0] }
pub const ICBTAG_STRATEGY_TYPE_UNDEF:u16=0; pub const ICBTAG_STRATEGY_TYPE_1:u16=1; pub const ICBTAG_STRATEGY_TYPE_2:u16=2; pub const ICBTAG_STRATEGY_TYPE_3:u16=3; pub const ICBTAG_STRATEGY_TYPE_4:u16=4;
pub const ICBTAG_FILE_TYPE_UNDEF:u8=0; pub const ICBTAG_FILE_TYPE_USE:u8=1; pub const ICBTAG_FILE_TYPE_PIE:u8=2; pub const ICBTAG_FILE_TYPE_IE:u8=3; pub const ICBTAG_FILE_TYPE_DIRECTORY:u8=4; pub const ICBTAG_FILE_TYPE_REGULAR:u8=5; pub const ICBTAG_FILE_TYPE_BLOCK:u8=6; pub const ICBTAG_FILE_TYPE_CHAR:u8=7; pub const ICBTAG_FILE_TYPE_EA:u8=8; pub const ICBTAG_FILE_TYPE_FIFO:u8=9; pub const ICBTAG_FILE_TYPE_SOCKET:u8=10; pub const ICBTAG_FILE_TYPE_TE:u8=11; pub const ICBTAG_FILE_TYPE_SYMLINK:u8=12; pub const ICBTAG_FILE_TYPE_STREAMDIR:u8=13;
pub const ICBTAG_FLAG_AD_MASK:u16=7; pub const ICBTAG_FLAG_AD_SHORT:u16=0; pub const ICBTAG_FLAG_AD_LONG:u16=1; pub const ICBTAG_FLAG_AD_EXTENDED:u16=2; pub const ICBTAG_FLAG_AD_IN_ICB:u16=3; pub const ICBTAG_FLAG_SORTED:u16=8; pub const ICBTAG_FLAG_NONRELOCATABLE:u16=0x10; pub const ICBTAG_FLAG_ARCHIVE:u16=0x20; pub const ICBTAG_FLAG_SETUID:u16=0x40; pub const ICBTAG_FLAG_SETGID:u16=0x80; pub const ICBTAG_FLAG_STICKY:u16=0x100; pub const ICBTAG_FLAG_CONTIGUOUS:u16=0x200; pub const ICBTAG_FLAG_SYSTEM:u16=0x400; pub const ICBTAG_FLAG_TRANSFORMED:u16=0x800; pub const ICBTAG_FLAG_MULTIVERSIONS:u16=0x1000; pub const ICBTAG_FLAG_STREAM:u16=0x2000;
pub const FE_PERM_O_EXEC:u32=1; pub const FE_PERM_O_WRITE:u32=2; pub const FE_PERM_O_READ:u32=4; pub const FE_PERM_O_CHATTR:u32=8; pub const FE_PERM_O_DELETE:u32=0x10; pub const FE_PERM_G_EXEC:u32=0x20; pub const FE_PERM_G_WRITE:u32=0x40; pub const FE_PERM_G_READ:u32=0x80; pub const FE_PERM_G_CHATTR:u32=0x100; pub const FE_PERM_G_DELETE:u32=0x200; pub const FE_PERM_U_EXEC:u32=0x400; pub const FE_PERM_U_WRITE:u32=0x800; pub const FE_PERM_U_READ:u32=0x1000; pub const FE_PERM_U_CHATTR:u32=0x2000; pub const FE_PERM_U_DELETE:u32=0x4000;
#[repr(C, packed)] pub struct extendedAttrHeaderDesc { pub descTag:tag,pub impAttrLocation:__le32,pub appAttrLocation:__le32 }
#[repr(C, packed)] pub struct genericFormat { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub attrData:[u8;0] }
#[repr(C, packed)] pub struct charSetInfo { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub escapeSeqLength:__le32,pub charSetType:u8,pub escapeSeq:[u8;0] }
#[repr(C, packed)] pub struct altPerms { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub ownerIdent:__le16,pub groupIdent:__le16,pub permission:__le16 }
#[repr(C, packed)] pub struct fileTimesExtAttr { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub dataLength:__le32,pub fileTimeExistence:__le32,pub fileTimes:u8 }
#[repr(C, packed)] pub struct infoTimesExtAttr { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub dataLength:__le32,pub infoTimeExistence:__le32,pub infoTimes:[u8;0] }
#[repr(C, packed)] pub struct deviceSpec { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub impUseLength:__le32,pub majorDeviceIdent:__le32,pub minorDeviceIdent:__le32,pub impUse:[u8;0] }
#[repr(C, packed)] pub struct impUseExtAttr { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub impUseLength:__le32,pub impIdent:regid,pub impUse:[u8;0] }
#[repr(C, packed)] pub struct appUseExtAttr { pub attrType:__le32,pub attrSubtype:u8,pub reserved:[u8;3],pub attrLength:__le32,pub appUseLength:__le32,pub appIdent:regid,pub appUse:[u8;0] }
pub const EXTATTR_CHAR_SET:u32=1; pub const EXTATTR_ALT_PERMS:u32=3; pub const EXTATTR_FILE_TIMES:u32=5; pub const EXTATTR_INFO_TIMES:u32=6; pub const EXTATTR_DEV_SPEC:u32=12; pub const EXTATTR_IMP_USE:u32=2048; pub const EXTATTR_APP_USE:u32=65536; pub const EXTATTR_SUBTYPE:u8=1;
#[repr(C, packed)] pub struct unallocSpaceEntry { pub descTag:tag,pub icbTag:icbtag,pub lengthAllocDescs:__le32,pub allocDescs:[u8;0] }
#[repr(C, packed)] pub struct spaceBitmapDesc { pub descTag:tag,pub numOfBits:__le32,pub numOfBytes:__le32,pub bitmap:[u8;0] }
#[repr(C, packed)] pub struct partitionIntegrityEntry { pub descTag:tag,pub icbTag:icbtag,pub recordingDateAndTime:timestamp,pub integrityType:u8,pub reserved:[u8;175],pub impIdent:regid,pub impUse:[u8;256] }
pub const EXT_LENGTH_MASK:u32=0x3fffffff; pub const EXT_TYPE_MASK:u32=0xc0000000; pub const EXT_RECORDED_ALLOCATED:u32=0; pub const EXT_NOT_RECORDED_ALLOCATED:u32=0x40000000; pub const EXT_NOT_RECORDED_NOT_ALLOCATED:u32=0x80000000; pub const EXT_NEXT_EXTENT_ALLOCDESCS:u32=0xc0000000;
#[repr(C, packed)] pub struct logicalVolHeaderDesc { pub uniqueID:__le64,pub reserved:[u8;24] }
#[repr(C, packed)] pub struct pathComponent { pub componentType:u8,pub lengthComponentIdent:u8,pub componentFileVersionNum:__le16,pub componentIdent:[dchars;0] }
#[repr(C, packed)] pub struct extendedFileEntry { pub descTag:tag,pub icbTag:icbtag,pub uid:__le32,pub gid:__le32,pub permissions:__le32,pub fileLinkCount:__le16,pub recordFormat:u8,pub recordDisplayAttr:u8,pub recordLength:__le32,pub informationLength:__le64,pub objectSize:__le64,pub logicalBlocksRecorded:__le64,pub accessTime:timestamp,pub modificationTime:timestamp,pub createTime:timestamp,pub attrTime:timestamp,pub checkpoint:__le32,pub reserved:u32,pub extendedAttrICB:long_ad,pub streamDirectoryICB:long_ad,pub impIdent:regid,pub uniqueID:__le64,pub lengthExtendedAttr:__le32,pub lengthAllocDescs:__le32,pub extendedAttr:[u8;0] }
pub const FE_RECORD_FMT_UNDEF:u8=0; pub const FE_RECORD_FMT_FIXED_PAD:u8=1; pub const FE_RECORD_FMT_FIXED:u8=2; pub const FE_RECORD_FMT_VARIABLE8:u8=3; pub const FE_RECORD_FMT_VARIABLE16:u8=4; pub const FE_RECORD_FMT_VARIABLE16_MSB:u8=5; pub const FE_RECORD_FMT_VARIABLE32:u8=6; pub const FE_RECORD_FMT_PRINT:u8=7; pub const FE_RECORD_FMT_LF:u8=8; pub const FE_RECORD_FMT_CR:u8=9; pub const FE_RECORD_FMT_CRLF:u8=10; pub const FE_RECORD_FMT_LFCR:u8=11;
pub const FE_RECORD_DISPLAY_ATTR_UNDEF:u8=0; pub const FE_RECORD_DISPLAY_ATTR_1:u8=1; pub const FE_RECORD_DISPLAY_ATTR_2:u8=2; pub const FE_RECORD_DISPLAY_ATTR_3:u8=3;
pub const FTE_CREATION:u32=1; pub const FTE_DELETION:u32=4; pub const FTE_EFFECTIVE:u32=8; pub const FTE_BACKUP:u32=2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
