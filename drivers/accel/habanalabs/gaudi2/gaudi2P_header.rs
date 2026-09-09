/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2020-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies supplied by the corresponding C headers are intentionally external.

pub const GAUDI2_LINUX_FW_FILE: &str = "habanalabs/gaudi2/gaudi2-fit.itb";
pub const GAUDI2_BOOT_FIT_FILE: &str = "habanalabs/gaudi2/gaudi2-boot-fit.itb";
pub const GAUDI2_CPU_TIMEOUT_USEC: u32 = 30000000;
pub const NUMBER_OF_PDMA_QUEUES: usize = 2;
pub const NUMBER_OF_EDMA_QUEUES: usize = 8;
pub const NUMBER_OF_MME_QUEUES: usize = 4;
pub const NUMBER_OF_TPC_QUEUES: usize = 25;
pub const NUMBER_OF_NIC_QUEUES: usize = 24;
pub const NUMBER_OF_ROT_QUEUES: usize = 2;
pub const NUMBER_OF_CPU_QUEUES: usize = 1;
pub const NUMBER_OF_HW_QUEUES: usize = (NUMBER_OF_PDMA_QUEUES + NUMBER_OF_EDMA_QUEUES + NUMBER_OF_MME_QUEUES + NUMBER_OF_TPC_QUEUES + NUMBER_OF_NIC_QUEUES + NUMBER_OF_ROT_QUEUES + NUMBER_OF_CPU_QUEUES) * NUM_OF_PQ_PER_QMAN;
pub const NUMBER_OF_QUEUES: usize = NUMBER_OF_CPU_QUEUES + NUMBER_OF_HW_QUEUES;
pub const DCORE_NUM_OF_SOB: usize = ((mmDCORE0_SYNC_MNGR_OBJS_SOB_OBJ_8191 - mmDCORE0_SYNC_MNGR_OBJS_SOB_OBJ_0 + 4) >> 2);
pub const DCORE_NUM_OF_MONITORS: usize = ((mmDCORE0_SYNC_MNGR_OBJS_MON_STATUS_2047 - mmDCORE0_SYNC_MNGR_OBJS_MON_STATUS_0 + 4) >> 2);
pub const NUMBER_OF_DEC: usize = NUM_OF_DEC_PER_DCORE * NUM_OF_DCORES + NUMBER_OF_PCIE_DEC;
pub const NUM_OF_USER_ACP_BLOCKS: usize = NUM_OF_SCHEDULER_ARC + 2;
pub const NUM_OF_USER_NIC_UMR_BLOCKS: usize = 15;
pub const NUM_OF_EXPOSED_SM_BLOCKS: usize = (NUM_OF_DCORES - 1) * 2;
pub const NUM_USER_MAPPED_BLOCKS: usize = NUM_ARC_CPUS + NUM_OF_USER_ACP_BLOCKS + NUMBER_OF_DEC + NUM_OF_EXPOSED_SM_BLOCKS + NIC_NUMBER_OF_ENGINES * NUM_OF_USER_NIC_UMR_BLOCKS;
pub const USR_MAPPED_BLK_DEC_START_IDX: usize = NUM_ARC_CPUS + NUM_OF_USER_ACP_BLOCKS + NIC_NUMBER_OF_ENGINES * NUM_OF_USER_NIC_UMR_BLOCKS;
pub const USR_MAPPED_BLK_SM_START_IDX: usize = NUM_ARC_CPUS + NUM_OF_USER_ACP_BLOCKS + NUMBER_OF_DEC + NIC_NUMBER_OF_ENGINES * NUM_OF_USER_NIC_UMR_BLOCKS;
pub const SM_OBJS_BLOCK_SIZE: usize = mmDCORE0_SYNC_MNGR_OBJS_SM_SEC_0 - mmDCORE0_SYNC_MNGR_OBJS_SOB_OBJ_0;
pub const GAUDI2_MAX_PENDING_CS: usize = 64;
pub const CORESIGHT_TIMEOUT_USEC: u32 = 100000;
pub const GAUDI2_PREBOOT_REQ_TIMEOUT_USEC: u32 = 25000000;
pub const GAUDI2_PREBOOT_EXTENDED_REQ_TIMEOUT_USEC: u32 = 85000000;
pub const GAUDI2_BOOT_FIT_REQ_TIMEOUT_USEC: u32 = 10000000;
pub const GAUDI2_NIC_CLK_FREQ: u64 = 450000000;
pub const DC_POWER_DEFAULT: u32 = 60000;
pub const GAUDI2_HBM_NUM: usize = 6;
pub const DMA_MAX_TRANSFER_SIZE: u32 = U32_MAX;
pub const GAUDI2_DEFAULT_CARD_NAME: &str = "HL225";
pub const QMAN_STREAMS: usize = 4;
pub const NUM_OF_MME_SBTE_PORTS: usize = 5;
pub const NUM_OF_MME_WB_PORTS: usize = 2;
pub const GAUDI2_ENGINE_ID_DCORE_OFFSET: u32 = GAUDI2_DCORE1_ENGINE_ID_EDMA_0 - GAUDI2_DCORE0_ENGINE_ID_EDMA_0;
pub const CPU_FW_IMAGE_SIZE: u64 = 0x10000000;
pub const CPU_FW_IMAGE_ADDR: u64 = DRAM_PHYS_BASE;
pub const PMMU_PAGE_TABLES_SIZE: u64 = 0x10000000;
pub const EDMA_PQS_SIZE: u64 = SZ_2M;
pub const EDMA_SCRATCHPAD_SIZE: u64 = SZ_1M;
pub const HMMU_PAGE_TABLES_SIZE: u64 = SZ_1M;
pub const NIC_NUMBER_OF_PORTS: usize = NIC_NUMBER_OF_ENGINES;
pub const NUMBER_OF_PCIE_DEC: usize = 2;
pub const PCIE_DEC_SHIFT: usize = 8;
pub const SRAM_USER_BASE_OFFSET: usize = 0;
pub const MAX_FAULTY_HBMS: usize = 1;
pub const GAUDI2_XBAR_EDGE_FULL_MASK: u32 = 0xF;
pub const GAUDI2_EDMA_FULL_MASK: u32 = 0xFF;
pub const GAUDI2_DRAM_FULL_MASK: u32 = 0x3F;
pub const VA_HOST_SPACE_PAGE_START: u64 = 0xFFF0000000000000;
pub const VA_HOST_SPACE_PAGE_END: u64 = 0xFFF0800000000000;
pub const VA_HOST_SPACE_HPAGE_START: u64 = 0xFFF0800000000000;
pub const VA_HOST_SPACE_HPAGE_END: u64 = 0xFFF1000000000000;
pub const VA_HOST_SPACE_PAGE_SIZE: u64 = VA_HOST_SPACE_PAGE_END - VA_HOST_SPACE_PAGE_START;
pub const VA_HOST_SPACE_HPAGE_SIZE: u64 = VA_HOST_SPACE_HPAGE_END - VA_HOST_SPACE_HPAGE_START;
pub const VA_HOST_SPACE_SIZE: u64 = VA_HOST_SPACE_PAGE_SIZE + VA_HOST_SPACE_HPAGE_SIZE;
pub const HOST_SPACE_INTERNAL_CB_SZ: u64 = SZ_2M;
pub const VA_HBM_SPACE_END: u64 = 0x1002000000000000;

