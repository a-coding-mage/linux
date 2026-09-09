/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from linux/hdreg.h. `__u8` and related types are supplied by linux/types.h.

pub const HDIO_DRIVE_CMD_HDR_SIZE: usize = 4 * core::mem::size_of::<__u8>();
pub const HDIO_DRIVE_HOB_HDR_SIZE: usize = 8 * core::mem::size_of::<__u8>();
pub const HDIO_DRIVE_TASK_HDR_SIZE: usize = 8 * core::mem::size_of::<__u8>();
pub const IDE_DRIVE_TASK_NO_DATA: i32 = 0;
// The following constants are excluded by the C header when __KERNEL__ is defined.
pub const IDE_DRIVE_TASK_INVALID: i32 = -1;
pub const IDE_DRIVE_TASK_SET_XFER: i32 = 1;
pub const IDE_DRIVE_TASK_IN: i32 = 2;
pub const IDE_DRIVE_TASK_OUT: i32 = 3;
pub const IDE_DRIVE_TASK_RAW_WRITE: i32 = 4;
pub const IDE_TASKFILE_STD_IN_FLAGS: u32 = 0xfe;
pub const IDE_HOB_STD_IN_FLAGS: u32 = 0x3c;
pub const IDE_TASKFILE_STD_OUT_FLAGS: u32 = 0xfe;
pub const IDE_HOB_STD_OUT_FLAGS: u32 = 0x3c;
pub type task_ioreg_t = u8;
pub type sata_ioreg_t = ::core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ide_reg_valid_s {
    pub all: u16,
    // C bitfields occupy this same 16-bit word, from bit 0 through bit 15.
    pub b: u16,
}
pub type ide_reg_valid_t = ide_reg_valid_s;

#[repr(C)]
pub struct ide_task_request_s {
    pub io_ports: [__u8; 8],
    pub hob_ports: [__u8; 8],
    pub out_flags: ide_reg_valid_t,
    pub in_flags: ide_reg_valid_t,
    pub data_phase: ::core::ffi::c_int,
    pub req_cmd: ::core::ffi::c_int,
    pub out_size: ::core::ffi::c_ulong,
    pub in_size: ::core::ffi::c_ulong,
}
pub type ide_task_request_t = ide_task_request_s;

#[repr(C)]
pub struct ide_ioctl_request_s {
    pub task_request: *mut ide_task_request_t,
    pub out_buffer: *mut u8,
    pub in_buffer: *mut u8,
}
pub type ide_ioctl_request_t = ide_ioctl_request_s;

#[repr(C)]
pub struct hd_drive_cmd_hdr { pub command: __u8, pub sector_number: __u8, pub feature: __u8, pub sector_count: __u8 }

#[repr(C)]
pub struct hd_drive_task_hdr {
    pub data: __u8, pub feature: __u8, pub sector_count: __u8, pub sector_number: __u8,
    pub low_cylinder: __u8, pub high_cylinder: __u8, pub device_head: __u8, pub command: __u8,
}
pub type task_struct_t = hd_drive_task_hdr;
#[repr(C)]
pub struct hd_drive_hob_hdr {
    pub data: __u8, pub feature: __u8, pub sector_count: __u8, pub sector_number: __u8,
    pub low_cylinder: __u8, pub high_cylinder: __u8, pub device_head: __u8, pub control: __u8,
}
pub type hob_struct_t = hd_drive_hob_hdr;

