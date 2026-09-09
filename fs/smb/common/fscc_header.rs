/* SPDX-License-Identifier: LGPL-2.1 */
/* Translated from fscc.h. */

#[repr(C, packed)]
pub struct reparse_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub DataBuffer: [__u8; 0] }
#[repr(C, packed)]
pub struct reparse_guid_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub ReparseGuid: [__u8; 16], pub DataBuffer: [__u8; 0] }
#[repr(C, packed)]
pub struct reparse_mount_point_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub SubstituteNameOffset: __le16, pub SubstituteNameLength: __le16, pub PrintNameOffset: __le16, pub PrintNameLength: __le16, pub PathBuffer: [__u8; 0] }
pub const SYMLINK_FLAG_RELATIVE: u32 = 0x00000001;
#[repr(C, packed)]
pub struct reparse_symlink_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub SubstituteNameOffset: __le16, pub SubstituteNameLength: __le16, pub PrintNameOffset: __le16, pub PrintNameLength: __le16, pub Flags: __le32, pub PathBuffer: [__u8; 0] }

pub const NFS_SPECFILE_LNK: u64 = 0x00000000014B4E4C;
pub const NFS_SPECFILE_CHR: u64 = 0x0000000000524843;
pub const NFS_SPECFILE_BLK: u64 = 0x00000000004B4C42;
pub const NFS_SPECFILE_FIFO: u64 = 0x000000004F464946;
pub const NFS_SPECFILE_SOCK: u64 = 0x000000004B434F53;
#[repr(C, packed)]
pub struct reparse_nfs_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub InodeType: __le64, pub DataBuffer: [__u8; 0] }
#[repr(C, packed)]
pub struct reparse_wsl_symlink_data_buffer { pub ReparseTag: __le32, pub ReparseDataLength: __le16, pub Reserved: __u16, pub Version: __le32, pub Target: [__u8; 0] }

#[repr(C, packed)]
pub struct duplicate_extents_to_file { pub PersistentFileHandle: __u64, pub VolatileFileHandle: __u64, pub SourceFileOffset: __le64, pub TargetFileOffset: __le64, pub ByteCount: __le64 }
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 0x00000001;
#[repr(C, packed)]
pub struct duplicate_extents_to_file_ex { pub StructureSize: __le64, pub PersistentFileHandle: __u64, pub VolatileFileHandle: __u64, pub SourceFileOffset: __le64, pub TargetFileOffset: __le64, pub ByteCount: __le64, pub Flags: __le32, pub Reserved: __le32 }
pub const COMPRESSION_FORMAT_NONE: u16 = 0x0000;
pub const COMPRESSION_FORMAT_DEFAULT: u16 = 0x0001;
pub const COMPRESSION_FORMAT_LZNT1: u16 = 0x0002;
#[repr(C, packed)] pub struct compress_ioctl { pub CompressionState: __le16 }
#[repr(C, packed)] pub struct fsctl_get_integrity_information_rsp { pub ChecksumAlgorithm: __le16, pub Reserved: __le16, pub Flags: __le32, pub ChecksumChunkSizeInBytes: __le32, pub ClusterSizeInBytes: __le32 }
#[repr(C, packed)] pub struct file_allocated_range_buffer { pub file_offset: __le64, pub length: __le64 }
#[repr(C, packed)] pub struct fsctl_query_file_regions_req { pub FileOffset: __le64, pub Length: __le64, pub DesiredUsage: __le32, pub Reserved: __le32 }
pub const FILE_USAGE_INVALID_RANGE: u32 = 0; pub const FILE_USAGE_VALID_CACHED_DATA: u32 = 1; pub const FILE_USAGE_NONCACHED_DATA: u32 = 2;
#[repr(C, packed)] pub struct file_region_info { pub FileOffset: __le64, pub Length: __le64, pub DesiredUsage: __le32, pub Reserved: __le32 }
#[repr(C, packed)] pub struct fsctl_query_file_region_rsp { pub Flags: __le32, pub TotalRegionEntryCount: __le32, pub RegionEntryCount: __le32, pub Reserved: __u32, pub Regions: [file_region_info; 0] }
#[repr(C, packed)] pub struct fsctl_query_on_disk_vol_info_rsp { pub DirectoryCount: __le64, pub FileCount: __le64, pub FsFormatMajVersion: __le16, pub FsFormatMinVersion: __le16, pub FsFormatName: [__u8;24], pub FormatTime: __le64, pub LastUpdateTime: __le64, pub CopyrightInfo: [__u8;68], pub AbstractInfo: [__u8;68], pub FormatImplInfo: [__u8;68], pub LastModifyImplInfo: [__u8;68] }
#[repr(C, packed)] pub struct fsctl_set_integrity_information_req { pub ChecksumAlgorithm: __le16, pub Reserved: __le16, pub Flags: __le32 }
#[repr(C, packed)] pub struct fsctl_set_integrity_info_ex_req { pub EnableIntegrity: __u8, pub KeepState: __u8, pub Reserved: __u16, pub Flags: __le32, pub Version: __u8, pub Reserved2: [__u8;7] }
#[repr(C, packed)] pub struct file_zero_data_information { pub FileOffset: __le64, pub BeyondFinalZero: __le64 }
#[repr(C, packed)] pub struct file_level_trim_range { pub Offset: __le64, pub Length: __le64 }
#[repr(C, packed)] pub struct file_level_trim { pub Key: __le32, pub NumRanges: __le32, pub Ranges: [file_level_trim_range;0] }
#[repr(C, packed)] pub struct file_level_trim_output { pub NumRangesProcessed: __le32 }

