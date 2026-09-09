/* SPDX-License-Identifier: GPL-2.0-only or MIT */
/* Copyright 2025 Arm, Ltd. */

// Translated from ethosu_device.h. External kernel/DRM types and symbols are
// intentionally referenced but not implemented here.

pub const NPU_REG_ID: u32 = 0x0000;
pub const NPU_REG_STATUS: u32 = 0x0004;
pub const NPU_REG_CMD: u32 = 0x0008;
pub const NPU_REG_RESET: u32 = 0x000c;
pub const NPU_REG_QBASE: u32 = 0x0010;
pub const NPU_REG_QBASE_HI: u32 = 0x0014;
pub const NPU_REG_QREAD: u32 = 0x0018;
pub const NPU_REG_QCONFIG: u32 = 0x001c;
pub const NPU_REG_QSIZE: u32 = 0x0020;
pub const NPU_REG_PROT: u32 = 0x0024;
pub const NPU_REG_CONFIG: u32 = 0x0028;
pub const NPU_REG_REGIONCFG: u32 = 0x003c;
pub const NPU_REG_AXILIMIT0: u32 = 0x0040; // U65
pub const NPU_REG_AXILIMIT1: u32 = 0x0044; // U65
pub const NPU_REG_AXILIMIT2: u32 = 0x0048; // U65
pub const NPU_REG_AXILIMIT3: u32 = 0x004c; // U65
pub const NPU_REG_MEM_ATTR0: u32 = 0x0040; // U85
pub const NPU_REG_MEM_ATTR1: u32 = 0x0044; // U85
pub const NPU_REG_MEM_ATTR2: u32 = 0x0048; // U85
pub const NPU_REG_MEM_ATTR3: u32 = 0x004c; // U85
pub const NPU_REG_AXI_SRAM: u32 = 0x0050; // U85
pub const NPU_REG_AXI_EXT: u32 = 0x0054; // U85

#[inline]
pub const fn NPU_REG_BASEP(x: u32) -> u32 { 0x0080u32.wrapping_add(x.wrapping_mul(8)) }
#[inline]
pub const fn NPU_REG_BASEP_HI(x: u32) -> u32 { 0x0084u32.wrapping_add(x.wrapping_mul(8)) }
pub const NPU_BASEP_REGION_MAX: u32 = 8;
pub const NPU_REG_PMCR: u32 = 0x0180;
pub const NPU_REG_PMCNTENSET: u32 = 0x0184;
pub const NPU_REG_PMCNTENCLR: u32 = 0x0188;
pub const NPU_REG_PMCCNTR_LO: u32 = 0x01A0;
pub const NPU_REG_PMCCNTR_HI: u32 = 0x01A4;
pub const NPU_REG_PMCCNTR_CFG: u32 = 0x01A8;
#[inline] pub const fn NPU_REG_PMU_EVCNTR(x: u32) -> u32 { 0x0300u32.wrapping_add(x.wrapping_mul(4)) }
#[inline] pub const fn NPU_REG_PMU_EVTYPER(x: u32) -> u32 { 0x0380u32.wrapping_add(x.wrapping_mul(4)) }