pub const TASKFILE_NO_DATA: u16 = 0x0000;
pub const TASKFILE_IN: u16 = 0x0001;
pub const TASKFILE_MULTI_IN: u16 = 0x0002;
pub const TASKFILE_OUT: u16 = 0x0004;
pub const TASKFILE_MULTI_OUT: u16 = 0x0008;
pub const TASKFILE_IN_OUT: u16 = 0x0010;
pub const TASKFILE_IN_DMA: u16 = 0x0020;
pub const TASKFILE_OUT_DMA: u16 = 0x0040;
pub const TASKFILE_IN_DMAQ: u16 = 0x0080;
pub const TASKFILE_OUT_DMAQ: u16 = 0x0100;
pub const TASKFILE_P_IN: u16 = 0x0200;
pub const TASKFILE_P_OUT: u16 = 0x0400;
pub const TASKFILE_P_IN_DMA: u16 = 0x0800;
pub const TASKFILE_P_OUT_DMA: u16 = 0x1000;
pub const TASKFILE_P_IN_DMAQ: u16 = 0x2000;
pub const TASKFILE_P_OUT_DMAQ: u16 = 0x4000;
pub const TASKFILE_48: u16 = 0x8000;
pub const TASKFILE_INVALID: u16 = 0x7fff;

// ATA/ATAPI command constants (the C header exposes these outside kernel builds).
pub const WIN_NOP: u8 = 0x00; pub const CFA_REQ_EXT_ERROR_CODE: u8 = 0x03;
pub const WIN_SRST: u8 = 0x08; pub const WIN_DEVICE_RESET: u8 = 0x08;
pub const WIN_RECAL: u8 = 0x10; pub const WIN_RESTORE: u8 = WIN_RECAL;
pub const WIN_READ: u8 = 0x20; pub const WIN_READ_ONCE: u8 = 0x21; pub const WIN_READ_LONG: u8 = 0x22; pub const WIN_READ_LONG_ONCE: u8 = 0x23;
pub const WIN_READ_EXT: u8 = 0x24; pub const WIN_READDMA_EXT: u8 = 0x25; pub const WIN_READDMA_QUEUED_EXT: u8 = 0x26; pub const WIN_READ_NATIVE_MAX_EXT: u8 = 0x27;
pub const WIN_MULTREAD_EXT: u8 = 0x29; pub const WIN_WRITE: u8 = 0x30; pub const WIN_WRITE_ONCE: u8 = 0x31; pub const WIN_WRITE_LONG: u8 = 0x32; pub const WIN_WRITE_LONG_ONCE: u8 = 0x33;
pub const WIN_WRITE_EXT: u8 = 0x34; pub const WIN_WRITEDMA_EXT: u8 = 0x35; pub const WIN_WRITEDMA_QUEUED_EXT: u8 = 0x36; pub const WIN_SET_MAX_EXT: u8 = 0x37; pub const CFA_WRITE_SECT_WO_ERASE: u8 = 0x38; pub const WIN_MULTWRITE_EXT: u8 = 0x39;
pub const WIN_WRITE_VERIFY: u8 = 0x3c; pub const WIN_VERIFY: u8 = 0x40; pub const WIN_VERIFY_ONCE: u8 = 0x41; pub const WIN_VERIFY_EXT: u8 = 0x42; pub const WIN_FORMAT: u8 = 0x50; pub const WIN_INIT: u8 = 0x60; pub const WIN_SEEK: u8 = 0x70;
pub const CFA_TRANSLATE_SECTOR: u8 = 0x87; pub const WIN_DIAGNOSE: u8 = 0x90; pub const WIN_SPECIFY: u8 = 0x91; pub const WIN_DOWNLOAD_MICROCODE: u8 = 0x92; pub const WIN_STANDBYNOW2: u8 = 0x94; pub const WIN_STANDBY2: u8 = 0x96; pub const WIN_SETIDLE2: u8 = 0x97; pub const WIN_CHECKPOWERMODE2: u8 = 0x98; pub const WIN_SLEEPNOW2: u8 = 0x99;
pub const WIN_PACKETCMD: u8 = 0xa0; pub const WIN_PIDENTIFY: u8 = 0xa1; pub const WIN_QUEUED_SERVICE: u8 = 0xa2; pub const WIN_SMART: u8 = 0xb0; pub const CFA_ERASE_SECTORS: u8 = 0xc0; pub const WIN_MULTREAD: u8 = 0xc4; pub const WIN_MULTWRITE: u8 = 0xc5; pub const WIN_SETMULT: u8 = 0xc6; pub const WIN_READDMA_QUEUED: u8 = 0xc7; pub const WIN_READDMA: u8 = 0xc8; pub const WIN_READDMA_ONCE: u8 = 0xc9; pub const WIN_WRITEDMA: u8 = 0xca; pub const WIN_WRITEDMA_ONCE: u8 = 0xcb; pub const WIN_WRITEDMA_QUEUED: u8 = 0xcc; pub const CFA_WRITE_MULTI_WO_ERASE: u8 = 0xcd;
pub const WIN_GETMEDIASTATUS: u8 = 0xda; pub const WIN_ACKMEDIACHANGE: u8 = 0xdb; pub const WIN_POSTBOOT: u8 = 0xdc; pub const WIN_PREBOOT: u8 = 0xdd; pub const WIN_DOORLOCK: u8 = 0xde; pub const WIN_DOORUNLOCK: u8 = 0xdf; pub const WIN_STANDBYNOW1: u8 = 0xe0; pub const WIN_IDLEIMMEDIATE: u8 = 0xe1; pub const WIN_STANDBY: u8 = 0xe2; pub const WIN_SETIDLE1: u8 = 0xe3; pub const WIN_READ_BUFFER: u8 = 0xe4; pub const WIN_CHECKPOWERMODE1: u8 = 0xe5; pub const WIN_SLEEPNOW1: u8 = 0xe6; pub const WIN_FLUSH_CACHE: u8 = 0xe7; pub const WIN_WRITE_BUFFER: u8 = 0xe8; pub const WIN_WRITE_SAME: u8 = 0xe9; pub const WIN_FLUSH_CACHE_EXT: u8 = 0xea; pub const WIN_IDENTIFY: u8 = 0xec; pub const WIN_MEDIAEJECT: u8 = 0xed; pub const WIN_IDENTIFY_DMA: u8 = 0xee; pub const WIN_SETFEATURES: u8 = 0xef; pub const EXABYTE_ENABLE_NEST: u8 = 0xf0; pub const WIN_SECURITY_SET_PASS: u8 = 0xf1; pub const WIN_SECURITY_UNLOCK: u8 = 0xf2; pub const WIN_SECURITY_ERASE_PREPARE: u8 = 0xf3; pub const WIN_SECURITY_ERASE_UNIT: u8 = 0xf4; pub const WIN_SECURITY_FREEZE_LOCK: u8 = 0xf5; pub const WIN_SECURITY_DISABLE: u8 = 0xf6; pub const WIN_READ_NATIVE_MAX: u8 = 0xf8; pub const WIN_SET_MAX: u8 = 0xf9; pub const DISABLE_SEAGATE: u8 = 0xfb;

