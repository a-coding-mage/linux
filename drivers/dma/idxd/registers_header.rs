/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* Linux header dependencies are supplied by the surrounding translation unit. */

pub const PCI_DEVICE_ID_INTEL_IAA_PTL: u32 = 0xb02d;
pub const PCI_DEVICE_ID_INTEL_IAA_WCL: u32 = 0xfd2d;
pub const DEVICE_VERSION_1: u32 = 0x100;
pub const DEVICE_VERSION_2: u32 = 0x200;
pub const DEVICE_VERSION_3: u32 = 0x300;
pub const IDXD_MMIO_BAR: u32 = 0;
pub const IDXD_WQ_BAR: u32 = 2;
pub const IDXD_PORTAL_SIZE: usize = 4096; /* PAGE_SIZE */
pub const IDXD_VER_OFFSET: u32 = 0x00;
pub const IDXD_VER_MAJOR_MASK: u32 = 0xf0;
pub const IDXD_VER_MINOR_MASK: u32 = 0x0f;
#[inline] pub const fn GET_IDXD_VER_MAJOR(x: u32) -> u32 { (x & IDXD_VER_MAJOR_MASK) >> 4 }
#[inline] pub const fn GET_IDXD_VER_MINOR(x: u32) -> u32 { x & IDXD_VER_MINOR_MASK }

/* C bit-field register overlays are represented by their native integer storage. */
#[repr(C)] #[derive(Copy, Clone)] pub union gen_cap_reg { pub bits: u64 }
pub const IDXD_GENCAP_OFFSET: u32 = 0x10;
#[repr(C)] #[derive(Copy, Clone)] pub union wq_cap_reg { pub bits: u64 }
pub const IDXD_WQCAP_OFFSET: u32 = 0x20; pub const IDXD_WQCFG_MIN: u32 = 5;
#[repr(C)] #[derive(Copy, Clone)] pub union group_cap_reg { pub bits: u64 }
pub const IDXD_GRPCAP_OFFSET: u32 = 0x30;
#[repr(C)] #[derive(Copy, Clone)] pub union engine_cap_reg { pub bits: u64 }
pub const IDXD_ENGCAP_OFFSET: u32 = 0x38;
pub const IDXD_OPCAP_NOOP: u64 = 0x0001; pub const IDXD_OPCAP_BATCH: u64 = 0x0002; pub const IDXD_OPCAP_MEMMOVE: u64 = 0x0008;
#[repr(C)] pub struct opcap { pub bits: [u64; 4] }
pub const IDXD_MAX_OPCAP_BITS: u32 = 256; pub const IDXD_OPCAP_OFFSET: u32 = 0x40; pub const IDXD_TABLE_OFFSET: u32 = 0x60;
#[repr(C)] #[derive(Copy, Clone)] pub union offsets_reg { pub bits: [u64; 2] }
pub const IDXD_TABLE_MULT: u32 = 0x100; pub const IDXD_GENCFG_OFFSET: u32 = 0x80;
#[repr(C)] #[derive(Copy, Clone)] pub union gencfg_reg { pub bits: u32 }
pub const IDXD_GENCTRL_OFFSET: u32 = 0x88;
#[repr(C)] #[derive(Copy, Clone)] pub union genctrl_reg { pub bits: u32 }
pub const IDXD_GENSTATS_OFFSET: u32 = 0x90;
#[repr(C)] #[derive(Copy, Clone)] pub union gensts_reg { pub bits: u32 }

