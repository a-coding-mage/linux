/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the surrounding translation unit: linux/types.h

/* general boundary definitions */
pub const SENSEINFOBYTES: usize = 32; /* note that this value may vary
                                         between host implementations */

/* Command Status value */
pub const CMD_SUCCESS: u16 = 0x0000;
pub const CMD_TARGET_STATUS: u16 = 0x0001;
pub const CMD_DATA_UNDERRUN: u16 = 0x0002;
pub const CMD_DATA_OVERRUN: u16 = 0x0003;
pub const CMD_INVALID: u16 = 0x0004;
pub const CMD_PROTOCOL_ERR: u16 = 0x0005;
pub const CMD_HARDWARE_ERR: u16 = 0x0006;
pub const CMD_CONNECTION_LOST: u16 = 0x0007;
pub const CMD_ABORTED: u16 = 0x0008;
pub const CMD_ABORT_FAILED: u16 = 0x0009;
pub const CMD_UNSOLICITED_ABORT: u16 = 0x000A;
pub const CMD_TIMEOUT: u16 = 0x000B;
pub const CMD_UNABORTABLE: u16 = 0x000C;

/* transfer direction */
pub const XFER_NONE: u8 = 0x00;
pub const XFER_WRITE: u8 = 0x01;
pub const XFER_READ: u8 = 0x02;
pub const XFER_RSVD: u8 = 0x03;

/* task attribute */
pub const ATTR_UNTAGGED: u8 = 0x00;
pub const ATTR_SIMPLE: u8 = 0x04;
pub const ATTR_HEADOFQUEUE: u8 = 0x05;
pub const ATTR_ORDERED: u8 = 0x06;
pub const ATTR_ACA: u8 = 0x07;

/* cdb type */
pub const TYPE_CMD: u8 = 0x00;
pub const TYPE_MSG: u8 = 0x01;

/* Type defs used in the following structs */
pub type BYTE = u8;
pub type WORD = u16;
pub type HWORD = u16;
pub type DWORD = u32;

pub const CISS_MAX_LUN: usize = 1024;
pub const LEVEL2LUN: usize = 1; /* index into Target(x) structure, due to byte swapping */
pub const LEVEL3LUN: usize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union SCSI3Addr_struct {
    pub PeripDev: SCSI3Addr_PeripDev,
    pub LogDev: SCSI3Addr_LogDev,
    pub LogUnit: SCSI3Addr_LogUnit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SCSI3Addr_PeripDev {
    pub Dev: BYTE,
    // C bitfields: Bus:6, Mode:2
    pub Bus_Mode: BYTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SCSI3Addr_LogDev {
    pub DevLSB: BYTE,
    // C bitfields: DevMSB:6, Mode:2
    pub DevMSB_Mode: BYTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SCSI3Addr_LogUnit {
    // C bitfields: Dev:5, Bus:3
    pub Dev_Bus: BYTE,
    // C bitfields: Targ:6, Mode:2
    pub Targ_Mode: BYTE,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PhysDevAddr_struct {
    // C bitfields: TargetId:24, Bus:6, Mode:2
    pub TargetId_Bus_Mode: DWORD,
    pub Target: [SCSI3Addr_struct; 2], /* 2 level target device addr */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct LogDevAddr_struct {
    // C bitfields: VolId:30, Mode:2
    pub VolId_Mode: DWORD,
    pub reserved: [BYTE; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union LUNAddr_struct {
    pub LunAddrBytes: [BYTE; 8],
    pub SCSI3Lun: [SCSI3Addr_struct; 4],
    pub PhysDev: PhysDevAddr_struct,
    pub LogDev: LogDevAddr_struct,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct RequestBlock_struct {
    pub CDBLen: BYTE,
    pub Type: RequestBlock_Type,
    pub Timeout: HWORD,
    pub CDB: [BYTE; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RequestBlock_Type {
    // C bitfields: Type:3, Attribute:3, Direction:2
    pub Type_Attribute_Direction: BYTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MoreErrInfo_struct {
    pub Common_Info: MoreErrInfo_Common_Info,
    pub Invalid_Cmd: MoreErrInfo_Invalid_Cmd,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MoreErrInfo_Common_Info {
    pub Reserved: [BYTE; 3],
    pub Type: BYTE,
    pub ErrorInfo: DWORD,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MoreErrInfo_Invalid_Cmd {
    pub Reserved: [BYTE; 2],
    pub offense_size: BYTE, /* size of offending entry */
    pub offense_num: BYTE, /* byte # of offense 0-base */
    pub offense_value: DWORD,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ErrorInfo_struct {
    pub ScsiStatus: BYTE,
    pub SenseLen: BYTE,
    pub CommandStatus: HWORD,
    pub ResidualCnt: DWORD,
    pub MoreErrInfo: MoreErrInfo_struct,
    pub SenseInfo: [BYTE; SENSEINFOBYTES],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
