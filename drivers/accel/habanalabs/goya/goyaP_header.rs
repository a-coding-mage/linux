/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding translation unit correspond to the
// original kernel and Goya header includes.

pub const NUMBER_OF_CMPLT_QUEUES: u32 = 5;
pub const NUMBER_OF_EXT_HW_QUEUES: u32 = 5;
pub const NUMBER_OF_CPU_HW_QUEUES: u32 = 1;
pub const NUMBER_OF_INT_HW_QUEUES: u32 = 9;
pub const NUMBER_OF_HW_QUEUES: u32 = NUMBER_OF_EXT_HW_QUEUES + NUMBER_OF_CPU_HW_QUEUES + NUMBER_OF_INT_HW_QUEUES;

pub const NUMBER_OF_INTERRUPTS: u32 = NUMBER_OF_CMPLT_QUEUES + 1;

pub const QMAN_FENCE_TIMEOUT_USEC: u32 = 10000;
pub const QMAN_STOP_TIMEOUT_USEC: u32 = 100000;
pub const CORESIGHT_TIMEOUT_USEC: u32 = 100000;
pub const GOYA_CPU_TIMEOUT_USEC: u32 = 15000000;
pub const TPC_ENABLED_MASK: u32 = 0xFF;
pub const PLL_HIGH_DEFAULT: u32 = 1575000000;
pub const MAX_POWER_DEFAULT: u32 = 200000;
pub const DC_POWER_DEFAULT: u32 = 20000;
pub const DRAM_PHYS_DEFAULT_SIZE: u64 = 0x100000000;
pub const GOYA_DEFAULT_CARD_NAME: &str = "HL1000";
pub const GOYA_MAX_PENDING_CS: u32 = 64;

pub const CPU_FW_IMAGE_SIZE: u32 = 0x10000000;
pub const MMU_PAGE_TABLES_SIZE: u32 = 0x0FC00000;
pub const MMU_DRAM_DEFAULT_PAGE_SIZE: u32 = 0x00200000;
pub const MMU_CACHE_MNG_SIZE: u32 = 0x00001000;
pub const CPU_FW_IMAGE_ADDR: u64 = DRAM_PHYS_BASE;
pub const MMU_PAGE_TABLES_ADDR: u64 = CPU_FW_IMAGE_ADDR + CPU_FW_IMAGE_SIZE as u64;
pub const MMU_DRAM_DEFAULT_PAGE_ADDR: u64 = MMU_PAGE_TABLES_ADDR + MMU_PAGE_TABLES_SIZE as u64;
pub const MMU_CACHE_MNG_ADDR: u64 = MMU_DRAM_DEFAULT_PAGE_ADDR + MMU_DRAM_DEFAULT_PAGE_SIZE as u64;
pub const DRAM_DRIVER_END_ADDR: u64 = MMU_CACHE_MNG_ADDR + MMU_CACHE_MNG_SIZE as u64;
pub const DRAM_BASE_ADDR_USER: u32 = 0x20000000;

pub const MME_QMAN_BASE_OFFSET: u32 = 0x000000;
pub const MME_QMAN_LENGTH: u32 = 64;
pub const TPC_QMAN_LENGTH: u32 = 64;
pub const TPC0_QMAN_BASE_OFFSET: u32 = MME_QMAN_BASE_OFFSET + MME_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC1_QMAN_BASE_OFFSET: u32 = TPC0_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC2_QMAN_BASE_OFFSET: u32 = TPC1_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC3_QMAN_BASE_OFFSET: u32 = TPC2_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC4_QMAN_BASE_OFFSET: u32 = TPC3_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC5_QMAN_BASE_OFFSET: u32 = TPC4_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC6_QMAN_BASE_OFFSET: u32 = TPC5_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const TPC7_QMAN_BASE_OFFSET: u32 = TPC6_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const SRAM_DRIVER_RES_OFFSET: u32 = TPC7_QMAN_BASE_OFFSET + TPC_QMAN_LENGTH * QMAN_PQ_ENTRY_SIZE;
pub const SRAM_USER_BASE_OFFSET: u32 = GOYA_KMD_SRAM_RESERVED_SIZE_FROM_START;

