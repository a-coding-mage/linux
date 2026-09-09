/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2019-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// uapi/drm/habanalabs_accel.h, ../common/habanalabs.h,
// linux/habanalabs/hl_boot_if.h, and the Gaudi packet/device/FW headers.

pub const NUMBER_OF_EXT_HW_QUEUES: usize = 8;
pub const NUMBER_OF_CMPLT_QUEUES: usize = NUMBER_OF_EXT_HW_QUEUES;
pub const NUMBER_OF_CPU_HW_QUEUES: usize = 1;
pub const NUMBER_OF_INT_HW_QUEUES: usize = 100;
pub const NUMBER_OF_HW_QUEUES: usize = NUMBER_OF_EXT_HW_QUEUES + NUMBER_OF_CPU_HW_QUEUES + NUMBER_OF_INT_HW_QUEUES;
pub const NUMBER_OF_COLLECTIVE_QUEUES: usize = 12;
pub const NUMBER_OF_SOBS_IN_GRP: usize = 11;
pub const GAUDI_STREAM_MASTER_ARR_SIZE: usize = 8;
pub const CORESIGHT_TIMEOUT_USEC: u32 = 100000;
pub const GAUDI_MAX_CLK_FREQ: u64 = 2200000000;
pub const MAX_POWER_DEFAULT_PCI: u32 = 200000;
pub const MAX_POWER_DEFAULT_PMC: u32 = 350000;
pub const DC_POWER_DEFAULT_PCI: u32 = 60000;
pub const DC_POWER_DEFAULT_PMC: u32 = 60000;
pub const DC_POWER_DEFAULT_PMC_SEC: u32 = 97000;
pub const GAUDI_CPU_TIMEOUT_USEC: u32 = 30000000;
pub const TPC_ENABLED_MASK: u32 = 0xFF;
pub const GAUDI_HBM_SIZE_32GB: u64 = 0x800000000;
pub const GAUDI_HBM_DEVICES: usize = 4;
pub const GAUDI_HBM_CHANNELS: usize = 8;
pub const DMA_MAX_TRANSFER_SIZE: u32 = u32::MAX;
pub const GAUDI_DEFAULT_CARD_NAME: &str = "HL205";
pub const PCI_DMA_NUMBER_OF_CHNLS: usize = 2;
pub const HBM_DMA_NUMBER_OF_CHNLS: usize = 6;
pub const DMA_NUMBER_OF_CHNLS: usize = PCI_DMA_NUMBER_OF_CHNLS + HBM_DMA_NUMBER_OF_CHNLS;
pub const MME_NUMBER_OF_SLAVE_ENGINES: usize = 2;
pub const QMAN_STREAMS: usize = 4;
pub const PQ_FETCHER_CACHE_SIZE: usize = 8;
pub const MONITOR_MAX_SOBS: usize = 8;

