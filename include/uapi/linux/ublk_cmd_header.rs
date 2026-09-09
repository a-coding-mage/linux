/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from ublk_cmd.h. Kernel integer aliases and ioctl encoding
 * primitives are supplied by the including environment. */

pub const UBLK_CMD_GET_QUEUE_AFFINITY: u32 = 0x01;
pub const UBLK_CMD_GET_DEV_INFO: u32 = 0x02;
pub const UBLK_CMD_ADD_DEV: u32 = 0x04;
pub const UBLK_CMD_DEL_DEV: u32 = 0x05;
pub const UBLK_CMD_START_DEV: u32 = 0x06;
pub const UBLK_CMD_STOP_DEV: u32 = 0x07;
pub const UBLK_CMD_SET_PARAMS: u32 = 0x08;
pub const UBLK_CMD_GET_PARAMS: u32 = 0x09;
pub const UBLK_CMD_START_USER_RECOVERY: u32 = 0x10;
pub const UBLK_CMD_END_USER_RECOVERY: u32 = 0x11;
pub const UBLK_CMD_GET_DEV_INFO2: u32 = 0x12;

/* Any new ctrl command should encode by __IO*(). */
macro_rules! _IOR { ($t:expr, $n:expr, $ty:ty) => { crate::_IOR!($t, $n, $ty) }; }
macro_rules! _IOWR { ($t:expr, $n:expr, $ty:ty) => { crate::_IOWR!($t, $n, $ty) }; }
pub const UBLK_U_CMD_GET_QUEUE_AFFINITY: u64 = _IOR!('u', UBLK_CMD_GET_QUEUE_AFFINITY, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_GET_DEV_INFO: u64 = _IOR!('u', UBLK_CMD_GET_DEV_INFO, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_ADD_DEV: u64 = _IOWR!('u', UBLK_CMD_ADD_DEV, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_DEL_DEV: u64 = _IOWR!('u', UBLK_CMD_DEL_DEV, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_START_DEV: u64 = _IOWR!('u', UBLK_CMD_START_DEV, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_STOP_DEV: u64 = _IOWR!('u', UBLK_CMD_STOP_DEV, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_SET_PARAMS: u64 = _IOWR!('u', UBLK_CMD_SET_PARAMS, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_GET_PARAMS: u64 = _IOR!('u', UBLK_CMD_GET_PARAMS, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_START_USER_RECOVERY: u64 = _IOWR!('u', UBLK_CMD_START_USER_RECOVERY, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_END_USER_RECOVERY: u64 = _IOWR!('u', UBLK_CMD_END_USER_RECOVERY, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_GET_DEV_INFO2: u64 = _IOR!('u', UBLK_CMD_GET_DEV_INFO2, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_GET_FEATURES: u64 = _IOR!('u', 0x13, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_DEL_DEV_ASYNC: u64 = _IOR!('u', 0x14, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_UPDATE_SIZE: u64 = _IOWR!('u', 0x15, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_QUIESCE_DEV: u64 = _IOWR!('u', 0x16, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_TRY_STOP_DEV: u64 = _IOWR!('u', 0x17, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_REG_BUF: u64 = _IOWR!('u', 0x18, ublksrv_ctrl_cmd);
pub const UBLK_U_CMD_UNREG_BUF: u64 = _IOWR!('u', 0x19, ublksrv_ctrl_cmd);

#[repr(C)]
pub struct ublk_shmem_buf_reg { pub addr: u64, pub len: u64, pub flags: u32, pub reserved: u32 }
pub const UBLK_SHMEM_BUF_READ_ONLY: u32 = 1u32 << 0;
pub const UBLK_FEATURES_LEN: usize = 8;

pub const UBLK_IO_FETCH_REQ: u32 = 0x20;
pub const UBLK_IO_COMMIT_AND_FETCH_REQ: u32 = 0x21;
pub const UBLK_IO_NEED_GET_DATA: u32 = 0x22;
pub const UBLK_U_IO_FETCH_REQ: u64 = _IOWR!('u', UBLK_IO_FETCH_REQ, ublksrv_io_cmd);
pub const UBLK_U_IO_COMMIT_AND_FETCH_REQ: u64 = _IOWR!('u', UBLK_IO_COMMIT_AND_FETCH_REQ, ublksrv_io_cmd);
pub const UBLK_U_IO_NEED_GET_DATA: u64 = _IOWR!('u', UBLK_IO_NEED_GET_DATA, ublksrv_io_cmd);
pub const UBLK_U_IO_REGISTER_IO_BUF: u64 = _IOWR!('u', 0x23, ublksrv_io_cmd);
pub const UBLK_U_IO_UNREGISTER_IO_BUF: u64 = _IOWR!('u', 0x24, ublksrv_io_cmd);
pub const UBLK_U_IO_PREP_IO_CMDS: u64 = _IOWR!('u', 0x25, ublk_batch_io);
pub const UBLK_U_IO_COMMIT_IO_CMDS: u64 = _IOWR!('u', 0x26, ublk_batch_io);
pub const UBLK_U_IO_FETCH_IO_CMDS: u64 = _IOWR!('u', 0x27, ublk_batch_io);

pub const UBLK_IO_RES_OK: i32 = 0;
pub const UBLK_IO_RES_NEED_GET_DATA: i32 = 1;
pub const UBLK_IO_RES_ABORT: i32 = -19; // -ENODEV
pub const UBLKSRV_CMD_BUF_OFFSET: u32 = 0;
pub const UBLKSRV_IO_BUF_OFFSET: u32 = 0x80000000;
pub const UBLK_MAX_QUEUE_DEPTH: u32 = 4096;
pub const UBLK_IO_BUF_OFF: u32 = 0;
pub const UBLK_IO_BUF_BITS: u32 = 25;
pub const UBLK_IO_BUF_BITS_MASK: u64 = (1u64 << UBLK_IO_BUF_BITS) - 1;
pub const UBLK_TAG_OFF: u32 = UBLK_IO_BUF_BITS;
pub const UBLK_TAG_BITS: u32 = 16;
pub const UBLK_TAG_BITS_MASK: u64 = (1u64 << UBLK_TAG_BITS) - 1;
pub const UBLK_QID_OFF: u32 = UBLK_TAG_OFF + UBLK_TAG_BITS;
pub const UBLK_QID_BITS: u32 = 12;
pub const UBLK_QID_BITS_MASK: u64 = (1u64 << UBLK_QID_BITS) - 1;
pub const UBLK_MAX_NR_QUEUES: u32 = 1u32 << UBLK_QID_BITS;
pub const UBLKSRV_IO_BUF_TOTAL_BITS: u32 = UBLK_QID_OFF + UBLK_QID_BITS;
pub const UBLKSRV_IO_BUF_TOTAL_SIZE: u64 = 1u64 << UBLKSRV_IO_BUF_TOTAL_BITS;
pub const UBLK_INTEGRITY_FLAG_OFF: u32 = 62;
pub const UBLKSRV_IO_INTEGRITY_FLAG: u64 = 1u64 << UBLK_INTEGRITY_FLAG_OFF;

pub const UBLK_F_SUPPORT_ZERO_COPY: u64 = 1 << 0;
pub const UBLK_F_URING_CMD_COMP_IN_TASK: u64 = 1 << 1;
pub const UBLK_F_NEED_GET_DATA: u64 = 1 << 2;
pub const UBLK_F_USER_RECOVERY: u64 = 1 << 3;
pub const UBLK_F_USER_RECOVERY_REISSUE: u64 = 1 << 4;
pub const UBLK_F_UNPRIVILEGED_DEV: u64 = 1 << 5;
pub const UBLK_F_CMD_IOCTL_ENCODE: u64 = 1 << 6;
pub const UBLK_F_USER_COPY: u64 = 1 << 7;
pub const UBLK_F_ZONED: u64 = 1 << 8;
pub const UBLK_F_USER_RECOVERY_FAIL_IO: u64 = 1 << 9;
pub const UBLK_F_UPDATE_SIZE: u64 = 1 << 10;
pub const UBLK_F_AUTO_BUF_REG: u64 = 1 << 11;
pub const UBLK_F_QUIESCE: u64 = 1 << 12;
pub const UBLK_F_PER_IO_DAEMON: u64 = 1 << 13;
pub const UBLK_F_BUF_REG_OFF_DAEMON: u64 = 1 << 14;
pub const UBLK_F_BATCH_IO: u64 = 1 << 15;
pub const UBLK_F_INTEGRITY: u64 = 1 << 16;
pub const UBLK_F_SAFE_STOP_DEV: u64 = 1 << 17;
pub const UBLK_F_NO_AUTO_PART_SCAN: u64 = 1 << 18;
pub const UBLK_F_SHMEM_ZC: u64 = 1 << 19;
pub const UBLK_F_IO_DESC_SIZE: u64 = 1 << 20;
pub const UBLK_S_DEV_DEAD: u32 = 0;
pub const UBLK_S_DEV_LIVE: u32 = 1;
pub const UBLK_S_DEV_QUIESCED: u32 = 2;
pub const UBLK_S_DEV_FAIL_IO: u32 = 3;

#[repr(C)]
pub struct ublksrv_ctrl_cmd { pub dev_id: u32, pub queue_id: u16, pub len: u16, pub addr: u64, pub data: [u64; 1], pub dev_path_len: u16, pub pad: u16, pub reserved: u32 }
#[repr(C)]
pub struct ublksrv_ctrl_dev_info { pub nr_hw_queues: u16, pub queue_depth: u16, pub state: u16, pub io_desc_size: u16, pub max_io_buf_bytes: u32, pub dev_id: u32, pub ublksrv_pid: i32, pub pad1: u32, pub flags: u64, pub ublksrv_flags: u64, pub owner_uid: u32, pub owner_gid: u32, pub reserved1: u64, pub reserved2: u64 }

pub const UBLK_IO_OP_READ: u32 = 0; pub const UBLK_IO_OP_WRITE: u32 = 1; pub const UBLK_IO_OP_FLUSH: u32 = 2; pub const UBLK_IO_OP_DISCARD: u32 = 3; pub const UBLK_IO_OP_WRITE_SAME: u32 = 4; pub const UBLK_IO_OP_WRITE_ZEROES: u32 = 5; pub const UBLK_IO_OP_ZONE_OPEN: u32 = 10; pub const UBLK_IO_OP_ZONE_CLOSE: u32 = 11; pub const UBLK_IO_OP_ZONE_FINISH: u32 = 12; pub const UBLK_IO_OP_ZONE_APPEND: u32 = 13; pub const UBLK_IO_OP_ZONE_RESET_ALL: u32 = 14; pub const UBLK_IO_OP_ZONE_RESET: u32 = 15; pub const UBLK_IO_OP_REPORT_ZONES: u32 = 18;
pub const UBLK_IO_F_FAILFAST_DEV: u32 = 1 << 8; pub const UBLK_IO_F_FAILFAST_TRANSPORT: u32 = 1 << 9; pub const UBLK_IO_F_FAILFAST_DRIVER: u32 = 1 << 10; pub const UBLK_IO_F_META: u32 = 1 << 11; pub const UBLK_IO_F_FUA: u32 = 1 << 13; pub const UBLK_IO_F_NOUNMAP: u32 = 1 << 15; pub const UBLK_IO_F_SWAP: u32 = 1 << 16; pub const UBLK_IO_F_NEED_REG_BUF: u32 = 1 << 17; pub const UBLK_IO_F_INTEGRITY: u32 = 1 << 18; pub const UBLK_IO_F_SHMEM_ZC: u32 = 1 << 19;

#[repr(C)] pub union ublksrv_io_desc_n { pub nr_sectors: u32, pub nr_zones: u32 }
#[repr(C)] pub struct ublksrv_io_desc { pub op_flags: u32, pub n: ublksrv_io_desc_n, pub start_sector: u64, pub addr: u64 }
#[inline] pub unsafe fn ublksrv_get_op(iod: *const ublksrv_io_desc) -> u8 { ((*iod).op_flags & 0xff) as u8 }
#[inline] pub unsafe fn ublksrv_get_flags(iod: *const ublksrv_io_desc) -> u32 { (*iod).op_flags >> 8 }

pub const UBLK_AUTO_BUF_REG_FALLBACK: u32 = 1 << 0;
pub const UBLK_AUTO_BUF_REG_F_MASK: u32 = UBLK_AUTO_BUF_REG_FALLBACK;
#[repr(C)] pub struct ublk_auto_buf_reg { pub index: u16, pub flags: u8, pub reserved0: u8, pub reserved1: u32 }
#[inline] pub fn ublk_sqe_addr_to_auto_buf_reg(sqe_addr: u64) -> ublk_auto_buf_reg { ublk_auto_buf_reg { index: sqe_addr as u16, flags: (sqe_addr >> 16) as u8, reserved0: (sqe_addr >> 24) as u8, reserved1: (sqe_addr >> 32) as u32 } }
#[inline] pub unsafe fn ublk_auto_buf_reg_to_sqe_addr(buf: *const ublk_auto_buf_reg) -> u64 { (*buf).index as u64 | ((*buf).flags as u64) << 16 | ((*buf).reserved0 as u64) << 24 | ((*buf).reserved1 as u64) << 32 }

#[repr(C)] pub union ublksrv_io_cmd_u { pub addr: u64, pub zone_append_lba: u64 }
#[repr(C)] pub struct ublksrv_io_cmd { pub q_id: u16, pub tag: u16, pub result: i32, pub u: ublksrv_io_cmd_u }
#[repr(C)] pub struct ublk_elem_header { pub tag: u16, pub buf_index: u16, pub result: i32 }
#[repr(C)] pub struct ublk_batch_io { pub q_id: u16, pub flags: u16, pub nr_elem: u16, pub elem_bytes: u8, pub reserved: u8, pub reserved2: u64 }
pub const UBLK_BATCH_F_HAS_ZONE_LBA: u16 = 1 << 0; pub const UBLK_BATCH_F_HAS_BUF_ADDR: u16 = 1 << 1; pub const UBLK_BATCH_F_AUTO_BUF_REG_FALLBACK: u16 = 1 << 2;

#[repr(C)] pub struct ublk_param_basic { pub attrs: u32, pub logical_bs_shift: u8, pub physical_bs_shift: u8, pub io_opt_shift: u8, pub io_min_shift: u8, pub max_sectors: u32, pub chunk_sectors: u32, pub dev_sectors: u64, pub virt_boundary_mask: u64 }
pub const UBLK_ATTR_READ_ONLY: u32 = 1 << 0; pub const UBLK_ATTR_ROTATIONAL: u32 = 1 << 1; pub const UBLK_ATTR_VOLATILE_CACHE: u32 = 1 << 2; pub const UBLK_ATTR_FUA: u32 = 1 << 3;
#[repr(C)] pub struct ublk_param_discard { pub discard_alignment: u32, pub discard_granularity: u32, pub max_discard_sectors: u32, pub max_write_zeroes_sectors: u32, pub max_discard_segments: u16, pub reserved0: u16 }
#[repr(C)] pub struct ublk_param_devt { pub char_major: u32, pub char_minor: u32, pub disk_major: u32, pub disk_minor: u32 }
#[repr(C)] pub struct ublk_param_zoned { pub max_open_zones: u32, pub max_active_zones: u32, pub max_zone_append_sectors: u32, pub reserved: [u8; 20] }
#[repr(C)] pub struct ublk_param_dma_align { pub alignment: u32, pub pad: [u8; 4] }
pub const UBLK_MIN_SEGMENT_SIZE: u32 = 4096;
#[repr(C)] pub struct ublk_param_segment { pub seg_boundary_mask: u64, pub max_segment_size: u32, pub max_segments: u16, pub pad: [u8; 2] }
#[repr(C)] pub struct ublk_param_integrity { pub flags: u32, pub max_integrity_segments: u16, pub interval_exp: u8, pub metadata_size: u8, pub pi_offset: u8, pub csum_type: u8, pub tag_size: u8, pub pad: [u8; 5] }
#[repr(C)] pub struct ublk_params { pub len: u32, pub types: u32, pub basic: ublk_param_basic, pub discard: ublk_param_discard, pub devt: ublk_param_devt, pub zoned: ublk_param_zoned, pub dma: ublk_param_dma_align, pub seg: ublk_param_segment, pub integrity: ublk_param_integrity }
pub const UBLK_PARAM_TYPE_BASIC: u32 = 1 << 0; pub const UBLK_PARAM_TYPE_DISCARD: u32 = 1 << 1; pub const UBLK_PARAM_TYPE_DEVT: u32 = 1 << 2; pub const UBLK_PARAM_TYPE_ZONED: u32 = 1 << 3; pub const UBLK_PARAM_TYPE_DMA_ALIGN: u32 = 1 << 4; pub const UBLK_PARAM_TYPE_SEGMENT: u32 = 1 << 5; pub const UBLK_PARAM_TYPE_INTEGRITY: u32 = 1 << 6;

pub const UBLK_SHMEM_ZC_OFF_MASK: u64 = 0xffffffff; pub const UBLK_SHMEM_ZC_IDX_OFF: u32 = 32; pub const UBLK_SHMEM_ZC_IDX_MASK: u64 = 0xffff;
#[inline] pub fn ublk_shmem_zc_addr(index: u16, offset: u32) -> u64 { ((index as u64) << UBLK_SHMEM_ZC_IDX_OFF) | offset as u64 }
#[inline] pub fn ublk_shmem_zc_index(addr: u64) -> u16 { ((addr >> UBLK_SHMEM_ZC_IDX_OFF) & UBLK_SHMEM_ZC_IDX_MASK) as u16 }
#[inline] pub fn ublk_shmem_zc_offset(addr: u64) -> u32 { (addr & UBLK_SHMEM_ZC_OFF_MASK) as u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
