/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of genwqe_card.h. Linux ioctl/type dependencies are external. */

pub const GENWQE_DEVNAME: &str = "genwqe";
pub const GENWQE_TYPE_ALTERA_230: u32 = 0x00;
pub const GENWQE_TYPE_ALTERA_530: u32 = 0x01;
pub const GENWQE_TYPE_ALTERA_A4: u32 = 0x02;
pub const GENWQE_TYPE_ALTERA_A7: u32 = 0x03;
#[inline] pub const fn GENWQE_UID_OFFS(uid: u64) -> u64 { uid << 24 }
pub const GENWQE_SLU_OFFS: u64 = GENWQE_UID_OFFS(0);
pub const GENWQE_HSU_OFFS: u64 = GENWQE_UID_OFFS(1);
pub const GENWQE_APP_OFFS: u64 = GENWQE_UID_OFFS(2);
pub const GENWQE_MAX_UNITS: u32 = 3;

pub const IO_EXTENDED_ERROR_POINTER:u64=0x48; pub const IO_ERROR_INJECT_SELECTOR:u64=0x60;
pub const IO_EXTENDED_DIAG_SELECTOR:u64=0x70; pub const IO_EXTENDED_DIAG_READ_MBX:u64=0x78;
#[inline] pub const fn IO_EXTENDED_DIAG_MAP(ring:u64)->u64 {0x500|(ring<<3)}
#[inline] pub const fn GENWQE_EXTENDED_DIAG_SELECTOR(ring:u64,trace:u64)->u64 {(ring<<8)|trace}