#[repr(C, packed)] pub struct smb2_file_all_info { pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub Attributes: __le32, pub Pad1: __u32, pub AllocationSize: __le64, pub EndOfFile: __le64, pub NumberOfLinks: __le32, pub DeletePending: __u8, pub Directory: __u8, pub Pad2: __u16, pub IndexNumber: __le64, pub EASize: __le32, pub AccessFlags: __le32, pub CurrentByteOffset: __le64, pub Mode: __le32, pub AlignmentRequirement: __le32, pub FileNameLength: __le32, pub __pad: u8, pub FileName: [i8;0] }
#[repr(C, packed)] pub struct file_basic_info { pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub Attributes: __le32, pub Pad: __u32 }
pub type FILE_BASIC_INFO = file_basic_info;
#[repr(C, packed)] pub struct FILE_BOTH_DIRECTORY_INFO { pub NextEntryOffset: __le32, pub FileIndex: __u32, pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub EndOfFile: __le64, pub AllocationSize: __le64, pub ExtFileAttributes: __le32, pub FileNameLength: __le32, pub EaSize: __le32, pub ShortNameLength: __u8, pub Reserved: __u8, pub ShortName: [__u8;24], pub FileName: [i8;0] }
#[repr(C, packed)] pub struct FILE_DIRECTORY_INFO { pub NextEntryOffset: __le32, pub FileIndex: __u32, pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub EndOfFile: __le64, pub AllocationSize: __le64, pub ExtFileAttributes: __le32, pub FileNameLength: __le32, pub FileName: [i8;0] }
#[repr(C, packed)] pub struct smb2_file_eof_info { pub EndOfFile: __le64 }
#[repr(C, packed)] pub struct smb2_file_alloc_info { pub AllocationSize: __le64 }
#[repr(C, packed)] pub struct FILE_FULL_DIRECTORY_INFO { pub NextEntryOffset: __le32, pub FileIndex: __u32, pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub EndOfFile: __le64, pub AllocationSize: __le64, pub ExtFileAttributes: __le32, pub FileNameLength: __le32, pub EaSize: __le32, pub FileName: [i8;0] }
#[repr(C, packed)] pub struct FILE_ID_FULL_DIR_INFO { pub NextEntryOffset: __le32, pub FileIndex: __u32, pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub EndOfFile: __le64, pub AllocationSize: __le64, pub ExtFileAttributes: __le32, pub FileNameLength: __le32, pub EaSize: __le32, pub Reserved: __le32, pub UniqueId: __le64, pub FileName: [i8;0] }
#[repr(C, packed)] pub struct smb2_file_internal_info { pub IndexNumber: __le64 }
#[repr(C, packed)] pub struct smb2_file_link_info { pub ReplaceIfExists: __u8, pub Reserved: [__u8;7], pub RootDirectory: __u64, pub FileNameLength: __le32, pub FileName: [i8;0] }
#[repr(C, packed)] pub struct smb2_file_network_open_info { pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub AllocationSize: __le64, pub EndOfFile: __le64, pub Attributes: __le32, pub Reserved: __le32 }
#[repr(C, packed)] pub struct smb2_file_rename_info { pub ReplaceIfExists: __u8, pub Reserved: [__u8;7], pub RootDirectory: __u64, pub FileNameLength: __le32, pub FileName: [i8;0] }

