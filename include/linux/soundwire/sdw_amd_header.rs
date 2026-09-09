/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/*
 * Copyright (C) 2023-24 Advanced Micro Devices, Inc. All rights reserved.
 */

// Dependencies supplied by the Linux SoundWire and ACPI interfaces are
// intentionally left as external Rust types.

/* AMD pm_runtime quirk definitions */
pub const AMD_SDW_CLK_STOP_MODE: u32 = 1;
pub const AMD_SDW_POWER_OFF_MODE: u32 = 2;
pub const ACP_SDW0: u32 = 0;
pub const ACP_SDW1: u32 = 1;
pub const AMD_SDW_MAX_MANAGER_COUNT: usize = 2;
pub const ACP63_PCI_REV_ID: u32 = 0x63;
pub const ACP70_PCI_REV_ID: u32 = 0x70;
pub const ACP71_PCI_REV_ID: u32 = 0x71;
pub const ACP72_PCI_REV_ID: u32 = 0x72;

#[repr(C)]
pub struct acp_sdw_pdata {
    pub instance: u16,
    pub acp_rev: u32,
    /* mutex to protect acp common register access */
    pub acp_sdw_lock: *mut mutex,
}

/**
 * struct sdw_amd_dai_runtime: AMD sdw dai runtime  data
 *
 * @name: SoundWire stream name
 * @stream: stream runtime
 * @bus: Bus handle
 * @stream_type: Stream type
 */
#[repr(C)]
pub struct sdw_amd_dai_runtime {
    pub name: *mut i8,
    pub stream: *mut sdw_stream_runtime,
    pub bus: *mut sdw_bus,
    pub stream_type: sdw_stream_type,
}

/**
 * struct amd_sdw_manager - amd manager driver context
 * @bus: bus handle
 * @dev: linux device
 * @mmio: SoundWire registers mmio base
 * @acp_mmio: acp registers mmio base
 * @amd_sdw_irq_thread: SoundWire manager irq workqueue
 * @amd_sdw_work: peripheral status work queue
 * @acp_sdw_lock: mutex to protect acp share register access
 * @status: peripheral devices status array
 * @num_din_ports: number of input ports
 * @num_dout_ports: number of output ports
 * @max_ports: total number of input ports and output ports
 * @cols_index: Column index in frame shape
 * @rows_index: Rows index in frame shape
 * @port_offset_map: dynamic array to map port block offset
 * @instance: SoundWire manager instance
 * @quirks: SoundWire manager quirks
 * @wake_en_mask: wake enable mask per SoundWire manager
 * @acp_rev: acp pci device revision id
 * @clk_stopped: flag set to true when clock is stopped
 * @power_mode_mask: flag interprets amd SoundWire manager power mode
 * @dai_runtime_array: dai runtime array
 */
#[repr(C)]
pub struct amd_sdw_manager {
    pub bus: sdw_bus,
    pub dev: *mut device,
    pub mmio: *mut core::ffi::c_void,
    pub acp_mmio: *mut core::ffi::c_void,
    pub amd_sdw_irq_thread: work_struct,
    pub amd_sdw_work: work_struct,
    /* mutex to protect acp common register access */
    pub acp_sdw_lock: *mut mutex,
    pub status: [sdw_slave_status; SDW_MAX_DEVICES + 1],
    pub num_din_ports: i32,
    pub num_dout_ports: i32,
    pub max_ports: i32,
    pub cols_index: i32,
    pub rows_index: i32,
    pub port_offset_map: *mut i32,
    pub instance: u32,
    pub quirks: u32,
    pub wake_en_mask: u32,
    pub power_mode_mask: u32,
    pub acp_rev: u32,
    pub clk_stopped: bool,
    pub dai_runtime_array: *mut *mut sdw_amd_dai_runtime,
}

/** Soundwire AMD information found in ACPI tables. */
#[repr(C)]
pub struct sdw_amd_acpi_info {
    pub handle: acpi_handle,
    pub count: i32,
    pub link_mask: u32,
}

/** Context allocated by the controller driver probe. */
#[repr(C)]
pub struct sdw_amd_ctx {
    pub count: i32,
    pub link_mask: u32,
    pub pdev: [*mut platform_device; AMD_SDW_MAX_MANAGER_COUNT],
    pub peripherals: *mut sdw_peripherals,
}

/** Soundwire AMD global resource structure. */
#[repr(C)]
pub struct sdw_amd_res {
    pub acp_rev: u32,
    pub addr: u32,
    pub reg_range: u32,
    pub link_mask: u32,
    pub count: i32,
    pub mmio_base: *mut core::ffi::c_void,
    pub handle: acpi_handle,
    pub parent: *mut device,
    pub dev: *mut device,
    /* use to protect acp common registers access */
    pub acp_lock: *mut mutex,
}

unsafe extern "C" {
    pub fn sdw_amd_probe(res: *mut sdw_amd_res, ctx: *mut *mut sdw_amd_ctx) -> i32;
    pub fn sdw_amd_exit(ctx: *mut sdw_amd_ctx);
    pub fn sdw_amd_get_slave_info(ctx: *mut sdw_amd_ctx) -> i32;
    pub fn amd_sdw_scan_controller(info: *mut sdw_amd_acpi_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