pub const HW_CAP_PLL: u64 = BIT_ULL(0); pub const HW_CAP_DRAM: u64 = BIT_ULL(1); pub const HW_CAP_PMMU: u64 = BIT_ULL(2); pub const HW_CAP_CPU: u64 = BIT_ULL(3); pub const HW_CAP_MSIX: u64 = BIT_ULL(4);
pub const HW_CAP_CPU_Q: u64 = BIT_ULL(5); pub const HW_CAP_CPU_Q_SHIFT: u32 = 5; pub const HW_CAP_CLK_GATE: u64 = BIT_ULL(6); pub const HW_CAP_KDMA: u64 = BIT_ULL(7); pub const HW_CAP_SRAM_SCRAMBLER: u64 = BIT_ULL(8);
pub const HW_CAP_DMMU_MASK: u64 = GENMASK_ULL(24, 9); pub const HW_CAP_DMMU_SHIFT: u32 = 9; pub const HW_CAP_PDMA_MASK: u64 = BIT_ULL(26); pub const HW_CAP_EDMA_MASK: u64 = GENMASK_ULL(34, 27); pub const HW_CAP_EDMA_SHIFT: u32 = 27; pub const HW_CAP_MME_MASK: u64 = GENMASK_ULL(38, 35); pub const HW_CAP_MME_SHIFT: u32 = 35; pub const HW_CAP_ROT_MASK: u64 = GENMASK_ULL(40, 39); pub const HW_CAP_ROT_SHIFT: u32 = 39;
pub const HW_CAP_HBM_SCRAMBLER_HW_RESET: u64 = BIT_ULL(41); pub const HW_CAP_HBM_SCRAMBLER_SW_RESET: u64 = BIT_ULL(42); pub const HW_CAP_HBM_SCRAMBLER_MASK: u64 = HW_CAP_HBM_SCRAMBLER_HW_RESET | HW_CAP_HBM_SCRAMBLER_SW_RESET; pub const HW_CAP_HBM_SCRAMBLER_SHIFT: u32 = 41; pub const HW_CAP_RESERVED: u64 = BIT(43); pub const HW_CAP_MMU_MASK: u64 = HW_CAP_PMMU | HW_CAP_DMMU_MASK;
pub const RR_TYPE_SHORT: u32 = 0; pub const RR_TYPE_LONG: u32 = 1; pub const RR_TYPE_SHORT_PRIV: u32 = 2; pub const RR_TYPE_LONG_PRIV: u32 = 3; pub const NUM_SHORT_LBW_RR: usize = 14; pub const NUM_LONG_LBW_RR: usize = 4; pub const NUM_SHORT_HBW_RR: usize = 6; pub const NUM_LONG_HBW_RR: usize = 4;
pub const RAZWI_INITIATOR_X_SHIFT: u32 = 0; pub const RAZWI_INITIATOR_X_MASK: u32 = 0x1F; pub const RAZWI_INITIATOR_Y_SHIFT: u32 = 5; pub const RAZWI_INITIATOR_Y_MASK: u32 = 0xF;
#[inline] pub const fn RTR_ID_X_Y(x: u32, y: u32) -> u32 { ((y & RAZWI_INITIATOR_Y_MASK) << RAZWI_INITIATOR_Y_SHIFT) | ((x & RAZWI_INITIATOR_X_MASK) << RAZWI_INITIATOR_X_SHIFT) }
pub const HW_CAP_DEC_SHIFT: u32 = 0; pub const HW_CAP_DEC_MASK: u64 = GENMASK_ULL(9, 0); pub const HW_CAP_TPC_SHIFT: u32 = 0; pub const HW_CAP_TPC_MASK: u64 = GENMASK_ULL(24, 0); pub const HW_CAP_NIC_SHIFT: u32 = 0; pub const HW_CAP_NIC_MASK: u64 = GENMASK_ULL(NIC_NUMBER_OF_ENGINES - 1, 0);
#[inline] pub const fn GAUDI2_ARC_PCI_MSB_ADDR(addr: u64) -> u64 { (addr & GENMASK_ULL(49, 28)) >> 28 }
pub const GAUDI2_NUM_TESTED_QS: usize = GAUDI2_QUEUE_ID_CPU_PQ - GAUDI2_QUEUE_ID_PDMA_0_0;