pub const FS_VOLUME_INFORMATION:u32=1; pub const FS_LABEL_INFORMATION:u32=2; pub const FS_SIZE_INFORMATION:u32=3; pub const FS_DEVICE_INFORMATION:u32=4; pub const FS_ATTRIBUTE_INFORMATION:u32=5; pub const FS_CONTROL_INFORMATION:u32=6; pub const FS_FULL_SIZE_INFORMATION:u32=7; pub const FS_OBJECT_ID_INFORMATION:u32=8; pub const FS_DRIVER_PATH_INFORMATION:u32=9; pub const FS_SECTOR_SIZE_INFORMATION:u32=11; pub const FS_POSIX_INFORMATION:u32=100;
pub const MAX_FS_NAME_LEN:u32=52;
#[repr(C, packed)] pub struct FILE_SYSTEM_ATTRIBUTE_INFO { pub Attributes: __le32, pub MaxPathNameComponentLength: __le32, pub FileSystemNameLen: __le32, pub FileSystemName: [__le16;0] }
pub const FILE_SUPPORTS_SPARSE_VDL:u32=0x10000000; pub const FILE_SUPPORTS_BLOCK_REFCOUNTING:u32=0x08000000; pub const FILE_SUPPORT_INTEGRITY_STREAMS:u32=0x04000000; pub const FILE_SUPPORTS_USN_JOURNAL:u32=0x02000000; pub const FILE_SUPPORTS_OPEN_BY_FILE_ID:u32=0x01000000; pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES:u32=0x00800000; pub const FILE_SUPPORTS_HARD_LINKS:u32=0x00400000; pub const FILE_SUPPORTS_TRANSACTIONS:u32=0x00200000; pub const FILE_SEQUENTIAL_WRITE_ONCE:u32=0x00100000; pub const FILE_READ_ONLY_VOLUME:u32=0x00080000; pub const FILE_NAMED_STREAMS:u32=0x00040000; pub const FILE_SUPPORTS_ENCRYPTION:u32=0x00020000; pub const FILE_SUPPORTS_OBJECT_IDS:u32=0x00010000; pub const FILE_VOLUME_IS_COMPRESSED:u32=0x00008000; pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME:u32=0x00000400; pub const FILE_RETURNS_CLEANUP_RESULT_INFO:u32=0x00000200; pub const FILE_SUPPORTS_REMOTE_STORAGE:u32=0x00000100; pub const FILE_SUPPORTS_REPARSE_POINTS:u32=0x00000080; pub const FILE_SUPPORTS_SPARSE_FILES:u32=0x00000040; pub const FILE_VOLUME_QUOTAS:u32=0x00000020; pub const FILE_FILE_COMPRESSION:u32=0x00000010; pub const FILE_PERSISTENT_ACLS:u32=0x00000008; pub const FILE_UNICODE_ON_DISK:u32=0x00000004; pub const FILE_CASE_PRESERVED_NAMES:u32=0x00000002; pub const FILE_CASE_SENSITIVE_SEARCH:u32=0x00000001;
#[repr(C, packed)] pub struct smb2_fs_control_info { pub FreeSpaceStartFiltering: __le64, pub FreeSpaceThreshold: __le64, pub FreeSpaceStopFiltering: __le64, pub DefaultQuotaThreshold: __le64, pub DefaultQuotaLimit: __le64, pub FileSystemControlFlags: __le32, pub Padding: __le32 }
#[repr(C, packed)] pub struct smb2_fs_full_size_info { pub TotalAllocationUnits: __le64, pub CallerAvailableAllocationUnits: __le64, pub ActualAvailableAllocationUnits: __le64, pub SectorsPerAllocationUnit: __le32, pub BytesPerSector: __le32 }
pub const SSINFO_FLAGS_ALIGNED_DEVICE:u32=1; pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE:u32=2; pub const SSINFO_FLAGS_NO_SEEK_PENALTY:u32=4; pub const SSINFO_FLAGS_TRIM_ENABLED:u32=8;
#[repr(C, packed)] pub struct smb3_fs_ss_info { pub LogicalBytesPerSector: __le32, pub PhysicalBytesPerSectorForAtomicity: __le32, pub PhysicalBytesPerSectorForPerf: __le32, pub FSEffPhysicalBytesPerSectorForAtomicity: __le32, pub Flags: __le32, pub ByteOffsetForSectorAlignment: __le32, pub ByteOffsetForPartitionAlignment: __le32 }
#[repr(C, packed)] pub struct FILE_SYSTEM_SIZE_INFO { pub TotalAllocationUnits: __le64, pub AvailableAllocationUnits: __le64, pub SectorsPerAllocationUnit: __le32, pub BytesPerSector: __le32 }
pub const MAX_VOL_LABEL_LEN:u32=32;
#[repr(C, packed)] pub struct filesystem_vol_info { pub VolumeCreationTime: __le64, pub VolumeSerialNumber: __le32, pub VolumeLabelLength: __le32, pub SupportsObjects: __u8, pub Reserved: __u8, pub VolumeLabel: [__u8;0] }
#[repr(C, packed)] pub struct FILE_SYSTEM_DEVICE_INFO { pub DeviceType: __le32, pub DeviceCharacteristics: __le32 }

