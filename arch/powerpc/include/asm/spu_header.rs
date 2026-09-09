/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SPU core / file system interface and HW structures (Rust translation). */

// The original declarations are kernel-only and depend on Linux/PowerPC types.

pub const LS_SIZE: usize = 256 * 1024;
pub const LS_ADDR_MASK: usize = LS_SIZE - 1;

pub const MFC_PUT_CMD: u32 = 0x20; pub const MFC_PUTS_CMD: u32 = 0x28;
pub const MFC_PUTR_CMD: u32 = 0x30; pub const MFC_PUTF_CMD: u32 = 0x22;
pub const MFC_PUTB_CMD: u32 = 0x21; pub const MFC_PUTFS_CMD: u32 = 0x2a;
pub const MFC_PUTBS_CMD: u32 = 0x29; pub const MFC_PUTRF_CMD: u32 = 0x32;
pub const MFC_PUTRB_CMD: u32 = 0x31; pub const MFC_PUTL_CMD: u32 = 0x24;
pub const MFC_PUTRL_CMD: u32 = 0x34; pub const MFC_PUTLF_CMD: u32 = 0x26;
pub const MFC_PUTLB_CMD: u32 = 0x25; pub const MFC_PUTRLF_CMD: u32 = 0x36;
pub const MFC_PUTRLB_CMD: u32 = 0x35;
pub const MFC_GET_CMD: u32 = 0x40; pub const MFC_GETS_CMD: u32 = 0x48;
pub const MFC_GETF_CMD: u32 = 0x42; pub const MFC_GETB_CMD: u32 = 0x41;
pub const MFC_GETFS_CMD: u32 = 0x4a; pub const MFC_GETBS_CMD: u32 = 0x49;
pub const MFC_GETL_CMD: u32 = 0x44; pub const MFC_GETLF_CMD: u32 = 0x46;
pub const MFC_GETLB_CMD: u32 = 0x45;
pub const MFC_SDCRT_CMD: u32 = 0x80; pub const MFC_SDCRTST_CMD: u32 = 0x81;
pub const MFC_SDCRZ_CMD: u32 = 0x89; pub const MFC_SDCRS_CMD: u32 = 0x8d;
pub const MFC_SDCRF_CMD: u32 = 0x8f; pub const MFC_GETLLAR_CMD: u32 = 0xd0;
pub const MFC_PUTLLC_CMD: u32 = 0xb4; pub const MFC_PUTLLUC_CMD: u32 = 0xb0;
pub const MFC_PUTQLLUC_CMD: u32 = 0xb8; pub const MFC_SNDSIG_CMD: u32 = 0xa0;
pub const MFC_SNDSIGB_CMD: u32 = 0xa1; pub const MFC_SNDSIGF_CMD: u32 = 0xa2;
pub const MFC_BARRIER_CMD: u32 = 0xc0; pub const MFC_EIEIO_CMD: u32 = 0xc8;
pub const MFC_SYNC_CMD: u32 = 0xcc;
pub const MFC_MIN_DMA_SIZE_SHIFT: u32 = 4; pub const MFC_MAX_DMA_SIZE_SHIFT: u32 = 14;
pub const MFC_MIN_DMA_SIZE: u32 = 1 << MFC_MIN_DMA_SIZE_SHIFT;
pub const MFC_MAX_DMA_SIZE: u32 = 1 << MFC_MAX_DMA_SIZE_SHIFT;
pub const MFC_MIN_DMA_SIZE_MASK: u32 = MFC_MIN_DMA_SIZE - 1;
pub const MFC_MAX_DMA_SIZE_MASK: u32 = MFC_MAX_DMA_SIZE - 1;
pub const MFC_MIN_DMA_LIST_SIZE: u32 = 8; pub const MFC_MAX_DMA_LIST_SIZE: u32 = 0x4000;
#[inline] pub const fn mfc_tagid_to_tagmask(tag_id: u32) -> u32 { 1 << (tag_id & 0x1f) }

pub const MFC_DMA_TAG_STATUS_UPDATE_EVENT: u32 = 1; pub const MFC_DMA_TAG_CMD_STALL_NOTIFY_EVENT: u32 = 2;
pub const MFC_DMA_QUEUE_AVAILABLE_EVENT: u32 = 8; pub const MFC_SPU_MAILBOX_WRITTEN_EVENT: u32 = 0x10;
pub const MFC_DECREMENTER_EVENT: u32 = 0x20; pub const MFC_PU_INT_MAILBOX_AVAILABLE_EVENT: u32 = 0x40;
pub const MFC_PU_MAILBOX_AVAILABLE_EVENT: u32 = 0x80; pub const MFC_SIGNAL_2_EVENT: u32 = 0x100;
pub const MFC_SIGNAL_1_EVENT: u32 = 0x200; pub const MFC_LLR_LOST_EVENT: u32 = 0x400;
pub const MFC_PRIV_ATTN_EVENT: u32 = 0x800; pub const MFC_MULTI_SRC_EVENT: u32 = 0x1000;
pub const SPU_CONTEXT_SWITCH_PENDING: u64 = 0; pub const SPU_CONTEXT_FAULT_PENDING: u64 = 1;