#[repr(C)] pub enum gaudi2_reserved_sob_id { GAUDI2_RESERVED_SOB_CS_COMPLETION_FIRST, GAUDI2_RESERVED_SOB_CS_COMPLETION_LAST = GAUDI2_RESERVED_SOB_CS_COMPLETION_FIRST as isize + GAUDI2_MAX_PENDING_CS as isize - 1, GAUDI2_RESERVED_SOB_KDMA_COMPLETION, GAUDI2_RESERVED_SOB_DEC_NRM_FIRST, GAUDI2_RESERVED_SOB_DEC_NRM_LAST = GAUDI2_RESERVED_SOB_DEC_NRM_FIRST as isize + NUMBER_OF_DEC as isize - 1, GAUDI2_RESERVED_SOB_DEC_ABNRM_FIRST, GAUDI2_RESERVED_SOB_DEC_ABNRM_LAST = GAUDI2_RESERVED_SOB_DEC_ABNRM_FIRST as isize + NUMBER_OF_DEC as isize - 1, GAUDI2_RESERVED_SOB_NUMBER }
#[repr(C)] pub enum gaudi2_reserved_mon_id { GAUDI2_RESERVED_MON_CS_COMPLETION_FIRST, GAUDI2_RESERVED_MON_CS_COMPLETION_LAST = GAUDI2_RESERVED_MON_CS_COMPLETION_FIRST as isize + GAUDI2_MAX_PENDING_CS as isize - 1, GAUDI2_RESERVED_MON_KDMA_COMPLETION, GAUDI2_RESERVED_MON_DEC_NRM_FIRST, GAUDI2_RESERVED_MON_DEC_NRM_LAST = GAUDI2_RESERVED_MON_DEC_NRM_FIRST as isize + 3 * NUMBER_OF_DEC as isize - 1, GAUDI2_RESERVED_MON_DEC_ABNRM_FIRST, GAUDI2_RESERVED_MON_DEC_ABNRM_LAST = GAUDI2_RESERVED_MON_DEC_ABNRM_FIRST as isize + 3 * NUMBER_OF_DEC as isize - 1, GAUDI2_RESERVED_MON_NUMBER }
#[repr(C)] pub enum gaudi2_reserved_cq_id { GAUDI2_RESERVED_CQ_CS_COMPLETION, GAUDI2_RESERVED_CQ_KDMA_COMPLETION, GAUDI2_RESERVED_CQ_NUMBER }
#[repr(C)] pub enum substitude_tpc { FAULTY_TPC_SUBTS_1_TPC_24, FAULTY_TPC_SUBTS_2_TPC_23, MAX_FAULTY_TPCS }
#[repr(C)] pub enum gaudi2_dma_core_id { DMA_CORE_ID_PDMA0, DMA_CORE_ID_PDMA1, DMA_CORE_ID_EDMA0, DMA_CORE_ID_EDMA1, DMA_CORE_ID_EDMA2, DMA_CORE_ID_EDMA3, DMA_CORE_ID_EDMA4, DMA_CORE_ID_EDMA5, DMA_CORE_ID_EDMA6, DMA_CORE_ID_EDMA7, DMA_CORE_ID_KDMA, DMA_CORE_ID_SIZE }
#[repr(C)] pub enum gaudi2_rotator_id { ROTATOR_ID_0, ROTATOR_ID_1, ROTATOR_ID_SIZE }
#[repr(C)] pub enum gaudi2_mme_id { MME_ID_DCORE0, MME_ID_DCORE1, MME_ID_DCORE2, MME_ID_DCORE3, MME_ID_SIZE }
#[repr(C)] pub enum gaudi2_tpc_id { TPC_ID_DCORE0_TPC0, TPC_ID_DCORE0_TPC1, TPC_ID_DCORE0_TPC2, TPC_ID_DCORE0_TPC3, TPC_ID_DCORE0_TPC4, TPC_ID_DCORE0_TPC5, TPC_ID_DCORE1_TPC0, TPC_ID_DCORE1_TPC1, TPC_ID_DCORE1_TPC2, TPC_ID_DCORE1_TPC3, TPC_ID_DCORE1_TPC4, TPC_ID_DCORE1_TPC5, TPC_ID_DCORE2_TPC0, TPC_ID_DCORE2_TPC1, TPC_ID_DCORE2_TPC2, TPC_ID_DCORE2_TPC3, TPC_ID_DCORE2_TPC4, TPC_ID_DCORE2_TPC5, TPC_ID_DCORE3_TPC0, TPC_ID_DCORE3_TPC1, TPC_ID_DCORE3_TPC2, TPC_ID_DCORE3_TPC3, TPC_ID_DCORE3_TPC4, TPC_ID_DCORE3_TPC5, TPC_ID_DCORE0_TPC6, TPC_ID_SIZE }
#[repr(C)] pub enum gaudi2_dec_id { DEC_ID_DCORE0_DEC0, DEC_ID_DCORE0_DEC1, DEC_ID_DCORE1_DEC0, DEC_ID_DCORE1_DEC1, DEC_ID_DCORE2_DEC0, DEC_ID_DCORE2_DEC1, DEC_ID_DCORE3_DEC0, DEC_ID_DCORE3_DEC1, DEC_ID_PCIE_VDEC0, DEC_ID_PCIE_VDEC1, DEC_ID_SIZE }
#[repr(C)] pub enum gaudi2_hbm_id { HBM_ID0, HBM_ID1, HBM_ID2, HBM_ID3, HBM_ID4, HBM_ID5, HBM_ID_SIZE }
#[repr(C)] pub enum gaudi2_edma_id { EDMA_ID_DCORE0_INSTANCE0, EDMA_ID_DCORE0_INSTANCE1, EDMA_ID_DCORE1_INSTANCE0, EDMA_ID_DCORE1_INSTANCE1, EDMA_ID_DCORE2_INSTANCE0, EDMA_ID_DCORE2_INSTANCE1, EDMA_ID_DCORE3_INSTANCE0, EDMA_ID_DCORE3_INSTANCE1, EDMA_ID_SIZE }
pub const GAUDI2_NUM_USER_INTERRUPTS: usize = 64; pub const GAUDI2_NUM_RESERVED_INTERRUPTS: usize = 1; pub const GAUDI2_TOTAL_USER_INTERRUPTS: usize = GAUDI2_NUM_USER_INTERRUPTS + GAUDI2_NUM_RESERVED_INTERRUPTS;
#[repr(C)] pub enum gaudi2_irq_num { GAUDI2_IRQ_NUM_EVENT_QUEUE = GAUDI2_EVENT_QUEUE_MSIX_IDX as isize, GAUDI2_IRQ_NUM_DCORE0_DEC0_NRM, GAUDI2_IRQ_NUM_DCORE0_DEC0_ABNRM, GAUDI2_IRQ_NUM_DCORE0_DEC1_NRM, GAUDI2_IRQ_NUM_DCORE0_DEC1_ABNRM, GAUDI2_IRQ_NUM_DCORE1_DEC0_NRM, GAUDI2_IRQ_NUM_DCORE1_DEC0_ABNRM, GAUDI2_IRQ_NUM_DCORE1_DEC1_NRM, GAUDI2_IRQ_NUM_DCORE1_DEC1_ABNRM, GAUDI2_IRQ_NUM_DCORE2_DEC0_NRM, GAUDI2_IRQ_NUM_DCORE2_DEC0_ABNRM, GAUDI2_IRQ_NUM_DCORE2_DEC1_NRM, GAUDI2_IRQ_NUM_DCORE2_DEC1_ABNRM, GAUDI2_IRQ_NUM_DCORE3_DEC0_NRM, GAUDI2_IRQ_NUM_DCORE3_DEC0_ABNRM, GAUDI2_IRQ_NUM_DCORE3_DEC1_NRM, GAUDI2_IRQ_NUM_DCORE3_DEC1_ABNRM, GAUDI2_IRQ_NUM_SHARED_DEC0_NRM, GAUDI2_IRQ_NUM_SHARED_DEC0_ABNRM, GAUDI2_IRQ_NUM_SHARED_DEC1_NRM, GAUDI2_IRQ_NUM_SHARED_DEC1_ABNRM, GAUDI2_IRQ_NUM_DEC_LAST, GAUDI2_IRQ_NUM_COMPLETION, GAUDI2_IRQ_NUM_NIC_PORT_FIRST, GAUDI2_IRQ_NUM_NIC_PORT_LAST = GAUDI2_IRQ_NUM_NIC_PORT_FIRST as isize + NIC_NUMBER_OF_PORTS as isize - 1, GAUDI2_IRQ_NUM_TPC_ASSERT, GAUDI2_IRQ_NUM_EQ_ERROR, GAUDI2_IRQ_NUM_USER_FIRST, GAUDI2_IRQ_NUM_USER_LAST = GAUDI2_IRQ_NUM_USER_FIRST as isize + GAUDI2_NUM_USER_INTERRUPTS as isize - 1, GAUDI2_IRQ_NUM_RESERVED_FIRST, GAUDI2_IRQ_NUM_RESERVED_LAST = GAUDI2_MSIX_ENTRIES as isize - GAUDI2_NUM_RESERVED_INTERRUPTS as isize - 1, GAUDI2_IRQ_NUM_UNEXPECTED_ERROR = RESERVED_MSIX_UNEXPECTED_USER_ERROR_INTERRUPT as isize, GAUDI2_IRQ_NUM_LAST = GAUDI2_MSIX_ENTRIES as isize - 1 }

