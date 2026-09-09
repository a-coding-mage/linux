/* SPDX-License-Identifier: LGPL-2.1+ */
/*
 *   SMB, CIFS, SMB2 FSCTL definitions
 *
 *   Copyright (c) International Business Machines  Corp., 2002,2013
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */

/* IOCTL information */
/*
 * List of ioctl/fsctl function codes that are or could be useful in the
 * future to remote clients like cifs or SMB2/SMB3 client.  This is probably
 * a slightly larger set of fsctls that NTFS local filesystem could handle,
 * including the seven below that we do not have struct definitions for.
 * Even with protocol definitions for most of these now available, we still
 * need to do some experimentation to identify which are practical to do
 * remotely.  Some of the following, such as the encryption/compression ones
 * could be invoked from tools via a specialized hook into the VFS rather
 * than via the standard vfs entry points
 *
 * See MS-SMB2 Section 2.2.31 (last checked September 2021, all of that list are
 * below). Additional detail on less common ones can be found in MS-FSCC
 * section 2.3.
 */

/*
 * FSCTL values are 32 bits and are constructed as
 * <device 16bits> <access 2bits> <function 12bits> <method 2bits>
 */
/* Device */
pub const FSCTL_DEVICE_DFS: u32 = 0x0006 << 16;
pub const FSCTL_DEVICE_FILE_SYSTEM: u32 = 0x0009 << 16;
pub const FSCTL_DEVICE_NAMED_PIPE: u32 = 0x0011 << 16;
pub const FSCTL_DEVICE_NETWORK_FILE_SYSTEM: u32 = 0x0014 << 16;
pub const FSCTL_DEVICE_MASK: u32 = 0xffff0000;
/* Access */
pub const FSCTL_DEVICE_ACCESS_FILE_ANY_ACCESS: u32 = 0x00 << 14;
pub const FSCTL_DEVICE_ACCESS_FILE_READ_ACCESS: u32 = 0x01 << 14;
pub const FSCTL_DEVICE_ACCESS_FILE_WRITE_ACCESS: u32 = 0x02 << 14;
pub const FSCTL_DEVICE_ACCESS_FILE_READ_WRITE_ACCESS: u32 = 0x03 << 14;
pub const FSCTL_DEVICE_ACCESS_MASK: u32 = 0x0000c000;
/* Function */
pub const FSCTL_DEVICE_FUNCTION_MASK: u32 = 0x00003ffc;
/* Method */
pub const FSCTL_DEVICE_METHOD_BUFFERED: u32 = 0x00;
pub const FSCTL_DEVICE_METHOD_IN_DIRECT: u32 = 0x01;
pub const FSCTL_DEVICE_METHOD_OUT_DIRECT: u32 = 0x02;
pub const FSCTL_DEVICE_METHOD_NEITHER: u32 = 0x03;
pub const FSCTL_DEVICE_METHOD_MASK: u32 = 0x00000003;