pub const VA_HOST_SPACE_START: u64 = 0x1000000000000;
pub const VA_HOST_SPACE_END: u64 = 0x3FF8000000000;
pub const VA_HOST_SPACE_SIZE: u64 = VA_HOST_SPACE_END - VA_HOST_SPACE_START;
pub const VA_DDR_SPACE_START: u64 = 0x800000000;
pub const VA_DDR_SPACE_END: u64 = 0x2000000000;
pub const VA_DDR_SPACE_SIZE: u64 = VA_DDR_SPACE_END - VA_DDR_SPACE_START;
pub const VA_CPU_ACCESSIBLE_MEM_ADDR: u64 = 0x8000000000;
pub const DMA_MAX_TRANSFER_SIZE: u32 = U32_MAX;

pub const HW_CAP_PLL: u32 = 0x00000001;
pub const HW_CAP_DDR_0: u32 = 0x00000002;
pub const HW_CAP_DDR_1: u32 = 0x00000004;
pub const HW_CAP_MME: u32 = 0x00000008;
pub const HW_CAP_CPU: u32 = 0x00000010;
pub const HW_CAP_DMA: u32 = 0x00000020;
pub const HW_CAP_MSIX: u32 = 0x00000040;
pub const HW_CAP_CPU_Q: u32 = 0x00000080;
pub const HW_CAP_MMU: u32 = 0x00000100;
pub const HW_CAP_TPC_MBIST: u32 = 0x00000200;
pub const HW_CAP_GOLDEN: u32 = 0x00000400;
pub const HW_CAP_TPC: u32 = 0x00000800;

#[repr(C)]
pub struct goya_work_freq {
    pub hdev: *mut hl_device,
    pub work_freq: delayed_work,
}

#[repr(C)]
pub struct goya_device {
    // TODO: remove hw_queues_lock after moving to scheduler code
    pub hw_queues_lock: spinlock_t,
    pub goya_work: *mut goya_work_freq,
    pub mme_clk: u64,
    pub tpc_clk: u64,
    pub ic_clk: u64,
    pub ddr_bar_cur_addr: u64,
    pub events_stat: [u32; GOYA_ASYNC_EVENT_ID_SIZE as usize],
    pub events_stat_aggregate: [u32; GOYA_ASYNC_EVENT_ID_SIZE as usize],
    pub hw_cap_initialized: u32,
    pub device_cpu_mmu_mappings_done: u8,
    pub curr_pll_profile: hl_pll_frequency,
    pub pm_mng_profile: hl_pm_mng_profile,
}