pub const ID_ARCH_MAJOR_MASK: u32 = 0xf0000000;
pub const ID_ARCH_MINOR_MASK: u32 = 0x0ff00000;
pub const ID_ARCH_PATCH_MASK: u32 = 0x000f0000;
pub const ID_VER_MAJOR_MASK: u32 = 0x00000f00;
pub const ID_VER_MINOR_MASK: u32 = 0x000000f0;
pub const CONFIG_MACS_PER_CC_MASK: u32 = 0x0000000f;
pub const CONFIG_CMD_STREAM_VER_MASK: u32 = 0x000000f0;
pub const STATUS_STATE_RUNNING: u32 = 1 << 0;
pub const STATUS_IRQ_RAISED: u32 = 1 << 1;
pub const STATUS_BUS_STATUS: u32 = 1 << 2;
pub const STATUS_RESET_STATUS: u32 = 1 << 3;
pub const STATUS_CMD_PARSE_ERR: u32 = 1 << 4;
pub const STATUS_CMD_END_REACHED: u32 = 1 << 5;
pub const CMD_CLEAR_IRQ: u32 = 1 << 1;
pub const CMD_TRANSITION_TO_RUN: u32 = 1 << 0;
pub const RESET_PENDING_CSL: u32 = 1 << 1;
pub const RESET_PENDING_CPL: u32 = 1 << 0;
pub const PROT_ACTIVE_CSL: u32 = 1 << 1;
pub const PMCR_NUM_EVENT_CNT_MASK: u32 = 0x0000f800;
pub const PMCR_CYCLE_CNT_RST: u32 = 1 << 2;
pub const PMCR_EVENT_CNT_RST: u32 = 1 << 1;
pub const PMCR_CNT_EN: u32 = 1 << 0;
pub const PMU_EV_TYPE_NONE: u32 = 0;
pub const PMU_EV_TYPE_CYCLES: u32 = 0x11;
pub const PMU_EV_TYPE_IDLE: u32 = 0x20;

#[repr(u32)]
pub enum ethosu_cmds {
    NPU_OP_CONV = 0x2, NPU_OP_DEPTHWISE = 0x3, NPU_OP_POOL = 0x5,
    NPU_OP_ELEMENTWISE = 0x6, NPU_OP_RESIZE = 0x7, NPU_OP_DMA_START = 0x10,
    NPU_SET_IFM_PAD_TOP = 0x100, NPU_SET_IFM_PAD_LEFT = 0x101,
    NPU_SET_IFM_PAD_RIGHT = 0x102, NPU_SET_IFM_PAD_BOTTOM = 0x103,
    NPU_SET_IFM_DEPTH_M1 = 0x104, NPU_SET_IFM_PRECISION = 0x105,
    NPU_SET_IFM_BROADCAST = 0x108, NPU_SET_IFM_WIDTH0_M1 = 0x10a,
    NPU_SET_IFM_HEIGHT0_M1 = 0x10b, NPU_SET_IFM_HEIGHT1_M1 = 0x10c,
    NPU_SET_IFM_REGION = 0x10f, NPU_SET_OFM_WIDTH_M1 = 0x111,
    NPU_SET_OFM_HEIGHT_M1 = 0x112, NPU_SET_OFM_DEPTH_M1 = 0x113,
    NPU_SET_OFM_PRECISION = 0x114, NPU_SET_OFM_WIDTH0_M1 = 0x11a,
    NPU_SET_OFM_HEIGHT0_M1 = 0x11b, NPU_SET_OFM_HEIGHT1_M1 = 0x11c,
    NPU_SET_OFM_REGION = 0x11f, NPU_SET_KERNEL_WIDTH_M1 = 0x120,
    NPU_SET_KERNEL_HEIGHT_M1 = 0x121, NPU_SET_KERNEL_STRIDE = 0x122,
    NPU_SET_WEIGHT_REGION = 0x128, NPU_SET_SCALE_REGION = 0x129,
    NPU_SET_DMA0_SRC_REGION = 0x130, NPU_SET_DMA0_DST_REGION = 0x131,
    NPU_SET_DMA0_SIZE0 = 0x132, NPU_SET_DMA0_SIZE1 = 0x133,
    NPU_SET_IFM2_BROADCAST = 0x180, NPU_SET_IFM2_PRECISION = 0x185,
    NPU_SET_IFM2_WIDTH0_M1 = 0x18a, NPU_SET_IFM2_HEIGHT0_M1 = 0x18b,
    NPU_SET_IFM2_HEIGHT1_M1 = 0x18c, NPU_SET_IFM2_REGION = 0x18f,
    NPU_SET_IFM_BASE0 = 0x4000, NPU_SET_IFM_BASE1 = 0x4001, NPU_SET_IFM_BASE2 = 0x4002, NPU_SET_IFM_BASE3 = 0x4003,
    NPU_SET_IFM_STRIDE_X = 0x4004, NPU_SET_IFM_STRIDE_Y = 0x4005, NPU_SET_IFM_STRIDE_C = 0x4006,
    NPU_SET_OFM_BASE0 = 0x4010, NPU_SET_OFM_BASE1 = 0x4011, NPU_SET_OFM_BASE2 = 0x4012, NPU_SET_OFM_BASE3 = 0x4013,
    NPU_SET_OFM_STRIDE_X = 0x4014, NPU_SET_OFM_STRIDE_Y = 0x4015, NPU_SET_OFM_STRIDE_C = 0x4016,
    NPU_SET_WEIGHT_BASE = 0x4020, NPU_SET_WEIGHT_LENGTH = 0x4021, NPU_SET_SCALE_BASE = 0x4022, NPU_SET_SCALE_LENGTH = 0x4023,
    NPU_SET_DMA0_SRC = 0x4030, NPU_SET_DMA0_DST = 0x4031, NPU_SET_DMA0_LEN = 0x4032,
    NPU_SET_DMA0_SRC_STRIDE0 = 0x4033, NPU_SET_DMA0_SRC_STRIDE1 = 0x4034, NPU_SET_DMA0_DST_STRIDE0 = 0x4035, NPU_SET_DMA0_DST_STRIDE1 = 0x4036,
    NPU_SET_IFM2_BASE0 = 0x4080, NPU_SET_IFM2_BASE1 = 0x4081, NPU_SET_IFM2_BASE2 = 0x4082, NPU_SET_IFM2_BASE3 = 0x4083,
    NPU_SET_IFM2_STRIDE_X = 0x4084, NPU_SET_IFM2_STRIDE_Y = 0x4085, NPU_SET_IFM2_STRIDE_C = 0x4086,
    NPU_SET_WEIGHT1_BASE = 0x4090, NPU_SET_WEIGHT1_LENGTH = 0x4091, NPU_SET_SCALE1_BASE = 0x4092,
    NPU_SET_WEIGHT2_BASE = 0x4092, NPU_SET_SCALE1_LENGTH = 0x4093, NPU_SET_WEIGHT2_LENGTH = 0x4093,
    NPU_SET_WEIGHT3_BASE = 0x4094, NPU_SET_WEIGHT3_LENGTH = 0x4095,
}