#[repr(C)] pub struct gaudi2_device { pub cpucp_info_get: Option<unsafe extern "C" fn(*mut hl_device) -> i32>, pub mapped_blocks: [user_mapped_block; NUM_USER_MAPPED_BLOCKS], pub lfsr_rand_seeds: [i32; MME_NUM_OF_LFSR_SEEDS], pub hw_queues_lock: spinlock_t, pub scratchpad_kernel_address: *mut core::ffi::c_void, pub scratchpad_bus_address: dma_addr_t, pub virt_msix_db_cpu_addr: *mut core::ffi::c_void, pub virt_msix_db_dma_addr: dma_addr_t, pub dram_bar_cur_addr: u64, pub hw_cap_initialized: u64, pub active_hw_arc: u64, pub dec_hw_cap_initialized: u64, pub tpc_hw_cap_initialized: u64, pub active_tpc_arc: u64, pub nic_hw_cap_initialized: u64, pub active_nic_arc: u64, pub hw_events: [u32; GAUDI2_EVENT_SIZE], pub events_stat: [u32; GAUDI2_EVENT_SIZE], pub events_stat_aggregate: [u32; GAUDI2_EVENT_SIZE], pub num_of_valid_hw_events: u32, pub queues_test_info: [gaudi2_queues_test_info; GAUDI2_NUM_TESTED_QS] }

