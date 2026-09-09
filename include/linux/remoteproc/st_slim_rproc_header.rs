/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SLIM core rproc driver header
 *
 * Copyright (C) 2016 STMicroelectronics
 *
 * Author: Peter Griffin <peter.griffin@linaro.org>
 */

// C header guard: _ST_REMOTEPROC_SLIM_H

pub const ST_SLIM_MEM_MAX: usize = 2;
pub const ST_SLIM_MAX_CLK: usize = 4;

#[repr(C)]
pub enum StSlimMemType {
    ST_SLIM_DMEM,
    ST_SLIM_IMEM,
}

// External dependency types supplied by other translation units.
#[repr(C)]
pub struct rproc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

/**
 * struct st_slim_mem - slim internal memory structure
 * @cpu_addr: MPU virtual address of the memory region
 * @bus_addr: Bus address used to access the memory region
 * @size: Size of the memory region
 */
#[repr(C)]
pub struct st_slim_mem {
    pub cpu_addr: *mut core::ffi::c_void,
    pub bus_addr: usize,
    pub size: usize,
}

/**
 * struct st_slim_rproc - SLIM slim core
 * @rproc: rproc handle
 * @mem: slim memory information
 * @slimcore: slim slimcore regs
 * @peri: slim peripheral regs
 * @clks: slim clocks
 */
#[repr(C)]
pub struct st_slim_rproc {
    pub rproc: *mut rproc,
    pub mem: [st_slim_mem; ST_SLIM_MEM_MAX],
    pub slimcore: *mut core::ffi::c_void,
    pub peri: *mut core::ffi::c_void,

    /* st_slim_rproc private */
    pub clks: [*mut clk; ST_SLIM_MAX_CLK],
}

unsafe extern "C" {
    pub fn st_slim_rproc_alloc(
        pdev: *mut platform_device,
        fw_name: *mut core::ffi::c_char,
    ) -> *mut st_slim_rproc;
    pub fn st_slim_rproc_put(slim_rproc: *mut st_slim_rproc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