pub const SMART_READ_VALUES: u8 = 0xd0; pub const SMART_READ_THRESHOLDS: u8 = 0xd1; pub const SMART_AUTOSAVE: u8 = 0xd2; pub const SMART_SAVE: u8 = 0xd3; pub const SMART_IMMEDIATE_OFFLINE: u8 = 0xd4; pub const SMART_READ_LOG_SECTOR: u8 = 0xd5; pub const SMART_WRITE_LOG_SECTOR: u8 = 0xd6; pub const SMART_WRITE_THRESHOLDS: u8 = 0xd7; pub const SMART_ENABLE: u8 = 0xd8; pub const SMART_DISABLE: u8 = 0xd9; pub const SMART_STATUS: u8 = 0xda; pub const SMART_AUTO_OFFLINE: u8 = 0xdb;
pub const SMART_LCYL_PASS: u8 = 0x4f; pub const SMART_HCYL_PASS: u8 = 0xc2;
pub const SECURITY_SET_PASSWORD: u8 = 0xba; pub const SECURITY_UNLOCK: u8 = 0xbb; pub const SECURITY_ERASE_PREPARE: u8 = 0xbc; pub const SECURITY_ERASE_UNIT: u8 = 0xbd; pub const SECURITY_FREEZE_LOCK: u8 = 0xbe; pub const SECURITY_DISABLE_PASSWORD: u8 = 0xbf;
pub const SETFEATURES_EN_8BIT: u8 = 0x01; pub const SETFEATURES_EN_WCACHE: u8 = 0x02; pub const SETFEATURES_DIS_DEFECT: u8 = 0x04; pub const SETFEATURES_EN_APM: u8 = 0x05; pub const SETFEATURES_EN_SAME_R: u8 = 0x22; pub const SETFEATURES_DIS_MSN: u8 = 0x31; pub const SETFEATURES_DIS_RETRY: u8 = 0x33; pub const SETFEATURES_EN_AAM: u8 = 0x42; pub const SETFEATURES_RW_LONG: u8 = 0x44; pub const SETFEATURES_SET_CACHE: u8 = 0x54; pub const SETFEATURES_DIS_RLA: u8 = 0x55; pub const SETFEATURES_EN_RI: u8 = 0x5d; pub const SETFEATURES_EN_SI: u8 = 0x5e; pub const SETFEATURES_DIS_RPOD: u8 = 0x66; pub const SETFEATURES_DIS_ECC: u8 = 0x77; pub const SETFEATURES_DIS_8BIT: u8 = 0x81; pub const SETFEATURES_DIS_WCACHE: u8 = 0x82; pub const SETFEATURES_EN_DEFECT: u8 = 0x84; pub const SETFEATURES_DIS_APM: u8 = 0x85; pub const SETFEATURES_EN_ECC: u8 = 0x88; pub const SETFEATURES_EN_MSN: u8 = 0x95; pub const SETFEATURES_EN_RETRY: u8 = 0x99; pub const SETFEATURES_EN_RLA: u8 = 0xaa; pub const SETFEATURES_PREFETCH: u8 = 0xab; pub const SETFEATURES_EN_REST: u8 = 0xac; pub const SETFEATURES_4B_RW_LONG: u8 = 0xbb; pub const SETFEATURES_DIS_AAM: u8 = 0xc2; pub const SETFEATURES_EN_RPOD: u8 = 0xcc; pub const SETFEATURES_DIS_RI: u8 = 0xdd; pub const SETFEATURES_EN_SAME_M: u8 = 0xdd; pub const SETFEATURES_DIS_SI: u8 = 0xde;

