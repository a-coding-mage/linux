/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Navi's PM4 definitions.  The C preprocessor expressions are represented
 * below as Rust constants and hygienic expression macros; all arithmetic is
 * intentionally performed with u32 wrapping semantics.
 */

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

pub const PACKET_TYPE0: u32 = 0;
pub const PACKET_TYPE1: u32 = 1;
pub const PACKET_TYPE2: u32 = 2;
pub const PACKET_TYPE3: u32 = 3;

#[macro_export]
macro_rules! CP_PACKET_GET_TYPE { ($h:expr) => { (($h as u32 >> 30) & 3) }; }
#[macro_export]
macro_rules! CP_PACKET_GET_COUNT { ($h:expr) => { (($h as u32 >> 16) & 0x3fff) }; }
#[macro_export]
macro_rules! CP_PACKET0_GET_REG { ($h:expr) => { ($h as u32 & 0xffff) }; }
#[macro_export]
macro_rules! CP_PACKET3_GET_OPCODE { ($h:expr) => { (($h as u32 >> 8) & 0xff) }; }
#[macro_export]
macro_rules! PACKET0 { ($reg:expr, $n:expr) => { (($reg as u32 & 0xffff) | (($n as u32 & 0x3fff) << 16)) }; }
pub const CP_PACKET2: u32 = 0x8000_0000;
pub const PACKET2_PAD_SHIFT: u32 = 0;
pub const PACKET2_PAD_MASK: u32 = 0x3fff_ffff;
#[macro_export]
macro_rules! PACKET2 { ($v:expr) => { $crate::CP_PACKET2 | REG_SET!(PACKET2_PAD, $v) }; }
#[macro_export]
macro_rules! PACKET3 { ($op:expr, $n:expr) => { ((($op as u32 & 0xff) << 8) | (($n as u32 & 0x3fff) << 16) | (3u32 << 30)) }; }
#[macro_export]
macro_rules! PACKET3_COMPUTE { ($op:expr, $n:expr) => { PACKET3!($op, $n) | (1u32 << 1) }; }

/* Packet 3 types and field encoders. */
pub const PACKET3_NOP: u32 = 0x10;
pub const PACKET3_SET_BASE: u32 = 0x11;
pub const CE_PARTITION_BASE: u32 = 3;
pub const PACKET3_CLEAR_STATE: u32 = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: u32 = 0x13;
pub const PACKET3_DISPATCH_DIRECT: u32 = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: u32 = 0x16;
pub const PACKET3_INDIRECT_BUFFER_END: u32 = 0x17;
pub const PACKET3_INDIRECT_BUFFER_CNST_END: u32 = 0x19;
pub const PACKET3_ATOMIC_GDS: u32 = 0x1d;
pub const PACKET3_ATOMIC_MEM: u32 = 0x1e;

#[macro_export]
macro_rules! nvd_field { ($x:expr, $mask:expr, $shift:expr) => { (($x as u32 & $mask) << $shift) }; }
#[macro_export]
macro_rules! PACKET3_ATOMIC_MEM__ATOMIC { ($x:expr) => { nvd_field!($x, 0x7f, 0) }; }
#[macro_export]
macro_rules! PACKET3_ATOMIC_MEM__COMMAND { ($x:expr) => { nvd_field!($x, 0xf, 8) }; }
#[macro_export]
macro_rules! PACKET3_ATOMIC_MEM__CACHE_POLICY { ($x:expr) => { nvd_field!($x, 3, 25) }; }
pub const PACKET3_ATOMIC_MEM__COMMAND__SINGLE_PASS_ATOMIC: u32 = 0;
pub const PACKET3_ATOMIC_MEM__COMMAND__LOOP_UNTIL_COMPARE_SATISFIED: u32 = 1;
pub const PACKET3_ATOMIC_MEM__COMMAND__WAIT_FOR_WRITE_CONFIRMATION: u32 = 2;
pub const PACKET3_ATOMIC_MEM__COMMAND__SEND_AND_CONTINUE: u32 = 3;
pub const PACKET3_ATOMIC_MEM__CACHE_POLICY__LRU: u32 = 0;
pub const PACKET3_ATOMIC_MEM__CACHE_POLICY__STREAM: u32 = 1;
pub const PACKET3_ATOMIC_MEM__CACHE_POLICY__NOA: u32 = 2;
pub const PACKET3_ATOMIC_MEM__CACHE_POLICY__BYPASS: u32 = 3;