pub const FSCTL_DFS_GET_REFERRALS: u32 = 0x00060194;
pub const FSCTL_DFS_GET_REFERRALS_EX: u32 = 0x000601B0;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: u32 = 0x00090000;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: u32 = 0x00090004;
pub const FSCTL_REQUEST_BATCH_OPLOCK: u32 = 0x00090008;
pub const FSCTL_LOCK_VOLUME: u32 = 0x00090018;
pub const FSCTL_UNLOCK_VOLUME: u32 = 0x0009001C;
pub const FSCTL_IS_PATHNAME_VALID: u32 = 0x0009002C; /* BB add struct */
pub const FSCTL_GET_COMPRESSION: u32 = 0x0009003C;
pub const FSCTL_SET_COMPRESSION: u32 = 0x0009C040;
pub const FSCTL_QUERY_FAT_BPB: u32 = 0x00090058; /* BB add struct */
/* Verify the next FSCTL number, we had it as 0x00090090 before */
pub const FSCTL_FILESYSTEM_GET_STATS: u32 = 0x00090060; /* BB add struct */
pub const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 0x00090064; /* BB add struct */
pub const FSCTL_GET_RETRIEVAL_POINTERS: u32 = 0x00090073; /* BB add struct */
pub const FSCTL_IS_VOLUME_DIRTY: u32 = 0x00090078; /* BB add struct */
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: u32 = 0x00090083; /* BB add struct */
pub const FSCTL_REQUEST_FILTER_OPLOCK: u32 = 0x0009008C;
pub const FSCTL_FIND_FILES_BY_SID: u32 = 0x0009008F; /* BB add struct */
pub const FSCTL_SET_OBJECT_ID: u32 = 0x00090098; /* BB add struct */
pub const FSCTL_GET_OBJECT_ID: u32 = 0x0009009C; /* BB add struct */
pub const FSCTL_DELETE_OBJECT_ID: u32 = 0x000900A0; /* BB add struct */
pub const FSCTL_SET_REPARSE_POINT: u32 = 0x000900A4; /* BB add struct */
pub const FSCTL_GET_REPARSE_POINT: u32 = 0x000900A8; /* BB add struct */
pub const FSCTL_DELETE_REPARSE_POINT: u32 = 0x000900AC; /* BB add struct */
pub const FSCTL_SET_OBJECT_ID_EXTENDED: u32 = 0x000900BC; /* BB add struct */
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: u32 = 0x000900C0; /* BB add struct */
pub const FSCTL_SET_SPARSE: u32 = 0x000900C4; /* BB add struct */
pub const FSCTL_SET_ZERO_DATA: u32 = 0x000980C8;
pub const FSCTL_SET_ENCRYPTION: u32 = 0x000900D7; /* BB add struct */
pub const FSCTL_ENCRYPTION_FSCTL_IO: u32 = 0x000900DB; /* BB add struct */
pub const FSCTL_WRITE_RAW_ENCRYPTED: u32 = 0x000900DF; /* BB add struct */
pub const FSCTL_READ_RAW_ENCRYPTED: u32 = 0x000900E3; /* BB add struct */
pub const FSCTL_READ_FILE_USN_DATA: u32 = 0x000900EB; /* BB add struct */
pub const FSCTL_WRITE_USN_CLOSE_RECORD: u32 = 0x000900EF; /* BB add struct */
pub const FSCTL_MARK_HANDLE: u32 = 0x000900FC; /* BB add struct */
pub const FSCTL_SIS_COPYFILE: u32 = 0x00090100; /* BB add struct */
pub const FSCTL_RECALL_FILE: u32 = 0x00090117; /* BB add struct */
pub const FSCTL_QUERY_SPARING_INFO: u32 = 0x00090138; /* BB add struct */
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: u32 = 0x0009013C;
pub const FSCTL_SET_ZERO_ON_DEALLOC: u32 = 0x00090194; /* BB add struct */
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: u32 = 0x000901B4; /* BB add struct */
pub const FSCTL_GET_INTEGRITY_INFORMATION: u32 = 0x0009027C;
pub const FSCTL_QUERY_FILE_REGIONS: u32 = 0x00090284;
pub const FSCTL_GET_REFS_VOLUME_DATA: u32 = 0x000902D8; /* See MS-FSCC 2.3.24 */
pub const FSCTL_SET_INTEGRITY_INFORMATION_EXT: u32 = 0x00090380;
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: u32 = 0x000903d3;
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: u32 = 0x0009042b;
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: u32 = 0x00090440;
pub const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 0x000940CF;
pub const FSCTL_OFFLOAD_READ: u32 = 0x00094264; /* BB add struct */
pub const FSCTL_OFFLOAD_WRITE: u32 = 0x00098268; /* BB add struct */
pub const FSCTL_SET_DEFECT_MANAGEMENT: u32 = 0x00098134; /* BB add struct */
pub const FSCTL_FILE_LEVEL_TRIM: u32 = 0x00098208; /* BB add struct */
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: u32 = 0x00098344;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: u32 = 0x000983E8;
pub const FSCTL_SIS_LINK_FILES: u32 = 0x0009C104;
pub const FSCTL_SET_INTEGRITY_INFORMATION: u32 = 0x0009C280;
pub const FSCTL_PIPE_PEEK: u32 = 0x0011400C; /* BB add struct */
pub const FSCTL_PIPE_TRANSCEIVE: u32 = 0x0011C017; /* BB add struct */
/* strange that the number for this op is not sequential with previous op */
pub const FSCTL_PIPE_WAIT: u32 = 0x00110018; /* BB add struct */
/* Enumerate previous versions of a file */
pub const FSCTL_SRV_ENUMERATE_SNAPSHOTS: u32 = 0x00144064;
/* Retrieve an opaque file reference for server-side data movement ie copy */
pub const FSCTL_SRV_REQUEST_RESUME_KEY: u32 = 0x00140078;
pub const FSCTL_SRV_ENUM_SNAPS: u32 = 0x00144064;
pub const FSCTL_LMR_REQUEST_RESILIENCY: u32 = 0x001401D4;
pub const FSCTL_LMR_GET_LINK_TRACK_INF: u32 = 0x001400E8; /* BB add struct */
pub const FSCTL_LMR_SET_LINK_TRACK_INF: u32 = 0x001400EC; /* BB add struct */
pub const FSCTL_VALIDATE_NEGOTIATE_INFO: u32 = 0x00140204;
/* Perform server-side data movement */
pub const FSCTL_SRV_COPYCHUNK: u32 = 0x001440F2;
pub const FSCTL_SRV_COPYCHUNK_WRITE: u32 = 0x001480F2;
pub const FSCTL_QUERY_NETWORK_INTERFACE_INFO: u32 = 0x001401FC; /* BB add struct */
pub const FSCTL_SRV_READ_HASH: u32 = 0x001441BB; /* BB add struct */