#[repr(C)] pub struct hd_geometry { pub heads: u8, pub sectors: u8, pub cylinders: u16, pub start: ::core::ffi::c_ulong }
pub const HDIO_GETGEO: u32 = 0x0301; pub const HDIO_GET_UNMASKINTR: u32 = 0x0302; pub const HDIO_GET_MULTCOUNT: u32 = 0x0304; pub const HDIO_GET_QDMA: u32 = 0x0305; pub const HDIO_SET_XFER: u32 = 0x0306; pub const HDIO_OBSOLETE_IDENTITY: u32 = 0x0307; pub const HDIO_GET_KEEPSETTINGS: u32 = 0x0308; pub const HDIO_GET_32BIT: u32 = 0x0309; pub const HDIO_GET_NOWERR: u32 = 0x030a; pub const HDIO_GET_DMA: u32 = 0x030b; pub const HDIO_GET_NICE: u32 = 0x030c; pub const HDIO_GET_IDENTITY: u32 = 0x030d; pub const HDIO_GET_WCACHE: u32 = 0x030e; pub const HDIO_GET_ACOUSTIC: u32 = 0x030f; pub const HDIO_GET_ADDRESS: u32 = 0x0310;
pub const HDIO_GET_BUSSTATE: u32 = 0x031a; pub const HDIO_TRISTATE_HWIF: u32 = 0x031b; pub const HDIO_DRIVE_RESET: u32 = 0x031c; pub const HDIO_DRIVE_TASKFILE: u32 = 0x031d; pub const HDIO_DRIVE_TASK: u32 = 0x031e; pub const HDIO_DRIVE_CMD: u32 = 0x031f; pub const HDIO_DRIVE_CMD_AEB: u32 = HDIO_DRIVE_TASK;
pub const HDIO_SET_MULTCOUNT: u32 = 0x0321; pub const HDIO_SET_UNMASKINTR: u32 = 0x0322; pub const HDIO_SET_KEEPSETTINGS: u32 = 0x0323; pub const HDIO_SET_32BIT: u32 = 0x0324; pub const HDIO_SET_NOWERR: u32 = 0x0325; pub const HDIO_SET_DMA: u32 = 0x0326; pub const HDIO_SET_PIO_MODE: u32 = 0x0327; pub const HDIO_SCAN_HWIF: u32 = 0x0328; pub const HDIO_UNREGISTER_HWIF: u32 = 0x032a; pub const HDIO_SET_NICE: u32 = 0x0329; pub const HDIO_SET_WCACHE: u32 = 0x032b; pub const HDIO_SET_ACOUSTIC: u32 = 0x032c; pub const HDIO_SET_BUSSTATE: u32 = 0x032d; pub const HDIO_SET_QDMA: u32 = 0x032e; pub const HDIO_SET_ADDRESS: u32 = 0x032f;
pub const BUSSTATE_OFF: u32 = 0; pub const BUSSTATE_ON: u32 = 1; pub const BUSSTATE_TRISTATE: u32 = 2;

