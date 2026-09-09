/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

use core::ffi::c_void;

/* External kernel and register definitions supplied by the surrounding build. */
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

/* The C token-pasting register names are represented by register expressions. */
macro_rules! rocket_pc_readl {
    ($core:expr, $reg:expr) => {
        unsafe { readl((*$core).pc_iomem.wrapping_add($reg as usize)) }
    };
}
macro_rules! rocket_pc_writel {
    ($core:expr, $reg:expr, $value:expr) => {
        unsafe { writel($value, (*$core).pc_iomem.wrapping_add($reg as usize)) }
    };
}

macro_rules! rocket_cna_readl {
    ($core:expr, $reg:expr, $status:expr) => {
        unsafe { readl((*$core).cna_iomem.wrapping_add(($reg as usize).wrapping_sub($status as usize))) }
    };
}
macro_rules! rocket_cna_writel {
    ($core:expr, $reg:expr, $status:expr, $value:expr) => {
        unsafe { writel($value, (*$core).cna_iomem.wrapping_add(($reg as usize).wrapping_sub($status as usize))) }
    };
}

macro_rules! rocket_core_readl {
    ($core:expr, $reg:expr, $status:expr) => {
        unsafe { readl((*$core).core_iomem.wrapping_add(($reg as usize).wrapping_sub($status as usize))) }
    };
}
macro_rules! rocket_core_writel {
    ($core:expr, $reg:expr, $status:expr, $value:expr) => {
        unsafe { writel($value, (*$core).core_iomem.wrapping_add(($reg as usize).wrapping_sub($status as usize))) }
    };
}

#[repr(C)]
pub struct rocket_core {
    pub dev: *mut device,
    pub rdev: *mut rocket_device,
    pub index: u32,

    pub irq: i32,
    pub pc_iomem: *mut c_void,
    pub cna_iomem: *mut c_void,
    pub core_iomem: *mut c_void,
    pub clks: [clk_bulk_data; 4],
    pub resets: [reset_control_bulk_data; 2],

    pub iommu_group: *mut iommu_group,

    pub job_lock: mutex,
    pub in_flight_job: *mut rocket_job,

    pub fence_lock: spinlock,

    pub reset: rocket_core_reset_state,

    pub sched: drm_gpu_scheduler,
    pub fence_context: u64,
    pub emit_seqno: u64,
}

#[repr(C)]
pub struct rocket_core_reset_state {
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub pending: atomic_t,
}

extern "C" {
    pub fn rocket_core_init(core: *mut rocket_core) -> i32;
    pub fn rocket_core_fini(core: *mut rocket_core);
    pub fn rocket_core_reset(core: *mut rocket_core);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
