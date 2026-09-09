/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 ARM Ltd.
 */

// Translated from the C header. Dependencies supplied by other files are
// intentionally referenced but not implemented here.

/**
 * struct cpu_operations - Callback operations for hotplugging CPUs.
 *
 * @name: Name of the property as appears in a devicetree cpu node's
 *        enable-method property. On systems booting with ACPI, @name
 *        identifies the struct cpu_operations entry corresponding to
 *        the boot protocol specified in the ACPI MADT table.
 * @cpu_init: Reads any data necessary for a specific enable-method for a
 *            proposed logical id.
 * @cpu_prepare: Early one-time preparation step for a cpu. If there is a
 *               mechanism for doing so, tests whether it is possible to boot
 *               the given CPU.
 * @cpu_boot: Boots a cpu into the kernel.
 * @cpu_postboot: Optionally, perform any post-boot cleanup or necessary
 *                synchronisation. Called from the cpu being booted.
 * @cpu_can_disable: Determines whether a CPU can be disabled based on
 *                   mechanism-specific information.
 * @cpu_disable: Prepares a cpu to die. May fail for some mechanism-specific
 *               reason, which will cause the hot unplug to be aborted. Called
 *               from the cpu to be killed.
 * @cpu_die: Makes a cpu leave the kernel. Must not fail. Called from the
 *           cpu being killed.
 * @cpu_kill: Ensures a cpu has left the kernel. Called from another cpu.
 */
#[repr(C)]
pub struct cpu_operations {
    pub name: *const core::ffi::c_char,
    pub cpu_init: Option<unsafe extern "C" fn(_: core::ffi::c_uint) -> core::ffi::c_int>,
    pub cpu_prepare: Option<unsafe extern "C" fn(_: core::ffi::c_uint) -> core::ffi::c_int>,
    pub cpu_boot: Option<unsafe extern "C" fn(_: core::ffi::c_uint) -> core::ffi::c_int>,
    pub cpu_postboot: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_can_disable:
        Option<unsafe extern "C" fn(cpu: core::ffi::c_uint) -> bool>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_disable:
        Option<unsafe extern "C" fn(cpu: core::ffi::c_uint) -> core::ffi::c_int>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_die: Option<unsafe extern "C" fn(cpu: core::ffi::c_uint)>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_kill:
        Option<unsafe extern "C" fn(cpu: core::ffi::c_uint) -> core::ffi::c_int>,
}

pub unsafe extern "C" fn init_cpu_ops(cpu: core::ffi::c_int) -> core::ffi::c_int;
pub unsafe extern "C" fn get_cpu_ops(
    cpu: core::ffi::c_int,
) -> *const cpu_operations;

pub unsafe extern "C" fn init_bootcpu_ops() {
    init_cpu_ops(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