pub const IO_SLU_UNITCFG:u64=0; pub const IO_SLU_UNITCFG_TYPE_MASK:u64=0x000000000ff00000;
pub const IO_SLU_FIR:u64=8; pub const IO_SLU_FIR_CLR:u64=0x10; pub const IO_SLU_FEC:u64=0x18;
pub const IO_SLU_ERR_ACT_MASK:u64=0x20; pub const IO_SLU_ERR_ATTN_MASK:u64=0x28;
pub const IO_SLU_FIRX1_ACT_MASK:u64=0x30; pub const IO_SLU_FIRX0_ACT_MASK:u64=0x38;
pub const IO_SLU_SEC_LEM_DEBUG_OVR:u64=0x40; pub const IO_SLU_EXTENDED_ERR_PTR:u64=0x48; pub const IO_SLU_COMMON_CONFIG:u64=0x60;
pub const IO_SLU_FLASH_FIR:u64=0x108; pub const IO_SLU_SLC_FIR:u64=0x110; pub const IO_SLU_RIU_TRAP:u64=0x280;
pub const IO_SLU_FLASH_FEC:u64=0x308; pub const IO_SLU_SLC_FEC:u64=0x310;
pub const IO_SLC_QUEUE_SEGMENT:u64=0x10000; pub const IO_SLC_VF_QUEUE_SEGMENT:u64=0x50000;
pub const IO_SLC_QUEUE_OFFSET:u64=0x10008; pub const IO_SLC_VF_QUEUE_OFFSET:u64=0x50008;
pub const IO_SLC_QUEUE_CONFIG:u64=0x10010; pub const IO_SLC_VF_QUEUE_CONFIG:u64=0x50010;
pub const IO_SLC_APPJOB_TIMEOUT:u64=0x10018; pub const IO_SLC_VF_APPJOB_TIMEOUT:u64=0x50018;
pub const TIMEOUT_250MS:u64=0xf; pub const HEARTBEAT_DISABLE:u64=0xff00;
pub const IO_SLC_QUEUE_INITSQN:u64=0x10020; pub const IO_SLC_VF_QUEUE_INITSQN:u64=0x50020;
pub const IO_SLC_QUEUE_WRAP:u64=0x10028; pub const IO_SLC_VF_QUEUE_WRAP:u64=0x50028;
pub const IO_SLC_QUEUE_STATUS:u64=0x10100; pub const IO_SLC_VF_QUEUE_STATUS:u64=0x50100;
pub const IO_SLC_QUEUE_WTIME:u64=0x10030; pub const IO_SLC_VF_QUEUE_WTIME:u64=0x50030;
pub const IO_SLC_QUEUE_ERRCNTS:u64=0x10038; pub const IO_SLC_VF_QUEUE_ERRCNTS:u64=0x50038;
pub const IO_SLC_QUEUE_LRW:u64=0x10040; pub const IO_SLC_VF_QUEUE_LRW:u64=0x50040;
pub const IO_SLC_FREE_RUNNING_TIMER:u64=0x10108; pub const IO_SLC_VF_FREE_RUNNING_TIMER:u64=0x50108;
pub const IO_PF_SLC_VIRTUAL_REGION:u64=0x50000; pub const IO_PF_SLC_VIRTUAL_WINDOW:u64=0x60000;
#[inline] pub const fn IO_PF_SLC_JOBPEND(n:u64)->u64 {0x61000+8*n} #[inline] pub const fn IO_SLC_JOBPEND(n:u64)->u64 {IO_PF_SLC_JOBPEND(n)}
#[inline] pub const fn IO_SLU_SLC_PARSE_TRAP(n:u64)->u64 {0x11000+8*n} #[inline] pub const fn IO_SLU_SLC_DISP_TRAP(n:u64)->u64 {0x11200+8*n}
pub const IO_SLC_CFGREG_GFIR:u64=0x20000; pub const GFIR_ERR_TRIGGER:u64=0xffff; pub const IO_SLC_CFGREG_SOFTRESET:u64=0x20018;
pub const IO_SLC_MISC_DEBUG:u64=0x20060; pub const IO_SLC_MISC_DEBUG_CLR:u64=0x20068; pub const IO_SLC_MISC_DEBUG_SET:u64=0x20070;
pub const IO_SLU_TEMPERATURE_SENSOR:u64=0x30000; pub const IO_SLU_TEMPERATURE_CONFIG:u64=0x30008;
pub const IO_SLU_VOLTAGE_CONTROL:u64=0x30080; pub const IO_SLU_VOLTAGE_NOMINAL:u64=0; pub const IO_SLU_VOLTAGE_DOWN5:u64=6; pub const IO_SLU_VOLTAGE_UP5:u64=7;
pub const IO_SLU_LEDCONTROL:u64=0x30100; pub const IO_SLU_FLASH_DIRECTACCESS:u64=0x40010; pub const IO_SLU_FLASH_DIRECTACCESS2:u64=0x40020; pub const IO_SLU_FLASH_CMDINTF:u64=0x40030; pub const IO_SLU_BITSTREAM:u64=0x40040;
pub const IO_HSU_ERR_BEHAVIOR:u64=0x01001010; pub const IO_SLC2_SQB_TRAP:u64=0x62000; pub const IO_SLC2_QUEUE_MANAGER_TRAP:u64=0x62008; pub const IO_SLC2_FLS_MASTER_TRAP:u64=0x62010;