#[repr(C)] pub enum gaudi2_block_types { GAUDI2_BLOCK_TYPE_PLL, GAUDI2_BLOCK_TYPE_RTR, GAUDI2_BLOCK_TYPE_CPU, GAUDI2_BLOCK_TYPE_HIF, GAUDI2_BLOCK_TYPE_HBM, GAUDI2_BLOCK_TYPE_NIC, GAUDI2_BLOCK_TYPE_PCIE, GAUDI2_BLOCK_TYPE_PCIE_PMA, GAUDI2_BLOCK_TYPE_PDMA, GAUDI2_BLOCK_TYPE_EDMA, GAUDI2_BLOCK_TYPE_PMMU, GAUDI2_BLOCK_TYPE_PSOC, GAUDI2_BLOCK_TYPE_ROT, GAUDI2_BLOCK_TYPE_ARC_FARM, GAUDI2_BLOCK_TYPE_DEC, GAUDI2_BLOCK_TYPE_MME, GAUDI2_BLOCK_TYPE_EU_BIST, GAUDI2_BLOCK_TYPE_SYNC_MNGR, GAUDI2_BLOCK_TYPE_STLB, GAUDI2_BLOCK_TYPE_TPC, GAUDI2_BLOCK_TYPE_HMMU, GAUDI2_BLOCK_TYPE_SRAM, GAUDI2_BLOCK_TYPE_XBAR, GAUDI2_BLOCK_TYPE_KDMA, GAUDI2_BLOCK_TYPE_XDMA, GAUDI2_BLOCK_TYPE_XFT, GAUDI2_BLOCK_TYPE_MAX }