/* See FSCC 2.1.2.5 */
pub const IO_REPARSE_TAG_MOUNT_POINT: u32 = 0xA0000003;
pub const IO_REPARSE_TAG_HSM: u32 = 0xC0000004;
pub const IO_REPARSE_TAG_SIS: u32 = 0x80000007;
pub const IO_REPARSE_TAG_HSM2: u32 = 0x80000006;
pub const IO_REPARSE_TAG_DRIVER_EXTENDER: u32 = 0x80000005;
/* Used by the DFS filter. See MS-DFSC */
pub const IO_REPARSE_TAG_DFS: u32 = 0x8000000A;
/* Used by the DFS filter See MS-DFSC */
pub const IO_REPARSE_TAG_DFSR: u32 = 0x80000012;
pub const IO_REPARSE_TAG_FILTER_MANAGER: u32 = 0x8000000B;
/* Native SMB symlinks since Windows Vista, see MS-FSCC 2.1.2.4 */
pub const IO_REPARSE_TAG_SYMLINK: u32 = 0xA000000C;
pub const IO_REPARSE_TAG_DEDUP: u32 = 0x80000013;
pub const IO_REPARSE_APPXSTREAM: u32 = 0xC0000014;
/* NFS special files used by Windows NFS server since Windows Server 2012, see MS-FSCC 2.1.2.6 */
pub const IO_REPARSE_TAG_NFS: u32 = 0x80000014;
/*
 * AzureFileSync - see
 * https://docs.microsoft.com/en-us/azure/storage/files/storage-sync-cloud-tiering
 */
pub const IO_REPARSE_TAG_AZ_FILE_SYNC: u32 = 0x8000001e;
/* Native Win32 AF_UNIX sockets since Windows 10 April 2018 Update, used also by WSL */
pub const IO_REPARSE_TAG_AF_UNIX: u32 = 0x80000023;
/* WSL reparse tags */
pub const IO_REPARSE_TAG_LX_SYMLINK: u32 = 0xA000001D;
pub const IO_REPARSE_TAG_LX_FIFO: u32 = 0x80000024;
pub const IO_REPARSE_TAG_LX_CHR: u32 = 0x80000025;
pub const IO_REPARSE_TAG_LX_BLK: u32 = 0x80000026;

/* `cpu_to_le32` is supplied by an external dependency. */
#[macro_export]
macro_rules! IO_REPARSE_TAG_LX_SYMLINK_LE { () => { cpu_to_le32(IO_REPARSE_TAG_LX_SYMLINK) }; }
#[macro_export]
macro_rules! IO_REPARSE_TAG_AF_UNIX_LE { () => { cpu_to_le32(IO_REPARSE_TAG_AF_UNIX) }; }
#[macro_export]
macro_rules! IO_REPARSE_TAG_LX_FIFO_LE { () => { cpu_to_le32(IO_REPARSE_TAG_LX_FIFO) }; }
#[macro_export]
macro_rules! IO_REPARSE_TAG_LX_CHR_LE { () => { cpu_to_le32(IO_REPARSE_TAG_LX_CHR) }; }
#[macro_export]
macro_rules! IO_REPARSE_TAG_LX_BLK_LE { () => { cpu_to_le32(IO_REPARSE_TAG_LX_BLK) }; }

/* If Name Surrogate Bit is set, the file or directory represents another named entity in the system. */
#[macro_export]
macro_rules! IS_REPARSE_TAG_NAME_SURROGATE { ($tag:expr) => { (($tag & 0x20000000) != 0) }; }

/* fsctl flags */
/* If Flags is set to this value, the request is an FSCTL not ioctl request */
pub const SMB2_0_IOCTL_IS_FSCTL: u32 = 0x00000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