extern "C" {
    pub fn goya_set_fixed_properties(hdev: *mut hl_device) -> i32;
    pub fn goya_mmu_init(hdev: *mut hl_device) -> i32;
    pub fn goya_init_dma_qmans(hdev: *mut hl_device);
    pub fn goya_init_mme_qmans(hdev: *mut hl_device);
    pub fn goya_init_tpc_qmans(hdev: *mut hl_device);
    pub fn goya_init_cpu_queues(hdev: *mut hl_device) -> i32;
    pub fn goya_init_security(hdev: *mut hl_device);
    pub fn goya_ack_protection_bits_errors(hdev: *mut hl_device);
    pub fn goya_late_init(hdev: *mut hl_device) -> i32;
    pub fn goya_late_fini(hdev: *mut hl_device);
    pub fn goya_ring_doorbell(hdev: *mut hl_device, hw_queue_id: u32, pi: u32);
    pub fn goya_pqe_write(hdev: *mut hl_device, pqe: *mut __le64, bd: *mut hl_bd);
    pub fn goya_update_eq_ci(hdev: *mut hl_device, val: u32);
    pub fn goya_restore_phase_topology(hdev: *mut hl_device);
    pub fn goya_context_switch(hdev: *mut hl_device, asid: u32) -> i32;
    pub fn goya_debugfs_i2c_read(hdev: *mut hl_device, i2c_bus: u8, i2c_addr: u8, i2c_reg: u8, val: *mut u32) -> i32;
    pub fn goya_debugfs_i2c_write(hdev: *mut hl_device, i2c_bus: u8, i2c_addr: u8, i2c_reg: u8, val: u32) -> i32;
    pub fn goya_debugfs_led_set(hdev: *mut hl_device, led: u8, state: u8);
    pub fn goya_test_queue(hdev: *mut hl_device, hw_queue_id: u32) -> i32;
    pub fn goya_test_queues(hdev: *mut hl_device) -> i32;
    pub fn goya_test_cpu_queue(hdev: *mut hl_device) -> i32;
    pub fn goya_send_cpu_message(hdev: *mut hl_device, msg: *mut u32, len: u16, timeout: u32, result: *mut u64) -> i32;
    pub fn goya_get_temperature(hdev: *mut hl_device, sensor_index: i32, attr: u32) -> i64;
    pub fn goya_get_voltage(hdev: *mut hl_device, sensor_index: i32, attr: u32) -> i64;
    pub fn goya_get_current(hdev: *mut hl_device, sensor_index: i32, attr: u32) -> i64;
    pub fn goya_get_fan_speed(hdev: *mut hl_device, sensor_index: i32, attr: u32) -> i64;
    pub fn goya_get_pwm_info(hdev: *mut hl_device, sensor_index: i32, attr: u32) -> i64;
    pub fn goya_set_pwm_info(hdev: *mut hl_device, sensor_index: i32, attr: u32, value: i64);
    pub fn goya_get_max_power(hdev: *mut hl_device) -> u64;
    pub fn goya_set_max_power(hdev: *mut hl_device, value: u64);
    pub fn goya_set_pll_profile(hdev: *mut hl_device, freq: hl_pll_frequency);
    pub fn goya_add_device_attr(hdev: *mut hl_device, dev_clk_attr_grp: *mut attribute_group, dev_vrm_attr_grp: *mut attribute_group);
    pub fn goya_cpucp_info_get(hdev: *mut hl_device) -> i32;
    pub fn goya_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut core::ffi::c_void) -> i32;
    pub fn goya_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
    pub fn goya_suspend(hdev: *mut hl_device) -> i32;
    pub fn goya_resume(hdev: *mut hl_device) -> i32;
    pub fn goya_handle_eqe(hdev: *mut hl_device, eq_entry: *mut hl_eq_entry);
    pub fn goya_get_events_stat(hdev: *mut hl_device, aggregate: bool, size: *mut u32) -> *mut core::ffi::c_void;
    pub fn goya_add_end_of_cb_packets(hdev: *mut hl_device, kernel_address: *mut core::ffi::c_void, len: u32, original_len: u32, cq_addr: u64, cq_val: u32, msix_vec: u32, eb: bool);
    pub fn goya_cs_parser(hdev: *mut hl_device, parser: *mut hl_cs_parser) -> i32;
    pub fn goya_scrub_device_mem(hdev: *mut hl_device) -> i32;
    pub fn goya_get_int_queue_base(hdev: *mut hl_device, queue_id: u32, dma_handle: *mut dma_addr_t, queue_len: *mut u16) -> *mut core::ffi::c_void;
    pub fn goya_get_dma_desc_list_size(hdev: *mut hl_device, sgt: *mut sg_table) -> u32;
    pub fn goya_send_heartbeat(hdev: *mut hl_device) -> i32;
    pub fn goya_cpu_accessible_dma_pool_alloc(hdev: *mut hl_device, size: usize, dma_handle: *mut dma_addr_t) -> *mut core::ffi::c_void;
    pub fn goya_cpu_accessible_dma_pool_free(hdev: *mut hl_device, size: usize, vaddr: *mut core::ffi::c_void);
    pub fn goya_mmu_remove_device_cpu_mappings(hdev: *mut hl_device);
    pub fn goya_get_queue_id_for_cq(hdev: *mut hl_device, cq_idx: u32) -> u32;
    pub fn goya_get_device_time(hdev: *mut hl_device) -> u64;
    pub fn goya_set_frequency(hdev: *mut hl_device, freq: hl_pll_frequency) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