extern "C" { pub static mut gaudi2_engine_id_str: *const *const core::ffi::c_char; pub static mut gaudi2_queue_id_str: *const *const core::ffi::c_char; }
#[inline] pub unsafe fn GAUDI2_ENG_ID_TO_STR(initiator: usize) -> *const core::ffi::c_char { if initiator >= GAUDI2_ENGINE_ID_SIZE { c"not found".as_ptr() } else { *gaudi2_engine_id_str.add(initiator) } }
#[inline] pub unsafe fn GAUDI2_QUEUE_ID_TO_STR(initiator: usize) -> *const core::ffi::c_char { if initiator >= GAUDI2_QUEUE_ID_SIZE { c"not found".as_ptr() } else { *gaudi2_queue_id_str.add(initiator) } }

#[repr(C)] pub struct dup_block_ctx { pub instance_cfg_fn: Option<unsafe extern "C" fn(*mut hl_device, u64, *mut core::ffi::c_void)>, pub data: *mut core::ffi::c_void, pub base: u64, pub block_off: u64, pub instance_off: u64, pub enabled_mask: u64, pub blocks: u32, pub instances: u32 }
#[repr(C)] pub struct gaudi2_queues_test_info { pub dma_addr: dma_addr_t, pub kern_addr: *mut core::ffi::c_void }

