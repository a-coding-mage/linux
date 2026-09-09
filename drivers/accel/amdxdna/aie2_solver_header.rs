/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

pub const XRS_MAX_COL: u32 = 128;

/*
 * Structure used to describe a partition. A partition is column based
 * allocation unit described by its start column and number of columns.
 */
#[repr(C)]
pub struct aie_part {
    pub start_col: u32,
    pub ncols: u32,
}

/*
 * The QoS capabilities of a given AIE partition.
 */
#[repr(C)]
pub struct aie_qos_cap {
    pub opc: u32,    /* operations per cycle */
    pub dma_bw: u32, /* DMA bandwidth */
}

/*
 * QoS requirement of a resource allocation.
 */
#[repr(C)]
pub struct aie_qos {
    pub gops: u32,      /* Giga operations */
    pub fps: u32,       /* Frames per second */
    pub dma_bw: u32,    /* DMA bandwidth */
    pub latency: u32,   /* Frame response latency */
    pub exec_time: u32, /* Frame execution time */
    pub priority: u32,  /* Request priority */
}

/*
 * Structure used to describe a relocatable CDO (Configuration Data Object).
 */
#[repr(C)]
pub struct cdo_parts {
    pub start_cols: *mut u32, /* Start column array */
    pub cols_len: u32,        /* Length of start column array */
    pub ncols: u32,           /* # of column */
    pub qos_cap: aie_qos_cap, /* CDO QoS capabilities */
}

/*
 * Structure used to describe a request to allocate.
 */
#[repr(C)]
pub struct alloc_requests {
    pub rid: u64,
    pub cdo: cdo_parts,
    pub rqos: aie_qos, /* Requested QoS */
}

/*
 * Load callback argument
 */
#[repr(C)]
pub struct xrs_action_load {
    pub rid: u32,
    pub part: aie_part,
}

/*
 * Define the power level available
 *
 * POWER_LEVEL_MIN:
 *     Lowest power level. Usually set when all actions are unloaded.
 *
 * POWER_LEVEL_n
 *     Power levels 0 - n, is a step increase in system frequencies
 */
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_level {
    POWER_LEVEL_MIN = 0x0,
    POWER_LEVEL_0 = 0x1,
    POWER_LEVEL_1 = 0x2,
    POWER_LEVEL_2 = 0x3,
    POWER_LEVEL_3 = 0x4,
    POWER_LEVEL_4 = 0x5,
    POWER_LEVEL_5 = 0x6,
    POWER_LEVEL_6 = 0x7,
    POWER_LEVEL_7 = 0x8,
    POWER_LEVEL_NUM = 0x9,
}

/*
 * Structure used to describe the frequency table.
 * Resource solver chooses the frequency from the table
 * to meet the QOS requirements.
 */
#[repr(C)]
pub struct clk_list_info {
    pub num_levels: u32, /* available power levels */
    pub cu_clk_list: [u32; power_level::POWER_LEVEL_NUM as usize], /* available aie clock frequencies in Mhz*/
}

#[repr(C)]
pub struct xrs_action_ops {
    pub load: Option<unsafe extern "C" fn(cb_arg: *mut core::ffi::c_void, action: *mut xrs_action_load) -> i32>,
    pub unload: Option<unsafe extern "C" fn(cb_arg: *mut core::ffi::c_void) -> i32>,
    pub set_dft_dpm_level: Option<unsafe extern "C" fn(ddev: *mut drm_device, level: u32) -> i32>,
}

/*
 * Structure used to describe information for solver during initialization.
 */
#[repr(C)]
pub struct init_config {
    pub total_col: u32,
    pub sys_eff_factor: u32, /* system efficiency factor */
    pub latency_adj: u32,    /* latency adjustment in ms */
    pub clk_list: clk_list_info, /* List of frequencies available in system */
    pub ddev: *mut drm_device,
    pub actions: *mut xrs_action_ops,
}

extern "C" {
    /* xrsm_init() - Register resource solver. Resource solver client needs to
     * call this function to register itself.
     *
     * @cfg: The system metrics for resource solver to use
     *
     * Return: A resource solver handle
     *
     * Note: We should only create one handle per AIE array to be managed.
     */
    pub fn xrsm_init(cfg: *mut init_config) -> *mut core::ffi::c_void;

    /* xrs_allocate_resource() - Request to allocate resources for a given context
     * and a partition metadata. (See struct part_meta)
     *
     * @hdl: Resource solver handle obtained from xrs_init()
     * @req: Input to the Resource solver including request id
     *       and partition metadata.
     * @cb_arg: callback argument pointer
     *
     * Return: 0 when successful, or standard error number when failing.
     */
    pub fn xrs_allocate_resource(
        hdl: *mut core::ffi::c_void,
        req: *mut alloc_requests,
        cb_arg: *mut core::ffi::c_void,
    ) -> i32;

    /* xrs_release_resource() - Request to free resources for a given context. */
    pub fn xrs_release_resource(hdl: *mut core::ffi::c_void, rid: u64) -> i32;
}

/* External dependency supplied by the surrounding kernel code. */
pub enum drm_device {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