// These expressions intentionally retain references to constants supplied by included headers.
pub const GAUDI_HBM_CFG_BASE: u64 = mmHBM0_BASE - CFG_BASE;
pub const GAUDI_HBM_CFG_OFFSET: u64 = mmHBM1_BASE - mmHBM0_BASE;
pub const GAUDI_MAX_PENDING_CS: usize = SZ_16K;
pub const MME_NUMBER_OF_ENGINES: usize = MME_NUMBER_OF_MASTER_ENGINES + MME_NUMBER_OF_SLAVE_ENGINES;
pub const MME_NUMBER_OF_QMANS: usize = MME_NUMBER_OF_MASTER_ENGINES * QMAN_STREAMS;
pub const DMA_QMAN_OFFSET: u64 = mmDMA1_QM_BASE - mmDMA0_QM_BASE;
pub const TPC_QMAN_OFFSET: u64 = mmTPC1_QM_BASE - mmTPC0_QM_BASE;
pub const MME_QMAN_OFFSET: u64 = mmMME1_QM_BASE - mmMME0_QM_BASE;
pub const NIC_MACRO_QMAN_OFFSET: u64 = mmNIC1_QM0_BASE - mmNIC0_QM0_BASE;
pub const NIC_ENGINE_QMAN_OFFSET: u64 = mmNIC0_QM1_BASE - mmNIC0_QM0_BASE;
pub const TPC_CFG_OFFSET: u64 = mmTPC1_CFG_BASE - mmTPC0_CFG_BASE;
pub const DMA_CORE_OFFSET: u64 = mmDMA1_CORE_BASE - mmDMA0_CORE_BASE;
pub const QMAN_LDMA_SRC_OFFSET: u64 = mmDMA0_CORE_SRC_BASE_LO - mmDMA0_CORE_CFG_0;
pub const QMAN_LDMA_DST_OFFSET: u64 = mmDMA0_CORE_DST_BASE_LO - mmDMA0_CORE_CFG_0;
pub const QMAN_LDMA_SIZE_OFFSET: u64 = mmDMA0_CORE_DST_TSIZE_0 - mmDMA0_CORE_CFG_0;
pub const QMAN_CPDMA_SRC_OFFSET: u64 = mmDMA0_QM_CQ_PTR_LO_4 - mmDMA0_CORE_CFG_0;
pub const QMAN_CPDMA_DST_OFFSET: u64 = mmDMA0_CORE_DST_BASE_LO - mmDMA0_CORE_CFG_0;
pub const QMAN_CPDMA_SIZE_OFFSET: u64 = mmDMA0_QM_CQ_TSIZE_4 - mmDMA0_CORE_CFG_0;
pub const SIF_RTR_CTRL_OFFSET: u64 = mmSIF_RTR_CTRL_1_BASE - mmSIF_RTR_CTRL_0_BASE;
pub const NIF_RTR_CTRL_OFFSET: u64 = mmNIF_RTR_CTRL_1_BASE - mmNIF_RTR_CTRL_0_BASE;
pub const MME_ACC_OFFSET: u64 = mmMME1_ACC_BASE - mmMME0_ACC_BASE;
pub const SRAM_BANK_OFFSET: u64 = mmSRAM_Y0_X1_RTR_BASE - mmSRAM_Y0_X0_RTR_BASE;
pub const NUM_OF_SOB_IN_BLOCK: u32 = ((mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_2047 - mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_0) + 4) >> 2;
pub const NUM_OF_MONITORS_IN_BLOCK: u32 = ((mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_511 - mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_MON_STATUS_0) + 4) >> 2;
pub const CPU_FW_IMAGE_SIZE: u64 = 0x10000000;
pub const MMU_PAGE_TABLES_SIZE: u64 = 0x0BF00000;
pub const MMU_CACHE_MNG_SIZE: u64 = 0x00100000;
pub const RESERVED: u64 = 0x04000000;
pub const CPU_FW_IMAGE_ADDR: u64 = DRAM_PHYS_BASE;
pub const MMU_PAGE_TABLES_ADDR: u64 = CPU_FW_IMAGE_ADDR + CPU_FW_IMAGE_SIZE;
pub const MMU_CACHE_MNG_ADDR: u64 = MMU_PAGE_TABLES_ADDR + MMU_PAGE_TABLES_SIZE;
pub const DRAM_DRIVER_END_ADDR: u64 = MMU_CACHE_MNG_ADDR + MMU_CACHE_MNG_SIZE + RESERVED;
pub const DRAM_BASE_ADDR_USER: u64 = 0x20000000;
pub const MME_QMAN_LENGTH: usize = 1024;
pub const MME_QMAN_SIZE_IN_BYTES: usize = MME_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const HBM_DMA_QMAN_LENGTH: usize = 4096;
pub const HBM_DMA_QMAN_SIZE_IN_BYTES: usize = HBM_DMA_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC_QMAN_LENGTH: usize = 1024;
pub const TPC_QMAN_SIZE_IN_BYTES: usize = TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const NIC_QMAN_LENGTH: usize = 4096;
pub const NIC_QMAN_SIZE_IN_BYTES: usize = NIC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const SRAM_USER_BASE_OFFSET: u64 = GAUDI_DRIVER_SRAM_RESERVED_SIZE_FROM_START;
pub const VA_HOST_SPACE_START: u64 = 0x1000000000000;
pub const VA_HOST_SPACE_END: u64 = 0x3FF8000000000;
pub const VA_HOST_SPACE_SIZE: u64 = VA_HOST_SPACE_END - VA_HOST_SPACE_START;
pub const HOST_SPACE_INTERNAL_CB_SZ: usize = SZ_2M;

pub const HW_CAP_PLL: u32 = 1 << 0;
pub const HW_CAP_HBM: u32 = 1 << 1;
pub const HW_CAP_MMU: u32 = 1 << 2;
pub const HW_CAP_MME: u32 = 1 << 3;
pub const HW_CAP_CPU: u32 = 1 << 4;
pub const HW_CAP_PCI_DMA: u32 = 1 << 5;
pub const HW_CAP_MSI: u32 = 1 << 6;
pub const HW_CAP_CPU_Q: u32 = 1 << 7;
pub const HW_CAP_HBM_DMA: u32 = 1 << 8;
pub const HW_CAP_SRAM_SCRAMBLER: u32 = 1 << 10;
pub const HW_CAP_HBM_SCRAMBLER: u32 = 1 << 11;
pub const HW_CAP_NIC_MASK: u32 = 0x3ff << 14;
pub const HW_CAP_NIC_SHIFT: u32 = 14;
pub const HW_CAP_TPC_MASK: u32 = 0xff << 24;
pub const HW_CAP_TPC_SHIFT: u32 = 24;
pub const NEXT_SYNC_OBJ_ADDR_INTERVAL: u64 = mmSYNC_MNGR_W_N_SYNC_MNGR_OBJS_SOB_OBJ_0 - mmSYNC_MNGR_E_N_SYNC_MNGR_OBJS_SOB_OBJ_0;
pub const NUM_OF_MME_ENGINES: usize = 2;
pub const NUM_OF_MME_SUB_ENGINES: usize = 2;
pub const NUM_OF_TPC_ENGINES: usize = 8;
pub const NUM_OF_DMA_ENGINES: usize = 8;
pub const NUM_OF_QUEUES: usize = 5;
pub const NUM_OF_STREAMS: usize = 4;
pub const NUM_OF_FENCES: usize = 4;