pub const IDE_NICE_DSC_OVERLAP: i32 = 0; pub const IDE_NICE_ATAPI_OVERLAP: i32 = 1; pub const IDE_NICE_1: i32 = 3; pub const IDE_NICE_0: i32 = 2; pub const IDE_NICE_2: i32 = 4;

#[repr(C)]
pub struct hd_driveid {
    pub config: u16, pub cyls: u16, pub reserved2: u16, pub heads: u16,
    pub track_bytes: u16, pub sector_bytes: u16, pub sectors: u16,
    pub vendor0: u16, pub vendor1: u16, pub vendor2: u16,
    pub serial_no: [u8; 20], pub buf_type: u16, pub buf_size: u16,
    pub ecc_bytes: u16, pub fw_rev: [u8; 8], pub model: [u8; 40],
    pub max_multsect: u8, pub vendor3: u8, pub dword_io: u16,
    pub vendor4: u8, pub capability: u8, pub reserved50: u16,
    pub vendor5: u8, pub tPIO: u8, pub vendor6: u8, pub tDMA: u8,
    pub field_valid: u16, pub cur_cyls: u16, pub cur_heads: u16,
    pub cur_sectors: u16, pub cur_capacity0: u16, pub cur_capacity1: u16,
    pub multsect: u8, pub multsect_valid: u8, pub lba_capacity: u32,
    pub dma_1word: u16, pub dma_mword: u16, pub eide_pio_modes: u16,
    pub eide_dma_min: u16, pub eide_dma_time: u16, pub eide_pio: u16,
    pub eide_pio_iordy: u16, pub words69_70: [u16; 2], pub words71_74: [u16; 4],
    pub queue_depth: u16, pub words76_79: [u16; 4], pub major_rev_num: u16,
    pub minor_rev_num: u16, pub command_set_1: u16, pub command_set_2: u16,
    pub cfsse: u16, pub cfs_enable_1: u16, pub cfs_enable_2: u16,
    pub csf_default: u16, pub dma_ultra: u16, pub trseuc: u16, pub trsEuc: u16,
    pub CurAPMvalues: u16, pub mprc: u16, pub hw_config: u16, pub acoustic: u16,
    pub msrqs: u16, pub sxfert: u16, pub sal: u16, pub spg: u32,
    pub lba_capacity_2: u64, pub words104_125: [u16; 22], pub last_lun: u16,
    pub word127: u16, pub dlf: u16, pub csfo: u16, pub words130_155: [u16; 26],
    pub word156: u16, pub words157_159: [u16; 3], pub cfa_power: u16,
    pub words161_175: [u16; 15], pub words176_205: [u16; 30],
    pub words206_254: [u16; 49], pub integrity_word: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