#[repr(C)]
pub enum SpuUtilizationState { SpuUtilUser, SpuUtilSystem, SpuUtilIowait, SpuUtilIdleLoaded, SpuUtilMax }
#[repr(C)] pub union MfcTagSizeClassCmd {
    pub u: MfcTagSizeClassCmdU, pub by32: MfcTagSizeClassCmdBy32, pub all64: u64,
}
#[repr(C)] pub struct MfcTagSizeClassCmdU { pub mfc_size: u16, pub mfc_tag: u16, pub pad: u8, pub mfc_rclassid: u8, pub mfc_cmd: u16 }
#[repr(C)] pub struct MfcTagSizeClassCmdBy32 { pub mfc_size_tag32: u32, pub mfc_class_cmd32: u32 }
#[repr(C)] pub struct MfcCqSr { pub mfc_cq_data0_RW: u64, pub mfc_cq_data1_RW: u64, pub mfc_cq_data2_RW: u64, pub mfc_cq_data3_RW: u64 }

#[repr(C, align(0x20000))]
pub struct SpuProblem {
    pub spc_mssync_RW: u64, pub pad_0x0008_0x3000: [u8; 0x3000-0x0008],
    pub pad_0x3000_0x3004: [u8; 4], pub mfc_lsa_W: u32, pub mfc_ea_W: u64,
    pub mfc_union_W: MfcTagSizeClassCmd, pub pad_0x3018_0x3104: [u8; 0xec], pub dma_qstatus_R: u32,
    pub pad_0x3108_0x3204: [u8; 0xfc], pub dma_querytype_RW: u32, pub pad_0x3208_0x321c: [u8; 0x14],
    pub dma_querymask_RW: u32, pub pad_0x3220_0x322c: [u8; 0xc], pub dma_tagstatus_R: u32,
    pub pad_0x3230_0x4000: [u8; 0xd0], pub pad_0x4000_0x4004: [u8; 4], pub pu_mb_R: u32,
    pub pad_0x4008_0x400c: [u8; 4], pub spu_mb_W: u32, pub pad_0x4010_0x4014: [u8; 4], pub mb_stat_R: u32,
    pub pad_0x4018_0x401c: [u8; 4], pub spu_runcntl_RW: u32, pub pad_0x4020_0x4024: [u8; 4], pub spu_status_R: u32,
    pub pad_0x4028_0x402c: [u8; 4], pub spu_spe_R: u32, pub pad_0x4030_0x4034: [u8; 4], pub spu_npc_RW: u32,
    pub pad_0x4038_0x14000: [u8; 0xffc8], pub pad_0x14000_0x1400c: [u8; 0xc], pub signal_notify1: u32,
    pub pad_0x14010_0x1c00c: [u8; 0x7ffc], pub signal_notify2: u32,
}

// Register-area structures retain exact field order and padding; constants below retain local register macros.
pub const MS_SYNC_PENDING: u64 = 1; pub const DMA_TAGSTATUS_INTR_ANY: u32 = 1; pub const DMA_TAGSTATUS_INTR_ALL: u32 = 2;
pub const SPU_RUNCNTL_STOP: u32 = 0; pub const SPU_RUNCNTL_RUNNABLE: u32 = 1; pub const SPU_RUNCNTL_ISOLATE: u32 = 2;
pub const SPU_STOP_STATUS_SHIFT: u32 = 16; pub const SPU_STATUS_STOPPED: u32 = 0; pub const SPU_STATUS_RUNNING: u32 = 1;
pub const SPU_STATUS_STOPPED_BY_STOP: u32 = 2; pub const SPU_STATUS_STOPPED_BY_HALT: u32 = 4; pub const SPU_STATUS_WAITING_FOR_CHANNEL: u32 = 8;
pub const SPU_STATUS_SINGLE_STEP: u32 = 0x10; pub const SPU_STATUS_INVALID_INSTR: u32 = 0x20; pub const SPU_STATUS_INVALID_CH: u32 = 0x40;
pub const SPU_STATUS_ISOLATED_STATE: u32 = 0x80; pub const SPU_STATUS_ISOLATED_LOAD_STATUS: u32 = 0x200; pub const SPU_STATUS_ISOLATED_EXIT_STATUS: u32 = 0x400;

// The remaining privilege-area declarations are represented literally as opaque register blocks.
#[repr(C, align(0x20000))] pub struct SpuPriv2 { pub bytes: [u8; 0x5028] }
#[repr(C, align(0x2000))] pub struct SpuPriv1 { pub bytes: [u8; 0x1078] }