pub const FILE_ATTRIBUTE_READONLY:u32=1; pub const FILE_ATTRIBUTE_HIDDEN:u32=2; pub const FILE_ATTRIBUTE_SYSTEM:u32=4; pub const FILE_ATTRIBUTE_DIRECTORY:u32=0x10; pub const FILE_ATTRIBUTE_ARCHIVE:u32=0x20; pub const FILE_ATTRIBUTE_NORMAL:u32=0x80; pub const FILE_ATTRIBUTE_TEMPORARY:u32=0x100; pub const FILE_ATTRIBUTE_SPARSE_FILE:u32=0x200; pub const FILE_ATTRIBUTE_REPARSE_POINT:u32=0x400; pub const FILE_ATTRIBUTE_COMPRESSED:u32=0x800; pub const FILE_ATTRIBUTE_OFFLINE:u32=0x1000; pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED:u32=0x2000; pub const FILE_ATTRIBUTE_ENCRYPTED:u32=0x4000; pub const FILE_ATTRIBUTE_INTEGRITY_STREAM:u32=0x8000; pub const FILE_ATTRIBUTE_NO_SCRUB_DATA:u32=0x20000;
pub const FILE_ATTRIBUTE_MASK:u32=FILE_ATTRIBUTE_READONLY|FILE_ATTRIBUTE_HIDDEN|FILE_ATTRIBUTE_SYSTEM|FILE_ATTRIBUTE_DIRECTORY|FILE_ATTRIBUTE_ARCHIVE|FILE_ATTRIBUTE_NORMAL|FILE_ATTRIBUTE_TEMPORARY|FILE_ATTRIBUTE_SPARSE_FILE|FILE_ATTRIBUTE_REPARSE_POINT|FILE_ATTRIBUTE_COMPRESSED|FILE_ATTRIBUTE_OFFLINE|FILE_ATTRIBUTE_NOT_CONTENT_INDEXED|FILE_ATTRIBUTE_ENCRYPTED|FILE_ATTRIBUTE_INTEGRITY_STREAM|FILE_ATTRIBUTE_NO_SCRUB_DATA;
pub const FILE_ATTRIBUTE_READONLY_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_READONLY); pub const FILE_ATTRIBUTE_HIDDEN_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_HIDDEN); pub const FILE_ATTRIBUTE_SYSTEM_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_SYSTEM); pub const FILE_ATTRIBUTE_DIRECTORY_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_DIRECTORY); pub const FILE_ATTRIBUTE_ARCHIVE_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_ARCHIVE); pub const FILE_ATTRIBUTE_NORMAL_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_NORMAL); pub const FILE_ATTRIBUTE_TEMPORARY_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_TEMPORARY); pub const FILE_ATTRIBUTE_SPARSE_FILE_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_SPARSE_FILE); pub const FILE_ATTRIBUTE_REPARSE_POINT_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_REPARSE_POINT); pub const FILE_ATTRIBUTE_COMPRESSED_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_COMPRESSED); pub const FILE_ATTRIBUTE_OFFLINE_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_OFFLINE); pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_NOT_CONTENT_INDEXED); pub const FILE_ATTRIBUTE_ENCRYPTED_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_ENCRYPTED); pub const FILE_ATTRIBUTE_INTEGRITY_STREAM_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_INTEGRITY_STREAM); pub const FILE_ATTRIBUTE_NO_SCRUB_DATA_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_NO_SCRUB_DATA); pub const FILE_ATTRIBUTE_MASK_LE:__le32=cpu_to_le32(FILE_ATTRIBUTE_MASK);
pub const FILE_ACTION_ADDED:u32=1; pub const FILE_ACTION_REMOVED:u32=2; pub const FILE_ACTION_MODIFIED:u32=3; pub const FILE_ACTION_RENAMED_OLD_NAME:u32=4; pub const FILE_ACTION_RENAMED_NEW_NAME:u32=5; pub const FILE_ACTION_ADDED_STREAM:u32=6; pub const FILE_ACTION_REMOVED_STREAM:u32=7; pub const FILE_ACTION_MODIFIED_STREAM:u32=8; pub const FILE_ACTION_REMOVED_BY_DELETE:u32=9; pub const FILE_ACTION_ID_NOT_TUNNELLED:u32=10; pub const FILE_ACTION_TUNNELLED_ID_COLLISION:u32=11;
#[repr(C, packed)] pub struct file_notify_information { pub NextEntryOffset: __le32, pub Action: __le32, pub FileNameLength: __le32, pub FileName: [__u8;0] }
#[repr(C, packed)] pub struct FILE_SYSTEM_POSIX_INFO { pub OptimalTransferSize: __le32, pub BlockSize: __le32, pub TotalBlocks: __le64, pub BlocksAvail: __le64, pub UserBlocksAvail: __le64, pub TotalFileNodes: __le64, pub FreeFileNodes: __le64, pub FileSysIdentifier: __le64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