extern "C" {
    pub static gaudi2_dma_core_blocks_bases: [u32; DMA_CORE_ID_SIZE]; pub static gaudi2_qm_blocks_bases: [u32; GAUDI2_QUEUE_ID_SIZE]; pub static gaudi2_mme_acc_blocks_bases: [u32; MME_ID_SIZE]; pub static gaudi2_mme_ctrl_lo_blocks_bases: [u32; MME_ID_SIZE]; pub static edma_stream_base: [u32; NUM_OF_EDMA_PER_DCORE * NUM_OF_DCORES]; pub static gaudi2_rot_blocks_bases: [u32; ROTATOR_ID_SIZE];
    pub fn gaudi2_iterate_tpcs(hdev: *mut hl_device, ctx: *mut iterate_module_ctx); pub fn gaudi2_coresight_init(hdev: *mut hl_device) -> i32; pub fn gaudi2_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32; pub fn gaudi2_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx); pub fn gaudi2_init_blocks(hdev: *mut hl_device, cfg_ctx: *mut dup_block_ctx); pub fn gaudi2_is_hmmu_enabled(hdev: *mut hl_device, dcore_id: i32, hmmu_id: i32) -> bool; pub fn gaudi2_write_rr_to_all_lbw_rtrs(hdev: *mut hl_device, rr_type: u8, rr_index: u32, min_val: u64, max_val: u64); pub fn gaudi2_pb_print_security_errors(hdev: *mut hl_device, block_addr: u32, cause: u32, offended_addr: u32); pub fn gaudi2_init_security(hdev: *mut hl_device) -> i32; pub fn gaudi2_ack_protection_bits_errors(hdev: *mut hl_device); pub fn gaudi2_send_device_activity(hdev: *mut hl_device, open: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