pub const IO_HSU_UNITCFG:u64=0x1000000; pub const IO_HSU_FIR:u64=0x1000008; pub const IO_HSU_FIR_CLR:u64=0x1000010; pub const IO_HSU_FEC:u64=0x1000018; pub const IO_HSU_ERR_ACT_MASK:u64=0x1000020; pub const IO_HSU_ERR_ATTN_MASK:u64=0x1000028; pub const IO_HSU_FIRX1_ACT_MASK:u64=0x1000030; pub const IO_HSU_FIRX0_ACT_MASK:u64=0x1000038; pub const IO_HSU_SEC_LEM_DEBUG_OVR:u64=0x1000040; pub const IO_HSU_EXTENDED_ERR_PTR:u64=0x1000048; pub const IO_HSU_COMMON_CONFIG:u64=0x1000060;
pub const IO_APP_UNITCFG:u64=0x2000000; pub const IO_APP_FIR:u64=0x2000008; pub const IO_APP_FIR_CLR:u64=0x2000010; pub const IO_APP_FEC:u64=0x2000018; pub const IO_APP_ERR_ACT_MASK:u64=0x2000020; pub const IO_APP_ERR_ATTN_MASK:u64=0x2000028; pub const IO_APP_FIRX1_ACT_MASK:u64=0x2000030; pub const IO_APP_FIRX0_ACT_MASK:u64=0x2000038; pub const IO_APP_SEC_LEM_DEBUG_OVR:u64=0x2000040; pub const IO_APP_EXTENDED_ERR_PTR:u64=0x2000048; pub const IO_APP_COMMON_CONFIG:u64=0x2000060;
pub const IO_APP_DEBUG_REG_01:u64=0x2010000; pub const IO_APP_DEBUG_REG_02:u64=0x2010008; pub const IO_APP_DEBUG_REG_03:u64=0x2010010; pub const IO_APP_DEBUG_REG_04:u64=0x2010018; pub const IO_APP_DEBUG_REG_05:u64=0x2010020; pub const IO_APP_DEBUG_REG_06:u64=0x2010028; pub const IO_APP_DEBUG_REG_07:u64=0x2010030; pub const IO_APP_DEBUG_REG_08:u64=0x2010038; pub const IO_APP_DEBUG_REG_09:u64=0x2010040; pub const IO_APP_DEBUG_REG_10:u64=0x2010048; pub const IO_APP_DEBUG_REG_11:u64=0x2010050; pub const IO_APP_DEBUG_REG_12:u64=0x2010058; pub const IO_APP_DEBUG_REG_13:u64=0x2010060; pub const IO_APP_DEBUG_REG_14:u64=0x2010068; pub const IO_APP_DEBUG_REG_15:u64=0x2010070; pub const IO_APP_DEBUG_REG_16:u64=0x2010078; pub const IO_APP_DEBUG_REG_17:u64=0x2010080; pub const IO_APP_DEBUG_REG_18:u64=0x2010088;