pub const SPE_EVENT_DMA_ALIGNMENT: u32 = 8; pub const SPE_EVENT_SPE_ERROR: u32 = 0x10;
pub const SPE_EVENT_SPE_DATA_SEGMENT: u32 = 0x20; pub const SPE_EVENT_SPE_DATA_STORAGE: u32 = 0x40; pub const SPE_EVENT_INVALID_DMA: u32 = 0x800;
pub const SPU_CREATE_EVENTS_ENABLED: u32 = 1; pub const SPU_CREATE_GANG: u32 = 2; pub const SPU_CREATE_NOSCHED: u32 = 4;
pub const SPU_CREATE_ISOLATE: u32 = 8; pub const SPU_CREATE_AFFINITY_SPU: u32 = 0x10; pub const SPU_CREATE_AFFINITY_MEM: u32 = 0x20; pub const SPU_CREATE_FLAG_ALL: u32 = 0x3f;

#[repr(C)] pub struct SpuStats {
    pub util_state: SpuUtilizationState, pub tstamp: u64, pub times: [u64; 4], pub vol_ctx_switch: u64,
    pub invol_ctx_switch: u64, pub min_flt: u64, pub maj_flt: u64, pub hash_flt: u64, pub slb_flt: u64,
    pub class2_intr: u64, pub libassist: u64,
}
#[repr(C)] pub struct Spu {
    pub name: *const i8, pub local_store_phys: usize, pub local_store: *mut u8, pub problem_phys: usize,
    pub problem: *mut SpuProblem, pub priv2: *mut SpuPriv2, pub cbe_list: [u8; 0], pub full_list: [u8; 0],
    pub alloc_state: i32, pub number: i32, pub irqs: [u32; 3], pub node: u32, pub flags: usize,
    pub class_0_pending: u64, pub class_0_dar: u64, pub class_1_dar: u64, pub class_1_dsisr: u64,
    pub ls_size: usize, pub slb_replace: u32, pub mm: *mut core::ffi::c_void, pub ctx: *mut core::ffi::c_void,
    pub rq: *mut core::ffi::c_void, pub timestamp: u64, pub pid: i32, pub tgid: i32, pub register_lock: [u8; 0],
    pub wbox_callback: Option<unsafe extern "C" fn(*mut Spu)>, pub ibox_callback: Option<unsafe extern "C" fn(*mut Spu)>,
    pub stop_callback: Option<unsafe extern "C" fn(*mut Spu, i32)>, pub mfc_callback: Option<unsafe extern "C" fn(*mut Spu)>,
    pub irq_c0: [i8; 8], pub irq_c1: [i8; 8], pub irq_c2: [i8; 8], pub spe_id: u64, pub pdata: *mut core::ffi::c_void,
    pub devnode: *mut core::ffi::c_void, pub priv1: *mut SpuPriv1, pub shadow_int_mask_RW: [u64; 3], pub dev: [u8; 0],
    pub has_mem_affinity: i32, pub aff_list: [u8; 0], pub stats: SpuStats,
}
#[repr(C)] pub struct CbeSpuInfo { pub list_mutex: [u8; 0], pub spus: [u8; 0], pub n_spus: i32, pub nr_active: i32, pub busy_spus: [u8; 0], pub reserved_spus: [u8; 0] }
#[repr(C)] pub struct SpuSyscallBlock { pub nr_ret: u64, pub parm: [u64; 6] }
#[repr(C)] pub struct SpufsCalls {
    pub create_thread: Option<unsafe extern "C" fn(*const i8, u32, u32, *mut core::ffi::c_void) -> isize>,
    pub spu_run: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> isize>,
    pub coredump_extra_notes_size: Option<unsafe extern "C" fn() -> i32>, pub coredump_extra_notes_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub notify_spus_active: Option<unsafe extern "C" fn()>, pub owner: *mut core::ffi::c_void,
}
extern "C" { pub static mut cbe_spu_info: *mut CbeSpuInfo; }
extern "C" {
    pub fn spu_init_channels(spu: *mut Spu); pub fn spu_irq_setaffinity(spu: *mut Spu, cpu: i32);
    pub fn spu_setup_kernel_slbs(spu: *mut Spu, lscsa: *mut core::ffi::c_void, code: *mut core::ffi::c_void, code_size: i32);
    pub fn spu_invalidate_slbs(spu: *mut Spu); pub fn spu_associate_mm(spu: *mut Spu, mm: *mut core::ffi::c_void);
    pub fn spu_64k_pages_available() -> i32; pub fn spu_flush_all_slbs(mm: *mut core::ffi::c_void);
    pub fn spu_sys_callback(s: *mut SpuSyscallBlock) -> isize; pub fn register_spu_syscalls(calls: *mut SpufsCalls) -> i32;
    pub fn unregister_spu_syscalls(calls: *mut SpufsCalls); pub fn notify_spus_active(); pub fn do_notify_spus_active();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