#[inline]
pub const fn gaudi_cpu_pci_msb_addr(addr: u64) -> u64 { (addr & (((1u64 << 11) - 1) << 39)) >> 39 }
#[inline]
pub fn gaudi_pci_to_cpu_addr(addr: &mut u64) { *addr = (*addr & !(((1u64 << 11) - 1) << 39)) | (1u64 << 39); }
#[inline]
pub fn gaudi_cpu_to_pci_addr(addr: &mut u64, extension: u64) { *addr = (*addr & !(((1u64 << 11) - 1) << 39)) | (extension << 39); }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gaudi_dma_channels { GAUDI_PCI_DMA_1, GAUDI_PCI_DMA_2, GAUDI_HBM_DMA_1, GAUDI_HBM_DMA_2, GAUDI_HBM_DMA_3, GAUDI_HBM_DMA_4, GAUDI_HBM_DMA_5, GAUDI_HBM_DMA_6, GAUDI_DMA_MAX }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gaudi_tpc_mask { GAUDI_TPC_MASK_TPC0 = 0x01, GAUDI_TPC_MASK_TPC1 = 0x02, GAUDI_TPC_MASK_TPC2 = 0x04, GAUDI_TPC_MASK_TPC3 = 0x08, GAUDI_TPC_MASK_TPC4 = 0x10, GAUDI_TPC_MASK_TPC5 = 0x20, GAUDI_TPC_MASK_TPC6 = 0x40, GAUDI_TPC_MASK_TPC7 = 0x80, GAUDI_TPC_MASK_ALL = 0xFF }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum gaudi_nic_mask { GAUDI_NIC_MASK_NIC0 = 0x01, GAUDI_NIC_MASK_NIC1 = 0x02, GAUDI_NIC_MASK_NIC2 = 0x04, GAUDI_NIC_MASK_NIC3 = 0x08, GAUDI_NIC_MASK_NIC4 = 0x10, GAUDI_NIC_MASK_NIC5 = 0x20, GAUDI_NIC_MASK_NIC6 = 0x40, GAUDI_NIC_MASK_NIC7 = 0x80, GAUDI_NIC_MASK_NIC8 = 0x100, GAUDI_NIC_MASK_NIC9 = 0x200, GAUDI_NIC_MASK_ALL = 0x3FF }

#[repr(C)]
pub struct gaudi_hw_sob_group { pub hdev: *mut hl_device, pub kref: kref, pub base_sob_id: u32, pub queue_id: u32 }
pub const NUM_SOB_GROUPS: usize = HL_RSVD_SOBS * QMAN_STREAMS;
#[repr(C)]
pub struct gaudi_collective_properties { pub hw_sob_group: [gaudi_hw_sob_group; NUM_SOB_GROUPS], pub next_sob_group_val: [u16; QMAN_STREAMS], pub curr_sob_group_idx: [u8; QMAN_STREAMS], pub mstr_sob_mask: [u8; HL_COLLECTIVE_RSVD_MSTR_MONS] }
#[repr(C)]
pub struct gaudi_internal_qman_info { pub pq_kernel_addr: *mut core::ffi::c_void, pub pq_dma_addr: dma_addr_t, pub pq_size: usize }
#[repr(C)]
pub struct gaudi_device { pub cpucp_info_get: Option<unsafe extern "C" fn(*mut hl_device) -> i32>, pub hw_queues_lock: spinlock_t, pub internal_qmans: [gaudi_internal_qman_info; GAUDI_QUEUE_ID_SIZE], pub collective_props: gaudi_collective_properties, pub hbm_bar_cur_addr: u64, pub events: [u32; GAUDI_EVENT_SIZE], pub events_stat: [u32; GAUDI_EVENT_SIZE], pub events_stat_aggregate: [u32; GAUDI_EVENT_SIZE], pub hw_cap_initialized: u32, pub mmu_cache_inv_pi: u8 }

extern "C" {
    pub fn gaudi_init_security(hdev: *mut hl_device);
    pub fn gaudi_ack_protection_bits_errors(hdev: *mut hl_device);
    pub fn gaudi_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32;
    pub fn gaudi_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
    pub fn gaudi_mmu_prepare_reg(hdev: *mut hl_device, reg: u64, asid: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