#[repr(C)] pub struct genwqe_reg_io { pub num:u64, pub val64:u64 }
pub const IO_ILLEGAL_VALUE:u64=0xffffffffffffffff;
pub const DDCB_ACFUNC_SLU:u8=0; pub const DDCB_ACFUNC_APP:u8=1;
pub const DDCB_RETC_IDLE:u16=0; pub const DDCB_RETC_PENDING:u16=0x101; pub const DDCB_RETC_COMPLETE:u16=0x102; pub const DDCB_RETC_FAULT:u16=0x104; pub const DDCB_RETC_ERROR:u16=0x108; pub const DDCB_RETC_FORCED_ERROR:u16=0x1ff; pub const DDCB_RETC_UNEXEC:u16=0x110; pub const DDCB_RETC_TERM:u16=0x120; pub const DDCB_RETC_RES0:u16=0x140; pub const DDCB_RETC_RES1:u16=0x180;
pub const DDCB_OPT_ECHO_FORCE_NO:u16=0; pub const DDCB_OPT_ECHO_FORCE_102:u16=1; pub const DDCB_OPT_ECHO_FORCE_104:u16=2; pub const DDCB_OPT_ECHO_FORCE_108:u16=3; pub const DDCB_OPT_ECHO_FORCE_110:u16=4; pub const DDCB_OPT_ECHO_FORCE_120:u16=5; pub const DDCB_OPT_ECHO_FORCE_140:u16=6; pub const DDCB_OPT_ECHO_FORCE_180:u16=7; pub const DDCB_OPT_ECHO_COPY_NONE:u16=0; pub const DDCB_OPT_ECHO_COPY_ALL:u16=1<<5;
pub const SLCMD_ECHO_SYNC:u8=0; pub const SLCMD_MOVE_FLASH:u8=6; pub const SLCMD_MOVE_FLASH_FLAGS_MODE:u8=3; pub const SLCMD_MOVE_FLASH_FLAGS_DLOAD:u8=0; pub const SLCMD_MOVE_FLASH_FLAGS_EMUL:u8=1; pub const SLCMD_MOVE_FLASH_FLAGS_UPLOAD:u8=2; pub const SLCMD_MOVE_FLASH_FLAGS_VERIFY:u8=3; pub const SLCMD_MOVE_FLASH_FLAG_NOTAP:u8=1<<2; pub const SLCMD_MOVE_FLASH_FLAG_POLL:u8=1<<3; pub const SLCMD_MOVE_FLASH_FLAG_PARTITION:u8=1<<4; pub const SLCMD_MOVE_FLASH_FLAG_ERASE:u8=1<<5;
#[repr(u32)] pub enum genwqe_card_state { GENWQE_CARD_UNUSED=0, GENWQE_CARD_USED=1, GENWQE_CARD_FATAL_ERROR=2, GENWQE_CARD_RELOAD_BITSTREAM=3, GENWQE_CARD_STATE_MAX }
#[repr(C)] pub struct genwqe_bitstream { pub data_addr:u64,pub size:u32,pub crc:u32,pub target_addr:u64,pub partition:u32,pub uid:u32,pub slu_id:u64,pub app_id:u64,pub retc:u16,pub attn:u16,pub progress:u32 }
pub const DDCB_LENGTH:usize=256; pub const DDCB_ASIV_LENGTH:usize=104; pub const DDCB_ASIV_LENGTH_ATS:usize=96; pub const DDCB_ASV_LENGTH:usize=64; pub const DDCB_FIXUPS:usize=12;
#[repr(C)] pub struct genwqe_debug_data { pub driver_version:[u8;64],pub slu_unitcfg:u64,pub app_unitcfg:u64,pub ddcb_before:[u8;DDCB_LENGTH],pub ddcb_prev:[u8;DDCB_LENGTH],pub ddcb_finished:[u8;DDCB_LENGTH] }
pub const ATS_TYPE_DATA:u64=0; pub const ATS_TYPE_FLAT_RD:u64=4; pub const ATS_TYPE_FLAT_RDWR:u64=5; pub const ATS_TYPE_SGL_RD:u64=6; pub const ATS_TYPE_SGL_RDWR:u64=7;
#[macro_export] macro_rules! ATS_SET_FLAGS { ($s:ty,$f:tt,$flags:expr) => { (($flags & 0xf) << (44 - (4 * (core::mem::offset_of!($s,$f) / 8)))) }; }
#[macro_export] macro_rules! ATS_GET_FLAGS { ($ats:expr,$byte_offs:expr) => { (($ats >> (44 - (4 * ($byte_offs / 8)))) & 0xf) }; }
#[repr(C)] pub union genwqe_ddcb_cmd_asiv { pub ats_data: genwqe_ddcb_cmd_ats, pub __asiv:[u8;DDCB_ASIV_LENGTH] }
#[repr(C)] pub struct genwqe_ddcb_cmd_ats { pub ats:u64,pub asiv:[u8;DDCB_ASIV_LENGTH_ATS] }
#[repr(C)] pub struct genwqe_ddcb_cmd { pub next_addr:u64,pub flags:u64,pub acfunc:u8,pub cmd:u8,pub asiv_length:u8,pub asv_length:u8,pub cmdopts:u16,pub retc:u16,pub attn:u16,pub vcrc:u16,pub progress:u32,pub deque_ts:u64,pub cmplt_ts:u64,pub disp_ts:u64,pub ddata_addr:u64,pub asv:[u8;DDCB_ASV_LENGTH], pub asiv:genwqe_ddcb_cmd_asiv }
#[repr(C)] pub struct genwqe_mem { pub addr:u64,pub size:u64,pub direction:u64,pub flags:u64 }
pub const GENWQE_IOC_CODE:u8=0xa5;
/* ioctl encodings depend on the external Linux _IOR/_IOW/_IOWR definitions. */
pub const GENWQE_READ_REG64:u32=30; pub const GENWQE_WRITE_REG64:u32=31;
pub const GENWQE_READ_REG32:u32=32; pub const GENWQE_WRITE_REG32:u32=33;
pub const GENWQE_READ_REG16:u32=34; pub const GENWQE_WRITE_REG16:u32=35;
pub const GENWQE_GET_CARD_STATE:u32=36;
pub const GENWQE_EXECUTE_DDCB:u32=50; pub const GENWQE_EXECUTE_RAW_DDCB:u32=51; pub const GENWQE_SLU_UPDATE:u32=80; pub const GENWQE_SLU_READ:u32=81; pub const GENWQE_PIN_MEM:u32=40; pub const GENWQE_UNPIN_MEM:u32=41;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