/* The remaining declarations are direct opcode constants from the header. */
pub const PACKET3_OCCLUSION_QUERY:u32=0x1f; pub const PACKET3_SET_PREDICATION:u32=0x20; pub const PACKET3_REG_RMW:u32=0x21; pub const PACKET3_COND_EXEC:u32=0x22; pub const PACKET3_PRED_EXEC:u32=0x23; pub const PACKET3_DRAW_INDIRECT:u32=0x24; pub const PACKET3_DRAW_INDEX_INDIRECT:u32=0x25; pub const PACKET3_INDEX_BASE:u32=0x26; pub const PACKET3_DRAW_INDEX_2:u32=0x27; pub const PACKET3_CONTEXT_CONTROL:u32=0x28; pub const PACKET3_INDEX_TYPE:u32=0x2a; pub const PACKET3_DRAW_INDIRECT_MULTI:u32=0x2c; pub const PACKET3_DRAW_INDEX_AUTO:u32=0x2d; pub const PACKET3_NUM_INSTANCES:u32=0x2f; pub const PACKET3_DRAW_INDEX_MULTI_AUTO:u32=0x30; pub const PACKET3_WRITE_DATA:u32=0x37; pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI:u32=0x38; pub const PACKET3_MEM_SEMAPHORE:u32=0x39; pub const PACKET3_COPY_DW:u32=0x3b; pub const PACKET3_WAIT_REG_MEM:u32=0x3c; pub const PACKET3_INDIRECT_BUFFER:u32=0x3f; pub const PACKET3_COPY_DATA:u32=0x40; pub const PACKET3_CP_DMA:u32=0x41; pub const PACKET3_PFP_SYNC_ME:u32=0x42; pub const PACKET3_SURFACE_SYNC:u32=0x43; pub const PACKET3_ME_INITIALIZE:u32=0x44; pub const PACKET3_COND_WRITE:u32=0x45; pub const PACKET3_EVENT_WRITE:u32=0x46; pub const PACKET3_EVENT_WRITE_EOP:u32=0x47; pub const PACKET3_EVENT_WRITE_EOS:u32=0x48; pub const PACKET3_RELEASE_MEM:u32=0x49; pub const PACKET3_PREAMBLE_CNTL:u32=0x4a; pub const PACKET3_DMA_DATA:u32=0x50; pub const PACKET3_ACQUIRE_MEM:u32=0x58; pub const PACKET3_REWIND:u32=0x59; pub const PACKET3_INTERRUPT:u32=0x5a; pub const PACKET3_SET_CONFIG_REG:u32=0x68; pub const PACKET3_SET_CONTEXT_REG:u32=0x69; pub const PACKET3_SET_SH_REG:u32=0x76; pub const PACKET3_SET_UCONFIG_REG:u32=0x79; pub const PACKET3_FRAME_CONTROL:u32=0x90; pub const PACKET3_INVALIDATE_TLBS:u32=0x98; pub const PACKET3_SET_RESOURCES:u32=0xa0; pub const PACKET3_MAP_PROCESS:u32=0xa1; pub const PACKET3_MAP_QUEUES:u32=0xa2; pub const PACKET3_UNMAP_QUEUES:u32=0xa3; pub const PACKET3_QUERY_STATUS:u32=0xa4; pub const PACKET3_RUN_LIST:u32=0xa5; pub const PACKET3_MAP_PROCESS_VM:u32=0xa6; pub const PACKET3_RUN_CLEANER_SHADER:u32=0xd2; pub const PACKET3_SET_Q_PREEMPTION_MODE:u32=0xf0;

pub const PACKET3_SET_CONFIG_REG_START:u32=0x2000; pub const PACKET3_SET_CONFIG_REG_END:u32=0x2c00; pub const PACKET3_SET_CONTEXT_REG_START:u32=0xa000; pub const PACKET3_SET_CONTEXT_REG_END:u32=0xa400; pub const PACKET3_SET_SH_REG_START:u32=0x2c00; pub const PACKET3_SET_SH_REG_END:u32=0x3000; pub const PACKET3_SET_UCONFIG_REG_START:u32=0xc000; pub const PACKET3_SET_UCONFIG_REG_END:u32=0xc400;

/* Generic encoders preserve the corresponding C field macros for callers. */
#[macro_export] macro_rules! PACKET3_FIELD { ($x:expr,$mask:expr,$shift:expr) => { nvd_field!($x,$mask,$shift) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
