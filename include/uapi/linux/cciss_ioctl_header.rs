/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C dependencies supplied by other headers: LUNAddr_struct, RequestBlock_struct,
// ErrorInfo_struct, WORD, BYTE, DWORD, and the Linux ioctl encoding definitions.

pub const CCISS_IOC_MAGIC: u8 = b'B';

#[repr(C)]
pub struct cciss_pci_info_struct {
    pub bus: u8,
    pub dev_fn: u8,
    pub domain: u16,
    pub board_id: u32,
}

#[repr(C)]
pub struct cciss_coalint_struct {
    pub delay: u32,
    pub count: u32,
}

pub type NodeName_type = [core::ffi::c_char; 16];
pub type Heartbeat_type = u32;

pub const CISS_PARSCSIU2: u32 = 0x0001;
pub const CISS_PARCSCIU3: u32 = 0x0002;
pub const CISS_FIBRE1G: u32 = 0x0100;
pub const CISS_FIBRE2G: u32 = 0x0200;
pub type BusTypes_type = u32;

pub type FirmwareVer_type = [core::ffi::c_char; 4];
pub type DriverVer_type = u32;

pub const MAX_KMALLOC_SIZE: usize = 128000;

#[repr(C)]
pub struct IOCTL_Command_struct {
    pub LUN_info: LUNAddr_struct,
    pub Request: RequestBlock_struct,
    pub error_info: ErrorInfo_struct,
    pub buf_size: WORD,
    pub buf: *mut BYTE,
}

#[repr(C)]
pub struct BIG_IOCTL_Command_struct {
    pub LUN_info: LUNAddr_struct,
    pub Request: RequestBlock_struct,
    pub error_info: ErrorInfo_struct,
    pub malloc_size: DWORD,
    pub buf_size: DWORD,
    pub buf: *mut BYTE,
}

#[repr(C)]
pub struct LogvolInfo_struct {
    pub LunID: u32,
    pub num_opens: core::ffi::c_int,
    pub num_parts: core::ffi::c_int,
}

// no longer used... use REGNEWD instead

pub const CCISS_GETPCIINFO: u32 = _IOR(CCISS_IOC_MAGIC, 1, cciss_pci_info_struct);
pub const CCISS_GETINTINFO: u32 = _IOR(CCISS_IOC_MAGIC, 2, cciss_coalint_struct);
pub const CCISS_SETINTINFO: u32 = _IOW(CCISS_IOC_MAGIC, 3, cciss_coalint_struct);
pub const CCISS_GETNODENAME: u32 = _IOR(CCISS_IOC_MAGIC, 4, NodeName_type);
pub const CCISS_SETNODENAME: u32 = _IOW(CCISS_IOC_MAGIC, 5, NodeName_type);
pub const CCISS_GETHEARTBEAT: u32 = _IOR(CCISS_IOC_MAGIC, 6, Heartbeat_type);
pub const CCISS_GETBUSTYPES: u32 = _IOR(CCISS_IOC_MAGIC, 7, BusTypes_type);
pub const CCISS_GETFIRMVER: u32 = _IOR(CCISS_IOC_MAGIC, 8, FirmwareVer_type);
pub const CCISS_GETDRIVVER: u32 = _IOR(CCISS_IOC_MAGIC, 9, DriverVer_type);
pub const CCISS_REVALIDVOLS: u32 = _IO(CCISS_IOC_MAGIC, 10);
pub const CCISS_PASSTHRU: u32 = _IOWR(CCISS_IOC_MAGIC, 11, IOCTL_Command_struct);
pub const CCISS_DEREGDISK: u32 = _IO(CCISS_IOC_MAGIC, 12);
pub const CCISS_REGNEWDISK: u32 = _IOW(CCISS_IOC_MAGIC, 13, core::ffi::c_int);
pub const CCISS_REGNEWD: u32 = _IO(CCISS_IOC_MAGIC, 14);
pub const CCISS_RESCANDISK: u32 = _IO(CCISS_IOC_MAGIC, 16);
pub const CCISS_GETLUNINFO: u32 = _IOR(CCISS_IOC_MAGIC, 17, LogvolInfo_struct);
pub const CCISS_BIG_PASSTHRU: u32 = _IOWR(CCISS_IOC_MAGIC, 18, BIG_IOCTL_Command_struct);
pub const CCISS_BIG_PASSTHRU_SUPPORTED: u32 = _IO(CCISS_IOC_MAGIC, 19);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
