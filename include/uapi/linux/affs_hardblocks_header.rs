/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Linux types: __be32 is represented as u32, and __u8 as u8.

/* Just the needed definitions for the RDB of an Amiga HD. */

#[repr(C)]
pub struct RigidDiskBlock {
    pub rdb_ID: u32,
    pub rdb_SummedLongs: u32,
    pub rdb_ChkSum: u32,
    pub rdb_HostID: u32,
    pub rdb_BlockBytes: u32,
    pub rdb_Flags: u32,
    pub rdb_BadBlockList: u32,
    pub rdb_PartitionList: u32,
    pub rdb_FileSysHeaderList: u32,
    pub rdb_DriveInit: u32,
    pub rdb_Reserved1: [u32; 6],
    pub rdb_Cylinders: u32,
    pub rdb_Sectors: u32,
    pub rdb_Heads: u32,
    pub rdb_Interleave: u32,
    pub rdb_Park: u32,
    pub rdb_Reserved2: [u32; 3],
    pub rdb_WritePreComp: u32,
    pub rdb_ReducedWrite: u32,
    pub rdb_StepRate: u32,
    pub rdb_Reserved3: [u32; 5],
    pub rdb_RDBBlocksLo: u32,
    pub rdb_RDBBlocksHi: u32,
    pub rdb_LoCylinder: u32,
    pub rdb_HiCylinder: u32,
    pub rdb_CylBlocks: u32,
    pub rdb_AutoParkSeconds: u32,
    pub rdb_HighRDSKBlock: u32,
    pub rdb_Reserved4: u32,
    pub rdb_DiskVendor: [i8; 8],
    pub rdb_DiskProduct: [i8; 16],
    pub rdb_DiskRevision: [i8; 4],
    pub rdb_ControllerVendor: [i8; 8],
    pub rdb_ControllerProduct: [i8; 16],
    pub rdb_ControllerRevision: [i8; 4],
    pub rdb_Reserved5: [u32; 10],
}

pub const IDNAME_RIGIDDISK: u32 = 0x5244_534B; /* "RDSK" */

#[repr(C)]
pub struct PartitionBlock {
    pub pb_ID: u32,
    pub pb_SummedLongs: u32,
    pub pb_ChkSum: u32,
    pub pb_HostID: u32,
    pub pb_Next: u32,
    pub pb_Flags: u32,
    pub pb_Reserved1: [u32; 2],
    pub pb_DevFlags: u32,
    pub pb_DriveName: [u8; 32],
    pub pb_Reserved2: [u32; 15],
    pub pb_Environment: [u32; 17],
    pub pb_EReserved: [u32; 15],
}

pub const IDNAME_PARTITION: u32 = 0x5041_5254; /* "PART" */

pub const RDB_ALLOCATION_LIMIT: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