pub const ETHOSU_SRAM_REGION: u32 = 2; // Matching Vela compiler

pub struct ethosu_perfmon;

#[repr(C)]
pub struct ethosu_device {
    pub base: drm_device,
    pub regs: *mut core::ffi::c_void,
    pub pmu_regs: *mut core::ffi::c_void,
    pub sram: *mut core::ffi::c_void,
    pub srampool: *mut gen_pool,
    pub sramphys: dma_addr_t,
    pub clks: *mut clk_bulk_data,
    pub num_clks: i32,
    pub irq: i32,
    pub npu_info: drm_ethosu_npu_info,
    pub in_flight_job: *mut ethosu_job,
    pub fence_lock: spinlock_t,
    pub sched: drm_gpu_scheduler,
    pub sched_lock: mutex,
    pub fence_context: u64,
    pub emit_seqno: u64,
    pub perfmon_state: ethosu_perfmon_state,
    pub global_perfmon: *mut ethosu_perfmon,
}

#[repr(C)]
pub struct ethosu_perfmon_state { pub lock: mutex, pub active: *mut ethosu_perfmon }

#[inline]
pub unsafe fn to_ethosu_device(drm_dev: *mut drm_device) -> *mut ethosu_device {
    drm_dev as *mut ethosu_device
}

#[inline]
pub unsafe fn ethosu_is_u65(ethosudev: *const ethosu_device) -> bool {
    ((*ethosudev).npu_info.id & ID_ARCH_MAJOR_MASK) >> 28 == 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
