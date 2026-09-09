/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 by Rivos Inc.
 */

// C header guard: __ASM_CPU_OPS_SBI_H

// The following dependencies are supplied by the surrounding translation:
// #include <linux/init.h>
// #include <linux/sched.h>
// #include <linux/threads.h>

extern "C" {
    pub static cpu_ops_sbi: cpu_operations;
}

/**
 * struct sbi_hart_boot_data - Hart specific boot used during booting and
 *                              cpu hotplug.
 * @task_ptr: A pointer to the hart specific tp
 * @stack_ptr: A pointer to the hart specific sp
 */
#[repr(C)]
pub struct sbi_hart_boot_data {
    pub task_ptr: *mut core::ffi::c_void,
    pub stack_ptr: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
