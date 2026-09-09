// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the corresponding kernel headers:
unsafe extern "C" {
    fn io_master(cpu: core::ffi::c_int) -> bool;
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn arch_cpu_is_hotpluggable(cpu: core::ffi::c_int) -> bool {
    !io_master(cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