#[repr(u32)] pub enum idxd_device_status_state { IDXD_DEVICE_STATE_DISABLED=0, IDXD_DEVICE_STATE_ENABLED, IDXD_DEVICE_STATE_DRAIN, IDXD_DEVICE_STATE_HALT }
#[repr(u32)] pub enum idxd_device_reset_type { IDXD_DEVICE_RESET_SOFTWARE=0, IDXD_DEVICE_RESET_FLR, IDXD_DEVICE_RESET_WARM, IDXD_DEVICE_RESET_COLD }
pub const IDXD_INTCAUSE_OFFSET:u32=0x98; pub const IDXD_INTC_ERR:u32=1; pub const IDXD_INTC_CMD:u32=2; pub const IDXD_INTC_OCCUPY:u32=4; pub const IDXD_INTC_PERFMON_OVFL:u32=8; pub const IDXD_INTC_HALT_STATE:u32=0x10; pub const IDXD_INTC_EVL:u32=0x20; pub const IDXD_INTC_INT_HANDLE_REVOKED:u32=0x80000000;
pub const IDXD_CMD_OFFSET:u32=0xa0;
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_command_reg { pub bits:u32 }
#[repr(u32)] pub enum idxd_cmd { IDXD_CMD_ENABLE_DEVICE=1, IDXD_CMD_DISABLE_DEVICE, IDXD_CMD_DRAIN_ALL, IDXD_CMD_ABORT_ALL, IDXD_CMD_RESET_DEVICE, IDXD_CMD_ENABLE_WQ, IDXD_CMD_DISABLE_WQ, IDXD_CMD_DRAIN_WQ, IDXD_CMD_ABORT_WQ, IDXD_CMD_RESET_WQ, IDXD_CMD_DRAIN_PASID, IDXD_CMD_ABORT_PASID, IDXD_CMD_REQUEST_INT_HANDLE, IDXD_CMD_RELEASE_INT_HANDLE }
pub const CMD_INT_HANDLE_IMS:u32=0x10000; pub const IDXD_CMDSTS_OFFSET:u32=0xa8;
#[repr(C)] #[derive(Copy,Clone)] pub union cmdsts_reg { pub bits:u32 }
pub const IDXD_CMDSTS_ACTIVE:u32=0x80000000; pub const IDXD_CMDSTS_ERR_MASK:u32=0xff; pub const IDXD_CMDSTS_RES_SHIFT:u32=8;
#[repr(u32)] pub enum idxd_cmdsts_err { IDXD_CMDSTS_SUCCESS=0, IDXD_CMDSTS_INVAL_CMD, IDXD_CMDSTS_INVAL_WQIDX, IDXD_CMDSTS_HW_ERR, IDXD_CMDSTS_ERR_DEV_ENABLED=0x10, IDXD_CMDSTS_ERR_CONFIG, IDXD_CMDSTS_ERR_BUSMASTER_EN, IDXD_CMDSTS_ERR_PASID_INVAL, IDXD_CMDSTS_ERR_WQ_SIZE_ERANGE, IDXD_CMDSTS_ERR_GRP_CONFIG, IDXD_CMDSTS_ERR_GRP_CONFIG2, IDXD_CMDSTS_ERR_GRP_CONFIG3, IDXD_CMDSTS_ERR_GRP_CONFIG4, IDXD_CMDSTS_ERR_DEV_NOTEN=0x20, IDXD_CMDSTS_ERR_WQ_ENABLED, IDXD_CMDSTS_ERR_WQ_SIZE, IDXD_CMDSTS_ERR_WQ_PRIOR, IDXD_CMDSTS_ERR_WQ_MODE, IDXD_CMDSTS_ERR_BOF_EN, IDXD_CMDSTS_ERR_PASID_EN, IDXD_CMDSTS_ERR_MAX_BATCH_SIZE, IDXD_CMDSTS_ERR_MAX_XFER_SIZE, IDXD_CMDSTS_ERR_DIS_DEV_EN=0x31, IDXD_CMDSTS_ERR_DEV_NOT_EN, IDXD_CMDSTS_ERR_INVAL_INT_IDX=0x41, IDXD_CMDSTS_ERR_NO_HANDLE }
pub const IDXD_CMDCAP_OFFSET:u32=0xb0; pub const IDXD_SWERR_OFFSET:u32=0xc0; pub const IDXD_SWERR_VALID:u32=1; pub const IDXD_SWERR_OVERFLOW:u32=2; pub const IDXD_SWERR_ACK:u32=3;
#[repr(C)] #[derive(Copy,Clone)] pub union sw_err_reg { pub bits:[u64;4] }
#[repr(C)] #[derive(Copy,Clone)] pub union iaa_cap_reg { pub bits:u64 }
pub const IDXD_IAACAP_OFFSET:u32=0x180; pub const IDXD_EVLCFG_OFFSET:u32=0xe0;
#[repr(C)] #[derive(Copy,Clone)] pub union evlcfg_reg { pub bits:[u64;2] }
pub const IDXD_EVL_SIZE_MIN:u32=0x0040; pub const IDXD_EVL_SIZE_MAX:u32=0xffff;
#[repr(C)] #[derive(Copy,Clone)] pub union msix_perm { pub bits:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union group_flags { pub bits:u64 }
#[repr(C)] pub struct grpcfg { pub wqs:[u64;4], pub engines:u64, pub flags:group_flags }
#[repr(C)] #[derive(Copy,Clone)] pub union wqcfg { pub bits:[u32;16] }
pub const WQCFG_PASID_IDX:usize=2; pub const WQCFG_PRIVL_IDX:usize=2; pub const WQCFG_OCCUP_IDX:usize=6; pub const WQCFG_OCCUP_MASK:u32=0xffff;
pub const GRPCFG_SIZE:usize=64; pub const GRPWQCFG_STRIDES:usize=4;
/* Offset macros retain C's field access and arithmetic semantics. */
#[macro_export] macro_rules! WQCFG_OFFSET { ($idxd_dev:expr, $n:expr, $ofs:expr) => { ($idxd_dev).wqcfg_offset + ($n) * ($idxd_dev).wqcfg_size + core::mem::size_of::<u32>() * ($ofs) }; }
#[macro_export] macro_rules! WQCFG_STRIDES { ($idxd_dev:expr) => { ($idxd_dev).wqcfg_size / core::mem::size_of::<u32>() }; }
#[macro_export] macro_rules! GRPWQCFG_OFFSET { ($idxd_dev:expr, $n:expr, $ofs:expr) => { ($idxd_dev).grpcfg_offset + ($n) * GRPCFG_SIZE + core::mem::size_of::<u64>() * ($ofs) }; }
#[macro_export] macro_rules! GRPENGCFG_OFFSET { ($idxd_dev:expr, $n:expr) => { ($idxd_dev).grpcfg_offset + ($n) * GRPCFG_SIZE + 32 }; }
#[macro_export] macro_rules! GRPFLGCFG_OFFSET { ($idxd_dev:expr, $n:expr) => { ($idxd_dev).grpcfg_offset + ($n) * GRPCFG_SIZE + 40 }; }
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_perfcap { pub bits:u64 }
pub const IDXD_PERFCAP_OFFSET:u32=0; pub const IDXD_EVNTCAP_OFFSET:u32=0x80;
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_evntcap { pub bits:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_event { pub val:u32 }
pub const IDXD_CNTRCAP_OFFSET:u32=0x800;
#[repr(C)] pub struct idxd_cntrcap { pub val:u32, pub events:[idxd_event;0] }
pub const IDXD_PERFRST_OFFSET:u32=0x10;
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_perfrst { pub val:u32 }
pub const IDXD_OVFSTATUS_OFFSET:u32=0x30; pub const IDXD_PERFFRZ_OFFSET:u32=0x20; pub const IDXD_CNTRCFG_OFFSET:u32=0x100;
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_cntrcfg { pub val:u64 }
pub const IDXD_FLTCFG_OFFSET:u32=0x300; pub const IDXD_CNTRDATA_OFFSET:u32=0x200;
#[repr(C)] #[derive(Copy,Clone)] pub union idxd_cntrdata { pub val:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union event_cfg { pub val:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union filter_cfg { pub val:u64 }
pub const IDXD_EVLSTATUS_OFFSET:u32=0xf0;
#[repr(C)] #[derive(Copy,Clone)] pub union evl_status_reg { pub bits:u64 }
pub const IDXD_DSACAP0_OFFSET:u32=0x180; #[repr(C)] #[derive(Copy,Clone)] pub union dsacap0_reg { pub bits:u64 }
pub const IDXD_DSACAP1_OFFSET:u32=0x188; #[repr(C)] #[derive(Copy,Clone)] pub union dsacap1_reg { pub bits:u64 }
pub const IDXD_DSACAP2_OFFSET:u32=0x190; #[repr(C)] #[derive(Copy,Clone)] pub union dsacap2_reg { pub bits:u64 }
pub const IDXD_MAX_BATCH_IDENT:u32=256;
#[repr(C)] pub struct __evl_entry { pub bits:u64, pub batch_idx:u16, pub rsvd3:u16, pub invalid_flags:u32, pub fault_addr:u64, pub rsvd5:u64 }
#[repr(C)] pub struct dsa_evl_entry { pub e:__evl_entry, pub cr:dsa_completion_record }
#[repr(C)] pub struct iax_evl_entry { pub e:__evl_entry, pub rsvd:[u64;4], pub cr:iax_completion_record }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
