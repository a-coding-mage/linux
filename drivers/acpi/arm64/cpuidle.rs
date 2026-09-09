// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM64 CPU idle arch support
 *
 * Copyright (C) 2014 ARM Ltd.
 * Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 */

// Dependencies supplied by the kernel headers are intentionally left external.

const EINVAL: i32 = 22;
const EOPNOTSUPP: i32 = 95;

#[repr(C)]
pub struct AcpiLpiState {
    pub address: u64,
    pub arch_flags: u32,
    pub index: u32,
}

#[repr(C)]
pub struct AcpiProcessorFlags {
    pub has_lpi: bool,
}

#[repr(C)]
pub struct AcpiPower {
    pub count: i32,
    pub lpi_states: *mut AcpiLpiState,
}

#[repr(C)]
pub struct AcpiProcessor {
    pub flags: AcpiProcessorFlags,
    pub power: AcpiPower,
}

#[repr(C)]
pub struct PsciOps {
    pub cpu_suspend: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut psci_ops: PsciOps;
    static mut processors: *mut AcpiProcessor;

    fn psci_power_state_is_valid(state: u32) -> bool;
    fn psci_cpu_suspend_enter(index: u32, state: u32) -> i32;
    fn pr_warn(format: *const core::ffi::c_char, ...);
}

#[inline]
fn arm64_lpi_is_retention_state(arch_flags: u32) -> bool {
    arch_flags == 0
}

unsafe fn psci_acpi_cpu_init_idle(cpu: u32) -> i32 {
    let pr = processors.add(cpu as usize);

    if pr.is_null() || !(*pr).flags.has_lpi {
        return -EINVAL;
    }

    /*
     * If the PSCI cpu_suspend function hook has not been initialized
     * idle states must not be enabled, so bail out
     */
    if (*core::ptr::addr_of!(psci_ops)).cpu_suspend.is_none() {
        return -EOPNOTSUPP;
    }

    let mut i = 1;
    while i < (*pr).power.count {
        let lpi = (*pr).power.lpi_states.add(i as usize);
        /*
         * Only bits[31:0] represent a PSCI power_state while
         * bits[63:32] must be 0x0 as per ARM ACPI FFH Specification
         */
        let state = (*lpi).address as u32;
        if !psci_power_state_is_valid(state) {
            pr_warn(b"Invalid PSCI power state %#x\n\0".as_ptr() as *const core::ffi::c_char, state);
            return -EINVAL;
        }
        i += 1;
    }

    0
}

pub unsafe extern "C" fn acpi_processor_ffh_lpi_probe(cpu: u32) -> i32 {
    psci_acpi_cpu_init_idle(cpu)
}

pub unsafe extern "C" fn acpi_processor_ffh_lpi_enter(lpi: *mut AcpiLpiState) -> i32 {
    let state = (*lpi).address as u32;

    if arm64_lpi_is_retention_state((*lpi).arch_flags) {
        // Equivalent to CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM_RCU(...).
        psci_cpu_suspend_enter((*lpi).index, state)
    } else {
        // Equivalent to CPU_PM_CPU_IDLE_ENTER_PARAM_RCU(...).
        psci_cpu_suspend_enter((*lpi).index, state)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
