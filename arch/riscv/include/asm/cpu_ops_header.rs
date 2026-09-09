/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 * Based on arch/arm64/include/asm/cpu_ops.h
 */

/**
 * struct cpu_operations - Callback operations for hotplugging CPUs.
 *
 * @cpu_start:        Boots a cpu into the kernel.
 * @cpu_stop:         Makes a cpu leave the kernel. Must not fail. Called from
 *                    the cpu being stopped.
 * @cpu_is_stopped:   Ensures a cpu has left the kernel. Called from another
 *                    cpu.
 */
#[repr(C)]
pub struct cpu_operations {
    pub cpu_start: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_uint, tidle: *mut task_struct) -> ::core::ffi::c_int>,
    // #ifdef CONFIG_HOTPLUG_CPU
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_stop: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_is_stopped: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_uint) -> bool>,
    // #endif
}

extern "C" {
    pub static cpu_ops_spinwait: cpu_operations;
    pub static cpu_ops: *const cpu_operations;
    // __init
    pub fn cpu_set_ops();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
